# NORMAL_ARCHITECTURE.md - 正規分布アーキテクチャ設計

## アーキテクチャ概要

正規分布実装は、lawkit統合プラットフォームの第四の統計法則として設計。統計学で最も重要な分布である正規分布の包括的分析を提供し、データ品質評価・異常値検出・工程管理の実用的ツールとして機能する。

## モジュール構造

```
src/laws/normal/
├── mod.rs              # モジュール定義・公開API
├── result.rs           # NormalResult構造体・統計計算
└── analysis.rs         # 専用分析機能・品質管理

src/subcommands/
└── normal.rs           # CLI実装・マルチモード対応
```

## 核心データ構造

### NormalResult構造体
```rust
#[derive(Debug, Clone)]
pub struct NormalResult {
    // 基本情報
    pub dataset_name: String,
    pub numbers_analyzed: usize,
    pub risk_level: RiskLevel,
    
    // 分布パラメータ
    pub mean: f64,                    // 平均値
    pub std_dev: f64,                // 標準偏差
    pub variance: f64,               // 分散
    pub skewness: f64,               // 歪度（左右の偏り）
    pub kurtosis: f64,               // 尖度（分布の尖り）
    
    // 正規性検定結果
    pub shapiro_wilk_statistic: f64,      // Shapiro-Wilk検定統計量
    pub shapiro_wilk_p_value: f64,        // Shapiro-Wilk p値
    pub anderson_darling_statistic: f64,   // Anderson-Darling検定統計量
    pub anderson_darling_p_value: f64,     // Anderson-Darling p値
    pub kolmogorov_smirnov_statistic: f64, // Kolmogorov-Smirnov検定統計量
    pub kolmogorov_smirnov_p_value: f64,   // Kolmogorov-Smirnov p値
    
    // 適合度評価
    pub normality_score: f64,        // 正規性総合スコア（0-1）
    pub qq_correlation: f64,         // Q-Q plot相関係数
    pub distribution_quality: f64,   // 分布品質スコア
    
    // 異常値検出
    pub outliers_z_score: Vec<(usize, f64, f64)>,    // (インデックス, 値, Z-score)
    pub outliers_modified_z: Vec<(usize, f64, f64)>, // (インデックス, 値, 修正Z-score)
    pub outliers_iqr: Vec<(usize, f64)>,             // (インデックス, 値)
    
    // 信頼区間・範囲
    pub mean_confidence_interval: (f64, f64),        // 平均の95%信頼区間
    pub prediction_interval_95: (f64, f64),          // 95%予測区間
    pub three_sigma_limits: (f64, f64),              // 3σ限界
    
    // 品質管理指標
    pub within_1_sigma_percent: f64,    // 1σ以内の割合
    pub within_2_sigma_percent: f64,    // 2σ以内の割合
    pub within_3_sigma_percent: f64,    // 3σ以内の割合
}
```

## 核心アルゴリズム

### 1. 分布パラメータ計算
```rust
// 平均値計算
fn calculate_mean(numbers: &[f64]) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}

// 分散計算（不偏分散）
fn calculate_variance(numbers: &[f64], mean: f64) -> f64 {
    let sum_squared_diff: f64 = numbers.iter()
        .map(|&x| (x - mean).powi(2))
        .sum();
    sum_squared_diff / (numbers.len() - 1) as f64
}

// 歪度計算（標準化第3次モーメント）
fn calculate_skewness(numbers: &[f64], mean: f64, std_dev: f64) -> f64 {
    let n = numbers.len() as f64;
    let sum_cubed: f64 = numbers.iter()
        .map(|&x| ((x - mean) / std_dev).powi(3))
        .sum();
    sum_cubed / n
}

// 尖度計算（超過尖度：正規分布基準-3）
fn calculate_kurtosis(numbers: &[f64], mean: f64, std_dev: f64) -> f64 {
    let n = numbers.len() as f64;
    let sum_fourth: f64 = numbers.iter()
        .map(|&x| ((x - mean) / std_dev).powi(4))
        .sum();
    (sum_fourth / n) - 3.0
}
```

### 2. 正規性検定アルゴリズム

