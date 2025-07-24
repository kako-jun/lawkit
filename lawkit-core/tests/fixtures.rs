use serde_json::{json, Value};
use std::path::Path;

/// Common test fixtures shared across lawkit core/python/js tests
/// These fixtures are focused on statistical law analysis unified API testing

pub struct TestFixtures;

impl TestFixtures {
    /// Get path to shared CLI fixtures directory
    pub fn cli_fixtures_dir() -> &'static str {
        "../../tests/fixtures"
    }
    
    /// Load JSON file from CLI fixtures
    pub fn load_cli_fixture(filename: &str) -> Value {
        let path = format!("{}/{}", Self::cli_fixtures_dir(), filename);
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read fixture: {}", path));
        
        if filename.ends_with(".json") {
            serde_json::from_str(&content).unwrap()
        } else {
            panic!("Only JSON fixtures supported in unified API tests")
        }
    }
    
    /// Basic configuration comparison fixtures (shared with diffx/diffai)
    pub fn config_v1() -> Value {
        Self::load_cli_fixture("config_v1.json")
    }
    
    pub fn config_v2() -> Value {
        Self::load_cli_fixture("config_v2.json")
    }
    
    /// Statistical law analysis specific test fixtures
    
    /// Benford's law test data fixtures
    pub fn benford_compliant_data() -> Value {
        json!({
            "financial_data": [
                123.45, 187.92, 234.67, 298.34, 345.78, 456.23, 567.89, 678.12, 789.56,
                1234.56, 1876.43, 2345.67, 2987.34, 3456.78, 4567.89, 5678.12, 6789.34, 7890.45,
                12345.67, 18765.43, 23456.78, 29876.54, 34567.89, 45678.12, 56789.34, 67890.45, 78901.23,
                123456.78, 187654.32, 234567.89, 298765.43, 345678.91, 456789.12, 567890.23, 678901.34
            ],
            "invoice_amounts": [
                101.50, 125.00, 198.75, 267.30, 334.20, 445.60, 523.80, 612.40, 785.90,
                1055.25, 1287.60, 1934.75, 2156.80, 3245.70, 4123.50, 5678.25, 6234.80, 7891.20,
                10234.50, 12876.30, 19847.60, 21568.90, 32457.80, 41235.60, 56782.40, 62348.70, 78912.30
            ]
        })
    }
    
    pub fn benford_non_compliant_data() -> Value {
        json!({
            "uniform_data": [
                500.0, 501.0, 502.0, 503.0, 504.0, 505.0, 506.0, 507.0, 508.0, 509.0,
                510.0, 511.0, 512.0, 513.0, 514.0, 515.0, 516.0, 517.0, 518.0, 519.0,
                520.0, 521.0, 522.0, 523.0, 524.0, 525.0, 526.0, 527.0, 528.0, 529.0
            ],
            "suspicious_data": [
                7000.0, 7001.0, 7002.0, 7003.0, 7004.0, 7005.0, 7006.0, 7007.0, 7008.0, 7009.0,
                8000.0, 8001.0, 8002.0, 8003.0, 8004.0, 8005.0, 8006.0, 8007.0, 8008.0, 8009.0,
                9000.0, 9001.0, 9002.0, 9003.0, 9004.0, 9005.0, 9006.0, 9007.0, 9008.0, 9009.0
            ]
        })
    }
    
    /// Pareto principle test data fixtures
    pub fn pareto_compliant_data() -> Value {
        json!({
            "sales_data": [
                // Top 20% should contribute ~80% of total
                10000.0, 9500.0, 9000.0, 8500.0, // Top 4 items (20% of 20 items)
                1000.0, 950.0, 900.0, 850.0, 800.0, 750.0, 700.0, 650.0,
                600.0, 550.0, 500.0, 450.0, 400.0, 350.0, 300.0, 250.0
            ],
            "customer_revenue": [
                50000.0, 45000.0, 40000.0, 35000.0, 30000.0, // Top 5 customers
                2000.0, 1900.0, 1800.0, 1700.0, 1600.0, 1500.0, 1400.0, 1300.0,
                1200.0, 1100.0, 1000.0, 900.0, 800.0, 700.0, 600.0, 500.0, 400.0, 300.0, 200.0, 100.0
            ]
        })
    }
    
    pub fn pareto_non_compliant_data() -> Value {
        json!({
            "uniform_distribution": [
                1000.0, 1010.0, 1020.0, 1030.0, 1040.0, 1050.0, 1060.0, 1070.0, 1080.0, 1090.0,
                1100.0, 1110.0, 1120.0, 1130.0, 1140.0, 1150.0, 1160.0, 1170.0, 1180.0, 1190.0
            ],
            "reverse_pareto": [
                100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 1000.0,
                1100.0, 1200.0, 1300.0, 1400.0, 1500.0, 1600.0, 1700.0, 1800.0, 1900.0, 2000.0
            ]
        })
    }
    
    /// Zipf's law test data fixtures
    pub fn zipf_compliant_data() -> Value {
        json!({
            "word_frequencies": [
                // Frequencies following Zipf's law: f(r) = f(1)/r
                10000.0, 5000.0, 3333.33, 2500.0, 2000.0, 1666.67, 1428.57, 1250.0, 1111.11, 1000.0,
                909.09, 833.33, 769.23, 714.29, 666.67, 625.0, 588.24, 555.56, 526.32, 500.0
            ],
            "website_visits": [
                100000.0, 50000.0, 33333.33, 25000.0, 20000.0, 16666.67, 14285.71, 12500.0, 11111.11, 10000.0,
                9090.91, 8333.33, 7692.31, 7142.86, 6666.67, 6250.0, 5882.35, 5555.56, 5263.16, 5000.0
            ]
        })
    }
    
    pub fn zipf_non_compliant_data() -> Value {
        json!({
            "random_frequencies": [
                5000.0, 3200.0, 7800.0, 1200.0, 9500.0, 2100.0, 6700.0, 4300.0, 8900.0, 1800.0,
                3400.0, 7600.0, 2700.0, 5800.0, 9100.0, 1500.0, 6200.0, 4700.0, 8300.0, 2900.0
            ],
            "exponential_decay": [
                10000.0, 9000.0, 8100.0, 7290.0, 6561.0, 5904.9, 5314.41, 4782.97, 4304.67, 3874.2
            ]
        })
    }
    
    /// Normal distribution test data fixtures
    pub fn normal_distribution_data() -> Value {
        json!({
            "normal_sample": [
                98.5, 99.2, 100.1, 99.8, 100.4, 99.6, 100.8, 99.9, 100.2, 99.7,
                100.3, 99.4, 100.6, 99.1, 100.9, 99.3, 100.5, 99.0, 101.0, 99.8,
                100.0, 99.5, 100.7, 99.2, 100.3, 99.6, 100.1, 99.9, 100.4, 99.7
            ],
            "quality_measurements": [
                2.1, 2.0, 2.3, 1.9, 2.2, 2.0, 2.4, 1.8, 2.1, 2.2,
                1.9, 2.3, 2.0, 2.1, 1.8, 2.4, 2.2, 1.9, 2.0, 2.3,
                2.1, 1.8, 2.4, 2.0, 2.2, 1.9, 2.3, 2.1, 2.0, 1.8
            ]
        })
    }
    
    pub fn non_normal_distribution_data() -> Value {
        json!({
            "skewed_data": [
                1.0, 1.1, 1.2, 1.3, 1.5, 1.8, 2.2, 2.8, 3.6, 4.7,
                6.1, 8.0, 10.4, 13.5, 17.6, 22.9, 29.8, 38.7, 50.3, 65.4,
                85.0, 110.5, 143.7, 186.8, 242.8, 315.6, 410.3, 533.4, 693.4, 901.4
            ],
            "bimodal_data": [
                5.0, 5.1, 5.2, 4.9, 5.3, 4.8, 5.4, 4.7, 5.5, 4.6,
                15.0, 15.1, 15.2, 14.9, 15.3, 14.8, 15.4, 14.7, 15.5, 14.6,
                5.0, 5.1, 5.2, 4.9, 5.3, 4.8, 5.4, 4.7, 5.5, 4.6
            ]
        })
    }
    
    /// Poisson distribution test data fixtures
    pub fn poisson_distribution_data() -> Value {
        json!({
            "event_counts": [
                0, 1, 2, 1, 3, 0, 2, 1, 4, 2, 1, 0, 3, 2, 1, 5, 0, 2, 1, 3,
                2, 1, 0, 4, 2, 1, 3, 0, 2, 1, 2, 3, 1, 0, 2, 1, 4, 2, 0, 3
            ],
            "customer_arrivals": [
                2, 3, 1, 4, 2, 0, 3, 2, 5, 1, 2, 3, 0, 4, 2, 1, 3, 2, 6, 0,
                3, 2, 1, 4, 2, 3, 0, 2, 1, 5, 3, 2, 1, 0, 4, 2, 3, 1, 2, 0
            ]
        })
    }
    
    pub fn non_poisson_data() -> Value {
        json!({
            "high_variance": [
                0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 0, 0, 0, 0, 0,
                100, 100, 100, 100, 100, 0, 0, 0, 0, 0, 25, 25, 25, 25, 25
            ],
            "negative_values": [
                -1, -2, 0, 1, 2, -3, 0, 1, -1, 2, 0, -2, 1, 0, -1,
                2, -1, 0, 1, -2, 0, 1, -1, 2, 0, -1, 1, 0, -2, 1
            ]
        })
    }
    
    /// Integration analysis test data
    pub fn integration_analysis_data() -> Value {
        json!({
            "comprehensive_dataset": {
                "financial_transactions": [
                    123.45, 187.92, 234.67, 298.34, 345.78, 456.23, 567.89, 678.12, 789.56,
                    1234.56, 1876.43, 2345.67, 2987.34, 3456.78, 4567.89, 5678.12, 6789.34, 7890.45
                ],
                "sales_amounts": [
                    50000.0, 45000.0, 40000.0, 35000.0, 30000.0,
                    2000.0, 1900.0, 1800.0, 1700.0, 1600.0, 1500.0, 1400.0, 1300.0,
                    1200.0, 1100.0, 1000.0, 900.0, 800.0, 700.0, 600.0
                ],
                "quality_scores": [
                    98.5, 99.2, 100.1, 99.8, 100.4, 99.6, 100.8, 99.9, 100.2, 99.7,
                    100.3, 99.4, 100.6, 99.1, 100.9, 99.3, 100.5, 99.0, 101.0, 99.8
                ],
                "incident_counts": [
                    0, 1, 2, 1, 3, 0, 2, 1, 4, 2, 1, 0, 3, 2, 1, 5, 0, 2, 1, 3
                ]
            }
        })
    }
    
    /// Validation test data fixtures
    pub fn validation_test_data() -> Value {
        json!({
            "valid_dataset": [
                123.45, 234.67, 345.89, 456.12, 567.34, 678.56, 789.78, 890.23, 901.45, 123.67,
                234.89, 345.12, 456.34, 567.56, 678.78, 789.01, 890.23, 901.45, 123.67, 234.89
            ],
            "invalid_dataset": [
                123.45, null, 345.89, "invalid", 567.34, "infinity", 789.78, "NaN", 901.45, 123.67
            ],
            "small_dataset": [1.0, 2.0, 3.0],
            "empty_dataset": []
        })
    }
    
    /// Diagnostic test data fixtures
    pub fn diagnostic_test_data() -> Value {
        json!({
            "normal_with_outliers": [
                98.5, 99.2, 100.1, 99.8, 100.4, 99.6, 100.8, 99.9, 100.2, 99.7,
                100.3, 99.4, 100.6, 99.1, 100.9, 99.3, 100.5, 99.0, 101.0, 99.8,
                150.0, // Outlier
                100.0, 99.5, 100.7, 99.2, 100.3, 99.6, 100.1, 99.9, 100.4, 99.7,
                50.0   // Another outlier
            ],
            "skewed_distribution": [
                1.0, 1.1, 1.2, 1.3, 1.5, 1.8, 2.2, 2.8, 3.6, 4.7,
                6.1, 8.0, 10.4, 13.5, 17.6, 22.9, 29.8, 38.7, 50.3, 65.4
            ]
        })
    }
    
    /// Generation configuration fixtures
    pub fn generation_configs() -> Value {
        json!({
            "benford_config": {
                "type": "benford",
                "count": 1000,
                "base": 10
            },
            "normal_config": {
                "type": "normal",
                "count": 500,
                "mean": 100.0,
                "std_dev": 15.0
            },
            "poisson_config": {
                "type": "poisson", 
                "count": 300,
                "lambda": 5.0
            }
        })
    }
}

