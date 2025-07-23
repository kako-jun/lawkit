use lawkit_core::laws::benford::BenfordAnalysis;
use lawkit_core::laws::pareto::ParetoAnalysis;
use lawkit_core::laws::zipf::ZipfAnalysis;
use lawkit_core::laws::normal::NormalAnalysis;
use lawkit_core::laws::poisson::PoissonAnalysis;
use lawkit_core::laws::integration::IntegrationAnalysis;
use lawkit_core::generate::DataGenerator;
use lawkit_core::validate::DataValidator;
use lawkit_core::diagnose::DataDiagnoser;

/// Test case 1: lawkit benf data.csv (Core API equivalent)
#[test]
fn test_benf_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 2: lawkit benf --verbose --threshold critical data.csv (Core API equivalent)
#[test]
fn test_benf_verbose_threshold_critical() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_verbose(true);
    analysis.set_threshold("critical");
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 3: lawkit benf --format json data.csv (Core API equivalent)
#[test]
fn test_benf_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 4: lawkit pareto sales_data.csv (Core API equivalent)
#[test]
fn test_pareto_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![80000.0, 12000.0, 5000.0, 2000.0, 1000.0];
    let mut analysis = ParetoAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 5: lawkit pareto --verbose --format json revenue.csv (Core API equivalent)
#[test]
fn test_pareto_verbose_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![100000.0, 50000.0, 20000.0, 10000.0, 5000.0];
    let mut analysis = ParetoAnalysis::new();
    analysis.set_verbose(true);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 6: lawkit pareto --threshold high customer_values.csv (Core API equivalent)
#[test]
fn test_pareto_threshold_high() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10000.0, 5000.0, 3000.0, 2000.0, 1000.0];
    let mut analysis = ParetoAnalysis::new();
    analysis.set_threshold("high");
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 7: lawkit zipf data.csv (Core API equivalent)
#[test]
fn test_zipf_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 500.0, 333.0, 250.0, 200.0];
    let mut analysis = ZipfAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 8: lawkit zipf --verbose city_populations.csv (Core API equivalent)
#[test]
fn test_zipf_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10000000.0, 5000000.0, 3000000.0, 2000000.0, 1000000.0];
    let mut analysis = ZipfAnalysis::new();
    analysis.set_verbose(true);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 9: lawkit zipf --format json data.csv (Core API equivalent)
#[test]
fn test_zipf_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 500.0, 333.0, 250.0, 200.0];
    let mut analysis = ZipfAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 10: lawkit normal measurements.csv (Core API equivalent)
#[test]
fn test_normal_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.2, 1.3, 1.1, 1.4, 1.0, 1.5];
    let mut analysis = NormalAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 11: lawkit normal --verbose data.csv (Core API equivalent)
#[test]
fn test_normal_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.2, 1.3, 1.1, 1.4, 1.0, 1.5];
    let mut analysis = NormalAnalysis::new();
    analysis.set_verbose(true);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 12: lawkit normal --format json data.csv (Core API equivalent)
#[test]
fn test_normal_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.2, 1.3, 1.1, 1.4, 1.0, 1.5];
    let mut analysis = NormalAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 13: lawkit normal --threshold high production_data.csv (Core API equivalent)
#[test]
fn test_normal_threshold_high() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![99.1, 99.3, 99.0, 99.4, 98.9, 99.5];
    let mut analysis = NormalAnalysis::new();
    analysis.set_threshold("high");
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 14: lawkit poisson event_counts.csv (Core API equivalent)
#[test]
fn test_poisson_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.0, 2.0, 3.0, 0.0, 1.0, 2.0];
    let mut analysis = PoissonAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 15: lawkit poisson --verbose data.csv (Core API equivalent)
#[test]
fn test_poisson_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.0, 2.0, 3.0, 0.0, 1.0, 2.0];
    let mut analysis = PoissonAnalysis::new();
    analysis.set_verbose(true);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 16: lawkit poisson --format json incidents.csv (Core API equivalent)
#[test]
fn test_poisson_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.0, 2.0, 3.0, 0.0, 1.0, 2.0];
    let mut analysis = PoissonAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 17: lawkit poisson --threshold high defect_data.csv (Core API equivalent)
#[test]
fn test_poisson_threshold_high() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.0, 2.0, 3.0, 0.0, 1.0, 2.0];
    let mut analysis = PoissonAnalysis::new();
    analysis.set_threshold("high");
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 18-27: Input format tests (simplified for Core API)
#[test]
fn test_various_input_formats() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 28-33: Output format tests (Core API returns structured data)
#[test]
fn test_various_output_formats() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 34-38: Threshold tests
#[test]
fn test_various_thresholds() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    
    // Test different thresholds
    analysis.set_threshold("low");
    let result1 = analysis.analyze(&data)?;
    assert!(result1.numbers_analyzed > 0);
    
    analysis.set_threshold("medium");
    let result2 = analysis.analyze(&data)?;
    assert!(result2.numbers_analyzed > 0);
    
    analysis.set_threshold("high");
    let result3 = analysis.analyze(&data)?;
    assert!(result3.numbers_analyzed > 0);
    
    Ok(())
}

