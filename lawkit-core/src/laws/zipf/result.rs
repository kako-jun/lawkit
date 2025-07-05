use crate::{
    common::risk::RiskLevel,
    error::{BenfError, Result},
};

/// ジップの法則（Zipf's law）解析結果
#[derive(Debug, Clone)]
pub struct ZipfResult {
    pub dataset_name: String,
    pub numbers_analyzed: usize,
    pub risk_level: RiskLevel,

    // Zipf分析メトリクス
    pub zipf_exponent: f64,           // Zipf指数（理論値は1.0）
    pub correlation_coefficient: f64, // 相関係数（適合度）
    pub distribution_quality: f64,    // 分布品質スコア

    // 頻度分析
    pub total_observations: usize,               // 総観測数
    pub unique_items: usize,                     // ユニーク項目数
    pub top_item_frequency: f64,                 // 最頻項目の出現率
    pub rank_frequency_pairs: Vec<(usize, f64)>, // ランク-頻度ペア（上位20項目）

    // 分布特性
    pub concentration_index: f64, // 集中度指数
    pub diversity_index: f64,     // 多様性指数（Shannon entropy）
    pub power_law_fit: f64,       // べき乗法則適合度
}

impl ZipfResult {
    pub fn new(dataset_name: String, frequencies: &[f64]) -> Result<Self> {
        if frequencies.len() < 5 {
            return Err(BenfError::InsufficientData(frequencies.len()));
        }

        // データの基本統計
        let numbers_analyzed = frequencies.len();
        let total_observations: usize = frequencies.iter().sum::<f64>() as usize;
        let unique_items = frequencies.len();
        let top_item_frequency = frequencies.first().copied().unwrap_or(0.0);

        // ランク-頻度ペアを生成（上位20項目または全データ）
        let rank_frequency_pairs: Vec<(usize, f64)> = frequencies
            .iter()
            .enumerate()
            .take(20.min(frequencies.len()))
            .map(|(rank, &freq)| (rank + 1, freq))
            .collect();

        // Zipf指数計算（対数線形回帰）
        let zipf_exponent = calculate_zipf_exponent(frequencies);

        // 相関係数計算（理論Zipf分布との適合度）
        let correlation_coefficient = calculate_correlation_with_theoretical_zipf(frequencies);

        // 分布品質スコア（Zipf法則への適合度）
        let distribution_quality =
            calculate_distribution_quality(zipf_exponent, correlation_coefficient);

        // 集中度指数（Gini係数）
        let concentration_index = calculate_concentration_index(frequencies);

        // 多様性指数（Shannon entropy）
        let diversity_index = calculate_diversity_index(frequencies);

        // べき乗法則適合度
        let power_law_fit = calculate_power_law_fit(frequencies);

        // リスクレベル判定
        let risk_level =
            determine_risk_level(zipf_exponent, correlation_coefficient, distribution_quality);

        Ok(ZipfResult {
            dataset_name,
            numbers_analyzed,
            risk_level,
            zipf_exponent,
            correlation_coefficient,
            distribution_quality,
            total_observations,
            unique_items,
            top_item_frequency,
            rank_frequency_pairs,
            concentration_index,
            diversity_index,
            power_law_fit,
        })
    }
}

/// Zipf指数を計算（対数線形回帰）
fn calculate_zipf_exponent(frequencies: &[f64]) -> f64 {
    if frequencies.len() < 2 {
        return 0.0;
    }

    // 対数変換（log(rank) vs log(frequency)）
    let mut log_ranks = Vec::new();
    let mut log_freqs = Vec::new();

    for (i, &freq) in frequencies.iter().enumerate() {
        if freq > 0.0 {
            log_ranks.push(((i + 1) as f64).ln());
            log_freqs.push(freq.ln());
        }
    }

    if log_ranks.len() < 2 {
        return 0.0;
    }

    // 線形回帰でスロープを計算
    let n = log_ranks.len() as f64;
    let sum_x: f64 = log_ranks.iter().sum();
    let sum_y: f64 = log_freqs.iter().sum();
    let sum_xy: f64 = log_ranks
        .iter()
        .zip(log_freqs.iter())
        .map(|(x, y)| x * y)
        .sum();
    let sum_x2: f64 = log_ranks.iter().map(|x| x * x).sum();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);

    // Zipf指数は負の傾きの絶対値
    -slope
}

/// 理論Zipf分布との相関係数を計算
fn calculate_correlation_with_theoretical_zipf(frequencies: &[f64]) -> f64 {
    if frequencies.is_empty() {
        return 0.0;
    }

    let total: f64 = frequencies.iter().sum();
    if total == 0.0 {
        return 0.0;
    }

    // 理論Zipf分布を生成（1/rank）
    let theoretical: Vec<f64> = (1..=frequencies.len())
        .map(|rank| 1.0 / rank as f64)
        .collect();

    // 正規化
    let theoretical_sum: f64 = theoretical.iter().sum();
    let theoretical_normalized: Vec<f64> =
        theoretical.iter().map(|&x| x / theoretical_sum).collect();

    let observed_normalized: Vec<f64> = frequencies.iter().map(|&x| x / total).collect();

    // ピアソン相関係数を計算
    calculate_pearson_correlation(&observed_normalized, &theoretical_normalized)
}

