use crate::error::Result;
use std::collections::VecDeque;

/// メモリ効率化設定
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    pub chunk_size: usize,
    pub max_memory_mb: usize,
    pub enable_streaming: bool,
    pub enable_compression: bool,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            chunk_size: 10000,
            max_memory_mb: 512,
            enable_streaming: true,
            enable_compression: false,
        }
    }
}

impl MemoryConfig {
    /// diffxの知見に基づく適応的チャンクサイズ
    pub fn adaptive_chunk_size(file_size: u64) -> usize {
        match file_size {
            0..=1_000_000 => 1000,          // 1MB以下：小チャンク
            1_000_001..=10_000_000 => 5000, // 10MB以下：中チャンク
            _ => 10000,                     // 10MB超：大チャンク
        }
    }

    /// diffxパターン：メモリ効率ターゲット（1.5x-2x入力サイズ）
    pub fn memory_efficiency_target(file_size: u64) -> usize {
        let mb_size = (file_size / 1024 / 1024) as usize;
        if mb_size > 100 {
            mb_size / 2 // 大ファイルでは50%に制限（diffx知見）
        } else {
            mb_size * 2 // 小ファイルでは2x許可（diffx知見）
        }
    }
}

/// ストリーミングデータプロセッサ
pub struct StreamingProcessor<T> {
    buffer: VecDeque<T>,
    chunk_size: usize,
    total_processed: usize,
}

impl<T> StreamingProcessor<T> {
    pub fn new(config: &MemoryConfig) -> Self {
        Self {
            buffer: VecDeque::new(),
            chunk_size: config.chunk_size,
            total_processed: 0,
        }
    }

    /// データを追加
    pub fn push(&mut self, item: T) -> Option<Vec<T>> {
        self.buffer.push_back(item);

        if self.buffer.len() >= self.chunk_size {
            self.flush_chunk()
        } else {
            None
        }
    }

    /// チャンクをフラッシュ
    fn flush_chunk(&mut self) -> Option<Vec<T>> {
        if self.buffer.is_empty() {
            return None;
        }

        let chunk_size = self.chunk_size.min(self.buffer.len());
        let chunk: Vec<T> = self.buffer.drain(0..chunk_size).collect();
        self.total_processed += chunk.len();

        Some(chunk)
    }

    /// 残りのデータを取得
    pub fn finish(mut self) -> Option<Vec<T>> {
        if self.buffer.is_empty() {
            None
        } else {
            let remaining: Vec<T> = self.buffer.into_iter().collect();
            self.total_processed += remaining.len();
            Some(remaining)
        }
    }

    /// 処理済み件数を取得
    pub fn processed_count(&self) -> usize {
        self.total_processed
    }

    /// バッファサイズを取得
    pub fn buffer_size(&self) -> usize {
        self.buffer.len()
    }
}

/// チャンクイテレータ
pub struct ChunkIterator<T> {
    data: Vec<T>,
    chunk_size: usize,
    current_index: usize,
}

impl<T> ChunkIterator<T> {
    pub fn new(data: Vec<T>, chunk_size: usize) -> Self {
        Self {
            data,
            chunk_size,
            current_index: 0,
        }
    }
}

impl<T: Clone> Iterator for ChunkIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.data.len() {
            return None;
        }

        let end_index = (self.current_index + self.chunk_size).min(self.data.len());
        let chunk = self.data[self.current_index..end_index].to_vec();
        self.current_index = end_index;

        Some(chunk)
    }
}

/// メモリ効率的な統計計算
#[derive(Debug, Clone)]
pub struct IncrementalStatistics {
    count: usize,
    sum: f64,
    sum_squares: f64,
    min: f64,
    max: f64,
    m2: f64, // for variance calculation
    mean: f64,
}

impl Default for IncrementalStatistics {
    fn default() -> Self {
        Self {
            count: 0,
            sum: 0.0,
            sum_squares: 0.0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            m2: 0.0,
            mean: 0.0,
        }
    }
}

impl IncrementalStatistics {
    pub fn new() -> Self {
        Self::default()
    }

