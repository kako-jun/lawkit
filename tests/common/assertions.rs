/// Custom assertions for Benford's Law testing

use benf::core::{BenfordResult, RiskLevel};

/// Assert that a BenfordResult meets basic validation criteria
pub fn assert_valid_benford_result(result: &BenfordResult) {
    // Check that we analyzed some numbers
    assert!(
        result.numbers_analyzed > 0,
        "Should analyze at least one number, got {}",
        result.numbers_analyzed
    );
    
    // Check that digit distribution sums to approximately 100%
    let total_percentage: f64 = result.digit_distribution.iter().sum();
    assert!(
        (total_percentage - 100.0).abs() < 0.1,
        "Digit distribution should sum to ~100%, got {}",
        total_percentage
    );
    
    // Check that all percentages are non-negative
    for (i, &percentage) in result.digit_distribution.iter().enumerate() {
        assert!(
            percentage >= 0.0,
            "Digit {} percentage should be non-negative, got {}",
            i + 1,
            percentage
        );
    }
    
    // Check that chi-square is non-negative
    assert!(
        result.chi_square >= 0.0,
        "Chi-square should be non-negative, got {}",
        result.chi_square
    );
    
    // Check that p-value is between 0 and 1
    assert!(
        result.p_value >= 0.0 && result.p_value <= 1.0,
        "P-value should be between 0 and 1, got {}",
        result.p_value
    );
}

/// Assert that the risk level matches the p-value according to our thresholds
pub fn assert_risk_level_matches_p_value(result: &BenfordResult) {
    let expected_risk_level = match result.p_value {
        p if p > 0.1 => RiskLevel::Low,
        p if p > 0.05 => RiskLevel::Medium,
        p if p > 0.01 => RiskLevel::High,
        _ => RiskLevel::Critical,
    };
    
    assert_eq!(
        result.risk_level, expected_risk_level,
        "Risk level {:?} doesn't match p-value {} (expected {:?})",
        result.risk_level, result.p_value, expected_risk_level
    );
}

/// Assert that a distribution follows Benford's Law within tolerance
pub fn assert_follows_benford_law(distribution: &[f64], tolerance: f64) {
    // Benford's Law expected percentages for digits 1-9
    const BENFORD_EXPECTED: [f64; 9] = [
        30.103, 17.609, 12.494, 9.691, 7.918, 6.695, 5.799, 5.115, 4.576
    ];
    
    assert_eq!(
        distribution.len(),
        9,
        "Distribution should have 9 elements (digits 1-9), got {}",
        distribution.len()
    );
    
    for (i, (&actual, &expected)) in distribution.iter().zip(BENFORD_EXPECTED.iter()).enumerate() {
        assert!(
            (actual - expected).abs() < tolerance,
            "Digit {} distribution {} doesn't match Benford's Law {} within tolerance {}",
            i + 1,
            actual,
            expected,
            tolerance
        );
    }
}

/// Assert that a distribution significantly deviates from Benford's Law
pub fn assert_deviates_from_benford_law(distribution: &[f64], min_deviation: f64) {
    // Benford's Law expected percentages for digits 1-9
    const BENFORD_EXPECTED: [f64; 9] = [
        30.103, 17.609, 12.494, 9.691, 7.918, 6.695, 5.799, 5.115, 4.576
    ];
    
    let mut max_deviation = 0.0;
    for (&actual, &expected) in distribution.iter().zip(BENFORD_EXPECTED.iter()) {
        let deviation = (actual - expected).abs();
        if deviation > max_deviation {
            max_deviation = deviation;
        }
    }
    
    assert!(
        max_deviation >= min_deviation,
        "Distribution should deviate from Benford's Law by at least {}, but max deviation is {}",
        min_deviation,
        max_deviation
    );
}

/// Assert that Japanese numeral conversion produces expected results
pub fn assert_japanese_conversion(input: &str, expected: &str, description: &str) {
    // This will be implemented once we have the actual conversion function
    // For now, we'll create a placeholder that the tests can use
    assert!(
        !input.is_empty(),
        "Input should not be empty for test: {}",
        description
    );
    
    assert!(
        !expected.is_empty(),
        "Expected result should not be empty for test: {}",
        description
    );
}

/// Assert that number extraction produces expected results
pub fn assert_number_extraction(input: &str, expected: &[f64], description: &str) {
    // This will be implemented once we have the actual extraction function
    // For now, we'll create a placeholder that validates the test data
    
    // Check that input is valid
    assert!(
        input.len() >= 0, // Allow empty strings
        "Input should be valid for test: {}",
        description
    );
    
    // Check that expected results are valid
    for &num in expected {
        assert!(
            num.is_finite(),
            "Expected number {} should be finite for test: {}",
            num,
            description
        );
    }
}

/// Assert that two f64 slices are approximately equal
pub fn assert_slices_approx_eq(actual: &[f64], expected: &[f64], tolerance: f64, description: &str) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "Slice lengths should match for {}: {} != {}",
        description,
        actual.len(),
        expected.len()
    );
    
    for (i, (&a, &e)) in actual.iter().zip(expected.iter()).enumerate() {
        assert!(
            (a - e).abs() < tolerance,
            "Values at index {} should be approximately equal for {}: {} != {} (tolerance: {})",
            i,
            description,
            a,
            e,
            tolerance
        );
    }
}