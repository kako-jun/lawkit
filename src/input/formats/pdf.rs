use pdf_extract::extract_text;
use std::path::Path;
use crate::core::international::extract_numbers_international;

/// Parse PDF files and extract numbers from text content
pub fn parse_pdf_file(file_path: &Path) -> crate::Result<Vec<f64>> {
    // Extract text from PDF file path
    let text = extract_text(file_path)
        .map_err(|e| crate::BenfError::ParseError(format!("Failed to extract text from PDF: {}", e)))?;
    
    // Extract numbers from the text (including international numerals)
    let numbers = extract_numbers_international(&text);
    
    if numbers.is_empty() {
        return Err(crate::BenfError::NoNumbersFound);
    }
    
    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_pdf_parsing_concept() {
        // This test demonstrates the concept - actual PDF files would be needed for real testing
        let test_path = PathBuf::from("nonexistent.pdf");
        
        // Test should fail gracefully for non-existent file
        let result = parse_pdf_file(&test_path);
        assert!(result.is_err());
        
        // Check error type - pdf-extract returns ParseError for non-existent files
        match result {
            Err(crate::BenfError::ParseError(_)) => {
                // Expected parse error for non-existent file
            },
            Err(crate::BenfError::FileError(_)) => {
                // Also acceptable as file error
            },
            _ => panic!("Expected parse or file error for non-existent PDF file"),
        }
    }
}