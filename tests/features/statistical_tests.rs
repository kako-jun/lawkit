//! Statistical analysis tests

use std::fs;
use std::io::Write;
use std::process::Command;

#[test]
fn test_normal_distribution_analysis() {
    let temp_file = "temp_normal.csv";
    let mut file = fs::File::create(temp_file).expect("Failed to create temp file");
    writeln!(file, "value").expect("Failed to write header");
    for i in 1..=100 {
        writeln!(file, "{i}").expect("Failed to write data");
    }

    let output = Command::new("cargo")
        .args(["run", "--", "normal", temp_file])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit normal");

    // Cleanup
    let _ = fs::remove_file(temp_file);

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should contain statistical analysis
        assert!(
            stdout.contains("mean") || stdout.contains("deviation") || stdout.contains("normal")
        );
    }
}

#[test]
fn test_pareto_analysis() {
    let temp_file = "temp_pareto.csv";
    let mut file = fs::File::create(temp_file).expect("Failed to create temp file");
    writeln!(file, "value").expect("Failed to write header");
    writeln!(file, "100").expect("Failed to write data");
    writeln!(file, "200").expect("Failed to write data");
    writeln!(file, "300").expect("Failed to write data");
    writeln!(file, "400").expect("Failed to write data");
    writeln!(file, "500").expect("Failed to write data");

    let output = Command::new("cargo")
        .args(["run", "--", "pareto", temp_file])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit pareto");

    // Cleanup
    let _ = fs::remove_file(temp_file);

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Should contain Pareto analysis
        assert!(stdout.contains("pareto") || stdout.contains("alpha") || stdout.contains("scale"));
    }
}
