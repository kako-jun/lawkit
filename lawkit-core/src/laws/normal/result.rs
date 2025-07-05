use crate::{
    common::risk::RiskLevel,
    error::{BenfError, Result},
};

/// 正規分布解析結果
#[derive(Debug, Clone)]
pub struct NormalResult {
    pub dataset_name: String,
    pub numbers_analyzed: usize,
    pub risk_level: RiskLevel,

    // 分布パラメータ
    pub mean: f64,     // 平均値
    pub std_dev: f64,  // 標準偏差
    pub variance: f64, // 分散
    pub skewness: f64, // 歪度（左右の偏り）
    pub kurtosis: f64, // 尖度（分布の尖り）

    // 正規性検定結果
    pub shapiro_wilk_statistic: f64,       // Shapiro-Wilk検定統計量
    pub shapiro_wilk_p_value: f64,         // Shapiro-Wilk p値
    pub anderson_darling_statistic: f64,   // Anderson-Darling検定統計量
    pub anderson_darling_p_value: f64,     // Anderson-Darling p値
    pub kolmogorov_smirnov_statistic: f64, // Kolmogorov-Smirnov検定統計量
    pub kolmogorov_smirnov_p_value: f64,   // Kolmogorov-Smirnov p値

    // 適合度評価
    pub normality_score: f64,      // 正規性総合スコア（0-1）
    pub qq_correlation: f64,       // Q-Q plot相関係数
    pub distribution_quality: f64, // 分布品質スコア

    // 異常値検出
    pub outliers_z_score: Vec<(usize, f64, f64)>, // (インデックス, 値, Z-score)
    pub outliers_modified_z: Vec<(usize, f64, f64)>, // (インデックス, 値, 修正Z-score)
    pub outliers_iqr: Vec<(usize, f64)>,          // (インデックス, 値)

    // 信頼区間・範囲
    pub mean_confidence_interval: (f64, f64), // 平均の95%信頼区間
    pub prediction_interval_95: (f64, f64),   // 95%予測区間
    pub three_sigma_limits: (f64, f64),       // 3σ限界

    // 品質管理指標
    pub within_1_sigma_percent: f64, // 1σ以内の割合
    pub within_2_sigma_percent: f64, // 2σ以内の割合
    pub within_3_sigma_percent: f64, // 3σ以内の割合
}

impl NormalResult {
    pub fn new(dataset_name: String, numbers: &[f64]) -> Result<Self> {
        if numbers.len() < 8 {
            return Err(BenfError::InsufficientData(numbers.len()));
        }

        let numbers_analyzed = numbers.len();

        // 基本統計量計算
        let mean = calculate_mean(numbers);
        let variance = calculate_variance(numbers, mean);
        let std_dev = variance.sqrt();
        let skewness = calculate_skewness(numbers, mean, std_dev);
        let kurtosis = calculate_kurtosis(numbers, mean, std_dev);

        // 正規性検定
        let shapiro_result = shapiro_wilk_test(numbers);
        let anderson_result = anderson_darling_test(numbers, mean, std_dev);
        let ks_result = kolmogorov_smirnov_test(numbers, mean, std_dev);

        // 適合度評価
        let qq_correlation = calculate_qq_correlation(numbers, mean, std_dev);
        let normality_score = calculate_normality_score(
            shapiro_result.1,
            anderson_result.1,
            ks_result.1,
            qq_correlation,
        );
        let distribution_quality =
            calculate_distribution_quality(skewness, kurtosis, normality_score);

        // 異常値検出
        let outliers_z_score = detect_outliers_z_score(numbers, mean, std_dev);
        let outliers_modified_z = detect_outliers_modified_z_score(numbers);
        let outliers_iqr = detect_outliers_iqr(numbers);

        // 信頼区間・範囲計算
        let mean_confidence_interval = calculate_mean_confidence_interval(numbers, mean, std_dev);
        let prediction_interval_95 = (mean - 1.96 * std_dev, mean + 1.96 * std_dev);
        let three_sigma_limits = (mean - 3.0 * std_dev, mean + 3.0 * std_dev);

        // 品質管理指標
        let within_1_sigma_percent = calculate_within_sigma_percent(numbers, mean, std_dev, 1.0);
        let within_2_sigma_percent = calculate_within_sigma_percent(numbers, mean, std_dev, 2.0);
        let within_3_sigma_percent = calculate_within_sigma_percent(numbers, mean, std_dev, 3.0);

        // リスクレベル判定
        let risk_level =
            determine_risk_level(normality_score, &outliers_z_score, skewness, kurtosis);

        Ok(NormalResult {
            dataset_name,
            numbers_analyzed,
            risk_level,
            mean,
            std_dev,
            variance,
            skewness,
            kurtosis,
            shapiro_wilk_statistic: shapiro_result.0,
            shapiro_wilk_p_value: shapiro_result.1,
            anderson_darling_statistic: anderson_result.0,
            anderson_darling_p_value: anderson_result.1,
            kolmogorov_smirnov_statistic: ks_result.0,
            kolmogorov_smirnov_p_value: ks_result.1,
            normality_score,
            qq_correlation,
            distribution_quality,
            outliers_z_score,
            outliers_modified_z,
            outliers_iqr,
            mean_confidence_interval,
            prediction_interval_95,
            three_sigma_limits,
            within_1_sigma_percent,
            within_2_sigma_percent,
            within_3_sigma_percent,
        })
    }
}

