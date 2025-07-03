# POISSON_ARCHITECTURE.md - ポアソン分布アーキテクチャ設計

## アーキテクチャ概要

ポアソン分布実装は、lawkit統合プラットフォームの第五の統計法則として設計。離散確率分布の中で最も重要なポアソン分布の包括的分析を提供し、イベント発生頻度・稀少事象分析・確率予測の実用的ツールとして機能する。

## モジュール構造

```
src/laws/poisson/
├── mod.rs              # モジュール定義・公開API
├── result.rs           # PoissonResult構造体・統計計算
└── analysis.rs         # 専用分析機能・確率予測

src/subcommands/
└── poisson.rs          # CLI実装・マルチモード対応
```

## 核心データ構造

### PoissonResult構造体
```rust
#[derive(Debug, Clone)]
pub struct PoissonResult {
    // 基本情報
    pub dataset_name: String,
    pub numbers_analyzed: usize,
    pub risk_level: RiskLevel,
    
    // ポアソン分布パラメータ
    pub lambda: f64,                    // 平均発生率（λ）
    pub sample_mean: f64,               // 標本平均
    pub sample_variance: f64,           // 標本分散
    pub variance_ratio: f64,            // 分散/平均比（1に近いほどポアソン分布に適合）
    
    // 適合度検定
    pub chi_square_statistic: f64,      // カイ二乗検定統計量
    pub chi_square_p_value: f64,        // カイ二乗検定p値
    pub kolmogorov_smirnov_statistic: f64, // KS検定統計量
    pub kolmogorov_smirnov_p_value: f64,   // KS検定p値
    
    // 適合度評価
    pub goodness_of_fit_score: f64,     // 適合度総合スコア（0-1）
    pub poisson_quality: f64,           // ポアソン性品質スコア
    pub distribution_assessment: PoissonAssessment,
    
    // 発生頻度分析
    pub frequency_distribution: HashMap<u32, u32>, // 値 -> 出現回数
    pub expected_frequencies: HashMap<u32, f64>,   // 理論期待値
    pub rare_events_threshold: u32,     // 稀少事象の閾値
    pub rare_events_count: usize,       // 稀少事象の数
    
    // 予測・推定
    pub probability_zero: f64,          // 発生確率0の確率
    pub probability_one: f64,           // 発生確率1の確率
    pub probability_two_or_more: f64,   // 2回以上発生の確率
    pub confidence_interval_lambda: (f64, f64), // λの95%信頼区間
    
    // 時系列特性（将来拡張）
    pub mean_time_between_events: Option<f64>,   // 平均発生間隔
    pub exponential_fit_quality: Option<f64>,    // 指数分布適合度
    pub is_homogeneous_process: Option<bool>,    // 斉次過程かどうか
}
```

### ポアソン分布適合度評価
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum PoissonAssessment {
    Excellent,      // 優秀なポアソン適合
    Good,           // 良好なポアソン適合
    Moderate,       // 中程度のポアソン適合
    Poor,           // 不十分なポアソン適合
    NonPoisson,     // ポアソン分布でない
}
```

## 核心アルゴリズム

### 1. ポアソン確率計算（数値安定性重視）
```rust
/// ポアソン確率計算 P(X = k) = (λ^k * e^(-λ)) / k!
fn poisson_probability(k: u32, lambda: f64) -> f64 {
    if lambda <= 0.0 {
        return if k == 0 { 1.0 } else { 0.0 };
    }
    
    // 対数スケールで計算（数値安定性のため）
    let ln_prob = k as f64 * lambda.ln() - lambda - ln_factorial(k);
    ln_prob.exp()
}

