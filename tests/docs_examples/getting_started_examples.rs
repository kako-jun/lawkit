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

/// Test case 1: lawkit benf data.csv
#[test]
fn test_benf_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 2: lawkit benf data.csv --verbose
#[test]
fn test_benf_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 3: lawkit benf data.csv --format json
#[test]
fn test_benf_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 4: lawkit benf data.csv --threshold high
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

/// Test case 5: lawkit benf audit_data.csv --confidence 0.99 --verbose
#[test]
fn test_benf_confidence_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("audit_data\n1234\n2345\n3456\n1111\n2222\n3333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--confidence")
        .arg("0.99")
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 6: lawkit benf large_data.csv --sample-size 10000 --optimize
#[test]
fn test_benf_sample_size_optimize() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("large_data\n100000\n200000\n300000\n110000\n210000\n310000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--sample-size")
        .arg("10000");
    cmd.assert().success();
    Ok(())
}

/// Test case 7: lawkit benf financial_data.csv --min-value 100
#[test]
fn test_benf_min_value() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("financial_data\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--min-value")
        .arg("100");
    cmd.assert().success();
    Ok(())
}

/// Test case 8: lawkit pareto sales.csv
#[test]
fn test_pareto_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("sales\n80000\n12000\n5000\n2000\n1000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 9: lawkit pareto sales.csv --concentration 0.9
#[test]
fn test_pareto_concentration() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("sales\n80000\n12000\n5000\n2000\n1000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--concentration")
        .arg("0.9");
    cmd.assert().success();
    Ok(())
}

/// Test case 10: lawkit pareto sales.csv --gini-coefficient
#[test]
fn test_pareto_gini_coefficient() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("sales\n80000\n12000\n5000\n2000\n1000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--gini-coefficient");
    cmd.assert().success();
    Ok(())
}

/// Test case 11: lawkit zipf document.txt
#[test]
fn test_zipf_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("text\nthe quick brown fox\njumps over the lazy dog");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 12: lawkit zipf japanese_text.txt
#[test]
fn test_zipf_japanese() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("japanese_text\n日本語\nテキスト\n分析");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 13: lawkit zipf text.txt --min-count 5
#[test]
fn test_zipf_min_count() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("text\nthe quick brown fox\njumps over the lazy dog");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(temp_file.path())
        .arg("--min-count")
        .arg("5");
    cmd.assert().success();
    Ok(())
}

/// Test case 14: lawkit normal measurements.csv
#[test]
fn test_normal_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("measurements\n1.2\n1.3\n1.1\n1.4\n1.0\n1.5");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 15: lawkit normal quality_data.csv --verbose
#[test]
fn test_normal_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("quality_data\n1.2\n1.3\n1.1\n1.4\n1.0\n1.5");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 16: lawkit normal process_data.csv --outliers
#[test]
fn test_normal_outliers() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("process_data\n1.2\n1.3\n1.1\n1.4\n1.0\n1.5");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(temp_file.path())
        .arg("--outliers");
    cmd.assert().success();
    Ok(())
}

/// Test case 17: lawkit poisson events.csv
#[test]
fn test_poisson_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("events\n1\n2\n3\n0\n1\n2");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 18: lawkit poisson events.csv --verbose
#[test]
fn test_poisson_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("events\n1\n2\n3\n0\n1\n2");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 19: lawkit poisson critical_events.csv --confidence 0.99 --verbose
#[test]
fn test_poisson_confidence_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("critical_events\n1\n2\n3\n0\n1\n2");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(temp_file.path())
        .arg("--confidence")
        .arg("0.99")
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 20: lawkit analyze data.csv --laws benf,pareto,normal
#[test]
fn test_analyze_multi_laws() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benf,pareto,normal");
    cmd.assert().success();
    Ok(())
}

/// Test case 21: lawkit validate data.csv --laws all
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

/// Test case 22: lawkit diagnose data.csv --focus conflict
#[test]
fn test_diagnose_focus_conflict() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("diagnose")
        .arg(temp_file.path())
        .arg("--focus")
        .arg("conflict");
    cmd.assert().success();
    Ok(())
}

/// Test case 23: lawkit benf accounting.csv
#[test]
fn test_benf_accounting() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("TransactionID,Amount,Date\n1,1234,2024-01-01\n2,2345,2024-01-02\n3,3456,2024-01-03");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 24: lawkit pareto sales.csv --threshold 0.8
#[test]
fn test_pareto_threshold() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("Product,Sales\nProduct A,80000\nProduct B,12000\nProduct C,5000\nProduct D,2000\nProduct E,1000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--threshold")
        .arg("0.8");
    cmd.assert().success();
    Ok(())
}

/// Test case 25: lawkit benf data.csv (text format default)
#[test]
fn test_benf_text_format() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 26: lawkit benf data.csv --format csv
#[test]
fn test_benf_format_csv() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("csv");
    cmd.assert().success();
    Ok(())
}

/// Test case 27: lawkit benf data.csv --format yaml
#[test]
fn test_benf_format_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("yaml");
    cmd.assert().success();
    Ok(())
}

/// Test case 28: lawkit benf data.csv --format xml
#[test]
fn test_benf_format_xml() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("xml");
    cmd.assert().success();
    Ok(())
}

/// Test case 29: echo "１２３４５６ ７８９０" | lawkit benf
#[test]
fn test_benf_japanese_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("１２３４５６ ７８９０");
    cmd.assert().success();
    Ok(())
}

/// Test case 30: echo "一千二百三十四" | lawkit benf
#[test]
fn test_benf_chinese_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("一千二百三十四");
    cmd.assert().success();
    Ok(())
}

/// Test case 31: echo "壹萬貳仟參佰肆拾伍" | lawkit benf
#[test]
fn test_benf_traditional_chinese() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("壹萬貳仟參佰肆拾伍");
    cmd.assert().success();
    Ok(())
}

/// Test case 32: echo "五万六千七百八十九" | lawkit benf
#[test]
fn test_benf_japanese_kanji() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("五万六千七百八十九");
    cmd.assert().success();
    Ok(())
}

/// Test case 33: lawkit benf data.csv --filter ">=1000"
#[test]
fn test_benf_filter() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1000\n2000\n3000\n1100\n2200\n3300");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--filter")
        .arg(">=1000");
    cmd.assert().success();
    Ok(())
}

/// Test case 34: lawkit pareto sales.csv --concentration 0.95
#[test]
fn test_pareto_concentration_95() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("sales\n80000\n12000\n5000\n2000\n1000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--concentration")
        .arg("0.95");
    cmd.assert().success();
    Ok(())
}