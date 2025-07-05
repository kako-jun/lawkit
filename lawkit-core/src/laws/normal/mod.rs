mod analysis;
mod result;

pub use analysis::{
    analyze_normal_distribution, detect_outliers, quality_control_analysis, test_normality,
    NormalityTest, NormalityTestResult, OutlierDetectionMethod, OutlierDetectionResult,
    ProcessCapability, QualityControlResult,
};
pub use result::NormalResult;
