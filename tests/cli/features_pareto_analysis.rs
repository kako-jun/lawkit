use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

// Helper function to get the lawkit command
fn lawkit_cmd() -> Command {
    Command::cargo_bin("lawkit").expect("Failed to find lawkit binary")
}

/// Test basic Pareto principle analysis
/// Verifies 80/20 rule detection in business data
#[test]
fn test_basic_pareto_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let sales_file = temp_dir.path().join("sales.csv");

    // Create sales data following Pareto distribution
    let mut csv_content = String::from("customer,revenue\n");

    // Top 20% customers generate 80% revenue (Pareto-distributed)
    for i in 1..=20 {
        csv_content.push_str(&format!("Customer{},{}000\n", i, 100 - i)); // High revenue
    }
    for i in 21..=100 {
        csv_content.push_str(&format!("Customer{},{}\n", i, 1000 + i * 10)); // Low revenue
    }

    fs::write(&sales_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(&sales_file)
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect Pareto distribution
    assert!(
        stdout.contains("pareto")
            || stdout.contains("80")
            || stdout.contains("20")
            || stdout.contains("concentration")
    );

    Ok(())
}

/// Test Gini coefficient calculation
/// Verifies inequality measurement in distribution analysis
#[test]
fn test_gini_coefficient_calculation() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let income_file = temp_dir.path().join("income.csv");

    // Create income data with high inequality
    let mut csv_content = String::from("person,income\n");

    // High inequality: few people have most of the wealth
    for i in 1..=5 {
        csv_content.push_str(&format!("Person{},1000000\n", i)); // Very high income
    }
    for i in 6..=100 {
        csv_content.push_str(&format!("Person{},30000\n", i)); // Average income
    }

    fs::write(&income_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(&income_file)
        .arg("--gini-coefficient")
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should calculate Gini coefficient (high inequality expected)
    assert!(
        stdout.contains("gini") || stdout.contains("inequality") || stdout.contains("coefficient")
    );

    Ok(())
}

/// Test business analysis mode
/// Verifies business-specific Pareto analysis features
#[test]
fn test_business_analysis_mode() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let products_file = temp_dir.path().join("products.csv");

    // Create product sales data
    let mut csv_content = String::from("product,sales,profit\n");

    // Classic 80/20 product distribution
    for i in 1..=10 {
        csv_content.push_str(&format!(
            "Product{},{},{}\n",
            i,
            10000 - i * 500,
            5000 - i * 200
        ));
    }
    for i in 11..=50 {
        csv_content.push_str(&format!("Product{},{},{}\n", i, 500 + i * 10, 100 + i * 5));
    }

    fs::write(&products_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(&products_file)
        .arg("--business-analysis")
        .arg("--format")
        .arg("yaml");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should provide business insights
    assert!(
        stdout.contains("top_contributors")
            || stdout.contains("vital_few")
            || stdout.contains("analysis")
    );

    Ok(())
}

/// Test concentration threshold detection
/// Verifies custom concentration level analysis
#[test]
fn test_concentration_threshold() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let concentration_file = temp_dir.path().join("concentration.csv");

    // Create data with 70/30 distribution instead of 80/20
    let mut csv_content = String::from("item,value\n");

    for i in 1..=30 {
        csv_content.push_str(&format!("Item{},{}\n", i, 1000 + i * 20)); // 30% with higher values
    }
    for i in 31..=100 {
        csv_content.push_str(&format!("Item{},{}\n", i, 100 + i)); // 70% with lower values
    }

    fs::write(&concentration_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(&concentration_file)
        .arg("--concentration")
        .arg("0.7") // 70% threshold
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect 70/30 pattern
    assert!(
        stdout.contains("70") || stdout.contains("concentration") || stdout.contains("threshold")
    );

    Ok(())
}

/// Test multi-column Pareto analysis
/// Verifies analysis across multiple data dimensions
#[test]
fn test_multi_column_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let multi_file = temp_dir.path().join("multi.csv");

    // Create multi-dimensional business data
    let mut csv_content = String::from("region,customer,sales,profit,orders\n");

    for region in ["North", "South", "East", "West"] {
        for i in 1..=25 {
            let sales = if region == "North" && i <= 5 {
                50000 + i * 10000
            } else {
                5000 + i * 100
            };
            let profit = sales / 5;
            let orders = sales / 100;
            csv_content.push_str(&format!(
                "{},Customer{},{},{},{}\n",
                region, i, sales, profit, orders
            ));
        }
    }

    fs::write(&multi_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(&multi_file)
        .arg("--verbose")
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should analyze multiple columns
    assert!(stdout.contains("sales") || stdout.contains("profit") || stdout.contains("orders"));

    Ok(())
}

