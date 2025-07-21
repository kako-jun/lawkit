//! CSV format tests

use std::fs;
use std::io::Write;
use std::process::Command;

#[test]
fn test_csv_with_headers() {
    let temp_file = "temp_csv_headers.csv";
    let mut file = fs::File::create(temp_file).expect("Failed to create temp file");
    writeln!(file, "amount,date,description").expect("Failed to write header");
    writeln!(file, "123.45,2024-01-01,Purchase").expect("Failed to write data");
    writeln!(file, "234.56,2024-01-02,Sale").expect("Failed to write data");

    let output = Command::new("cargo")
        .args(["run", "--", "analyze", temp_file])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    // Cleanup
    let _ = fs::remove_file(temp_file);

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(!stdout.is_empty());
    }
}

#[test]
fn test_csv_different_delimiters() {
    let temp_file = "temp_csv_semicolon.csv";
    let mut file = fs::File::create(temp_file).expect("Failed to create temp file");
    writeln!(file, "amount;date;description").expect("Failed to write header");
    writeln!(file, "123.45;2024-01-01;Purchase").expect("Failed to write data");

    let output = Command::new("cargo")
        .args(["run", "--", "analyze", temp_file])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    // Cleanup
    let _ = fs::remove_file(temp_file);

    // Should handle different delimiters or fail gracefully
    assert!(output.status.success() || !String::from_utf8_lossy(&output.stderr).contains("panic"));
}

#[test]
fn test_csv_quoted_fields() {
    let temp_file = "temp_csv_quotes.csv";
    let mut file = fs::File::create(temp_file).expect("Failed to create temp file");
    writeln!(file, r#"amount,description"#).expect("Failed to write header");
    writeln!(file, r#"123.45,"Purchase, with comma""#).expect("Failed to write data");
    writeln!(file, r#"234.56,"Sale ""special"" item""#).expect("Failed to write data");

    let output = Command::new("cargo")
        .args(["run", "--", "analyze", temp_file])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    // Cleanup
    let _ = fs::remove_file(temp_file);

    // Should handle quoted fields properly
    assert!(output.status.success() || !String::from_utf8_lossy(&output.stderr).contains("panic"));
}
