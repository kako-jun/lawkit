mod result;
mod analysis;

pub use result::ZipfResult;
pub use analysis::{analyze_text_zipf, analyze_numeric_zipf, analyze_combined_zipf, evaluate_zipf_quality, ZipfQualityReport};