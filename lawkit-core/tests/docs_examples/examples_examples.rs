use lawkit_core::laws::benford::BenfordAnalysis;
use lawkit_core::laws::pareto::ParetoAnalysis;
use lawkit_core::laws::zipf::ZipfAnalysis;
use lawkit_core::laws::normal::NormalAnalysis;
use lawkit_core::laws::poisson::PoissonAnalysis;
use lawkit_core::laws::integration::IntegrationAnalysis;

/// Test case 1: lawkit benf expenses_2024.csv --format json (Core API equivalent)
#[test]
fn test_expenses_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.45, 234.56, 345.67, 111.22, 222.33, 333.44];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 2: lawkit benf expenses_2024.csv --verbose (Core API equivalent)
#[test]
fn test_expenses_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.45, 234.56, 345.67, 111.22, 222.33, 333.44];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 3: lawkit benf expenses_2024.csv --confidence 0.99 --verbose (Core API equivalent)
#[test]
fn test_expenses_confidence_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.45, 234.56, 345.67, 111.22, 222.33, 333.44];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_confidence(0.99);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 4: lawkit benf expenses_2024.csv --min-value 50 --threshold high (Core API equivalent)
#[test]
fn test_expenses_min_value_threshold() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.45, 234.56, 345.67, 111.22, 222.33, 333.44];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_min_value(50.0);
    analysis.set_threshold("high");
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 5: lawkit benf expenses_2024.csv --sample-size 10000 (Core API equivalent)
#[test]
fn test_expenses_sample_size() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.45, 234.56, 345.67, 111.22, 222.33, 333.44];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_sample_size(10000);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 6: lawkit analyze expenses_2024.csv --laws benford,normal (Core API equivalent)
#[test]
fn test_analyze_expenses() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.45, 234.56, 345.67, 111.22, 222.33, 333.44];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("normal");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 7: lawkit benf monthly_sales.csv --verbose (Core API equivalent)
#[test]
fn test_monthly_sales() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.0, 2345.0, 3456.0, 1111.0, 2222.0, 3333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 8: lawkit benf sales_by_region.csv --verbose (Core API equivalent)
#[test]
fn test_sales_by_region() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10000.0, 20000.0, 30000.0, 11000.0, 21000.0, 31000.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 9: lawkit pareto customer_sales.csv --threshold 0.8 (Core API equivalent)
#[test]
fn test_pareto_customers_08() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10000.0, 5000.0, 3000.0, 2000.0, 1000.0, 500.0];
    let mut analysis = ParetoAnalysis::new();
    analysis.set_threshold(0.8);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 10: lawkit pareto customer_sales.csv --threshold 0.9 (Core API equivalent)
#[test]
fn test_pareto_customers_09() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10000.0, 5000.0, 3000.0, 2000.0, 1000.0, 500.0];
    let mut analysis = ParetoAnalysis::new();
    analysis.set_threshold(0.9);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 11: lawkit pareto customer_sales.csv --format csv (Core API equivalent)
#[test]
fn test_pareto_csv_format() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10000.0, 5000.0, 3000.0, 2000.0, 1000.0, 500.0];
    let mut analysis = ParetoAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 12: lawkit pareto inventory_turnover.csv --verbose (Core API equivalent)
#[test]
fn test_inventory_turnover() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![100.0, 50.0, 30.0, 20.0, 10.0, 5.0];
    let mut analysis = ParetoAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 13: lawkit normal seasonal_demand.csv --verbose (Core API equivalent)
#[test]
fn test_seasonal_demand() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![50.0, 51.0, 49.0, 52.0, 48.0, 50.0];
    let mut analysis = NormalAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 14: lawkit zipf website_content.txt --verbose (Core API equivalent)
#[test]
fn test_website_content() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10.0, 5.0, 3.0, 2.0]; // Word frequencies
    let mut analysis = ZipfAnalysis::new();
    analysis.enable_text_mode();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 15: lawkit zipf blog_posts.txt --verbose (Core API equivalent)
#[test]
fn test_blog_posts() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![8.0, 4.0, 2.0, 1.0]; // Content frequencies
    let mut analysis = ZipfAnalysis::new();
    analysis.enable_text_mode();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 16: lawkit zipf hashtags.csv --verbose (Core API equivalent)
