#[allow(unused_imports)]
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

// Helper function to get the lawkit command
fn lawkit_cmd() -> Command {
    Command::cargo_bin("lawkit").expect("Failed to find lawkit binary")
}

// Helper function to create temporary CSV files for testing
fn create_temp_csv(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(file, "{}", content).expect("Failed to write to temp file");
    file
}

/// Test case 1: lawkit analyze "$file" --laws benford,normal --format json
#[test]
fn test_ci_cd_analyze_with_laws_and_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1\n2\n3\n11\n12\n13\n21\n22\n31");

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford,normal")
        .arg("--format")
        .arg("json");

    cmd.assert().success();
    Ok(())
}

/// Test case 2: lawkit analyze data/financial.csv --laws all --format json
#[test]
fn test_gitlab_analyze_all_laws_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("amount\n1000\n2000\n3000\n1100\n1200\n1300\n2100\n2200\n3100");

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("all")
        .arg("--format")
        .arg("json");

    cmd.assert().success();
    Ok(())
}

/// Test case 3: lawkit benf data_file --format json (Python API example)
#[test]
fn test_python_api_benford_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("digit\n1\n2\n3\n11\n12\n13\n21\n22");

    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");

    cmd.assert().success();
    Ok(())
}

/// Test case 4: lawkit benf dataFile --format json (Node.js example)
#[test]
fn test_nodejs_benford_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("sales\n1234\n2345\n3456\n1111\n1222\n1333\n2111");

    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");

    cmd.assert().success();
    Ok(())
}

/// Test case 5: lawkit benf /tmp/data.csv --format json (PostgreSQL example)
#[test]
fn test_postgresql_benford_csv_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("transaction_amount\n1500\n2600\n3700\n1800\n1900\n2000");

    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");

    cmd.assert().success();
    Ok(())
}

/// Test case 6: lawkit analyze data_source --laws all --format json (Tableau example)
#[test]
fn test_tableau_analyze_all_laws_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file =
        create_temp_csv("business_value\n1000\n2000\n3000\n1100\n1200\n2100\n3100\n4000");

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("all")
        .arg("--format")
        .arg("json");

    cmd.assert().success();
    Ok(())
}

/// Test case 7: lawkit benf temp_file --format json (Power BI example)
#[test]
fn test_powerbi_benford_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("revenue\n15000\n26000\n37000\n18000\n19000\n20000");

    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");

    cmd.assert().success();
    Ok(())
}

/// Test case 8: lawkit benf /tmp/data.csv --format json (AWS Lambda example)
#[test]
fn test_aws_lambda_benford_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("financial_data\n12000\n23000\n34000\n11000\n12500\n21000");

    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");

    cmd.assert().success();
    Ok(())
}

/// Test case 9: lawkit analyze /tmp/data.csv --laws all --format json (GCP example)
#[test]
fn test_gcp_analyze_all_laws_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("cloud_data\n10000\n20000\n30000\n11000\n12000\n21000\n31000");

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("all")
        .arg("--format")
        .arg("json");

    cmd.assert().success();
    Ok(())
}

/// Test case 10: lawkit benf data_file --format json (Prometheus example)
#[test]
fn test_prometheus_benford_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("metric_value\n1500\n2600\n3700\n1800\n1900\n2000");

    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");

    cmd.assert().success();
    Ok(())
}

/// Test case 11: lawkit benf file_path --format json (Rust custom example)
#[test]
fn test_rust_custom_benford_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("custom_data\n1234\n2345\n3456\n1567\n1678\n2789");

    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");

    cmd.assert().success();
    Ok(())
}