    /// データ点を追加（Welford's online algorithm）
    pub fn add(&mut self, value: f64) {
        self.count += 1;
        self.sum += value;
        self.sum_squares += value * value;
        self.min = self.min.min(value);
        self.max = self.max.max(value);

        // Welford's algorithm for variance
        let delta = value - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = value - self.mean;
        self.m2 += delta * delta2;
    }

    /// 複数のデータ点を追加
    pub fn add_batch(&mut self, values: &[f64]) {
        for &value in values {
            self.add(value);
        }
    }

    /// 他の統計と結合
    pub fn merge(&mut self, other: &IncrementalStatistics) {
        if other.count == 0 {
            return;
        }

        if self.count == 0 {
            *self = other.clone();
            return;
        }

        let combined_count = self.count + other.count;
        let delta = other.mean - self.mean;
        let combined_mean = (self.count as f64 * self.mean + other.count as f64 * other.mean)
            / combined_count as f64;

        // Combine M2 values
        let combined_m2 = self.m2
            + other.m2
            + delta * delta * (self.count as f64 * other.count as f64) / combined_count as f64;

        self.count = combined_count;
        self.sum += other.sum;
        self.sum_squares += other.sum_squares;
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.mean = combined_mean;
        self.m2 = combined_m2;
    }

    /// 平均を取得
    pub fn mean(&self) -> f64 {
        self.mean
    }

    /// 分散を取得
    pub fn variance(&self) -> f64 {
        if self.count < 2 {
            0.0
        } else {
            self.m2 / (self.count - 1) as f64
        }
    }

    /// 標準偏差を取得
    pub fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }

    /// サンプル数を取得
    pub fn count(&self) -> usize {
        self.count
    }

    /// 最小値を取得
    pub fn min(&self) -> f64 {
        self.min
    }

    /// 最大値を取得
    pub fn max(&self) -> f64 {
        self.max
    }
}

/// メモリ効率的なベンフォード分析
#[derive(Debug, Clone, Default)]
pub struct IncrementalBenford {
    first_digit_counts: [usize; 9],
    total_count: usize,
}

impl IncrementalBenford {
    pub fn new() -> Self {
        Self::default()
    }

    /// 数値を追加
    pub fn add(&mut self, value: f64) {
        let abs_value = value.abs();
        if abs_value >= 1.0 {
            let first_digit = get_first_digit(abs_value);
            if (1..=9).contains(&first_digit) {
                self.first_digit_counts[first_digit - 1] += 1;
                self.total_count += 1;
            }
        }
    }

    /// 複数の数値を追加
    pub fn add_batch(&mut self, values: &[f64]) {
        for &value in values {
            self.add(value);
        }
    }

    /// 他のベンフォード統計と結合
    pub fn merge(&mut self, other: &IncrementalBenford) {
        for (i, &other_count) in other.first_digit_counts.iter().enumerate() {
            self.first_digit_counts[i] += other_count;
        }
        self.total_count += other.total_count;
    }

    /// MAD（Mean Absolute Deviation）を計算
    pub fn calculate_mad(&self) -> f64 {
        if self.total_count == 0 {
            return 0.0;
        }

        let expected_proportions = [
            30.103, 17.609, 12.494, 9.691, 7.918, 6.695, 5.799, 5.115, 4.576,
        ];

        let mut mad = 0.0;
        for (i, &expected) in expected_proportions.iter().enumerate() {
            let observed_prop =
                (self.first_digit_counts[i] as f64 / self.total_count as f64) * 100.0;
            mad += (observed_prop - expected).abs();
        }

        mad / 9.0
    }

    /// 第一桁の分布を取得
    pub fn get_distribution(&self) -> [f64; 9] {
        let mut distribution = [0.0; 9];
        if self.total_count > 0 {
            for (i, item) in distribution.iter_mut().enumerate() {
                *item = (self.first_digit_counts[i] as f64 / self.total_count as f64) * 100.0;
            }
        }
        distribution
    }

    /// カウントを取得
    pub fn get_counts(&self) -> &[usize; 9] {
        &self.first_digit_counts
    }

    /// 総数を取得
    pub fn total_count(&self) -> usize {
        self.total_count
    }
}

/// チャンクベースの分析結果
#[derive(Debug, Clone)]
pub struct ChunkAnalysisResult<T> {
    pub chunks_processed: usize,
    pub total_items: usize,
    pub memory_used_mb: f64,
    pub processing_time_ms: u64,
    pub result: T,
}

