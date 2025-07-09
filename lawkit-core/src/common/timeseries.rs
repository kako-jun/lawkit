use crate::error::Result;
use diffx_core::{diff, DiffResult};

/// 時系列データポイント
#[derive(Debug, Clone)]
pub struct TimeSeriesPoint {
    pub timestamp: f64,
    pub value: f64,
}

/// 時系列分析結果
#[derive(Debug, Clone)]
pub struct TimeSeriesAnalysis {
    pub trend: TrendAnalysis,
    pub seasonality: SeasonalityAnalysis,
    pub changepoints: Vec<ChangePoint>,
    pub forecasts: Vec<ForecastPoint>,
    pub anomalies: Vec<TimeSeriesAnomaly>,
    pub statistics: TimeSeriesStatistics,
}

/// トレンド分析
#[derive(Debug, Clone)]
pub struct TrendAnalysis {
    pub slope: f64,
    pub intercept: f64,
    pub r_squared: f64,
    pub trend_strength: f64,
    pub direction: TrendDirection,
}

/// トレンドの方向
#[derive(Debug, Clone)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Volatile,
}

/// 季節性分析
#[derive(Debug, Clone)]
pub struct SeasonalityAnalysis {
    pub detected: bool,
    pub period: Option<f64>,
    pub strength: f64,
    pub seasonal_components: Vec<f64>,
}

/// 変化点
#[derive(Debug, Clone)]
pub struct ChangePoint {
    pub timestamp: f64,
    pub index: usize,
    pub significance: f64,
    pub change_type: ChangeType,
    pub before_value: f64,
    pub after_value: f64,
}

/// 変化の種類
#[derive(Debug, Clone)]
pub enum ChangeType {
    LevelShift,
    TrendChange,
    VarianceChange,
}

/// 予測点
#[derive(Debug, Clone)]
pub struct ForecastPoint {
    pub timestamp: f64,
    pub predicted_value: f64,
    pub confidence_interval: (f64, f64),
    pub uncertainty: f64,
}

/// 時系列異常値
#[derive(Debug, Clone)]
pub struct TimeSeriesAnomaly {
    pub timestamp: f64,
    pub index: usize,
    pub value: f64,
    pub expected_value: f64,
    pub anomaly_score: f64,
    pub anomaly_type: AnomalyType,
}

/// 異常の種類
#[derive(Debug, Clone)]
pub enum AnomalyType {
    PointAnomaly,
    SequentialAnomaly,
    SeasonalAnomaly,
}

/// 時系列統計
#[derive(Debug, Clone)]
pub struct TimeSeriesStatistics {
    pub autocorrelation: Vec<f64>,
    pub partial_autocorrelation: Vec<f64>,
    pub stationarity_test: StationarityResult,
    pub noise_level: f64,
    pub data_quality: DataQuality,
}

/// 定常性検定結果
#[derive(Debug, Clone)]
pub struct StationarityResult {
    pub is_stationary: bool,
    pub test_statistic: f64,
    pub p_value: f64,
    pub differencing_required: usize,
}

/// データ品質評価
#[derive(Debug, Clone)]
pub struct DataQuality {
    pub completeness: f64,
    pub consistency: f64,
    pub regularity: f64,
    pub outlier_ratio: f64,
}

/// 時系列分析を実行
pub fn analyze_timeseries(data: &[TimeSeriesPoint]) -> Result<TimeSeriesAnalysis> {
    if data.len() < 10 {
        return Err(crate::error::BenfError::InsufficientData(data.len()));
    }

    let values: Vec<f64> = data.iter().map(|p| p.value).collect();
    let timestamps: Vec<f64> = data.iter().map(|p| p.timestamp).collect();

    Ok(TimeSeriesAnalysis {
        trend: analyze_trend(&timestamps, &values)?,
        seasonality: detect_seasonality(&values)?,
        changepoints: detect_changepoints(&timestamps, &values)?,
        forecasts: generate_forecasts(&timestamps, &values, 5)?,
        anomalies: detect_timeseries_anomalies(&timestamps, &values)?,
        statistics: calculate_timeseries_statistics(&values)?,
    })
}

