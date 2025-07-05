use super::result::NormalResult;
use crate::error::Result;

/// 正規分布分析を実行
pub fn analyze_normal_distribution(numbers: &[f64], dataset_name: &str) -> Result<NormalResult> {
    NormalResult::new(dataset_name.to_string(), numbers)
}

/// 正規性検定を実行
pub fn test_normality(numbers: &[f64], test_type: NormalityTest) -> Result<NormalityTestResult> {
    let result = NormalResult::new("normality_test".to_string(), numbers)?;

    match test_type {
        NormalityTest::ShapiroWilk => Ok(NormalityTestResult {
            test_name: "Shapiro-Wilk".to_string(),
            statistic: result.shapiro_wilk_statistic,
            p_value: result.shapiro_wilk_p_value,
            critical_value: 0.05,
            is_normal: result.shapiro_wilk_p_value > 0.05,
        }),
        NormalityTest::AndersonDarling => Ok(NormalityTestResult {
            test_name: "Anderson-Darling".to_string(),
            statistic: result.anderson_darling_statistic,
            p_value: result.anderson_darling_p_value,
            critical_value: 0.05,
            is_normal: result.anderson_darling_p_value > 0.05,
        }),
        NormalityTest::KolmogorovSmirnov => Ok(NormalityTestResult {
            test_name: "Kolmogorov-Smirnov".to_string(),
            statistic: result.kolmogorov_smirnov_statistic,
            p_value: result.kolmogorov_smirnov_p_value,
            critical_value: 0.05,
            is_normal: result.kolmogorov_smirnov_p_value > 0.05,
        }),
        NormalityTest::All => {
            // 複数検定の統合結果
            let overall_p = (result.shapiro_wilk_p_value
                + result.anderson_darling_p_value
                + result.kolmogorov_smirnov_p_value)
                / 3.0;
            Ok(NormalityTestResult {
                test_name: "Combined Test".to_string(),
                statistic: result.normality_score,
                p_value: overall_p,
                critical_value: 0.05,
                is_normal: overall_p > 0.05,
            })
        }
    }
}

/// 異常値検出を実行
pub fn detect_outliers(
    numbers: &[f64],
    method: OutlierDetectionMethod,
) -> Result<OutlierDetectionResult> {
    let result = NormalResult::new("outlier_detection".to_string(), numbers)?;

    match method {
        OutlierDetectionMethod::ZScore => Ok(OutlierDetectionResult {
            method_name: "Z-Score".to_string(),
            outliers: result
                .outliers_z_score
                .into_iter()
                .map(|(idx, val, score)| OutlierInfo {
                    index: idx,
                    value: val,
                    score,
                    is_outlier: score.abs() > 2.5,
                })
                .collect(),
            threshold: 2.5,
        }),
        OutlierDetectionMethod::ModifiedZScore => Ok(OutlierDetectionResult {
            method_name: "Modified Z-Score".to_string(),
            outliers: result
                .outliers_modified_z
                .into_iter()
                .map(|(idx, val, score)| OutlierInfo {
                    index: idx,
                    value: val,
                    score,
                    is_outlier: score.abs() > 3.5,
                })
                .collect(),
            threshold: 3.5,
        }),
        OutlierDetectionMethod::IQR => Ok(OutlierDetectionResult {
            method_name: "IQR Method".to_string(),
            outliers: result
                .outliers_iqr
                .into_iter()
                .map(|(idx, val)| OutlierInfo {
                    index: idx,
                    value: val,
                    score: 0.0, // IQR法ではスコアなし
                    is_outlier: true,
                })
                .collect(),
            threshold: 1.5, // IQR倍数
        }),
    }
}

/// 品質管理分析を実行
pub fn quality_control_analysis(
    numbers: &[f64],
    spec_limits: Option<(f64, f64)>,
) -> Result<QualityControlResult> {
    let result = NormalResult::new("quality_control".to_string(), numbers)?;

    let (cp, cpk, process_capability) = if let Some((lsl, usl)) = spec_limits {
        let cp = (usl - lsl) / (6.0 * result.std_dev);
        let cpu = (usl - result.mean) / (3.0 * result.std_dev);
        let cpl = (result.mean - lsl) / (3.0 * result.std_dev);
        let cpk = cpu.min(cpl);

        let capability = if cpk >= 1.33 {
            ProcessCapability::Excellent
        } else if cpk >= 1.0 {
            ProcessCapability::Adequate
        } else if cpk >= 0.67 {
            ProcessCapability::Poor
        } else {
            ProcessCapability::Inadequate
        };

        (Some(cp), Some(cpk), Some(capability))
    } else {
        (None, None, None)
    };

    Ok(QualityControlResult {
        mean: result.mean,
        std_dev: result.std_dev,
        cp,
        cpk,
        process_capability,
        within_spec_percent: spec_limits.map(|(lsl, usl)| {
            let within_spec_count = numbers.iter().filter(|&&x| x >= lsl && x <= usl).count();
            (within_spec_count as f64 / numbers.len() as f64) * 100.0
        }),
        three_sigma_limits: result.three_sigma_limits,
        control_chart_violations: detect_control_chart_violations(
            numbers,
            result.mean,
            result.std_dev,
        ),
    })
}

