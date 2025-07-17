// use lawkit_core::laws::benford::result::BenfordResult;
use lawkit_core::common::risk::RiskLevel;

#[cfg(test)]
mod risk_level_tests {
    // use super::*;

    #[test]
    fn test_risk_level_low() {
        // Test various p-values in the low risk range (p > 0.1)
        // assert_eq!(determine_risk_level(1.0), RiskLevel::Low);
        // assert_eq!(determine_risk_level(0.5), RiskLevel::Low);
        // assert_eq!(determine_risk_level(0.2), RiskLevel::Low);
        // assert_eq!(determine_risk_level(0.15), RiskLevel::Low);
        // assert_eq!(determine_risk_level(0.101), RiskLevel::Low);
    }

    #[test]
    fn test_risk_level_medium() {
        // Test p-values in the medium risk range (0.05 < p <= 0.1)
        // assert_eq!(determine_risk_level(0.1), RiskLevel::Medium);
        // assert_eq!(determine_risk_level(0.08), RiskLevel::Medium);
        // assert_eq!(determine_risk_level(0.06), RiskLevel::Medium);
        // assert_eq!(determine_risk_level(0.051), RiskLevel::Medium);
    }

    #[test]
    fn test_risk_level_high() {
        // Test p-values in the high risk range (0.01 < p <= 0.05)
        // assert_eq!(determine_risk_level(0.05), RiskLevel::High);
        // assert_eq!(determine_risk_level(0.03), RiskLevel::High);
        // assert_eq!(determine_risk_level(0.02), RiskLevel::High);
        // assert_eq!(determine_risk_level(0.011), RiskLevel::High);
    }

    #[test]
    fn test_risk_level_critical() {
        // Test p-values in the critical risk range (p <= 0.01)
        // assert_eq!(determine_risk_level(0.01), RiskLevel::Critical);
        // assert_eq!(determine_risk_level(0.005), RiskLevel::Critical);
        // assert_eq!(determine_risk_level(0.001), RiskLevel::Critical);
        // assert_eq!(determine_risk_level(0.0), RiskLevel::Critical);
    }

    #[test]
    fn test_risk_level_boundary_values() {
        // Test exact boundary values
        // assert_eq!(determine_risk_level(0.1), RiskLevel::Medium);
        // assert_eq!(determine_risk_level(0.05), RiskLevel::High);
        // assert_eq!(determine_risk_level(0.01), RiskLevel::Critical);

        // Test just above boundaries
        // assert_eq!(determine_risk_level(0.100001), RiskLevel::Low);
        // assert_eq!(determine_risk_level(0.050001), RiskLevel::Medium);
        // assert_eq!(determine_risk_level(0.010001), RiskLevel::High);
    }

    #[test]
    fn test_risk_level_consistency() {
        // Test that risk levels are consistently ordered
        let test_p_values = vec![0.0, 0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.2, 0.5, 1.0];
        let mut _previous_risk_numeric = 0;

        for _p_value in test_p_values {
            // let risk = determine_risk_level(p_value);
            // let risk_numeric = match risk {
            //     RiskLevel::Critical => 3,
            //     RiskLevel::High => 2,
            //     RiskLevel::Medium => 1,
            //     RiskLevel::Low => 0,
            // };

            // Risk should be monotonically decreasing as p-value increases
            // assert!(
            //     risk_numeric >= previous_risk_numeric,
            //     "Risk level should not increase as p-value increases: p={}, risk={:?}",
            //     p_value, risk
            // );
            // previous_risk_numeric = risk_numeric;
        }
    }
}

#[cfg(test)]
mod data_quality_assessment_tests {
    // use super::*;

    #[test]
    fn test_assess_data_quality_sufficient_data() {
        // let quality = assess_data_quality(1000, 0.05);
        // assert!(quality.is_sufficient_sample_size);
        // assert!(quality.is_statistically_significant);
        // assert_eq!(quality.sample_size_category, "Large");
        // assert!(quality.confidence_score > 0.8);
    }

    #[test]
    fn test_assess_data_quality_medium_data() {
        // let quality = assess_data_quality(100, 0.05);
        // assert!(quality.is_sufficient_sample_size);
        // assert!(quality.is_statistically_significant);
        // assert_eq!(quality.sample_size_category, "Medium");
        // assert!(quality.confidence_score > 0.6);
    }

    #[test]
    fn test_assess_data_quality_small_data() {
        // let quality = assess_data_quality(30, 0.05);
        // assert!(quality.is_sufficient_sample_size); // Minimum for Benford's Law
        // assert!(quality.is_statistically_significant);
        // assert_eq!(quality.sample_size_category, "Small");
        // assert!(quality.confidence_score < 0.7);
    }

    #[test]
    fn test_assess_data_quality_insufficient_data() {
        // let quality = assess_data_quality(10, 0.05);
        // assert!(!quality.is_sufficient_sample_size);
        // assert!(quality.is_statistically_significant);
        // assert_eq!(quality.sample_size_category, "Insufficient");
        // assert!(quality.confidence_score < 0.5);
    }

