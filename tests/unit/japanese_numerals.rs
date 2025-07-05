use lawkit::laws::benford::japanese::{convert_japanese_numerals, extract_numbers_from_japanese_text};
use crate::test_common::{FULLWIDTH_DIGITS, KANJI_NUMERALS, KANJI_WITH_POSITIONS, MIXED_JAPANESE};

#[cfg(test)]
mod japanese_numeral_conversion_tests {
    use super::*;

    #[test]
    fn test_fullwidth_digit_conversion() {
        // Test individual full-width digits
        assert_eq!(convert_japanese_numerals("０"), "0");
        assert_eq!(convert_japanese_numerals("１"), "1");
        assert_eq!(convert_japanese_numerals("２"), "2");
        assert_eq!(convert_japanese_numerals("３"), "3");
        assert_eq!(convert_japanese_numerals("４"), "4");
        assert_eq!(convert_japanese_numerals("５"), "5");
        assert_eq!(convert_japanese_numerals("６"), "6");
        assert_eq!(convert_japanese_numerals("７"), "7");
        assert_eq!(convert_japanese_numerals("８"), "8");
        assert_eq!(convert_japanese_numerals("９"), "9");
    }

    #[test]
    fn test_fullwidth_number_sequences() {
        assert_eq!(convert_japanese_numerals("１２３"), "123");
        assert_eq!(convert_japanese_numerals("０１２３４５６７８９"), "0123456789");
        assert_eq!(convert_japanese_numerals("２０２３"), "2023");
    }

    #[test]
    fn test_basic_kanji_numerals() {
        assert_eq!(convert_japanese_numerals("一"), "1");
        assert_eq!(convert_japanese_numerals("二"), "2");
        assert_eq!(convert_japanese_numerals("三"), "3");
        assert_eq!(convert_japanese_numerals("四"), "4");
        assert_eq!(convert_japanese_numerals("五"), "5");
        assert_eq!(convert_japanese_numerals("六"), "6");
        assert_eq!(convert_japanese_numerals("七"), "7");
        assert_eq!(convert_japanese_numerals("八"), "8");
        assert_eq!(convert_japanese_numerals("九"), "9");
        assert_eq!(convert_japanese_numerals("十"), "10");
        assert_eq!(convert_japanese_numerals("百"), "100");
        assert_eq!(convert_japanese_numerals("千"), "1000");
        assert_eq!(convert_japanese_numerals("万"), "10000");
    }

    #[test]
    fn test_kanji_position_values() {
        // Tens
        assert_eq!(convert_japanese_numerals("一十"), "10");
        assert_eq!(convert_japanese_numerals("二十"), "20");
        assert_eq!(convert_japanese_numerals("三十"), "30");
        assert_eq!(convert_japanese_numerals("九十"), "90");
        
        // Tens with units
        assert_eq!(convert_japanese_numerals("二十一"), "21");
        assert_eq!(convert_japanese_numerals("三十五"), "35");
        assert_eq!(convert_japanese_numerals("九十九"), "99");
    }

    #[test]
    fn test_kanji_hundreds() {
        // Hundreds
        assert_eq!(convert_japanese_numerals("一百"), "100");
        assert_eq!(convert_japanese_numerals("二百"), "200");
        assert_eq!(convert_japanese_numerals("九百"), "900");
        
        // Hundreds with tens and units
        assert_eq!(convert_japanese_numerals("一百二十三"), "123");
        assert_eq!(convert_japanese_numerals("二百五十"), "250");
        assert_eq!(convert_japanese_numerals("九百九十九"), "999");
    }

    #[test]
    fn test_kanji_thousands() {
        // Thousands
        assert_eq!(convert_japanese_numerals("一千"), "1000");
        assert_eq!(convert_japanese_numerals("二千"), "2000");
        assert_eq!(convert_japanese_numerals("九千"), "9000");
        
        // Complex thousands
        assert_eq!(convert_japanese_numerals("一千二百三十四"), "1234");
        assert_eq!(convert_japanese_numerals("五千六百七十八"), "5678");
        assert_eq!(convert_japanese_numerals("九千九百九十九"), "9999");
    }

