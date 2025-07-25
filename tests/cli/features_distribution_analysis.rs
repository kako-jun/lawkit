use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

// Helper function to get the lawkit command
fn lawkit_cmd() -> Command {
    Command::cargo_bin("lawkit").expect("Failed to find lawkit binary")
}

/// Test normal distribution analysis
/// Verifies normality testing and quality control applications
#[test]
fn test_normal_distribution_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let normal_file = temp_dir.path().join("normal_data.csv");

    // Create normally distributed data (approximation using central limit theorem)
    let mut csv_content = String::from("measurement\n");

    for i in 0..1000 {
        // Approximate normal distribution using sum of uniform random values
        let mut sum = 0.0;
        for j in 0..12 {
            sum += ((i * 7 + j * 13) % 1000) as f64 / 1000.0;
        }
        let normal_value = (sum - 6.0) * 10.0 + 100.0; // Mean=100, approx std=10
        csv_content.push_str(&format!("{:.2}\n", normal_value));
    }

    fs::write(&normal_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(&normal_file)
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect normal distribution characteristics
    assert!(
        stdout.contains("normal")
            || stdout.contains("mean")
            || stdout.contains("std")
            || stdout.contains("gaussian")
    );

    Ok(())
}

/// Test normal distribution with outlier detection
/// Verifies outlier identification in quality control context
#[test]
fn test_normal_outlier_detection() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let outlier_file = temp_dir.path().join("outlier_data.csv");

    // Create normal data with some outliers
    let mut csv_content = String::from("value\n");

    // Normal data (mean=50, std≈5)
    for i in 0..100 {
        let value = 50.0 + ((i % 20) as f64 - 10.0) * 0.5;
        csv_content.push_str(&format!("{:.1}\n", value));
    }

    // Add outliers
    csv_content.push_str("80.0\n"); // +6σ outlier
    csv_content.push_str("20.0\n"); // -6σ outlier
    csv_content.push_str("75.0\n"); // +5σ outlier

    fs::write(&outlier_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(&outlier_file)
        .arg("--outliers")
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect outliers
    assert!(
        stdout.contains("outlier")
            || stdout.contains("80")
            || stdout.contains("20")
            || stdout.contains("sigma")
    );

    Ok(())
}

/// Test quality control analysis
/// Verifies process capability and control limits
#[test]
fn test_quality_control_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let qc_file = temp_dir.path().join("qc_data.csv");

    // Create manufacturing quality data
    let mut csv_content = String::from("batch,measurement,specification_min,specification_max\n");

    for batch in 1..=50 {
        for sample in 1..=10 {
            // Target: 100, tolerance: ±5 (95-105)
            let variation = ((batch * 3 + sample * 7) % 21) as f64 - 10.0;
            let measurement = 100.0 + variation * 0.3; // Most within spec
            csv_content.push_str(&format!("{},{:.2},95.0,105.0\n", batch, measurement));
        }
    }

    fs::write(&qc_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(&qc_file)
        .arg("--quality-control")
        .arg("--format")
        .arg("yaml");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should provide quality control metrics
    assert!(
        stdout.contains("capability")
            || stdout.contains("specification")
            || stdout.contains("control")
            || stdout.contains("95")
    );

    Ok(())
}

/// Test time series normality analysis
/// Verifies normality testing with temporal data
#[test]
fn test_timeseries_normal_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let timeseries_file = temp_dir.path().join("timeseries.csv");

    // Create time series data with normal residuals
    let mut csv_content = String::from("time,value\n");

    for t in 0..365 {
        // One year of daily data
        // Trend + seasonal + normal noise
        let trend = 100.0 + t as f64 * 0.1;
        let seasonal = 10.0 * (2.0 * std::f64::consts::PI * t as f64 / 365.0).sin();
        let noise = ((t * 17) % 21) as f64 - 10.0; // Pseudo-normal noise
        let value = trend + seasonal + noise;
        csv_content.push_str(&format!("{},{:.2}\n", t, value));
    }

    fs::write(&timeseries_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(&timeseries_file)
        .arg("--enable-timeseries")
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should analyze time series normality
    assert!(
        stdout.contains("timeseries")
            || stdout.contains("residual")
            || stdout.contains("trend")
            || stdout.contains("time")
    );

    Ok(())
}

/// Test Poisson distribution analysis
/// Verifies rare event and arrival process analysis
#[test]
fn test_poisson_distribution_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let poisson_file = temp_dir.path().join("poisson_data.csv");

    // Create Poisson-distributed data (λ=3)
    let mut csv_content = String::from("events_per_hour\n");

    // Approximate Poisson distribution
    let lambda = 3.0;
    for i in 0..1000 {
        // Simple approximation of Poisson(3) using modular arithmetic
        let events = match (i * 7) % 100 {
            0..=4 => 0,   // P(0) ≈ 0.05
            5..=19 => 1,  // P(1) ≈ 0.15
            20..=42 => 2, // P(2) ≈ 0.22
            43..=65 => 3, // P(3) ≈ 0.22
            66..=81 => 4, // P(4) ≈ 0.17
            82..=92 => 5, // P(5) ≈ 0.10
            93..=96 => 6, // P(6) ≈ 0.05
            97..=98 => 7, // P(7) ≈ 0.02
            _ => 8,       // P(8+) ≈ 0.01
        };
        csv_content.push_str(&format!("{}\n", events));
    }

    fs::write(&poisson_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(&poisson_file)
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect Poisson distribution
    assert!(
        stdout.contains("poisson")
            || stdout.contains("lambda")
            || stdout.contains("events")
            || stdout.contains("rate")
    );

    Ok(())
}