/// 平均値計算
fn calculate_mean(numbers: &[f64]) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}

/// 分散計算
fn calculate_variance(numbers: &[f64], mean: f64) -> f64 {
    let sum_squared_diff: f64 = numbers.iter().map(|&x| (x - mean).powi(2)).sum();
    sum_squared_diff / (numbers.len() - 1) as f64
}

/// 歪度計算（標準化第3次モーメント）
fn calculate_skewness(numbers: &[f64], mean: f64, std_dev: f64) -> f64 {
    let n = numbers.len() as f64;
    let sum_cubed: f64 = numbers
        .iter()
        .map(|&x| ((x - mean) / std_dev).powi(3))
        .sum();

    sum_cubed / n
}

/// 尖度計算（標準化第4次モーメント - 3）
fn calculate_kurtosis(numbers: &[f64], mean: f64, std_dev: f64) -> f64 {
    let n = numbers.len() as f64;
    let sum_fourth: f64 = numbers
        .iter()
        .map(|&x| ((x - mean) / std_dev).powi(4))
        .sum();

    (sum_fourth / n) - 3.0 // 正規分布の尖度3を基準とした超過尖度
}

/// Shapiro-Wilk検定（簡易版）
fn shapiro_wilk_test(numbers: &[f64]) -> (f64, f64) {
    let n = numbers.len();
    if !(8..=5000).contains(&n) {
        return (0.0, 1.0); // 適用範囲外
    }

    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // 簡易実装: 分位数相関係数を使用
    let qq_corr = calculate_qq_correlation(
        &sorted,
        calculate_mean(&sorted),
        calculate_variance(&sorted, calculate_mean(&sorted)).sqrt(),
    );
    let w_statistic = qq_corr * qq_corr;

    // 簡易p値推定（厳密な計算は複雑な積分が必要）
    let p_value = if w_statistic > 0.95 {
        0.5
    } else if w_statistic > 0.90 {
        0.1
    } else if w_statistic > 0.80 {
        0.01
    } else {
        0.001
    };

    (w_statistic, p_value)
}

/// Anderson-Darling検定（簡易版）
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
            a_squared += (2.0 * (i + 1) as f64 - 1.0) * (phi.ln() + phi_complement.ln()) / n;
        }
    }

    a_squared = -n - a_squared;

    // 簡易p値推定
    let p_value = if a_squared < 0.5 {
        0.5
    } else if a_squared < 1.0 {
        0.1
    } else if a_squared < 2.0 {
        0.01
    } else {
        0.001
    };

    (a_squared, p_value)
}

/// Kolmogorov-Smirnov検定（簡易版）
fn kolmogorov_smirnov_test(numbers: &[f64], mean: f64, std_dev: f64) -> (f64, f64) {
    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = sorted.len() as f64;
    let mut max_diff: f64 = 0.0;

    for (i, &x) in sorted.iter().enumerate() {
        let z = (x - mean) / std_dev;
        let expected_cdf = standard_normal_cdf(z);
        let empirical_cdf = (i + 1) as f64 / n;

        let diff = (expected_cdf - empirical_cdf).abs();
        max_diff = max_diff.max(diff);
    }

    // 簡易p値推定
    let ks_critical = 1.36 / n.sqrt(); // α=0.05での臨界値
    let p_value = if max_diff < ks_critical * 0.5 {
        0.5
    } else if max_diff < ks_critical {
        0.1
    } else if max_diff < ks_critical * 1.5 {
        0.01
    } else {
        0.001
    };

    (max_diff, p_value)
}

