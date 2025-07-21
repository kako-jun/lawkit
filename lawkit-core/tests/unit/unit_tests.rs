// Unit tests for lawkit core components
// Test individual functions and modules in isolation

#[cfg(test)]
mod tests {
    use lawkit_core::laws::benford::analysis::{
        get_first_digit, calculate_digit_distribution, BENFORD_EXPECTED_PERCENTAGES
    };
    use lawkit_core::laws::pareto::analysis::{
        calculate_concentration, find_critical_percentage
    };
    use lawkit_core::laws::zipf::analysis::{
        calculate_rank_frequency_distribution, calculate_zipf_coefficient
    };
    use lawkit_core::laws::normal::analysis::{
        calculate_mean, calculate_std_dev, test_normality
    };
    use lawkit_core::laws::poisson::analysis::{
        calculate_lambda, test_poisson_fit
    };

    mod benford_tests {
        use super::*;

        #[test]
        fn test_get_first_digit() {
            assert_eq!(get_first_digit(123.45), Some(1));
            assert_eq!(get_first_digit(987.65), Some(9));
            assert_eq!(get_first_digit(0.5), None);
            assert_eq!(get_first_digit(-456.78), Some(4));
            assert_eq!(get_first_digit(5000.0), Some(5));
        }

        #[test]
        fn test_calculate_digit_distribution() {
            let numbers = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
            let distribution = calculate_digit_distribution(&numbers);
            
            // Check that distribution sums to approximately 100%
            let sum: f64 = distribution.iter().sum();
            assert!((sum - 100.0).abs() < 0.01);
            
            // Check specific distributions
            assert!(distribution[0] > 30.0); // digit 1
            assert!(distribution[1] > 30.0); // digit 2
            assert!(distribution[2] > 30.0); // digit 3
        }

        #[test]
        fn test_benford_expected_percentages() {
            // Verify the expected percentages sum to 100%
            let sum: f64 = BENFORD_EXPECTED_PERCENTAGES.iter().sum();
            assert!((sum - 100.0).abs() < 0.01);
            
            // Verify descending order
            for i in 0..8 {
                assert!(BENFORD_EXPECTED_PERCENTAGES[i] > BENFORD_EXPECTED_PERCENTAGES[i + 1]);
            }
        }
    }

    mod pareto_tests {
        use super::*;

        #[test]
        fn test_calculate_concentration() {
            let values = vec![100.0, 50.0, 30.0, 10.0, 5.0, 3.0, 1.0, 1.0];
            let concentration = calculate_concentration(&values, 0.2);
            
            // Top 20% should have more than 50% of total value
            assert!(concentration > 50.0);
        }

        #[test]
        fn test_find_critical_percentage() {
            let values = vec![100.0, 50.0, 30.0, 10.0, 5.0, 3.0, 1.0, 1.0];
            let critical = find_critical_percentage(&values, 80.0);
            
            // Should find percentage that captures 80% of value
            assert!(critical > 0.0 && critical < 100.0);
        }
    }

    mod zipf_tests {
        use super::*;

        #[test]
        fn test_calculate_rank_frequency_distribution() {
            let values = vec![100.0, 50.0, 33.0, 25.0, 20.0];
            let distribution = calculate_rank_frequency_distribution(&values);
            
            assert_eq!(distribution.len(), values.len());
            // First rank should have highest frequency
            assert!(distribution[0].1 > distribution[1].1);
        }

        #[test]
        fn test_calculate_zipf_coefficient() {
            let distribution = vec![(1, 100.0), (2, 50.0), (3, 33.0), (4, 25.0)];
            let coefficient = calculate_zipf_coefficient(&distribution);
            
            // For perfect Zipf's law, coefficient should be close to 1.0
            assert!(coefficient > 0.5 && coefficient < 1.5);
        }
    }

    mod normal_tests {
        use super::*;

        #[test]
        fn test_calculate_mean() {
            let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
            let mean = calculate_mean(&values);
            assert!((mean - 3.0).abs() < 0.01);
        }

        #[test]
        fn test_calculate_std_dev() {
            let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
            let mean = calculate_mean(&values);
            let std_dev = calculate_std_dev(&values, mean);
            assert!(std_dev > 1.0 && std_dev < 2.0);
        }

        #[test]
        fn test_normality() {
            // Generate normally distributed data
            let mut values = Vec::new();
            for i in -50..=50 {
                let x = i as f64 / 10.0;
                let freq = (-(x * x) / 2.0).exp();
                for _ in 0..(freq * 10.0) as usize {
                    values.push(x);
                }
            }
            
            let result = test_normality(&values);
            assert!(result.is_normal);
        }
    }

    mod poisson_tests {
        use super::*;

        #[test]
        fn test_calculate_lambda() {
            let values = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
            let lambda = calculate_lambda(&values);
            assert!((lambda - 2.5).abs() < 0.1);
        }

        #[test]
        fn test_poisson_fit() {
            // Generate Poisson-like data
            let values = vec![
                0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0,
                3.0, 3.0, 3.0, 4.0, 4.0, 5.0
            ];
            let result = test_poisson_fit(&values);
            assert!(result.good_fit);
        }
    }
}