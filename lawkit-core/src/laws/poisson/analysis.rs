use super::result::PoissonResult;
use crate::error::Result;

/// ポアソン分布分析を実行
pub fn analyze_poisson_distribution(numbers: &[f64], dataset_name: &str) -> Result<PoissonResult> {
    PoissonResult::new(dataset_name.to_string(), numbers)
}

/// ポアソン適合度検定を実行
pub fn test_poisson_fit(numbers: &[f64], test_type: PoissonTest) -> Result<PoissonTestResult> {
    let result = PoissonResult::new("poisson_test".to_string(), numbers)?;

    match test_type {
        PoissonTest::ChiSquare => Ok(PoissonTestResult {
            test_name: "Chi-Square Goodness of Fit".to_string(),
            statistic: result.chi_square_statistic,
            p_value: result.chi_square_p_value,
            critical_value: 0.05,
            is_poisson: result.chi_square_p_value > 0.05,
            parameter_lambda: result.lambda,
        }),
        PoissonTest::KolmogorovSmirnov => Ok(PoissonTestResult {
            test_name: "Kolmogorov-Smirnov".to_string(),
            statistic: result.kolmogorov_smirnov_statistic,
            p_value: result.kolmogorov_smirnov_p_value,
            critical_value: 0.05,
            is_poisson: result.kolmogorov_smirnov_p_value > 0.05,
            parameter_lambda: result.lambda,
        }),
        PoissonTest::VarianceTest => {
            // 分散/平均比テスト
            let test_statistic = result.variance_ratio;
            let p_value = variance_mean_ratio_p_value(test_statistic, numbers.len());

            Ok(PoissonTestResult {
                test_name: "Variance-to-Mean Ratio Test".to_string(),
                statistic: test_statistic,
                p_value,
                critical_value: 0.05,
                is_poisson: p_value > 0.05,
                parameter_lambda: result.lambda,
            })
        }
        PoissonTest::All => {
            // 複数検定の統合結果
            let overall_p = (result.chi_square_p_value + result.kolmogorov_smirnov_p_value) / 2.0;
            let variance_p = variance_mean_ratio_p_value(result.variance_ratio, numbers.len());
            let combined_p = (overall_p + variance_p) / 2.0;

            Ok(PoissonTestResult {
                test_name: "Combined Poisson Tests".to_string(),
                statistic: result.goodness_of_fit_score,
                p_value: combined_p,
                critical_value: 0.05,
                is_poisson: combined_p > 0.05,
                parameter_lambda: result.lambda,
            })
        }
    }
}

