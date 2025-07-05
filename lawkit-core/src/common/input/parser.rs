use super::file_detector::{detect_file_format, parse_file_by_format};
use crate::common::international::extract_numbers_international;
use std::path::Path;

/// Extract numbers from text input
use crate::error::Result;

pub fn parse_text_input(text: &str) -> Result<Vec<f64>> {
    let numbers = extract_numbers_international(text);

    if numbers.is_empty() {
        return Err(crate::error::BenfError::NoNumbersFound);
    }

    Ok(numbers)
}

/// Parse any supported file format and extract numbers
pub fn parse_file_input(file_path: &Path) -> Result<Vec<f64>> {
    // First check if file exists
    if !file_path.exists() {
        return Err(crate::error::BenfError::FileError(format!(
            "File not found: {}",
            file_path.display()
        )));
    }

    // Detect file format
    let format = detect_file_format(file_path);

    // Parse based on detected format
    parse_file_by_format(file_path, &format)
}

/// Parse input that could be either a file path or text content
pub fn parse_input_auto(input: &str) -> Result<Vec<f64>> {
    let path = Path::new(input);

    if path.exists() {
        // Input is a file path
        parse_file_input(path)
    } else {
        // Input is text content
        parse_text_input(input)
    }
}