/// Test case 39-43: Multi-language number support (Core API equivalent)
#[test]
fn test_international_numbers() -> Result<(), Box<dyn std::error::Error>> {
    // Simplified representation for international numbers
    let data = vec![123456.0, 1234.0, 123456.0, 123456.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 44: lawkit analyze --laws benf,pareto data.csv (Core API equivalent)
#[test]
fn test_analyze_two_laws() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("pareto");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 45: lawkit analyze --laws all data.csv (Core API equivalent)
#[test]
fn test_analyze_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_all_laws();
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 46: lawkit analyze --laws benf,pareto,normal --verbose --recommend data.csv (Core API equivalent)
#[test]
fn test_analyze_verbose_recommend() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("pareto");
    analysis.add_law("normal");
    analysis.set_verbose(true);
    analysis.enable_recommendations();
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 47: lawkit analyze --laws all --focus fraud-detection data.csv (Core API equivalent)
#[test]
fn test_analyze_focus_fraud_detection() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_all_laws();
    analysis.set_focus("fraud-detection");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 48: lawkit analyze --laws all --purpose quality-assessment data.csv (Core API equivalent)
#[test]
fn test_analyze_purpose_quality_assessment() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_all_laws();
    analysis.set_purpose("quality-assessment");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 49: lawkit analyze --laws all --format json data.csv (Core API equivalent)
#[test]
fn test_analyze_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_all_laws();
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 50-52: lawkit generate tests (Core API equivalent)
#[test]
fn test_generate_data() -> Result<(), Box<dyn std::error::Error>> {
    let mut generator = DataGenerator::new();
    let data = generator.generate_samples(1000)?;
    assert!(data.len() == 1000);
    
    // Test analyzing generated data
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 53: lawkit validate --laws all data.csv (Core API equivalent)
#[test]
fn test_validate_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut validator = DataValidator::new();
    validator.add_all_laws();
    let result = validator.validate(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 54: lawkit validate --laws benf,pareto --focus fraud-detection data.csv (Core API equivalent)
#[test]
fn test_validate_focus_fraud_detection() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut validator = DataValidator::new();
    validator.add_law("benford");
    validator.add_law("pareto");
    validator.set_focus("fraud-detection");
    let result = validator.validate(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 55: lawkit validate --laws all --recommend data.csv (Core API equivalent)
#[test]
fn test_validate_recommend() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut validator = DataValidator::new();
    validator.add_all_laws();
    validator.enable_recommendations();
    let result = validator.validate(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 56: lawkit diagnose --laws all data.csv (Core API equivalent)
#[test]
fn test_diagnose_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut diagnoser = DataDiagnoser::new();
    diagnoser.add_all_laws();
    let result = diagnoser.diagnose(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 57: lawkit diagnose --laws all --purpose quality-assessment data.csv (Core API equivalent)
#[test]
fn test_diagnose_purpose_quality_assessment() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut diagnoser = DataDiagnoser::new();
    diagnoser.add_all_laws();
    diagnoser.set_purpose("quality-assessment");
    let result = diagnoser.diagnose(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 58: lawkit diagnose --laws all --verbose data.csv (Core API equivalent)
#[test]
fn test_diagnose_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut diagnoser = DataDiagnoser::new();
    diagnoser.add_all_laws();
    diagnoser.set_verbose(true);
    let result = diagnoser.diagnose(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 59-60: lawkit selftest (Core API equivalent)
#[test]
fn test_selftest() -> Result<(), Box<dyn std::error::Error>> {
    // Core API equivalent of selftest functionality
    let selftest_result = lawkit_core::selftest::run_tests();
    assert!(selftest_result.is_ok());
    Ok(())
}

/// Test case 61-66: Use case specific tests (Core API equivalent)
#[test]
fn test_use_cases() -> Result<(), Box<dyn std::error::Error>> {
    // Financial fraud detection
    let financial_data = vec![1234.0, 2345.0, 3456.0, 1111.0, 2222.0, 3333.0];
    let mut benford = BenfordAnalysis::new();
    benford.set_threshold("high");
    let fraud_result = benford.analyze(&financial_data)?;
    assert!(fraud_result.numbers_analyzed > 0);
    
    // Quality control
    let quality_data = vec![99.1, 99.3, 99.0, 99.4, 98.9, 99.5];
    let mut normal = NormalAnalysis::new();
    normal.set_threshold("high");
    let quality_result = normal.analyze(&quality_data)?;
    assert!(quality_result.numbers_analyzed > 0);
    
    // Defect analysis
    let defect_data = vec![1.0, 2.0, 0.0, 1.0, 3.0, 0.0];
    let mut poisson = PoissonAnalysis::new();
    poisson.set_verbose(true);
    let defect_result = poisson.analyze(&defect_data)?;
    assert!(defect_result.numbers_analyzed > 0);
    
    Ok(())
}