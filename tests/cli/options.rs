//! CLI options tests for lawkit

use std::process::Command;

#[test]
fn test_color_options() {
    let output = Command::new("cargo")
        .args(["run", "--", "analyze", "--help"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--color"));
}

#[test]
fn test_verbose_option() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("verbose") || stdout.contains("-v"));
}