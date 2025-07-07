/// Convert Japanese numerals (full-width digits and kanji) to standard numbers
pub fn convert_japanese_numerals(text: &str) -> String {
    let mut result = text.to_string();

    // Convert full-width digits to half-width
    result = convert_full_width_digits(&result);

    // Convert kanji numerals to Arabic numerals
    result = convert_kanji_numerals(&result);

    result
}

/// Convert full-width digits (０１２３４５６７８９) to half-width (0123456789)
fn convert_full_width_digits(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '０' => '0',
            '１' => '1',
            '２' => '2',
            '３' => '3',
            '４' => '4',
            '５' => '5',
            '６' => '6',
            '７' => '7',
            '８' => '8',
            '９' => '9',
            _ => c,
        })
        .collect()
}

/// Convert kanji numerals to Arabic numerals using proper positional notation parsing
fn convert_kanji_numerals(text: &str) -> String {
    use regex::Regex;

    // Extended pattern to include Chinese traditional financial numerals
    let kanji_pattern =
        Regex::new(r"[一二三四五六七八九十百千万萬〇零壹貳參肆伍陸柒捌玖拾佰仟]+").unwrap();

    kanji_pattern
        .replace_all(text, |caps: &regex::Captures| {
            let kanji_num = caps.get(0).unwrap().as_str();
            match parse_kanji_number_advanced(kanji_num) {
                Ok(arabic_num) => arabic_num.to_string(),
                Err(_) => kanji_num.to_string(), // Keep original if parsing fails
            }
        })
        .to_string()
}

/// Advanced parser for kanji numbers with proper handling of all patterns
fn parse_kanji_number_advanced(kanji: &str) -> Result<u64, String> {
    if kanji.is_empty() {
        return Err("Empty string".to_string());
    }

    // Handle simple single-character cases including Chinese financial numerals
    match kanji {
        "〇" | "零" => return Ok(0),
        "一" | "壹" => return Ok(1),
        "二" | "貳" => return Ok(2),
        "三" | "參" => return Ok(3),
        "四" | "肆" => return Ok(4),
        "五" | "伍" => return Ok(5),
        "六" | "陸" => return Ok(6),
        "七" | "柒" => return Ok(7),
        "八" | "捌" => return Ok(8),
        "九" | "玖" => return Ok(9),
        "十" | "拾" => return Ok(10),
        "百" | "佰" => return Ok(100),
        "千" | "仟" => return Ok(1000),
        "万" | "萬" => return Ok(10000),
        _ => {}
    }

    let chars: Vec<char> = kanji.chars().collect();

    // Special case: if it ends with 万, 千, 百, or 十 and all preceding chars are digits,
    // treat it as digits followed by positional marker (e.g., 四五六万 = 456*10000)
    if let Some(&last_char) = chars.last() {
        if matches!(
            last_char,
            '万' | '萬' | '千' | '仟' | '百' | '佰' | '十' | '拾'
        ) {
            let digit_chars = &chars[..chars.len() - 1];
            let all_digits = digit_chars.iter().all(|&c| {
                matches!(
                    c,
                    '一' | '二'
                        | '三'
                        | '四'
                        | '五'
                        | '六'
                        | '七'
                        | '八'
                        | '九'
                        | '〇'
                        | '零'
                        | '壹'
                        | '貳'
                        | '參'
                        | '肆'
                        | '伍'
                        | '陸'
                        | '柒'
                        | '捌'
                        | '玖'
                )
            });

            if all_digits {
                // Convert digit sequence to number
                let mut digit_value = 0u64;
                for &ch in digit_chars {
                    let digit = match ch {
                        '一' | '壹' => 1,
                        '二' | '貳' => 2,
                        '三' | '參' => 3,
                        '四' | '肆' => 4,
                        '五' | '伍' => 5,
                        '六' | '陸' => 6,
                        '七' | '柒' => 7,
                        '八' | '捌' => 8,
                        '九' | '玖' => 9,
                        '〇' | '零' => 0,
                        _ => return Err(format!("Invalid digit character: {ch}")),
                    };
                    digit_value = digit_value * 10 + digit;
                }

                let multiplier = match last_char {
                    '万' | '萬' => 10000,
                    '千' | '仟' => 1000,
                    '百' | '佰' => 100,
                    '十' | '拾' => 10,
                    _ => 1,
                };

                return Ok(digit_value * multiplier);
            }
        }
    }

    // Check if this is a pure digit sequence (no positional markers)
    let has_positional_markers = chars
        .iter()
        .any(|&c| matches!(c, '十' | '拾' | '百' | '佰' | '千' | '仟' | '万' | '萬'));

    if !has_positional_markers {
        // Pure digit sequence like 一二三 = 123
        let mut result_str = String::new();
        for ch in chars {
            match ch {
                '一' | '壹' => result_str.push('1'),
                '二' | '貳' => result_str.push('2'),
                '三' | '參' => result_str.push('3'),
                '四' | '肆' => result_str.push('4'),
                '五' | '伍' => result_str.push('5'),
                '六' | '陸' => result_str.push('6'),
                '七' | '柒' => result_str.push('7'),
                '八' | '捌' => result_str.push('8'),
                '九' | '玖' => result_str.push('9'),
                '〇' | '零' => result_str.push('0'),
                _ => return Err(format!("Invalid character in digit sequence: {ch}")),
            }
        }
        return result_str
            .parse::<u64>()
            .map_err(|e| format!("Parse error: {e}"));
    }

    // Parse positional notation (e.g., 一千二百三十四)
    let mut total = 0u64;
    let mut current = 0u64;
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        match ch {
            '一' | '壹' => current += 1,
            '二' | '貳' => current += 2,
            '三' | '參' => current += 3,
            '四' | '肆' => current += 4,
            '五' | '伍' => current += 5,
            '六' | '陸' => current += 6,
            '七' | '柒' => current += 7,
            '八' | '捌' => current += 8,
            '九' | '玖' => current += 9,
            '〇' | '零' => current += 0,
            '十' | '拾' => {
                if current == 0 {
                    current = 1; // bare 十 means 10
                }
                total += current * 10;
                current = 0;
            }
            '百' | '佰' => {
                if current == 0 {
                    current = 1; // bare 百 means 100
                }
                total += current * 100;
                current = 0;
            }
            '千' | '仟' => {
                if current == 0 {
                    current = 1; // bare 千 means 1000
                }
                total += current * 1000;
                current = 0;
            }
            '万' | '萬' => {
                if current == 0 && total == 0 {
                    return Ok(10000); // bare 万
                }
                // For 万, multiply everything so far by 10000
                total = (total + current) * 10000;
                current = 0;
            }
            _ => return Err(format!("Invalid kanji character: {ch}")),
        }
        i += 1;
    }

    total += current;
    Ok(total)
}