/// イベント発生確率予測
pub fn predict_event_probabilities(lambda: f64, max_events: u32) -> EventProbabilityResult {
    let mut probabilities = Vec::new();
    let mut cumulative_probabilities = Vec::new();
    let mut cumulative = 0.0;

    for k in 0..=max_events {
        let prob = poisson_probability(k, lambda);
        cumulative += prob;

        probabilities.push(EventProbability {
            event_count: k,
            probability: prob,
            cumulative_probability: cumulative,
        });
        cumulative_probabilities.push(cumulative);
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

/// 稀少事象分析
pub fn analyze_rare_events(numbers: &[f64], lambda: f64) -> RareEventAnalysis {
    let event_counts: Vec<u32> = numbers.iter().map(|&x| x as u32).collect();

    // 稀少事象の定義（例：上位5%）
    let threshold_95 = poisson_quantile(0.95, lambda);
    let threshold_99 = poisson_quantile(0.99, lambda);
    let threshold_999 = poisson_quantile(0.999, lambda);

    let rare_95 = event_counts.iter().filter(|&&x| x >= threshold_95).count();
    let rare_99 = event_counts.iter().filter(|&&x| x >= threshold_99).count();
    let rare_999 = event_counts.iter().filter(|&&x| x >= threshold_999).count();

    let extreme_events: Vec<ExtremeEvent> = event_counts
        .iter()
        .enumerate()
        .filter(|&(_, &count)| count >= threshold_99)
        .map(|(index, &count)| ExtremeEvent {
            index,
            event_count: count,
            probability: poisson_probability(count, lambda),
            rarity_level: if count >= threshold_999 {
                RarityLevel::ExtremelyRare
            } else if count >= threshold_99 {
                RarityLevel::VeryRare
            } else {
                RarityLevel::Rare
            },
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

/// イベント発生時間間隔分析（時系列データ用）
pub fn analyze_time_intervals(intervals: &[f64]) -> Result<TimeIntervalAnalysis> {
    if intervals.len() < 5 {
        return Err(crate::error::BenfError::InsufficientData(intervals.len()));
    }

    let mean_interval = intervals.iter().sum::<f64>() / intervals.len() as f64;
    let lambda_estimate = 1.0 / mean_interval; // 単位時間あたりの発生率

    // 指数分布適合度検定
    let exponential_fit = test_exponential_fit(intervals, mean_interval);

    // メモリレス性検定
    let memoryless_test = test_memoryless_property(intervals);

    // 斉次性検定（発生率が一定かどうか）
    let homogeneity_test = test_homogeneity(intervals);

    Ok(TimeIntervalAnalysis {
        mean_interval,
        lambda_estimate,
        exponential_fit_p_value: exponential_fit,
        memoryless_p_value: memoryless_test,
        homogeneity_p_value: homogeneity_test,
        is_poisson_process: exponential_fit > 0.05
            && memoryless_test > 0.05
            && homogeneity_test > 0.05,
        confidence_interval_lambda: calculate_lambda_ci_from_intervals(
            lambda_estimate,
            intervals.len(),
        ),
    })
}

// ヘルパー関数群

fn poisson_probability(k: u32, lambda: f64) -> f64 {
    if lambda <= 0.0 {
        return if k == 0 { 1.0 } else { 0.0 };
    }

    let ln_prob = k as f64 * lambda.ln() - lambda - ln_factorial(k);
    ln_prob.exp()
}

fn ln_factorial(n: u32) -> f64 {
    if n <= 1 {
        return 0.0;
    }

    if n > 10 {
        let n_f = n as f64;
        n_f * n_f.ln() - n_f + 0.5 * (2.0 * std::f64::consts::PI * n_f).ln()
    } else {
        (2..=n).map(|i| (i as f64).ln()).sum()
    }
}

fn variance_mean_ratio_p_value(ratio: f64, sample_size: usize) -> f64 {
    // インデックス分散検定の簡易版
    // H0: ratio = 1 (ポアソン分布)
    let test_statistic = (ratio - 1.0) * (sample_size as f64).sqrt();

    // 正規近似でp値推定
    let abs_stat = test_statistic.abs();
    if abs_stat > 2.58 {
        0.01
    } else if abs_stat > 1.96 {
        0.05
    } else if abs_stat > 1.64 {
        0.1
    } else {
        0.5
    }
}

fn poisson_quantile(p: f64, lambda: f64) -> u32 {
    // 累積分布関数の逆関数（近似）
    let mut k = 0;
    let mut cumulative = 0.0;

    while cumulative < p {
        cumulative += poisson_probability(k, lambda);
        if cumulative < p {
            k += 1;
        }

        if k > 1000 {
            // 無限ループ防止
            break;
        }
    }

    k
}

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

fn test_exponential_fit(intervals: &[f64], mean_interval: f64) -> f64 {
    // 指数分布適合度の簡易検定
    // KS検定の簡易版
    let mut sorted_intervals = intervals.to_vec();
    sorted_intervals.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = sorted_intervals.len() as f64;
    let mut max_diff: f64 = 0.0;

    for (i, &interval) in sorted_intervals.iter().enumerate() {
        let theoretical_cdf = 1.0 - (-interval / mean_interval).exp();
        let empirical_cdf = (i + 1) as f64 / n;
        let diff = (theoretical_cdf - empirical_cdf).abs();
        max_diff = max_diff.max(diff);
    }

    // 簡易p値推定
    let critical = 1.36 / n.sqrt();
    if max_diff > critical {
        0.01
    } else {
        0.1
    }
}

fn test_memoryless_property(intervals: &[f64]) -> f64 {
    // メモリレス性の簡易検定
    // 時間間隔の相関をチェック
    if intervals.len() < 3 {
        return 1.0;
    }

    let mut correlation_sum = 0.0;
    let mean = intervals.iter().sum::<f64>() / intervals.len() as f64;

    for i in 0..intervals.len() - 1 {
        correlation_sum += (intervals[i] - mean) * (intervals[i + 1] - mean);
    }

    // 相関が小さければメモリレス性が成立
    let abs_correlation = correlation_sum.abs() / intervals.len() as f64;
    if abs_correlation < mean * 0.1 {
        0.1
    } else {
        0.01
    }
}

fn test_homogeneity(intervals: &[f64]) -> f64 {
    // 斉次性検定（発生率が一定かどうか）
    // 時系列を前半・後半に分けて比較
    let mid = intervals.len() / 2;
    let first_half_mean = intervals[..mid].iter().sum::<f64>() / mid as f64;
    let second_half_mean = intervals[mid..].iter().sum::<f64>() / (intervals.len() - mid) as f64;

    let ratio = first_half_mean / second_half_mean;

    // 比が1に近いほど斉次
    if (ratio - 1.0).abs() < 0.2 {
        0.1
    } else {
        0.01
    }
}

fn calculate_lambda_ci_from_intervals(lambda: f64, n: usize) -> (f64, f64) {
    let std_error = lambda / (n as f64).sqrt();
    let margin = 1.96 * std_error;
    ((lambda - margin).max(0.0), lambda + margin)
}

// データ構造定義

/// ポアソン検定タイプ
#[derive(Debug, Clone)]
pub enum PoissonTest {
    ChiSquare,         // カイ二乗適合度検定
    KolmogorovSmirnov, // KS検定
    VarianceTest,      // 分散/平均比検定
    All,               // 統合検定
}

/// ポアソン検定結果
#[derive(Debug, Clone)]
pub struct PoissonTestResult {
    pub test_name: String,
    pub statistic: f64,
    pub p_value: f64,
    pub critical_value: f64,
    pub is_poisson: bool,
    pub parameter_lambda: f64,
}

/// イベント確率
#[derive(Debug, Clone)]
pub struct EventProbability {
    pub event_count: u32,
    pub probability: f64,
    pub cumulative_probability: f64,
}

/// イベント確率予測結果
#[derive(Debug, Clone)]
pub struct EventProbabilityResult {
    pub lambda: f64,
    pub max_events: u32,
    pub probabilities: Vec<EventProbability>,
    pub tail_probability: f64,
    pub most_likely_count: u32,
    pub expected_value: f64,
    pub variance: f64,
}

/// 稀少事象レベル
#[derive(Debug, Clone, PartialEq)]
pub enum RarityLevel {
    Rare,          // 稀（5%以下）
    VeryRare,      // 非常に稀（1%以下）
    ExtremelyRare, // 極稀（0.1%以下）
}

/// 極端事象
#[derive(Debug, Clone)]
pub struct ExtremeEvent {
    pub index: usize,
    pub event_count: u32,
    pub probability: f64,
    pub rarity_level: RarityLevel,
}

/// 稀少事象分析結果
#[derive(Debug, Clone)]
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

/// 時間間隔分析結果
#[derive(Debug, Clone)]
pub struct TimeIntervalAnalysis {
    pub mean_interval: f64,
    pub lambda_estimate: f64,
    pub exponential_fit_p_value: f64,
    pub memoryless_p_value: f64,
    pub homogeneity_p_value: f64,
    pub is_poisson_process: bool,
    pub confidence_interval_lambda: (f64, f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poisson_probability() {
        let lambda = 2.0;
        let prob_0 = poisson_probability(0, lambda);
        let prob_1 = poisson_probability(1, lambda);
        let prob_2 = poisson_probability(2, lambda);

        // P(X=0) = e^(-2) ≈ 0.135
        assert!((prob_0 - 0.135).abs() < 0.01);
        // P(X=1) = 2*e^(-2) ≈ 0.271
        assert!((prob_1 - 0.271).abs() < 0.01);
        // P(X=2) = 2*e^(-2) ≈ 0.271
        assert!((prob_2 - 0.271).abs() < 0.01);
    }

    #[test]
    fn test_poisson_analysis() {
        let numbers = vec![0.0, 1.0, 2.0, 1.0, 0.0, 3.0, 1.0, 2.0, 0.0, 1.0];
        let result = analyze_poisson_distribution(&numbers, "test").unwrap();

        assert_eq!(result.numbers_analyzed, 10);
        assert!(result.lambda > 0.0);
        assert!(result.sample_mean > 0.0);
    }

    #[test]
    fn test_event_probability_prediction() {
        let lambda = 1.5;
        let result = predict_event_probabilities(lambda, 5);

        assert_eq!(result.lambda, lambda);
        assert_eq!(result.expected_value, lambda);
        assert_eq!(result.variance, lambda);
        assert_eq!(result.probabilities.len(), 6); // 0-5の6個
    }

    #[test]
    fn test_poisson_tests() {
        let numbers = vec![0.0, 1.0, 0.0, 2.0, 1.0, 0.0, 1.0, 3.0, 0.0, 1.0];

        let chi_result = test_poisson_fit(&numbers, PoissonTest::ChiSquare).unwrap();
        assert_eq!(chi_result.test_name, "Chi-Square Goodness of Fit");
        assert!(chi_result.parameter_lambda > 0.0);

        let ks_result = test_poisson_fit(&numbers, PoissonTest::KolmogorovSmirnov).unwrap();
        assert_eq!(ks_result.test_name, "Kolmogorov-Smirnov");
    }
}
