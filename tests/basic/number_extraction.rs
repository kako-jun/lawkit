use lawkit_core::common::input::parser::{extract_numbers, filter_benford_valid_numbers};

#[cfg(test)]
mod number_extraction_tests {
    use super::*;

    #[test]
    fn test_basic_number_extraction() {
        // Test simple space-separated numbers
        let numbers = extract_numbers("123 456 789");
        assert_eq!(numbers, vec![123.0, 456.0, 789.0]);
        
        // Test comma-separated numbers
        let numbers = extract_numbers("123, 456, 789");
        assert_eq!(numbers, vec![123.0, 456.0, 789.0]);
        
        // Test numbers with decimal points
        let numbers = extract_numbers("12.34 56.78 90.12");
        assert_eq!(numbers, vec![12.34, 56.78, 90.12]);
    }

    #[test]
    fn test_number_extraction_with_text() {
        // Test numbers embedded in text
        let numbers = extract_numbers("Price: $123.45, Quantity: 67, Total: $890.12");
        assert_eq!(numbers, vec![123.45, 67.0, 890.12]);
        
        // Test numbers with units
        let numbers = extract_numbers("Distance: 123km, Time: 45min, Speed: 67.8km/h");
        assert_eq!(numbers, vec![123.0, 45.0, 67.8]);
    }

    #[test]
    fn test_number_extraction_edge_cases() {
        // Test empty string
        let numbers = extract_numbers("");
        assert_eq!(numbers, vec![]);
        
        // Test string with no numbers
        let numbers = extract_numbers("No numbers here!");
        assert_eq!(numbers, vec![]);
        
        // Test string with only text
        let numbers = extract_numbers("Hello World");
        assert_eq!(numbers, vec![]);
    }

    #[test]
    fn test_negative_numbers() {
        let numbers = extract_numbers("-123 -456.78 -0.9");
        assert_eq!(numbers, vec![-123.0, -456.78, -0.9]);
        
        // Test mixed positive and negative
        let numbers = extract_numbers("123 -456 789.0 -12.34");
        assert_eq!(numbers, vec![123.0, -456.0, 789.0, -12.34]);
    }

    #[test]
    fn test_scientific_notation() {
        let numbers = extract_numbers("1.23e10 4.56E-5 7.89e+3");
        assert_eq!(numbers, vec![1.23e10, 4.56e-5, 7.89e3]);
    }

    #[test]
    fn test_numbers_with_commas() {
        // Test thousands separators
        let numbers = extract_numbers("1,234 5,678.90 1,000,000");
        assert_eq!(numbers, vec![1234.0, 5678.90, 1000000.0]);
    }

    #[test]
    fn test_leading_zeros() {
        let numbers = extract_numbers("0123 0456.789 00890");
        assert_eq!(numbers, vec![123.0, 456.789, 890.0]);
    }

    #[test]
    fn test_percentage_numbers() {
        // Test numbers that might have % signs
        let numbers = extract_numbers("Success rate: 95.5%, Error rate: 4.5%");
        assert_eq!(numbers, vec![95.5, 4.5]);
    }

    #[test]
    fn test_currency_numbers() {
        // Test various currency formats
        let numbers = extract_numbers("$123.45 €67.89 ¥1000 £234.56");
        assert_eq!(numbers, vec![123.45, 67.89, 1000.0, 234.56]);
    }

    #[test]
    fn test_all_fixture_cases() {
        for test_case in NUMBER_EXTRACTION_TESTS {
            let numbers = extract_numbers(test_case.input);
            assert_eq!(
                numbers, test_case.expected,
                "Failed test case: {} - Input: '{}', Expected: {:?}, Got: {:?}",
                test_case.description, test_case.input, test_case.expected, numbers
            );
        }
    }
}

#[cfg(test)]
mod benford_filtering_tests {
    use super::*;

    #[test]
    fn test_filter_benford_valid_numbers() {
        // Test filtering numbers for Benford's Law analysis
        let numbers = vec![0.0, 123.0, 0.001, 456.0, -789.0, 0.0123];
        let filtered = filter_benford_valid_numbers(&numbers);
        
        // Should include: 123.0, 456.0, 789.0 (absolute value), 0.0123 (first digit is 1)
        // Should exclude: 0.0 (zero), 0.001 (leading zero after decimal)
        let expected = vec![123.0, 456.0, 789.0, 0.0123];
        assert_eq!(filtered, expected);
    }

    #[test]
    fn test_filter_zero_values() {
        let numbers = vec![0.0, 0.00, 100.0, 200.0];
        let filtered = filter_benford_valid_numbers(&numbers);
        
        // Should exclude zeros
        assert_eq!(filtered, vec![100.0, 200.0]);
    }

    #[test]
    fn test_filter_very_small_numbers() {
        // Test handling of very small numbers
        let numbers = vec![0.0001, 0.001, 0.01, 0.1, 1.0];
        let filtered = filter_benford_valid_numbers(&numbers);
        
        // All should be valid as they have non-zero first digits
        assert_eq!(filtered, vec![0.0001, 0.001, 0.01, 0.1, 1.0]);
    }

