use crate::common::international::extract_numbers_international;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum FileFormat {
    Excel,        // .xlsx, .xls
    Pdf,          // .pdf
    Word,         // .docx, .doc (future)
    PowerPoint,   // .pptx, .ppt (future)
    OpenDocument, // .ods, .odt (future)
    Csv,          // .csv
    Tsv,          // .tsv
    Json,         // .json
    Xml,          // .xml
    Yaml,         // .yaml, .yml
    Toml,         // .toml
    Html,         // .html, .htm
    Text,         // .txt, or fallback
}

/// Detect file format based on extension and optionally file content
pub fn detect_file_format(file_path: &Path) -> FileFormat {
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "xlsx" | "xls" => FileFormat::Excel,
        "pdf" => FileFormat::Pdf,
        "docx" | "doc" => FileFormat::Word,
        "pptx" | "ppt" => FileFormat::PowerPoint,
        "ods" | "odt" => FileFormat::OpenDocument,
        "csv" => FileFormat::Csv,
        "tsv" => FileFormat::Tsv,
        "json" => FileFormat::Json,
        "xml" => FileFormat::Xml,
        "yaml" | "yml" => FileFormat::Yaml,
        "toml" => FileFormat::Toml,
        "html" | "htm" => FileFormat::Html,
        "txt" => FileFormat::Text,
        _ => {
            // Try content-based detection for files without clear extensions
            detect_format_by_content(file_path).unwrap_or(FileFormat::Text)
        }
    }
}

/// Attempt to detect format by examining file content
fn detect_format_by_content(file_path: &Path) -> Option<FileFormat> {
    // Read first few bytes to detect file signature
    let bytes = std::fs::read(file_path).ok()?;

    if bytes.len() < 4 {
        return None;
    }

    // Check for common file signatures
    match &bytes[0..4] {
        // ZIP-based formats (Excel, Word, etc.)
        [0x50, 0x4B, 0x03, 0x04] | [0x50, 0x4B, 0x05, 0x06] | [0x50, 0x4B, 0x07, 0x08] => {
            // Could be Excel, Word, or other ZIP-based format
            // Try to read as string and look for content clues
            if let Ok(partial_content) =
                String::from_utf8(bytes[0..std::cmp::min(1024, bytes.len())].to_vec())
            {
                if partial_content.contains("xl/") {
                    return Some(FileFormat::Excel);
                } else if partial_content.contains("word/") {
                    return Some(FileFormat::Word);
                } else if partial_content.contains("ppt/") {
                    return Some(FileFormat::PowerPoint);
                }
            }
            Some(FileFormat::Excel) // Default to Excel for ZIP files
        }
        // PDF signature
        [0x25, 0x50, 0x44, 0x46] => Some(FileFormat::Pdf), // "%PDF"
        _ => {
            // Try to parse as text and detect structured formats
            if let Ok(content) = std::fs::read_to_string(file_path) {
                detect_text_format(&content)
            } else {
                None
            }
        }
    }
}

/// Detect format for text-based files
fn detect_text_format(content: &str) -> Option<FileFormat> {
    let trimmed = content.trim();

    if trimmed.is_empty() {
        return None;
    }

    // JSON detection
    if ((trimmed.starts_with('{') && trimmed.ends_with('}'))
        || (trimmed.starts_with('[') && trimmed.ends_with(']')))
        && serde_json::from_str::<serde_json::Value>(trimmed).is_ok()
    {
        return Some(FileFormat::Json);
    }

    // XML/HTML detection
    if trimmed.starts_with('<') {
        if trimmed.contains("<!DOCTYPE html") || trimmed.contains("<html") {
            return Some(FileFormat::Html);
        } else {
            return Some(FileFormat::Xml);
        }
    }

    // TOML detection (sections in [brackets] or key = value)
    let has_brackets = trimmed.contains('[') && trimmed.contains(']');
    let has_equals = content.lines().any(|line| {
        let line = line.trim();
        !line.is_empty() && !line.starts_with('#') && line.contains('=') && !line.contains("://")
    });

    if (has_brackets || has_equals) && toml::from_str::<toml::Value>(content).is_ok() {
        return Some(FileFormat::Toml);
    }

    // YAML detection (starts with --- or has key: value patterns)
    if (trimmed.starts_with("---")
        || content.lines().any(|line| {
            let line = line.trim();
            !line.is_empty()
                && !line.starts_with('#')
                && line.contains(':')
                && !line.contains("://")
        }))
        && serde_yaml::from_str::<serde_yaml::Value>(content).is_ok()
    {
        return Some(FileFormat::Yaml);
    }

    // CSV detection (comma-separated values)
    let lines: Vec<&str> = content.lines().take(5).collect();
    if lines.len() > 1 {
        let comma_count = lines[0].matches(',').count();
        if comma_count > 0 && lines[1].matches(',').count() == comma_count {
            return Some(FileFormat::Csv);
        }

        // TSV detection (tab-separated values)
        let tab_count = lines[0].matches('\t').count();
        if tab_count > 0 && lines[1].matches('\t').count() == tab_count {
            return Some(FileFormat::Tsv);
        }
    }

    None // Default to None if no format detected
}

