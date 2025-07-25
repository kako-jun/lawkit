#[allow(unused_imports)]
use lawkit_core::laws::benford::japanese::extract_numbers;

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
        assert_eq!(numbers, Vec::<f64>::new());

        // Test string with no numbers
        let numbers = extract_numbers("No numbers here!");
        assert_eq!(numbers, Vec::<f64>::new());
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
        assert!(
            duration.as_millis() < 1000,
            "Large text extraction took too long: {:?}",
            duration
        );
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
        assert!(
            duration.as_millis() < 500,
            "Complex regex took too long: {:?}",
            duration
        );
    }
}