    #[test]
    fn test_filter_negative_numbers() {
        // Test that negative numbers are converted to absolute values
        let numbers = vec![-123.0, -456.0, 789.0];
        let filtered = filter_benford_valid_numbers(&numbers);
        
        // Should use absolute values
        assert_eq!(filtered, vec![123.0, 456.0, 789.0]);
    }

    #[test]
    fn test_filter_empty_input() {
        let numbers = vec![];
        let filtered = filter_benford_valid_numbers(&numbers);
        assert_eq!(filtered, vec![]);
    }

    #[test]
    fn test_filter_all_invalid() {
        // Test input with only invalid numbers (zeros)
        let numbers = vec![0.0, 0.00, 0.000];
        let filtered = filter_benford_valid_numbers(&numbers);
        assert_eq!(filtered, vec![]);
    }
}

#[cfg(test)]
mod regex_pattern_tests {
    use super::*;
    use benf::core::extraction::NUMBER_PATTERN;
    use regex::Regex;

    #[test]
    fn test_number_regex_pattern() {
        let re = Regex::new(NUMBER_PATTERN).expect("Invalid regex pattern");
        
        // Test basic integers
        assert!(re.is_match("123"));
        assert!(re.is_match("0"));
        assert!(re.is_match("9876543210"));
        
        // Test decimals
        assert!(re.is_match("123.456"));
        assert!(re.is_match("0.123"));
        assert!(re.is_match(".123"));
        
        // Test negative numbers
        assert!(re.is_match("-123"));
        assert!(re.is_match("-123.456"));
        
        // Test scientific notation
        assert!(re.is_match("1.23e10"));
        assert!(re.is_match("1.23E-5"));
        assert!(re.is_match("1e10"));
        
        // Test numbers with commas
        assert!(re.is_match("1,234"));
        assert!(re.is_match("1,234,567.89"));
    }

    #[test]
    fn test_number_regex_no_match() {
        let re = Regex::new(NUMBER_PATTERN).expect("Invalid regex pattern");
        
        // Test non-numeric strings
        assert!(!re.is_match("abc"));
        assert!(!re.is_match(""));
        assert!(!re.is_match("no numbers"));
        
        // Test invalid number formats
        assert!(!re.is_match("123.456.789")); // Multiple decimal points
        assert!(!re.is_match("--123")); // Multiple negative signs
    }
}

#[cfg(test)]
mod japanese_number_extraction_tests {
    use super::*;
    use benf::core::extraction::extract_numbers_after_japanese_conversion;

    #[test]
    fn test_extract_after_japanese_conversion() {
        // Test full-width digit extraction
        let numbers = extract_numbers_after_japanese_conversion("１２３ ４５６ ７８９");
        assert_eq!(numbers, vec![123.0, 456.0, 789.0]);
        
        // Test kanji numeral extraction
        let numbers = extract_numbers_after_japanese_conversion("一二三 四五六 七八九");
        assert_eq!(numbers, vec![123.0, 456.0, 789.0]);
        
        // Test mixed context
        let numbers = extract_numbers_after_japanese_conversion("売上１２３万円 経費四十五万円");
        assert_eq!(numbers, vec![123.0, 45.0]);
    }

    #[test]
    fn test_complex_japanese_numbers() {
        // Test complex kanji numbers
        let numbers = extract_numbers_after_japanese_conversion("一千二百三十四");
        assert_eq!(numbers, vec![1234.0]);
        
        let numbers = extract_numbers_after_japanese_conversion("九万八千七百六十五");
        assert_eq!(numbers, vec![98765.0]);
    }

    #[test]
    fn test_mixed_japanese_and_regular_numbers() {
        // Test text with both Japanese and regular numbers
        let numbers = extract_numbers_after_japanese_conversion("123 一二三 456 七八九");
        assert_eq!(numbers, vec![123.0, 123.0, 456.0, 789.0]);
    }
}

#[cfg(test)]
mod performance_extraction_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_large_text_extraction_performance() {
        // Test extraction performance with large text
        let large_text = "123.45 ".repeat(10000);
        
        let start = Instant::now();
        let numbers = extract_numbers(&large_text);
        let duration = start.elapsed();
        
        assert_eq!(numbers.len(), 10000);
        assert!(numbers.iter().all(|&n| n == 123.45));
        
        // Should complete within reasonable time
        assert!(duration.as_millis() < 1000, "Large text extraction took too long: {:?}", duration);
    }

    #[test]
    fn test_complex_regex_performance() {
        // Test with text containing many potential number matches
        let complex_text = "abc123def456ghi789jkl012mno345pqr678stu901vwx234yz".repeat(1000);
        
        let start = Instant::now();
        let numbers = extract_numbers(&complex_text);
        let duration = start.elapsed();
        
        // Should find numbers efficiently
        assert!(!numbers.is_empty());
        assert!(duration.as_millis() < 500, "Complex regex took too long: {:?}", duration);
    }
}