/// Parse a kanji number string into an Arabic number (legacy function)
#[allow(dead_code)]
fn parse_kanji_number(kanji: &str) -> Result<u64, String> {
    if kanji.is_empty() {
        return Err("Empty string".to_string());
    }

    // Handle simple single-character cases first
    match kanji {
        "〇" | "零" => return Ok(0),
        "一" => return Ok(1),
        "二" => return Ok(2),
        "三" => return Ok(3),
        "四" => return Ok(4),
        "五" => return Ok(5),
        "六" => return Ok(6),
        "七" => return Ok(7),
        "八" => return Ok(8),
        "九" => return Ok(9),
        "十" => return Ok(10),
        "百" => return Ok(100),
        "千" => return Ok(1000),
        "万" => return Ok(10000),
        _ => {}
    }

    // Check if this is a sequence of single digits + positional markers
    let chars: Vec<char> = kanji.chars().collect();

    // First check if this ends with a positional marker and has digits before it
    if let Some(&last_char) = chars.last() {
        match last_char {
            '万' | '千' | '百' | '十' => {
                // This is a positional notation, process normally
            }
            _ => {
                // Check if this is a pure sequence of single digits (like 一二三 = 123)
                let is_single_digit_sequence = chars.iter().all(|&c| {
                    matches!(
                        c,
                        '一' | '二' | '三' | '四' | '五' | '六' | '七' | '八' | '九' | '〇' | '零'
                    )
                });

                if is_single_digit_sequence {
                    let mut result_str = String::new();
                    for ch in chars {
                        match ch {
                            '一' => result_str.push('1'),
                            '二' => result_str.push('2'),
                            '三' => result_str.push('3'),
                            '四' => result_str.push('4'),
                            '五' => result_str.push('5'),
                            '六' => result_str.push('6'),
                            '七' => result_str.push('7'),
                            '八' => result_str.push('8'),
                            '九' => result_str.push('9'),
                            '〇' | '零' => result_str.push('0'),
                            _ => return Err(format!("Invalid character in digit sequence: {ch}")),
                        }
                    }
                    return result_str
                        .parse::<u64>()
                        .map_err(|e| format!("Parse error: {e}"));
                }
            }
        }
    }

    // Parse as positional notation (like 一千二百三十四 = 1234)
    let mut result = 0u64;
    let mut current = 0u64;
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        match ch {
            '一' => current += 1,
            '二' => current += 2,
            '三' => current += 3,
            '四' => current += 4,
            '五' => current += 5,
            '六' => current += 6,
            '七' => current += 7,
            '八' => current += 8,
            '九' => current += 9,
            '〇' | '零' => current += 0,
            '十' => {
                if current == 0 {
                    current = 1; // 十 means 10, not 0*10
                }
                current *= 10;
                result += current;
                current = 0;
            }
            '百' => {
                if current == 0 {
                    current = 1; // 百 means 100, not 0*100
                }
                current *= 100;
                result += current;
                current = 0;
            }
            '千' => {
                if current == 0 {
                    current = 1; // 千 means 1000, not 0*1000
                }
                current *= 1000;
                result += current;
                current = 0;
            }
            '万' => {
                if current == 0 && result == 0 {
                    return Ok(10000); // bare 万
                }
                let temp = result + current;
                result = temp * 10000;
                current = 0;
            }
            _ => return Err(format!("Invalid kanji character: {ch}")),
        }
        i += 1;
    }

    result += current;
    Ok(result)
}

