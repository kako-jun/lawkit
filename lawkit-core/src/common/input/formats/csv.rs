use crate::common::international::extract_numbers_international;
use std::path::Path;

/// Parse CSV/TSV files and extract numbers
pub fn parse_csv_file(file_path: &Path) -> crate::error::Result<Vec<f64>> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| crate::error::BenfError::FileError(format!("Failed to read CSV file: {e}")))?;

    parse_csv_content(&content)
}

/// Parse CSV content from string
pub fn parse_csv_content(content: &str) -> crate::error::Result<Vec<f64>> {
    let mut all_numbers = Vec::new();

    // Simple CSV parsing - split by lines and then by commas/tabs
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue; // Skip empty lines and comments
        }

        // Try comma-separated first
        let fields: Vec<&str> = if line.contains(',') {
            line.split(',').collect()
        } else {
            // Fall back to tab-separated
            line.split('\t').collect()
        };

        for field in fields {
            let field = field.trim().trim_matches('"'); // Remove quotes and whitespace

            // Extract numbers from each field (including international numerals)
            let numbers = extract_numbers_international(field);
            all_numbers.extend(numbers);
        }
    }

    if all_numbers.is_empty() {
        return Err(crate::error::BenfError::NoNumbersFound);
    }

    Ok(all_numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_content_parsing() {
        let csv_content = r#"Name,Amount,Date
Sales,1234.56,2023-01-01
Expenses,567.89,2023-01-02
Revenue,9876.54,2023-01-03"#;

        let numbers = parse_csv_content(csv_content).unwrap();
        // Should extract amounts and date components (year, month, day)
        assert!(numbers.len() >= 6); // At least 3 amounts + 3 years
        assert!(numbers.contains(&1234.56));
        assert!(numbers.contains(&567.89));
        assert!(numbers.contains(&9876.54));
        assert!(numbers.contains(&2023.0)); // Years from dates
    }

    #[test]
    fn test_tsv_content_parsing() {
        let tsv_content =
            "Name\tAmount\tDate\nSales\t1234.56\t2023-01-01\nExpenses\t567.89\t2023-01-02";

        let numbers = parse_csv_content(tsv_content).unwrap();
        assert!(!numbers.is_empty());
        assert!(numbers.contains(&1234.56));
        assert!(numbers.contains(&567.89));
    }

    #[test]
    fn test_csv_with_japanese_numerals() {
        let csv_content = "商品,金額\n商品A,一千二百三十四\n商品B,五六七八";

        let numbers = parse_csv_content(csv_content).unwrap();
        assert!(!numbers.is_empty());
        // Numbers from Japanese numerals should be extracted
    }
}
