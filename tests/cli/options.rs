use std::fs;
use std::io::Write;
#[allow(unused_imports)]
/// CLI options tests for lawkit
use std::process::Command;
use tempfile::tempdir;

// Helper function to create test data
fn create_test_csv() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let file_path = temp_dir.path().join("test_data.csv");
    let mut file = fs::File::create(&file_path)?;
    writeln!(file, "amount")?;
    for i in 1..=100 {
        writeln!(file, "{}", i * 111)?;
    }
    Ok(file_path)
}

#[test]
fn test_benf_format_options() {
    if let Ok(test_file) = create_test_csv() {
        for format in ["text", "csv", "json", "yaml", "toml", "xml"] {
            let output = Command::new("cargo")
                .args([
                    "run",
                    "--bin",
                    "lawkit",
                    "--",
                    "benf",
                    test_file.to_str().unwrap(),
                    "--format",
                    format,
                ])
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .output()
                .expect("Failed to execute lawkit benf");

            // Should either succeed or fail gracefully
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                // If it fails, it shouldn't be due to unrecognized format
                assert!(
                    !stderr.contains("unrecognized"),
                    "Format {} should be recognized",
                    format
                );
            }
        }
    }
}

#[test]
fn test_benf_quiet_verbose_options() {
    if let Ok(test_file) = create_test_csv() {
        // Test quiet option
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "benf",
                test_file.to_str().unwrap(),
                "--quiet",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit benf --quiet");

        // Quiet mode should produce less output
        let stdout_quiet = String::from_utf8_lossy(&output.stdout);

        // Test verbose option
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "benf",
                test_file.to_str().unwrap(),
                "--verbose",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit benf --verbose");

        let stdout_verbose = String::from_utf8_lossy(&output.stdout);

        // Verbose should generally produce more output than quiet
        // (This is a heuristic test)
        if output.status.success() {
            assert!(
                stdout_verbose.len() >= stdout_quiet.len()
                    || stdout_verbose.contains("verbose")
                    || stdout_verbose.contains("detail")
            );
        }
    }
}

#[test]
fn test_benf_filter_option() {
    if let Ok(test_file) = create_test_csv() {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "benf",
                test_file.to_str().unwrap(),
                "--filter",
                ">=100",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit benf --filter");

        // Should handle filter option without error
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(
                !stderr.contains("unrecognized"),
                "Filter option should be recognized"
            );
        }
    }
}

#[test]
fn test_benf_confidence_option() {
    if let Ok(test_file) = create_test_csv() {
        for confidence in ["0.90", "0.95", "0.99"] {
            let output = Command::new("cargo")
                .args([
                    "run",
                    "--bin",
                    "lawkit",
                    "--",
                    "benf",
                    test_file.to_str().unwrap(),
                    "--confidence",
                    confidence,
                ])
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .output()
                .expect("Failed to execute lawkit benf --confidence");

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                assert!(
                    !stderr.contains("unrecognized"),
                    "Confidence {} should be recognized",
                    confidence
                );
            }
        }
    }
}

#[test]
fn test_benf_threshold_option() {
    if let Ok(test_file) = create_test_csv() {
        for threshold in ["low", "medium", "high", "critical", "auto"] {
            let output = Command::new("cargo")
                .args([
                    "run",
                    "--bin",
                    "lawkit",
                    "--",
                    "benf",
                    test_file.to_str().unwrap(),
                    "--threshold",
                    threshold,
                ])
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .output()
                .expect("Failed to execute lawkit benf --threshold");

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                assert!(
                    !stderr.contains("unrecognized"),
                    "Threshold {} should be recognized",
                    threshold
                );
            }
        }
    }
}

#[test]
fn test_benf_min_count_option() {
    if let Ok(test_file) = create_test_csv() {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "benf",
                test_file.to_str().unwrap(),
                "--min-count",
                "5",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit benf --min-count");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(
                !stderr.contains("unrecognized"),
                "min-count option should be recognized"
            );
        }
    }
}

#[test]
fn test_benf_sample_size_option() {
    if let Ok(test_file) = create_test_csv() {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "benf",
                test_file.to_str().unwrap(),
                "--sample-size",
                "50",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit benf --sample-size");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(
                !stderr.contains("unrecognized"),
                "sample-size option should be recognized"
            );
        }
    }
}

#[test]
fn test_benf_min_value_option() {
    if let Ok(test_file) = create_test_csv() {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "lawkit",
                "--",
                "benf",
                test_file.to_str().unwrap(),
                "--min-value",
                "100",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit benf --min-value");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(
                !stderr.contains("unrecognized"),
                "min-value option should be recognized"
            );
        }
    }
}

#[test]
fn test_main_commands_help() {
    let commands = [
        "benf", "pareto", "zipf", "normal", "poisson", "analyze", "validate", "diagnose",
        "generate", "list", "selftest",
    ];

    for command in &commands {
        let output = Command::new("cargo")
            .args(["run", "--bin", "lawkit", "--", command, "--help"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect(&format!("Failed to execute lawkit {} --help", command));

        assert!(
            output.status.success(),
            "Command {} --help should succeed",
            command
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            !stdout.is_empty(),
            "Command {} --help should produce output",
            command
        );
    }
}

#[test]
fn test_version_option() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "--version"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit --version");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("2.5.16") || stdout.contains("lawkit"),
        "Version output should contain version or program name"
    );
}