#### Shapiro-Wilk検定（簡易実装）
```rust
fn shapiro_wilk_test(numbers: &[f64]) -> (f64, f64) {
    let n = numbers.len();
    if n < 8 || n > 5000 {
        return (0.0, 1.0); // 適用範囲外
    }
    
    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    // Q-Q相関係数を利用した簡易W統計量
    let qq_corr = calculate_qq_correlation(&sorted, mean, std_dev);
    let w_statistic = qq_corr * qq_corr;
    
    // 簡易p値推定
    let p_value = match w_statistic {
        w if w > 0.95 => 0.5,
        w if w > 0.90 => 0.1,
        w if w > 0.80 => 0.01,
        _ => 0.001,
    };
    
    (w_statistic, p_value)
}
```

#### Anderson-Darling検定
```rust
fn anderson_darling_test(numbers: &[f64], mean: f64, std_dev: f64) -> (f64, f64) {
    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let n = sorted.len() as f64;
    let mut a_squared = 0.0;
    
    for (i, &x) in sorted.iter().enumerate() {
        let z = (x - mean) / std_dev;
        let phi = standard_normal_cdf(z);
        let phi_complement = 1.0 - standard_normal_cdf(-z);
        
        if phi > 0.0 && phi < 1.0 && phi_complement > 0.0 {
            a_squared += (2.0 * (i + 1) as f64 - 1.0) * 
                        (phi.ln() + phi_complement.ln()) / n;
        }
    }
    
    a_squared = -n - a_squared;
    
    // 臨界値との比較でp値推定
    let p_value = estimate_ad_p_value(a_squared);
    (a_squared, p_value)
}
```

#### Kolmogorov-Smirnov検定
```rust
fn kolmogorov_smirnov_test(numbers: &[f64], mean: f64, std_dev: f64) -> (f64, f64) {
    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let n = sorted.len() as f64;
    let mut max_diff = 0.0;
    
    for (i, &x) in sorted.iter().enumerate() {
        let z = (x - mean) / std_dev;
        let expected_cdf = standard_normal_cdf(z);
        let empirical_cdf = (i + 1) as f64 / n;
        
        let diff = (expected_cdf - empirical_cdf).abs();
        max_diff = max_diff.max(diff);
    }
    
    // Lilliefors補正を適用したKS統計量
    let ks_critical = 1.36 / n.sqrt();
    let p_value = estimate_ks_p_value(max_diff, ks_critical);
    
    (max_diff, p_value)
}
```

### 3. 標準正規分布関数

#### 累積分布関数（CDF）
```rust
fn standard_normal_cdf(z: f64) -> f64 {
    if z > 6.0 { return 1.0; }
    if z < -6.0 { return 0.0; }
    
    // Abramowitz and Stegun近似
    let t = 1.0 / (1.0 + 0.2316419 * z.abs());
    let d = 0.3989423 * (-z * z / 2.0).exp();
    let probability = d * t * (0.3193815 + 
        t * (-0.3565638 + 
        t * (1.7814779 + 
        t * (-1.8212560 + 
        t * 1.3302744))));
    
    if z >= 0.0 { 1.0 - probability } else { probability }
}
```

#### 逆関数（Quantile関数）
```rust
fn inverse_standard_normal(p: f64) -> f64 {
    if p <= 0.0 { return -6.0; }
    if p >= 1.0 { return 6.0; }
    
    // Beasley-Springer-Moro algorithm
    let a = p - 0.5;
    if a.abs() < 0.42 {
        let r = a * a;
        return a * (((2.5066282 + r * 0.3001648) + r * 0.0010805) / 
                   ((1.0 + r * 1.6372227) + r * 0.0312753));
    }
    
    let r = if a < 0.0 { p } else { 1.0 - p };
    let s = (-r.ln()).sqrt();
    let t = s - ((2.515517 + s * 0.802853) + s * s * 0.010328) / 
              ((1.0 + s * 1.432788) + s * s * 0.189269);
    
    if a < 0.0 { -t } else { t }
}
```

### 4. Q-Q Plot相関計算
```rust
fn calculate_qq_correlation(numbers: &[f64], mean: f64, std_dev: f64) -> f64 {
    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let n = sorted.len();
    let mut theoretical_quantiles = Vec::new();
    
    // 理論分位点計算
    for i in 0..n {
        let p = (i + 1) as f64 / (n + 1) as f64;
        let z = inverse_standard_normal(p);
        theoretical_quantiles.push(mean + std_dev * z);
    }
    
    // ピアソン相関係数計算
    pearson_correlation(&sorted, &theoretical_quantiles)
}
```

