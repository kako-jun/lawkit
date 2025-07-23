use lawkit_core::laws::benford::BenfordAnalysis;
use lawkit_core::laws::integration::IntegrationAnalysis;

/// Test case 1: lawkit analyze "$file" --laws benford,normal --format json (Core API equivalent)
#[test]
fn test_ci_cd_analyze_with_laws() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.0, 2.0, 3.0, 11.0, 12.0, 13.0, 21.0, 22.0, 31.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("normal");
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 2: lawkit analyze data/financial.csv --laws all --format json (Core API equivalent)
#[test]
fn test_gitlab_analyze_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 2000.0, 3000.0, 1100.0, 1200.0, 1300.0, 2100.0, 2200.0, 3100.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("pareto");
    analysis.add_law("zipf");
    analysis.add_law("normal");
    analysis.add_law("poisson");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 3: lawkit benf data_file --format json (Core API equivalent - Python API)
#[test]
fn test_python_api_benford() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1.0, 2.0, 3.0, 11.0, 12.0, 13.0, 21.0, 22.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 4: lawkit benf dataFile --format json (Core API equivalent - Node.js)
#[test]
fn test_nodejs_benford() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.0, 2345.0, 3456.0, 1111.0, 1222.0, 1333.0, 2111.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 5: lawkit benf /tmp/data.csv --format json (Core API equivalent - PostgreSQL)
#[test]
fn test_postgresql_benford() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1500.0, 2600.0, 3700.0, 1800.0, 1900.0, 2000.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 6: lawkit analyze data_source --laws all --format json (Core API equivalent - Tableau)
#[test]
fn test_tableau_analyze_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 2000.0, 3000.0, 1100.0, 1200.0, 2100.0, 3100.0, 4000.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("pareto");
    analysis.add_law("zipf");
    analysis.add_law("normal");
    analysis.add_law("poisson");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 7: lawkit benf temp_file --format json (Core API equivalent - Power BI)
#[test]
fn test_powerbi_benford() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![15000.0, 26000.0, 37000.0, 18000.0, 19000.0, 20000.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 8: lawkit benf /tmp/data.csv --format json (Core API equivalent - AWS Lambda)
#[test]
fn test_aws_lambda_benford() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![12000.0, 23000.0, 34000.0, 11000.0, 12500.0, 21000.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 9: lawkit analyze /tmp/data.csv --laws all --format json (Core API equivalent - GCP)
#[test]
fn test_gcp_analyze_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10000.0, 20000.0, 30000.0, 11000.0, 12000.0, 21000.0, 31000.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("pareto");
    analysis.add_law("zipf");
    analysis.add_law("normal");
    analysis.add_law("poisson");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 10: lawkit benf data_file --format json (Core API equivalent - Prometheus)
#[test]
fn test_prometheus_benford() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1500.0, 2600.0, 3700.0, 1800.0, 1900.0, 2000.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 11: lawkit benf file_path --format json (Core API equivalent - Rust custom)
#[test]
fn test_rust_custom_benford() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1234.0, 2345.0, 3456.0, 1567.0, 1678.0, 2789.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}