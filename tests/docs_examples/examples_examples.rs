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

// Helper function to create temporary text files for testing
fn create_temp_txt(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(file, "{}", content).expect("Failed to write to temp file");
    file
}

/// Test case 1: lawkit benf expenses_2024.csv --format json
#[test]
fn test_expenses_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("expense\n123.45\n234.56\n345.67\n111.22\n222.33\n333.44");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 2: lawkit benf expenses_2024.csv --verbose
#[test]
fn test_expenses_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("expense\n123.45\n234.56\n345.67\n111.22\n222.33\n333.44");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 3: lawkit benf expenses_2024.csv --confidence 0.99 --verbose
#[test]
fn test_expenses_confidence_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("expense\n123.45\n234.56\n345.67\n111.22\n222.33\n333.44");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--confidence")
        .arg("0.99")
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 4: lawkit benf expenses_2024.csv --min-value 50 --threshold high
#[test]
fn test_expenses_min_value_threshold() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("expense\n123.45\n234.56\n345.67\n111.22\n222.33\n333.44");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--min-value")
        .arg("50")
        .arg("--threshold")
        .arg("high");
    cmd.assert().success();
    Ok(())
}

/// Test case 5: lawkit benf expenses_2024.csv --sample-size 10000 (removing --optimize as deprecated)
#[test]
fn test_expenses_sample_size() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("expense\n123.45\n234.56\n345.67\n111.22\n222.33\n333.44");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--sample-size")
        .arg("10000");
    cmd.assert().success();
    Ok(())
}

/// Test case 6: lawkit analyze expenses_2024.csv --laws benford,normal
#[test]
fn test_analyze_expenses() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("expense\n123.45\n234.56\n345.67\n111.22\n222.33\n333.44");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford,normal");
    cmd.assert().success();
    Ok(())
}

/// Test case 7: lawkit benf monthly_sales.csv --verbose
#[test]
fn test_monthly_sales() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("sales\n1234\n2345\n3456\n1111\n2222\n3333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 8: lawkit benf sales_by_region.csv --verbose
#[test]
fn test_sales_by_region() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("regional_sales\n10000\n20000\n30000\n11000\n21000\n31000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 9: lawkit pareto customer_sales.csv --threshold 0.8
#[test]
fn test_pareto_customers_08() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("customer_sales\n10000\n5000\n3000\n2000\n1000\n500");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--threshold")
        .arg("0.8");
    cmd.assert().success();
    Ok(())
}

/// Test case 10: lawkit pareto customer_sales.csv --threshold 0.9
#[test]
fn test_pareto_customers_09() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("customer_sales\n10000\n5000\n3000\n2000\n1000\n500");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--threshold")
        .arg("0.9");
    cmd.assert().success();
    Ok(())
}

/// Test case 11: lawkit pareto customer_sales.csv --format csv
#[test]
fn test_pareto_csv_format() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("customer_sales\n10000\n5000\n3000\n2000\n1000\n500");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--format")
        .arg("csv");
    cmd.assert().success();
    Ok(())
}

/// Test case 12: lawkit pareto inventory_turnover.csv --verbose
#[test]
fn test_inventory_turnover() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("turnover\n100\n50\n30\n20\n10\n5");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 13: lawkit normal seasonal_demand.csv --verbose
#[test]
fn test_seasonal_demand() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("demand\n50\n51\n49\n52\n48\n50");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 14: lawkit zipf website_content.txt --verbose
#[test]
fn test_website_content() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_txt("the quick brown fox jumps over the lazy dog the");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 15: lawkit zipf blog_posts.txt --verbose
#[test]
fn test_blog_posts() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_txt("content analysis blog posts website optimization");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 16: lawkit zipf hashtags.csv --verbose
#[test]
fn test_hashtags() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("hashtag_count\n100\n50\n33\n25\n20\n17");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 17: lawkit poisson post_engagements.csv --verbose
#[test]
fn test_post_engagements() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("engagements\n3\n2\n4\n1\n3\n2");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 18: lawkit normal product_dimensions.csv --verbose
#[test]
fn test_product_dimensions() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("dimension\n10.1\n9.9\n10.2\n9.8\n10.0\n10.1");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 19: lawkit poisson defect_rates.csv --confidence 0.99 --verbose
#[test]
fn test_defect_rates() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("defects\n2\n1\n3\n0\n2\n1");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(temp_file.path())
        .arg("--confidence")
        .arg("0.99")
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 20: lawkit normal response_times.csv --verbose
#[test]
fn test_response_times() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("response_time\n250\n260\n240\n270\n230\n250");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 21: lawkit poisson incidents.csv --confidence 0.95 --verbose
#[test]
fn test_incidents() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("incidents\n1\n0\n2\n1\n0\n3");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(temp_file.path())
        .arg("--confidence")
        .arg("0.95")
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 22: lawkit analyze financial_data.csv --laws benford,pareto,normal --purpose audit
#[test]
fn test_financial_data_audit() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("amount\n1234\n2345\n3456\n1111\n2222\n3333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford,pareto,normal")
        .arg("--purpose")
        .arg("fraud");
    cmd.assert().success();
    Ok(())
}