/// Helper functions for statistical law test data generation
pub mod law_generators {
    use super::*;
    
    pub fn generate_benford_data(count: usize, risk_level: &str) -> Value {
        let data = match risk_level {
            "low" => {
                // Generate data that follows Benford's law
                let mut values = Vec::new();
                for i in 1..=count {
                    let base = 1.0 + (i as f64 * 1.234567).fract() * 8.0; // 1-9 range
                    let magnitude = 10.0_f64.powf((i % 5) as f64);
                    values.push(base * magnitude);
                }
                values
            },
            "high" => {
                // Generate data that doesn't follow Benford's law (suspicious pattern)
                let mut values = Vec::new();
                for i in 0..count {
                    let suspicious_digit = 7.0 + (i % 3) as f64; // Mostly 7, 8, 9
                    let magnitude = 1000.0 + i as f64;
                    values.push(suspicious_digit * magnitude);
                }
                values
            },
            _ => {
                // Medium risk - mixed pattern
                let mut values = Vec::new();
                for i in 0..count {
                    if i % 3 == 0 {
                        // Some Benford-compliant
                        let base = 1.0 + (i as f64 * 1.234567).fract() * 8.0;
                        let magnitude = 10.0_f64.powf((i % 4) as f64);
                        values.push(base * magnitude);
                    } else {
                        // Some suspicious
                        let suspicious_digit = 8.0;
                        let magnitude = 1000.0 + i as f64;
                        values.push(suspicious_digit * magnitude);
                    }
                }
                values
            }
        };
        
        json!(data)
    }
    