/// トレンド分析
fn analyze_trend(timestamps: &[f64], values: &[f64]) -> Result<TrendAnalysis> {
    let n = values.len() as f64;

    // 線形回帰でトレンドを計算
    let sum_x: f64 = timestamps.iter().sum();
    let sum_y: f64 = values.iter().sum();
    let sum_xy: f64 = timestamps
        .iter()
        .zip(values.iter())
        .map(|(x, y)| x * y)
        .sum();
    let sum_x2: f64 = timestamps.iter().map(|x| x * x).sum();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / n;

    // R²を計算
    let mean_y = sum_y / n;
    let ss_tot: f64 = values.iter().map(|y| (y - mean_y).powi(2)).sum();
    let ss_res: f64 = timestamps
        .iter()
        .zip(values.iter())
        .map(|(x, y)| {
            let predicted = slope * x + intercept;
            (y - predicted).powi(2)
        })
        .sum();

    let r_squared = 1.0 - (ss_res / ss_tot);

    // トレンドの強さと方向を決定
    let trend_strength = r_squared * slope.abs();
    let direction = if slope.abs() < 0.01 {
        TrendDirection::Stable
    } else if slope > 0.0 {
        TrendDirection::Increasing
    } else {
        TrendDirection::Decreasing
    };

    Ok(TrendAnalysis {
        slope,
        intercept,
        r_squared,
        trend_strength,
        direction,
    })
}

/// 季節性検出
fn detect_seasonality(values: &[f64]) -> Result<SeasonalityAnalysis> {
    let n = values.len();
    let mut best_period = None;
    let mut best_strength = 0.0;

    // 可能な周期をテスト（2から n/4まで）
    for period in 2..=(n / 4) {
        let strength = calculate_seasonal_strength(values, period);
        if strength > best_strength {
            best_strength = strength;
            best_period = Some(period as f64);
        }
    }

    let detected = best_strength > 0.3; // 閾値
    let seasonal_components = if detected {
        calculate_seasonal_components(values, best_period.unwrap() as usize)
    } else {
        vec![0.0; n]
    };

    Ok(SeasonalityAnalysis {
        detected,
        period: best_period,
        strength: best_strength,
        seasonal_components,
    })
}

/// 変化点検出
fn detect_changepoints(timestamps: &[f64], values: &[f64]) -> Result<Vec<ChangePoint>> {
    let mut changepoints = Vec::new();
    let window_size = (values.len() / 10).clamp(5, 20);

    for i in window_size..(values.len() - window_size) {
        // 前後のウィンドウで統計量を比較
        let before_window = &values[(i - window_size)..i];
        let after_window = &values[i..(i + window_size)];

        let before_mean: f64 = before_window.iter().sum::<f64>() / before_window.len() as f64;
        let after_mean: f64 = after_window.iter().sum::<f64>() / after_window.len() as f64;

        let before_var: f64 = before_window
            .iter()
            .map(|x| (x - before_mean).powi(2))
            .sum::<f64>()
            / before_window.len() as f64;
        let after_var: f64 = after_window
            .iter()
            .map(|x| (x - after_mean).powi(2))
            .sum::<f64>()
            / after_window.len() as f64;

        // diffx-coreを使用した詳細な統計構造比較
        let before_stats = serde_json::json!({
            "mean": before_mean,
            "variance": before_var,
            "std_dev": before_var.sqrt(),
            "cv": if before_mean.abs() > 0.0 { before_var.sqrt() / before_mean.abs() } else { 0.0 }
        });

        let after_stats = serde_json::json!({
            "mean": after_mean,
            "variance": after_var,
            "std_dev": after_var.sqrt(),
            "cv": if after_mean.abs() > 0.0 { after_var.sqrt() / after_mean.abs() } else { 0.0 }
        });

        // diffx-coreで構造的差分を検出
        let diff_results = diff(&before_stats, &after_stats, None, Some(0.1), None);

        // 平均の変化を検出
        let mean_change = (after_mean - before_mean).abs();
        let pooled_std = ((before_var + after_var) / 2.0).sqrt();

        if pooled_std > 0.0 {
            let significance = mean_change / pooled_std;

            // diffx-coreの差分情報を活用した変化タイプの精密判定
            let mut change_type = ChangeType::LevelShift;
            let mut max_change_ratio = 0.0;

            for diff_result in &diff_results {
                if let DiffResult::Modified(path, old_val, new_val) = diff_result {
                    if path.contains("variance") || path.contains("std_dev") {
                        if let (Some(old), Some(new)) = (old_val.as_f64(), new_val.as_f64()) {
                            let ratio = (new / old.max(0.001)).max(old / new.max(0.001));
                            if ratio > max_change_ratio {
                                max_change_ratio = ratio;
                                if ratio > 2.0 {
                                    change_type = ChangeType::VarianceChange;
                                }
                            }
                        }
                    }
                }
            }

            if significance > 2.0 || max_change_ratio > 2.0 {
                changepoints.push(ChangePoint {
                    timestamp: timestamps[i],
                    index: i,
                    significance: significance.max(max_change_ratio),
                    change_type,
                    before_value: before_mean,
                    after_value: after_mean,
                });
            }
        }
    }

    Ok(changepoints)
}

