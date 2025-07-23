use lawkit_core::laws::benford::BenfordAnalysis;
use lawkit_core::laws::pareto::ParetoAnalysis;
use lawkit_core::laws::integration::IntegrationAnalysis;

fn create_test_data() -> Vec<f64> {
    vec![1234.0, 5678.0, 9012.0, 3456.0, 7890.0, 2345.0, 6789.0, 1023.0, 4567.0, 8901.0]
}

fn create_sales_data() -> Vec<f64> {
    vec![1000.0, 2000.0, 3000.0, 4000.0, 5000.0, 6000.0, 7000.0, 8000.0, 9000.0, 10000.0]
}

#[test]
fn test_benf_data_analysis() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 1: Core library Benford law analysis
    let data = create_test_data();
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    
    assert!(result.numbers_analyzed > 0);
    assert!(result.first_digit_distribution.len() == 9);
    
    Ok(())
}

#[test]
fn test_pareto_sales_analysis() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 2: Core library Pareto analysis
    let data = create_sales_data();
    let mut analysis = ParetoAnalysis::new();
    let result = analysis.analyze(&data)?;
    
    assert!(result.numbers_analyzed > 0);
    assert!(result.pareto_ratio >= 0.0 && result.pareto_ratio <= 1.0);
    
    Ok(())
}

#[test]
fn test_analyze_multi_laws() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 3: Core library multi-law comparison
    let data = create_test_data();
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benf");
    analysis.add_law("pareto");
    analysis.add_law("normal");
    let result = analysis.analyze(&data)?;
    
    assert!(result.laws_executed >= 3);
    assert!(result.overall_quality_score >= 0.0);
    
    Ok(())
}

#[test]
fn test_analyze_data_basic() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 4: Core library basic analyze
    let data = vec![100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 1000.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benf");
    let result = analysis.analyze(&data)?;
    
    assert!(result.numbers_analyzed > 0);
    assert!(result.laws_executed > 0);
    
    Ok(())
}