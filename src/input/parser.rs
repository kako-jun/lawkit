use crate::core::extract_numbers;

/// Extract numbers from text input
pub fn parse_text_input(text: &str) -> crate::Result<Vec<f64>> {
    let numbers = extract_numbers(text);
    
    if numbers.is_empty() {
        return Err(crate::BenfError::NoNumbersFound);
    }
    
    Ok(numbers)
}