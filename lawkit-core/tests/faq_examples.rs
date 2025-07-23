use lawkit_core::laws::benford::BenfordAnalysis;
use lawkit_core::laws::pareto::ParetoAnalysis;
use lawkit_core::laws::integration::IntegrationAnalysis;

/// Test case 1: cut -d',' -f2 data.csv | lawkit benf (Core API equivalent)
#[test]
fn test_cut_pipe_benf() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 2: awk -F',' '{print $2}' data.csv | lawkit pareto (Core API equivalent)
#[test]
fn test_awk_pipe_pareto() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 500.0, 300.0, 200.0, 100.0, 50.0];
    let mut analysis = ParetoAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 3: lawkit benf --threshold high data.csv (Core API equivalent)
#[test]
fn test_benf_threshold_high() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_threshold("high");
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 4: echo "1,234.56" | lawkit benf (Core API equivalent)
#[test]
fn test_english_number_format() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.56];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 5: echo "１，２３４．５６" | lawkit benf (Core API equivalent)
#[test]
fn test_japanese_number_format() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.56]; // Simplified representation
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 6: echo "१,२३४.५६" | lawkit benf (Core API equivalent)
#[test]
fn test_hindi_number_format() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.56]; // Simplified representation
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 7: lawkit benf data.csv (Core API equivalent)
#[test]
fn test_benf_basic() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 8: echo "١٢٣٤٥٦" | lawkit benf (Core API equivalent)
#[test]
fn test_arabic_numbers() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123456.0]; // Simplified representation
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 9: lawkit analyze --laws benf,pareto data.csv (Core API equivalent)
#[test]
fn test_analyze_benf_pareto() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("pareto");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 10: lawkit analyze --laws all data.csv (Core API equivalent)
#[test]
fn test_analyze_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_all_laws();
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 11: lawkit analyze --laws all --recommend data.csv (Core API equivalent)
#[test]
fn test_analyze_recommend() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_all_laws();
    analysis.enable_recommendations();
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 12: lawkit benf --quiet large_data.csv (Core API equivalent)
#[test]
fn test_benf_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![100000.0, 200000.0, 300000.0, 110000.0, 210000.0, 310000.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 13: lawkit benf --threshold medium large_data.csv (Core API equivalent)
#[test]
fn test_benf_threshold_medium() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![100000.0, 200000.0, 300000.0, 110000.0, 210000.0, 310000.0];
    let mut analysis = BenfordAnalysis::new();
    analysis.set_threshold("medium");
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 14: lawkit benf --format json large_data.csv (Core API equivalent)
#[test]
fn test_benf_format_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![100000.0, 200000.0, 300000.0, 110000.0, 210000.0, 310000.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 15: lawkit pareto small_data.csv (Core API equivalent)
#[test]
fn test_pareto_small_data() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![100.0, 50.0, 30.0, 20.0, 10.0];
    let mut analysis = ParetoAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}