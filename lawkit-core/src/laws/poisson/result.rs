use crate::common::risk::RiskLevel;
use crate::error::{BenfError, Result};
use std::collections::HashMap;

/// ポアソン分布分析結果
#[derive(Debug, Clone)]
pub struct PoissonResult {
    pub dataset_name: String,
    pub numbers_analyzed: usize,
    pub risk_level: RiskLevel,

    // ポアソン分布パラメータ
    pub lambda: f64,          // 平均発生率（λ）
    pub sample_mean: f64,     // 標本平均
    pub sample_variance: f64, // 標本分散
    pub variance_ratio: f64,  // 分散/平均比（1に近いほどポアソン分布に適合）

    // 適合度検定
    pub chi_square_statistic: f64,         // カイ二乗検定統計量
    pub chi_square_p_value: f64,           // カイ二乗検定p値
    pub kolmogorov_smirnov_statistic: f64, // KS検定統計量
    pub kolmogorov_smirnov_p_value: f64,   // KS検定p値

    // 適合度評価
    pub goodness_of_fit_score: f64, // 適合度総合スコア（0-1）
    pub poisson_quality: f64,       // ポアソン性品質スコア
    pub distribution_assessment: PoissonAssessment,

    // 発生頻度分析
    pub frequency_distribution: HashMap<u32, u32>, // 値 -> 出現回数
    pub expected_frequencies: HashMap<u32, f64>,   // 理論期待値
    pub rare_events_threshold: u32,                // 稀少事象の閾値
    pub rare_events_count: usize,                  // 稀少事象の数

    // 予測・推定
    pub probability_zero: f64,                  // 発生確率0の確率
    pub probability_one: f64,                   // 発生確率1の確率
    pub probability_two_or_more: f64,           // 2回以上発生の確率
    pub confidence_interval_lambda: (f64, f64), // λの95%信頼区間

    // 時系列特性（時間間隔データの場合）
    pub mean_time_between_events: Option<f64>, // 平均発生間隔
    pub exponential_fit_quality: Option<f64>,  // 指数分布適合度
    pub is_homogeneous_process: Option<bool>,  // 斉次過程かどうか
}

/// ポアソン分布適合度評価
#[derive(Debug, Clone, PartialEq)]
pub enum PoissonAssessment {
    Excellent,  // 優秀なポアソン適合
    Good,       // 良好なポアソン適合
    Moderate,   // 中程度のポアソン適合
    Poor,       // 不十分なポアソン適合
    NonPoisson, // ポアソン分布でない
}

impl PoissonResult {
    pub fn new(dataset_name: String, numbers: &[f64]) -> Result<Self> {
        if numbers.len() < 10 {
            return Err(BenfError::InsufficientData(numbers.len()));
        }

        // 非負整数値チェック
        let mut event_counts: Vec<u32> = Vec::new();
        for &num in numbers {
            if num < 0.0 || num.fract() != 0.0 {
                return Err(BenfError::ParseError(
                    "ポアソン分布分析には非負整数値が必要です".to_string(),
                ));
            }
            event_counts.push(num as u32);
        }

        // 基本統計計算
        let sample_mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
        let sample_variance = calculate_variance(numbers, sample_mean);
        let variance_ratio = sample_variance / sample_mean;
        let lambda = sample_mean; // MLE推定値

        // 頻度分布作成
        let frequency_distribution = create_frequency_distribution(&event_counts);
        let expected_frequencies =
            calculate_expected_frequencies(lambda, numbers.len(), &frequency_distribution);

        // 適合度検定
        let (chi_square_statistic, chi_square_p_value) =
            chi_square_goodness_of_fit_test(&frequency_distribution, &expected_frequencies);

        let (ks_statistic, ks_p_value) = kolmogorov_smirnov_poisson_test(&event_counts, lambda);

        // 適合度評価
        let goodness_of_fit_score =
            calculate_goodness_of_fit_score(variance_ratio, chi_square_p_value, ks_p_value);
        let poisson_quality = calculate_poisson_quality_score(variance_ratio, sample_mean);
        let distribution_assessment =
            assess_poisson_distribution(goodness_of_fit_score, variance_ratio);

        // リスク評価
        let risk_level = determine_risk_level(goodness_of_fit_score, &distribution_assessment);

        // 稀少事象分析
        let rare_events_threshold = calculate_rare_events_threshold(lambda);
        let rare_events_count = event_counts
            .iter()
            .filter(|&&x| x >= rare_events_threshold)
            .count();

        // 確率計算
        let probability_zero = poisson_probability(0, lambda);
        let probability_one = poisson_probability(1, lambda);
        let probability_two_or_more = 1.0 - probability_zero - probability_one;

        // 信頼区間
        let confidence_interval_lambda =
            calculate_lambda_confidence_interval(sample_mean, numbers.len());

        Ok(PoissonResult {
            dataset_name,
            numbers_analyzed: numbers.len(),
            risk_level,
            lambda,
            sample_mean,
            sample_variance,
            variance_ratio,
            chi_square_statistic,
            chi_square_p_value,
            kolmogorov_smirnov_statistic: ks_statistic,
            kolmogorov_smirnov_p_value: ks_p_value,
            goodness_of_fit_score,
            poisson_quality,
            distribution_assessment,
            frequency_distribution,
            expected_frequencies,
            rare_events_threshold,
            rare_events_count,
            probability_zero,
            probability_one,
            probability_two_or_more,
            confidence_interval_lambda,
            mean_time_between_events: None, // 時系列分析は将来実装
            exponential_fit_quality: None,
            is_homogeneous_process: None,
        })
    }
}