/// Test case 23: lawkit analyze data.csv --laws all --format json
#[test]
fn test_analyze_all_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
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

/// Test case 24: lawkit benf daily_transactions.csv --verbose
#[test]
fn test_daily_transactions() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("transaction\n1234\n2345\n3456\n1111\n2222\n3333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 25: lawkit pareto daily_sales.csv --verbose
#[test]
fn test_daily_sales() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("daily_sales\n10000\n5000\n3000\n2000\n1000\n500");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 26: lawkit normal process_metrics.csv --verbose
#[test]
fn test_process_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("metric\n95.5\n96.2\n94.8\n97.1\n93.9\n95.5");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 27: lawkit analyze daily_data.csv --laws benford,pareto,normal --format json
#[test]
fn test_daily_data_analyze() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("daily\n1234\n2345\n3456\n1111\n2222\n3333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford,pareto,normal")
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 28: lawkit benf large_dataset.csv --quiet
#[test]
fn test_large_dataset_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("large_data\n100000\n200000\n300000\n110000\n210000\n310000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--quiet");
    cmd.assert().success();
    Ok(())
}

/// Test case 29: lawkit benf huge_data.csv --format json (pipe test simplified)
#[test]
fn test_huge_data_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("huge_data\n1000000\n2000000\n3000000\n1100000\n2100000\n3100000");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 30: tail -f live_data.log | lawkit benf --quiet (simplified)
#[test]
fn test_streaming_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg("--quiet");
    cmd.write_stdin("123 456 789");
    cmd.assert().success();
    Ok(())
}

/// Test case 31: lawkit generate benf --samples 10000
#[test]
fn test_generate_benf_10000() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("benf")
        .arg("--samples")
        .arg("10000");
    cmd.assert().success();
    Ok(())
}

/// Test case 32: lawkit benf benf_test_data.csv --format json
#[test]
fn test_benf_test_data() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("test_data\n1234\n2345\n3456\n1111\n2222\n3333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 33: lawkit generate pareto --samples 5000
#[test]
fn test_generate_pareto_5000() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("pareto")
        .arg("--samples")
        .arg("5000");
    cmd.assert().success();
    Ok(())
}

/// Test case 34: lawkit generate zipf --samples 2000
#[test]
fn test_generate_zipf_2000() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("zipf")
        .arg("--samples")
        .arg("2000");
    cmd.assert().success();
    Ok(())
}

/// Test case 35: lawkit generate normal --samples 1000
#[test]
fn test_generate_normal_1000() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("normal")
        .arg("--samples")
        .arg("1000");
    cmd.assert().success();
    Ok(())
}

/// Test case 36: lawkit generate poisson --samples 1000
#[test]
fn test_generate_poisson_1000() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("poisson")
        .arg("--samples")
        .arg("1000");
    cmd.assert().success();
    Ok(())
}

/// Test case 37: lawkit generate benf --samples 5000
#[test]
fn test_generate_benf_5000() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("benf")
        .arg("--samples")
        .arg("5000");
    cmd.assert().success();
    Ok(())
}

/// Test case 38: lawkit validate test_benf.csv --laws benford
#[test]
fn test_validate_benf() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("benf_data\n1234\n2345\n3456\n1111\n2222\n3333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("validate")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("benford");
    cmd.assert().success();
    Ok(())
}

/// Test case 39: lawkit generate poisson --samples 1000 (duplicate)
#[test]
fn test_generate_poisson_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("poisson")
        .arg("--samples")
        .arg("1000");
    cmd.assert().success();
    Ok(())
}

/// Test case 40: lawkit poisson poisson_test.csv --format json
#[test]
fn test_poisson_test_json() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("poisson_test\n3\n2\n4\n1\n3\n2");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 41: lawkit generate normal --samples 5000
#[test]
fn test_generate_normal_5000() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("normal")
        .arg("--samples")
        .arg("5000");
    cmd.assert().success();
    Ok(())
}

