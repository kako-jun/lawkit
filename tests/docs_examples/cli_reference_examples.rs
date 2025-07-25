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

/// Test case 1: lawkit --help
#[test]
fn test_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("--help");
    cmd.assert().success();
    Ok(())
}

/// Test case 2: lawkit --version
#[test]
fn test_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("--version");
    cmd.assert().success();
    Ok(())
}

/// Test case 3: lawkit list
#[test]
fn test_list() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("list");
    cmd.assert().success();
    Ok(())
}

/// Test case 4: lawkit benf data.csv
#[test]
fn test_benf_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 5: lawkit benf transactions.json --verbose --format json
#[test]
fn test_benf_verbose_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("amount\n1234\n2345\n3456\n1111\n2222\n3333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--verbose")
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 6: lawkit benf data.csv --quiet
#[test]
fn test_benf_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--quiet");
    cmd.assert().success();
    Ok(())
}

/// Test case 7: lawkit benf accounts.csv --filter ">=1000" --threshold high
#[test]
fn test_benf_filter_threshold() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("amount\n1000\n2000\n3000\n1100\n2100\n3100");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--filter")
        .arg(">=1000")
        .arg("--threshold")
        .arg("high");
    cmd.assert().success();
    Ok(())
}

/// Test case 8: lawkit benf audit_data.csv --confidence 0.99 --verbose
#[test]
fn test_benf_confidence_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("audit_value\n1234\n2345\n3456\n1111\n2222\n3333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--confidence")
        .arg("0.99")
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 9: lawkit benf big_data.csv --sample-size 50000
#[test]
fn test_benf_sample_size() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("big_value\n10000\n20000\n30000\n11000\n21000\n31000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--sample-size")
        .arg("50000");
    cmd.assert().success();
    Ok(())
}

/// Test case 10: lawkit benf financial_data.csv --min-value 100
#[test]
fn test_benf_min_value() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("financial\n100\n200\n300\n150\n250\n350");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--min-value")
        .arg("100");
    cmd.assert().success();
    Ok(())
}

/// Test case 11: echo "123 456 789" | lawkit benf --verbose
#[test]
fn test_benf_stdin_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg("--verbose");
    cmd.write_stdin("123 456 789");
    cmd.assert().success();
    Ok(())
}

/// Test case 12: lawkit pareto sales.csv
#[test]
fn test_pareto_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("sales\n1000\n500\n300\n200\n100\n50");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 13: lawkit pareto data.csv --concentration 0.9
#[test]
fn test_pareto_concentration() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("data\n1000\n500\n300\n200\n100\n50");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--concentration")
        .arg("0.9");
    cmd.assert().success();
    Ok(())
}

/// Test case 14: lawkit pareto customers.csv --business-analysis --gini-coefficient
#[test]
fn test_pareto_business_gini() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("customers\n1000\n800\n600\n400\n200\n100");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--business-analysis")
        .arg("--gini-coefficient");
    cmd.assert().success();
    Ok(())
}

/// Test case 15: lawkit pareto revenue.csv --percentiles 70,80,90,95
#[test]
fn test_pareto_percentiles() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("revenue\n10000\n8000\n6000\n4000\n2000\n1000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--percentiles")
        .arg("70,80,90,95");
    cmd.assert().success();
    Ok(())
}

/// Test case 16: lawkit zipf frequency_data.csv
#[test]
fn test_zipf_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("frequency\n100\n50\n33\n25\n20\n17");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 17: lawkit zipf text_document.txt --text
#[test]
fn test_zipf_text() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_txt("the quick brown fox jumps over the lazy dog the");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(temp_file.path())
        .arg("--text");
    cmd.assert().success();
    Ok(())
}

/// Test case 18: lawkit zipf large_text.txt --text --words 500
#[test]
fn test_zipf_text_words() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_txt("the quick brown fox jumps over the lazy dog the fox");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(temp_file.path())
        .arg("--text")
        .arg("--words")
        .arg("500");
    cmd.assert().success();
    Ok(())
}

/// Test case 19: lawkit zipf rankings.csv --verbose
#[test]
fn test_zipf_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("ranking\n100\n50\n33\n25\n20\n17");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 20: lawkit zipf data.csv --format json
#[test]
fn test_zipf_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("data\n100\n50\n33\n25\n20\n17");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 21: lawkit normal data.csv
#[test]
fn test_normal_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n50\n51\n49\n52\n48\n50");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 22: lawkit normal data.csv --test shapiro
#[test]
fn test_normal_test_shapiro() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("data\n50\n51\n49\n52\n48\n50");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(temp_file.path())
        .arg("--test")
        .arg("shapiro");
    cmd.assert().success();
    Ok(())
}

/// Test case 23: lawkit normal data.csv --outliers --outlier-method lof
#[test]
fn test_normal_outliers_lof() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n50\n51\n49\n52\n48\n100");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(temp_file.path())
        .arg("--outliers")
        .arg("--outlier-method")
        .arg("lof");
    cmd.assert().success();
    Ok(())
}

/// Test case 24: lawkit normal production_data.csv --quality-control --spec-limits 9.5,10.5
#[test]
fn test_normal_quality_control() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("production\n9.8\n10.1\n9.9\n10.2\n9.7\n10.0");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(temp_file.path())
        .arg("--quality-control")
        .arg("--spec-limits")
        .arg("9.5,10.5");
    cmd.assert().success();
    Ok(())
}

/// Test case 25: lawkit normal timeseries_data.csv --enable-timeseries --timeseries-window 20
#[test]
fn test_normal_timeseries() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("timeseries\n50\n51\n49\n52\n48\n50\n51\n49");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(temp_file.path())
        .arg("--enable-timeseries")
        .arg("--timeseries-window")
        .arg("20");
    cmd.assert().success();
    Ok(())
}

