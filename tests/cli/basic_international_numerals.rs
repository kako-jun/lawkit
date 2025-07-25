/// Comprehensive tests for international numeral conversion
/// Tests Chinese, Hindi, and Arabic numeral support from international.rs
use lawkit_core::common::international::{
    convert_international_numerals, extract_numbers_international,
};

#[cfg(test)]
mod chinese_numerals_tests {
    use super::*;

    #[test]
    fn test_chinese_financial_numerals_basic() {
        // Test basic Chinese financial numerals
        let result = convert_international_numerals("金額壹拾貳萬參仟肆佰伍拾陸");
        assert!(result.contains("123456"));
    }

    #[test]
    fn test_chinese_financial_complex() {
        // Test complex Chinese financial numerals
        let test_cases = vec![
            ("壹", "1"),
            ("拾", "10"),
            ("壹拾", "10"),
            ("貳拾參", "23"),
            ("壹佰貳拾參", "123"),
            ("壹仟貳佰參拾肆", "1234"),
        ];

        for (chinese, expected) in test_cases {
            let result = convert_international_numerals(chinese);
            assert!(
                result.contains(expected),
                "Failed for {}: expected {}, got {}",
                chinese,
                expected,
                result
            );
        }
    }

    #[test]
    fn test_chinese_financial_large_numbers() {
        // Test large Chinese financial numerals
        let result = convert_international_numerals("壹萬貳仟參佰肆拾伍");
        assert!(result.contains("12345"));

        let result = convert_international_numerals("拾萬");
        assert!(result.contains("100000"));
    }
}

#[cfg(test)]
mod hindi_numerals_tests {
    use super::*;

    #[test]
    fn test_hindi_devanagari_basic() {
        // Test basic Devanagari numerals (0-9)
        let result = convert_international_numerals("१२३४५");
        assert_eq!(result, "12345");

        let result = convert_international_numerals("०९८७६");
        assert_eq!(result, "09876");
    }

    #[test]
    fn test_hindi_numerals_in_context() {
        // Test Hindi numerals in context
        let result = convert_international_numerals("राजस्व १२३४५६ रुपये");
        assert!(result.contains("123456"));

        let result = convert_international_numerals("abc१२३def");
        assert_eq!(result, "abc123def");
    }

    #[test]
    fn test_all_hindi_digits() {
        // Test all Devanagari digits
        let test_cases = vec![
            ("०", "0"),
            ("१", "1"),
            ("२", "2"),
            ("३", "3"),
            ("४", "4"),
            ("५", "5"),
            ("६", "6"),
            ("७", "7"),
            ("८", "8"),
            ("९", "9"),
        ];

        for (hindi, expected) in test_cases {
            let result = convert_international_numerals(hindi);
            assert_eq!(result, expected, "Failed for Hindi digit {}", hindi);
        }
    }
}

#[cfg(test)]
mod arabic_numerals_tests {
    use super::*;

    #[test]
    fn test_arabic_indic_basic() {
        // Test basic Arabic-Indic numerals (0-9)
        let result = convert_international_numerals("١٢٣٤٥");
        assert_eq!(result, "12345");

        let result = convert_international_numerals("٠٩٨٧٦");
        assert_eq!(result, "09876");
    }

    #[test]
    fn test_arabic_numerals_in_context() {
        // Test Arabic numerals in context
        let result = convert_international_numerals("المبلغ ١٢٣٤٥٦ ريال");
        assert!(result.contains("123456"));

        let result = convert_international_numerals("abc١٢٣def");
        assert_eq!(result, "abc123def");
    }

    #[test]
    fn test_all_arabic_digits() {
        // Test all Arabic-Indic digits
        let test_cases = vec![
            ("٠", "0"),
            ("١", "1"),
            ("٢", "2"),
            ("٣", "3"),
            ("٤", "4"),
            ("٥", "5"),
            ("٦", "6"),
            ("٧", "7"),
            ("٨", "8"),
            ("٩", "9"),
        ];

        for (arabic, expected) in test_cases {
            let result = convert_international_numerals(arabic);
            assert_eq!(result, expected, "Failed for Arabic digit {}", arabic);
        }
    }
}

#[cfg(test)]
mod mixed_numerals_tests {
    use super::*;

    #[test]
    fn test_mixed_script_conversion() {
        // Test mixed scripts in one text
        let mixed_text = "English 123, Hindi १२३, Arabic ١٢٣, Chinese 壹貳參";
        let result = convert_international_numerals(mixed_text);

        // Should convert all to standard digits
        assert!(result.contains("123"));
        // Check that it contains multiple instances of 123
        let count = result.matches("123").count();
        assert!(
            count >= 3,
            "Should find at least 3 instances of '123', found {}",
            count
        );
    }

    #[test]
    fn test_extract_numbers_international() {
        // Test comprehensive number extraction from mixed international text
        let hindi_text = "राजस्व १२३४५६ रुपये";
        let numbers = extract_numbers_international(hindi_text);
        assert!(numbers.contains(&123456.0));

        let arabic_text = "المبلغ ١٢٣٤٥٦ ريال";
        let numbers = extract_numbers_international(arabic_text);
        assert!(numbers.contains(&123456.0));

        let chinese_text = "金額壹拾貳萬參仟肆佰伍拾陸";
        let numbers = extract_numbers_international(chinese_text);
        assert!(numbers.len() > 0);
    }

    #[test]
    fn test_mixed_numerals_comprehensive() {
        // Test comprehensive mixed numeral extraction
        let mixed_text = "Sales: 100, हिंदी १०० units, العربية ١٠٠ items, 中文 壹佰 pieces";
        let numbers = extract_numbers_international(mixed_text);

        // Should extract multiple instances of 100
        assert!(numbers.contains(&100.0));
        let count_100 = numbers.iter().filter(|&&x| x == 100.0).count();
        assert!(
            count_100 >= 2,
            "Should find at least 2 instances of 100, found {}",
            count_100
        );
    }
}

#[cfg(test)]
mod japanese_integration_tests {
    use super::*;

    #[test]
    fn test_japanese_integration() {
        // Test that Japanese numerals work through international conversion
        let japanese_text = "売上 １２３４５ 円と 三万五千 円";
        let result = convert_international_numerals(japanese_text);

        // Should convert both full-width and kanji numerals
        assert!(result.contains("12345"));
        assert!(result.contains("35000"));
    }

    #[test]
    fn test_all_asian_numerals() {
        // Test integration of Japanese, Chinese, and Hindi
        let text = "日本 １２３ / 中国 壹貳參 / हिंदी १२३";
        let result = convert_international_numerals(text);

        // All should convert to 123
        let count = result.matches("123").count();
        assert!(
            count >= 3,
            "Should find at least 3 instances of '123', found {}",
            count
        );
    }
}