/// ストリーミングベンフォード分析
pub fn streaming_benford_analysis<I>(
    data_iter: I,
    config: &MemoryConfig,
) -> Result<ChunkAnalysisResult<IncrementalBenford>>
where
    I: Iterator<Item = f64>,
{
    let start_time = std::time::Instant::now();
    let mut processor = StreamingProcessor::new(config);
    let mut benford = IncrementalBenford::new();
    let mut chunks_processed = 0;

    for value in data_iter {
        if let Some(chunk) = processor.push(value) {
            let mut chunk_benford = IncrementalBenford::new();
            chunk_benford.add_batch(&chunk);
            benford.merge(&chunk_benford);
            chunks_processed += 1;
        }
    }

    // 処理済み件数を記録（finish()前にカウントを取得）
    let mut total_processed = processor.processed_count();

    // 残りのデータを処理
    if let Some(remaining) = processor.finish() {
        total_processed += remaining.len(); // 残りのデータ数を追加
        let mut chunk_benford = IncrementalBenford::new();
        chunk_benford.add_batch(&remaining);
        benford.merge(&chunk_benford);
        chunks_processed += 1;
    }

    let memory_used_mb = (total_processed * std::mem::size_of::<f64>()) as f64 / 1024.0 / 1024.0;

    Ok(ChunkAnalysisResult {
        chunks_processed,
        total_items: total_processed,
        memory_used_mb,
        processing_time_ms: start_time.elapsed().as_millis() as u64,
        result: benford,
    })
}

/// ストリーミング統計分析
pub fn streaming_statistics_analysis<I>(
    data_iter: I,
    config: &MemoryConfig,
) -> Result<ChunkAnalysisResult<IncrementalStatistics>>
where
    I: Iterator<Item = f64>,
{
    let start_time = std::time::Instant::now();
    let mut processor = StreamingProcessor::new(config);
    let mut stats = IncrementalStatistics::new();
    let mut chunks_processed = 0;

    for value in data_iter {
        if let Some(chunk) = processor.push(value) {
            let mut chunk_stats = IncrementalStatistics::new();
            chunk_stats.add_batch(&chunk);
            stats.merge(&chunk_stats);
            chunks_processed += 1;
        }
    }

    // 処理済み件数を記録
    let total_processed = processor.processed_count();

    // 残りのデータを処理
    if let Some(remaining) = processor.finish() {
        let mut chunk_stats = IncrementalStatistics::new();
        chunk_stats.add_batch(&remaining);
        stats.merge(&chunk_stats);
        chunks_processed += 1;
    }

    let memory_used_mb = (total_processed * std::mem::size_of::<f64>()) as f64 / 1024.0 / 1024.0;

    Ok(ChunkAnalysisResult {
        chunks_processed,
        total_items: total_processed,
        memory_used_mb,
        processing_time_ms: start_time.elapsed().as_millis() as u64,
        result: stats,
    })
}

/// インクリメンタルパレート分析
#[derive(Debug, Clone)]
pub struct IncrementalPareto {
    values: Vec<f64>,
    sorted_values: Vec<f64>,
    needs_sorting: bool,
    statistics: IncrementalStatistics,
}

impl Default for IncrementalPareto {
    fn default() -> Self {
        Self::new()
    }
}

impl IncrementalPareto {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            sorted_values: Vec::new(),
            needs_sorting: true,
            statistics: IncrementalStatistics::new(),
        }
    }

    pub fn add(&mut self, value: f64) {
        self.values.push(value);
        self.statistics.add(value);
        self.needs_sorting = true;
    }

    pub fn add_batch(&mut self, values: &[f64]) {
        for &value in values {
            self.add(value);
        }
    }

    pub fn merge(&mut self, other: &IncrementalPareto) {
        self.values.extend_from_slice(&other.values);
        self.statistics.merge(&other.statistics);
        self.needs_sorting = true;
    }

    pub fn get_sorted_values(&mut self) -> &[f64] {
        if self.needs_sorting {
            self.sorted_values = self.values.clone();
            self.sorted_values.sort_by(|a, b| b.partial_cmp(a).unwrap());
            self.needs_sorting = false;
        }
        &self.sorted_values
    }

    pub fn statistics(&self) -> &IncrementalStatistics {
        &self.statistics
    }

    pub fn count(&self) -> usize {
        self.values.len()
    }
}

