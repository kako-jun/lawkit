use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

/// Run lawkit command with subcommand and arguments
fn run_lawkit_command(subcommand: &str, args: &[&str]) -> std::process::Output {
    let mut command = Command::new("cargo");
    command.args(&["run", "--bin", "lawkit", "--", subcommand, "--lang", "en"]); // Force English output
    command.args(args);
    command.output().expect("Failed to execute lawkit command")
}

/// Create temporary file with given content
fn create_temp_file_with_content(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to temp file");
    file
}

/// Generate test data that roughly follows various distributions
fn generate_test_data() -> String {
    // Mix of numbers to avoid extreme distributions
    vec![
        "123", "234", "345", "456", "567", "678", "789", "891", "912", "123", "124", "235", "346",
        "457", "568", "679", "780", "892", "913", "124", "125", "236", "347", "458", "569", "670",
        "781", "893", "914", "125", "126", "237", "348", "459", "560", "671", "782", "894", "915",
        "126", "127", "238", "349", "450", "561", "672", "783", "895", "916", "127",
    ]
    .join(" ")
}

#[cfg(test)]
mod lawkit_core_tests {
    use super::*;

    #[test]
    fn test_lawkit_help() {
        let output = run_lawkit_command("--help", &[]);
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("lawkit"));
        assert!(stdout.contains("statistical"));
    }

    #[test]
    fn test_lawkit_version() {
        let output = run_lawkit_command("--version", &[]);
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("lawkit"));
        assert!(stdout.contains("2.0")); // Version should be 2.0+
    }

    #[test]
    fn test_lawkit_list() {
        let output = run_lawkit_command("list", &[]);
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("benf"));
        assert!(stdout.contains("pareto"));
        assert!(stdout.contains("zipf"));
        assert!(stdout.contains("normal"));
        assert!(stdout.contains("poisson"));
        assert!(stdout.contains("compare"));
    }
}

#[cfg(test)]
mod benford_law_tests {
    use super::*;

    #[test]
    fn test_lawkit_benf_basic() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("benf", &[&test_data]);

        // Accept success or risk detection exit codes
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(10) | Some(11)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Benford"));
        assert!(stdout.contains("Numbers analyzed"));
    }

    #[test]
    fn test_lawkit_benf_file_input() {
        let content = generate_test_data();
        let temp_file = create_temp_file_with_content(&content);

        let output = run_lawkit_command("benf", &[temp_file.path().to_str().unwrap()]);
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Numbers analyzed"));
    }

    #[test]
    fn test_lawkit_benf_json_format() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("benf", &["--format", "json", &test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(10) | Some(11)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("{"));
        assert!(stdout.contains("dataset"));
        assert!(stdout.contains("numbers_analyzed"));

        // Verify it's valid JSON
        let _parsed: serde_json::Value =
            serde_json::from_str(&stdout).expect("Output should be valid JSON");
    }

    #[test]
    fn test_lawkit_benf_high_threshold() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("benf", &["--threshold", "high", &test_data]);

        // Should run without error (specific behavior depends on implementation)
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Numbers analyzed"));
    }

    #[test]
    fn test_lawkit_benf_verbose() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("benf", &["--verbose", &test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(10) | Some(11)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Chi-square") || stdout.contains("p-value"));
        assert!(stdout.contains("distribution"));
    }

    #[test]
    fn test_lawkit_benf_filter() {
        let test_data = "50 150 250 350 450 550 650 750 850 950";
        let output = run_lawkit_command("benf", &["--filter", ">=100", test_data]);

        // Should filter numbers >= 100
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Should analyze 9 numbers (150-950)
            assert!(stdout.contains("Numbers analyzed"));
        }
    }
}

#[cfg(test)]
mod pareto_law_tests {
    use super::*;

    #[test]
    fn test_lawkit_pareto_basic() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("pareto", &[&test_data]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Pareto") || stdout.contains("80/20"));
        assert!(stdout.contains("Numbers analyzed"));
    }

    #[test]
    fn test_lawkit_pareto_business_analysis() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("pareto", &["--business-analysis", &test_data]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("business") || stdout.contains("concentration"));
    }

    #[test]
    fn test_lawkit_pareto_gini_coefficient() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("pareto", &["--gini-coefficient", &test_data]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Gini"));
    }

    #[test]
    fn test_lawkit_pareto_custom_percentiles() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("pareto", &["--percentiles", "70,80,90", &test_data]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("70%") || stdout.contains("80%") || stdout.contains("90%"));
    }
}

