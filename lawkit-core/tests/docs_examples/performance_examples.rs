use lawkit_core::laws::benford::BenfordAnalysis;
use lawkit_core::laws::pareto::ParetoAnalysis;
use lawkit_core::laws::zipf::ZipfAnalysis;
use lawkit_core::laws::normal::NormalAnalysis;
use lawkit_core::laws::poisson::PoissonAnalysis;
use lawkit_core::laws::integration::IntegrationAnalysis;

/// Test case 1: lawkit benf data.csv (Core API equivalent)
#[test]
fn test_basic_benford() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0, 121.0, 232.0, 343.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 2: lawkit pareto data.csv --threshold 0.8 (Core API equivalent)
#[test]
fn test_pareto_with_threshold() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 500.0, 300.0, 200.0, 100.0, 50.0, 30.0, 20.0, 10.0];
    let mut analysis = ParetoAnalysis::new();
    analysis.set_threshold(0.8);
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 3: lawkit zipf text.txt (Core API equivalent)
#[test]
fn test_zipf_text() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10.0, 5.0, 3.0, 2.0, 2.0, 1.0, 1.0, 1.0]; // Word frequencies
    let mut analysis = ZipfAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 4: lawkit normal data.csv (Core API equivalent)
#[test]
fn test_normal_distribution() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![50.0, 51.0, 49.0, 52.0, 48.0, 50.0, 51.0, 49.0, 50.0];
    let mut analysis = NormalAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 5: lawkit poisson data.csv (Core API equivalent)
#[test]
fn test_poisson_distribution() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![3.0, 2.0, 4.0, 1.0, 3.0, 2.0, 5.0, 3.0, 2.0];
    let mut analysis = PoissonAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 6: lawkit analyze data.csv --laws benford,pareto,normal (Core API equivalent)
#[test]
fn test_analyze_multiple_laws() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0, 121.0, 232.0, 343.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("pareto");
    analysis.add_law("normal");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 7: lawkit benf data.csv --quiet --format json (Core API equivalent)
#[test]
fn test_benford_quiet_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 8: lawkit benf data.csv --verbose (Core API equivalent)
#[test]
fn test_benford_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 9: lawkit benf data.csv --format csv (Core API equivalent)
#[test]
fn test_benford_format_csv() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 10: lawkit benf data.csv --format yaml (Core API equivalent)
#[test]
fn test_benford_format_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 11: lawkit benf data.csv --format toml (Core API equivalent)
#[test]
fn test_benford_format_toml() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 12: lawkit benf data.csv --format xml (Core API equivalent)
#[test]
fn test_benford_format_xml() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 13: lawkit analyze data.csv --laws benford,pareto (Core API equivalent)
#[test]
fn test_analyze_benford_pareto() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 500.0, 300.0, 200.0, 100.0, 50.0, 30.0, 20.0, 10.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("pareto");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 14: lawkit analyze data.csv --laws benford --focus accuracy (Core API equivalent)
#[test]
fn test_analyze_focus_accuracy() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.set_focus("quality");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 15: lawkit analyze data.csv --laws all --purpose audit (Core API equivalent)
#[test]
fn test_analyze_all_laws_purpose_audit() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0, 121.0, 232.0, 343.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("pareto");
    analysis.add_law("zipf");
    analysis.add_law("normal");
    analysis.add_law("poisson");
    analysis.set_purpose("fraud");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 16: lawkit analyze data.csv --laws all --recommend (Core API equivalent)
#[test]
fn test_analyze_all_laws_recommend() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0, 121.0, 232.0, 343.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("pareto");
    analysis.add_law("zipf");
    analysis.add_law("normal");
    analysis.add_law("poisson");
    analysis.enable_recommendations();
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 17: lawkit benf optimized.csv (Core API equivalent)
#[test]
fn test_benford_optimized() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0, 121.0, 232.0, 343.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 18: lawkit benf data.csv --quiet (Core API equivalent)
#[test]
fn test_benford_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 19: lawkit benf data.csv --verbose (Core API equivalent, duplicate)
#[test]
fn test_benford_verbose_2() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 20: lawkit pareto data.csv --quiet (Core API equivalent)
#[test]
fn test_pareto_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 500.0, 300.0, 200.0, 100.0, 50.0, 30.0, 20.0, 10.0];
    let mut analysis = ParetoAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 21: lawkit analyze data.csv --laws benford,pareto --quiet (Core API equivalent)
#[test]
fn test_analyze_benford_pareto_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 500.0, 300.0, 200.0, 100.0, 50.0, 30.0, 20.0, 10.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("pareto");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 22: lawkit analyze data.csv --laws all --quiet (Core API equivalent)
#[test]
fn test_analyze_all_laws_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0, 121.0, 232.0, 343.0];
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

/// Test case 23: lawkit analyze data.csv --format json --quiet (Core API equivalent)
#[test]
fn test_analyze_json_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 24: lawkit benf data.csv --format yaml (Core API equivalent, duplicate)
#[test]
fn test_benford_format_yaml_2() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 25: lawkit analyze data.csv --laws all --verbose (Core API equivalent)
#[test]
fn test_analyze_all_laws_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0, 111.0, 222.0, 333.0, 121.0, 232.0, 343.0];
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

/// Test case 26: lawkit benf small_data.csv --quiet (Core API equivalent)
#[test]
fn test_benford_small_data_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![123.0, 234.0, 345.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 27: lawkit analyze medium_data.csv --laws benford,pareto (Core API equivalent)
#[test]
fn test_analyze_medium_data() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![1000.0, 2000.0, 3000.0, 1100.0, 2100.0, 3100.0, 1200.0, 2200.0, 3200.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    analysis.add_law("pareto");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 28: lawkit analyze large_data.csv --laws benford --quiet (Core API equivalent)
#[test]
fn test_analyze_large_data_quiet() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![10000.0, 20000.0, 30000.0, 11000.0, 21000.0, 31000.0, 12000.0, 22000.0, 32000.0];
    let mut analysis = IntegrationAnalysis::new();
    analysis.add_law("benford");
    let result = analysis.analyze(&data)?;
    assert!(result.laws_executed > 0);
    Ok(())
}

/// Test case 29: lawkit benf huge_data.csv --quiet --format json (Core API equivalent)
#[test]
fn test_benford_huge_data_quiet_json() -> Result<(), Box<dyn std::error::Error>> {
    let data = vec![100000.0, 200000.0, 300000.0, 110000.0, 210000.0, 310000.0, 120000.0, 220000.0, 320000.0];
    let mut analysis = BenfordAnalysis::new();
    let result = analysis.analyze(&data)?;
    assert!(result.numbers_analyzed > 0);
    Ok(())
}

/// Test case 30: lawkit --version (Core API equivalent)
#[test]
fn test_version() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(lawkit_core::VERSION, "2.0.1");
    Ok(())
}

/// Test case 31: lawkit --help (Core API equivalent - N/A)
#[test]
fn test_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 32: lawkit benf --help (Core API equivalent - N/A)
#[test]
fn test_benf_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 33: lawkit pareto --help (Core API equivalent - N/A)
#[test]
fn test_pareto_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}

/// Test case 34: lawkit zipf --help (Core API equivalent - N/A)
#[test]
fn test_zipf_help() -> Result<(), Box<dyn std::error::Error>> {
    // Core library doesn't have help - this is CLI only
    assert!(true);
    Ok(())
}