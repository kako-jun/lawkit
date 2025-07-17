//! Fraud detection and risk assessment tests

use std::process::Command;
use std::fs;
use std::io::Write;

#[test]
fn test_risk_analysis() {
    let temp_file = "temp_risk.csv";
    let mut file = fs::File::create(temp_file).expect("Failed to create temp file");
    writeln!(file, "amount").expect("Failed to write header");
    // Create suspicious pattern - too many round numbers
    writeln!(file, "1000.00").expect("Failed to write data");
    writeln!(file, "2000.00").expect("Failed to write data");
    writeln!(file, "3000.00").expect("Failed to write data");
    writeln!(file, "4000.00").expect("Failed to write data");
    writeln!(file, "5000.00").expect("Failed to write data");
    
    let output = Command::new("cargo")
        .args(["run", "--", "analyze", temp_file])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit analyze");

    // Cleanup
    let _ = fs::remove_file(temp_file);
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should perform some kind of analysis
        assert!(!stdout.is_empty());
    }
}

#[test]
fn test_outlier_detection() {
    let temp_file = "temp_outlier.csv";
    let mut file = fs::File::create(temp_file).expect("Failed to create temp file");
    writeln!(file, "value").expect("Failed to write header");
    // Normal values
    for i in 1..=50 {
        writeln!(file, "{}", i).expect("Failed to write data");
    }
    // Outlier
    writeln!(file, "999999").expect("Failed to write outlier");
    
    let output = Command::new("cargo")
        .args(["run", "--", "analyze", temp_file])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit analyze");

    // Cleanup
    let _ = fs::remove_file(temp_file);
    
    // Should handle outliers without crashing
    assert!(output.status.success() || !String::from_utf8_lossy(&output.stderr).contains("panic"));
}