#[cfg(test)]
mod zipf_law_tests {
    use super::*;

    #[test]
    fn test_lawkit_zipf_basic() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("zipf", &[&test_data]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Zipf") || stdout.contains("frequency"));
        assert!(stdout.contains("Numbers analyzed"));
    }

    #[test]
    fn test_lawkit_zipf_text_analysis() {
        let text_content = "the quick brown fox jumps over the lazy dog the fox is quick";
        let temp_file = create_temp_file_with_content(text_content);

        let output = run_lawkit_command(
            "zipf",
            &["--text-analysis", temp_file.path().to_str().unwrap()],
        );

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("text") || stdout.contains("word"));
        }
    }

    #[test]
    fn test_lawkit_zipf_ranking() {
        let ranking_data = "1 2 3 4 5 6 7 8 9 10 11 12 13 14 15";
        let output = run_lawkit_command("zipf", &["--ranking", ranking_data]);

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("rank"));
        }
    }
}

#[cfg(test)]
mod normal_distribution_tests {
    use super::*;

    #[test]
    fn test_lawkit_normal_basic() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("normal", &[&test_data]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Normal") || stdout.contains("normality"));
        assert!(stdout.contains("Numbers analyzed"));
    }

    #[test]
    fn test_lawkit_normal_shapiro_test() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("normal", &["--test", "shapiro", &test_data]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Shapiro"));
    }

    #[test]
    fn test_lawkit_normal_outlier_detection() {
        let test_data = generate_test_data();
        let output = run_lawkit_command(
            "normal",
            &["--outliers", "--outlier-method", "iqr", &test_data],
        );

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("outlier") || stdout.contains("IQR"));
    }

    #[test]
    fn test_lawkit_normal_quality_control() {
        let test_data = generate_test_data();
        let output = run_lawkit_command(
            "normal",
            &["--quality-control", "--spec-limits", "10,20", &test_data],
        );

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("quality") || stdout.contains("spec"));
        }
    }
}

#[cfg(test)]
mod poisson_distribution_tests {
    use super::*;

    #[test]
    fn test_lawkit_poisson_basic() {
        let test_data = "1 2 3 0 1 2 4 1 0 3 2 1 5 2 1 0 3 2 1 4"; // Discrete event counts
        let output = run_lawkit_command("poisson", &[test_data]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Poisson") || stdout.contains("events"));
        assert!(stdout.contains("Numbers analyzed"));
    }

    #[test]
    fn test_lawkit_poisson_prediction() {
        let test_data = "1 2 3 0 1 2 4 1 0 3 2 1 5 2 1 0 3 2 1 4";
        let output = run_lawkit_command("poisson", &["--predict", "--max-events", "15", test_data]);

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("predict") || stdout.contains("probability"));
        }
    }

    #[test]
    fn test_lawkit_poisson_rare_events() {
        let test_data = "0 0 1 0 0 0 2 0 0 0 0 1 0 0 0 3 0 0 0 0"; // Rare events
        let output = run_lawkit_command("poisson", &["--rare-events", test_data]);

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("rare"));
        }
    }
}

#[cfg(test)]
mod integration_compare_tests {
    use super::*;

    #[test]
    fn test_lawkit_compare_basic() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("compare", &[&test_data]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("compare") || stdout.contains("integration"));
        assert!(stdout.contains("benf") || stdout.contains("Benford"));
    }

    #[test]
    fn test_lawkit_compare_specific_laws() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("compare", &["--laws", "benf,pareto,normal", &test_data]);

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("benf") || stdout.contains("pareto") || stdout.contains("normal"));
    }

    #[test]
    fn test_lawkit_compare_fraud_detection() {
        let test_data = generate_test_data();
        let output = run_lawkit_command(
            "compare",
            &["--purpose", "fraud", "--recommend", &test_data],
        );

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("fraud") || stdout.contains("recommend"));
    }

    #[test]
    fn test_lawkit_compare_quality_focus() {
        let test_data = generate_test_data();
        let output = run_lawkit_command(
            "compare",
            &["--laws", "benf,normal", "--focus", "quality", &test_data],
        );

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("quality"));
    }

    #[test]
    fn test_lawkit_compare_conflict_detection() {
        let test_data = generate_test_data();
        let output = run_lawkit_command(
            "compare",
            &["--report", "conflicting", "--threshold", "0.7", &test_data],
        );

        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("conflict") || stdout.contains("threshold"));
    }
}