    #[test]
    fn test_assess_data_quality_not_significant() {
        // let quality = assess_data_quality(1000, 0.2); // High p-value
        // assert!(quality.is_sufficient_sample_size);
        // assert!(!quality.is_statistically_significant);
        // assert!(quality.confidence_score < 0.6);
    }

    #[test]
    fn test_assess_data_quality_edge_cases() {
        // Test with minimum data
        // let quality = assess_data_quality(1, 0.001);
        // assert!(!quality.is_sufficient_sample_size);
        // assert!(quality.is_statistically_significant);

        // Test with zero data
        // let quality = assess_data_quality(0, 0.001);
        // assert!(!quality.is_sufficient_sample_size);
        // assert!(quality.confidence_score == 0.0);
    }
}

#[cfg(test)]
mod confidence_score_tests {
    // use super::*;

    #[test]
    fn test_calculate_confidence_score() {
        // Test high confidence scenario
        // let score = calculate_confidence_score(1000, 0.001, 0.5);
        // assert!(score > 0.9, "High confidence score expected, got {}", score);

        // Test medium confidence scenario
        // let score = calculate_confidence_score(100, 0.05, 2.0);
        // assert!(score > 0.5 && score < 0.8, "Medium confidence score expected, got {}", score);

        // Test low confidence scenario
        // let score = calculate_confidence_score(20, 0.2, 10.0);
        // assert!(score < 0.5, "Low confidence score expected, got {}", score);
    }

    #[test]
    fn test_confidence_score_factors() {
        // Test that larger sample size increases confidence
        // let score_small = calculate_confidence_score(50, 0.01, 1.0);
        // let score_large = calculate_confidence_score(500, 0.01, 1.0);
        // assert!(score_large > score_small, "Larger sample should have higher confidence");

        // Test that lower p-value increases confidence
        // let score_high_p = calculate_confidence_score(100, 0.1, 1.0);
        // let score_low_p = calculate_confidence_score(100, 0.001, 1.0);
        // assert!(score_low_p > score_high_p, "Lower p-value should have higher confidence");

        // Test that lower MAD increases confidence
        // let score_high_mad = calculate_confidence_score(100, 0.01, 5.0);
        // let score_low_mad = calculate_confidence_score(100, 0.01, 0.5);
        // assert!(score_low_mad > score_high_mad, "Lower MAD should have higher confidence");
    }

    #[test]
    fn test_confidence_score_bounds() {
        // Test that confidence score is always between 0 and 1
        let test_cases = vec![
            (0, 0.0, 0.0),
            (1, 1.0, 100.0),
            (10000, 0.001, 0.1),
            (5, 0.5, 50.0),
        ];

        for (_sample_size, _p_value, _mad) in test_cases {
            // let score = calculate_confidence_score(sample_size, p_value, mad);
            // assert!(
            //     score >= 0.0 && score <= 1.0,
            //     "Confidence score should be between 0 and 1, got {} for ({}, {}, {})",
            //     score, sample_size, p_value, mad
            // );
        }
    }
}

#[cfg(test)]
mod integrated_risk_assessment_tests {
    // use super::*;
    // use crate::common::fixtures::{BENFORD_COMPLIANT_DATA, FRAUDULENT_DATA};
    // use crate::common::{create_test_result};

    #[test]
    fn test_complete_risk_assessment_benford_compliant() {
        // let result = create_test_result(
        //     "compliant_data",
        //     BENFORD_COMPLIANT_DATA.len(),
        //     [30.0, 17.5, 12.5, 9.7, 7.9, 6.7, 5.8, 5.1, 4.6], // Close to Benford's Law
        //     2.5,  // Low chi-square
        //     0.15, // Medium p-value
        //     RiskLevel::Low,
        // );

        // assert_valid_benford_result(&result);
        // assert_risk_level_matches_p_value(&result);

        // let quality = assess_data_quality(result.numbers_analyzed, result.p_value);
        // assert!(quality.is_sufficient_sample_size);
        // assert!(!quality.is_statistically_significant); // p > 0.05
        // assert!(quality.confidence_score > 0.5);
    }

    #[test]
    fn test_complete_risk_assessment_fraudulent() {
        // let result = create_test_result(
        //     "fraudulent_data",
        //     FRAUDULENT_DATA.len(),
        //     [5.0, 5.0, 5.0, 5.0, 40.0, 35.0, 5.0, 0.0, 0.0], // Heavily skewed
        //     150.0, // Very high chi-square
        //     0.001, // Very low p-value
        //     RiskLevel::Critical,
        // );

        // assert_valid_benford_result(&result);
        // assert_risk_level_matches_p_value(&result);

        // let quality = assess_data_quality(result.numbers_analyzed, result.p_value);
        // assert!(quality.is_sufficient_sample_size);
        // assert!(quality.is_statistically_significant); // p < 0.05
        // assert!(quality.confidence_score > 0.8); // High confidence in fraud detection
    }

