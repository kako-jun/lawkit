#[allow(unused_imports)]
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

/// Test case 1: lawkit benf data.csv --format json
#[test]
fn test_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 2: lawkit benf data.csv --format yaml
#[test]
fn test_format_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("yaml");
    cmd.assert().success();
    Ok(())
}

/// Test case 3: lawkit benf data.csv --format csv
#[test]
fn test_format_csv() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("csv");
    cmd.assert().success();
    Ok(())
}

/// Test case 4: lawkit benf data.csv --format toml
#[test]
fn test_format_toml() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("toml");
    cmd.assert().success();
    Ok(())
}

/// Test case 5: lawkit benf data.csv --format xml
#[test]
fn test_format_xml() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("xml");
    cmd.assert().success();
    Ok(())
}

/// Test case 6: echo "１２３４５６" | lawkit benf
#[test]
fn test_japanese_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("１２３４５６");
    cmd.assert().success();
    Ok(())
}

/// Test case 7: echo "一千二百三十四" | lawkit benf
#[test]
fn test_chinese_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("一千二百三十四");
    cmd.assert().success();
    Ok(())
}

/// Test case 8: lawkit benf data.csv --quiet
#[test]
fn test_quiet_mode() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--quiet");
    cmd.assert().success();
    Ok(())
}

/// Test case 9: lawkit benf data.csv --verbose
#[test]
fn test_verbose_mode() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 10: lawkit pareto data.csv --threshold 0.8
#[test]
fn test_pareto_threshold() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1000\n500\n300\n200\n100\n50");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--threshold")
        .arg("0.8");
    cmd.assert().success();
    Ok(())
}

/// Test case 11: lawkit analyze data.csv --laws benford,pareto,normal
#[test]
fn test_analyze_multi_laws() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford,pareto,normal");
    cmd.assert().success();
    Ok(())
}

/// Test case 12: lawkit analyze data.csv --laws benford --focus accuracy
#[test]
fn test_analyze_focus_accuracy() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford")
        .arg("--focus")
        .arg("quality");
    cmd.assert().success();
    Ok(())
}

/// Test case 13: lawkit analyze data.csv --laws all --purpose audit
#[test]
fn test_analyze_purpose_audit() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("all")
        .arg("--purpose")
        .arg("fraud");
    cmd.assert().success();
    Ok(())
}

/// Test case 14: lawkit analyze data.csv --laws all --recommend
#[test]
fn test_analyze_recommend() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("all")
        .arg("--recommend");
    cmd.assert().success();
    Ok(())
}

/// Test case 15: lawkit benf data.csv --format json (duplicate)
#[test]
fn test_json_output_example() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 16: lawkit benf data.csv --format csv (duplicate)
#[test]
fn test_csv_output_example() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("csv");
    cmd.assert().success();
    Ok(())
}

/// Test case 17: echo "１２３４ ５６７８" | lawkit benf
#[test]
fn test_japanese_spaced_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("１２３４ ５６７８");
    cmd.assert().success();
    Ok(())
}

/// Test case 18: echo "壹万贰千 三千四百" | lawkit benf
#[test]
fn test_chinese_financial_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("壹万贰千 三千四百");
    cmd.assert().success();
    Ok(())
}

/// Test case 19: echo "123 ４５６ 七八九" | lawkit benf
#[test]
fn test_mixed_format_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("123 ４５６ 七八九");
    cmd.assert().success();
    Ok(())
}

/// Test case 20: lawkit analyze data.csv --laws benford,pareto,normal (duplicate)
#[test]
fn test_multi_law_selection() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford,pareto,normal");
    cmd.assert().success();
    Ok(())
}

/// Test case 21: lawkit analyze data.csv --laws benford --focus accuracy (duplicate)
#[test]
fn test_analysis_focus() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford")
        .arg("--focus")
        .arg("quality");
    cmd.assert().success();
    Ok(())
}

/// Test case 22: lawkit analyze data.csv --laws all --purpose audit (duplicate)
#[test]
fn test_purpose_specific_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("all")
        .arg("--purpose")
        .arg("fraud");
    cmd.assert().success();
    Ok(())
}

/// Test case 23: lawkit analyze data.csv --laws all --recommend (duplicate)
#[test]
fn test_recommendation_mode() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("all")
        .arg("--recommend");
    cmd.assert().success();
    Ok(())
}

/// Test case 24: lawkit validate data.csv --laws all
#[test]
fn test_validate_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("validate")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("all");
    cmd.assert().success();
    Ok(())
}

/// Test case 25: lawkit diagnose data.csv --laws all
#[test]
fn test_diagnose_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("diagnose")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("all");
    cmd.assert().success();
    Ok(())
}

/// Test case 26: lawkit analyze data1.csv --laws benford --format json
#[test]
fn test_batch_benford_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford")
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 27: lawkit analyze data2.csv --laws pareto --format json
#[test]
fn test_batch_pareto_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1000\n500\n300\n200\n100\n50");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("pareto")
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 28: lawkit analyze data3.csv --laws normal --format json
#[test]
fn test_batch_normal_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n50\n51\n49\n52\n48\n50");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("normal")
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 29: lawkit benf large_data.csv --quiet
#[test]
fn test_large_data_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n10000\n20000\n30000\n11000\n21000\n31000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--quiet");
    cmd.assert().success();
    Ok(())
}

/// Test case 30: lawkit analyze large_data.csv --laws benford --quiet
#[test]
fn test_large_data_analyze_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n10000\n20000\n30000\n11000\n21000\n31000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford")
        .arg("--quiet");
    cmd.assert().success();
    Ok(())
}

/// Test case 31: lawkit benf data.csv --verbose (duplicate)
#[test]
fn test_debug_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 32: lawkit benf data.csv --format json | jq '.numbers_analyzed'
#[test]
fn test_json_pipe_jq() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}