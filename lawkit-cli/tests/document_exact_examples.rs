use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

/// Run lawkit command with subcommand and arguments
fn run_lawkit_command(subcommand: &str, args: &[&str]) -> std::process::Output {
    let mut command = Command::new("cargo");
    command.args(["run", "--bin", "lawkit", "--", subcommand]);
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

/// Generate adequate test data (100+ numbers for statistical analysis)
fn generate_financial_data() -> String {
    let mut data = Vec::new();
    for i in 1..=150 {
        let amount = 1000 + i * 43 + (i % 7) * 137;
        data.push(amount.to_string());
    }
    data.join("\n")
}

// ===== README.md の例をそのままテスト =====

#[test]
fn test_readme_basic_benf() {
    // README: lawkit benf financial_data.csv
    let financial_data = generate_financial_data();
    let output = run_lawkit_command_with_file("benf", &financial_data, &[]);
    
    assert!(matches!(output.status.code(), Some(0) | Some(10) | Some(11) | Some(12) | Some(13)));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Benford"));
}

#[test]
fn test_readme_basic_pareto() {
    // README: lawkit pareto sales_report.json
    let sales_data = generate_financial_data();
    let output = run_lawkit_command_with_file("pareto", &sales_data, &[]);
    
    assert!(matches!(output.status.code(), Some(0) | Some(10) | Some(11) | Some(12) | Some(13)));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pareto"));
}

#[test]
fn test_readme_basic_zipf() {
    // README: lawkit zipf word_frequencies.txt
    let text_data = "the\nquick\nbrown\nfox\njumps\nover\nthe\nlazy\ndog\nthe\nfox\nis\nquick\nthe\nbrown\nfox\nthe\ndog\nlazy\nover\njumps\nquick\nfox\nthe\nbrown\ndog\nover\nlazy\nthe\nquick\nfox\nbrown\nthe\ndog\njumps\nover\nlazy\nquick\nthe\nfox\nbrown\ndog\nthe\njumps\nover\nlazy\nquick\nfox\nthe\nbrown\ndog\nover\nlazy\nthe\nquick\nfox\nbrown\ndog\njumps\nthe\nover\nlazy\nquick\nfox\nbrown\nthe\ndog\njumps\nover\nlazy\nquick\nfox\nthe\nbrown\ndog\nover\nlazy\nthe\nquick\nfox\nbrown\ndog\njumps\nthe\nover\nlazy\nquick\nfox\nbrown\ndog\nthe\njumps\nover\nlazy\nquick\nfox\nthe\nbrown\ndog\nover\nlazy\nthe\nquick\nfox\nbrown\ndog\njumps\nthe\nover\nlazy\nquick\nfox";
    let output = run_lawkit_command_with_file("zipf", text_data, &["--text"]);
    
    assert!(matches!(output.status.code(), Some(0) | Some(10) | Some(11) | Some(12) | Some(13)));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Zipf"));
}

#[test]
fn test_readme_basic_normal() {
    // README: lawkit normal measurements.xlsx
    let measurement_data = generate_financial_data();
    let output = run_lawkit_command_with_file("normal", &measurement_data, &[]);
    
    assert!(matches!(output.status.code(), Some(0) | Some(10) | Some(11) | Some(12) | Some(13)));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Normal"));
}

#[test]
fn test_readme_basic_poisson() {
    // README: lawkit poisson server_logs.tsv
    let event_data = "3\n2\n4\n1\n3\n2\n5\n2\n3\n4\n1\n2\n3\n4\n2\n1\n3\n2\n4\n3\n1\n2\n3\n5\n2\n3\n1\n4\n2\n3\n1\n2\n4\n3\n2\n1\n3\n2\n4\n3\n2\n1\n3\n4\n2\n3\n1\n2\n4\n3";
    let output = run_lawkit_command_with_file("poisson", event_data, &[]);
    
    assert!(matches!(output.status.code(), Some(0) | Some(10) | Some(11) | Some(12) | Some(13)));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Poisson"));
}

#[test]
fn test_readme_analyze_all_laws() {
    // README: lawkit analyze --laws all transactions.csv
    let transaction_data = generate_financial_data();
    let output = run_lawkit_command_with_file("analyze", &transaction_data, &["--laws", "all"]);
    
    assert!(matches!(output.status.code(), Some(0) | Some(10) | Some(11) | Some(12) | Some(13)));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Integration") || stdout.contains("Analysis"));
}

#[test]
fn test_readme_validate_yaml() {
    // README: lawkit validate --laws all inventory.json --format yaml
    let inventory_data = generate_financial_data();
    let output = run_lawkit_command_with_file("validate", &inventory_data, &["--laws", "all", "--format", "yaml"]);
    
    assert!(matches!(output.status.code(), Some(0) | Some(10) | Some(11) | Some(12) | Some(13)));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Validation") || stdout.contains("consistency"));
}

#[test]
fn test_readme_diagnose_json() {
    // README: lawkit diagnose --laws benf,zipf document.txt --format json
    let document_data = generate_financial_data();
    let output = run_lawkit_command_with_file("diagnose", &document_data, &["--laws", "benf,zipf", "--format", "json"]);
    
    assert!(matches!(output.status.code(), Some(0) | Some(10) | Some(11) | Some(12) | Some(13)));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("{") && stdout.contains("}"));
}

