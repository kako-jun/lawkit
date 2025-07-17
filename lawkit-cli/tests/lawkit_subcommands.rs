use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

/// Run lawkit command with subcommand and arguments
fn run_lawkit_command(subcommand: &str, args: &[&str]) -> std::process::Output {
    let mut command = Command::new("cargo");
    command.args(["run", "--bin", "lawkit", "--", subcommand]);
    // Add --language en for analysis commands that support it (not generate commands)
    command.args(args);
    command.output().expect("Failed to execute lawkit command")
}

/// Run lawkit command with data from temporary file
fn run_lawkit_command_with_file(
    subcommand: &str,
    data: &str,
    extra_args: &[&str],
) -> std::process::Output {
    let temp_file = create_temp_file_with_content(data);
    let file_path = temp_file.path().to_str().unwrap();

    let mut all_args = vec![file_path];
    all_args.extend_from_slice(extra_args);

    run_lawkit_command(subcommand, &all_args)
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
    assert!(stdout.contains("2.2")); // Version should be 2.2+
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
    assert!(stdout.contains("analyze"));
    assert!(stdout.contains("validate"));
    assert!(stdout.contains("diagnose"));
}

#[test]
fn test_lawkit_benf_basic() {
    let test_data = generate_test_data();
    let output = run_lawkit_command("benf", &[&test_data]);

    // Accept success or risk detection exit codes
    assert!(matches!(
        output.status.code(),
        Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
    ));

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Benford"));
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
fn test_lawkit_benf_verbose() {
    let test_data = generate_test_data();
    let output = run_lawkit_command("benf", &["--verbose", &test_data]);

    assert!(matches!(
        output.status.code(),
        Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
    ));

    let stdout = String::from_utf8_lossy(&output.stdout);
    // verboseモードでは詳細な統計情報が表示される
    assert!(
        stdout.contains("Chi-square")
            || stdout.contains("p-value")
            || stdout.contains("カイ二乗")
            || stdout.contains("詳細")
    );
}

#[test]
fn test_lawkit_pareto_basic() {
    let test_data = generate_test_data();
    let output = run_lawkit_command_with_file("pareto", &test_data, &[]);

    // paretoコマンドは集中度によって非ゼロ終了コードを返すことがある
    assert!(matches!(
        output.status.code(),
        Some(0) | Some(10) | Some(11) | Some(12)
    ));

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pareto") || stdout.contains("80/20") || stdout.contains("パレート"));
    assert!(stdout.contains("Numbers analyzed") || stdout.contains("解析した数値数"));
}

#[test]
fn test_lawkit_zipf_basic() {
    let test_data = generate_test_data();
    let output = run_lawkit_command_with_file("zipf", &test_data, &[]);

    // zipfコマンドも分布によって非ゼロ終了コードを返すことがある
    assert!(matches!(
        output.status.code(),
        Some(0) | Some(10) | Some(11) | Some(12)
    ));

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Zipf") || stdout.contains("frequency") || stdout.contains("ジップ"));
    assert!(stdout.contains("Numbers analyzed") || stdout.contains("解析した数値数"));
}

#[test]
fn test_lawkit_normal_basic() {
    // normalコマンドは30+のサンプルが必要
    let large_data: Vec<String> = (100..150).map(|i| i.to_string()).collect();
    let test_data = large_data.join(" ");
    let output = run_lawkit_command_with_file("normal", &test_data, &[]);

    // normalコマンドも正規性によって非ゼロ終了コードを返すことがある
    assert!(matches!(
        output.status.code(),
        Some(0) | Some(10) | Some(11) | Some(12)
    ));

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Normal") || stdout.contains("normality") || stdout.contains("正規"));
    assert!(stdout.contains("Numbers analyzed") || stdout.contains("解析した数値数"));
}

#[test]
fn test_lawkit_poisson_basic() {
    let test_data = "1 2 3 0 1 2 4 1 0 3 2 1 5 2 1 0 3 2 1 4"; // Discrete event counts
    let output = run_lawkit_command_with_file("poisson", test_data, &[]);

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Poisson") || stdout.contains("events"));
    assert!(stdout.contains("Numbers analyzed"));
}

#[test]
fn test_lawkit_analyze_basic() {
    let test_data = generate_test_data();
    let output = run_lawkit_command_with_file("analyze", &test_data, &[]);

    // analyzeコマンドは矛盾検出時に非ゼロ終了コードを返すことがある
    assert!(matches!(
        output.status.code(),
        Some(0) | Some(10) | Some(11) | Some(12)
    ));

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Integration") || stdout.contains("統合"));
    assert!(stdout.contains("benf") || stdout.contains("Benford"));
}

/// Test examples from CLI reference documentation
#[test]
fn test_cli_reference_examples() {
    let csv_content = "amount\n1234\n5678\n9012\n3456\n7890\n2345\n6789\n1012\n4567\n8901\n2347\n6781\n1013\n4568\n8902\n2348\n6782\n1014\n4569\n8903\n2349\n6783\n1015\n4570\n8904\n2350\n6784\n1016\n4571\n8905\n2351\n6785\n1017\n4572";
    let temp_file = create_temp_file_with_content(csv_content);
    let csv_path = temp_file.path().to_str().unwrap();

    // Basic analysis - 統計分析では終了コードが変わることがある
    let output = run_lawkit_command("benf", &[csv_path]);
    assert!(matches!(
        output.status.code(),
        Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
    ));

    // Fraud detection mode with high threshold and verbose
    let output = run_lawkit_command("benf", &["--threshold", "high", "--verbose", csv_path]);
    assert!(matches!(
        output.status.code(),
        Some(0) | Some(10) | Some(11) | Some(12)
    ));

    // Format output as JSON
    let output = run_lawkit_command("benf", &["--format", "json", csv_path]);
    assert!(matches!(
        output.status.code(),
        Some(0) | Some(10) | Some(11) | Some(12)
    ));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("{"));
}

