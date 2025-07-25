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

/// Test case 2: lawkit benf --verbose --threshold critical data.csv
#[test]
fn test_benf_verbose_threshold_critical() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg("--verbose")
        .arg("--threshold")
        .arg("critical")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 3: lawkit benf --format json data.csv
#[test]
fn test_benf_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg("--format")
        .arg("json")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 4: lawkit pareto sales_data.csv
#[test]
fn test_pareto_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("sales_data\n80000\n12000\n5000\n2000\n1000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 5: lawkit pareto --verbose --format json revenue.csv
#[test]
fn test_pareto_verbose_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("revenue\n100000\n50000\n20000\n10000\n5000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg("--verbose")
        .arg("--format")
        .arg("json")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 6: lawkit pareto --threshold high customer_values.csv
#[test]
fn test_pareto_threshold_high() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("customer_values\n10000\n5000\n3000\n2000\n1000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg("--threshold")
        .arg("high")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 7: lawkit zipf data.csv
#[test]
fn test_zipf_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1000\n500\n333\n250\n200");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 8: lawkit zipf --verbose city_populations.csv
#[test]
fn test_zipf_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("city_populations\n10000000\n5000000\n3000000\n2000000\n1000000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg("--verbose")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 9: lawkit zipf --format json data.csv
#[test]
fn test_zipf_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1000\n500\n333\n250\n200");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg("--format")
        .arg("json")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 10: lawkit normal measurements.csv
#[test]
fn test_normal_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("measurements\n1.2\n1.3\n1.1\n1.4\n1.0\n1.5");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 11: lawkit normal --verbose data.csv
#[test]
fn test_normal_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1.2\n1.3\n1.1\n1.4\n1.0\n1.5");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg("--verbose")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 12: lawkit normal --format json data.csv
#[test]
fn test_normal_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1.2\n1.3\n1.1\n1.4\n1.0\n1.5");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg("--format")
        .arg("json")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 13: lawkit normal --threshold high production_data.csv
#[test]
fn test_normal_threshold_high() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("production_data\n99.1\n99.3\n99.0\n99.4\n98.9\n99.5");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg("--threshold")
        .arg("high")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 14: lawkit poisson event_counts.csv
#[test]
fn test_poisson_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("event_counts\n1\n2\n3\n0\n1\n2");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 15: lawkit poisson --verbose data.csv
#[test]
fn test_poisson_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1\n2\n3\n0\n1\n2");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg("--verbose")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 16: lawkit poisson --format json incidents.csv
#[test]
fn test_poisson_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("incidents\n1\n2\n3\n0\n1\n2");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg("--format")
        .arg("json")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 17: lawkit poisson --threshold high defect_data.csv
#[test]
fn test_poisson_threshold_high() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("defect_data\n1\n2\n3\n0\n1\n2");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg("--threshold")
        .arg("high")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 18: lawkit benf data.csv (CSV files)
#[test]
fn test_benf_csv_format() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 19: lawkit pareto spreadsheet.xlsx
#[test]
fn test_pareto_xlsx_format() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("spreadsheet\n80000\n12000\n5000\n2000\n1000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 20: lawkit zipf data.json
#[test]
fn test_zipf_json_format() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("data\n1000\n500\n333\n250\n200");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 21: lawkit normal config.yaml
#[test]
fn test_normal_yaml_format() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("config\n1.2\n1.3\n1.1\n1.4\n1.0\n1.5");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 22: lawkit benf document.txt
#[test]
fn test_benf_txt_format() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("document\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 23: lawkit benf report.docx
#[test]
fn test_benf_docx_format() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("report\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 24: lawkit pareto presentation.pptx
#[test]
fn test_pareto_pptx_format() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("presentation\n80000\n12000\n5000\n2000\n1000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 25: echo "123,456,789" | lawkit benf
#[test]
fn test_benf_pipe_input() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("123,456,789");
    cmd.assert().success();
    Ok(())
}

/// Test case 26: lawkit pareto "100,200,300,400,500"
#[test]
fn test_pareto_command_line_string() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto");
    cmd.write_stdin("100,200,300,400,500");
    cmd.assert().success();
    Ok(())
}

/// Test case 27: cat data.txt | lawkit zipf
#[test]
fn test_zipf_stdin() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf");
    cmd.write_stdin("1000\n500\n333\n250\n200");
    cmd.assert().success();
    Ok(())
}

/// Test case 28: lawkit benf data.csv (Text output default)
#[test]
fn test_benf_text_output() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 29: lawkit benf --format json data.csv
#[test]
fn test_benf_json_output() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg("--format")
        .arg("json")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 30: lawkit pareto --format csv data.csv
#[test]
fn test_pareto_csv_output() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n80000\n12000\n5000\n2000\n1000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg("--format")
        .arg("csv")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 31: lawkit normal --format yaml data.csv
#[test]
fn test_normal_yaml_output() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1.2\n1.3\n1.1\n1.4\n1.0\n1.5");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg("--format")
        .arg("yaml")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 32: lawkit poisson --format toml data.csv
#[test]
fn test_poisson_toml_output() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n1\n2\n3\n0\n1\n2");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg("--format")
        .arg("toml")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 33: lawkit analyze --laws benf,pareto --format xml data.csv
#[test]
fn test_analyze_xml_output() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg("--laws")
        .arg("benf,pareto")
        .arg("--format")
        .arg("xml")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 34: lawkit benf --threshold low data.csv
#[test]
fn test_benf_threshold_low() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg("--threshold")
        .arg("low")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 35: lawkit benf --threshold medium data.csv
#[test]
fn test_benf_threshold_medium() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg("--threshold")
        .arg("medium")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 36: lawkit benf --threshold high data.csv
#[test]
fn test_benf_threshold_high() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg("--threshold")
        .arg("high")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 37: lawkit benf --threshold critical data.csv
#[test]
fn test_benf_threshold_critical() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg("--threshold")
        .arg("critical")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 38: lawkit benf --threshold auto data.csv
#[test]
fn test_benf_threshold_auto() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg("--threshold")
        .arg("auto")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 39: lawkit benf data.csv (English output unified)
#[test]
fn test_benf_english_output() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 40: echo "１２３４５６" | lawkit benf
#[test]
fn test_benf_japanese_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("１２３４５６");
    cmd.assert().success();
    Ok(())
}

/// Test case 41: echo "一千二百三十四" | lawkit benf
#[test]
fn test_benf_chinese_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("一千二百三十四");
    cmd.assert().success();
    Ok(())
}

/// Test case 42: echo "१२३४५६" | lawkit benf
#[test]
fn test_benf_hindi_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("१२३४५६");
    cmd.assert().success();
    Ok(())
}

/// Test case 43: echo "١٢٣٤٥٦" | lawkit benf
#[test]
fn test_benf_arabic_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf");
    cmd.write_stdin("١٢٣٤٥٦");
    cmd.assert().success();
    Ok(())
}

/// Test case 44: lawkit analyze --laws benf,pareto data.csv
#[test]
fn test_analyze_two_laws() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg("--laws")
        .arg("benf,pareto")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 45: lawkit analyze --laws all data.csv
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

/// Test case 46: lawkit analyze --laws benf,pareto,normal --verbose --recommend data.csv
#[test]
fn test_analyze_verbose_recommend() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg("--laws")
        .arg("benf,pareto,normal")
        .arg("--verbose")
        .arg("--recommend")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 47: lawkit analyze --laws all --focus fraud-detection data.csv
#[test]
fn test_analyze_focus_fraud_detection() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg("--laws")
        .arg("all")
        .arg("--focus")
        .arg("fraud-detection")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 48: lawkit analyze --laws all --purpose quality-assessment data.csv
#[test]
fn test_analyze_purpose_quality_assessment() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg("--laws")
        .arg("all")
        .arg("--purpose")
        .arg("quality-assessment")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 49: lawkit analyze --laws all --format json data.csv
#[test]
fn test_analyze_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg("--laws")
        .arg("all")
        .arg("--format")
        .arg("json")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 50: lawkit generate --samples 1000 | lawkit benf
#[test]
fn test_generate_pipe_benf() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("--samples")
        .arg("1000");
    
    let output = cmd.output()?;
    assert!(output.status.success());
    
    let mut cmd2 = lawkit_cmd();
    cmd2.arg("benf");
    cmd2.write_stdin(std::str::from_utf8(&output.stdout)?);
    cmd2.assert().success();
    Ok(())
}

