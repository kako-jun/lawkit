// Benford's Law calculation tests
#[cfg(test)]
mod benford_distribution_tests {
    use lawkit_core::laws::benford::analysis;

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
        let distribution =
            lawkit_core::laws::benford::analysis::calculate_digit_distribution(&numbers);

        // Should be all zeros for empty input
        let expected = [0.0; 9];
        for (i, &expected_val) in expected.iter().enumerate() {
            assert_eq!(
                distribution[i], expected_val,
                "Empty distribution should be all zeros"
            );
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
}

#[cfg(test)]
mod performance_tests {
    use std::time::Instant;

    #[test]
    fn test_large_dataset_performance() {
        // 大きなデータセットでのパフォーマンステスト
        let large_dataset: Vec<f64> = (1..100000)
            .map(|i| i as f64 * 1.618) // 黄金比を使用して様々な先頭桁を生成
            .collect();

        let start = Instant::now();
        let distribution =
            lawkit_core::laws::benford::analysis::calculate_digit_distribution(&large_dataset);
        let duration = start.elapsed();

        // 分布の合計が100%に近いことを確認
        let sum: f64 = distribution.iter().sum();
        assert!(
            (sum - 100.0).abs() < 0.1,
            "分布の合計は100%付近であるべき、実際: {}",
            sum
        );

        // 合理的な時間内で完了すること
        assert!(
            duration.as_millis() < 1000,
            "大きなデータセット分析に時間がかかりすぎる: {:?}",
            duration
        );
    }
}
