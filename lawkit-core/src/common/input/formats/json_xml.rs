use crate::common::international::extract_numbers_international;
use std::path::Path;

/// Parse JSON files and extract numbers
pub fn parse_json_file(file_path: &Path) -> crate::error::Result<Vec<f64>> {
    let content = std::fs::read_to_string(file_path).map_err(|e| {
        crate::error::BenfError::FileError(format!("Failed to read JSON file: {e}"))
    })?;

    parse_json_content(&content)
}

/// Parse JSON content from string
pub fn parse_json_content(content: &str) -> crate::error::Result<Vec<f64>> {
    let json_value: serde_json::Value = serde_json::from_str(content)
        .map_err(|e| crate::error::BenfError::ParseError(format!("Invalid JSON: {e}")))?;

    let numbers = extract_numbers_from_json_value(&json_value);

    if numbers.is_empty() {
        return Err(crate::error::BenfError::NoNumbersFound);
    }

    Ok(numbers)
}

/// Parse XML files and extract numbers
pub fn parse_xml_file(file_path: &Path) -> crate::error::Result<Vec<f64>> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| crate::error::BenfError::FileError(format!("Failed to read XML file: {e}")))?;

    parse_xml_content(&content)
}

/// Parse XML content from string
pub fn parse_xml_content(content: &str) -> crate::error::Result<Vec<f64>> {
    // Simple XML parsing - extract text content and parse numbers
    // For more complex XML, consider using a proper XML parser
    let text_content = extract_text_from_xml(content);
    let numbers = extract_numbers_international(&text_content);

    if numbers.is_empty() {
        return Err(crate::error::BenfError::NoNumbersFound);
    }

    Ok(numbers)
}

/// Parse YAML files and extract numbers
pub fn parse_yaml_file(file_path: &Path) -> crate::error::Result<Vec<f64>> {
    let content = std::fs::read_to_string(file_path).map_err(|e| {
        crate::error::BenfError::FileError(format!("Failed to read YAML file: {e}"))
    })?;

    parse_yaml_content(&content)
}

/// Parse YAML content from string
pub fn parse_yaml_content(content: &str) -> crate::error::Result<Vec<f64>> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(content)
        .map_err(|e| crate::error::BenfError::ParseError(format!("Invalid YAML: {e}")))?;

    let numbers = extract_numbers_from_yaml_value(&yaml_value);

    if numbers.is_empty() {
        return Err(crate::error::BenfError::NoNumbersFound);
    }

    Ok(numbers)
}

/// Parse TOML files and extract numbers
pub fn parse_toml_file(file_path: &Path) -> crate::error::Result<Vec<f64>> {
    let content = std::fs::read_to_string(file_path).map_err(|e| {
        crate::error::BenfError::FileError(format!("Failed to read TOML file: {e}"))
    })?;

    parse_toml_content(&content)
}

/// Parse TOML content from string
pub fn parse_toml_content(content: &str) -> crate::error::Result<Vec<f64>> {
    let toml_value: toml::Value = toml::from_str(content)
        .map_err(|e| crate::error::BenfError::ParseError(format!("Invalid TOML: {e}")))?;

    let numbers = extract_numbers_from_toml_value(&toml_value);

    if numbers.is_empty() {
        return Err(crate::error::BenfError::NoNumbersFound);
    }

    Ok(numbers)
}

/// Recursively extract numbers from JSON value
fn extract_numbers_from_json_value(value: &serde_json::Value) -> Vec<f64> {
    let mut numbers = Vec::new();

    match value {
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                if f != 0.0 && f.is_finite() {
                    numbers.push(f);
                }
            }
        }
        serde_json::Value::String(s) => {
            // Extract numbers from string content (including international numerals)
            numbers.extend(extract_numbers_international(s));
        }
        serde_json::Value::Array(arr) => {
            for item in arr {
                numbers.extend(extract_numbers_from_json_value(item));
            }
        }
        serde_json::Value::Object(obj) => {
            for (_, val) in obj {
                numbers.extend(extract_numbers_from_json_value(val));
            }
        }
        _ => {} // Skip null, bool
    }

    numbers
}

/// Recursively extract numbers from YAML value
fn extract_numbers_from_yaml_value(value: &serde_yaml::Value) -> Vec<f64> {
    let mut numbers = Vec::new();

    match value {
        serde_yaml::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                if f != 0.0 && f.is_finite() {
                    numbers.push(f);
                }
            }
        }
        serde_yaml::Value::String(s) => {
            numbers.extend(extract_numbers_international(s));
        }
        serde_yaml::Value::Sequence(seq) => {
            for item in seq {
                numbers.extend(extract_numbers_from_yaml_value(item));
            }
        }
        serde_yaml::Value::Mapping(map) => {
            for (_, val) in map {
                numbers.extend(extract_numbers_from_yaml_value(val));
            }
        }
        _ => {} // Skip null, bool
    }

    numbers
}

/// Recursively extract numbers from TOML value
fn extract_numbers_from_toml_value(value: &toml::Value) -> Vec<f64> {
    let mut numbers = Vec::new();

    match value {
        toml::Value::Integer(i) => {
            if *i != 0 {
                numbers.push(*i as f64);
            }
        }
        toml::Value::Float(f) => {
            if *f != 0.0 && f.is_finite() {
                numbers.push(*f);
            }
        }
        toml::Value::String(s) => {
            numbers.extend(extract_numbers_international(s));
        }
        toml::Value::Array(arr) => {
            for item in arr {
                numbers.extend(extract_numbers_from_toml_value(item));
            }
        }
        toml::Value::Table(table) => {
            for (_, val) in table {
                numbers.extend(extract_numbers_from_toml_value(val));
            }
        }
        _ => {} // Skip datetime, bool
    }

    numbers
}

/// Simple XML text extraction
fn extract_text_from_xml(xml_content: &str) -> String {
    let mut text = String::new();
    let mut in_tag = false;

    for c in xml_content.chars() {
        match c {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                text.push(' '); // Add space between elements
            }
            _ if !in_tag => text.push(c),
            _ => {} // Skip tag content
        }
    }

    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parsing() {
        let json_content = r#"{
            "sales": 1234.56,
            "expenses": 567.89,
            "items": [123, 456, 789],
            "description": "Total: 一千二百三十四"
        }"#;

        let numbers = parse_json_content(json_content).unwrap();
        assert!(!numbers.is_empty());
        assert!(numbers.contains(&1234.56));
        assert!(numbers.contains(&567.89));
        assert!(numbers.contains(&123.0));
    }

    #[test]
    fn test_yaml_parsing() {
        let yaml_content = r#"
financial_report:
  revenue: 1234567
  expenses: 567890
transactions:
  - amount: 123.45
  - amount: 234.56
"#;

        let numbers = parse_yaml_content(yaml_content).unwrap();
        assert!(!numbers.is_empty());
        assert!(numbers.contains(&1234567.0));
        assert!(numbers.contains(&567890.0));
    }

    #[test]
    fn test_toml_parsing() {
        let toml_content = r#"
[financial_report]
revenue = 1234567
expenses = 567890

[[transactions]]
amount = 123.45

[[transactions]]
amount = 234.56
"#;

        let numbers = parse_toml_content(toml_content).unwrap();
        assert!(!numbers.is_empty());
        assert!(numbers.contains(&1234567.0));
        assert!(numbers.contains(&567890.0));
    }

    #[test]
    fn test_xml_text_extraction() {
        let xml_content = "<root><item>123</item><value>456.78</value></root>";
        let text = extract_text_from_xml(xml_content);
        assert!(text.contains("123"));
        assert!(text.contains("456.78"));
    }
}