/// Test case 51: lawkit generate --samples 500 > test_data.csv (simulated)
#[test]
fn test_generate_samples() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("--samples")
        .arg("500");
    cmd.assert().success();
    Ok(())
}

/// Test case 52: lawkit generate --samples 100 | lawkit pareto
#[test]
fn test_generate_pipe_pareto() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("--samples")
        .arg("100");
    
    let output = cmd.output()?;
    assert!(output.status.success());
    
    let mut cmd2 = lawkit_cmd();
    cmd2.arg("pareto");
    cmd2.write_stdin(std::str::from_utf8(&output.stdout)?);
    cmd2.assert().success();
    Ok(())
}

/// Test case 53: lawkit validate --laws all data.csv
#[test]
fn test_validate_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("validate")
        .arg("--laws")
        .arg("all")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 54: lawkit validate --laws benf,pareto --focus fraud-detection data.csv
#[test]
fn test_validate_focus_fraud_detection() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("validate")
        .arg("--laws")
        .arg("benf,pareto")
        .arg("--focus")
        .arg("fraud-detection")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 55: lawkit validate --laws all --recommend data.csv
#[test]
fn test_validate_recommend() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("validate")
        .arg("--laws")
        .arg("all")
        .arg("--recommend")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 56: lawkit diagnose --laws all data.csv
