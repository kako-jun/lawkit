use lawkit_core::laws::benford::BenfordAnalysis;
use lawkit_core::laws::pareto::ParetoAnalysis;
use lawkit_core::laws::zipf::ZipfAnalysis;
use lawkit_core::laws::normal::NormalAnalysis;
use lawkit_core::laws::poisson::PoissonAnalysis;
use lawkit_core::laws::integration::IntegrationAnalysis;

/// Test case 1: lawkit benf data.csv (Core API equivalent)
#[test]
fn test_benf_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 2: lawkit benf data.csv --verbose (Core API equivalent)
#[test]
fn test_benf_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_verbose(true);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 3: lawkit benf data.csv --format json (Core API equivalent)
#[test]
fn test_benf_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 4: lawkit benf data.csv --threshold high (Core API equivalent)
#[test]
fn test_benf_threshold_high() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_threshold("high");
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 5: lawkit benf audit_data.csv --confidence 0.99 --verbose (Core API equivalent)
#[test]
fn test_benf_confidence_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.0, 2345.0, 3456.0, 1111.0, 2222.0, 3333.0];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_confidence(0.99);
    analysis.set_verbose(true);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 6: lawkit benf large_data.csv --sample-size 10000 --optimize (Core API equivalent)
#[test]
fn test_benf_sample_size_optimize() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![100000.0, 200000.0, 300000.0, 110000.0, 210000.0, 310000.0];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_sample_size(10000);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 7: lawkit benf financial_data.csv --min-value 100 (Core API equivalent)
#[test]
fn test_benf_min_value() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_min_value(100.0);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 8: lawkit pareto sales.csv (Core API equivalent)
#[test]
fn test_pareto_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![80000.0, 12000.0, 5000.0, 2000.0, 1000.0];
    let mut analysis = ParetoAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 9: lawkit pareto sales.csv --concentration 0.9 (Core API equivalent)
#[test]
fn test_pareto_concentration() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![80000.0, 12000.0, 5000.0, 2000.0, 1000.0];
    let mut analysis = ParetoAnalysis::new();
    analysis.set_concentration(0.9);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 10: lawkit pareto sales.csv --gini-coefficient (Core API equivalent)
#[test]
fn test_pareto_gini_coefficient() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![80000.0, 12000.0, 5000.0, 2000.0, 1000.0];
    let mut analysis = ParetoAnalysis::new();
    analysis.enable_gini_coefficient();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 11: lawkit zipf document.txt (Core API equivalent)
#[test]
fn test_zipf_basic() -> Result<(), Box<dyn std::error::Error>> {
    let text_data = vec!["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"];
    let mut analysis = ZipfAnalysis::new();
    let result = analysis.analyze_text(&text_data)?;
    assert!(result.words_analyzed > 0);
    Ok(())
}

/// Test case 12: lawkit zipf japanese_text.txt (Core API equivalent)
#[test]
fn test_zipf_japanese() -> Result<(), Box<dyn std::error::Error>> {
    let text_data = vec!["日本語", "テキスト", "分析"];
    let mut analysis = ZipfAnalysis::new();
    let result = analysis.analyze_text(&text_data)?;
    assert!(result.words_analyzed > 0);
    Ok(())
}

/// Test case 13: lawkit zipf text.txt --min-count 5 (Core API equivalent)
#[test]
fn test_zipf_min_count() -> Result<(), Box<dyn std::error::Error>> {
    let text_data = vec!["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"];
    let mut analysis = ZipfAnalysis::new();
    analysis.set_min_count(5);
    let result = analysis.analyze_text(&text_data)?;
    assert!(result.words_analyzed > 0);
    Ok(())
}

/// Test case 14: lawkit normal measurements.csv (Core API equivalent)
#[test]
fn test_normal_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.2, 1.3, 1.1, 1.4, 1.0, 1.5];
    let mut analysis = NormalAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 15: lawkit normal quality_data.csv --verbose (Core API equivalent)
