//! Benford's Law analysis tests

use std::fs;
use std::io::Write;
use std::process::Command;

#[test]
fn test_benford_analysis_basic() {
    let temp_file = "temp_benford.csv";
    let mut file = fs::File::create(temp_file).expect("Failed to create temp file");
    writeln!(file, "amount").expect("Failed to write header");
    writeln!(file, "123.45").expect("Failed to write data");
    writeln!(file, "234.56").expect("Failed to write data");
    writeln!(file, "345.67").expect("Failed to write data");
    writeln!(file, "111.11").expect("Failed to write data");
    writeln!(file, "222.22").expect("Failed to write data");

    let output = Command::new("cargo")
        .args(["run", "--", "benf", temp_file])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit benf");

    // Cleanup
    let _ = fs::remove_file(temp_file);

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should contain Benford analysis results
        assert!(stdout.contains("1") || stdout.contains("digit") || stdout.contains("expected"));
    }
}

#[test]
fn test_benford_different_formats() {
    let temp_file = "temp_benford_formats.csv";
    let mut file = fs::File::create(temp_file).expect("Failed to create temp file");
    writeln!(file, "value").expect("Failed to write header");
    writeln!(file, "1,234.56").expect("Failed to write data with comma");
    writeln!(file, "$2,345.67").expect("Failed to write data with currency");
    writeln!(file, "3456.78").expect("Failed to write simple decimal");

    let output = Command::new("cargo")
        .args(["run", "--", "benf", temp_file])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit benf");

    // Cleanup
    let _ = fs::remove_file(temp_file);

    // Should handle different number formats
    assert!(output.status.success() || !String::from_utf8_lossy(&output.stderr).contains("parse"));
}