/// ピアソン相関係数を計算
fn calculate_pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.is_empty() {
        return 0.0;
    }

    let n = x.len() as f64;
    let mean_x: f64 = x.iter().sum::<f64>() / n;
    let mean_y: f64 = y.iter().sum::<f64>() / n;

    let mut sum_xy = 0.0;
    let mut sum_x2 = 0.0;
    let mut sum_y2 = 0.0;

    for (&xi, &yi) in x.iter().zip(y.iter()) {
        let dx = xi - mean_x;
        let dy = yi - mean_y;
        sum_xy += dx * dy;
        sum_x2 += dx * dx;
        sum_y2 += dy * dy;
    }

    let denominator = (sum_x2 * sum_y2).sqrt();
    if denominator == 0.0 {
        0.0
    } else {
        sum_xy / denominator
    }
}

/// 分布品質スコアを計算
fn calculate_distribution_quality(zipf_exponent: f64, correlation: f64) -> f64 {
    // 理想的なZipf指数は1.0、相関係数は1.0に近いほど良い
    let exponent_score = 1.0 - (zipf_exponent - 1.0).abs().min(1.0);
    let correlation_score = correlation.abs();

    // 重み付き平均
    (exponent_score * 0.6 + correlation_score * 0.4).clamp(0.0, 1.0)
}

/// 集中度指数（Gini係数）を計算
fn calculate_concentration_index(frequencies: &[f64]) -> f64 {
    if frequencies.len() < 2 {
        return 0.0;
    }

    let mut sorted_freqs = frequencies.to_vec();
    sorted_freqs.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = sorted_freqs.len() as f64;
    let sum: f64 = sorted_freqs.iter().sum();

    if sum == 0.0 {
        return 0.0;
    }

    let mut gini = 0.0;
    for (i, &freq) in sorted_freqs.iter().enumerate() {
        gini += freq * (2.0 * (i + 1) as f64 - n - 1.0);
    }

    gini / (n * sum)
}

/// 多様性指数（Shannon entropy）を計算
fn calculate_diversity_index(frequencies: &[f64]) -> f64 {
    let total: f64 = frequencies.iter().sum();
    if total == 0.0 {
        return 0.0;
    }

    let mut entropy = 0.0;
    for &freq in frequencies {
        if freq > 0.0 {
            let p = freq / total;
            entropy -= p * p.ln();
        }
    }

    entropy
}

/// べき乗法則適合度を計算
fn calculate_power_law_fit(frequencies: &[f64]) -> f64 {
    if frequencies.len() < 3 {
        return 0.0;
    }

    // Kolmogorov-Smirnov統計量の簡易版
    let theoretical_zipf = calculate_zipf_exponent(frequencies);
    if theoretical_zipf == 0.0 {
        return 0.0;
    }

    // 理論分布と観測分布の最大差を計算
    let total: f64 = frequencies.iter().sum();
    let mut max_diff: f64 = 0.0;
    let mut cumulative_observed = 0.0;
    let mut cumulative_theoretical = 0.0;

    for (i, &freq) in frequencies.iter().enumerate() {
        cumulative_observed += freq / total;

        let theoretical_freq = 1.0 / ((i + 1) as f64).powf(theoretical_zipf);
        cumulative_theoretical += theoretical_freq;

        let diff = (cumulative_observed - cumulative_theoretical).abs();
        max_diff = max_diff.max(diff);
    }

    // 適合度スコア（1.0 - KS統計量）
    (1.0 - max_diff).max(0.0)
}

/// リスクレベルを判定
fn determine_risk_level(zipf_exponent: f64, correlation: f64, quality: f64) -> RiskLevel {
    // Zipf法則への適合度に基づいてリスクレベルを判定
    if quality >= 0.8 && (zipf_exponent - 1.0).abs() < 0.2 && correlation > 0.8 {
        RiskLevel::Low // 理想的なZipf分布
    } else if quality >= 0.6 && (zipf_exponent - 1.0).abs() < 0.4 && correlation > 0.6 {
        RiskLevel::Medium // 軽微な偏差
    } else if quality >= 0.4 && (zipf_exponent - 1.0).abs() < 0.7 {
        RiskLevel::High // 有意な偏差
    } else {
        RiskLevel::Critical // 重大な偏差
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_zipf_distribution() {
        // 理想的なZipf分布（1/rank）
        let frequencies: Vec<f64> = (1..=20).map(|rank| 1000.0 / rank as f64).collect();

        let result = ZipfResult::new("test".to_string(), &frequencies).unwrap();

        assert_eq!(result.numbers_analyzed, 20);
        assert!((result.zipf_exponent - 1.0).abs() < 0.1); // 理論値1.0に近い
        assert!(result.correlation_coefficient > 0.9); // 高い相関
        assert!(matches!(result.risk_level, RiskLevel::Low));
    }

    #[test]
    fn test_uniform_distribution() {
        // 均等分布（Zipf法則に合わない）
        let frequencies = vec![100.0; 20];
        let result = ZipfResult::new("uniform".to_string(), &frequencies).unwrap();

        assert_eq!(result.numbers_analyzed, 20);
        assert!(result.zipf_exponent < 0.5 || result.zipf_exponent > 2.0); // 理論値から大きく外れる
        assert!(matches!(result.risk_level, RiskLevel::Critical));
    }

    #[test]
    fn test_insufficient_data() {
        let frequencies = vec![1.0, 2.0]; // 5個未満
        let result = ZipfResult::new("test".to_string(), &frequencies);

        assert!(result.is_err());
    }
}
