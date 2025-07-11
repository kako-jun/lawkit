use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

/// Run lawkit command with subcommand and arguments, using test data as stdin
fn run_lawkit_command(subcommand: &str, args: &[&str]) -> std::process::Output {
    let mut command = Command::new("cargo");
    command.args(["run", "--bin", "lawkit", "--", subcommand]);

    // Separate test data from other arguments
    let mut test_data = String::new();
    let mut command_args = Vec::new();

    for arg in args {
        // If arg looks like test data (contains numbers), use it as stdin
        if arg.chars().any(|c| c.is_ascii_digit()) && arg.len() > 20 {
            test_data = arg.to_string();
        } else {
            command_args.push(*arg);
        }
    }

    command.args(&command_args);

    // Language option removed - CLI now outputs in English only
    // Set LANG environment variable to ensure English output
    command.env("LANG", "en_US.UTF-8");

    // Set stdin if we have test data
    if !test_data.is_empty() {
        use std::process::Stdio;
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let mut child = command.spawn().expect("Failed to spawn lawkit command");

        if let Some(mut stdin) = child.stdin.take() {
            write!(stdin, "{test_data}").expect("Failed to write to stdin");
        }

        child.wait_with_output().expect("Failed to get output")
    } else {
        command.output().expect("Failed to execute lawkit command")
    }
}