/// 対数階乗計算（Stirlingの近似使用）
fn ln_factorial(n: u32) -> f64 {
    if n <= 1 {
        return 0.0;
    }
    
    if n > 10 {
        // Stirlingの近似: ln(n!) ≈ n*ln(n) - n + 0.5*ln(2*π*n)
        let n_f = n as f64;
        n_f * n_f.ln() - n_f + 0.5 * (2.0 * std::f64::consts::PI * n_f).ln()
    } else {
        // 小さな値は直接計算
        (2..=n).map(|i| (i as f64).ln()).sum()
    }
}
```

### 2. λパラメータ推定
```rust
/// 最尤推定法による λ 推定
fn estimate_lambda_mle(numbers: &[f64]) -> f64 {
    // λ̂ = x̄ （標本平均）
    numbers.iter().sum::<f64>() / numbers.len() as f64
}

/// λの信頼区間計算（大標本近似）
fn calculate_lambda_confidence_interval(sample_mean: f64, sample_size: usize) -> (f64, f64) {
    // 大標本近似: λ ± 1.96 * √(λ/n)
    let std_error = (sample_mean / sample_size as f64).sqrt();
    let margin = 1.96 * std_error;
    
    ((sample_mean - margin).max(0.0), sample_mean + margin)
}
```

### 3. 頻度分布分析
```rust
/// 観測頻度分布作成
fn create_frequency_distribution(event_counts: &[u32]) -> HashMap<u32, u32> {
    let mut freq_dist = HashMap::new();
    for &count in event_counts {
        *freq_dist.entry(count).or_insert(0) += 1;
    }
    freq_dist
}

/// 理論期待頻度計算
fn calculate_expected_frequencies(lambda: f64, sample_size: usize, observed: &HashMap<u32, u32>) -> HashMap<u32, f64> {
    let mut expected = HashMap::new();
    let max_value = *observed.keys().max().unwrap_or(&0);
    
    for k in 0..=max_value {
        let probability = poisson_probability(k, lambda);
        expected.insert(k, probability * sample_size as f64);
    }
    
    expected
}
```

### 4. 適合度検定アルゴリズム

#### カイ二乗適合度検定
```rust
fn chi_square_goodness_of_fit_test(
    observed: &HashMap<u32, u32>,
    expected: &HashMap<u32, f64>
) -> (f64, f64) {
    let mut chi_square = 0.0;
    let mut degrees_of_freedom: i32 = 0;
    
    for (&k, &obs_freq) in observed {
        if let Some(&exp_freq) = expected.get(&k) {
            if exp_freq >= 5.0 {  // 期待度数5以上の階級のみ使用
                let diff = obs_freq as f64 - exp_freq;
                chi_square += (diff * diff) / exp_freq;
                degrees_of_freedom += 1;
            }
        }
    }
    
    degrees_of_freedom = degrees_of_freedom.saturating_sub(2); // パラメータ数（λ）を考慮
    
    // 簡易p値推定
    let p_value = estimate_chi_square_p_value(chi_square, degrees_of_freedom);
    
    (chi_square, p_value)
}
```

#### Kolmogorov-Smirnov検定（ポアソン分布用）
```rust
fn kolmogorov_smirnov_poisson_test(event_counts: &[u32], lambda: f64) -> (f64, f64) {
    let mut sorted_counts = event_counts.to_vec();
    sorted_counts.sort();
    
    let n = sorted_counts.len() as f64;
    let mut max_diff: f64 = 0.0;
    
    for (i, &k) in sorted_counts.iter().enumerate() {
        // 理論累積分布関数
        let theoretical_cdf = poisson_cdf(k, lambda);
        
        // 経験累積分布関数
        let empirical_cdf = (i + 1) as f64 / n;
        
        let diff = (theoretical_cdf - empirical_cdf).abs();
        max_diff = max_diff.max(diff);
    }
    
    // 簡易p値推定
    let critical_value = 1.36 / n.sqrt();
    let p_value = if max_diff > critical_value { 0.01 } else { 0.1 };
    
    (max_diff, p_value)
}

