use benf::core::BenfordResult;
use benf::core::RiskLevel;

/// Common test utilities and helper functions
pub mod fixtures;
pub mod assertions;

/// Test data for Japanese numerals
pub const FULLWIDTH_DIGITS: &str = "１２３４５６７８９０";
pub const KANJI_NUMERALS: &str = "一二三四五六七八九";
pub const KANJI_WITH_POSITIONS: &str = "一千二百三十四 五千六百七十八 九万一千二百";
pub const MIXED_JAPANESE: &str = "売上123万円 経費45万6千円 利益78万９千円";

/// Test data that follows Benford's Law (approximated)
pub const BENFORD_COMPLIANT_DATA: &[f64] = &[
    // First digit distribution roughly matching Benford's Law
    // 1: ~30.1%
    1.23, 1.45, 1.67, 1.89, 1.91, 1.13, 1.25, 1.37, 1.49, 1.51,
    1.63, 1.75, 1.87, 1.99, 1.11, 1.22, 1.33, 1.44, 1.55, 1.66,
    1.77, 1.88, 1.99, 1.01, 1.02, 1.03, 1.04, 1.05, 1.06, 1.07,
    
    // 2: ~17.6%
    2.34, 2.56, 2.78, 2.90, 2.12, 2.23, 2.34, 2.45, 2.56, 2.67,
    2.78, 2.89, 2.91, 2.92, 2.93, 2.94, 2.95, 2.96,
    
    // 3: ~12.5%
    3.45, 3.67, 3.89, 3.01, 3.12, 3.23, 3.34, 3.45, 3.56, 3.67,
    3.78, 3.89, 3.90,
    
    // 4: ~9.7%
    4.56, 4.78, 4.90, 4.12, 4.23, 4.34, 4.45, 4.56, 4.67, 4.78,
    
    // 5: ~7.9%
    5.67, 5.89, 5.01, 5.12, 5.23, 5.34, 5.45, 5.56,
    
    // 6: ~6.7%
    6.78, 6.90, 6.12, 6.23, 6.34, 6.45, 6.56,
    
    // 7: ~5.8%
    7.89, 7.01, 7.12, 7.23, 7.34, 7.45,
    
    // 8: ~5.1%
    8.90, 8.12, 8.23, 8.34, 8.45,
    
    // 9: ~4.6%
    9.01, 9.12, 9.23, 9.34,
];

/// Test data that is artificially skewed (fraud-like)
pub const FRAUDULENT_DATA: &[f64] = &[
    // Too many 5s and 6s (common in manipulated data)
    5.00, 5.10, 5.20, 5.30, 5.40, 5.50, 5.60, 5.70, 5.80, 5.90,
    6.00, 6.10, 6.20, 6.30, 6.40, 6.50, 6.60, 6.70, 6.80, 6.90,
    5.01, 5.11, 5.21, 5.31, 5.41, 5.51, 5.61, 5.71, 5.81, 5.91,
    6.01, 6.11, 6.21, 6.31, 6.41, 6.51, 6.61, 6.71, 6.81, 6.91,
    // Few 1s (unnatural)
    1.23, 1.45,
    // Few other digits
    2.34, 3.45, 4.56, 7.89, 8.90, 9.01,
];

/// Helper function to create a test BenfordResult
pub fn create_test_result(
    dataset_name: &str,
    numbers_analyzed: usize,
    digit_distribution: [f64; 9],
    chi_square: f64,
    p_value: f64,
    risk_level: RiskLevel,
) -> BenfordResult {
    BenfordResult {
        dataset_name: dataset_name.to_string(),
        numbers_analyzed,
        digit_distribution,
        chi_square,
        p_value,
        risk_level,
    }
}

/// Assert that two f64 values are approximately equal
pub fn assert_approx_eq(a: f64, b: f64, tolerance: f64) {
    assert!(
        (a - b).abs() < tolerance,
        "Values not approximately equal: {} != {} (tolerance: {})",
        a, b, tolerance
    );
}

/// Assert that an array of f64 values matches expected values within tolerance
pub fn assert_array_approx_eq(actual: &[f64], expected: &[f64], tolerance: f64) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "Array lengths differ: {} != {}",
        actual.len(),
        expected.len()
    );
    
    for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
        assert!(
            (a - e).abs() < tolerance,
            "Values at index {} not approximately equal: {} != {} (tolerance: {})",
            i, a, e, tolerance
        );
    }
}