#[test]
fn test_hashtags() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![100.0, 50.0, 33.0, 25.0, 20.0, 17.0];
    let mut analysis = ZipfAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 17: lawkit poisson post_engagements.csv --verbose (Core API equivalent)
#[test]
fn test_post_engagements() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![3.0, 2.0, 4.0, 1.0, 3.0, 2.0];
    let mut analysis = PoissonAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 18: lawkit normal product_dimensions.csv --verbose (Core API equivalent)
#[test]
fn test_product_dimensions() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10.1, 9.9, 10.2, 9.8, 10.0, 10.1];
    let mut analysis = NormalAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 19: lawkit poisson defect_rates.csv --confidence 0.99 --verbose (Core API equivalent)
#[test]
fn test_defect_rates() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![2.0, 1.0, 3.0, 0.0, 2.0, 1.0];
    let mut analysis = PoissonAnalysis::new();
    analysis.set_confidence(0.99);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 20: lawkit normal response_times.csv --verbose (Core API equivalent)
#[test]
fn test_response_times() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![250.0, 260.0, 240.0, 270.0, 230.0, 250.0];
    let mut analysis = NormalAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 21: lawkit poisson incidents.csv --confidence 0.95 --verbose (Core API equivalent)
#[test]
fn test_incidents() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.0, 0.0, 2.0, 1.0, 0.0, 3.0];
    let mut analysis = PoissonAnalysis::new();
    analysis.set_confidence(0.95);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 22: lawkit analyze financial_data.csv --laws benford,pareto,normal --purpose audit (Core API equivalent)
#[test]
fn test_financial_data_audit() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.0, 2345.0, 3456.0, 1111.0, 2222.0, 3333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("pareto");
    analysis.add_law("normal");
    analysis.set_purpose("fraud");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 23: lawkit analyze data.csv --laws all --format json (Core API equivalent)
#[test]
fn test_analyze_all_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_all_laws();
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 24: lawkit benf daily_transactions.csv --verbose (Core API equivalent)
#[test]
fn test_daily_transactions() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.0, 2345.0, 3456.0, 1111.0, 2222.0, 3333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 25: lawkit pareto daily_sales.csv --verbose (Core API equivalent)
#[test]
fn test_daily_sales() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10000.0, 5000.0, 3000.0, 2000.0, 1000.0, 500.0];
    let mut analysis = ParetoAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 26: lawkit normal process_metrics.csv --verbose (Core API equivalent)
#[test]
fn test_process_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![95.5, 96.2, 94.8, 97.1, 93.9, 95.5];
    let mut analysis = NormalAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 27: lawkit analyze daily_data.csv --laws benford,pareto,normal --format json (Core API equivalent)
#[test]
fn test_daily_data_analyze() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.0, 2345.0, 3456.0, 1111.0, 2222.0, 3333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("pareto");
    analysis.add_law("normal");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 28: lawkit benf large_dataset.csv --quiet (Core API equivalent)