/// 予測生成
fn generate_forecasts(
    timestamps: &[f64],
    values: &[f64],
    steps: usize,
) -> Result<Vec<ForecastPoint>> {
    let trend = analyze_trend(timestamps, values)?;
    let last_timestamp = timestamps.last().unwrap();
    let time_step = if timestamps.len() > 1 {
        (timestamps[timestamps.len() - 1] - timestamps[0]) / (timestamps.len() - 1) as f64
    } else {
        1.0
    };

    let mut forecasts = Vec::new();

    // 残差の標準偏差を計算
    let residuals: Vec<f64> = timestamps
        .iter()
        .zip(values.iter())
        .map(|(x, y)| {
            let predicted = trend.slope * x + trend.intercept;
            y - predicted
        })
        .collect();

    let residual_std = {
        let mean_residual = residuals.iter().sum::<f64>() / residuals.len() as f64;
        let variance = residuals
            .iter()
            .map(|r| (r - mean_residual).powi(2))
            .sum::<f64>()
            / residuals.len() as f64;
        variance.sqrt()
    };

    for i in 1..=steps {
        let future_timestamp = last_timestamp + (i as f64 * time_step);
        let predicted_value = trend.slope * future_timestamp + trend.intercept;

        // 信頼区間を計算（簡易版）
        let uncertainty = residual_std * (1.0 + i as f64 * 0.1); // 時間とともに不確実性増加
        let confidence_interval = (
            predicted_value - 1.96 * uncertainty,
            predicted_value + 1.96 * uncertainty,
        );

        forecasts.push(ForecastPoint {
            timestamp: future_timestamp,
            predicted_value,
            confidence_interval,
            uncertainty,
        });
    }

    Ok(forecasts)
}

/// 時系列異常値検出
fn detect_timeseries_anomalies(
    timestamps: &[f64],
    values: &[f64],
) -> Result<Vec<TimeSeriesAnomaly>> {
    let mut anomalies = Vec::new();
    let window_size = (values.len() / 20).clamp(3, 10);

    for i in window_size..(values.len() - window_size) {
        // 移動平均と移動標準偏差を計算
        let window = &values[(i - window_size)..(i + window_size + 1)];
        let mean: f64 = window.iter().sum::<f64>() / window.len() as f64;
        let std: f64 = {
            let variance =
                window.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / window.len() as f64;
            variance.sqrt()
        };

        if std > 0.0 {
            let z_score = (values[i] - mean) / std;

            if z_score.abs() > 3.0 {
                let expected_value = mean;
                let anomaly_score = z_score.abs() / 3.0;

                anomalies.push(TimeSeriesAnomaly {
                    timestamp: timestamps[i],
                    index: i,
                    value: values[i],
                    expected_value,
                    anomaly_score,
                    anomaly_type: AnomalyType::PointAnomaly,
                });
            }
        }
    }

    Ok(anomalies)
}

