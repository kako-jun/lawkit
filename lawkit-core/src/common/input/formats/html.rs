use crate::common::international::extract_numbers_international;
use regex::Regex;
use scraper::Html;
use std::path::Path;

/// Parse HTML files and extract numbers from text content
pub fn parse_html_file(file_path: &Path) -> crate::error::Result<Vec<f64>> {
    let content = std::fs::read_to_string(file_path).map_err(|e| {
        crate::error::BenfError::FileError(format!("Failed to read HTML file: {e}"))
    })?;

    parse_html_content(&content)
}

/// Parse HTML content from string
pub fn parse_html_content(content: &str) -> crate::error::Result<Vec<f64>> {
    let _document = Html::parse_document(content);

    // First remove script and style elements from the document
    let mut clean_html = content.to_string();

    // Remove script tags and their content using regex
    let script_regex = Regex::new(r"(?is)<script[^>]*>.*?</script>").unwrap();
    clean_html = script_regex.replace_all(&clean_html, "").to_string();

    // Remove style tags and their content using regex
    let style_regex = Regex::new(r"(?is)<style[^>]*>.*?</style>").unwrap();
    clean_html = style_regex.replace_all(&clean_html, "").to_string();

    // Parse the cleaned HTML
    let clean_document = Html::parse_document(&clean_html);

    let mut all_text = String::new();

    // Extract all text from the cleaned document
    for text in clean_document.tree.nodes().filter_map(|node| {
        if let scraper::node::Node::Text(text_node) = node.value() {
            Some(text_node.text.as_ref())
        } else {
            None
        }
    }) {
        all_text.push_str(text);
        all_text.push(' ');
    }

    // Extract numbers from the collected text (including international numerals)
    let numbers = extract_numbers_international(&all_text);

    if numbers.is_empty() {
        return Err(crate::error::BenfError::NoNumbersFound);
    }

    Ok(numbers)
}

/// Parse HTML content from a URL response (for web scraping)
pub fn parse_html_from_url_response(content: &str) -> crate::error::Result<Vec<f64>> {
    parse_html_content(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_parsing() {
        let html_content = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Financial Report</title>
            <script>console.log("ignore this 999");</script>
        </head>
        <body>
            <h1>Sales Report</h1>
            <p>Revenue: $1,234.56</p>
            <p>Expenses: $567.89</p>
            <table>
                <tr><td>Item 1</td><td>123</td></tr>
                <tr><td>Item 2</td><td>456</td></tr>
            </table>
            <div>Total in Japanese: 一千二百三十四</div>
            <style>.hidden { display: none; } .amount-888 { color: red; }</style>
        </body>
        </html>
        "#;

        let numbers = parse_html_content(html_content).unwrap();
        assert!(!numbers.is_empty());

        // Should extract numbers from text but not from script/style tags
        assert!(numbers.contains(&1234.56) || numbers.contains(&1234.0));
        assert!(numbers.contains(&567.89) || numbers.contains(&567.0));
        assert!(numbers.contains(&123.0));
        assert!(numbers.contains(&456.0));

        // Should NOT contain numbers from script tags
        assert!(!numbers.contains(&999.0));
        // Should NOT contain numbers from CSS in style tags
        assert!(!numbers.contains(&888.0));
    }

    #[test]
    fn test_html_with_mixed_content() {
        let html_content = r#"
        <html>
        <body>
            <p>Sales: ১২৩৪ (Bengali)</p>
            <p>Cost: ١٢٣٤ (Arabic)</p>
            <p>Price: १२३४ (Hindi)</p>
        </body>
        </html>
        "#;

        let numbers = parse_html_content(html_content).unwrap();
        assert!(!numbers.is_empty());
        // Should extract international numerals
    }

    #[test]
    fn test_empty_html() {
        let html_content = "<html><body></body></html>";

        let result = parse_html_content(html_content);
        assert!(result.is_err());
        match result {
            Err(crate::error::BenfError::NoNumbersFound) => {
                // Expected behavior for HTML with no numbers
            }
            _ => panic!("Expected NoNumbersFound error for empty HTML"),
        }
    }
}