    pub fn generate_pareto_data(count: usize, compliance: bool) -> Value {
        let mut values = Vec::new();
        
        if compliance {
            // Top 20% contributes ~80%
            let top_20_count = (count as f64 * 0.2) as usize;
            let bottom_80_count = count - top_20_count;
            
            // High values for top 20%
            for i in 0..top_20_count {
                values.push(10000.0 - i as f64 * 200.0);
            }
            
            // Low values for bottom 80%
            for i in 0..bottom_80_count {
                values.push(100.0 + i as f64 * 10.0);
            }
        } else {
            // Uniform distribution (non-Pareto)
            for i in 0..count {
                values.push(1000.0 + i as f64 * 10.0);
            }
        }
        
        json!(values)
    }
    
    pub fn generate_zipf_data(count: usize, exponent: f64) -> Value {
        let mut values = Vec::new();
        
        for rank in 1..=count {
            // Zipf distribution: f(k) = C / k^s
            let frequency = 10000.0 / (rank as f64).powf(exponent);
            values.push(frequency);
        }
        
        json!(values)
    }
    
    pub fn generate_normal_data(count: usize, mean: f64, std_dev: f64, add_outliers: bool) -> Value {
        let mut values = Vec::new();
        
        // Simple normal-ish data generation
        for i in 0..count {
            let u1 = (i as f64 + 1.0) / (count as f64 + 1.0);
            let u2 = ((i * 7 + 3) % count) as f64 / count as f64;
            
            // Box-Muller transformation
            let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
            let value = mean + std_dev * z;
            values.push(value);
        }
        
        if add_outliers {
            // Add some outliers
            values.push(mean + 5.0 * std_dev);
            values.push(mean - 5.0 * std_dev);
        }
        
        json!(values)
    }
    
