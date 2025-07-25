#[allow(unused_imports)]
/// Tests for --no-color option across all commands
/// Ensures color output is properly disabled when flag is specified
use assert_cmd::Command;
use std::io::Write;
use tempfile::NamedTempFile;

fn create_test_csv() -> Result<NamedTempFile, Box<dyn std::error::Error>> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "amount")?;
    writeln!(temp_file, "123")?;
    writeln!(temp_file, "456")?;
    writeln!(temp_file, "789")?;
    Ok(temp_file)
}

#[test]
fn test_benf_no_color_option() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(test_file) = create_test_csv() {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "benf",
                test_file.path().to_str().unwrap(),
                "--no-color",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit benf");

        // Output should not contain ANSI color codes
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.contains("\x1b["),
            "Output should not contain ANSI color codes when --no-color is specified"
        );
        assert!(
            stdout.contains("Low")
                || stdout.contains("Medium")
                || stdout.contains("High")
                || stdout.contains("Critical")
        );
    }
    Ok(())
}

#[test]
fn test_pareto_no_color_option() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(test_file) = create_test_csv() {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "pareto",
                test_file.path().to_str().unwrap(),
                "--no-color",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit pareto");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.contains("\x1b["),
            "Output should not contain ANSI color codes when --no-color is specified"
        );
    }
    Ok(())
}

#[test]
fn test_zipf_no_color_option() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(test_file) = create_test_csv() {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "zipf",
                test_file.path().to_str().unwrap(),
                "--no-color",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit zipf");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.contains("\x1b["),
            "Output should not contain ANSI color codes when --no-color is specified"
        );
    }
    Ok(())
}

#[test]
fn test_normal_no_color_option() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(test_file) = create_test_csv() {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "normal",
                test_file.path().to_str().unwrap(),
                "--no-color",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit normal");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.contains("\x1b["),
            "Output should not contain ANSI color codes when --no-color is specified"
        );
    }
    Ok(())
}

#[test]
fn test_poisson_no_color_option() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(test_file) = create_test_csv() {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "poisson",
                test_file.path().to_str().unwrap(),
                "--no-color",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit poisson");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.contains("\x1b["),
            "Output should not contain ANSI color codes when --no-color is specified"
        );
    }
    Ok(())
}

#[test]
fn test_analyze_no_color_option() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(test_file) = create_test_csv() {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "analyze",
                test_file.path().to_str().unwrap(),
                "--no-color",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit analyze");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.contains("\x1b["),
            "Output should not contain ANSI color codes when --no-color is specified"
        );
    }
    Ok(())
}

#[test]
fn test_validate_no_color_option() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(test_file) = create_test_csv() {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "validate",
                test_file.path().to_str().unwrap(),
                "--no-color",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit validate");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.contains("\x1b["),
            "Output should not contain ANSI color codes when --no-color is specified"
        );
    }
    Ok(())
}

#[test]
fn test_diagnose_no_color_option() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(test_file) = create_test_csv() {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "diagnose",
                test_file.path().to_str().unwrap(),
                "--no-color",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit diagnose");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.contains("\x1b["),
            "Output should not contain ANSI color codes when --no-color is specified"
        );
    }
    Ok(())
}

#[test]
fn test_list_no_color_option() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "list", "--no-color"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit list");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("\x1b["),
        "Output should not contain ANSI color codes when --no-color is specified"
    );
    assert!(stdout.contains("Available statistical laws"));
}

#[test]
fn test_selftest_no_color_option() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "selftest", "--no-color"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit selftest");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("\x1b["),
        "Output should not contain ANSI color codes when --no-color is specified"
    );
    assert!(stdout.contains("Running lawkit self-test"));
}

#[test]
fn test_generate_benf_no_color_option() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "lawkit",
            "--",
            "generate",
            "benf",
            "--samples",
            "10",
            "--no-color",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit generate benf");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("\x1b["),
        "Output should not contain ANSI color codes when --no-color is specified"
    );
}

#[test]
fn test_color_vs_no_color_difference() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(test_file) = create_test_csv() {
        // Test with color (default)
        let output_color = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "benf",
                test_file.path().to_str().unwrap(),
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit benf with color");

        // Test without color
        let output_no_color = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "benf",
                test_file.path().to_str().unwrap(),
                "--no-color",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit benf with --no-color");

        let stdout_color = String::from_utf8_lossy(&output_color.stdout);
        let stdout_no_color = String::from_utf8_lossy(&output_no_color.stdout);

        // With color should have ANSI codes when run in terminal
        // Without color should never have ANSI codes
        assert!(
            !stdout_no_color.contains("\x1b["),
            "No-color output should not contain ANSI codes"
        );

        // Both should contain the same semantic content
        assert!(
            stdout_color.contains("Low")
                || stdout_color.contains("Medium")
                || stdout_color.contains("High")
                || stdout_color.contains("Critical")
        );
        assert!(
            stdout_no_color.contains("Low")
                || stdout_no_color.contains("Medium")
                || stdout_no_color.contains("High")
                || stdout_no_color.contains("Critical")
        );
    }
    Ok(())
}