    #[test]
    fn test_kanji_ten_thousands() {
        // Ten thousands (万)
        assert_eq!(convert_japanese_numerals("一万"), "10000");
        assert_eq!(convert_japanese_numerals("五万"), "50000");
        assert_eq!(convert_japanese_numerals("九万"), "90000");
        
        // Complex ten thousands
        assert_eq!(convert_japanese_numerals("一万二千三百四十五"), "12345");
        assert_eq!(convert_japanese_numerals("九万八千七百六十五"), "98765");
    }

    #[test]
    fn test_mixed_japanese_and_context() {
        // Full-width in context
        assert_eq!(
            convert_japanese_numerals("売上１２３万円"),
            "売上123万円"
        );
        
        // Kanji in context
        assert_eq!(
            convert_japanese_numerals("利益一千二百万円"),
            "利益1200万円"
        );
        
        // Mixed context
        assert_eq!(
            convert_japanese_numerals("売上１２３万円 経費四十五万円"),
            "売上123万円 経費45万円"
        );
    }

    #[test]
    fn test_edge_cases() {
        // Empty string
        assert_eq!(convert_japanese_numerals(""), "");
        
        // No Japanese numerals
        assert_eq!(convert_japanese_numerals("Hello World 123"), "Hello World 123");
        
        // Only non-numeral Japanese text
        assert_eq!(convert_japanese_numerals("こんにちは"), "こんにちは");
        
        // Mixed with regular numbers
        assert_eq!(
            convert_japanese_numerals("123 一二三 456"),
            "123 123 456"
        );
    }

    #[test]
    fn test_all_fixture_cases() {
        for test_case in JAPANESE_NUMERAL_TESTS {
            let result = convert_japanese_numerals(test_case.input);
            assert_eq!(
                result, test_case.expected,
                "Failed test case: {} - Input: '{}', Expected: '{}', Got: '{}'",
                test_case.description, test_case.input, test_case.expected, result
            );
        }
    }

    #[test]
    fn test_extract_numbers_from_japanese_text() {
        // Test extracting numbers after Japanese conversion
        let numbers = extract_numbers_from_japanese_text("一二三 四五六 七八九");
        assert_eq!(numbers, vec![123.0, 456.0, 789.0]);
        
        let numbers = extract_numbers_from_japanese_text("売上１２３万円 経費４５万円");
        // Note: This should extract 123 and 45, ignoring the 万 (ten thousand) marker for now
        assert_eq!(numbers, vec![123.0, 45.0]);
        
        let numbers = extract_numbers_from_japanese_text("一千二百三十四");
        assert_eq!(numbers, vec![1234.0]);
    }

    #[test]
    fn test_preserve_non_numeral_text() {
        // Test that non-numeral text is preserved
        let result = convert_japanese_numerals("価格は一千円です");
        assert_eq!(result, "価格は1000円です");
        
        let result = convert_japanese_numerals("数量：１２個、価格：三千円");
        assert_eq!(result, "数量：12個、価格：3000円");
    }

    #[test]
    #[should_panic(expected = "Invalid kanji numeral")]
    fn test_invalid_kanji_sequence() {
        // This should panic or return an error for invalid sequences
        // For example: 万千 (ten-thousand thousand) doesn't make sense
        convert_japanese_numerals("万千");
    }

    #[test]
    fn test_zero_handling() {
        // Test how zero is handled in different contexts
        assert_eq!(convert_japanese_numerals("０"), "0");
        
        // Zero in kanji (rarely used, but should be handled)
        // Note: 零 is sometimes used for zero in formal contexts
        // For now, we'll focus on the more common cases
    }

    #[test]
    fn test_performance_with_large_text() {
        // Test with a large string containing many Japanese numerals
        let large_text = "一千二百三十四 ".repeat(1000);
        let result = convert_japanese_numerals(&large_text);
        
        // Should convert all instances efficiently
        assert!(result.contains("1234"));
        assert!(!result.contains("一千"));
    }
}

#[cfg(test)]
mod japanese_numeral_helper_tests {
    use super::*;

    #[test]
    fn test_is_fullwidth_digit() {
        // This would test a helper function to identify full-width digits
        // Implementation will be added when we create the helper functions
    }

    #[test]
    fn test_is_kanji_numeral() {
        // This would test a helper function to identify kanji numerals
        // Implementation will be added when we create the helper functions
    }

    #[test]
    fn test_kanji_to_arabic_conversion() {
        // This would test the core conversion logic
        // Implementation will be added when we create the conversion functions
    }
}