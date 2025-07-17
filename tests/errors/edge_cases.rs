//! Edge case tests for lawkit

use std::fs;
use std::io::Write;
use std::process::Command;

#[test]
fn test_empty_file() {
    let temp_file = "temp_empty.csv";
    fs::File::create(temp_file).expect("Failed to create temp file");

    let output = Command::new("cargo")
        .args(["run", "--", "analyze", temp_file])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    // Cleanup
    let _ = fs::remove_file(temp_file);

    // Should handle empty files gracefully
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success() || stderr.contains("empty") || stdout.contains("no data"));
}

#[test]
fn test_very_large_numbers() {
    let temp_file = "temp_large.csv";
    let mut file = fs::File::create(temp_file).expect("Failed to create temp file");
    writeln!(file, "value").expect("Failed to write header");
    writeln!(file, "999999999999999").expect("Failed to write large number");
    writeln!(file, "1000000000000000").expect("Failed to write large number");

    let output = Command::new("cargo")
        .args(["run", "--", "benf", temp_file])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    // Cleanup
    let _ = fs::remove_file(temp_file);

    // Should handle large numbers without crashing
    assert!(output.status.success() || !String::from_utf8_lossy(&output.stderr).contains("panic"));
}