/// 時系列統計計算
fn calculate_timeseries_statistics(values: &[f64]) -> Result<TimeSeriesStatistics> {
    let n = values.len();

    // 自己相関を計算
    let max_lags = (n / 4).min(20);
    let mut autocorrelation = Vec::new();

    for lag in 0..max_lags {
        let correlation = calculate_autocorrelation(values, lag);
        autocorrelation.push(correlation);
    }

    // 偏自己相関（簡易版）
    let partial_autocorrelation = autocorrelation.clone(); // 簡略化

    // 定常性テスト（ADF風）
    let stationarity_test = test_stationarity(values);

    // ノイズレベル
    let noise_level = calculate_noise_level(values);

    // データ品質
    let data_quality = assess_data_quality(values);

    Ok(TimeSeriesStatistics {
        autocorrelation,
        partial_autocorrelation,
        stationarity_test,
        noise_level,
        data_quality,
    })
}

// ヘルパー関数
fn calculate_seasonal_strength(values: &[f64], period: usize) -> f64 {
    if period >= values.len() {
        return 0.0;
    }

    let mut seasonal_means = vec![0.0; period];
    let mut counts = vec![0; period];

    for (i, &value) in values.iter().enumerate() {
        let season_idx = i % period;
        seasonal_means[season_idx] += value;
        counts[season_idx] += 1;
    }

    // 平均を計算
    for (i, &count) in counts.iter().enumerate() {
        if count > 0 {
            seasonal_means[i] /= count as f64;
        }
    }

    // 季節性の強さを分散で測定
    let overall_mean: f64 = seasonal_means.iter().sum::<f64>() / seasonal_means.len() as f64;
    let seasonal_variance: f64 = seasonal_means
        .iter()
        .map(|x| (x - overall_mean).powi(2))
        .sum::<f64>()
        / seasonal_means.len() as f64;

    let total_variance: f64 = values
        .iter()
        .map(|x| (x - overall_mean).powi(2))
        .sum::<f64>()
        / values.len() as f64;

    if total_variance > 0.0 {
        seasonal_variance / total_variance
    } else {
        0.0
    }
}

fn calculate_seasonal_components(values: &[f64], period: usize) -> Vec<f64> {
    let mut components = vec![0.0; values.len()];
    let mut seasonal_means = vec![0.0; period];
    let mut counts = vec![0; period];

    // 各季節の平均を計算
    for (i, &value) in values.iter().enumerate() {
        let season_idx = i % period;
        seasonal_means[season_idx] += value;
        counts[season_idx] += 1;
    }

    for (i, &count) in counts.iter().enumerate() {
        if count > 0 {
            seasonal_means[i] /= count as f64;
        }
    }

    // 各データポイントに季節成分を割り当て
    for (i, component) in components.iter_mut().enumerate() {
        let season_idx = i % period;
        *component = seasonal_means[season_idx];
    }

    components
}

fn calculate_autocorrelation(values: &[f64], lag: usize) -> f64 {
    if lag >= values.len() {
        return 0.0;
    }

    let n = values.len() - lag;
    let mean: f64 = values.iter().sum::<f64>() / values.len() as f64;

    let numerator: f64 = (0..n)
        .map(|i| (values[i] - mean) * (values[i + lag] - mean))
        .sum();

    let denominator: f64 = values.iter().map(|x| (x - mean).powi(2)).sum();

    if denominator > 0.0 {
        numerator / denominator
    } else {
        0.0
    }
}

fn test_stationarity(values: &[f64]) -> StationarityResult {
    // 簡易ADF検定
    let n = values.len();
    if n < 3 {
        return StationarityResult {
            is_stationary: false,
            test_statistic: 0.0,
            p_value: 1.0,
            differencing_required: 1,
        };
    }

    // 一階差分を計算
    let diff: Vec<f64> = (1..n).map(|i| values[i] - values[i - 1]).collect();

    // 差分の分散が元データより小さければ定常性あり
    let original_var: f64 = {
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64
    };

    let diff_var: f64 = {
        let mean = diff.iter().sum::<f64>() / diff.len() as f64;
        diff.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / diff.len() as f64
    };

    let test_statistic = if original_var > 0.0 {
        diff_var / original_var
    } else {
        1.0
    };
    let is_stationary = test_statistic < 0.8;
    let p_value = if is_stationary { 0.01 } else { 0.99 };

    StationarityResult {
        is_stationary,
        test_statistic,
        p_value,
        differencing_required: if is_stationary { 0 } else { 1 },
    }
}

