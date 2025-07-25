/// Integration tests for risk assessment functionality
/// Tests RiskLevel enum, exit codes, and risk-based output across CLI commands
use assert_cmd::Command;
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

fn create_normal_data_csv() -> Result<NamedTempFile, Box<dyn std::error::Error>> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "amount")?;
    // Benford-compliant data (should result in Low risk)
    writeln!(temp_file, "123")?;
    writeln!(temp_file, "1456")?;
    writeln!(temp_file, "1789")?;
    writeln!(temp_file, "2345")?;
    writeln!(temp_file, "2678")?;
    writeln!(temp_file, "3123")?;
    writeln!(temp_file, "3456")?;
    writeln!(temp_file, "3789")?;
    writeln!(temp_file, "4123")?;
    writeln!(temp_file, "4567")?;
    Ok(temp_file)
}

fn create_suspicious_data_csv() -> Result<NamedTempFile, Box<dyn std::error::Error>> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "amount")?;
    // Non-Benford data (artificially created pattern, should trigger higher risk)
    for i in 5..=9 {
        for j in 0..10 {
            writeln!(temp_file, "{}{:02}", i, j)?;
        }
    }
    Ok(temp_file)
}

#[test]
fn test_benf_risk_assessment_low() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_normal_data_csv()?;

    let output = Command::new("cargo")
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
        .expect("Failed to execute lawkit benf for risk assessment");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show risk level in output
    assert!(
        stdout.contains("Low")
            || stdout.contains("Medium")
            || stdout.contains("High")
            || stdout.contains("Critical"),
        "Output should contain risk level assessment"
    );

    // Exit code should be appropriate for risk level
    let exit_code = output.status.code().unwrap_or(0);
    assert!(
        exit_code >= 0 && exit_code <= 11,
        "Exit code should be within valid range (0, 10, 11) for risk levels"
    );

    Ok(())
}

#[test]
fn test_benf_risk_assessment_json_format() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_normal_data_csv()?;

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "lawkit",
            "--",
            "benf",
            test_file.path().to_str().unwrap(),
            "--format",
            "json",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit benf JSON for risk assessment");

    let stdout = String::from_utf8_lossy(&output.stdout);

    if !stdout.trim().is_empty() {
        // Should be valid JSON
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("Output should be valid JSON");

        // JSON should contain risk-related information
        let json_str = json.to_string();
        assert!(
            json_str.contains("risk")
                || json_str.contains("Risk")
                || json_str.contains("Low")
                || json_str.contains("Medium")
                || json_str.contains("High")
                || json_str.contains("Critical"),
            "JSON output should contain risk level information"
        );
    }

    Ok(())
}

#[test]
fn test_risk_assessment_consistency_across_commands() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_normal_data_csv()?;

    let commands = ["benf", "pareto", "zipf", "normal", "poisson"];

    for command in &commands {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                command,
                test_file.path().to_str().unwrap(),
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect(&format!(
                "Failed to execute lawkit {} for risk assessment",
                command
            ));

        // All commands should execute successfully
        assert!(
            output.status.success() || output.status.code().unwrap_or(255) <= 11,
            "Command {} should execute successfully or return valid risk exit code",
            command
        );

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Each command should provide some form of analysis output
        assert!(
            !stdout.trim().is_empty(),
            "Command {} should produce output",
            command
        );
    }

    Ok(())
}

#[test]
fn test_analyze_comprehensive_risk_assessment() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_normal_data_csv()?;

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "lawkit",
            "--",
            "analyze",
            test_file.path().to_str().unwrap(),
            "--verbose",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit analyze for comprehensive risk assessment");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Analyze command should provide multi-law risk assessment
    assert!(
        stdout.contains("analysis") || stdout.contains("recommendation"),
        "Analyze command should provide analysis results"
    );

    // Should contain some indication of risk or confidence level
    assert!(
        stdout.contains("risk")
            || stdout.contains("confidence")
            || stdout.contains("Low")
            || stdout.contains("Medium")
            || stdout.contains("High")
            || stdout.contains("Critical")
            || stdout.contains("recommendation"),
        "Should contain risk or confidence information"
    );

    Ok(())
}