## 異常値検出エンジン

### Z-score法
```rust
fn detect_outliers_z_score(numbers: &[f64], mean: f64, std_dev: f64) -> Vec<(usize, f64, f64)> {
    let mut outliers = Vec::new();
    let threshold = 2.5; // 2.5σ閾値
    
    for (i, &value) in numbers.iter().enumerate() {
        let z_score = (value - mean) / std_dev;
        if z_score.abs() > threshold {
            outliers.push((i, value, z_score));
        }
    }
    outliers
}
```

### 修正Z-score法（MADベース）
```rust
fn detect_outliers_modified_z_score(numbers: &[f64]) -> Vec<(usize, f64, f64)> {
    let median = calculate_median(numbers);
    let mad = calculate_mad(numbers, median);
    let mut outliers = Vec::new();
    let threshold = 3.5;
    
    for (i, &value) in numbers.iter().enumerate() {
        let modified_z = 0.6745 * (value - median) / mad;
        if modified_z.abs() > threshold {
            outliers.push((i, value, modified_z));
        }
    }
    outliers
}

fn calculate_mad(numbers: &[f64], median: f64) -> f64 {
    let deviations: Vec<f64> = numbers.iter()
        .map(|&x| (x - median).abs())
        .collect();
    calculate_median(&deviations)
}
```

### IQR法
```rust
fn detect_outliers_iqr(numbers: &[f64]) -> Vec<(usize, f64)> {
    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let n = sorted.len();
    let q1 = sorted[n / 4];
    let q3 = sorted[3 * n / 4];
    let iqr = q3 - q1;
    let lower_bound = q1 - 1.5 * iqr;
    let upper_bound = q3 + 1.5 * iqr;
    
    let mut outliers = Vec::new();
    for (i, &value) in numbers.iter().enumerate() {
        if value < lower_bound || value > upper_bound {
            outliers.push((i, value));
        }
    }
    outliers
}
```

## 品質管理分析エンジン

### 工程能力指数計算
```rust
pub fn quality_control_analysis(numbers: &[f64], spec_limits: Option<(f64, f64)>) -> Result<QualityControlResult> {
    let result = NormalResult::new("quality_control".to_string(), numbers)?;
    
    let (cp, cpk, process_capability) = if let Some((lsl, usl)) = spec_limits {
        // Cp指数: 工程の精度能力
        let cp = (usl - lsl) / (6.0 * result.std_dev);
        
        // Cpk指数: 工程の総合能力
        let cpu = (usl - result.mean) / (3.0 * result.std_dev);
        let cpl = (result.mean - lsl) / (3.0 * result.std_dev);
        let cpk = cpu.min(cpl);
        
        // 工程能力評価
        let capability = match cpk {
            cpk if cpk >= 1.33 => ProcessCapability::Excellent,
            cpk if cpk >= 1.0 => ProcessCapability::Adequate,
            cpk if cpk >= 0.67 => ProcessCapability::Poor,
            _ => ProcessCapability::Inadequate,
        };
        
        (Some(cp), Some(cpk), Some(capability))
    } else {
        (None, None, None)
    };
    
    Ok(QualityControlResult {
        mean: result.mean,
        std_dev: result.std_dev,
        cp, cpk, process_capability,
        within_spec_percent: calculate_within_spec_percent(numbers, spec_limits),
        three_sigma_limits: result.three_sigma_limits,
        control_chart_violations: detect_control_chart_violations(numbers, result.mean, result.std_dev),
    })
}
```