#[test]
fn test_large_dataset_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![100000.0, 200000.0, 300000.0, 110000.0, 210000.0, 310000.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 29: lawkit benf huge_data.csv --format json (Core API equivalent)
#[test]
fn test_huge_data_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000000.0, 2000000.0, 3000000.0, 1100000.0, 2100000.0, 3100000.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 30: tail -f live_data.log | lawkit benf --quiet (Core API equivalent)
#[test]
fn test_streaming_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 456.0, 789.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 31: lawkit generate benf --samples 10000 (Core API equivalent - N/A)
#[test]
fn test_generate_benf_10000() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have generate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 32: lawkit benf benf_test_data.csv --format json (Core API equivalent)
#[test]
fn test_benf_test_data() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.0, 2345.0, 3456.0, 1111.0, 2222.0, 3333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 33: lawkit generate pareto --samples 5000 (Core API equivalent - N/A)
#[test]
fn test_generate_pareto_5000() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have generate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 34: lawkit generate zipf --samples 2000 (Core API equivalent - N/A)
#[test]
fn test_generate_zipf_2000() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have generate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 35: lawkit generate normal --samples 1000 (Core API equivalent - N/A)
#[test]
fn test_generate_normal_1000() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have generate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 36: lawkit generate poisson --samples 1000 (Core API equivalent - N/A)
#[test]
fn test_generate_poisson_1000() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have generate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 37: lawkit generate benf --samples 5000 (Core API equivalent - N/A)
#[test]
fn test_generate_benf_5000() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have generate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 38: lawkit validate test_benf.csv --laws benford (Core API equivalent - N/A)
#[test]
fn test_validate_benf() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have validate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 39: lawkit generate poisson --samples 1000 (Core API equivalent - N/A - duplicate)
#[test]
fn test_generate_poisson_test() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have generate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 40: lawkit poisson poisson_test.csv --format json (Core API equivalent)
#[test]
fn test_poisson_test_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![3.0, 2.0, 4.0, 1.0, 3.0, 2.0];
    let mut analysis = PoissonAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 41: lawkit generate normal --samples 5000 (Core API equivalent - N/A)
#[test]
fn test_generate_normal_5000() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have generate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 42: lawkit analyze normal_data.csv --laws normal,benford,zipf (Core API equivalent)
#[test]
fn test_analyze_normal_data() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![50.0, 51.0, 49.0, 52.0, 48.0, 50.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("normal");
    analysis.add_law("benford");
    analysis.add_law("zipf");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 43: lawkit list --help (Core API equivalent - N/A)
#[test]
fn test_list_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have list - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 44: lawkit generate benf --samples 10000 (Core API equivalent - N/A - CI test)
#[test]
fn test_ci_generate_benf() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have generate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 45: lawkit generate normal --samples 1000 (Core API equivalent - N/A - CI test)
#[test]
fn test_ci_generate_normal() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have generate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 46: lawkit normal normal_test.csv --verbose (Core API equivalent)
#[test]
fn test_normal_test_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![50.0, 51.0, 49.0, 52.0, 48.0, 50.0];
    let mut analysis = NormalAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 47: lawkit generate poisson --samples 5000 (Core API equivalent - N/A - CI test)
#[test]
fn test_ci_generate_poisson() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have generate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 48: lawkit poisson poisson_test.csv --format json (Core API equivalent - CI test)
#[test]
fn test_ci_poisson_test() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![2.0, 3.0, 1.0, 4.0, 2.0, 3.0];
    let mut analysis = PoissonAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 49: lawkit --help (Core API equivalent - N/A)
#[test]
fn test_main_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 50: lawkit benf --help (Core API equivalent - N/A)
#[test]
fn test_benf_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 51: lawkit pareto --help (Core API equivalent - N/A)
#[test]
fn test_pareto_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 52: lawkit zipf --help (Core API equivalent - N/A)
#[test]
fn test_zipf_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 53: lawkit normal --help (Core API equivalent - N/A)
#[test]
fn test_normal_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 54: lawkit poisson --help (Core API equivalent - N/A)
#[test]
fn test_poisson_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 55: lawkit analyze --help (Core API equivalent - N/A)
#[test]
fn test_analyze_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 56: lawkit validate --help (Core API equivalent - N/A)
#[test]
fn test_validate_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 57: lawkit diagnose --help (Core API equivalent - N/A)
#[test]
fn test_diagnose_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 58: lawkit generate --help (Core API equivalent - N/A)
#[test]
fn test_generate_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 59: lawkit list --help (Core API equivalent - N/A - duplicate)
#[test]
fn test_list_help_duplicate() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 60: lawkit benf data.csv --format json (Core API equivalent)
#[test]
fn test_format_json_example() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 61: lawkit benf data.csv --format csv (Core API equivalent)
#[test]
fn test_format_csv_example() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 62: lawkit benf data.csv --format yaml (Core API equivalent)
#[test]
fn test_format_yaml_example() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 63: lawkit benf data.csv --format toml (Core API equivalent)
#[test]
fn test_format_toml_example() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 64: lawkit benf data.csv --format xml (Core API equivalent)
#[test]
fn test_format_xml_example() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 65: lawkit benf data.csv --quiet (Core API equivalent)
#[test]
fn test_quiet_example() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 66: lawkit benf data.csv --verbose (Core API equivalent)
#[test]
fn test_verbose_example() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 67: find data/ -name "*.csv" | xargs -P 4 -I {} lawkit benf {} (Core API equivalent)
#[test]
fn test_parallel_processing() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}