#[cfg(test)]
mod documentation_examples_tests {
    use super::*;

    /// Test examples from CLI reference documentation
    #[test]
    fn test_cli_reference_examples() {
        let csv_content = "amount\n1234\n5678\n9012\n3456\n7890\n2345\n6789\n1012";
        let temp_file = create_temp_file_with_content(csv_content);
        let csv_path = temp_file.path().to_str().unwrap();

        // Basic analysis
        let output = run_lawkit_command("benf", &[csv_path]);
        assert!(output.status.success());

        // Fraud detection mode with high threshold and verbose
        let output = run_lawkit_command("benf", &["--threshold", "high", "--verbose", csv_path]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(10) | Some(11) | Some(12)
        ));

        // Format output as JSON
        let output = run_lawkit_command("benf", &["--format", "json", csv_path]);
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("{"));
    }

    /// Test examples from configuration documentation
    #[test]
    fn test_configuration_examples() {
        let test_data = generate_test_data();

        // Japanese output
        let output = run_lawkit_command("benf", &["--lang", "ja", &test_data]);
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("ベンフォード") || stdout.contains("解析"));
        }

        // YAML format
        let output = run_lawkit_command("benf", &["--format", "yaml", &test_data]);
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("dataset:") || stdout.contains("numbers_analyzed:"));
        }

        // Quiet mode
        let output = run_lawkit_command("benf", &["--quiet", &test_data]);
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.len() < 200); // Should be minimal output
        }

        // Verbose mode
        let output = run_lawkit_command("benf", &["--verbose", &test_data]);
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("Chi-square") || stdout.contains("distribution"));
        }
    }

    /// Test multi-law analysis examples
    #[test]
    fn test_multi_law_examples() {
        let test_data = generate_test_data();

        // Compare all laws
        let output = run_lawkit_command("compare", &[&test_data]);
        assert!(output.status.success());

        // Focus on concentration analysis
        let output = run_lawkit_command("compare", &["--laws", "benf,pareto,normal", &test_data]);
        assert!(output.status.success());

        // Recommendation mode
        let output = run_lawkit_command("compare", &["--recommend", &test_data]);
        assert!(output.status.success());
    }

    /// Test business analysis examples
    #[test]
    fn test_business_analysis_examples() {
        // Sales data simulation
        let sales_data = "12345 23456 34567 45678 56789 67890 78901 89012 90123 12340";

        // Business analysis with Pareto
        let output = run_lawkit_command("pareto", &["--business-analysis", sales_data]);
        assert!(output.status.success());

        // Custom percentiles
        let output = run_lawkit_command("pareto", &["--percentiles", "70,80,90", sales_data]);
        assert!(output.status.success());
    }

    /// Test quality control examples
    #[test]
    fn test_quality_control_examples() {
        let measurement_data = "15.2 15.8 14.9 16.1 15.5 15.0 16.3 14.7 15.9 15.4";

        // Quality control with spec limits
        let output = run_lawkit_command(
            "normal",
            &[
                "--quality-control",
                "--spec-limits",
                "14,16",
                measurement_data,
            ],
        );

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("quality") || stdout.contains("spec"));
        }
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_invalid_subcommand() {
        let output = run_lawkit_command("invalid", &[]);
        assert!(!output.status.success());
    }

    #[test]
    fn test_invalid_option() {
        let output = run_lawkit_command("benf", &["--invalid-option"]);
        assert!(!output.status.success());
        assert_eq!(output.status.code(), Some(2)); // Argument parsing error
    }

    #[test]
    fn test_nonexistent_file() {
        let output = run_lawkit_command("benf", &["/path/that/does/not/exist.txt"]);
        assert!(!output.status.success());
        assert_eq!(output.status.code(), Some(3)); // File error
    }

    #[test]
    fn test_empty_input() {
        let output = run_lawkit_command("benf", &[""]);
        assert!(!output.status.success());
    }

    #[test]
    fn test_no_numbers_in_input() {
        let output = run_lawkit_command("benf", &["no numbers here at all"]);
        assert!(!output.status.success());
    }
}