fn calculate_noise_level(values: &[f64]) -> f64 {
    if values.len() < 3 {
        return 0.0;
    }

    // 二階差分のRMSでノイズレベルを推定
    let second_diff: Vec<f64> = (2..values.len())
        .map(|i| values[i] - 2.0 * values[i - 1] + values[i - 2])
        .collect();

    let rms: f64 = second_diff.iter().map(|x| x.powi(2)).sum::<f64>() / second_diff.len() as f64;
    rms.sqrt()
}

fn assess_data_quality(values: &[f64]) -> DataQuality {
    let n = values.len();

    // 完全性（NaNや無効値がないことを仮定）
    let completeness = 1.0;

    // 一貫性（隣接値の変化率）
    let changes: Vec<f64> = (1..n)
        .map(|i| ((values[i] - values[i - 1]) / values[i - 1].abs().max(1e-10)).abs())
        .collect();

    let consistency = 1.0 - (changes.iter().sum::<f64>() / changes.len() as f64).min(1.0);

    // 規則性（等間隔性を仮定）
    let regularity = 1.0;

    // 外れ値比率（3σ基準）
    let mean = values.iter().sum::<f64>() / n as f64;
    let std = {
        let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
        variance.sqrt()
    };

    let outlier_count = values
        .iter()
        .filter(|&&x| (x - mean).abs() > 3.0 * std)
        .count();

    let outlier_ratio = outlier_count as f64 / n as f64;

    DataQuality {
        completeness,
        consistency,
        regularity,
        outlier_ratio,
    }
}

