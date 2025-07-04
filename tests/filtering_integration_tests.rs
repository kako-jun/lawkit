use crate::common::filtering::{NumberFilter, RiskThreshold, apply_number_filter};
use crate::laws::benford::BenfordResult;
use crate::error::BenfError;
use std::str::FromStr;

#[test]
fn test_number_filter_integration() {
    let test_numbers = vec![10.0, 50.0, 100.0, 150.0, 200.0, 300.0, 500.0, 1000.0];
    
    // Test >= filter
    let ge_filter = NumberFilter::parse(">=100").unwrap();
    let filtered = apply_number_filter(&test_numbers, &ge_filter);
    assert_eq!(filtered, vec![100.0, 150.0, 200.0, 300.0, 500.0, 1000.0]);
    
    // Test range filter
    let range_filter = NumberFilter::parse("100-500").unwrap();
    let filtered = apply_number_filter(&test_numbers, &range_filter);
    assert_eq!(filtered, vec![100.0, 150.0, 200.0, 300.0, 500.0]);
    
    // Test < filter
    let lt_filter = NumberFilter::parse("<200").unwrap();
    let filtered = apply_number_filter(&test_numbers, &lt_filter);
    assert_eq!(filtered, vec![10.0, 50.0, 100.0, 150.0]);
}

#[test]
fn test_risk_threshold_integration() {
    let test_numbers = vec![111.0, 222.0, 333.0, 444.0, 555.0, 666.0, 777.0, 888.0, 999.0, 1000.0];
    
    // Test auto threshold
    let auto_threshold = RiskThreshold::Auto;
    let result = BenfordResult::new_with_threshold("test".to_string(), &test_numbers, &auto_threshold, 5).unwrap();
    // Auto threshold should use standard Benford evaluation
    
    // Test custom threshold
    let low_threshold = RiskThreshold::Low;
    let result_low = BenfordResult::new_with_threshold("test".to_string(), &test_numbers, &low_threshold, 5).unwrap();
    
    // Both should analyze the same numbers but may have different risk evaluations
    assert_eq!(result.numbers_analyzed, result_low.numbers_analyzed);
    assert_eq!(result.digit_distribution, result_low.digit_distribution);
}

#[test]
fn test_minimum_count_requirement() {
    let small_dataset = vec![100.0, 200.0]; // Only 2 numbers
    let large_dataset = vec![100.0, 200.0, 300.0, 400.0, 500.0, 600.0]; // 6 numbers
    
    // Should fail with minimum count of 5
    let result_small = BenfordResult::new_with_threshold("small".to_string(), &small_dataset, &RiskThreshold::Auto, 5);
    assert!(result_small.is_err());
    match result_small {
        Err(BenfError::InsufficientData(count)) => {
            assert_eq!(count, 2);
        }
        _ => panic!("Expected InsufficientData error"),
    }
    
    // Should succeed with minimum count of 5
    let result_large = BenfordResult::new_with_threshold("large".to_string(), &large_dataset, &RiskThreshold::Auto, 5);
    assert!(result_large.is_ok());
    
    // Should succeed with lower minimum count
    let result_small_min = BenfordResult::new_with_threshold("small".to_string(), &small_dataset, &RiskThreshold::Auto, 2);
    assert!(result_small_min.is_ok());
}

#[test]
fn test_filter_with_benford_analysis() {
    // Create a larger dataset that would normally pass minimum requirements
    let numbers = vec![
        111.0, 122.0, 133.0, 144.0, 155.0, 166.0, 177.0, 188.0, 199.0, 200.0,
        211.0, 222.0, 233.0, 244.0, 255.0, 266.0, 277.0, 288.0, 299.0, 300.0,
        311.0, 322.0, 333.0, 444.0, 555.0, 666.0, 777.0, 888.0, 999.0, 1000.0
    ];
    
    // Filter to only numbers >= 500
    let filter = NumberFilter::parse(">=500").unwrap();
    let filtered_numbers = apply_number_filter(&numbers, &filter);
    
    // Should have fewer numbers after filtering
    assert!(filtered_numbers.len() < numbers.len());
    
    // All filtered numbers should be >= 500
    for &num in &filtered_numbers {
        assert!(num >= 500.0);
    }
    
    // Should still be able to perform Benford analysis on filtered data
    let result = BenfordResult::new_with_threshold("filtered".to_string(), &filtered_numbers, &RiskThreshold::Auto, 1);
    assert!(result.is_ok());
}

#[test]
fn test_custom_threshold_parsing() {
    // Test string parsing for different threshold levels
    assert_eq!(RiskThreshold::from_str("auto").unwrap(), RiskThreshold::Auto);
    assert_eq!(RiskThreshold::from_str("low").unwrap(), RiskThreshold::Low);
    assert_eq!(RiskThreshold::from_str("medium").unwrap(), RiskThreshold::Medium);
    assert_eq!(RiskThreshold::from_str("high").unwrap(), RiskThreshold::High);
    assert_eq!(RiskThreshold::from_str("critical").unwrap(), RiskThreshold::Critical);
    
    // Test custom p-value
    assert_eq!(RiskThreshold::from_str("0.03").unwrap(), RiskThreshold::Custom(0.03));
    
    // Test invalid inputs
    assert!(RiskThreshold::from_str("invalid").is_err());
    assert!(RiskThreshold::from_str("2.0").is_err()); // Out of range
    assert!(RiskThreshold::from_str("-0.1").is_err()); // Negative
}

#[test]
fn test_edge_cases() {
    // Test empty filter (should pass all numbers)
    let numbers = vec![100.0, 200.0, 300.0];
    let no_filter = NumberFilter::parse("").unwrap();
    let filtered = apply_number_filter(&numbers, &no_filter);
    assert_eq!(filtered, numbers);
    
    // Test filter that matches no numbers
    let gt_filter = NumberFilter::parse(">1000").unwrap();
    let filtered = apply_number_filter(&numbers, &gt_filter);
    assert!(filtered.is_empty());
    
    // Test equal filter
    let eq_filter = NumberFilter::parse("=200").unwrap();
    let filtered = apply_number_filter(&numbers, &eq_filter);
    assert_eq!(filtered, vec![200.0]);
}