    pub fn generate_poisson_data(count: usize, lambda: f64, add_variance: bool) -> Value {
        let mut values = Vec::new();
        
        if add_variance {
            // Non-Poisson data with high variance
            for i in 0..count {
                if i % 10 == 0 {
                    values.push((lambda * 10.0) as u32);
                } else {
                    values.push(0);
                }
            }
        } else {
            // Approximate Poisson data
            for i in 0..count {
                let u = (i as f64 + 0.5) / (count as f64 + 1.0);
                let k = (-lambda * u.ln()).floor().max(0.0) as u32;
                values.push(k);
            }
        }
        
        json!(values)
    }
    
    pub fn generate_integration_test_data() -> Value {
        json!({
            "benford_data": generate_benford_data(100, "low"),
            "pareto_data": generate_pareto_data(50, true),
            "normal_data": generate_normal_data(80, 100.0, 15.0, false),
            "poisson_data": generate_poisson_data(60, 3.0, false)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_benford_fixtures() {
        let compliant = TestFixtures::benford_compliant_data();
        let non_compliant = TestFixtures::benford_non_compliant_data();
        
        assert!(compliant["financial_data"].is_array());
        assert!(non_compliant["uniform_data"].is_array());
        
        // Check that we have different data patterns
        let financial_data = &compliant["financial_data"].as_array().unwrap();
        let uniform_data = &non_compliant["uniform_data"].as_array().unwrap();
        
        assert!(financial_data.len() > 0);
        assert!(uniform_data.len() > 0);
        assert_ne!(financial_data[0], uniform_data[0]);
    }
    
    #[test]
    fn test_pareto_fixtures() {
        let compliant = TestFixtures::pareto_compliant_data();
        let non_compliant = TestFixtures::pareto_non_compliant_data();
        
        assert!(compliant["sales_data"].is_array());
        assert!(non_compliant["uniform_distribution"].is_array());
        
        // Pareto data should have high variance
        let sales_data = compliant["sales_data"].as_array().unwrap();
        let first_val = sales_data[0].as_f64().unwrap();
        let last_val = sales_data[sales_data.len() - 1].as_f64().unwrap();
        assert!(first_val > last_val * 2.0); // Should have large range
    }
    
    #[test]
    fn test_law_generators() {
        let benford_low = law_generators::generate_benford_data(100, "low");
        let benford_high = law_generators::generate_benford_data(100, "high");
        
        assert!(benford_low.is_array());
        assert!(benford_high.is_array());
        
        let pareto_compliant = law_generators::generate_pareto_data(50, true);
        let pareto_non_compliant = law_generators::generate_pareto_data(50, false);
        
        assert!(pareto_compliant.is_array());
        assert!(pareto_non_compliant.is_array());
        
        // Test that generators produce different patterns
        assert_ne!(benford_low, benford_high);
        assert_ne!(pareto_compliant, pareto_non_compliant);
    }
    
    #[test]
    fn test_normal_data_generation() {
        let normal_data = law_generators::generate_normal_data(100, 50.0, 10.0, false);
        let normal_with_outliers = law_generators::generate_normal_data(100, 50.0, 10.0, true);
        
        assert!(normal_data.is_array());
        assert!(normal_with_outliers.is_array());
        
        // With outliers should have more data points
        let normal_len = normal_data.as_array().unwrap().len();
        let outlier_len = normal_with_outliers.as_array().unwrap().len();
        assert!(outlier_len > normal_len);
    }
    
    #[test]
    fn test_poisson_data_generation() {
        let poisson_normal = law_generators::generate_poisson_data(50, 3.0, false);
        let poisson_high_var = law_generators::generate_poisson_data(50, 3.0, true);
        
        assert!(poisson_normal.is_array());
        assert!(poisson_high_var.is_array());
        
        // High variance version should be different
        assert_ne!(poisson_normal, poisson_high_var);
    }
    
    #[test]
    fn test_integration_test_data() {
        let integration_data = law_generators::generate_integration_test_data();
        
        assert!(integration_data["benford_data"].is_array());
        assert!(integration_data["pareto_data"].is_array());
        assert!(integration_data["normal_data"].is_array());
        assert!(integration_data["poisson_data"].is_array());
    }
}