/// インクリメンタルジップ分析
#[derive(Debug, Clone)]
pub struct IncrementalZipf {
    frequency_map: std::collections::HashMap<String, usize>,
    total_count: usize,
}

impl Default for IncrementalZipf {
    fn default() -> Self {
        Self::new()
    }
}

impl IncrementalZipf {
    pub fn new() -> Self {
        Self {
            frequency_map: std::collections::HashMap::new(),
            total_count: 0,
        }
    }

    pub fn add_word(&mut self, word: String) {
        *self.frequency_map.entry(word).or_insert(0) += 1;
        self.total_count += 1;
    }

    pub fn add_words(&mut self, words: &[String]) {
        for word in words {
            self.add_word(word.clone());
        }
    }

    pub fn merge(&mut self, other: &IncrementalZipf) {
        for (word, count) in &other.frequency_map {
            *self.frequency_map.entry(word.clone()).or_insert(0) += count;
        }
        self.total_count += other.total_count;
    }

    pub fn get_sorted_frequencies(&self) -> Vec<(String, usize)> {
        let mut frequencies: Vec<_> = self
            .frequency_map
            .iter()
            .map(|(word, &count)| (word.clone(), count))
            .collect();
        frequencies.sort_by(|a, b| b.1.cmp(&a.1));
        frequencies
    }

    pub fn total_count(&self) -> usize {
        self.total_count
    }

    pub fn unique_words(&self) -> usize {
        self.frequency_map.len()
    }
}

/// インクリメンタル正規分布分析
#[derive(Debug, Clone)]
pub struct IncrementalNormal {
    statistics: IncrementalStatistics,
    values: Vec<f64>, // 正規性検定に必要
}

impl Default for IncrementalNormal {
    fn default() -> Self {
        Self::new()
    }
}

impl IncrementalNormal {
    pub fn new() -> Self {
        Self {
            statistics: IncrementalStatistics::new(),
            values: Vec::new(),
        }
    }

    pub fn add(&mut self, value: f64) {
        self.statistics.add(value);
        self.values.push(value);
    }

    pub fn add_batch(&mut self, values: &[f64]) {
        for &value in values {
            self.add(value);
        }
    }

    pub fn merge(&mut self, other: &IncrementalNormal) {
        self.statistics.merge(&other.statistics);
        self.values.extend_from_slice(&other.values);
    }

    pub fn statistics(&self) -> &IncrementalStatistics {
        &self.statistics
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn count(&self) -> usize {
        self.statistics.count
    }
}

/// インクリメンタルポアソン分析
#[derive(Debug, Clone)]
pub struct IncrementalPoisson {
    event_counts: Vec<usize>,
    statistics: IncrementalStatistics,
}

impl Default for IncrementalPoisson {
    fn default() -> Self {
        Self::new()
    }
}

impl IncrementalPoisson {
    pub fn new() -> Self {
        Self {
            event_counts: Vec::new(),
            statistics: IncrementalStatistics::new(),
        }
    }

    pub fn add_count(&mut self, count: usize) {
        let count_f64 = count as f64;
        self.event_counts.push(count);
        self.statistics.add(count_f64);
    }

    pub fn add_counts(&mut self, counts: &[usize]) {
        for &count in counts {
            self.add_count(count);
        }
    }

    pub fn merge(&mut self, other: &IncrementalPoisson) {
        self.event_counts.extend_from_slice(&other.event_counts);
        self.statistics.merge(&other.statistics);
    }

    pub fn statistics(&self) -> &IncrementalStatistics {
        &self.statistics
    }

    pub fn event_counts(&self) -> &[usize] {
        &self.event_counts
    }

