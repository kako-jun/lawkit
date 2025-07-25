#[allow(unused_imports)]
/// Basic CLI command tests for lawkit
use std::process::Command;

#[test]
fn test_lawkit_version() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "--version"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("lawkit"));
}

#[test]
fn test_lawkit_help() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "--help"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("benf"));
    assert!(stdout.contains("analyze"));
}

#[test]
fn test_lawkit_benf_help() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "benf", "--help"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit benf");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Benford"));
}