/// Parse file based on detected format
pub fn parse_file_by_format(
    file_path: &Path,
    format: &FileFormat,
) -> crate::error::Result<Vec<f64>> {
    use crate::common::input::formats::*;

    match format {
        FileFormat::Excel => excel::parse_excel_file(file_path),
        FileFormat::Pdf => pdf::parse_pdf_file(file_path),
        FileFormat::Word => word::parse_word_file(file_path),
        FileFormat::PowerPoint => powerpoint::parse_powerpoint_file(file_path),
        FileFormat::Csv => csv::parse_csv_file(file_path),
        FileFormat::Tsv => csv::parse_csv_file(file_path), // TSV uses same parser as CSV
        FileFormat::Json => json_xml::parse_json_file(file_path),
        FileFormat::Xml => json_xml::parse_xml_file(file_path),
        FileFormat::Yaml => json_xml::parse_yaml_file(file_path),
        FileFormat::Toml => json_xml::parse_toml_file(file_path),
        FileFormat::Html => html::parse_html_file(file_path),
        FileFormat::Text => {
            // Fallback: read as plain text
            let content = std::fs::read_to_string(file_path).map_err(|e| {
                crate::error::BenfError::FileError(format!("Failed to read text file: {e}"))
            })?;
            let numbers = extract_numbers_international(&content);
            if numbers.is_empty() {
                Err(crate::error::BenfError::NoNumbersFound)
            } else {
                Ok(numbers)
            }
        }
        FileFormat::OpenDocument => opendocument::parse_opendocument_file(file_path),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_format_detection_by_extension() {
        assert_eq!(
            detect_file_format(&PathBuf::from("test.xlsx")),
            FileFormat::Excel
        );
        assert_eq!(
            detect_file_format(&PathBuf::from("data.csv")),
            FileFormat::Csv
        );
        assert_eq!(
            detect_file_format(&PathBuf::from("config.json")),
            FileFormat::Json
        );
        assert_eq!(
            detect_file_format(&PathBuf::from("document.pdf")),
            FileFormat::Pdf
        );
        assert_eq!(
            detect_file_format(&PathBuf::from("page.html")),
            FileFormat::Html
        );
        assert_eq!(
            detect_file_format(&PathBuf::from("report.docx")),
            FileFormat::Word
        );
        assert_eq!(
            detect_file_format(&PathBuf::from("legacy.doc")),
            FileFormat::Word
        );
    }

    #[test]
    fn test_text_format_detection() {
        assert_eq!(
            detect_text_format(r#"{"key": "value"}"#),
            Some(FileFormat::Json)
        );
        assert_eq!(
            detect_text_format("<html><body></body></html>"),
            Some(FileFormat::Html)
        );
        assert_eq!(
            detect_text_format("key: value\nother: data"),
            Some(FileFormat::Yaml)
        );

        assert_eq!(
            detect_text_format("key = \"value\"\n[section]"),
            Some(FileFormat::Toml)
        );

        assert_eq!(
            detect_text_format("name,age,city\nJohn,25,NYC"),
            Some(FileFormat::Csv)
        );
    }
}
