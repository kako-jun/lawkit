//! Error handling tests for lawkit

use std::process::Command;

#[test]
fn test_missing_file_error() {
    let output = Command::new("cargo")
        .args(["run", "--", "analyze", "nonexistent_file.csv"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("No such file") || stderr.contains("not found") || stderr.contains("error")
    );
}

#[test]
fn test_invalid_subcommand() {
    let output = Command::new("cargo")
        .args(["run", "--", "invalid_command"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    assert!(!output.status.success());
}

#[test]
fn test_invalid_option() {
    let output = Command::new("cargo")
        .args(["run", "--", "analyze", "--invalid-option"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    assert!(!output.status.success());
}