/// Test case 42: lawkit analyze normal_data.csv --laws normal,benford,zipf
#[test]
fn test_analyze_normal_data() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("normal_data\n50\n51\n49\n52\n48\n50");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(temp_file.path())
        .arg("--laws")
        .arg("normal,benford,zipf");
    cmd.assert().success();
    Ok(())
}

/// Test case 43: lawkit list --help
#[test]
fn test_list_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("list").arg("--help");
    cmd.assert().success();
    Ok(())
}

/// Test case 44: lawkit generate benf --samples 10000 (CI test)
#[test]
fn test_ci_generate_benf() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("benf")
        .arg("--samples")
        .arg("10000");
    cmd.assert().success();
    Ok(())
}

/// Test case 45: lawkit generate normal --samples 1000 (CI test)
#[test]
fn test_ci_generate_normal() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("normal")
        .arg("--samples")
        .arg("1000");
    cmd.assert().success();
    Ok(())
}

/// Test case 46: lawkit normal normal_test.csv --verbose
#[test]
fn test_normal_test_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("normal_test\n50\n51\n49\n52\n48\n50");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 47: lawkit generate poisson --samples 5000 (CI test)
#[test]
fn test_ci_generate_poisson() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("poisson")
        .arg("--samples")
        .arg("5000");
    cmd.assert().success();
    Ok(())
}

/// Test case 48: lawkit poisson poisson_test.csv --format json (CI test)
#[test]
fn test_ci_poisson_test() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("poisson_test\n2\n3\n1\n4\n2\n3");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 49: lawkit --help
#[test]
fn test_main_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("--help");
    cmd.assert().success();
    Ok(())
}

/// Test case 50: lawkit benf --help
#[test]
fn test_benf_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg("--help");
    cmd.assert().success();
    Ok(())
}

/// Test case 51: lawkit pareto --help
#[test]
fn test_pareto_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("pareto").arg("--help");
    cmd.assert().success();
    Ok(())
}

/// Test case 52: lawkit zipf --help
#[test]
fn test_zipf_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("zipf").arg("--help");
    cmd.assert().success();
    Ok(())
}

/// Test case 53: lawkit normal --help
#[test]
fn test_normal_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("normal").arg("--help");
    cmd.assert().success();
    Ok(())
}

/// Test case 54: lawkit poisson --help
#[test]
fn test_poisson_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("poisson").arg("--help");
    cmd.assert().success();
    Ok(())
}

/// Test case 55: lawkit analyze --help
#[test]
fn test_analyze_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze").arg("--help");
    cmd.assert().success();
    Ok(())
}

/// Test case 56: lawkit validate --help
#[test]
fn test_validate_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("validate").arg("--help");
    cmd.assert().success();
    Ok(())
}

/// Test case 57: lawkit diagnose --help
#[test]
fn test_diagnose_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("diagnose").arg("--help");
    cmd.assert().success();
    Ok(())
}

/// Test case 58: lawkit generate --help
#[test]
fn test_generate_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("generate").arg("--help");
    cmd.assert().success();
    Ok(())
}

/// Test case 59: lawkit list --help (duplicate)
#[test]
fn test_list_help_duplicate() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("list").arg("--help");
    cmd.assert().success();
    Ok(())
}

/// Test case 60: lawkit benf data.csv --format json
#[test]
fn test_format_json_example() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("json");
    cmd.assert().success();
    Ok(())
}

/// Test case 61: lawkit benf data.csv --format csv
#[test]
fn test_format_csv_example() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("csv");
    cmd.assert().success();
    Ok(())
}

/// Test case 62: lawkit benf data.csv --format yaml
#[test]
fn test_format_yaml_example() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("yaml");
    cmd.assert().success();
    Ok(())
}

/// Test case 63: lawkit benf data.csv --format toml
#[test]
fn test_format_toml_example() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("toml");
    cmd.assert().success();
    Ok(())
}

/// Test case 64: lawkit benf data.csv --format xml
#[test]
fn test_format_xml_example() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--format")
        .arg("xml");
    cmd.assert().success();
    Ok(())
}

/// Test case 65: lawkit benf data.csv --quiet
#[test]
fn test_quiet_example() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--quiet");
    cmd.assert().success();
    Ok(())
}

/// Test case 66: lawkit benf data.csv --verbose
#[test]
fn test_verbose_example() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("value\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(temp_file.path())
        .arg("--verbose");
    cmd.assert().success();
    Ok(())
}

/// Test case 67: find data/ -name "*.csv" | xargs -P 4 -I {} lawkit benf {} (simplified as single command)
#[test]
fn test_parallel_processing() -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = create_temp_csv("parallel_data\n123\n234\n345\n111\n222\n333");
    
    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert().success();
    Ok(())
}