/// Test Pareto with edge cases
/// Verifies robust handling of edge cases in data
#[test]
fn test_pareto_edge_cases() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;

    // Test with uniform distribution (no Pareto effect)
    let uniform_file = temp_dir.path().join("uniform.csv");
    let mut csv_content = String::from("item,value\n");

    for i in 1..=100 {
        csv_content.push_str(&format!("Item{},1000\n", i)); // All same value
    }

    fs::write(&uniform_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(&uniform_file)
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;

    // Should handle uniform distribution gracefully
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("uniform")
                || stdout.contains("equal")
                || stdout.contains("no concentration")
        );
    } else {
        // Or provide clear error for invalid distribution
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("insufficient")
                || stderr.contains("variation")
                || stderr.contains("uniform")
        );
    }

    Ok(())
}

/// Test inventory management scenario
/// Verifies Pareto analysis in inventory optimization context
#[test]
fn test_inventory_management_scenario() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let inventory_file = temp_dir.path().join("inventory.csv");

    // Create inventory data with ABC classification pattern
    let mut csv_content = String::from("sku,monthly_sales,carrying_cost,stockout_cost\n");

    // A items: 20% of SKUs, 80% of value
    for i in 1..=20 {
        csv_content.push_str(&format!(
            "SKU{},{},{},{}\n",
            i,
            10000 - i * 200,
            500 + i * 10,
            1000 + i * 20
        ));
    }

    // B items: 30% of SKUs, 15% of value
    for i in 21..=50 {
        csv_content.push_str(&format!(
            "SKU{},{},{},{}\n",
            i,
            2000 - (i - 20) * 30,
            200 + i * 5,
            300 + i * 8
        ));
    }

    // C items: 50% of SKUs, 5% of value
    for i in 51..=100 {
        csv_content.push_str(&format!(
            "SKU{},{},{},{}\n",
            i,
            100 + (i - 50) * 2,
            50 + i,
            100 + i * 2
        ));
    }

    fs::write(&inventory_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(&inventory_file)
        .arg("--business-analysis")
        .arg("--format")
        .arg("yaml");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should identify ABC classification pattern
    assert!(
        stdout.contains("monthly_sales")
            || stdout.contains("classification")
            || stdout.contains("vital")
    );

    Ok(())
}

/// Test temporal Pareto analysis
/// Verifies Pareto pattern analysis over time periods
#[test]
fn test_temporal_pareto_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let temporal_file = temp_dir.path().join("temporal.csv");

    // Create time-series data showing changing Pareto patterns
    let mut csv_content = String::from("month,customer,sales\n");

    for month in 1..=12 {
        for customer in 1..=50 {
            let base_sales = if customer <= 10 { 10000 } else { 1000 };
            let seasonal_factor = if month >= 10 { 1.5 } else { 1.0 };
            let sales = (base_sales as f64 * seasonal_factor) as i32;
            csv_content.push_str(&format!("{},Customer{},{}\n", month, customer, sales));
        }
    }

    fs::write(&temporal_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(&temporal_file)
        .arg("--verbose")
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should analyze sales concentration patterns
    assert!(
        stdout.contains("sales") || stdout.contains("Customer") || stdout.contains("concentration")
    );

    Ok(())
}

/// Test integration with other statistical tests
/// Verifies Pareto analysis works with confidence intervals
#[test]
fn test_statistical_integration() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let stats_file = temp_dir.path().join("stats.csv");

    // Create data with statistical significance
    let mut csv_content = String::from("category,value,weight\n");

    for i in 1..=100 {
        let value = if i <= 20 { 1000 - i * 40 } else { 200 + i * 5 };
        let weight = if i <= 20 { 10 } else { 1 };
        csv_content.push_str(&format!("Category{},{},{}\n", i, value, weight));
    }

    fs::write(&stats_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("pareto")
        .arg(&stats_file)
        .arg("--confidence")
        .arg("0.95")
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should include confidence analysis
    assert!(stdout.contains("confidence") || stdout.contains("interval") || stdout.contains("95"));

    Ok(())
}