/// ポアソン分布累積分布関数
fn poisson_cdf(k: u32, lambda: f64) -> f64 {
    let mut cdf = 0.0;
    for i in 0..=k {
        cdf += poisson_probability(i, lambda);
    }
    cdf
}
```

#### 分散/平均比検定
```rust
fn variance_mean_ratio_test(numbers: &[f64]) -> (f64, f64) {
    let sample_mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
    let sample_variance = calculate_variance(numbers, sample_mean);
    let variance_ratio = sample_variance / sample_mean;
    
    // インデックス分散検定の簡易版
    // H0: ratio = 1 (ポアソン分布)
    let test_statistic = (variance_ratio - 1.0) * (numbers.len() as f64).sqrt();
    
    // 正規近似でp値推定
    let abs_stat = test_statistic.abs();
    let p_value = if abs_stat > 2.58 { 0.01 }
                  else if abs_stat > 1.96 { 0.05 }
                  else if abs_stat > 1.64 { 0.1 }
                  else { 0.5 };
    
    (variance_ratio, p_value)
}
```

### 5. 適合度スコア算出
```rust
/// 総合適合度スコア計算
fn calculate_goodness_of_fit_score(variance_ratio: f64, chi_square_p: f64, ks_p: f64) -> f64 {
    // 分散/平均比が1に近いほど高スコア
    let variance_score = if variance_ratio > 0.0 {
        let ratio_deviation = (variance_ratio - 1.0).abs();
        (1.0 / (1.0 + ratio_deviation)).max(0.0)
    } else {
        0.0
    };
    
    // p値が高いほど高スコア
    let p_value_score = (chi_square_p + ks_p) / 2.0;
    
    // 総合スコア（分散比重視）
    (variance_score * 0.6 + p_value_score * 0.4).min(1.0)
}

/// ポアソン品質スコア計算
fn calculate_poisson_quality_score(variance_ratio: f64, mean: f64) -> f64 {
    let ratio_quality = if variance_ratio > 0.0 {
        let deviation = (variance_ratio - 1.0).abs();
        (1.0 / (1.0 + 2.0 * deviation)).max(0.0)
    } else {
        0.0
    };
    
    // 平均が適度な値（0.1-10）の場合にボーナス
    let mean_quality = if mean >= 0.1 && mean <= 10.0 {
        1.0
    } else if mean > 10.0 {
        (10.0 / mean).min(1.0)
    } else {
        mean / 0.1
    };
    
    (ratio_quality * 0.8 + mean_quality * 0.2).min(1.0)
}
```

## 確率予測エンジン

### イベント発生確率予測
```rust
/// イベント発生確率予測
pub fn predict_event_probabilities(lambda: f64, max_events: u32) -> EventProbabilityResult {
    let mut probabilities = Vec::new();
    let mut cumulative = 0.0;
    
    for k in 0..=max_events {
        let prob = poisson_probability(k, lambda);
        cumulative += prob;
        
        probabilities.push(EventProbability {
            event_count: k,
            probability: prob,
            cumulative_probability: cumulative,
        });
    }
    
    EventProbabilityResult {
        lambda,
        max_events,
        probabilities,
        tail_probability: 1.0 - cumulative,
        most_likely_count: find_mode(lambda),
        expected_value: lambda,
        variance: lambda,
    }
}