/// 標準正規分布の累積分布関数（簡易実装）
fn standard_normal_cdf(z: f64) -> f64 {
    if z > 6.0 {
        return 1.0;
    }
    if z < -6.0 {
        return 0.0;
    }

    // Box-Muller変換の逆関数による近似
    let t = 1.0 / (1.0 + 0.2316419 * z.abs());
    let d = 0.3989423 * (-z * z / 2.0).exp();
    let probability =
        d * t * (0.3193815 + t * (-0.3565638 + t * (1.7814779 + t * (-1.8212560 + t * 1.3302744))));

    if z >= 0.0 {
        1.0 - probability
    } else {
        probability
    }
}

/// Q-Q plot相関係数計算
fn calculate_qq_correlation(numbers: &[f64], mean: f64, std_dev: f64) -> f64 {
    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = sorted.len();
    let mut theoretical_quantiles = Vec::new();

    for i in 0..n {
        let p = (i + 1) as f64 / (n + 1) as f64;
        let z = inverse_standard_normal(p);
        theoretical_quantiles.push(mean + std_dev * z);
    }

    pearson_correlation(&sorted, &theoretical_quantiles)
}

/// 標準正規分布の逆関数（簡易実装）
fn inverse_standard_normal(p: f64) -> f64 {
    if p <= 0.0 {
        return -6.0;
    }
    if p >= 1.0 {
        return 6.0;
    }

    // Beasley-Springer-Moro algorithm の簡易版
    let a = p - 0.5;
    if a.abs() < 0.42 {
        let r = a * a;
        return a
            * (((2.5066282 + r * 0.3001648) + r * 0.0010805)
                / ((1.0 + r * 1.6372227) + r * 0.0312753));
    }

    let r = if a < 0.0 { p } else { 1.0 - p };
    let s = (-r.ln()).sqrt();
    let t = s
        - ((2.515517 + s * 0.802853) + s * s * 0.010328)
            / ((1.0 + s * 1.432788) + s * s * 0.189269);

    if a < 0.0 {
        -t
    } else {
        t
    }
}

/// ピアソン相関係数計算
fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() {
        return 0.0;
    }

    let n = x.len() as f64;
    let mean_x = x.iter().sum::<f64>() / n;
    let mean_y = y.iter().sum::<f64>() / n;

    let mut numerator = 0.0;
    let mut sum_x_sq = 0.0;
    let mut sum_y_sq = 0.0;

    for (&xi, &yi) in x.iter().zip(y.iter()) {
        let dx = xi - mean_x;
        let dy = yi - mean_y;
        numerator += dx * dy;
        sum_x_sq += dx * dx;
        sum_y_sq += dy * dy;
    }

    let denominator = (sum_x_sq * sum_y_sq).sqrt();
    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

/// 正規性総合スコア計算
fn calculate_normality_score(sw_p: f64, ad_p: f64, ks_p: f64, qq_corr: f64) -> f64 {
    // p値は高いほど正規分布に近い、相関係数も高いほど良い
    let p_score = (sw_p * 0.4 + ad_p * 0.3 + ks_p * 0.3).min(1.0);
    let corr_score = qq_corr.abs();

    (p_score * 0.6 + corr_score * 0.4).clamp(0.0, 1.0)
}

/// 分布品質スコア計算
fn calculate_distribution_quality(skewness: f64, kurtosis: f64, normality_score: f64) -> f64 {
    // 歪度・尖度が0に近いほど良い（正規分布では歪度=0, 超過尖度=0）
    let skew_score = 1.0 - (skewness.abs() / 2.0).min(1.0);
    let kurt_score = 1.0 - (kurtosis.abs() / 3.0).min(1.0);

    (normality_score * 0.5 + skew_score * 0.25 + kurt_score * 0.25).clamp(0.0, 1.0)
}