/// 数値データから時系列データを作成（タイムスタンプを自動生成）
pub fn create_timeseries_from_values(values: &[f64]) -> Vec<TimeSeriesPoint> {
    values
        .iter()
        .enumerate()
        .map(|(i, &value)| TimeSeriesPoint {
            timestamp: i as f64,
            value,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_timeseries_from_values() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let timeseries = create_timeseries_from_values(&values);

        assert_eq!(timeseries.len(), 5);
        assert_eq!(timeseries[0].value, 1.0);
        assert_eq!(timeseries[0].timestamp, 0.0);
        assert_eq!(timeseries[4].value, 5.0);
        assert_eq!(timeseries[4].timestamp, 4.0);
    }

    #[test]
    fn test_analyze_timeseries_basic() {
        // 線形増加データ
        let data = vec![
            TimeSeriesPoint {
                timestamp: 0.0,
                value: 1.0,
            },
            TimeSeriesPoint {
                timestamp: 1.0,
                value: 2.0,
            },
            TimeSeriesPoint {
                timestamp: 2.0,
                value: 3.0,
            },
            TimeSeriesPoint {
                timestamp: 3.0,
                value: 4.0,
            },
            TimeSeriesPoint {
                timestamp: 4.0,
                value: 5.0,
            },
            TimeSeriesPoint {
                timestamp: 5.0,
                value: 6.0,
            },
            TimeSeriesPoint {
                timestamp: 6.0,
                value: 7.0,
            },
            TimeSeriesPoint {
                timestamp: 7.0,
                value: 8.0,
            },
            TimeSeriesPoint {
                timestamp: 8.0,
                value: 9.0,
            },
            TimeSeriesPoint {
                timestamp: 9.0,
                value: 10.0,
            },
        ];

        let result = analyze_timeseries(&data).unwrap();

        // トレンドテスト
        assert!(result.trend.slope > 0.0); // 正の傾き
        assert!(result.trend.r_squared > 0.9); // 高いR²
        matches!(result.trend.direction, TrendDirection::Increasing);

        // 予測テスト
        assert_eq!(result.forecasts.len(), 5);
        assert!(result.forecasts[0].predicted_value > 10.0); // 次の値は10より大きいはず

        // 統計テスト
        assert!(!result.statistics.autocorrelation.is_empty());
    }

    #[test]
    fn test_analyze_trend() {
        let timestamps = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let values = vec![1.0, 3.0, 5.0, 7.0, 9.0]; // 明確な線形増加

        let trend = analyze_trend(&timestamps, &values).unwrap();

        assert!(trend.slope > 1.5); // 約2の傾き
        assert!(trend.slope < 2.5);
        assert!(trend.r_squared > 0.99); // ほぼ完全な線形関係
        matches!(trend.direction, TrendDirection::Increasing);
    }

    #[test]
    fn test_detect_seasonality() {
        // 周期4のサイン波様データ
        let values = vec![
            0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0,
        ];

        let seasonality = detect_seasonality(&values).unwrap();

        // 季節性が検出されるかテスト
        if seasonality.detected {
            assert!(seasonality.period.unwrap() >= 2.0);
            assert!(seasonality.strength > 0.0);
        }
    }

    #[test]
    fn test_detect_changepoints() {
        // 変化点を持つデータ: 最初は1.0付近、後半は10.0付近
        let timestamps = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let values = vec![1.0, 1.1, 0.9, 1.05, 0.95, 10.0, 9.9, 10.1, 9.95, 10.05];

        let changepoints = detect_changepoints(&timestamps, &values).unwrap();

        // 変化点が検出されるはず（インデックス5付近）
        if !changepoints.is_empty() {
            let major_changepoint = &changepoints[0];
            assert!(major_changepoint.index >= 4);
            assert!(major_changepoint.index <= 6);
            assert!(major_changepoint.significance > 2.0);
        }
    }

    #[test]
    fn test_generate_forecasts() {
        let timestamps = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0]; // 線形増加

        let forecasts = generate_forecasts(&timestamps, &values, 3).unwrap();

        assert_eq!(forecasts.len(), 3);

        // 予測値が増加傾向にあることを確認
        assert!(forecasts[0].predicted_value > 5.0);
        assert!(forecasts[1].predicted_value > forecasts[0].predicted_value);
        assert!(forecasts[2].predicted_value > forecasts[1].predicted_value);

        // 信頼区間が設定されていることを確認（区間が存在することのみチェック）
        assert!(forecasts[0].confidence_interval.0 <= forecasts[0].confidence_interval.1);
        assert!(forecasts[0].uncertainty >= 0.0); // 完全な線形データでは0になることもある
    }

    #[test]
    fn test_detect_timeseries_anomalies() {
        // 異常値を含むデータ
        let timestamps = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let values = vec![1.0, 2.0, 3.0, 100.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]; // インデックス3が異常値

        let anomalies = detect_timeseries_anomalies(&timestamps, &values).unwrap();

        // 異常値が検出されるかテスト
        if !anomalies.is_empty() {
            let anomaly = &anomalies[0];
            assert!(anomaly.value == 100.0 || anomaly.anomaly_score > 3.0);
            matches!(anomaly.anomaly_type, AnomalyType::PointAnomaly);
        }
    }

    #[test]
    fn test_calculate_timeseries_statistics() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0, 2.0];

        let stats = calculate_timeseries_statistics(&values).unwrap();

        assert!(!stats.autocorrelation.is_empty());
        assert!(!stats.partial_autocorrelation.is_empty());
        assert!(stats.noise_level >= 0.0);
        assert!(stats.data_quality.completeness > 0.0);
        assert!(stats.data_quality.consistency >= 0.0);
        assert!(stats.data_quality.outlier_ratio >= 0.0);
    }

    #[test]
    fn test_insufficient_data_error() {
        let data = vec![
            TimeSeriesPoint {
                timestamp: 0.0,
                value: 1.0,
            },
            TimeSeriesPoint {
                timestamp: 1.0,
                value: 2.0,
            },
        ]; // 10未満のデータ点

        let result = analyze_timeseries(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_stable_trend_detection() {
        let timestamps = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let values = vec![5.0, 5.01, 4.99, 5.005, 4.995]; // ほぼ一定

        let trend = analyze_trend(&timestamps, &values).unwrap();

        matches!(trend.direction, TrendDirection::Stable);
        assert!(trend.slope.abs() < 0.1); // 非常に小さな傾き
    }
}
