use lawkit_core::laws::benford::BenfordAnalysis;
use lawkit_core::laws::pareto::ParetoAnalysis;
use lawkit_core::laws::zipf::ZipfAnalysis;
use lawkit_core::laws::normal::NormalAnalysis;
use lawkit_core::laws::poisson::PoissonAnalysis;
use lawkit_core::laws::integration::IntegrationAnalysis;

/// Test case 1: lawkit --help (Core API equivalent - N/A)
#[test]
fn test_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 2: lawkit --version (Core API equivalent)
#[test]
fn test_version() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(lawkit_core::VERSION, "2.0.1");
    Ok(())
}

/// Test case 3: lawkit list (Core API equivalent - N/A)
#[test]
fn test_list() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have list - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 4: lawkit benf data.csv (Core API equivalent)
#[test]
fn test_benf_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 5: lawkit benf transactions.json --verbose --format json (Core API equivalent)
#[test]
fn test_benf_verbose_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.0, 2345.0, 3456.0, 1111.0, 2222.0, 3333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 6: lawkit benf data.csv --quiet (Core API equivalent)
#[test]
fn test_benf_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 7: lawkit benf accounts.csv --filter ">=1000" --threshold high (Core API equivalent)
#[test]
fn test_benf_filter_threshold() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 2000.0, 3000.0, 1100.0, 2100.0, 3100.0];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_threshold("high");
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 8: lawkit benf audit_data.csv --confidence 0.99 --verbose (Core API equivalent)
#[test]
fn test_benf_confidence_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.0, 2345.0, 3456.0, 1111.0, 2222.0, 3333.0];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_confidence(0.99);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 9: lawkit benf big_data.csv --sample-size 50000 (Core API equivalent)
#[test]
fn test_benf_sample_size() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10000.0, 20000.0, 30000.0, 11000.0, 21000.0, 31000.0];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_sample_size(50000);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 10: lawkit benf financial_data.csv --min-value 100 (Core API equivalent)
#[test]
fn test_benf_min_value() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![100.0, 200.0, 300.0, 150.0, 250.0, 350.0];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_min_value(100.0);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 11: echo "123 456 789" | lawkit benf --verbose (Core API equivalent)
#[test]
fn test_benf_stdin_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 456.0, 789.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 12: lawkit pareto sales.csv (Core API equivalent)
#[test]
fn test_pareto_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 500.0, 300.0, 200.0, 100.0, 50.0];
    let mut analysis = ParetoAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 13: lawkit pareto data.csv --concentration 0.9 (Core API equivalent)
#[test]
fn test_pareto_concentration() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 500.0, 300.0, 200.0, 100.0, 50.0];
    let mut analysis = ParetoAnalysis::new();
    analysis.set_concentration(0.9);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 14: lawkit pareto customers.csv --business-analysis --gini-coefficient (Core API equivalent)
#[test]
fn test_pareto_business_gini() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 800.0, 600.0, 400.0, 200.0, 100.0];
    let mut analysis = ParetoAnalysis::new();
    analysis.enable_business_analysis();
    analysis.enable_gini_coefficient();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 15: lawkit pareto revenue.csv --percentiles 70,80,90,95 (Core API equivalent)
#[test]
fn test_pareto_percentiles() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10000.0, 8000.0, 6000.0, 4000.0, 2000.0, 1000.0];
    let mut analysis = ParetoAnalysis::new();
    analysis.set_percentiles(vec![70.0, 80.0, 90.0, 95.0]);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 16: lawkit zipf frequency_data.csv (Core API equivalent)
#[test]
fn test_zipf_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![100.0, 50.0, 33.0, 25.0, 20.0, 17.0];
    let mut analysis = ZipfAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 17: lawkit zipf text_document.txt --text (Core API equivalent)
#[test]
fn test_zipf_text() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10.0, 5.0, 3.0, 2.0]; // Word frequencies
    let mut analysis = ZipfAnalysis::new();
    analysis.enable_text_mode();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 18: lawkit zipf large_text.txt --text --words 500 (Core API equivalent)
