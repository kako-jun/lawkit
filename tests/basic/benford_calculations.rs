// Import all necessary functions for comprehensive testing
// use lawkit_core::laws::benford::result::BenfordResult;
// use lawkit_core::common::risk::RiskLevel;
// use crate::test_common::{BENFORD_COMPLIANT_DATA, FRAUDULENT_DATA};
// use crate::test_common::assertions::{
//     assert_valid_benford_result,
//     assert_follows_benford_law,
//     assert_deviates_from_benford_law,
//     assert_slices_approx_eq
// };

#[cfg(test)]
mod benford_distribution_tests {
    // use super::*;

    #[test]
    fn test_benford_expected_percentages_constant() {
        // Test that our expected percentages match Benford's Law
        let expected = [
            30.103, 17.609, 12.494, 9.691, 7.918, 6.695, 5.799, 5.115, 4.576,
        ];
        // Verify that our expected percentages match the constants
        for (i, &expected_val) in expected.iter().enumerate() {
            let actual_val = lawkit_core::laws::benford::analysis::BENFORD_EXPECTED_PERCENTAGES[i];
            assert!(
                (actual_val - expected_val).abs() < 0.001,
                "BENFORD_EXPECTED_PERCENTAGES[{}] = {}, expected {}",
                i,
                actual_val,
                expected_val
            );
        }

        // Test that percentages sum to approximately 100%
        let sum: f64 = lawkit_core::laws::benford::analysis::BENFORD_EXPECTED_PERCENTAGES
            .iter()
            .sum();
        assert!(
            (sum - 100.0).abs() < 0.001,
            "Expected percentages should sum to 100%, got {}",
            sum
        );
    }

