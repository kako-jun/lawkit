use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;

fn lawkit_cmd() -> Command {
    Command::cargo_bin("lawkit").expect("Failed to find lawkit binary")
}

fn create_temp_csv(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    write!(file, "{}", content).expect("Failed to write to temp file");
    file
}

#[test]
fn test_benf_data_csv() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 1: Benford law analysis from index.md
    let csv_content = "value\n1234\n5678\n9012\n3456\n7890\n2345\n6789\n1023\n4567\n8901";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Benford"));

    Ok(())
}

#[test]
fn test_pareto_sales_csv() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 2: Pareto analysis from index.md
    let csv_content = "sales\n1000\n2000\n3000\n4000\n5000\n6000\n7000\n8000\n9000\n10000";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("pareto").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Pareto"));

    Ok(())
}

#[test]
fn test_analyze_multi_laws() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 3: Multi-law comparison from index.md
    let csv_content = "data\n123\n456\n789\n1011\n1213\n1415\n1617\n1819\n2021\n2223";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benf,pareto,normal");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("analyzed"));

    Ok(())
}

#[test]
fn test_analyze_data_csv() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 4: Basic analyze command from index.md
    let csv_content = "numbers\n100\n200\n300\n400\n500\n600\n700\n800\n900\n1000";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze").arg(temp_file.path());
    cmd.assert().success();

    Ok(())
}
