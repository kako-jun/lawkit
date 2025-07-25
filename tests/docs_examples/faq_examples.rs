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

/// Test case 1: cut -d',' -f2 data.csv | lawkit benf
#[test]
fn test_cut_pipe_benf() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("123\n234\n345\n111\n222\n333");
    cmd.assert().success();
    Ok(())
}

/// Test case 2: awk -F',' '{print $2}' data.csv | lawkit pareto
#[test]
fn test_awk_pipe_pareto() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto");
    cmd.write_stdin("1000\n500\n300\n200\n100\n50");
    cmd.assert().success();
    Ok(())
}

/// Test case 3: lawkit benf --threshold high data.csv
#[test]
fn test_benf_threshold_high() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--threshold")
        .arg("high");
    cmd.assert().success();
    Ok(())
}

/// Test case 4: echo "1,234.56" | lawkit benf
#[test]
fn test_english_number_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("1,234.56");
    cmd.assert().success();
    Ok(())
}

/// Test case 5: echo "１，２３４．５６" | lawkit benf
#[test]
fn test_japanese_number_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("１，２３４．５６");
    cmd.assert().success();
    Ok(())
}

/// Test case 6: echo "१,२३४.५६" | lawkit benf
#[test]
fn test_hindi_number_format() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("१,२३४.५६");
    cmd.assert().success();
    Ok(())
}

/// Test case 7: lawkit benf data.csv
#[test]
fn test_benf_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 8: echo "١٢٣٤٥٦" | lawkit benf
#[test]
fn test_arabic_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("١٢٣٤٥٦");
    cmd.assert().success();
    Ok(())
}

/// Test case 9: lawkit analyze --laws benf,pareto data.csv
#[test]
fn test_analyze_benf_pareto() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg("--laws")
        .arg("benf,pareto")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 10: lawkit analyze --laws all data.csv
#[test]
fn test_analyze_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg("--laws")
        .arg("all")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 11: lawkit analyze --laws all --recommend data.csv
#[test]
fn test_analyze_recommend() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg("--laws")
        .arg("all")
        .arg("--recommend")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 12: lawkit benf --quiet large_data.csv
#[test]
fn test_benf_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("large_data\n100000\n200000\n300000\n110000\n210000\n310000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg("--quiet")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 13: lawkit benf --threshold medium large_data.csv
#[test]
fn test_benf_threshold_medium() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("large_data\n100000\n200000\n300000\n110000\n210000\n310000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg("--threshold")
        .arg("medium")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 14: lawkit benf --format json large_data.csv
#[test]
fn test_benf_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("large_data\n100000\n200000\n300000\n110000\n210000\n310000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg("--format")
        .arg("json")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 15: lawkit pareto small_data.csv
#[test]
fn test_pareto_small_data() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("small_data\n100\n50\n30\n20\n10");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}