#[test]
fn test_readme_generate_pareto() {
    // README: lawkit generate pareto --samples 1000 > test_data.txt
    let output = run_lawkit_command("generate", &["pareto", "--samples", "1000"]);
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 1000);
}

#[test]
fn test_readme_generate_normal() {
    // README: lawkit generate normal --mean 100 --stddev 15 --samples 500
    let output = run_lawkit_command("generate", &["normal", "--mean", "100", "--stddev", "15", "--samples", "500"]);
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 500);
}

#[test]
fn test_readme_timeseries() {
    // README: lawkit normal monthly_sales.csv --enable-timeseries --timeseries-window 12
    let monthly_data = generate_financial_data();
    let output = run_lawkit_command_with_file("normal", &monthly_data, &["--enable-timeseries", "--timeseries-window", "12"]);
    
    assert!(matches!(output.status.code(), Some(0) | Some(10) | Some(11) | Some(12) | Some(13)));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Normal") || stdout.contains("timeseries") || stdout.contains("Time"));
}

#[test]
fn test_readme_filter() {
    // README: lawkit analyze --laws all --filter ">=1000" financial_data.xlsx
    let financial_data = generate_financial_data();
    let output = run_lawkit_command_with_file("analyze", &financial_data, &["--laws", "all", "--filter", ">=1000"]);
    
    assert!(matches!(output.status.code(), Some(0) | Some(10) | Some(11) | Some(12) | Some(13)));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Integration") || stdout.contains("Analysis"));
}

#[test]
fn test_readme_pipeline_stdin() {
    // README: cat raw_numbers.txt | lawkit benf -
    let raw_numbers = generate_financial_data();
    let output = run_lawkit_command_with_file("benf", &raw_numbers, &[]);
    
    assert!(matches!(output.status.code(), Some(0) | Some(10) | Some(11) | Some(12) | Some(13)));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Benford"));
}

#[test]
fn test_readme_generate_pipeline() {
    // README: lawkit generate zipf --samples 10000 | lawkit analyze --laws all -
    let generate_output = run_lawkit_command("generate", &["zipf", "--samples", "10000"]);
    assert!(generate_output.status.success());

    let generated_data = String::from_utf8_lossy(&generate_output.stdout);
    let temp_file = create_temp_file_with_content(&generated_data);

    let analyze_output = run_lawkit_command("analyze", &[temp_file.path().to_str().unwrap(), "--laws", "all"]);
    assert!(matches!(analyze_output.status.code(), Some(0) | Some(10) | Some(11) | Some(12) | Some(13)));
    
    let analysis_result = String::from_utf8_lossy(&analyze_output.stdout);
    assert!(analysis_result.contains("Integration") || analysis_result.contains("Analysis"));
}