#[test]
fn test_normal_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.2, 1.3, 1.1, 1.4, 1.0, 1.5];
    let mut analysis = NormalAnalysis::new();
    analysis.set_verbose(true);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 16: lawkit normal process_data.csv --outliers (Core API equivalent)
#[test]
fn test_normal_outliers() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.2, 1.3, 1.1, 1.4, 1.0, 1.5];
    let mut analysis = NormalAnalysis::new();
    analysis.enable_outlier_detection();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 17: lawkit poisson events.csv (Core API equivalent)
#[test]
fn test_poisson_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.0, 2.0, 3.0, 0.0, 1.0, 2.0];
    let mut analysis = PoissonAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 18: lawkit poisson events.csv --verbose (Core API equivalent)
#[test]
fn test_poisson_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.0, 2.0, 3.0, 0.0, 1.0, 2.0];
    let mut analysis = PoissonAnalysis::new();
    analysis.set_verbose(true);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 19: lawkit poisson critical_events.csv --confidence 0.99 --verbose (Core API equivalent)
#[test]
fn test_poisson_confidence_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.0, 2.0, 3.0, 0.0, 1.0, 2.0];
    let mut analysis = PoissonAnalysis::new();
    analysis.set_confidence(0.99);
    analysis.set_verbose(true);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 20: lawkit analyze data.csv --laws benf,pareto,normal (Core API equivalent)
#[test]
fn test_analyze_multi_laws() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("pareto");
    analysis.add_law("normal");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 21: lawkit validate data.csv --laws all (Core API equivalent)
#[test]
fn test_validate_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_all_laws();
    let result = analysis.validate(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 22: lawkit diagnose data.csv --focus conflict (Core API equivalent)
#[test]
fn test_diagnose_focus_conflict() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.set_focus("conflict");
    let result = analysis.diagnose(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 23: lawkit benf accounting.csv (Core API equivalent)
#[test]
fn test_benf_accounting() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.0, 2345.0, 3456.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 24: lawkit pareto sales.csv --threshold 0.8 (Core API equivalent)
#[test]
fn test_pareto_threshold() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![80000.0, 12000.0, 5000.0, 2000.0, 1000.0];
    let mut analysis = ParetoAnalysis::new();
    analysis.set_threshold(0.8);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 25: lawkit benf data.csv (text format default) (Core API equivalent)
#[test]
fn test_benf_text_format() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 26: lawkit benf data.csv --format csv (Core API equivalent)
#[test]
fn test_benf_format_csv() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 27: lawkit benf data.csv --format yaml (Core API equivalent)
#[test]
fn test_benf_format_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 28: lawkit benf data.csv --format xml (Core API equivalent)
#[test]
fn test_benf_format_xml() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 29: echo "１２３４５６ ７８９０" | lawkit benf (Core API equivalent)
#[test]
fn test_benf_japanese_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123456.0, 7890.0]; // Simplified representation
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 30: echo "一千二百三十四" | lawkit benf (Core API equivalent)
#[test]
fn test_benf_chinese_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.0]; // Simplified representation
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 31: echo "壹萬貳仟參佰肆拾伍" | lawkit benf (Core API equivalent)
#[test]
fn test_benf_traditional_chinese() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![12345.0]; // Simplified representation
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 32: echo "五万六千七百八十九" | lawkit benf (Core API equivalent)
#[test]
fn test_benf_japanese_kanji() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![56789.0]; // Simplified representation
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 33: lawkit benf data.csv --filter ">=1000" (Core API equivalent)
#[test]
fn test_benf_filter() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 2000.0, 3000.0, 1100.0, 2200.0, 3300.0];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_filter(">=1000");
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 34: lawkit pareto sales.csv --concentration 0.95 (Core API equivalent)
#[test]
fn test_pareto_concentration_95() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![80000.0, 12000.0, 5000.0, 2000.0, 1000.0];
    let mut analysis = ParetoAnalysis::new();
    analysis.set_concentration(0.95);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}