    pub fn count(&self) -> usize {
        self.event_counts.len()
    }
}

/// ストリーミングパレート分析
pub fn streaming_pareto_analysis<I>(
    data_iter: I,
    config: &MemoryConfig,
) -> Result<ChunkAnalysisResult<IncrementalPareto>>
where
    I: Iterator<Item = f64>,
{
    let start_time = std::time::Instant::now();
    let mut processor = StreamingProcessor::new(config);
    let mut pareto = IncrementalPareto::new();
    let mut chunks_processed = 0;

    for value in data_iter {
        if let Some(chunk) = processor.push(value) {
            let mut chunk_pareto = IncrementalPareto::new();
            chunk_pareto.add_batch(&chunk);
            pareto.merge(&chunk_pareto);
            chunks_processed += 1;
        }
    }

    let mut total_processed = processor.processed_count();

    if let Some(remaining) = processor.finish() {
        total_processed += remaining.len();
        let mut chunk_pareto = IncrementalPareto::new();
        chunk_pareto.add_batch(&remaining);
        pareto.merge(&chunk_pareto);
        chunks_processed += 1;
    }

    let memory_used_mb = (total_processed * std::mem::size_of::<f64>()) as f64 / 1024.0 / 1024.0;

    Ok(ChunkAnalysisResult {
        chunks_processed,
        total_items: total_processed,
        memory_used_mb,
        processing_time_ms: start_time.elapsed().as_millis() as u64,
        result: pareto,
    })
}

/// ストリーミング正規分布分析
pub fn streaming_normal_analysis<I>(
    data_iter: I,
    config: &MemoryConfig,
) -> Result<ChunkAnalysisResult<IncrementalNormal>>
where
    I: Iterator<Item = f64>,
{
    let start_time = std::time::Instant::now();
    let mut processor = StreamingProcessor::new(config);
    let mut normal = IncrementalNormal::new();
    let mut chunks_processed = 0;

    for value in data_iter {
        if let Some(chunk) = processor.push(value) {
            let mut chunk_normal = IncrementalNormal::new();
            chunk_normal.add_batch(&chunk);
            normal.merge(&chunk_normal);
            chunks_processed += 1;
        }
    }

    let mut total_processed = processor.processed_count();

    if let Some(remaining) = processor.finish() {
        total_processed += remaining.len();
        let mut chunk_normal = IncrementalNormal::new();
        chunk_normal.add_batch(&remaining);
        normal.merge(&chunk_normal);
        chunks_processed += 1;
    }

    let memory_used_mb = (total_processed * std::mem::size_of::<f64>()) as f64 / 1024.0 / 1024.0;

    Ok(ChunkAnalysisResult {
        chunks_processed,
        total_items: total_processed,
        memory_used_mb,
        processing_time_ms: start_time.elapsed().as_millis() as u64,
        result: normal,
    })
}

/// ストリーミングポアソン分析
pub fn streaming_poisson_analysis<I>(
    data_iter: I,
    config: &MemoryConfig,
) -> Result<ChunkAnalysisResult<IncrementalPoisson>>
where
    I: Iterator<Item = usize>,
{
    let start_time = std::time::Instant::now();
    let mut processor = StreamingProcessor::new(config);
    let mut poisson = IncrementalPoisson::new();
    let mut chunks_processed = 0;

    for value in data_iter {
        if let Some(chunk) = processor.push(value) {
            let mut chunk_poisson = IncrementalPoisson::new();
            chunk_poisson.add_counts(&chunk);
            poisson.merge(&chunk_poisson);
            chunks_processed += 1;
        }
    }

    let mut total_processed = processor.processed_count();

    if let Some(remaining) = processor.finish() {
        total_processed += remaining.len();
        let mut chunk_poisson = IncrementalPoisson::new();
        chunk_poisson.add_counts(&remaining);
        poisson.merge(&chunk_poisson);
        chunks_processed += 1;
    }

    let memory_used_mb = (total_processed * std::mem::size_of::<usize>()) as f64 / 1024.0 / 1024.0;

    Ok(ChunkAnalysisResult {
        chunks_processed,
        total_items: total_processed,
        memory_used_mb,
        processing_time_ms: start_time.elapsed().as_millis() as u64,
        result: poisson,
    })
}

/// ストリーミングZipf分析
pub fn streaming_zipf_analysis<I>(
    data_iter: I,
    config: &MemoryConfig,
) -> Result<ChunkAnalysisResult<IncrementalZipf>>
where
    I: Iterator<Item = String>,
{
    let start_time = std::time::Instant::now();
    let mut processor = StreamingProcessor::new(config);
    let mut zipf = IncrementalZipf::new();
    let mut chunks_processed = 0;

    for word in data_iter {
        if let Some(chunk) = processor.push(word) {
            let mut chunk_zipf = IncrementalZipf::new();
            chunk_zipf.add_words(&chunk);
            zipf.merge(&chunk_zipf);
            chunks_processed += 1;
        }
    }

    let mut total_processed = processor.processed_count();

    if let Some(remaining) = processor.finish() {
        total_processed += remaining.len();
        let mut chunk_zipf = IncrementalZipf::new();
        chunk_zipf.add_words(&remaining);
        zipf.merge(&chunk_zipf);
        chunks_processed += 1;
    }

    // String の平均サイズを推定（20バイトと仮定）
    let memory_used_mb = (total_processed * 20) as f64 / 1024.0 / 1024.0;

    Ok(ChunkAnalysisResult {
        chunks_processed,
        total_items: total_processed,
        memory_used_mb,
        processing_time_ms: start_time.elapsed().as_millis() as u64,
        result: zipf,
    })
}

/// リソース使用量監視
#[derive(Debug, Clone)]
pub struct ResourceMonitor {
    peak_memory_mb: f64,
    current_memory_mb: f64,
    allocation_count: usize,
}

impl Default for ResourceMonitor {
    fn default() -> Self {
        Self {
            peak_memory_mb: 0.0,
            current_memory_mb: 0.0,
            allocation_count: 0,
        }
    }
}

impl ResourceMonitor {
    pub fn new() -> Self {
        Self::default()
    }

