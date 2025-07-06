use crate::error::Result;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

/// 並列処理設定
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    pub num_threads: usize,
    pub chunk_size: usize,
    pub enable_parallel: bool,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        let num_cpus = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        Self {
            num_threads: num_cpus,
            chunk_size: 1000,
            enable_parallel: num_cpus > 1,
        }
    }
}

/// 並列処理結果
#[derive(Debug, Clone)]
pub struct ParallelResult<T> {
    pub results: Vec<T>,
    pub execution_time_ms: u64,
    pub threads_used: usize,
    pub chunks_processed: usize,
}

/// 並列データ処理
pub fn parallel_map<T, U, F>(
    data: &[T],
    config: &ParallelConfig,
    func: F,
) -> Result<ParallelResult<U>>
where
    T: Clone + Send + Sync + 'static,
    U: Send + Sync + 'static,
    F: Fn(&T) -> U + Send + Sync + Copy + 'static,
{
    let start_time = std::time::Instant::now();

    if !config.enable_parallel || data.len() < config.chunk_size {
        // シングルスレッド処理
        let results: Vec<U> = data.iter().map(func).collect();

        return Ok(ParallelResult {
            results,
            execution_time_ms: start_time.elapsed().as_millis() as u64,
            threads_used: 1,
            chunks_processed: 1,
        });
    }

    // 並列処理 - データを所有版に変換
    let owned_data: Vec<T> = data.to_vec();
    let chunks: Vec<Vec<T>> = owned_data
        .chunks(config.chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    let num_chunks = chunks.len();
    let num_threads = config.num_threads.min(num_chunks);

    let (tx, rx) = mpsc::channel();
    let chunk_queue = Arc::new(Mutex::new(chunks.into_iter().enumerate()));

    // ワーカースレッドを起動
    let mut handles = Vec::new();
    for _ in 0..num_threads {
        let tx = tx.clone();
        let queue = Arc::clone(&chunk_queue);

        let handle = thread::spawn(move || {
            while let Some((chunk_index, chunk)) = {
                let mut queue = queue.lock().unwrap();
                queue.next()
            } {
                let chunk_results: Vec<U> = chunk.iter().map(func).collect();
                if tx.send((chunk_index, chunk_results)).is_err() {
                    break;
                }
            }
        });

        handles.push(handle);
    }

    // 送信側を閉じる
    drop(tx);

    // 結果を収集
    let mut chunk_results: Vec<(usize, Vec<U>)> = Vec::new();
    while let Ok((chunk_index, results)) = rx.recv() {
        chunk_results.push((chunk_index, results));
    }

    // スレッドの完了を待つ
    for handle in handles {
        handle
            .join()
            .map_err(|_| crate::error::BenfError::ParseError("Thread join failed".to_string()))?;
    }

    // 結果をソートして結合
    chunk_results.sort_by_key(|(index, _)| *index);
    let results: Vec<U> = chunk_results
        .into_iter()
        .flat_map(|(_, results)| results)
        .collect();

    Ok(ParallelResult {
        results,
        execution_time_ms: start_time.elapsed().as_millis() as u64,
        threads_used: num_threads,
        chunks_processed: num_chunks,
    })
}

/// 並列リダクション処理
pub fn parallel_reduce<T, U, F, R>(
    data: &[T],
    config: &ParallelConfig,
    map_func: F,
    reduce_func: R,
    initial: U,
) -> Result<ParallelResult<U>>
where
    T: Clone + Send + Sync + 'static,
    U: Clone + Send + Sync + 'static,
    F: Fn(&T) -> U + Send + Sync + Copy + 'static,
    R: Fn(U, U) -> U + Send + Sync + 'static,
{
    let start_time = std::time::Instant::now();

    if !config.enable_parallel || data.len() < config.chunk_size {
        // シングルスレッド処理
        let result = data.iter().map(map_func).fold(initial, &reduce_func);

        return Ok(ParallelResult {
            results: vec![result],
            execution_time_ms: start_time.elapsed().as_millis() as u64,
            threads_used: 1,
            chunks_processed: 1,
        });
    }

    // 並列map処理
    let map_result = parallel_map(data, config, map_func)?;

    // 結果をリダクション
    let final_result = map_result.results.into_iter().fold(initial, reduce_func);

    Ok(ParallelResult {
        results: vec![final_result],
        execution_time_ms: start_time.elapsed().as_millis() as u64,
        threads_used: map_result.threads_used,
        chunks_processed: map_result.chunks_processed,
    })
}

/// 並列統計計算
pub fn parallel_statistics(
    data: &[f64],
    config: &ParallelConfig,
) -> Result<ParallelResult<StatisticsChunk>> {
    let start_time = std::time::Instant::now();

    if !config.enable_parallel || data.len() < config.chunk_size {
        let stats = calculate_chunk_statistics(data);
        return Ok(ParallelResult {
            results: vec![stats],
            execution_time_ms: start_time.elapsed().as_millis() as u64,
            threads_used: 1,
            chunks_processed: 1,
        });
    }

    // データを所有版に変換してチャンクごとに分割
    let owned_data: Vec<f64> = data.to_vec();
    let chunks: Vec<Vec<f64>> = owned_data
        .chunks(config.chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    // 各チャンクで統計を計算
    let chunk_stats = parallel_map(&chunks, config, |chunk| calculate_chunk_statistics(chunk))?;

    Ok(ParallelResult {
        results: chunk_stats.results,
        execution_time_ms: start_time.elapsed().as_millis() as u64,
        threads_used: chunk_stats.threads_used,
        chunks_processed: chunk_stats.chunks_processed,
    })
}

/// チャンク統計
#[derive(Debug, Clone)]
pub struct StatisticsChunk {
    pub count: usize,
    pub sum: f64,
    pub sum_squares: f64,
    pub min: f64,
    pub max: f64,
    pub first_digit_counts: [usize; 9], // ベンフォード法則用
}

/// チャンク統計を計算
fn calculate_chunk_statistics(data: &[f64]) -> StatisticsChunk {
    if data.is_empty() {
        return StatisticsChunk {
            count: 0,
            sum: 0.0,
            sum_squares: 0.0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            first_digit_counts: [0; 9],
        };
    }

    let mut sum = 0.0;
    let mut sum_squares = 0.0;
    let mut min_val = f64::INFINITY;
    let mut max_val = f64::NEG_INFINITY;
    let mut first_digit_counts = [0; 9];

    for &value in data {
        sum += value;
        sum_squares += value * value;
        min_val = min_val.min(value);
        max_val = max_val.max(value);

        // 第一桁をカウント（ベンフォード法則用）
        let abs_value = value.abs();
        if abs_value >= 1.0 {
            let first_digit = get_first_digit(abs_value);
            if (1..=9).contains(&first_digit) {
                first_digit_counts[first_digit - 1] += 1;
            }
        }
    }

    StatisticsChunk {
        count: data.len(),
        sum,
        sum_squares,
        min: min_val,
        max: max_val,
        first_digit_counts,
    }
}

/// 第一桁を取得
fn get_first_digit(value: f64) -> usize {
    let mut n = value;
    while n >= 10.0 {
        n /= 10.0;
    }
    n as usize
}

/// 複数のチャンク統計を結合
pub fn combine_statistics_chunks(chunks: &[StatisticsChunk]) -> StatisticsChunk {
    if chunks.is_empty() {
        return StatisticsChunk {
            count: 0,
            sum: 0.0,
            sum_squares: 0.0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            first_digit_counts: [0; 9],
        };
    }

    let mut combined = StatisticsChunk {
        count: 0,
        sum: 0.0,
        sum_squares: 0.0,
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
        first_digit_counts: [0; 9],
    };

    for chunk in chunks {
        combined.count += chunk.count;
        combined.sum += chunk.sum;
        combined.sum_squares += chunk.sum_squares;
        combined.min = combined.min.min(chunk.min);
        combined.max = combined.max.max(chunk.max);

        for i in 0..9 {
            combined.first_digit_counts[i] += chunk.first_digit_counts[i];
        }
    }

    combined
}

/// 並列ベンフォード分析
pub fn parallel_benford_analysis(
    data: &[f64],
    config: &ParallelConfig,
) -> Result<ParallelResult<BenfordChunkResult>> {
    let start_time = std::time::Instant::now();

    if !config.enable_parallel || data.len() < config.chunk_size {
        let result = analyze_benford_chunk(data);
        return Ok(ParallelResult {
            results: vec![result],
            execution_time_ms: start_time.elapsed().as_millis() as u64,
            threads_used: 1,
            chunks_processed: 1,
        });
    }

    // データを所有版に変換してチャンクごとに分割
    let owned_data: Vec<f64> = data.to_vec();
    let chunks: Vec<Vec<f64>> = owned_data
        .chunks(config.chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    let chunk_results = parallel_map(&chunks, config, |chunk| analyze_benford_chunk(chunk))?;

    Ok(ParallelResult {
        results: chunk_results.results,
        execution_time_ms: start_time.elapsed().as_millis() as u64,
        threads_used: chunk_results.threads_used,
        chunks_processed: chunk_results.chunks_processed,
    })
}

/// ベンフォードチャンク結果
#[derive(Debug, Clone)]
pub struct BenfordChunkResult {
    pub first_digit_counts: [usize; 9],
    pub total_count: usize,
    pub chunk_mad: f64,
}

/// ベンフォードチャンク分析
fn analyze_benford_chunk(data: &[f64]) -> BenfordChunkResult {
    let stats = calculate_chunk_statistics(data);

    // ベンフォード期待値
    let expected_proportions = [
        30.103, 17.609, 12.494, 9.691, 7.918, 6.695, 5.799, 5.115, 4.576,
    ];

    // MAD計算
    let mut mad = 0.0;
    let total_valid = stats.first_digit_counts.iter().sum::<usize>();

    if total_valid > 0 {
        for (i, &expected) in expected_proportions.iter().enumerate() {
            let observed_prop = (stats.first_digit_counts[i] as f64 / total_valid as f64) * 100.0;
            mad += (observed_prop - expected).abs();
        }
        mad /= 9.0;
    }

    BenfordChunkResult {
        first_digit_counts: stats.first_digit_counts,
        total_count: total_valid,
        chunk_mad: mad,
    }
}

/// 並列異常値検出
pub fn parallel_outlier_detection(
    data: &[f64],
    config: &ParallelConfig,
    z_threshold: f64,
) -> Result<ParallelResult<Vec<(usize, f64, f64)>>> {
    let start_time = std::time::Instant::now();

    // まず全体の統計を計算
    let overall_stats = parallel_statistics(data, config)?;
    let combined_stats = combine_statistics_chunks(&overall_stats.results);

    let mean = combined_stats.sum / combined_stats.count as f64;
    let variance = (combined_stats.sum_squares / combined_stats.count as f64) - (mean * mean);
    let std_dev = variance.sqrt();

    if !config.enable_parallel || data.len() < config.chunk_size {
        let outliers = detect_outliers_chunk(data, 0, mean, std_dev, z_threshold);
        return Ok(ParallelResult {
            results: vec![outliers],
            execution_time_ms: start_time.elapsed().as_millis() as u64,
            threads_used: 1,
            chunks_processed: 1,
        });
    }

    // データを所有版に変換してチャンクごとに分割
    let owned_data: Vec<f64> = data.to_vec();
    let chunks_with_offset: Vec<(usize, Vec<f64>)> = owned_data
        .chunks(config.chunk_size)
        .enumerate()
        .map(|(chunk_idx, chunk)| (chunk_idx * config.chunk_size, chunk.to_vec()))
        .collect();

    let outlier_results = parallel_map(&chunks_with_offset, config, move |(offset, chunk)| {
        detect_outliers_chunk(chunk, *offset, mean, std_dev, z_threshold)
    })?;

    Ok(ParallelResult {
        results: outlier_results.results,
        execution_time_ms: start_time.elapsed().as_millis() as u64,
        threads_used: outlier_results.threads_used,
        chunks_processed: outlier_results.chunks_processed,
    })
}

/// チャンク内異常値検出
fn detect_outliers_chunk(
    data: &[f64],
    offset: usize,
    mean: f64,
    std_dev: f64,
    z_threshold: f64,
) -> Vec<(usize, f64, f64)> {
    if std_dev == 0.0 {
        return Vec::new();
    }

    data.iter()
        .enumerate()
        .filter_map(|(i, &value)| {
            let z_score = (value - mean) / std_dev;
            if z_score.abs() > z_threshold {
                Some((offset + i, value, z_score))
            } else {
                None
            }
        })
        .collect()
}

/// 並列処理のパフォーマンス測定
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub serial_time_ms: u64,
    pub parallel_time_ms: u64,
    pub speedup: f64,
    pub efficiency: f64,
    pub threads_used: usize,
}

/// パフォーマンス比較
pub fn benchmark_parallel_performance<T, U, F>(
    data: &[T],
    config: &ParallelConfig,
    func: F,
) -> Result<PerformanceMetrics>
where
    T: Clone + Send + Sync + 'static,
    U: Send + Sync + 'static,
    F: Fn(&T) -> U + Send + Sync + Copy + 'static,
{
    // シリアル実行
    let serial_start = std::time::Instant::now();
    let _serial_result: Vec<U> = data.iter().map(func).collect();
    let serial_time = serial_start.elapsed().as_millis() as u64;

    // 並列実行
    let parallel_result = parallel_map(data, config, func)?;
    let parallel_time = parallel_result.execution_time_ms;

    let speedup = serial_time as f64 / parallel_time as f64;
    let efficiency = speedup / parallel_result.threads_used as f64;

    Ok(PerformanceMetrics {
        serial_time_ms: serial_time,
        parallel_time_ms: parallel_time,
        speedup,
        efficiency,
        threads_used: parallel_result.threads_used,
    })
}