### 管理図違反検出
```rust
fn detect_control_chart_violations(numbers: &[f64], mean: f64, std_dev: f64) -> Vec<ControlChartViolation> {
    let mut violations = Vec::new();
    let ucl = mean + 3.0 * std_dev; // 上方管理限界
    let lcl = mean - 3.0 * std_dev; // 下方管理限界
    let uwl = mean + 2.0 * std_dev; // 上方警告限界
    let lwl = mean - 2.0 * std_dev; // 下方警告限界
    
    // Rule 1: 管理限界外の点
    for (i, &value) in numbers.iter().enumerate() {
        if value > ucl || value < lcl {
            violations.push(ControlChartViolation {
                index: i,
                value,
                violation_type: ViolationType::OutOfControlLimits,
                description: "Point outside 3σ control limits".to_string(),
            });
        }
    }
    
    // Western Electric Rules の実装
    detect_run_violations(numbers, mean, std_dev, &mut violations);
    detect_trend_violations(numbers, &mut violations);
    
    violations
}

// Rule 2: 7点連続で中心線の片側
fn detect_run_violations(numbers: &[f64], mean: f64, _std_dev: f64, violations: &mut Vec<ControlChartViolation>) {
    let mut consecutive_above = 0;
    let mut consecutive_below = 0;
    
    for (i, &value) in numbers.iter().enumerate() {
        if value > mean {
            consecutive_above += 1;
            consecutive_below = 0;
            
            if consecutive_above >= 7 {
                violations.push(ControlChartViolation {
                    index: i,
                    value,
                    violation_type: ViolationType::RunAboveMean,
                    description: "7 consecutive points above mean".to_string(),
                });
            }
        } else if value < mean {
            consecutive_below += 1;
            consecutive_above = 0;
            
            if consecutive_below >= 7 {
                violations.push(ControlChartViolation {
                    index: i,
                    value,
                    violation_type: ViolationType::RunBelowMean,
                    description: "7 consecutive points below mean".to_string(),
                });
            }
        }
    }
}
```

## 専用分析API設計

### 正規性検定API
```rust
pub enum NormalityTest {
    ShapiroWilk,
    AndersonDarling,
    KolmogorovSmirnov,
    All, // 統合評価
}

pub struct NormalityTestResult {
    pub test_name: String,
    pub statistic: f64,
    pub p_value: f64,
    pub critical_value: f64,
    pub is_normal: bool,
}

pub fn test_normality(numbers: &[f64], test_type: NormalityTest) -> Result<NormalityTestResult>;
```

### 異常値検出API
```rust
pub enum OutlierDetectionMethod {
    ZScore,       // 2.5σ閾値
    ModifiedZScore, // MADベース、3.5閾値
    IQR,          // 四分位範囲、1.5倍
}

pub struct OutlierDetectionResult {
    pub method_name: String,
    pub outliers: Vec<OutlierInfo>,
    pub threshold: f64,
}

pub struct OutlierInfo {
    pub index: usize,
    pub value: f64,
    pub score: f64,    // Z-score等
    pub is_outlier: bool,
}

pub fn detect_outliers(numbers: &[f64], method: OutlierDetectionMethod) -> Result<OutlierDetectionResult>;
```

### 品質管理API
```rust
pub enum ProcessCapability {
    Excellent,  // Cpk >= 1.33
    Adequate,   // 1.0 <= Cpk < 1.33
    Poor,       // 0.67 <= Cpk < 1.0
    Inadequate, // Cpk < 0.67
}

pub struct QualityControlResult {
    pub mean: f64,
    pub std_dev: f64,
    pub cp: Option<f64>,
    pub cpk: Option<f64>,
    pub process_capability: Option<ProcessCapability>,
    pub within_spec_percent: Option<f64>,
    pub three_sigma_limits: (f64, f64),
    pub control_chart_violations: Vec<ControlChartViolation>,
}

pub fn quality_control_analysis(numbers: &[f64], spec_limits: Option<(f64, f64)>) -> Result<QualityControlResult>;
```

## CLI統合アーキテクチャ

### マルチモード処理
```rust
pub fn run(matches: &ArgMatches) -> Result<()> {
    // モード判定
    if let Some(test_type) = matches.get_one::<String>("test") {
        return run_normality_test_mode(matches, test_type);
    }
    
    if matches.get_flag("outliers") {
        return run_outlier_detection_mode(matches);
    }
    
    if matches.get_flag("quality-control") {
        return run_quality_control_mode(matches);
    }
    
    // デフォルト: 包括的正規分布分析
    run_comprehensive_analysis(matches)
}
```

### 規格限界パーサー
```rust
fn parse_spec_limits(limits_str: &str) -> Result<Option<(f64, f64)>> {
    let parts: Vec<&str> = limits_str.split(',').collect();
    if parts.len() != 2 {
        return Err(BenfError::ParseError("Spec limits format: 'lower,upper'".to_string()));
    }
    
    let lower = parts[0].trim().parse::<f64>()?;
    let upper = parts[1].trim().parse::<f64>()?;
    
    if lower >= upper {
        return Err(BenfError::ParseError("Lower limit must be less than upper limit".to_string()));
    }
    
    Ok(Some((lower, upper)))
}
```