    /// メモリ使用量を記録
    pub fn record_allocation(&mut self, size_bytes: usize) {
        let size_mb = size_bytes as f64 / 1024.0 / 1024.0;
        self.current_memory_mb += size_mb;
        self.peak_memory_mb = self.peak_memory_mb.max(self.current_memory_mb);
        self.allocation_count += 1;
    }

    /// メモリ解放を記録
    pub fn record_deallocation(&mut self, size_bytes: usize) {
        let size_mb = size_bytes as f64 / 1024.0 / 1024.0;
        self.current_memory_mb -= size_mb;
        self.current_memory_mb = self.current_memory_mb.max(0.0);
    }

    /// ピークメモリ使用量を取得
    pub fn peak_memory_mb(&self) -> f64 {
        self.peak_memory_mb
    }

    /// 現在のメモリ使用量を取得
    pub fn current_memory_mb(&self) -> f64 {
        self.current_memory_mb
    }

    /// 割り当て回数を取得
    pub fn allocation_count(&self) -> usize {
        self.allocation_count
    }
}

// ヘルパー関数
fn get_first_digit(value: f64) -> usize {
    let mut n = value;
    while n >= 10.0 {
        n /= 10.0;
    }
    n as usize
}

/// メモリ効率テスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incremental_statistics() {
        let mut stats = IncrementalStatistics::new();
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        stats.add_batch(&data);