/// 分散計算
fn calculate_variance(numbers: &[f64], mean: f64) -> f64 {
    let sum_squared_diff: f64 = numbers.iter().map(|&x| (x - mean).powi(2)).sum();
    sum_squared_diff / (numbers.len() - 1) as f64
}

/// 頻度分布作成
fn create_frequency_distribution(event_counts: &[u32]) -> HashMap<u32, u32> {
    let mut freq_dist = HashMap::new();
    for &count in event_counts {
        *freq_dist.entry(count).or_insert(0) += 1;
    }
    freq_dist
}

/// 期待頻度計算
fn calculate_expected_frequencies(
    lambda: f64,
    sample_size: usize,
    observed: &HashMap<u32, u32>,
) -> HashMap<u32, f64> {
    let mut expected = HashMap::new();
    let max_value = *observed.keys().max().unwrap_or(&0);

    for k in 0..=max_value {
        let probability = poisson_probability(k, lambda);
        expected.insert(k, probability * sample_size as f64);
    }

    expected
}

/// ポアソン確率計算 P(X = k) = (λ^k * e^(-λ)) / k!
fn poisson_probability(k: u32, lambda: f64) -> f64 {
    if lambda <= 0.0 {
        return if k == 0 { 1.0 } else { 0.0 };
    }

    let ln_prob = k as f64 * lambda.ln() - lambda - ln_factorial(k);
    ln_prob.exp()
}

/// 対数階乗計算（数値安定性のため）
fn ln_factorial(n: u32) -> f64 {
    if n <= 1 {
        return 0.0;
    }

    // Stirlingの近似 ln(n!) ≈ n*ln(n) - n + 0.5*ln(2*π*n)
    if n > 10 {
        let n_f = n as f64;
        n_f * n_f.ln() - n_f + 0.5 * (2.0 * std::f64::consts::PI * n_f).ln()
    } else {
        // 小さな値は直接計算
        (2..=n).map(|i| (i as f64).ln()).sum()
    }
}

/// カイ二乗適合度検定
fn chi_square_goodness_of_fit_test(
    observed: &HashMap<u32, u32>,
    expected: &HashMap<u32, f64>,
) -> (f64, f64) {
    let mut chi_square = 0.0;
    let mut degrees_of_freedom: i32 = 0;

    for (&k, &obs_freq) in observed {
        if let Some(&exp_freq) = expected.get(&k) {
            if exp_freq >= 5.0 {
                // 期待度数5以上の階級のみ使用
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

/// KS検定（ポアソン分布用）
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

/// 適合度スコア計算
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

    // 総合スコア
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
    let mean_quality = if (0.1..=10.0).contains(&mean) {
        1.0
    } else if mean > 10.0 {
        (10.0 / mean).min(1.0)
    } else {
        mean / 0.1
    };

    (ratio_quality * 0.8 + mean_quality * 0.2).min(1.0)
}

/// ポアソン分布評価
fn assess_poisson_distribution(_goodness_score: f64, variance_ratio: f64) -> PoissonAssessment {
    let ratio_deviation = (variance_ratio - 1.0).abs();

    match (_goodness_score, ratio_deviation) {
        (s, d) if s > 0.8 && d < 0.2 => PoissonAssessment::Excellent,
        (s, d) if s > 0.6 && d < 0.5 => PoissonAssessment::Good,
        (s, d) if s > 0.4 && d < 1.0 => PoissonAssessment::Moderate,
        (s, d) if s > 0.2 && d < 2.0 => PoissonAssessment::Poor,
        _ => PoissonAssessment::NonPoisson,
    }
}

/// リスク評価
fn determine_risk_level(_goodness_score: f64, assessment: &PoissonAssessment) -> RiskLevel {
    match assessment {
        PoissonAssessment::Excellent => RiskLevel::Low,
        PoissonAssessment::Good => RiskLevel::Low,
        PoissonAssessment::Moderate => RiskLevel::Medium,
        PoissonAssessment::Poor => RiskLevel::High,
        PoissonAssessment::NonPoisson => RiskLevel::Critical,
    }
}

/// 稀少事象閾値計算
fn calculate_rare_events_threshold(lambda: f64) -> u32 {
    // λ + 3√λ を閾値とする（99.7%以上の事象を稀少とする）
    (lambda + 3.0 * lambda.sqrt()).ceil() as u32
}

/// λの信頼区間計算
fn calculate_lambda_confidence_interval(sample_mean: f64, sample_size: usize) -> (f64, f64) {
    // 大標本近似: λ ± 1.96 * √(λ/n)
    let std_error = (sample_mean / sample_size as f64).sqrt();
    let margin = 1.96 * std_error;

    ((sample_mean - margin).max(0.0), sample_mean + margin)
}

/// カイ二乗分布p値簡易推定
fn estimate_chi_square_p_value(chi_square: f64, df: i32) -> f64 {
    if df <= 0 {
        return 1.0;
    }

    // 簡易推定（正確な計算には特殊関数が必要）
    let critical_values = match df {
        1 => vec![(3.84, 0.05), (6.64, 0.01), (10.83, 0.001)],
        2 => vec![(5.99, 0.05), (9.21, 0.01), (13.82, 0.001)],
        3 => vec![(7.81, 0.05), (11.34, 0.01), (16.27, 0.001)],
        _ => vec![(9.49, 0.05), (13.28, 0.01), (18.47, 0.001)], // df=4の値を近似
    };

    for (critical, alpha) in critical_values {
        if chi_square < critical {
            return 1.0 - alpha;
        }
    }

    0.001
}