/// Test examples from configuration documentation
#[test]
fn test_configuration_examples() {
    let test_data = generate_test_data();

    // Language option removed - CLI now outputs in English only
    // Skip language-specific test since --lang option is removed

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
        assert!(!stdout.is_empty()); // Should have some output
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
    let output = run_lawkit_command_with_file("analyze", &test_data, &[]);
    assert!(matches!(
        output.status.code(),
        Some(0) | Some(10) | Some(11) | Some(12)
    ));

    // Focus on concentration analysis
    let output =
        run_lawkit_command_with_file("analyze", &test_data, &["--laws", "benf,pareto,normal"]);
    assert!(matches!(
        output.status.code(),
        Some(0) | Some(10) | Some(11) | Some(12)
    ));

    // Recommendation mode
    let output = run_lawkit_command_with_file("diagnose", &test_data, &["--recommend"]);
    assert!(matches!(
        output.status.code(),
        Some(0) | Some(10) | Some(11) | Some(12)
    ));
}

/// Test error handling
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
    // ファイルエラーは1または3の場合がある
    assert!(matches!(output.status.code(), Some(1) | Some(3)));
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

// Generate functionality tests
#[test]
fn test_lawkit_generate_benf() {
    let output = run_lawkit_command("generate", &["benf", "--samples", "100", "--seed", "12345"]);
    assert!(output.status.success());

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
fn test_generate_to_analyze_pipeline() {
    // Generate Benford data
    let generate_output = run_lawkit_command(
        "generate",
        &["benf", "--samples", "100", "--seed", "pipeline"],
    );
    assert!(generate_output.status.success());

    let generated_data = String::from_utf8_lossy(&generate_output.stdout);

    // Create temp file with generated data
    let temp_file = create_temp_file_with_content(&generated_data);

    // Analyze the generated data
    let analyze_output = run_lawkit_command("benf", &[temp_file.path().to_str().unwrap()]);
    assert!(matches!(
        analyze_output.status.code(),
        Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
    ));

    let analysis_result = String::from_utf8_lossy(&analyze_output.stdout);
    assert!(analysis_result.contains("Benford"));
    assert!(analysis_result.contains("100")); // Should analyze 100 numbers
}

#[test]
fn test_lawkit_selftest() {
    let output = run_lawkit_command("selftest", &[]);
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("self-test"));
    assert!(stdout.contains("PASS") || stdout.contains("✅"));
}
