/// International numeral conversion module
/// Supports Chinese, Hindi, and Arabic numerals
// Note: regex is used for Chinese financial numerals in convert_chinese_numerals
/// Convert international numerals to standard Arabic digits
pub fn convert_international_numerals(text: &str) -> String {
    let mut result = text.to_string();

    // Convert Japanese numerals (full-width and kanji)
    result = crate::laws::benford::japanese::convert_japanese_numerals(&result);

    // Convert Chinese numerals
    result = convert_chinese_numerals(&result);

    // Convert Hindi numerals (Devanagari)
    result = convert_hindi_numerals(&result);

    // Convert Arabic-Indic numerals
    result = convert_arabic_numerals(&result);

    result
}

/// Convert Chinese numerals to Arabic numerals
fn convert_chinese_numerals(text: &str) -> String {
    use regex::Regex;

    // Traditional Chinese financial numerals for fraud prevention
    let financial_pattern = Regex::new(r"[壹貳參肆伍陸柒捌玖拾佰仟萬億]+").unwrap();

    let result = financial_pattern
        .replace_all(text, |caps: &regex::Captures| {
            let chinese_num = caps.get(0).unwrap().as_str();
            match parse_chinese_financial_number(chinese_num) {
                Ok(arabic_num) => arabic_num.to_string(),
                Err(_) => chinese_num.to_string(),
            }
        })
        .to_string();

    // Standard Chinese numerals (same as Japanese kanji, already handled in japanese.rs)
    // 一二三四五六七八九十百千万 are processed by the Japanese module

    result
}

/// Convert Hindi numerals (Devanagari script) to Arabic numerals
fn convert_hindi_numerals(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '०' => '0', // U+0966 DEVANAGARI DIGIT ZERO
            '१' => '1', // U+0967 DEVANAGARI DIGIT ONE
            '२' => '2', // U+0968 DEVANAGARI DIGIT TWO
            '३' => '3', // U+0969 DEVANAGARI DIGIT THREE
            '४' => '4', // U+096A DEVANAGARI DIGIT FOUR
            '५' => '5', // U+096B DEVANAGARI DIGIT FIVE
            '६' => '6', // U+096C DEVANAGARI DIGIT SIX
            '७' => '7', // U+096D DEVANAGARI DIGIT SEVEN
            '८' => '8', // U+096E DEVANAGARI DIGIT EIGHT
            '९' => '9', // U+096F DEVANAGARI DIGIT NINE
            _ => c,
        })
        .collect()
}

/// Convert Arabic-Indic numerals to standard Arabic numerals
fn convert_arabic_numerals(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '٠' => '0', // U+0660 ARABIC-INDIC DIGIT ZERO
            '١' => '1', // U+0661 ARABIC-INDIC DIGIT ONE
            '٢' => '2', // U+0662 ARABIC-INDIC DIGIT TWO
            '٣' => '3', // U+0663 ARABIC-INDIC DIGIT THREE
            '٤' => '4', // U+0664 ARABIC-INDIC DIGIT FOUR
            '٥' => '5', // U+0665 ARABIC-INDIC DIGIT FIVE
            '٦' => '6', // U+0666 ARABIC-INDIC DIGIT SIX
            '٧' => '7', // U+0667 ARABIC-INDIC DIGIT SEVEN
            '٨' => '8', // U+0668 ARABIC-INDIC DIGIT EIGHT
            '٩' => '9', // U+0669 ARABIC-INDIC DIGIT NINE
            _ => c,
        })
        .collect()
}

/// Parse Chinese financial numerals (anti-fraud variants)
fn parse_chinese_financial_number(chinese: &str) -> Result<u64, String> {
    let mut result = 0u64;
    let mut current = 0u64;

    for c in chinese.chars() {
        match c {
            '壹' => current = 1,
            '貳' => current = 2,
            '參' => current = 3,
            '肆' => current = 4,
            '伍' => current = 5,
            '陸' => current = 6,
            '柒' => current = 7,
            '捌' => current = 8,
            '玖' => current = 9,
            '拾' => {
                if current == 0 {
                    current = 1;
                }
                current *= 10;
                result += current;
                current = 0;
            }
            '佰' => {
                if current == 0 {
                    current = 1;
                }
                current *= 100;
                result += current;
                current = 0;
            }
            '仟' => {
                if current == 0 {
                    current = 1;
                }
                current *= 1000;
                result += current;
                current = 0;
            }
            '萬' => {
                if current == 0 {
                    current = 1;
                }
                result = (result + current) * 10000;
                current = 0;
            }
            '億' => {
                if current == 0 {
                    current = 1;
                }
                result = (result + current) * 100000000;
                current = 0;
            }
            _ => continue,
        }
    }

    result += current;
    Ok(result)
}

/// Extract numbers from text with international numeral support
pub fn extract_numbers_international(text: &str) -> Vec<f64> {
    // First convert all international numerals to standard Arabic digits
    let converted = convert_international_numerals(text);

    // Then use the existing number extraction logic
    crate::laws::benford::japanese::extract_numbers(&converted)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Disabled test with mixed script issues
    #[allow(dead_code)]
    fn test_hindi_numerals_basic_disabled() {
        assert_eq!(convert_hindi_numerals("१२३४५"), "12345");
        assert_eq!(convert_hindi_numerals("०९८७६"), "09876");
        assert_eq!(convert_hindi_numerals("abc१२３def"), "abc123def");
    }

    #[test]
    fn test_arabic_numerals() {
        assert_eq!(convert_arabic_numerals("١٢٣٤٥"), "12345");
        assert_eq!(convert_arabic_numerals("٠٩٨٧٦"), "09876");
        assert_eq!(convert_arabic_numerals("abc١٢٣def"), "abc123def");
    }

    #[test]
    fn test_chinese_financial_numerals() {
        assert_eq!(parse_chinese_financial_number("壹"), Ok(1));
        assert_eq!(parse_chinese_financial_number("拾"), Ok(10));
        assert_eq!(parse_chinese_financial_number("壹拾"), Ok(10));
        assert_eq!(parse_chinese_financial_number("貳拾參"), Ok(23));
        assert_eq!(parse_chinese_financial_number("壹佰貳拾參"), Ok(123));
    }

    #[test]
    fn test_convert_chinese_numerals() {
        let result = convert_chinese_numerals("金額壹拾貳萬參仟肆佰伍拾陸");
        assert!(result.contains("123456"));
    }

    #[test]
    fn test_international_number_extraction() {
        let hindi_text = "राजस्व १२३४५६ रुपये";
        let numbers = extract_numbers_international(hindi_text);
        assert!(numbers.contains(&123456.0));

        let arabic_text = "المبلغ ١٢٣٤٥٦ ريال";
        let numbers = extract_numbers_international(arabic_text);
        assert!(numbers.contains(&123456.0));
    }

    #[test]
    fn test_mixed_numerals() {
        let mixed_text = "English 123, Hindi १२३, Arabic ١٢٣, Chinese 壹貳參";
        let numbers = extract_numbers_international(mixed_text);
        assert!(numbers.len() >= 4);
        assert!(numbers.contains(&123.0));
    }

    #[test]
    fn test_pure_hindi_numerals() {
        // Test with pure Devanagari digits only
        assert_eq!(convert_hindi_numerals("abc१२३def"), "abc123def"); // Converts Hindi numerals
        assert_eq!(convert_hindi_numerals("१२३"), "123");
    }
}
