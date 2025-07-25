use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;

fn lawkit_cmd() -> Command {
    Command::cargo_bin("lawkit").expect("Failed to find lawkit binary")
}

fn create_temp_csv(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    write!(file, "{}", content).expect("Failed to write to temp file");
    file
}

#[test]
fn test_benf_basic_analysis() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 1: Basic Benford Law analysis from README
    let csv_content = "amount\n1234\n5678\n9012\n3456\n7890\n2345\n6789\n1023\n4567\n8901";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Benford Law Analysis Results"));

    Ok(())
}

#[test]
fn test_pareto_sales_analysis() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 2: Pareto analysis from README
    let csv_content = "sales\n1000\n2000\n3000\n4000\n5000\n6000\n7000\n8000\n9000\n10000";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("pareto").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Pareto Principle"));

    Ok(())
}

#[test]
fn test_analyze_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 3: Multi-law integration analysis
    let csv_content = "data\n123\n456\n789\n1011\n1213\n1415\n1617\n1819\n2021\n2223";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg("--laws")
        .arg("all")
        .arg(temp_file.path());
    cmd.assert().success().stdout(predicates::str::contains(
        "Statistical Laws Integration Analysis",
    ));

    Ok(())
}

#[test]
fn test_benf_performance_benchmark() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 4: Performance benchmark example
    let csv_content = "value\n1234\n5678\n9012\n3456\n7890";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Numbers analyzed"));

    Ok(())
}

#[test]
fn test_analyze_basic_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 5: Basic analyze command
    let csv_content = "numbers\n100\n200\n300\n400\n500";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze").arg(temp_file.path());
    cmd.assert().success();

    Ok(())
}

#[test]
fn test_zipf_usage_example() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 6: Zipf law analysis
    let csv_content = "frequency\n1000\n500\n333\n250\n200\n166\n142\n125\n111\n100";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("zipf").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Zipf"));

    Ok(())
}

#[test]
fn test_normal_distribution_analysis() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 7: Normal distribution analysis
    let csv_content = "values\n10\n12\n14\n16\n18\n20\n22\n24\n26\n28";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("normal").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Normal"));

    Ok(())
}

#[test]
fn test_poisson_distribution_analysis() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 8: Poisson distribution analysis
    let csv_content = "events\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("poisson").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Poisson"));

    Ok(())
}

#[test]
fn test_generate_sample_data() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 9: Generate sample data
    let mut cmd = lawkit_cmd();
    cmd.arg("generate")
        .arg("--law")
        .arg("benf")
        .arg("--count")
        .arg("100");
    cmd.assert().success();

    Ok(())
}

#[test]
fn test_validate_data_integrity() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 10: Data validation
    let csv_content = "amounts\n1234\n5678\n9012\n3456\n7890";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("validate").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("validation"));

    Ok(())
}

#[test]
fn test_diagnose_conflicts() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 11: Conflict diagnosis
    let csv_content = "data\n123\n456\n789\n1011\n1213";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("diagnose").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("conflicts"));

    Ok(())
}

#[test]
fn test_list_available_laws() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 12: List available statistical laws
    let mut cmd = lawkit_cmd();
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("laws"));

    Ok(())
}

#[test]
fn test_selftest_execution() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 13: Self-test for all laws
    let mut cmd = lawkit_cmd();
    cmd.arg("selftest");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("test"));

    Ok(())
}

#[test]
fn test_analyze_specific_laws() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 14: Analyze with specific laws
    let csv_content = "values\n100\n200\n300\n400\n500";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg("--laws")
        .arg("benf,pareto")
        .arg(temp_file.path());
    cmd.assert().success();

    Ok(())
}

#[test]
fn test_benf_with_visual_charts() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 15: Benford analysis with visual output
    let csv_content = "financial\n1234\n2345\n3456\n4567\n5678\n6789\n7890\n8901\n9012\n1023";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("First Digit Distribution"));

    Ok(())
}

#[test]
fn test_pareto_lorenz_curve() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 16: Pareto with Lorenz curve
    let csv_content =
        "wealth\n10000\n20000\n30000\n40000\n50000\n60000\n70000\n80000\n90000\n100000";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("pareto").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("80/20 Rule"));

    Ok(())
}

#[test]
fn test_analyze_integration_metrics() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 17: Integration analysis with metrics
    let csv_content = "dataset\n123\n234\n345\n456\n567\n678\n789\n890\n901\n012";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("analyzed"));

    Ok(())
}

#[test]
fn test_risk_assessment() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 18: Risk level assessment
    let csv_content = "transactions\n1111\n2222\n3333\n4444\n5555";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Risk Level"));

    Ok(())
}

#[test]
fn test_statistical_tests() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 19: Statistical test results
    let csv_content = "amounts\n1000\n2000\n3000\n4000\n5000";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Statistical Tests"));

    Ok(())
}

#[test]
fn test_chi_square_test() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 20: Chi-square test output
    let csv_content = "data\n123\n456\n789\n101\n112\n131";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Chi-square"));

    Ok(())
}

#[test]
fn test_mean_absolute_deviation() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 21: Mean Absolute Deviation test
    let csv_content = "numbers\n1234\n5678\n9012\n3456\n7890";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("benf").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Mean Absolute Deviation"));

    Ok(())
}

#[test]
fn test_cumulative_distribution() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 22: Cumulative distribution analysis
    let csv_content = "values\n10\n20\n30\n40\n50\n60\n70\n80\n90\n100";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("pareto").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("cumulative"));

    Ok(())
}

#[test]
fn test_quality_score_metrics() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 23: Quality score calculation
    let csv_content = "metrics\n100\n200\n300\n400\n500";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Quality Score"));

    Ok(())
}

#[test]
fn test_consistency_score() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 24: Consistency score analysis
    let csv_content = "consistency\n111\n222\n333\n444\n555";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Consistency Score"));

    Ok(())
}

#[test]
fn test_conflicts_detection() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 25: Conflicts detection in analysis
    let csv_content = "conflicts\n123\n456\n789\n101\n112";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Conflicts Detected"));

    Ok(())
}

#[test]
fn test_recommendation_confidence() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 26: Recommendation confidence level
    let csv_content = "recommendations\n1000\n2000\n3000\n4000\n5000";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze").arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Recommendation Confidence"));

    Ok(())
}

#[test]
fn test_laws_executed_count() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 27: Count of laws executed
    let csv_content = "execution\n111\n222\n333\n444\n555";
    let temp_file = create_temp_csv(csv_content);

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg("--laws")
        .arg("benf,pareto,zipf")
        .arg(temp_file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Laws Executed"));

    Ok(())
}

#[test]
fn test_help_command() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 28: Help command usage
    let mut cmd = lawkit_cmd();
    cmd.arg("--help");
    cmd.assert().success().stdout(predicates::str::contains(
        "Statistical law analysis toolkit",
    ));

    Ok(())
}

#[test]
fn test_version_command() -> Result<(), Box<dyn std::error::Error>> {
    // Test case 29: Version command usage
    let mut cmd = lawkit_cmd();
    cmd.arg("--version");
    cmd.assert().success();

    Ok(())
}