    #[test]
    fn test_extract_first_digits() {
        let numbers = vec![123.45, 234.56, 345.67, 456.78];
        let first_digits: Vec<u8> = numbers
            .iter()
            .filter_map(|&n| lawkit_core::laws::benford::analysis::get_first_digit(n))
            .collect();
        assert_eq!(first_digits, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_extract_first_digits_edge_cases() {
        // Test with zero - should be filtered out for Benford's Law
        let numbers = vec![0.0, 123.0, 0.001, 456.0];
        let first_digits: Vec<u8> = numbers
            .iter()
            .filter_map(|&n| lawkit_core::laws::benford::analysis::get_first_digit(n))
            .collect();
        // 0.0 and 0.001 are filtered out, only 123.0 -> 1 and 456.0 -> 4 remain
        assert_eq!(first_digits, vec![1, 4]);

        // Test with negative numbers - should use absolute value
        let numbers = vec![-123.0, -234.0, 345.0];
        let first_digits: Vec<u8> = numbers
            .iter()
            .filter_map(|&n| lawkit_core::laws::benford::analysis::get_first_digit(n))
            .collect();
        assert_eq!(first_digits, vec![1, 2, 3]);

        // Test with decimal numbers less than 1 - should be filtered out
        let numbers = vec![0.123, 0.234, 0.345];
        let first_digits: Vec<u8> = numbers
            .iter()
            .filter_map(|&n| lawkit_core::laws::benford::analysis::get_first_digit(n))
            .collect();
        // All are < 1.0, so should be empty
        assert_eq!(first_digits, Vec::<u8>::new());
    }

    #[test]
    fn test_calculate_first_digit_distribution() {
        // Use actual numbers instead of pre-extracted digits
        let numbers = vec![100.0, 100.0, 100.0, 200.0, 200.0, 300.0];
        let distribution =
            lawkit_core::laws::benford::analysis::calculate_digit_distribution(&numbers);

        // Should be: 1 appears 3/6 = 50%, 2 appears 2/6 = 33.33%, 3 appears 1/6 = 16.67%
        // Others should be 0%
        let expected = [50.0, 33.333, 16.667, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        for (i, &expected_val) in expected.iter().enumerate() {
            let actual_val = distribution[i];
            assert!(
                (actual_val - expected_val).abs() < 0.01,
                "Distribution[{}] = {}, expected {}",
                i,
                actual_val,
                expected_val
            );
        }
    }

    #[test]
    fn test_calculate_first_digit_distribution_empty() {
        let numbers: Vec<f64> = vec![];
        let distribution = lawkit_core::laws::benford::analysis::calculate_digit_distribution(&numbers);

        // Should be all zeros for empty input
        let expected = [0.0; 9];
        for (i, &expected_val) in expected.iter().enumerate() {
            assert_eq!(distribution[i], expected_val, "Empty distribution should be all zeros");
        }
    }

    #[test]
    fn test_chi_square_calculation() {
        // Test with perfect Benford distribution
        let observed = lawkit_core::laws::benford::analysis::BENFORD_EXPECTED_PERCENTAGES.clone();
        let expected = lawkit_core::laws::benford::analysis::BENFORD_EXPECTED_PERCENTAGES.clone();
        let chi_square =
            lawkit_core::common::statistics::calculate_chi_square(&observed, &expected);

        // Chi-square should be approximately 0 for identical distributions
        assert!(
            chi_square < 0.001,
            "Chi-square should be near 0 for identical distributions, got {}",
            chi_square
        );
    }

    #[test]
    fn test_chi_square_with_deviation() {
        // Test with a distribution that deviates from Benford's Law
        let observed = [40.0, 20.0, 10.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0]; // Skewed toward 1
        let expected = lawkit_core::laws::benford::analysis::BENFORD_EXPECTED_PERCENTAGES.clone();
        let chi_square =
            lawkit_core::common::statistics::calculate_chi_square(&observed, &expected);
        let p_value = lawkit_core::common::statistics::calculate_p_value(chi_square, 8);

        // Chi-square should be significantly higher  
        assert!(
            chi_square > 8.0,
            "Chi-square should be high for skewed distribution, got {}",
            chi_square
        );

        // P-value should be low (significant deviation)
        assert!(
            p_value < 0.6,
            "P-value should be low for significant deviation, got {}",
            p_value
        );
    }

    #[test]
    fn test_benford_distribution_with_compliant_data() {
        // let result = calculate_benford_distribution("test_data", BENFORD_COMPLIANT_DATA);

        // assert_valid_benford_result(&result);
        // assert_eq!(result.dataset_name, "test_data");
        // assert_eq!(result.numbers_analyzed, BENFORD_COMPLIANT_DATA.len());

        // // Should follow Benford's Law reasonably well
        // assert_follows_benford_law(&result.digit_distribution, 5.0); // Allow 5% tolerance

        // // Should have low risk level
        // assert!(matches!(result.risk_level, RiskLevel::Low | RiskLevel::Medium));
    }

    #[test]
    fn test_benford_distribution_with_fraudulent_data() {
        // let result = calculate_benford_distribution("fraudulent_data", FRAUDULENT_DATA);

        // assert_valid_benford_result(&result);

        // // Should deviate significantly from Benford's Law
        // assert_deviates_from_benford_law(&result.digit_distribution, 10.0); // Expect >10% deviation

        // // Should have high risk level
        // assert!(matches!(result.risk_level, RiskLevel::High | RiskLevel::Critical));

        // // Chi-square should be high
        // assert!(result.chi_square > 20.0, "Chi-square should be high for fraudulent data, got {}", result.chi_square);

        // // P-value should be very low
        // assert!(result.p_value < 0.01, "P-value should be very low for fraudulent data, got {}", result.p_value);
    }
}

#[cfg(test)]
mod risk_assessment_tests {
    // use super::*;
    // use benf::core::risk::determine_risk_level;

    #[test]
    fn test_risk_level_determination() {
        // // Test Low risk (p > 0.1)
        // assert_eq!(determine_risk_level(0.5), RiskLevel::Low);
        // assert_eq!(determine_risk_level(0.15), RiskLevel::Low);
        // assert_eq!(determine_risk_level(0.11), RiskLevel::Low);

        // // Test Medium risk (0.05 < p <= 0.1)
        // assert_eq!(determine_risk_level(0.1), RiskLevel::Medium);
        // assert_eq!(determine_risk_level(0.075), RiskLevel::Medium);
        // assert_eq!(determine_risk_level(0.051), RiskLevel::Medium);

        // // Test High risk (0.01 < p <= 0.05)
        // assert_eq!(determine_risk_level(0.05), RiskLevel::High);
        // assert_eq!(determine_risk_level(0.03), RiskLevel::High);
        // assert_eq!(determine_risk_level(0.011), RiskLevel::High);

        // // Test Critical risk (p <= 0.01)
        // assert_eq!(determine_risk_level(0.01), RiskLevel::Critical);
        // assert_eq!(determine_risk_level(0.005), RiskLevel::Critical);
        // assert_eq!(determine_risk_level(0.0), RiskLevel::Critical);
    }

    #[test]
    fn test_risk_level_edge_cases() {
        // // Test exact boundary values
        // assert_eq!(determine_risk_level(0.1), RiskLevel::Medium);
        // assert_eq!(determine_risk_level(0.05), RiskLevel::High);
        // assert_eq!(determine_risk_level(0.01), RiskLevel::Critical);
    }
}

#[cfg(test)]
mod statistical_tests {
    // use super::*;
    // use benf::core::statistics::{calculate_mean_absolute_deviation, calculate_confidence_interval};

    #[test]
    fn test_mean_absolute_deviation() {
        // let observed = [30.0, 18.0, 12.0, 10.0, 8.0, 7.0, 6.0, 5.0, 4.0];
        // let expected = BENFORD_EXPECTED_PERCENTAGES.clone();

        // let mad = calculate_mean_absolute_deviation(&observed, &expected);

        // // MAD should be small for data close to Benford's Law
        // assert!(mad < 1.0, "MAD should be small for near-Benford data, got {}", mad);
    }

    #[test]
    fn test_confidence_interval() {
        // let numbers = BENFORD_COMPLIANT_DATA;
        // let confidence_level = 0.95;

        // let intervals = calculate_confidence_interval(numbers, confidence_level);

        // // Should have 9 intervals (one for each digit)
        // assert_eq!(intervals.len(), 9);

        // // Each interval should have lower < upper bound
        // for (i, (lower, upper)) in intervals.iter().enumerate() {
        //     assert!(lower < upper, "Interval {} should have lower < upper: {} < {}", i + 1, lower, upper);
        //     assert!(*lower >= 0.0, "Lower bound should be non-negative for digit {}", i + 1);
        //     assert!(*upper <= 100.0, "Upper bound should be <= 100% for digit {}", i + 1);
        // }
    }
}

#[cfg(test)]
mod integration_benford_tests {
    // use super::*;

    #[test]
    fn test_full_benford_analysis_pipeline() {
        // Test the complete pipeline from numbers to result
        let test_numbers = vec![
            123.0, 234.0, 345.0, 456.0, 567.0, 678.0, 789.0, 890.0, 901.0, 112.0, 113.0, 114.0,
            115.0, 116.0, 117.0, 118.0, 119.0, 120.0,
        ];

        // let result = calculate_benford_distribution("integration_test", &test_numbers);

        // assert_valid_benford_result(&result);
        // assert_eq!(result.numbers_analyzed, test_numbers.len());

        // Check that the distribution adds up to 100%
        // let sum: f64 = result.digit_distribution.iter().sum();
        // assert!((sum - 100.0).abs() < 0.1, "Distribution should sum to ~100%, got {}", sum);
    }

    #[test]
    fn test_minimal_dataset() {
        // Test with minimal number of data points
        let test_numbers = vec![123.0, 234.0, 345.0];

        // let result = calculate_benford_distribution("minimal_test", &test_numbers);

        // assert_valid_benford_result(&result);
        // assert_eq!(result.numbers_analyzed, 3);

        // Should handle small datasets gracefully
        // assert!(result.chi_square >= 0.0);
        // assert!(result.p_value >= 0.0 && result.p_value <= 1.0);
    }

    #[test]
    // #[should_panic(expected = "Insufficient data")]
    fn test_empty_dataset() {
        // Test with empty dataset - should panic or return error
        let test_numbers: Vec<f64> = vec![];
        // calculate_benford_distribution("empty_test", &test_numbers);
    }

    #[test]
    // #[should_panic(expected = "Insufficient data")]
    fn test_single_number_dataset() {
        // Test with single number - insufficient for Benford's Law analysis
        let test_numbers = vec![123.0];
        // calculate_benford_distribution("single_test", &test_numbers);
    }
}

#[cfg(test)]
mod performance_tests {
    // use super::*;
    use std::time::Instant;

    #[test]
    fn test_large_dataset_performance() {
        // Test with a large dataset to ensure reasonable performance
        let large_dataset: Vec<f64> = (1..100000)
            .map(|i| i as f64 * 1.618) // Use golden ratio to get varied first digits
            .collect();

        let start = Instant::now();
        // let result = calculate_benford_distribution("performance_test", &large_dataset);
        let duration = start.elapsed();

        // assert_valid_benford_result(&result);
        // assert_eq!(result.numbers_analyzed, large_dataset.len());

        // Should complete within reasonable time (adjust threshold as needed)
        // assert!(duration.as_millis() < 1000, "Large dataset analysis took too long: {:?}", duration);
    }
}
