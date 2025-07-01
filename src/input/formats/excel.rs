use calamine::{Reader, Xlsx, Xls, open_workbook_auto, DataType};
use std::path::Path;
use crate::core::international::extract_numbers_international;

/// Parse Excel files (.xlsx, .xls) and extract numbers
pub fn parse_excel_file(file_path: &Path) -> crate::Result<Vec<f64>> {
    let extension = file_path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "xlsx" => parse_xlsx_file(file_path),
        "xls" => parse_xls_file(file_path),
        _ => {
            // Try auto-detection
            parse_excel_auto(file_path)
        }
    }
}

/// Parse XLSX files specifically
fn parse_xlsx_file(file_path: &Path) -> crate::Result<Vec<f64>> {
    let mut workbook: Xlsx<_> = calamine::open_workbook(file_path)
        .map_err(|e| crate::BenfError::FileError(format!("Failed to open XLSX file: {}", e)))?;

    extract_numbers_from_xlsx(&mut workbook)
}

/// Parse XLS files specifically  
fn parse_xls_file(file_path: &Path) -> crate::Result<Vec<f64>> {
    let mut workbook: Xls<_> = calamine::open_workbook(file_path)
        .map_err(|e| crate::BenfError::FileError(format!("Failed to open XLS file: {}", e)))?;

    extract_numbers_from_xls(&mut workbook)
}

/// Auto-detect Excel format and parse
fn parse_excel_auto(file_path: &Path) -> crate::Result<Vec<f64>> {
    let workbook = open_workbook_auto(file_path)
        .map_err(|e| crate::BenfError::FileError(format!("Failed to open Excel file: {}", e)))?;

    match workbook {
        calamine::Sheets::Xlsx(mut xlsx) => extract_numbers_from_xlsx(&mut xlsx),
        calamine::Sheets::Xls(mut xls) => extract_numbers_from_xls(&mut xls),
        calamine::Sheets::Xlsb(mut xlsb) => extract_numbers_from_xlsb(&mut xlsb),
        calamine::Sheets::Ods(mut ods) => extract_numbers_from_ods(&mut ods),
    }
}

/// Extract numbers from XLSX workbook
fn extract_numbers_from_xlsx(workbook: &mut Xlsx<std::io::BufReader<std::fs::File>>) -> crate::Result<Vec<f64>> {
    extract_numbers_from_workbook(workbook)
}

/// Extract numbers from XLS workbook
fn extract_numbers_from_xls(workbook: &mut Xls<std::io::BufReader<std::fs::File>>) -> crate::Result<Vec<f64>> {
    extract_numbers_from_workbook(workbook)
}

/// Extract numbers from XLSB workbook
fn extract_numbers_from_xlsb(workbook: &mut calamine::Xlsb<std::io::BufReader<std::fs::File>>) -> crate::Result<Vec<f64>> {
    extract_numbers_from_workbook(workbook)
}

/// Extract numbers from ODS workbook
fn extract_numbers_from_ods(workbook: &mut calamine::Ods<std::io::BufReader<std::fs::File>>) -> crate::Result<Vec<f64>> {
    extract_numbers_from_workbook(workbook)
}

/// Extract numbers from Excel workbook (generic version using Reader trait bound)
fn extract_numbers_from_workbook<R: Reader<std::io::BufReader<std::fs::File>>>(workbook: &mut R) -> crate::Result<Vec<f64>> {
    let mut all_numbers = Vec::new();
    
    // Get all worksheet names
    let sheet_names = workbook.sheet_names().to_vec();
    
    for sheet_name in sheet_names {
        if let Some(Ok(range)) = workbook.worksheet_range(&sheet_name) {
            // Process each cell in the range
            for row in range.rows() {
                for cell in row {
                    match cell {
                        // Direct numeric values
                        DataType::Float(f) => {
                            if *f != 0.0 && f.is_finite() {
                                all_numbers.push(*f);
                            }
                        },
                        DataType::Int(i) => {
                            if *i != 0 {
                                all_numbers.push(*i as f64);
                            }
                        },
                        // Text that might contain numbers (including international numerals)
                        DataType::String(s) => {
                            let extracted = extract_numbers_international(s);
                            all_numbers.extend(extracted);
                        },
                        // Skip other types (empty, bool, error, etc.)
                        _ => {}
                    }
                }
            }
        }
    }
    
    if all_numbers.is_empty() {
        return Err(crate::BenfError::NoNumbersFound);
    }
    
    Ok(all_numbers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_excel_parsing_concept() {
        // This test demonstrates the concept - actual files would be needed for real testing
        let test_path = PathBuf::from("nonexistent.xlsx");
        
        // Test should fail gracefully for non-existent file
        let result = parse_excel_file(&test_path);
        assert!(result.is_err());
        
        // Check error type
        match result {
            Err(crate::BenfError::FileError(_)) => {
                // Expected file error
            },
            _ => panic!("Expected file error for non-existent Excel file"),
        }
    }
}