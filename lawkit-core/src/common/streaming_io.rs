use crate::error::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// diffxの技術を活用した最適化IO処理
pub struct OptimizedFileReader {
    reader: Box<dyn BufRead>,
    file_size: Option<u64>,
    buffer_size: usize,
}

impl OptimizedFileReader {
    /// ファイルからの最適化読み込み（diffxパターン）
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(&path)?;
        let file_size = file.metadata()?.len();
        let buffer_size = Self::optimal_buffer_size(file_size);

        Ok(Self {
            reader: Box::new(BufReader::with_capacity(buffer_size, file)),
            file_size: Some(file_size),
            buffer_size,
        })
    }

    /// 標準入力からの読み込み
    pub fn from_stdin() -> Self {
        let stdin = std::io::stdin();
        Self {
            reader: Box::new(stdin.lock()),
            file_size: None,
            buffer_size: 64 * 1024, // デフォルト64KB
        }
    }

    /// diffxの経験値に基づく最適バッファサイズ
    fn optimal_buffer_size(file_size: u64) -> usize {
        match file_size {
            0..=1_000_000 => 8 * 1024,           // 8KB（小ファイル）
            1_000_001..=10_000_000 => 32 * 1024, // 32KB（中ファイル）
            _ => 128 * 1024,                     // 128KB（大ファイル）
        }
    }

    /// 大容量ファイルストリーミング判定（diffx 100MB閾値）
    pub fn should_use_streaming(&self) -> bool {
        const MAX_MEMORY_SIZE: u64 = 100 * 1024 * 1024; // 100MB

        if let Some(size) = self.file_size {
            size > MAX_MEMORY_SIZE
        } else {
            true // stdin は常にストリーミング
        }
    }

    /// ライン単位でのストリーミング読み込み
    pub fn read_lines_streaming<F, T>(&mut self, mut processor: F) -> Result<Vec<T>>
    where
        F: FnMut(String) -> Result<Option<T>>,
    {
        let mut results = Vec::new();
        let mut line = String::new();

        loop {
            line.clear();
            match self.reader.read_line(&mut line)? {
                0 => break, // EOF
                _ => {
                    let trimmed_line = line.trim_end().to_string();
                    if let Some(result) = processor(trimmed_line)? {
                        results.push(result);
                    }
                }
            }
        }

        Ok(results)
    }

    /// バッチ処理（diffxのバッチサイズ最適化）
    pub fn read_lines_batched<F, T>(
        &mut self,
        batch_size: usize,
        mut processor: F,
    ) -> Result<Vec<T>>
    where
        F: FnMut(Vec<String>) -> Result<Vec<T>>,
    {
        let mut results = Vec::new();
        let mut batch = Vec::with_capacity(batch_size);
        let mut line = String::new();

        loop {
            line.clear();
            match self.reader.read_line(&mut line)? {
                0 => break, // EOF
                _ => {
                    let trimmed_line = line.trim_end().to_string();
                    batch.push(trimmed_line);

                    if batch.len() >= batch_size {
                        let mut batch_results = processor(batch)?;
                        results.append(&mut batch_results);
                        batch = Vec::with_capacity(batch_size);
                    }
                }
            }
        }

        // 残りのバッチを処理
        if !batch.is_empty() {
            let mut batch_results = processor(batch)?;
            results.append(&mut batch_results);
        }

        Ok(results)
    }

    /// ファイルサイズ情報
    pub fn file_size(&self) -> Option<u64> {
        self.file_size
    }

    /// バッファサイズ情報
    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }
}

/// メモリ使用量の推定（diffxパターン）
pub fn estimate_memory_usage_for_processing(file_size: Option<u64>, data_points: usize) -> usize {
    const BASELINE_OVERHEAD: usize = 1024 * 1024; // 1MB基本オーバーヘッド

    let file_memory = if let Some(size) = file_size {
        // diffx知見：入力の1.5x-2x使用
        let multiplier = if size > 10_000_000 { 1.5 } else { 2.0 };
        (size as f64 * multiplier) as usize
    } else {
        data_points * 32 // 推定32バイト/データポイント
    };

    file_memory + BASELINE_OVERHEAD
}

/// 適応的処理戦略選択（diffxの自動最適化パターン）
#[derive(Debug, Clone)]
pub enum ProcessingStrategy {
    InMemory,  // 小データ：メモリ内処理
    Streaming, // 大データ：ストリーミング処理
    BatchedStreaming {
        // 超大データ：バッチ化ストリーミング
        batch_size: usize,
    },
}

impl ProcessingStrategy {
    pub fn select_optimal(file_size: Option<u64>, estimated_data_points: usize) -> Self {
        const SMALL_THRESHOLD: u64 = 1_000_000; // 1MB
        const LARGE_THRESHOLD: u64 = 100_000_000; // 100MB（diffx閾値）

        if let Some(size) = file_size {
            if size < SMALL_THRESHOLD {
                ProcessingStrategy::InMemory
            } else if size < LARGE_THRESHOLD {
                ProcessingStrategy::Streaming
            } else {
                // diffxの大ファイル戦略
                let batch_size = if size > 1_000_000_000 {
                    10000 // 1GB超：大バッチ
                } else {
                    5000 // 100MB-1GB：中バッチ
                };
                ProcessingStrategy::BatchedStreaming { batch_size }
            }
        } else {
            // stdin：データポイント数で判定
            if estimated_data_points < 10000 {
                ProcessingStrategy::InMemory
            } else {
                ProcessingStrategy::Streaming
            }
        }
    }
}