/// Extract numbers from text, including Japanese numerals
pub fn extract_numbers(text: &str) -> Vec<f64> {
    use regex::Regex;

    // First convert Japanese numerals
    let converted_text = convert_japanese_numerals(text);

    // Extract numbers using regex pattern
    let number_pattern = Regex::new(r"-?\d+(?:\.\d+)?").unwrap();

    number_pattern
        .find_iter(&converted_text)
        .filter_map(|m| m.as_str().parse::<f64>().ok())
        .filter(|&n| n != 0.0) // Filter out zeros for Benford's Law
        .collect()
}

/// Extract numbers from Japanese text (alias for compatibility)
pub fn extract_numbers_from_japanese_text(text: &str) -> Vec<f64> {
    extract_numbers(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_width_conversion() {
        assert_eq!(convert_full_width_digits("１２３４５"), "12345");
        assert_eq!(convert_full_width_digits("０６７８９"), "06789");
    }

    #[test]
    fn test_kanji_conversion() {
        // Test single digits as sequences
        assert_eq!(convert_kanji_numerals("一二三"), "123");

        // Test positional notation
        let result = convert_kanji_numerals("一千二百三十四");
        println!("Testing '一千二百三十四' -> '{result}'");
        assert_eq!(result, "1234");

        assert_eq!(convert_kanji_numerals("五万六千七百八十九"), "56789");

        // Test single digits
        assert_eq!(convert_kanji_numerals("一"), "1");
        assert_eq!(convert_kanji_numerals("九"), "9");

        // Test positional values
        assert_eq!(convert_kanji_numerals("十"), "10");
        assert_eq!(convert_kanji_numerals("百"), "100");
        assert_eq!(convert_kanji_numerals("千"), "1000");
        assert_eq!(convert_kanji_numerals("万"), "10000");

        // Test Chinese financial numerals
        assert_eq!(convert_kanji_numerals("壹貳參"), "123");
        assert_eq!(convert_kanji_numerals("肆伍陸"), "456");
        assert_eq!(convert_kanji_numerals("柒捌玖"), "789");

        // Test traditional Chinese wan
        assert_eq!(convert_kanji_numerals("萬"), "10000");
        assert_eq!(convert_kanji_numerals("拾萬"), "100000");
    }

    #[test]
    fn test_mixed_conversion() {
        let result = convert_japanese_numerals("売上１２３万円 経費四五六万円");
        println!("Input: '売上１２３万円 経費四五六万円'");
        println!("Result: '{result}'");
        assert!(result.contains("123"));
        assert!(result.contains("456"));
    }
}