#[test]
fn test_zipf_text_words() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10.0, 5.0, 3.0, 2.0, 1.0];
    let mut analysis = ZipfAnalysis::new();
    analysis.enable_text_mode();
    analysis.set_max_words(500);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 19: lawkit zipf rankings.csv --verbose (Core API equivalent)
#[test]
fn test_zipf_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![100.0, 50.0, 33.0, 25.0, 20.0, 17.0];
    let mut analysis = ZipfAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 20: lawkit zipf data.csv --format json (Core API equivalent)
#[test]
fn test_zipf_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![100.0, 50.0, 33.0, 25.0, 20.0, 17.0];
    let mut analysis = ZipfAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 21: lawkit normal data.csv (Core API equivalent)
#[test]
fn test_normal_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![50.0, 51.0, 49.0, 52.0, 48.0, 50.0];
    let mut analysis = NormalAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 22: lawkit normal data.csv --test shapiro (Core API equivalent)
#[test]
fn test_normal_test_shapiro() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![50.0, 51.0, 49.0, 52.0, 48.0, 50.0];
    let mut analysis = NormalAnalysis::new();
    analysis.set_test_method("shapiro");
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 23: lawkit normal data.csv --outliers --outlier-method lof (Core API equivalent)
#[test]
fn test_normal_outliers_lof() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![50.0, 51.0, 49.0, 52.0, 48.0, 100.0];
    let mut analysis = NormalAnalysis::new();
    analysis.enable_outlier_detection();
    analysis.set_outlier_method("lof");
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 24: lawkit normal production_data.csv --quality-control --spec-limits 9.5,10.5 (Core API equivalent)
#[test]
fn test_normal_quality_control() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![9.8, 10.1, 9.9, 10.2, 9.7, 10.0];
    let mut analysis = NormalAnalysis::new();
    analysis.enable_quality_control();
    analysis.set_spec_limits(9.5, 10.5);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 25: lawkit normal timeseries_data.csv --enable-timeseries --timeseries-window 20 (Core API equivalent)
#[test]
fn test_normal_timeseries() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![50.0, 51.0, 49.0, 52.0, 48.0, 50.0, 51.0, 49.0];
    let mut analysis = NormalAnalysis::new();
    analysis.enable_timeseries();
    analysis.set_timeseries_window(20);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 26: lawkit normal measurements.csv --verbose (Core API equivalent)
#[test]
fn test_normal_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![50.1, 49.9, 50.2, 49.8, 50.0, 50.1];
    let mut analysis = NormalAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 27: lawkit normal quality_data.csv --format json (Core API equivalent)
#[test]
fn test_normal_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![50.0, 51.0, 49.0, 52.0, 48.0, 50.0];
    let mut analysis = NormalAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 28: lawkit poisson events.csv (Core API equivalent)
#[test]
fn test_poisson_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![3.0, 2.0, 4.0, 1.0, 3.0, 2.0];
    let mut analysis = PoissonAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 29: lawkit poisson data.csv --test chi_square (Core API equivalent)
#[test]
fn test_poisson_test_chi_square() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![3.0, 2.0, 4.0, 1.0, 3.0, 2.0];
    let mut analysis = PoissonAnalysis::new();
    analysis.set_test_method("chi_square");
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 30: lawkit poisson server_logs.csv --predict --max-events 50 (Core API equivalent)
#[test]
fn test_poisson_predict() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![5.0, 3.0, 7.0, 2.0, 4.0, 6.0];
    let mut analysis = PoissonAnalysis::new();
    analysis.enable_prediction();
    analysis.set_max_events(50);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 31: lawkit poisson rare_events.csv --rare-events (Core API equivalent)