/// ポアソン分布の最頻値計算
fn find_mode(lambda: f64) -> u32 {
    // ポアソン分布の最頻値は floor(λ) または floor(λ) + 1
    let floor_lambda = lambda.floor() as u32;
    let prob_floor = poisson_probability(floor_lambda, lambda);
    let prob_floor_plus1 = poisson_probability(floor_lambda + 1, lambda);
    
    if prob_floor >= prob_floor_plus1 {
        floor_lambda
    } else {
        floor_lambda + 1
    }
}
```

### 分位点計算（逆引き）
```rust
/// ポアソン分布分位点計算
fn poisson_quantile(p: f64, lambda: f64) -> u32 {
    // 累積分布関数の逆関数（近似）
    let mut k = 0;
    let mut cumulative = 0.0;
    
    while cumulative < p {
        cumulative += poisson_probability(k, lambda);
        if cumulative < p {
            k += 1;
        }
        
        if k > 1000 { // 無限ループ防止
            break;
        }
    }
    
    k
}
```

## 稀少事象分析エンジン

### 稀少事象検出
```rust
/// 稀少事象分析
pub fn analyze_rare_events(numbers: &[f64], lambda: f64) -> RareEventAnalysis {
    let event_counts: Vec<u32> = numbers.iter().map(|&x| x as u32).collect();
    
    // 稀少事象の定義（分位点ベース）
    let threshold_95 = poisson_quantile(0.95, lambda);
    let threshold_99 = poisson_quantile(0.99, lambda);
    let threshold_999 = poisson_quantile(0.999, lambda);
    
    // 各閾値での稀少事象カウント
    let rare_95 = event_counts.iter().filter(|&&x| x >= threshold_95).count();
    let rare_99 = event_counts.iter().filter(|&&x| x >= threshold_99).count();
    let rare_999 = event_counts.iter().filter(|&&x| x >= threshold_999).count();
    
    // 極端事象の詳細分析
    let extreme_events: Vec<ExtremeEvent> = event_counts.iter().enumerate()
        .filter(|&(_, &count)| count >= threshold_99)
        .map(|(index, &count)| ExtremeEvent {
            index,
            event_count: count,
            probability: poisson_probability(count, lambda),
            rarity_level: classify_rarity_level(count, threshold_99, threshold_999),
        })
        .collect();
    
    RareEventAnalysis {
        lambda,
        total_observations: numbers.len(),
        threshold_95,
        threshold_99,
        threshold_999,
        rare_events_95: rare_95,
        rare_events_99: rare_99,
        rare_events_999: rare_999,
        extreme_events,
        expected_rare_99: (numbers.len() as f64 * 0.01) as usize,
        clustering_detected: detect_clustering(&event_counts, threshold_99),
    }
}

/// 稀少度レベル分類
fn classify_rarity_level(count: u32, threshold_99: u32, threshold_999: u32) -> RarityLevel {
    if count >= threshold_999 { 
        RarityLevel::ExtremelyRare 
    } else if count >= threshold_99 { 
        RarityLevel::VeryRare 
    } else { 
        RarityLevel::Rare 
    }
}

/// イベントクラスタリング検出
fn detect_clustering(event_counts: &[u32], threshold: u32) -> bool {
    // 連続する稀少事象の検出
    let mut consecutive_rare = 0;
    let mut max_consecutive = 0;
    
    for &count in event_counts {
        if count >= threshold {
            consecutive_rare += 1;
            max_consecutive = max_consecutive.max(consecutive_rare);
        } else {
            consecutive_rare = 0;
        }
    }
    
    // 2個以上連続で稀少事象が発生した場合をクラスタリングとみなす
    max_consecutive >= 2
}
```

## 専用分析API設計

### ポアソン検定API
```rust
pub enum PoissonTest {
    ChiSquare,          // カイ二乗適合度検定
    KolmogorovSmirnov,  // KS検定
    VarianceTest,       // 分散/平均比検定
    All,                // 統合検定
}

pub struct PoissonTestResult {
    pub test_name: String,
    pub statistic: f64,
    pub p_value: f64,
    pub critical_value: f64,
    pub is_poisson: bool,
    pub parameter_lambda: f64,
}

pub fn test_poisson_fit(numbers: &[f64], test_type: PoissonTest) -> Result<PoissonTestResult>;
```

### イベント確率予測API
```rust
pub struct EventProbability {
    pub event_count: u32,
    pub probability: f64,
    pub cumulative_probability: f64,
}

pub struct EventProbabilityResult {
    pub lambda: f64,
    pub max_events: u32,
    pub probabilities: Vec<EventProbability>,
    pub tail_probability: f64,
    pub most_likely_count: u32,
    pub expected_value: f64,
    pub variance: f64,
}

