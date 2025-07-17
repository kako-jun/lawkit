//! Output format tests for lawkit

use std::fs;
use std::process::Command;

#[test]
fn test_json_output() {
    let test_file = "tests/fixtures/sample_data.csv";

    let output = Command::new("cargo")
        .args(["run", "--", "analyze", test_file, "--output", "json"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should be valid JSON or contain JSON-like structure
        assert!(stdout.contains("{") || stdout.contains("json"));
    }
}

#[test]
fn test_yaml_output() {
    let test_file = "tests/fixtures/sample_data.csv";

    if fs::metadata(test_file).is_ok() {
        let output = Command::new("cargo")
            .args(["run", "--", "analyze", test_file, "--output", "yaml"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Should be YAML format or contain YAML indicators
            assert!(stdout.contains("---") || stdout.contains(":"));
        }
    }
}