## パフォーマンス設計

### 計算複雑度
- **基本統計**: O(n) - 一回のスキャンで計算
- **ソート処理**: O(n log n) - 正規性検定で必要
- **Q-Q相関**: O(n) - ソート後の線形処理
- **異常値検出**: O(n) - 線形スキャン
- **総合複雑度**: O(n log n) - ソートが支配的

### メモリ効率
```rust
impl NormalResult {
    pub fn new(dataset_name: String, numbers: &[f64]) -> Result<Self> {
        // 段階的計算でメモリ効率化
        if numbers.len() < 8 {
            return Err(BenfError::InsufficientData(numbers.len()));
        }
        
        // 基本統計（O(n)、一回のスキャン）
        let (mean, variance) = calculate_mean_variance_single_pass(numbers);
        let std_dev = variance.sqrt();
        
        // 必要時のみソート
        let sorted_for_tests = if needs_normality_tests() {
            Some(sort_numbers(numbers))
        } else {
            None
        };
        
        // 段階的構築
        build_result_incrementally(dataset_name, numbers, mean, std_dev, sorted_for_tests)
    }
}
```

### 数値安定性
- **分散計算**: Welford's online algorithm for numerical stability
- **正規化**: Z-score計算時のゼロ除算チェック
- **境界値**: 極値での安全な計算
- **精度保持**: 64bit浮動小数点での一貫した精度

## エラーハンドリング戦略

### データ検証
```rust
fn validate_input_data(numbers: &[f64]) -> Result<()> {
    if numbers.len() < 8 {
        return Err(BenfError::InsufficientData(numbers.len()));
    }
    
    if numbers.iter().all(|&x| x == numbers[0]) {
        return Err(BenfError::ParseError("All values are identical - zero variance".to_string()));
    }
    
    if numbers.iter().any(|x| x.is_nan() || x.is_infinite()) {
        return Err(BenfError::ParseError("NaN or infinite values detected".to_string()));
    }
    
    Ok(())
}
```

### グレースフルデグラデーション
- **検定失敗**: 一部検定が失敗しても他は継続
- **異常値なし**: 異常値が検出されなくても正常終了
- **規格限界なし**: Cp/Cpk計算不可でも基本分析は提供

## テスト設計

### 単体テスト
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_perfect_normal_distribution() {
        // Box-Muller法で生成した理想正規分布
        let numbers = generate_normal_distribution(0.0, 1.0, 100);
        let result = NormalResult::new("test".to_string(), &numbers).unwrap();
        
        assert!((result.mean - 0.0).abs() < 0.2);
        assert!((result.std_dev - 1.0).abs() < 0.2);
        assert!(result.normality_score > 0.7);
        assert!(matches!(result.risk_level, RiskLevel::Low | RiskLevel::Medium));
    }
    
    #[test]
    fn test_outlier_detection_accuracy() {
        let mut numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        numbers.push(100.0); // 明確な外れ値
        
        let result = NormalResult::new("outlier_test".to_string(), &numbers).unwrap();
        assert!(!result.outliers_z_score.is_empty());
        assert_eq!(result.outliers_z_score[0].1, 100.0); // 外れ値が検出される
    }
}
```

### 統合テスト
- **CLI全オプション**: 各モード・オプション組み合わせ
- **出力フォーマット**: JSON/CSV/XML等の妥当性
- **多言語対応**: 各言語での正確な出力
- **エラーケース**: 不正入力での適切なエラーハンドリング

## 将来拡張アーキテクチャ

### 高度な正規性検定
- **Jarque-Bera検定**: 歪度・尖度ベース
- **D'Agostino検定**: omnibus normality test
- **Lilliefors検定**: KS検定の改良版

### 多変量正規性
- **Mardia検定**: 多変量歪度・尖度
- **Henze-Zirkler検定**: 多変量正規性
- **Royston検定**: Shapiro-Wilkの多変量拡張

### ベイズ統計拡張
- **ベイズ因子**: モデル比較
- **事後分布**: パラメータ不確実性
- **予測分布**: 将来値の予測区間

### リアルタイム監視
- **ストリーミング分析**: リアルタイム品質監視
- **変化点検出**: 分布変化の自動検出
- **アラート機能**: 異常時の自動通知