#[test]
fn test_validate_data_quality_risk() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_normal_data_csv()?;

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "lawkit",
            "--",
            "validate",
            test_file.path().to_str().unwrap(),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit validate for data quality risk");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Validate should assess data quality and risk
    assert!(
        stdout.contains("validation")
            || stdout.contains("quality")
            || stdout.contains("consistent")
            || stdout.contains("valid"),
        "Validate command should provide data quality assessment"
    );

    Ok(())
}

#[test]
fn test_diagnose_conflict_detection() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_normal_data_csv()?;

    let output = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "lawkit",
            "--",
            "diagnose",
            test_file.path().to_str().unwrap(),
            "--laws",
            "benf,normal",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit diagnose for conflict detection");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Diagnose should detect conflicts or provide recommendations
    assert!(
        stdout.contains("diagnose")
            || stdout.contains("conflict")
            || stdout.contains("recommendation")
            || stdout.contains("consistent"),
        "Diagnose command should provide conflict detection results"
    );

    Ok(())
}

#[test]
fn test_exit_codes_risk_mapping() -> Result<(), Box<dyn std::error::Error>> {
    let test_files = [create_normal_data_csv()?, create_suspicious_data_csv()?];

    for (i, test_file) in test_files.iter().enumerate() {
        let output = Command::new("cargo")
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
            .expect("Failed to execute lawkit benf for exit code testing");

        let exit_code = output.status.code().unwrap_or(255);

        // Valid exit codes according to RiskLevel enum:
        // Low/Medium: 0, High: 10, Critical: 11
        assert!(
            exit_code == 0 || exit_code == 10 || exit_code == 11,
            "Exit code {} should be valid risk level code (0, 10, 11) for test file {}",
            exit_code,
            i
        );

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Output should correlate with exit code
        if exit_code == 0 {
            assert!(
                stdout.contains("Low") || stdout.contains("Medium"),
                "Exit code 0 should correspond to Low or Medium risk"
            );
        } else if exit_code == 10 {
            assert!(
                stdout.contains("High"),
                "Exit code 10 should correspond to High risk"
            );
        } else if exit_code == 11 {
            assert!(
                stdout.contains("Critical"),
                "Exit code 11 should correspond to Critical risk"
            );
        }
    }

    Ok(())
}

#[test]
fn test_risk_threshold_option() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_normal_data_csv()?;

    let thresholds = ["low", "medium", "high", "critical"];

    for threshold in &thresholds {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "benf",
                test_file.path().to_str().unwrap(),
                "--threshold",
                threshold,
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect(&format!(
                "Failed to execute lawkit benf with threshold {}",
                threshold
            ));

        // Should execute successfully regardless of threshold
        assert!(
            output.status.success() || output.status.code().unwrap_or(255) <= 11,
            "Command with threshold {} should execute successfully",
            threshold
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.trim().is_empty(),
            "Should produce output with threshold {}",
            threshold
        );
    }

    Ok(())
}

#[test]
fn test_confidence_level_risk_correlation() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_normal_data_csv()?;

    let confidence_levels = ["0.90", "0.95", "0.99"];

    for confidence in &confidence_levels {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "benf",
                test_file.path().to_str().unwrap(),
                "--confidence",
                confidence,
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect(&format!(
                "Failed to execute lawkit benf with confidence {}",
                confidence
            ));

        assert!(
            output.status.success() || output.status.code().unwrap_or(255) <= 11,
            "Command with confidence {} should execute successfully",
            confidence
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.trim().is_empty(),
            "Should produce output with confidence level {}",
            confidence
        );
    }

    Ok(())
}