/// 管理図違反検出
fn detect_control_chart_violations(
    numbers: &[f64],
    mean: f64,
    std_dev: f64,
) -> Vec<ControlChartViolation> {
    let mut violations = Vec::new();
    let ucl = mean + 3.0 * std_dev;
    let lcl = mean - 3.0 * std_dev;
    let uwl = mean + 2.0 * std_dev;
    let lwl = mean - 2.0 * std_dev;

    for (i, &value) in numbers.iter().enumerate() {
        if value > ucl || value < lcl {
            violations.push(ControlChartViolation {
                index: i,
                value,
                violation_type: ViolationType::OutOfControlLimits,
                description: "Point outside 3σ control limits".to_string(),
            });
        } else if value > uwl || value < lwl {
            violations.push(ControlChartViolation {
                index: i,
                value,
                violation_type: ViolationType::OutOfWarningLimits,
                description: "Point outside 2σ warning limits".to_string(),
            });
        }
    }

    // 連続点の検出（Western Electric Rules の簡易版）
    detect_run_violations(numbers, mean, std_dev, &mut violations);

    violations
}

/// 連続点違反検出
fn detect_run_violations(
    numbers: &[f64],
    mean: f64,
    _std_dev: f64,
    violations: &mut Vec<ControlChartViolation>,
) {
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
        } else {
            consecutive_above = 0;
            consecutive_below = 0;
        }
    }
}

/// 正規性検定タイプ
#[derive(Debug, Clone)]
pub enum NormalityTest {
    ShapiroWilk,
    AndersonDarling,
    KolmogorovSmirnov,
    All,
}

/// 正規性検定結果
#[derive(Debug, Clone)]
pub struct NormalityTestResult {
    pub test_name: String,
    pub statistic: f64,
    pub p_value: f64,
    pub critical_value: f64,
    pub is_normal: bool,
}

/// 外れ値検出手法
#[derive(Debug, Clone)]
pub enum OutlierDetectionMethod {
    ZScore,
    ModifiedZScore,
    IQR,
}

/// 外れ値検出結果
#[derive(Debug, Clone)]
pub struct OutlierDetectionResult {
    pub method_name: String,
    pub outliers: Vec<OutlierInfo>,
    pub threshold: f64,
}

/// 外れ値情報
#[derive(Debug, Clone)]
pub struct OutlierInfo {
    pub index: usize,
    pub value: f64,
    pub score: f64,
    pub is_outlier: bool,
}

/// 品質管理分析結果
#[derive(Debug, Clone)]
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

/// 工程能力評価
#[derive(Debug, Clone)]
pub enum ProcessCapability {
    Excellent,  // Cpk >= 1.33
    Adequate,   // 1.0 <= Cpk < 1.33
    Poor,       // 0.67 <= Cpk < 1.0
    Inadequate, // Cpk < 0.67
}

/// 管理図違反
#[derive(Debug, Clone)]
pub struct ControlChartViolation {
    pub index: usize,
    pub value: f64,
    pub violation_type: ViolationType,
    pub description: String,
}

/// 違反タイプ
#[derive(Debug, Clone)]
pub enum ViolationType {
    OutOfControlLimits,
    OutOfWarningLimits,
    RunAboveMean,
    RunBelowMean,
    Trend,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normality_tests() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0, 2.0];

        let shapiro_result = test_normality(&numbers, NormalityTest::ShapiroWilk).unwrap();
        assert_eq!(shapiro_result.test_name, "Shapiro-Wilk");
        assert!(shapiro_result.statistic >= 0.0);

        let all_result = test_normality(&numbers, NormalityTest::All).unwrap();
        assert_eq!(all_result.test_name, "Combined Test");
    }

    #[test]
    fn test_outlier_detection() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 100.0]; // 100.0は明らかな外れ値

        let z_result = detect_outliers(&numbers, OutlierDetectionMethod::ZScore).unwrap();
        assert_eq!(z_result.method_name, "Z-Score");
        assert!(!z_result.outliers.is_empty());

        let iqr_result = detect_outliers(&numbers, OutlierDetectionMethod::IQR).unwrap();
        assert_eq!(iqr_result.method_name, "IQR Method");
    }

    #[test]
    fn test_quality_control() {
        let numbers = vec![10.0, 11.0, 9.0, 10.5, 9.5, 10.2, 9.8, 10.3, 9.7, 10.1];
        let spec_limits = (8.0, 12.0);

        let qc_result = quality_control_analysis(&numbers, Some(spec_limits)).unwrap();
        assert!(qc_result.cp.is_some());
        assert!(qc_result.cpk.is_some());
        assert!(qc_result.within_spec_percent.is_some());
    }

    #[test]
    fn test_process_capability_assessment() {
        let numbers = vec![10.0; 10]; // 完全に一定のデータ
        let spec_limits = (8.0, 12.0);

        let qc_result = quality_control_analysis(&numbers, Some(spec_limits)).unwrap();
        // 標準偏差が0に近い場合、Cpは非常に高くなる
        if let Some(cp) = qc_result.cp {
            assert!(cp > 0.0);
        }
    }
}