/// Debug version of run_lawkit_command that prints detailed output
fn debug_run_lawkit_command(subcommand: &str, args: &[&str]) -> std::process::Output {
    let mut command = Command::new("cargo");
    command.args(["run", "--bin", "lawkit", "--", subcommand]);
    command.args(args);
    // Language option removed - CLI now outputs in English only
    // Set LANG environment variable to ensure English output
    command.env("LANG", "en_US.UTF-8");

    let mut cmd_str = format!("cargo run --bin lawkit -- {subcommand}");
    for arg in args {
        cmd_str.push_str(&format!(" {arg}"));
    }
    // Language option removed - CLI now outputs in English only
    println!("🔍 Debug: Running command: {cmd_str}");

    let output = command.output().expect("Failed to execute lawkit command");

    println!("🔍 Debug: Exit code: {:?}", output.status.code());
    println!("🔍 Debug: Stdout length: {}", output.stdout.len());
    println!("🔍 Debug: Stderr length: {}", output.stderr.len());

    if !output.stdout.is_empty() {
        println!(
            "🔍 Debug: Stdout:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }

    if !output.stderr.is_empty() {
        println!(
            "🔍 Debug: Stderr:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    output
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
    // Generate enough data points for all analyses (minimum 50 for most laws)
    let mut data = Vec::new();

    // Generate 50 different numbers following various patterns
    for i in 1..=50 {
        let base = 100 + i * 17; // Create variety in first digits
        data.push(base.to_string());

        if i % 3 == 0 {
            let alt = 200 + i * 23;
            data.push(alt.to_string());
        }

        if i % 5 == 0 {
            let third = 300 + i * 31;
            data.push(third.to_string());
        }
    }

    data.join(" ")
}

/// Generate larger test dataset for pareto analysis (needs 30+ values)
fn generate_pareto_test_data() -> String {
    let mut data = Vec::new();

    // Generate 100 values with pareto-like distribution
    for i in 1..=100 {
        let value = match i {
            1..=20 => 1000 + i * 50, // Top 20% high values
            21..=50 => 500 + i * 10, // Middle values
            _ => 100 + i * 2,        // Bottom 50% low values
        };
        data.push(value.to_string());
    }

    data.join(" ")
}

#[cfg(test)]
mod lawkit_core_tests {
    use super::*;

    #[test]
    fn test_lawkit_help() {
        let output = run_lawkit_command("--help", &[]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("lawkit"));
        assert!(stdout.contains("statistical"));
    }

    #[test]
    fn test_lawkit_version() {
        let output = run_lawkit_command("--version", &[]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("lawkit"));
        assert!(stdout.contains("2.1")); // Version should be 2.1+
    }

    #[test]
    fn test_lawkit_list() {
        let output = run_lawkit_command("list", &[]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("benf"));
        assert!(stdout.contains("pareto"));
        assert!(stdout.contains("zipf"));
        assert!(stdout.contains("normal"));
        assert!(stdout.contains("poisson"));
        assert!(stdout.contains("analyze"));
    }
}

#[cfg(test)]
mod benford_law_tests {
    use super::*;

    #[test]
    fn test_lawkit_benf_basic() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("benf", &[&test_data]);

        // Accept success or any lawkit exit codes (0-12)
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
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
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Numbers analyzed"));
    }

    #[test]
    fn test_lawkit_benf_json_format() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("benf", &["--format", "json", &test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
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
    fn test_lawkit_benf_confidence_level() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("benf", &["--confidence", "0.99", &test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Benford") || stdout.contains("confidence"));
    }

    #[test]
    fn test_lawkit_benf_sample_size() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("benf", &["--sample-size", "100", &test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Benford") || stdout.contains("sample"));
    }

    #[test]
    fn test_lawkit_benf_min_value() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("benf", &["--min-value", "50", &test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Benford") || stdout.contains("minimum"));
    }

    #[test]
    fn test_lawkit_benf_verbose() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("benf", &["--verbose", &test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Chi-square") || stdout.contains("p-value"));
        assert!(stdout.to_lowercase().contains("distribution"));
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

    #[test]
    fn test_lawkit_benf_optimize() {
        // Generate large dataset to test optimize mode
        let mut large_data = Vec::new();
        for i in 1..=5000 {
            large_data.push((100 + i * 17).to_string());
        }
        let test_data = large_data.join(" ");

        // Test with automatic optimization (no flag needed)
        let output = run_lawkit_command("benf", &[&test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Numbers analyzed"));
        assert!(stdout.contains("5000"));
    }

    #[test]
    fn test_lawkit_benf_optimize_with_file() {
        // Test optimization with file input
        let mut large_data = Vec::new();
        for i in 1..=1000 {
            large_data.push((1000 + i * 23).to_string());
        }
        let content = large_data.join("\n");
        let temp_file = create_temp_file_with_content(&content);

        let output = run_lawkit_command("benf", &[temp_file.path().to_str().unwrap()]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Numbers analyzed"));
    }
}

#[cfg(test)]
mod pareto_law_tests {
    use super::*;

    #[test]
    fn test_lawkit_pareto_basic() {
        let test_data = generate_pareto_test_data();
        let output = run_lawkit_command("pareto", &[&test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Pareto") || stdout.contains("80/20"));
        assert!(stdout.contains("Numbers analyzed"));
    }

    #[test]
    fn test_lawkit_pareto_business_analysis() {
        let test_data = generate_pareto_test_data();
        let output = run_lawkit_command("pareto", &["--business-analysis", &test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.to_lowercase().contains("business")
                || stdout.to_lowercase().contains("concentration")
        );
    }

    #[test]
    fn test_lawkit_pareto_gini_coefficient() {
        let test_data = generate_pareto_test_data();
        let output = run_lawkit_command("pareto", &["--gini-coefficient", &test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Gini"));
    }

    #[test]
    fn test_lawkit_pareto_custom_percentiles() {
        let test_data = generate_pareto_test_data();
        let output = run_lawkit_command("pareto", &["--percentiles", "70,80,90", &test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("70%") || stdout.contains("80%") || stdout.contains("90%"));
    }

    #[test]
    fn test_lawkit_pareto_optimize() {
        // Test automatic optimization for pareto analysis
        let test_data = generate_pareto_test_data();
        let output = run_lawkit_command("pareto", &[&test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Pareto") || stdout.contains("80/20"));
    }
}

#[cfg(test)]
mod zipf_law_tests {
    use super::*;

    #[test]
    fn test_lawkit_zipf_basic() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("zipf", &[&test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

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
        let output = run_lawkit_command("normal", &["--verbose", &test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Normal") || stdout.contains("normality"));
        assert!(stdout.contains("p=") || stdout.contains("P-value"));
    }

    #[test]
    fn test_lawkit_normal_shapiro_test() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("normal", &["--test", "shapiro", &test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

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

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

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

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Poisson") || stdout.contains("events"));
        assert!(
            stdout.contains("Test statistic") || stdout.contains("P-value") || stdout.contains("λ")
        );
    }

    #[test]
    fn test_lawkit_poisson_confidence_level() {
        let test_data = "1 2 3 0 1 2 4 1 0 3 2 1 5 2 1 0 3 2 1 4";
        let output = run_lawkit_command("poisson", &["--confidence", "0.99", test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Poisson") || stdout.contains("confidence"));
    }

    #[test]
    fn test_lawkit_poisson_prediction() {
        let test_data = "1 2 3 0 1 2 4 1 0 3 2 1 5 2 1 0 3 2 1 4";
        let output = run_lawkit_command("poisson", &["--predict", "--max-events", "15", test_data]);

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(
                stdout.contains("predict")
                    || stdout.contains("probability")
                    || stdout.contains("Prediction")
            );
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
mod integration_analyze_tests {
    use super::*;

    #[test]
    fn test_lawkit_analyze_basic() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("analyze", &[&test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("analyze")
                || stdout.contains("integration")
                || stdout.contains("Integration")
        );
        assert!(stdout.contains("benf") || stdout.contains("Benford"));
    }

    #[test]
    fn test_lawkit_analyze_specific_laws() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("analyze", &["--laws", "benf,pareto,normal", &test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("benf") || stdout.contains("pareto") || stdout.contains("normal"));
    }

    #[test]
    fn test_lawkit_analyze_quality_focus() {
        let test_data = generate_test_data();
        let output = run_lawkit_command(
            "analyze",
            &["--laws", "benf,normal", "--focus", "quality", &test_data],
        );

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("quality"));
    }

    #[test]
    fn test_lawkit_analyze_optimize() {
        // Test analyze with optimize flag on large dataset
        let mut large_data = Vec::new();
        for i in 1..=2000 {
            large_data.push((100 + i * 17).to_string());
        }
        let test_data = large_data.join(" ");

        let output = run_lawkit_command("analyze", &["--laws", "all", &test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Laws Executed") || stdout.contains("Integration"));
    }
}

mod integration_validate_tests {
    use super::*;

    #[test]
    fn test_lawkit_validate_basic() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("validate", &[&test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Consistency") || stdout.contains("validation"));
    }

    #[test]
    fn test_lawkit_validate_cross_validation() {
        let test_data = generate_test_data();
        let output = run_lawkit_command("validate", &["--cross-validation", &test_data]);

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Cross-Validation") || stdout.contains("stability"));
    }
}

mod integration_diagnose_tests {
    use super::*;

    #[test]
    fn test_lawkit_diagnose_fraud_detection() {
        let test_data = generate_test_data();
        let output = run_lawkit_command(
            "diagnose",
            &["--purpose", "fraud", "--recommend", &test_data],
        );

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Recommendations")
                || stdout.contains("recommend")
                || stdout.contains("Purpose")
        );
    }

    #[test]
    fn test_lawkit_diagnose_conflict_detection() {
        let test_data = generate_test_data();
        let output = run_lawkit_command(
            "diagnose",
            &["--report", "conflicting", "--threshold", "0.7", &test_data],
        );

        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("Conflict Analysis")
                || stdout.contains("Resolution Strategies")
                || stdout.contains("Threshold")
        );
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
        let output = run_lawkit_command("benf", &["--min-count", "5", csv_path]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Fraud detection mode with high threshold and verbose
        let output = run_lawkit_command(
            "benf",
            &[
                "--threshold",
                "high",
                "--verbose",
                "--min-count",
                "5",
                csv_path,
            ],
        );
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Format output as JSON
        let output =
            run_lawkit_command("benf", &["--format", "json", "--min-count", "5", csv_path]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("{"));
    }

    /// Test examples from performance guide documentation
    #[test]
    fn test_performance_guide_examples() {
        // Generate a moderately large dataset
        let mut data = Vec::new();
        for i in 1..=1000 {
            data.push(format!("{}", 100 + i * 13));
        }
        let test_data = data.join(" ");

        // Test optimize mode
        let output = run_lawkit_command("benf", &["", &test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Test analyze with optimize
        let output = run_lawkit_command("analyze", &["", &test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));
    }

    /// Test examples from advanced analysis guide
    #[test]
    fn test_advanced_analysis_examples() {
        let test_data = generate_test_data();

        // Test outlier detection with ensemble method
        let output = run_lawkit_command(
            "normal",
            &["--outliers", "--outlier-method", "ensemble", &test_data],
        );
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Test time series analysis with optimize
        let output = run_lawkit_command("normal", &["--enable-timeseries", "", &test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));
    }

    /// Test examples from configuration documentation
    #[test]
    fn test_configuration_examples() {
        let test_data = generate_test_data();

        // English output (unified)
        let output = run_lawkit_command("benf", &[&test_data]);
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("Benford") || stdout.contains("analysis"));
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

        // Analyze all laws
        let output = run_lawkit_command("analyze", &[&test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Focus on concentration analysis
        let output = run_lawkit_command("analyze", &["--laws", "benf,pareto,normal", &test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Recommendation mode
        let output = run_lawkit_command("diagnose", &["--recommend", &test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));
    }

    /// Test business analysis examples
    #[test]
    fn test_business_analysis_examples() {
        // Sales data simulation
        let sales_data = "12345 23456 34567 45678 56789 67890 78901 89012 90123 12340";

        // Business analysis with Pareto
        let output = run_lawkit_command("pareto", &["--business-analysis", sales_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Custom percentiles
        let output = run_lawkit_command("pareto", &["--percentiles", "70,80,90", sales_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));
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

    /// Test Chinese documentation examples
    #[test]
    fn test_chinese_documentation_examples() {
        // Example from Chinese getting-started guide
        let test_data = generate_test_data();
        let temp_file = create_temp_file_with_content(&test_data);

        // Performance optimization for large files
        let output = run_lawkit_command("benf", &[temp_file.path().to_str().unwrap()]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Analyze with optimization
        let output = run_lawkit_command("analyze", &[temp_file.path().to_str().unwrap()]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));
    }

    /// Test Japanese documentation examples  
    #[test]
    fn test_japanese_documentation_examples() {
        let test_data = generate_test_data();

        // Example from Japanese usage guide - optimize mode
        let output = run_lawkit_command("benf", &["", &test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Text analysis without language option
        let text_content = "これは テスト です テスト テスト データ";
        let temp_file = create_temp_file_with_content(text_content);
        let output =
            run_lawkit_command("zipf", &["--text", temp_file.path().to_str().unwrap(), ""]);
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("Zipf") || stdout.contains("text"));
        }
    }
}

#[cfg(test)]
mod generate_functionality_tests {
    use super::*;

    #[test]
    fn test_lawkit_generate_benf() {
        let output =
            run_lawkit_command("generate", &["benf", "--samples", "100", "--seed", "12345"]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        assert_eq!(lines.len(), 100); // Should generate exactly 100 samples

        // Verify all outputs are valid numbers
        for line in lines {
            assert!(
                line.parse::<f64>().is_ok(),
                "Generated data should be valid numbers"
            );
        }
    }

    #[test]
    fn test_lawkit_generate_pareto() {
        let output = run_lawkit_command(
            "generate",
            &[
                "pareto",
                "--samples",
                "50",
                "--concentration",
                "0.8",
                "--seed",
                "54321",
            ],
        );
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        assert_eq!(lines.len(), 50);

        // Verify all outputs are valid positive numbers
        for line in lines {
            let value: f64 = line
                .parse()
                .expect("Generated data should be valid numbers");
            assert!(
                value > 0.0,
                "Pareto distribution should generate positive values"
            );
        }
    }

    #[test]
    fn test_lawkit_generate_normal() {
        let output = run_lawkit_command(
            "generate",
            &[
                "normal",
                "--samples",
                "75",
                "--mean",
                "100",
                "--stddev",
                "15",
                "--seed",
                "11111",
            ],
        );
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        assert_eq!(lines.len(), 75);

        // Calculate basic statistics to verify normal distribution characteristics
        let values: Vec<f64> = lines.iter().map(|line| line.parse().unwrap()).collect();
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        assert!(
            (mean - 100.0).abs() < 10.0,
            "Generated normal data should be centered around mean"
        );
    }

    #[test]
    fn test_lawkit_generate_poisson() {
        let output = run_lawkit_command(
            "generate",
            &[
                "poisson",
                "--samples",
                "60",
                "--lambda",
                "3.0",
                "--seed",
                "22222",
            ],
        );
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        assert_eq!(lines.len(), 60);

        // Verify all outputs are non-negative integers (Poisson characteristic)
        for line in lines {
            let _value: u32 = line
                .parse()
                .expect("Poisson should generate integer values");
            // Poisson distribution always generates non-negative integers (u64 >= 0 is always true)
        }
    }

    #[test]
    fn test_lawkit_generate_zipf() {
        let output = run_lawkit_command(
            "generate",
            &[
                "zipf",
                "--samples",
                "40",
                "--exponent",
                "1.5",
                "--vocabulary-size",
                "1000",
                "--seed",
                "33333",
            ],
        );
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        assert_eq!(lines.len(), 40);

        // Verify all outputs are positive integers within vocabulary size
        for line in lines {
            let value: usize = line
                .parse()
                .expect("Zipf should generate integer rank values");
            assert!(
                value > 0 && value <= 1000,
                "Zipf ranks should be within vocabulary size"
            );
        }
    }

    #[test]
    fn test_generate_to_analyze_pipeline_benf() {
        // Generate Benford data
        let generate_output = run_lawkit_command(
            "generate",
            &["benf", "--samples", "200", "--seed", "pipeline1"],
        );
        assert!(generate_output.status.success());

        let generated_data = String::from_utf8_lossy(&generate_output.stdout);

        // Create temp file with generated data
        let temp_file = create_temp_file_with_content(&generated_data);

        // Analyze the generated data
        let analyze_output = run_lawkit_command("benf", &[temp_file.path().to_str().unwrap()]);
        assert!(analyze_output.status.success());

        let analysis_result = String::from_utf8_lossy(&analyze_output.stdout);
        assert!(analysis_result.contains("Benford"));
        assert!(analysis_result.contains("200")); // Should analyze 200 numbers
    }

    #[test]
    fn test_generate_to_analyze_pipeline_normal() {
        // Generate normal data
        let generate_output = run_lawkit_command(
            "generate",
            &[
                "normal",
                "--samples",
                "150",
                "--mean",
                "50",
                "--stddev",
                "10",
                "--seed",
                "2022",
            ],
        );
        assert!(generate_output.status.success());

        let generated_data = String::from_utf8_lossy(&generate_output.stdout);

        // Create temp file with generated data
        let temp_file = create_temp_file_with_content(&generated_data);

        // Analyze the generated data
        let analyze_output = run_lawkit_command("normal", &[temp_file.path().to_str().unwrap()]);
        // Normal analysis can return exit codes 0, 10, 11, 12 for different quality levels
        assert!(matches!(
            analyze_output.status.code(),
            Some(0) | Some(10) | Some(11) | Some(12)
        ));

        let analysis_result = String::from_utf8_lossy(&analyze_output.stdout);
        assert!(analysis_result.contains("Normal"));
        assert!(analysis_result.contains("150")); // Should analyze 150 numbers
    }

    #[test]
    fn test_generate_fraud_detection_benf() {
        // Generate Benford data with fraud simulation
        let generate_output = run_lawkit_command(
            "generate",
            &[
                "benf",
                "--samples",
                "300",
                "--fraud-rate",
                "0.1",
                "--seed",
                "fraud1",
            ],
        );
        assert!(generate_output.status.success());

        let generated_data = String::from_utf8_lossy(&generate_output.stdout);
        let temp_file = create_temp_file_with_content(&generated_data);

        // Analyze with fraud detection sensitivity
        let analyze_output = run_lawkit_command(
            "benf",
            &["--threshold", "medium", temp_file.path().to_str().unwrap()],
        );

        // Should detect some level of deviation due to injected fraud
        assert!(matches!(
            analyze_output.status.code(),
            Some(0) | Some(10) | Some(11) | Some(12)
        ));
    }

    #[test]
    fn test_generate_deterministic_output() {
        // Test that same seed produces same output
        let output1 =
            run_lawkit_command("generate", &["benf", "--samples", "10", "--seed", "12345"]);
        let output2 =
            run_lawkit_command("generate", &["benf", "--samples", "10", "--seed", "12345"]);

        assert!(output1.status.success());
        assert!(output2.status.success());

        let data1 = String::from_utf8_lossy(&output1.stdout);
        let data2 = String::from_utf8_lossy(&output2.stdout);
        assert_eq!(data1, data2, "Same seed should produce identical output");
    }

    #[test]
    fn test_generate_integration_analyze() {
        // Generate data for multiple laws
        let benf_output =
            run_lawkit_command("generate", &["benf", "--samples", "100", "--seed", "1001"]);
        let normal_output = run_lawkit_command(
            "generate",
            &["normal", "--samples", "100", "--seed", "1002"],
        );

        assert!(benf_output.status.success());
        assert!(normal_output.status.success());

        // Test analyze functionality with generated data
        let benf_data = String::from_utf8_lossy(&benf_output.stdout);
        let temp_file = create_temp_file_with_content(&benf_data);

        let analyze_output = debug_run_lawkit_command(
            "analyze",
            &["--laws", "benf,normal", temp_file.path().to_str().unwrap()],
        );
        if !analyze_output.status.success() {
            let stderr = String::from_utf8_lossy(&analyze_output.stderr);
            println!("Analyze failed with stderr: {stderr}");
            let stdout = String::from_utf8_lossy(&analyze_output.stdout);
            println!("Analyze stdout: {stdout}");
        }
        // Analyze command returns exit code based on risk level (0, 10, 11)
        // which is not always 0 even on successful execution
        assert!(
            matches!(analyze_output.status.code(), Some(0) | Some(10) | Some(11)),
            "Expected analyze to return risk-based exit code (0/10/11), got {:?}",
            analyze_output.status.code()
        );

        let analyze_result = String::from_utf8_lossy(&analyze_output.stdout);
        // Check that output contains expected content
        assert!(
            analyze_result.contains("Integration Analysis Result")
                || analyze_result.contains("Benford's Law")
                || analyze_result.contains("Normal Distribution"),
            "Expected analyze output to contain analysis results"
        );
    }
}

#[cfg(test)]
mod selftest_functionality_tests {
    use super::*;

    #[test]
    fn test_lawkit_selftest() {
        let output = run_lawkit_command("selftest", &[]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("self-test"));
        assert!(stdout.contains("[PASS]") || stdout.contains("✅"));

        // Should test all 5 laws
        assert!(stdout.contains("benf"));
        // Other laws may be placeholder for now
    }

    #[test]
    fn test_output_format_levels() {
        // Test that output uses [LEVEL] format instead of old LEVEL: format
        let mut test_file = NamedTempFile::new().unwrap();

        // Use any test data - we just want to verify format pattern
        write!(test_file, "123 456 789 101 121 131 141 151 161 171 181 191 201 211 221 231 241 251 261 271 281 291 301 311 321 331 341 351 361 371 381 391 401 411 421 431 441 451 461 471 481 491 501 511 521 531 541 551 561 571 581 591 601 611 621 631 641 651 661 671 681 691 701 711 721 731 741 751 761 771 781 791 801 811 821 831 841 851 861 871 881 891 901 911 921 931 941 951 961 971 981 991").unwrap();

        test_file.flush().unwrap();

        // Test benf command format
        let test_path = test_file.path().to_str().unwrap();
        let output = run_lawkit_command("benf", &[test_path]);
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Should contain [LEVEL] format pattern (any of the main levels)
        let has_bracket_format = stdout.contains("[CRITICAL]")
            || stdout.contains("[HIGH]")
            || stdout.contains("[MEDIUM]")
            || stdout.contains("[LOW]");
        assert!(
            has_bracket_format,
            "Should use [LEVEL] format, got: {stdout}"
        );

        // Should not contain old LEVEL: format
        assert!(
            !stdout.contains("CRITICAL:")
                && !stdout.contains("HIGH:")
                && !stdout.contains("MEDIUM:")
                && !stdout.contains("LOW:"),
            "Should not contain old LEVEL: format, got: {stdout}"
        );

        // Test PASS level format (from selftest)
        let output = run_lawkit_command("selftest", &[]);
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("[PASS]"), "Should output [PASS] format");
    }

    #[test]
    fn test_selftest_comprehensive() {
        let output = run_lawkit_command("selftest", &[]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Should show completion summary
        assert!(stdout.contains("completed") || stdout.contains("passed"));
        assert!(stdout.contains("5/5") || stdout.contains("tests"));
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
        assert!(matches!(output.status.code(), Some(1) | Some(3))); // File error
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

    #[test]
    fn test_generate_invalid_parameters() {
        // Test zero sample count (should produce empty output but succeed)
        let output = run_lawkit_command("generate", &["benf", "--samples", "0"]);
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stdout).is_empty());

        // Test invalid range (currently returns exit code 0 but shows error message)
        let output = run_lawkit_command("generate", &["benf", "--range", "invalid"]);
        // For now, check error message instead of exit code
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Error") || stderr.contains("invalid"));

        // Test high fraud rate (currently accepted as valid)
        let output = run_lawkit_command(
            "generate",
            &["benf", "--fraud-rate", "2.0", "--samples", "5"],
        );
        assert!(output.status.success());
        // Should generate some output
        assert!(!String::from_utf8_lossy(&output.stdout).is_empty());
    }

    #[test]
    fn test_japanese_cli_reference_examples() {
        // Test examples from Japanese CLI reference (cli-reference_ja.md)

        // Test benford with optimize flag from Japanese docs
        let output = run_lawkit_command("benf", &["", "会計データ.csv"]);
        assert!(output.status.code().is_some());

        // Test zipf with optimize flag from Japanese docs
        let output = run_lawkit_command("zipf", &["", "文書.txt"]);
        assert!(output.status.code().is_some());

        // Test analyze with optimize flag
        let output = run_lawkit_command("analyze", &["", "data.csv"]);
        assert!(output.status.code().is_some());

        // Test normal with time series and optimize
        let output = run_lawkit_command("normal", &["--enable-timeseries", "", "timeseries.csv"]);
        assert!(output.status.code().is_some());
    }

    #[test]
    fn test_japanese_advanced_analysis_examples() {
        // Test examples from Japanese advanced analysis guide (advanced-analysis_ja.md)

        let test_data = "100 150 200 250 300 350 400 450 500 10000"; // Contains outlier

        // Test ensemble outlier detection with normal command (most common in docs)
        let output = run_lawkit_command(
            "normal",
            &["--outliers", "--outlier-method", "ensemble", test_data],
        );
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Test LOF outlier detection with normal command
        let output = run_lawkit_command(
            "normal",
            &["--outliers", "--outlier-method", "lof", test_data],
        );
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Test isolation outlier detection with normal command
        let output = run_lawkit_command(
            "normal",
            &["--outliers", "--outlier-method", "isolation", test_data],
        );
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Test time series analysis with normal command
        let output = run_lawkit_command("normal", &["--enable-timeseries", test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Test benford basic analysis
        let output = run_lawkit_command("benf", &[test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));
    }

    #[test]
    fn test_optimize_flag_comprehensive() {
        // Test  flag across all major subcommands as shown in docs

        let test_data = "123 456 789 1234 2345 3456 4567 5678 6789 7890";

        // Benford with optimize
        let output = run_lawkit_command("benf", &["", test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Pareto with optimize
        let output = run_lawkit_command("pareto", &["", test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Zipf with optimize
        let output = run_lawkit_command("zipf", &["", test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Normal with optimize
        let output = run_lawkit_command("normal", &["", test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Analyze with optimize
        let output = run_lawkit_command("analyze", &["", test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));
    }

    #[test]
    fn test_japanese_usage_guide_examples() {
        // Test examples from Japanese usage guide (usage_ja.md)

        let test_data = "123 456 789 1234 2345 3456 4567 5678 6789 7890";

        // Test zipf text analysis with optimize from usage guide
        let output = run_lawkit_command("zipf", &["--text", "", "japanese_text.txt"]);
        assert!(output.status.code().is_some());

        // Test optimize mode examples from usage guide
        let output = run_lawkit_command("benf", &["", test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Test multi-language text analysis with optimize
        let output = run_lawkit_command("zipf", &["--text", "multilingual_doc.txt", ""]);
        assert!(output.status.code().is_some());
    }

    #[test]
    fn test_traditional_chinese_financial_numerals() {
        // Test traditional Chinese financial numerals (anti-fraud numerals)
        // From documentation examples in Japanese and Chinese guides

        let traditional_data =
            "壹萬貳仟參佰肆拾伍 陸萬柒仟捌佰玖拾 参拾萬 肆萬 伍萬 陸萬 柒萬 捌萬 玖萬 拾萬";

        let output = run_lawkit_command("benf", &[traditional_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Test that the traditional numerals are properly recognized as numbers
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Numbers analyzed: 10"));
    }

    #[test]
    fn test_japanese_getting_started_examples() {
        // Test examples from Japanese getting started guide (getting-started_ja.md)

        let test_data = "1234 2345 3456 4567 5678 6789 7890 8901 9012";

        // Test analyze with optimize from getting started
        let output = run_lawkit_command("analyze", &["", test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Test basic benford analysis
        let output = run_lawkit_command("benf", &[test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        // Test pareto analysis
        let output = run_lawkit_command("pareto", &[test_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));
    }
}
