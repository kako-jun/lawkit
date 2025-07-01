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

/// Convert kanji numerals to Arabic numerals
fn convert_kanji_numerals(text: &str) -> String {
    let mut result = text.to_string();
    
    // Define conversion patterns from most complex to simplest
    let patterns = [
        // Large numbers with positional notation
        ("九万", "90000"),
        ("八万", "80000"),
        ("七万", "70000"),
        ("六万", "60000"),
        ("五万", "50000"),
        ("四万", "40000"),
        ("三万", "30000"),
        ("二万", "20000"),
        ("一万", "10000"),
        ("万", "0000"), // Handle bare 万 as placeholder
        
        // Thousands
        ("九千", "9000"),
        ("八千", "8000"),
        ("七千", "7000"),
        ("六千", "6000"),
        ("五千", "5000"),
        ("四千", "4000"),
        ("三千", "3000"),
        ("二千", "2000"),
        ("一千", "1000"),
        ("千", "000"), // Handle bare 千 as placeholder
        
        // Hundreds
        ("九百", "900"),
        ("八百", "800"),
        ("七百", "700"),
        ("六百", "600"),
        ("五百", "500"),
        ("四百", "400"),
        ("三百", "300"),
        ("二百", "200"),
        ("一百", "100"),
        ("百", "00"), // Handle bare 百 as placeholder
        
        // Tens
        ("九十", "90"),
        ("八十", "80"),
        ("七十", "70"),
        ("六十", "60"),
        ("五十", "50"),
        ("四十", "40"),
        ("三十", "30"),
        ("二十", "20"),
        ("一十", "10"),
        ("十", "0"), // Handle bare 十 as placeholder
        
        // Single digits
        ("九", "9"),
        ("八", "8"),
        ("七", "7"),
        ("六", "6"),
        ("五", "5"),
        ("四", "4"),
        ("三", "3"),
        ("二", "2"),
        ("一", "1"),
        ("〇", "0"),
        ("零", "0"),
    ];
    
    // Apply conversions
    for (kanji, arabic) in patterns {
        result = result.replace(kanji, arabic);
    }
    
    // Clean up placeholder zeros and merge adjacent numbers
    result = clean_converted_numbers(&result);
    
    result
}

/// Clean up converted numbers by removing placeholder zeros and merging adjacent digits
fn clean_converted_numbers(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch.is_ascii_digit() {
            let mut number = String::new();
            number.push(ch);
            
            // Collect consecutive digits
            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_ascii_digit() {
                    number.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            
            // Convert to integer to remove leading zeros, then back to string
            if let Ok(num) = number.parse::<u64>() {
                result.push_str(&num.to_string());
            } else {
                result.push_str(&number);
            }
        } else {
            result.push(ch);
        }
    }
    
    result
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
        assert_eq!(convert_kanji_numerals("一二三"), "123");
        assert_eq!(convert_kanji_numerals("一千二百三十四"), "1234");
        assert_eq!(convert_kanji_numerals("五万六千七百八十九"), "56789");
    }

    #[test]
    fn test_mixed_conversion() {
        let result = convert_japanese_numerals("売上１２３万円 経費四五六万円");
        assert!(result.contains("123"));
        assert!(result.contains("456"));
    }
}