pub fn predict_event_probabilities(lambda: f64, max_events: u32) -> EventProbabilityResult;
```

### 稀少事象分析API
```rust
pub enum RarityLevel {
    Rare,           // 稀（5%以下）
    VeryRare,       // 非常に稀（1%以下）
    ExtremelyRare,  // 極稀（0.1%以下）
}

pub struct ExtremeEvent {
    pub index: usize,
    pub event_count: u32,
    pub probability: f64,
    pub rarity_level: RarityLevel,
}

pub struct RareEventAnalysis {
    pub lambda: f64,
    pub total_observations: usize,
    pub threshold_95: u32,
    pub threshold_99: u32,
    pub threshold_999: u32,
    pub rare_events_95: usize,
    pub rare_events_99: usize,
    pub rare_events_999: usize,
    pub extreme_events: Vec<ExtremeEvent>,
    pub expected_rare_99: usize,
    pub clustering_detected: bool,
}

pub fn analyze_rare_events(numbers: &[f64], lambda: f64) -> RareEventAnalysis;
```

## CLI統合アーキテクチャ

### マルチモード処理
```rust
pub fn run(matches: &ArgMatches) -> Result<()> {
    // モード判定
    if let Some(test_type) = matches.get_one::<String>("test") {
        return run_poisson_test_mode(matches, test_type);
    }
    
    if matches.get_flag("predict") {
        return run_prediction_mode(matches);
    }
    
    if matches.get_flag("rare-events") {
        return run_rare_events_mode(matches);
    }
    
    // デフォルト: 包括的ポアソン分布分析
    run_comprehensive_analysis(matches)
}