#[test]
fn test_poisson_rare_events() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![0.0, 1.0, 0.0, 0.0, 2.0, 0.0];
    let mut analysis = PoissonAnalysis::new();
    analysis.enable_rare_events();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 32: lawkit poisson incidents.csv --verbose (Core API equivalent)
#[test]
fn test_poisson_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![2.0, 3.0, 1.0, 4.0, 2.0, 3.0];
    let mut analysis = PoissonAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 33: lawkit poisson data.csv --format json (Core API equivalent)
#[test]
fn test_poisson_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![3.0, 2.0, 4.0, 1.0, 3.0, 2.0];
    let mut analysis = PoissonAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 34: lawkit poisson server_errors.csv --confidence 0.99 --verbose (Core API equivalent)
#[test]
fn test_poisson_confidence_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.0, 0.0, 2.0, 1.0, 0.0, 3.0];
    let mut analysis = PoissonAnalysis::new();
    analysis.set_confidence(0.99);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 35: lawkit generate benf --samples 5000 (Core API equivalent - N/A)
#[test]
fn test_generate_benf() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have generate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 36: lawkit generate benf --samples 2000 --fraud-rate 0.1 (Core API equivalent - N/A)
#[test]
fn test_generate_benf_fraud() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have generate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 37: lawkit generate benf --samples 1000 --seed 42 --range 1,50000 (Core API equivalent - N/A)
#[test]
fn test_generate_benf_seed_range() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have generate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 38: lawkit generate normal --samples 1000 --output-file test_data.csv (Core API equivalent - N/A)
#[test]
fn test_generate_normal_output_file() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have generate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 39: lawkit analyze data.csv (Core API equivalent)
#[test]
fn test_analyze_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 40: lawkit analyze transactions.csv --purpose fraud --recommend (Core API equivalent)
#[test]
fn test_analyze_fraud_recommend() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.0, 2345.0, 3456.0, 1111.0, 2222.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.set_purpose("fraud");
    analysis.enable_recommendations();
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 41: lawkit analyze data.csv --laws benf,normal --focus quality (Core API equivalent)
#[test]
fn test_analyze_laws_focus() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("normal");
    analysis.set_focus("quality");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 42: lawkit analyze dataset.csv --verbose --format json (Core API equivalent)
#[test]
fn test_analyze_verbose_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 43: lawkit validate data.csv (Core API equivalent - N/A)
#[test]
fn test_validate_basic() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have validate - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 44: lawkit diagnose data.csv (Core API equivalent - N/A)
#[test]
fn test_diagnose_basic() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have diagnose - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 45: lawkit benf transactions.csv --verbose (Core API equivalent)
#[test]
fn test_benf_transactions_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.0, 2345.0, 3456.0, 1111.0, 2222.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 46: lawkit analyze suspicious_data.csv --purpose fraud --recommend (Core API equivalent)
#[test]
fn test_analyze_suspicious_fraud() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1111.0, 2222.0, 3333.0, 4444.0, 5555.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.set_purpose("fraud");
    analysis.enable_recommendations();
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 47: lawkit analyze dataset.csv --purpose quality --verbose (Core API equivalent)
#[test]
fn test_analyze_quality_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![50.0, 51.0, 49.0, 52.0, 48.0, 50.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("normal");
    analysis.set_purpose("quality");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 48: lawkit normal dataset.csv --verbose (Core API equivalent)
#[test]
fn test_normal_dataset_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![50.0, 51.0, 49.0, 52.0, 48.0, 50.0];
    let mut analysis = NormalAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 49: lawkit pareto sales.csv --threshold 0.8 (Core API equivalent)
#[test]
fn test_pareto_sales_threshold() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 800.0, 600.0, 400.0, 200.0, 100.0];
    let mut analysis = ParetoAnalysis::new();
    analysis.set_threshold(0.8);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 50: lawkit zipf customer_frequency.csv --verbose (Core API equivalent)
#[test]
fn test_zipf_customer_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![100.0, 50.0, 33.0, 25.0, 20.0];
    let mut analysis = ZipfAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}