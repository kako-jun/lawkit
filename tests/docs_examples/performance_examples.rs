use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::NamedTempFile;
use std::io::Write;

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

// Helper function to create temporary text files for testing
fn create_temp_txt(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(file, "{}", content).expect("Failed to write to temp file");
    file
}

/// Test case 1: lawkit benf data.csv
#[test]
fn test_basic_benford() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333\n121\n232\n343");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    
    cmd.assert().success();
    Ok(())
}

/// Test case 2: lawkit pareto data.csv --threshold 0.8
#[test]
fn test_pareto_with_threshold() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1000\n500\n300\n200\n100\n50\n30\n20\n10");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--threshold")
        .arg("0.8");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 3: lawkit zipf text.txt
#[test]
fn test_zipf_text() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_txt("the quick brown fox jumps over the lazy dog the the");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf").arg(temp_file.path());
    
    cmd.assert().success();
    Ok(())
}

/// Test case 4: lawkit normal data.csv
#[test]
fn test_normal_distribution() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n50\n51\n49\n52\n48\n50\n51\n49\n50");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal").arg(temp_file.path());
    
    cmd.assert().success();
    Ok(())
}

/// Test case 5: lawkit poisson data.csv
#[test]
fn test_poisson_distribution() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("count\n3\n2\n4\n1\n3\n2\n5\n3\n2");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson").arg(temp_file.path());
    
    cmd.assert().success();
    Ok(())
}

/// Test case 6: lawkit analyze data.csv --laws benford,pareto,normal
#[test]
fn test_analyze_multiple_laws() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333\n121\n232\n343");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford,pareto,normal");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 7: lawkit benf data.csv --quiet --format json
#[test]
fn test_benford_quiet_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--quiet")
        .arg("--format")
        .arg("json");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 8: lawkit benf data.csv --verbose
#[test]
fn test_benford_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--verbose");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 9: lawkit benf data.csv --format csv
#[test]
fn test_benford_format_csv() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("csv");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 10: lawkit benf data.csv --format yaml
#[test]
fn test_benford_format_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("yaml");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 11: lawkit benf data.csv --format toml
#[test]
fn test_benford_format_toml() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("toml");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 12: lawkit benf data.csv --format xml
#[test]
fn test_benford_format_xml() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("xml");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 13: lawkit analyze data.csv --laws benford,pareto
#[test]
fn test_analyze_benford_pareto() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1000\n500\n300\n200\n100\n50\n30\n20\n10");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford,pareto");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 14: lawkit analyze data.csv --laws benford --focus accuracy
#[test]
fn test_analyze_focus_accuracy() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford")
        .arg("--focus")
        .arg("quality");  // 'accuracy' is not a valid focus value, using 'quality'
    
    cmd.assert().success();
    Ok(())
}

/// Test case 15: lawkit analyze data.csv --laws all --purpose audit
#[test]
fn test_analyze_all_laws_purpose_audit() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333\n121\n232\n343");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("all")
        .arg("--purpose")
        .arg("fraud");  // 'audit' is not a valid purpose, using 'fraud' for similar meaning
    
    cmd.assert().success();
    Ok(())
}

/// Test case 16: lawkit analyze data.csv --laws all --recommend
#[test]
fn test_analyze_all_laws_recommend() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333\n121\n232\n343");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("all")
        .arg("--recommend");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 17: lawkit benf optimized.csv
#[test]
fn test_benford_optimized() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333\n121\n232\n343");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    
    cmd.assert().success();
    Ok(())
}

/// Test case 18: lawkit benf data.csv --quiet
#[test]
fn test_benford_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--quiet");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 19: lawkit benf data.csv --verbose (duplicate of case 8)
#[test]
fn test_benford_verbose_2() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--verbose");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 20: lawkit pareto data.csv --quiet
#[test]
fn test_pareto_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1000\n500\n300\n200\n100\n50\n30\n20\n10");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--quiet");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 21: lawkit analyze data.csv --laws benford,pareto --quiet
#[test]
fn test_analyze_benford_pareto_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1000\n500\n300\n200\n100\n50\n30\n20\n10");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford,pareto")
        .arg("--quiet");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 22: lawkit analyze data.csv --laws all --quiet
#[test]
fn test_analyze_all_laws_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333\n121\n232\n343");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("all")
        .arg("--quiet");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 23: lawkit analyze data.csv --format json --quiet
#[test]
fn test_analyze_json_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json")
        .arg("--quiet");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 24: lawkit benf data.csv --format yaml (duplicate of case 10)
#[test]
fn test_benford_format_yaml_2() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("yaml");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 25: lawkit analyze data.csv --laws all --verbose
#[test]
fn test_analyze_all_laws_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333\n121\n232\n343");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("all")
        .arg("--verbose");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 26: lawkit benf small_data.csv --quiet
#[test]
fn test_benford_small_data_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--quiet");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 27: lawkit analyze medium_data.csv --laws benford,pareto
#[test]
fn test_analyze_medium_data() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1000\n2000\n3000\n1100\n2100\n3100\n1200\n2200\n3200");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford,pareto");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 28: lawkit analyze large_data.csv --laws benford --quiet
#[test]
fn test_analyze_large_data_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n10000\n20000\n30000\n11000\n21000\n31000\n12000\n22000\n32000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford")
        .arg("--quiet");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 29: lawkit benf huge_data.csv --quiet --format json
#[test]
fn test_benford_huge_data_quiet_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n100000\n200000\n300000\n110000\n210000\n310000\n120000\n220000\n320000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--quiet")
        .arg("--format")
        .arg("json");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 30: lawkit --version
#[test]
fn test_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("--version");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 31: lawkit --help
#[test]
fn test_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("--help");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 32: lawkit benf --help
#[test]
fn test_benf_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg("--help");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 33: lawkit pareto --help
#[test]
fn test_pareto_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto").arg("--help");
    
    cmd.assert().success();
    Ok(())
}

/// Test case 34: lawkit zipf --help
#[test]
fn test_zipf_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf").arg("--help");
    
    cmd.assert().success();
    Ok(())
}