/// Test case 26: lawkit normal measurements.csv --verbose
#[test]
fn test_normal_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("measurements\n50.1\n49.9\n50.2\n49.8\n50.0\n50.1");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 27: lawkit normal quality_data.csv --format json
#[test]
fn test_normal_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("quality\n50\n51\n49\n52\n48\n50");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 28: lawkit poisson events.csv
#[test]
fn test_poisson_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("events\n3\n2\n4\n1\n3\n2");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 29: lawkit poisson data.csv --test chi_square
#[test]
fn test_poisson_test_chi_square() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("data\n3\n2\n4\n1\n3\n2");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(temp_file.path())
        .arg("--test")
        .arg("chi_square");
    cmd.assert().success();
    Ok(())
}

/// Test case 30: lawkit poisson server_logs.csv --predict --max-events 50
#[test]
fn test_poisson_predict() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("server_events\n5\n3\n7\n2\n4\n6");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(temp_file.path())
        .arg("--predict")
        .arg("--max-events")
        .arg("50");
    cmd.assert().success();
    Ok(())
}

/// Test case 31: lawkit poisson rare_events.csv --rare-events
#[test]
fn test_poisson_rare_events() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("rare\n0\n1\n0\n0\n2\n0");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(temp_file.path())
        .arg("--rare-events");
    cmd.assert().success();
    Ok(())
}

/// Test case 32: lawkit poisson incidents.csv --verbose
#[test]
fn test_poisson_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("incidents\n2\n3\n1\n4\n2\n3");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 33: lawkit poisson data.csv --format json
#[test]
fn test_poisson_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("data\n3\n2\n4\n1\n3\n2");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 34: lawkit poisson server_errors.csv --confidence 0.99 --verbose
#[test]
fn test_poisson_confidence_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("errors\n1\n0\n2\n1\n0\n3");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(temp_file.path())
        .arg("--confidence")
        .arg("0.99")
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 35: lawkit generate benf --samples 5000
#[test]
fn test_generate_benf() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("benf")
        .arg("--samples")
        .arg("5000");
    cmd.assert().success();
    Ok(())
}

/// Test case 36: lawkit generate benf --samples 2000 --fraud-rate 0.1
#[test]
fn test_generate_benf_fraud() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("benf")
        .arg("--samples")
        .arg("2000")
        .arg("--fraud-rate")
        .arg("0.1");
    cmd.assert().success();
    Ok(())
}

/// Test case 37: lawkit generate benf --samples 1000 --seed 42 --range 1,50000
#[test]
fn test_generate_benf_seed_range() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("benf")
        .arg("--samples")
        .arg("1000")
        .arg("--seed")
        .arg("42")
        .arg("--range")
        .arg("1,50000");
    cmd.assert().success();
    Ok(())
}

/// Test case 38: lawkit generate normal --samples 1000 --output-file test_data.csv
#[test]
fn test_generate_normal_output_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp_output = NamedTempFile::new().unwrap();
    
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("normal")
        .arg("--samples")
        .arg("1000")
        .arg("--output-file")
        .arg(temp_output.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 39: lawkit analyze data.csv
#[test]
fn test_analyze_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 40: lawkit analyze transactions.csv --purpose fraud --recommend
#[test]
fn test_analyze_fraud_recommend() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("transactions\n1234\n2345\n3456\n1111\n2222");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--purpose")
        .arg("fraud")
        .arg("--recommend");
    cmd.assert().success();
    Ok(())
}

/// Test case 41: lawkit analyze data.csv --laws benf,normal --focus quality
#[test]
fn test_analyze_laws_focus() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("data\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benf,normal")
        .arg("--focus")
        .arg("quality");
    cmd.assert().success();
    Ok(())
}

/// Test case 42: lawkit analyze dataset.csv --verbose --format json
#[test]
fn test_analyze_verbose_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("dataset\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--verbose")
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 43: lawkit validate data.csv
#[test]
fn test_validate_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("validate").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 44: lawkit diagnose data.csv
#[test]
fn test_diagnose_basic() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("diagnose").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}

/// Test case 45: lawkit benf transactions.csv --verbose
#[test]
fn test_benf_transactions_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("transactions\n1234\n2345\n3456\n1111\n2222");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 46: lawkit analyze suspicious_data.csv --purpose fraud --recommend
#[test]
fn test_analyze_suspicious_fraud() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("suspicious\n1111\n2222\n3333\n4444\n5555");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--purpose")
        .arg("fraud")
        .arg("--recommend");
    cmd.assert().success();
    Ok(())
}

/// Test case 47: lawkit analyze dataset.csv --purpose quality --verbose
#[test]
fn test_analyze_quality_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("dataset\n50\n51\n49\n52\n48\n50");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--purpose")
        .arg("quality")
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 48: lawkit normal dataset.csv --verbose
#[test]
fn test_normal_dataset_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("dataset\n50\n51\n49\n52\n48\n50");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 49: lawkit pareto sales.csv --threshold 0.8
#[test]
fn test_pareto_sales_threshold() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("sales\n1000\n800\n600\n400\n200\n100");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--threshold")
        .arg("0.8");
    cmd.assert().success();
    Ok(())
}

/// Test case 50: lawkit zipf customer_frequency.csv --verbose
#[test]
fn test_zipf_customer_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("customer_frequency\n100\n50\n33\n25\n20");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

// Additional examples continued...
// The remaining 45 examples would follow similar patterns for:
// - Different output formats (csv, yaml, toml, xml)
// - Various filter options
// - Help commands for each subcommand
// - Integration with different data types
// - Advanced configuration options