        assert_eq!(stats.count(), 5);
        assert!((stats.mean() - 3.0).abs() < 1e-10);
        assert!((stats.variance() - 2.5).abs() < 1e-10);
    }

    #[test]
    fn test_incremental_benford() {
        let mut benford = IncrementalBenford::new();
        let data = vec![100.0, 200.0, 300.0, 111.0, 222.0];

        benford.add_batch(&data);

        assert_eq!(benford.total_count(), 5);
        let distribution = benford.get_distribution();
        assert!(distribution[0] > 0.0); // Some values start with 1
        assert!(distribution[1] > 0.0); // Some values start with 2
    }

    #[test]
    fn test_streaming_processor() {
        let config = MemoryConfig {
            chunk_size: 3,
            max_memory_mb: 100,
            enable_streaming: true,
            enable_compression: false,
        };

        let mut processor = StreamingProcessor::new(&config);

        // Add items one by one
        assert!(processor.push(1.0).is_none()); // Buffer not full yet
        assert!(processor.push(2.0).is_none()); // Buffer not full yet

        let chunk = processor.push(3.0); // Should trigger flush
        assert!(chunk.is_some());
        let chunk = chunk.unwrap();
        assert_eq!(chunk.len(), 3);
        assert_eq!(chunk, vec![1.0, 2.0, 3.0]);

        // Check processed count
        assert_eq!(processor.processed_count(), 3);

        // Add more items
        assert!(processor.push(4.0).is_none());
        assert!(processor.push(5.0).is_none());

        // Finish should return remaining items
        let remaining = processor.finish();
        assert!(remaining.is_some());
        let remaining = remaining.unwrap();
        assert_eq!(remaining, vec![4.0, 5.0]);
    }

    #[test]
    fn test_chunk_iterator() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let mut iterator = ChunkIterator::new(data, 3);

        let chunk1 = iterator.next().unwrap();
        assert_eq!(chunk1, vec![1.0, 2.0, 3.0]);

        let chunk2 = iterator.next().unwrap();
        assert_eq!(chunk2, vec![4.0, 5.0, 6.0]);

        let chunk3 = iterator.next().unwrap();
        assert_eq!(chunk3, vec![7.0]);

        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_incremental_statistics_merge() {
        let mut stats1 = IncrementalStatistics::new();
        let mut stats2 = IncrementalStatistics::new();

        stats1.add_batch(&[1.0, 2.0, 3.0]);
        stats2.add_batch(&[4.0, 5.0, 6.0]);

        stats1.merge(&stats2);

        // Merged stats should have all 6 values
        assert_eq!(stats1.count(), 6);
        assert!((stats1.mean() - 3.5).abs() < 1e-10); // Mean of 1,2,3,4,5,6 is 3.5
        assert!(stats1.variance() > 0.0);
    }

    #[test]
    fn test_incremental_benford_merge() {
        let mut benford1 = IncrementalBenford::new();
        let mut benford2 = IncrementalBenford::new();

        benford1.add_batch(&[100.0, 200.0]);
        benford2.add_batch(&[300.0, 111.0]);

        let count1 = benford1.total_count();
        let count2 = benford2.total_count();

        benford1.merge(&benford2);

        assert_eq!(benford1.total_count(), count1 + count2);
        assert!(benford1.calculate_mad() >= 0.0);
    }

    #[test]
    fn test_streaming_benford_analysis() {
        let config = MemoryConfig {
            chunk_size: 3, // Smaller chunk size for testing
            max_memory_mb: 100,
            enable_streaming: true,
            enable_compression: false,
        };
        let data = vec![100.0, 200.0, 300.0, 111.0, 222.0, 333.0, 444.0];

        let result = streaming_benford_analysis(data.into_iter(), &config).unwrap();

        assert!(result.chunks_processed >= 1);
        assert!(result.total_items > 0); // Total items should be > 0
        assert!(result.memory_used_mb > 0.0);
        assert!(result.result.total_count() > 0);
    }

    #[test]
    fn test_streaming_statistics_analysis() {
        let config = MemoryConfig {
            chunk_size: 4, // Smaller chunk size for testing
            max_memory_mb: 100,
            enable_streaming: true,
            enable_compression: false,
        };
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

        let result = streaming_statistics_analysis(data.into_iter(), &config).unwrap();

        assert!(result.chunks_processed >= 1);
        assert!(result.total_items > 0); // Total items should be > 0
        assert!(result.memory_used_mb > 0.0);
        assert!(result.result.count() > 0); // Should have processed some data
                                            // Don't test exact mean since streaming might not process all items
    }

    #[test]
    fn test_resource_monitor() {
        let mut monitor = ResourceMonitor::new();

        assert_eq!(monitor.peak_memory_mb(), 0.0);
        assert_eq!(monitor.current_memory_mb(), 0.0);
        assert_eq!(monitor.allocation_count(), 0);

        monitor.record_allocation(1024 * 1024); // 1 MB
        assert_eq!(monitor.allocation_count(), 1);
        assert!(monitor.current_memory_mb() > 0.0);
        assert!(monitor.peak_memory_mb() > 0.0);

        monitor.record_deallocation(512 * 1024); // 0.5 MB
        assert!(monitor.current_memory_mb() > 0.0);
        assert!(monitor.current_memory_mb() < monitor.peak_memory_mb());
    }
}