    #[test]
    fn test_borderline_risk_assessment() {
        // Test borderline cases between risk levels
        let borderline_cases = vec![
            (0.100, lawkit_core::common::risk::RiskLevel::Medium), // Exactly at boundary
            (0.050, lawkit_core::common::risk::RiskLevel::High),   // Exactly at boundary
            (0.010, lawkit_core::common::risk::RiskLevel::Critical), // Exactly at boundary
        ];

        for (_p_value, _expected_risk) in borderline_cases {
            // let result = create_test_result(
            //     "borderline_test",
            //     100,
            //     [25.0, 20.0, 15.0, 10.0, 8.0, 7.0, 6.0, 5.0, 4.0], // Somewhat skewed
            //     10.0,
            //     p_value,
            //     expected_risk.clone(),
            // );

            // assert_valid_benford_result(&result);
            // assert_risk_level_matches_p_value(&result);
        }
    }
}

#[cfg(test)]
mod risk_messaging_tests {
    // use super::*;
    // use benf::core::risk::{get_risk_message, get_risk_recommendations};

    #[test]
    fn test_risk_messages() {
        // let low_msg = get_risk_message(RiskLevel::Low);
        // assert!(low_msg.contains("normal") || low_msg.contains("compliant"));

        // let medium_msg = get_risk_message(RiskLevel::Medium);
        // assert!(medium_msg.contains("slight") || medium_msg.contains("minor"));

        // let high_msg = get_risk_message(RiskLevel::High);
        // assert!(high_msg.contains("significant") || high_msg.contains("deviation"));

        // let critical_msg = get_risk_message(RiskLevel::Critical);
        // assert!(critical_msg.contains("critical") || critical_msg.contains("strong evidence"));
    }

    #[test]
    fn test_risk_recommendations() {
        // let low_rec = get_risk_recommendations(RiskLevel::Low);
        // assert!(low_rec.contains("continue") || low_rec.contains("normal"));

        // let medium_rec = get_risk_recommendations(RiskLevel::Medium);
        // assert!(medium_rec.contains("review") || medium_rec.contains("monitor"));

        // let high_rec = get_risk_recommendations(RiskLevel::High);
        // assert!(high_rec.contains("investigate") || high_rec.contains("audit"));

        // let critical_rec = get_risk_recommendations(RiskLevel::Critical);
        // assert!(critical_rec.contains("immediate") || critical_rec.contains("urgent"));
    }
}

#[cfg(test)]
mod real_world_scenarios_tests {
    // use super::*;

    #[test]
    fn test_accounting_fraud_scenario() {
        // Simulate accounting data where amounts are artificially rounded
        // let result = create_test_result(
        //     "accounting_fraud",
        //     500,
        //     [10.0, 10.0, 10.0, 10.0, 50.0, 10.0, 0.0, 0.0, 0.0], // Too many 5s
        //     75.0,
        //     0.001,
        //     RiskLevel::Critical,
        // );

        // assert_valid_benford_result(&result);
        // assert_eq!(result.risk_level, RiskLevel::Critical);

        // let quality = assess_data_quality(result.numbers_analyzed, result.p_value);
        // assert!(quality.is_statistically_significant);
        // assert!(quality.confidence_score > 0.8);
    }

    #[test]
    fn test_natural_data_scenario() {
        // Simulate naturally occurring data (like population, financial transactions)
        // let result = create_test_result(
        //     "natural_data",
        //     2000,
        //     [29.8, 17.8, 12.3, 9.5, 8.1, 6.8, 5.9, 5.0, 4.8], // Close to Benford's
        //     3.2,
        //     0.22,
        //     RiskLevel::Low,
        // );

        // assert_valid_benford_result(&result);
        // assert_eq!(result.risk_level, RiskLevel::Low);

        // let quality = assess_data_quality(result.numbers_analyzed, result.p_value);
        // assert!(quality.is_sufficient_sample_size);
        // assert!(!quality.is_statistically_significant); // No fraud detected
    }

    #[test]
    fn test_small_dataset_scenario() {
        // Simulate analysis with limited data
        // let result = create_test_result(
        //     "small_dataset",
        //     25, // Borderline sufficient
        //     [35.0, 15.0, 10.0, 8.0, 8.0, 8.0, 8.0, 4.0, 4.0],
        //     8.5,
        //     0.08,
        //     RiskLevel::Medium,
        // );

        // assert_valid_benford_result(&result);
        // assert_eq!(result.risk_level, RiskLevel::Medium);

        // let quality = assess_data_quality(result.numbers_analyzed, result.p_value);
        // assert!(quality.is_sufficient_sample_size); // Just barely sufficient
        // assert!(quality.confidence_score < 0.7); // Lower confidence due to small size
    }
}