/// Z-scoreによる異常値検出
fn detect_outliers_z_score(numbers: &[f64], mean: f64, std_dev: f64) -> Vec<(usize, f64, f64)> {
    let mut outliers = Vec::new();
    let threshold = 2.5; // 2.5σ以上を異常値とする

    for (i, &value) in numbers.iter().enumerate() {
        let z_score = (value - mean) / std_dev;
        if z_score.abs() > threshold {
            outliers.push((i, value, z_score));
        }
    }

    outliers
}

/// 修正Z-scoreによる異常値検出
fn detect_outliers_modified_z_score(numbers: &[f64]) -> Vec<(usize, f64, f64)> {
    let median = calculate_median(numbers);
    let mad = calculate_mad(numbers, median);
    let mut outliers = Vec::new();
    let threshold = 3.5; // 修正Z-scoreの閾値

    for (i, &value) in numbers.iter().enumerate() {
        let modified_z = 0.6745 * (value - median) / mad;
        if modified_z.abs() > threshold {
            outliers.push((i, value, modified_z));
        }
    }

    outliers
}

/// IQR法による異常値検出
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

/// 中央値計算
fn calculate_median(numbers: &[f64]) -> f64 {
    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = sorted.len();

    if n % 2 == 0 {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    }
}

/// MAD（Median Absolute Deviation）計算
fn calculate_mad(numbers: &[f64], median: f64) -> f64 {
    let deviations: Vec<f64> = numbers.iter().map(|&x| (x - median).abs()).collect();
    calculate_median(&deviations)
}

/// 平均の信頼区間計算
fn calculate_mean_confidence_interval(numbers: &[f64], mean: f64, std_dev: f64) -> (f64, f64) {
    let n = numbers.len() as f64;
    let se = std_dev / n.sqrt();
    let t_critical = 1.96; // 大標本近似でt分布≈標準正規分布

    (mean - t_critical * se, mean + t_critical * se)
}

/// σ範囲内データの割合計算
fn calculate_within_sigma_percent(numbers: &[f64], mean: f64, std_dev: f64, sigma: f64) -> f64 {
    let lower = mean - sigma * std_dev;
    let upper = mean + sigma * std_dev;

    let within_count = numbers
        .iter()
        .filter(|&&x| x >= lower && x <= upper)
        .count();

    (within_count as f64 / numbers.len() as f64) * 100.0
}

/// リスクレベル判定
fn determine_risk_level(
    normality_score: f64,
    outliers: &[(usize, f64, f64)],
    skewness: f64,
    kurtosis: f64,
) -> RiskLevel {
    let outlier_ratio = outliers.len() as f64 / 100.0; // 仮の分母、実際は総データ数

    if normality_score > 0.8 && outlier_ratio < 0.05 && skewness.abs() < 0.5 && kurtosis.abs() < 0.5
    {
        RiskLevel::Low
    } else if normality_score > 0.6
        && outlier_ratio < 0.1
        && skewness.abs() < 1.0
        && kurtosis.abs() < 1.0
    {
        RiskLevel::Medium
    } else if normality_score > 0.3 && outlier_ratio < 0.2 {
        RiskLevel::High
    } else {
        RiskLevel::Critical
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_distribution() {
        // 標準正規分布に近いデータ
        let numbers = vec![
            0.0, 0.5, -0.3, 1.2, -0.8, 0.2, -0.1, 0.9, -0.6, 0.7, -0.4, 0.3, 1.1, -0.9, 0.6, -0.2,
            0.8, -0.5, 0.1, 0.4,
        ];

        let result = NormalResult::new("test".to_string(), &numbers).unwrap();

        assert_eq!(result.numbers_analyzed, 20);
        assert!((result.mean - 0.0).abs() < 0.3); // 平均は0に近い
        assert!(result.std_dev > 0.0);
        assert!(result.normality_score > 0.0);
    }

    #[test]
    fn test_insufficient_data() {
        let numbers = vec![1.0, 2.0, 3.0]; // 8個未満
        let result = NormalResult::new("test".to_string(), &numbers);

        assert!(result.is_err());
    }

    #[test]
    fn test_outlier_detection() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 100.0]; // 100.0が外れ値
        let result = NormalResult::new("outlier_test".to_string(), &numbers).unwrap();

        assert!(!result.outliers_z_score.is_empty());
    }
}