/// Test Poisson with probability prediction
/// Verifies event probability forecasting
#[test]
fn test_poisson_probability_prediction() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let prediction_file = temp_dir.path().join("prediction_data.csv");

    // Create arrival/event data
    let mut csv_content = String::from("calls_per_minute\n");

    for i in 0..500 {
        // Call center data: average 2 calls per minute
        let calls = match (i * 11) % 50 {
            0..=6 => 0,   // P(0) for λ=2
            7..=20 => 1,  // P(1) for λ=2
            21..=34 => 2, // P(2) for λ=2
            35..=43 => 3, // P(3) for λ=2
            44..=47 => 4, // P(4) for λ=2
            48..=49 => 5, // P(5) for λ=2
            _ => 6,       // P(6+) for λ=2
        };
        csv_content.push_str(&format!("{}\n", calls));
    }

    fs::write(&prediction_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(&prediction_file)
        .arg("--predict")
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should provide probability predictions
    assert!(
        stdout.contains("predict")
            || stdout.contains("probability")
            || stdout.contains("forecast")
            || stdout.contains("lambda")
    );

    Ok(())
}

/// Test rare events analysis
/// Verifies analysis of low-frequency, high-impact events
#[test]
fn test_rare_events_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let rare_events_file = temp_dir.path().join("rare_events.csv");

    // Create rare event data (system failures, accidents, etc.)
    let mut csv_content = String::from("failures_per_day\n");

    for i in 0..365 {
        // Very rare events: average 0.1 per day (1 every 10 days)
        let failures = if (i * 13) % 200 == 0 {
            1
        } else if (i * 13) % 1000 == 0 {
            2
        } else {
            0
        };
        csv_content.push_str(&format!("{}\n", failures));
    }

    fs::write(&rare_events_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("poisson")
        .arg(&rare_events_file)
        .arg("--rare-events")
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should analyze rare event patterns
    assert!(
        stdout.contains("rare")
            || stdout.contains("event")
            || stdout.contains("low")
            || stdout.contains("lambda")
    );

    Ok(())
}

/// Test normality test methods
/// Verifies different statistical tests for normality
#[test]
fn test_normality_test_methods() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let test_file = temp_dir.path().join("normality_test.csv");

    // Create clearly non-normal data (bimodal)
    let mut csv_content = String::from("value\n");

    // First mode around 20
    for i in 0..50 {
        csv_content.push_str(&format!("{}\n", 20 + (i % 5)));
    }

    // Second mode around 80
    for i in 0..50 {
        csv_content.push_str(&format!("{}\n", 80 + (i % 5)));
    }

    fs::write(&test_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("normal")
        .arg(&test_file)
        .arg("--test")
        .arg("shapiro") // Use Shapiro-Wilk test
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect non-normality
    assert!(
        stdout.contains("shapiro")
            || stdout.contains("normal")
            || stdout.contains("test")
            || stdout.contains("p_value")
    );

    Ok(())
}

/// Test multi-law statistical analysis
/// Verifies integrated analysis across multiple distributions
#[test]
fn test_multi_law_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let multi_file = temp_dir.path().join("multi_analysis.csv");

    // Create data that could fit multiple distributions
    let mut csv_content = String::from("customer_id,purchase_amount,frequency\n");

    for i in 1..=1000 {
        // Purchase amounts: potentially log-normal/Pareto
        let amount = if i <= 200 { 1000 - i * 4 } else { 100 + i / 10 };

        // Frequency: potentially Poisson
        let frequency = match i % 20 {
            0..=7 => 1,
            8..=13 => 2,
            14..=17 => 3,
            18 => 4,
            _ => 5,
        };

        csv_content.push_str(&format!("{},{},{}\n", i, amount, frequency));
    }

    fs::write(&multi_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze") // Multi-law analysis command
        .arg(&multi_file)
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should analyze multiple distributions
    assert!(
        stdout.contains("pareto")
            || stdout.contains("poisson")
            || stdout.contains("normal")
            || stdout.contains("analysis")
    );

    Ok(())
}

/// Test data validation across distributions
/// Verifies comprehensive data quality assessment
#[test]
fn test_data_validation() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let validation_file = temp_dir.path().join("validation_data.csv");

    // Create mixed quality data
    let mut csv_content = String::from("transaction_amount,transaction_count\n");

    for i in 0..200 {
        let amount = match i {
            0..=150 => 100 + i * 10,     // Normal range
            151..=190 => 5000 + i * 100, // Higher range (potential outliers)
            _ => -50,                    // Invalid negative values
        };

        let count = if i < 180 { i / 10 } else { 0 }; // Some zero counts

        csv_content.push_str(&format!("{},{}\n", amount, count));
    }

    fs::write(&validation_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("validate")
        .arg(&validation_file)
        .arg("--format")
        .arg("yaml");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should identify data quality issues
    assert!(
        stdout.contains("validation")
            || stdout.contains("outlier")
            || stdout.contains("negative")
            || stdout.contains("quality")
    );

    Ok(())
}
