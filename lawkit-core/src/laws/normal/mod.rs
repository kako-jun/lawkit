mod result;
mod analysis;

pub use result::NormalResult;
pub use analysis::{
    analyze_normal_distribution, test_normality, detect_outliers, quality_control_analysis,
    NormalityTest, OutlierDetectionMethod, NormalityTestResult, OutlierDetectionResult, 
    QualityControlResult, ProcessCapability
};