fn run_poisson_test_mode(matches: &ArgMatches, test_type: &str) -> Result<()> {
    let test = match test_type {
        "chi-square" => PoissonTest::ChiSquare,
        "ks" => PoissonTest::KolmogorovSmirnov,
        "variance" => PoissonTest::VarianceTest,
        "all" => PoissonTest::All,
        _ => return Err(BenfError::ParseError("Invalid test type".to_string())),
    };
    
    // 検定実行・結果出力
    let numbers = get_numbers_from_input(matches)?;
    let test_result = test_poisson_fit(&numbers, test)?;
    output_poisson_test_result(matches, &test_result);
    
    Ok(())
}
```

### 入力データ検証
```rust
fn validate_poisson_input(numbers: &[f64]) -> Result<Vec<u32>> {
    if numbers.len() < 10 {
        return Err(BenfError::InsufficientData(numbers.len()));
    }
    
    let mut event_counts: Vec<u32> = Vec::new();
    for &num in numbers {
        // 非負整数値チェック
        if num < 0.0 || num.fract() != 0.0 {
            return Err(BenfError::ParseError(
                "ポアソン分布分析には非負整数値が必要です".to_string()
            ));
        }
        event_counts.push(num as u32);
    }
    
    Ok(event_counts)
}
```

## パフォーマンス設計

### 計算複雑度
- **基本統計**: O(n) - 平均・分散計算
- **頻度分布**: O(n) - ハッシュマップ構築
- **確率計算**: O(k) - 最大イベント数に比例
- **検定処理**: O(n log n) - ソート処理
- **総合複雑度**: O(n log n)

### メモリ効率化
```rust
impl PoissonResult {
    pub fn new(dataset_name: String, numbers: &[f64]) -> Result<Self> {
        // 入力検証
        let event_counts = validate_poisson_input(numbers)?;
        
        // 基本統計（O(n)）
        let sample_mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
        let lambda = sample_mean; // MLE推定
        
        // 頻度分布作成（メモリ効率重視）
        let frequency_distribution = create_frequency_distribution(&event_counts);
        
        // 段階的構築（メモリ使用量最小化）
        build_result_incrementally(dataset_name, numbers, lambda, frequency_distribution)
    }
}
```

### 数値安定性
- **階乗計算**: Stirlingの近似・対数スケール計算
- **確率計算**: アンダーフロー・オーバーフロー対策
- **累積計算**: 精度保持・誤差蓄積防止
- **分位点計算**: 収束判定・無限ループ防止

## エラーハンドリング戦略

### 入力データ検証
```rust
fn validate_input_data(numbers: &[f64]) -> Result<()> {
    if numbers.len() < 10 {
        return Err(BenfError::InsufficientData(numbers.len()));
    }
    
    // 非負整数チェック
    for &num in numbers {
        if num < 0.0 || num.fract() != 0.0 {
            return Err(BenfError::ParseError(
                "ポアソン分布には非負整数が必要".to_string()
            ));
        }
    }
    
    // 全て同じ値のチェック
    if numbers.iter().all(|&x| x == numbers[0]) {
        return Err(BenfError::ParseError(
            "全て同じ値 - 分散が0".to_string()
        ));
    }
    
    Ok(())
}
```

### グレースフルデグラデーション
- **検定失敗**: 一部検定が失敗しても他は継続
- **稀少事象なし**: 稀少事象が検出されなくても正常終了
- **大きなλ値**: 近似計算への自動切り替え

## テスト設計

### 単体テスト
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poisson_probability() {
        let lambda = 2.0;
        let prob_0 = poisson_probability(0, lambda);
        let prob_1 = poisson_probability(1, lambda);
        
        // P(X=0) = e^(-2) ≈ 0.135
        assert!((prob_0 - 0.135).abs() < 0.01);
        // P(X=1) = 2*e^(-2) ≈ 0.271  
        assert!((prob_1 - 0.271).abs() < 0.01);
    }
    
    #[test]
    fn test_perfect_poisson_distribution() {
        // 理論的ポアソン分布データ生成
        let numbers = generate_poisson_sample(1.5, 100);
        let result = PoissonResult::new("test".to_string(), &numbers).unwrap();
        
        assert!((result.lambda - 1.5).abs() < 0.3);
        assert!(result.variance_ratio > 0.7 && result.variance_ratio < 1.3);
        assert!(result.goodness_of_fit_score > 0.5);
    }
    
    #[test]
    fn test_overdispersed_data() {
        // 過分散データ（負の二項分布等）
        let numbers = vec![0.0, 1.0, 2.0, 10.0, 15.0, 0.0, 1.0, 20.0];
        let result = PoissonResult::new("overdispersed".to_string(), &numbers).unwrap();
        
        assert!(result.variance_ratio > 1.5); // 過分散検出
        assert!(matches!(result.distribution_assessment, PoissonAssessment::Poor | PoissonAssessment::NonPoisson));
    }
}
```

### 統合テスト
- **CLI全オプション**: 各モード・オプション組み合わせ
- **確率予測**: 数学的正確性・境界値処理
- **稀少事象**: 閾値計算・クラスタリング検出
- **多言語出力**: 各言語での適切な表示

## 将来拡張アーキテクチャ

### 高度なポアソンモデル
- **複合ポアソン過程**: 重複・重なりイベント
- **非斉次ポアソン過程**: 時変λ（時間依存）
- **空間ポアソン過程**: 2次元・3次元空間分析
- **ベイズポアソン**: 事前分布・事後分布

### 時系列・動的分析
- **到着間隔**: 指数分布適合・メモリレス性検定
- **変化点検出**: λの変化時点特定
- **トレンド分析**: 発生率の時間的変化
- **季節性**: 周期的パターン検出

### リアルタイム監視
- **ストリーミング分析**: オンライン更新・監視
- **アラート機能**: 異常パターン自動検出
- **ダッシュボード**: リアルタイム可視化
- **API化**: RESTful・gRPC対応

この設計により、ポアソン分布分析は**科学的厳密性**と**実用的汎用性**を両立した、lawkitプラットフォームの重要な構成要素として機能している。