#[test]
fn test_diagnose_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("diagnose")
        .arg("--laws")
        .arg("all")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 57: lawkit diagnose --laws all --purpose quality-assessment data.csv
#[test]
fn test_diagnose_purpose_quality_assessment() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("diagnose")
        .arg("--laws")
        .arg("all")
        .arg("--purpose")
        .arg("quality-assessment")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 58: lawkit diagnose --laws all --verbose data.csv
#[test]
fn test_diagnose_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("diagnose")
        .arg("--laws")
        .arg("all")
        .arg("--verbose")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 59: lawkit selftest
#[test]
fn test_selftest() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("selftest");
    cmd.assert().success();
    Ok(())
}

/// Test case 60: lawkit selftest --verbose
#[test]
fn test_selftest_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("selftest")
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 61: lawkit benf --threshold high transactions.csv
#[test]
fn test_benf_fraud_detection() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("transactions\n1234\n2345\n3456\n1111\n2222\n3333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg("--threshold")
        .arg("high")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 62: lawkit pareto --verbose --format json daily_volumes.csv
#[test]
fn test_pareto_daily_volumes() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("daily_volumes\n100000\n50000\n30000\n20000\n10000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg("--verbose")
        .arg("--format")
        .arg("json")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 63: lawkit analyze --laws benf,pareto --focus fraud-detection financial_data.csv
#[test]
fn test_analyze_financial_fraud() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("financial_data\n1234\n2345\n3456\n1111\n2222\n3333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg("--laws")
        .arg("benf,pareto")
        .arg("--focus")
        .arg("fraud-detection")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 64: lawkit normal --threshold high measurements.csv
#[test]
fn test_normal_quality_control() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("measurements\n99.1\n99.3\n99.0\n99.4\n98.9\n99.5");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg("--threshold")
        .arg("high")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 65: lawkit validate --laws normal,poisson --purpose quality-control production_data.csv
#[test]
fn test_validate_quality_control() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("production_data\n99.1\n99.3\n99.0\n99.4\n98.9\n99.5");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("validate")
        .arg("--laws")
        .arg("normal,poisson")
        .arg("--purpose")
        .arg("quality-control")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 66: lawkit poisson --verbose defect_counts.csv
#[test]
fn test_poisson_defect_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("defect_counts\n1\n2\n0\n1\n3\n0");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg("--verbose")
        .arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}