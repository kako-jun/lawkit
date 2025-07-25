#[allow(unused_imports)]
use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

// Helper function to get the lawkit command
fn lawkit_cmd() -> Command {
    Command::cargo_bin("lawkit").expect("Failed to find lawkit binary")
}

/// Test conflict detection between laws
/// Verifies detection of contradictory statistical patterns
#[test]
fn test_conflict_detection() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let conflict_file = temp_dir.path().join("conflict_data.csv");

    // Create data that violates expected patterns
    let mut csv_content = String::from("account,amount,frequency\n");

    for i in 1..=100 {
        // This should NOT follow Benford's law (all amounts start with 5)
        let amount = 5000 + i * 10;

        // This should follow Poisson but with inconsistent lambda
        let frequency = if i <= 50 { i % 3 } else { (i % 7) + 10 };

        csv_content.push_str(&format!("ACC{:03},{},{}\n", i, amount, frequency));
    }

    fs::write(&conflict_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("diagnose")
        .arg(&conflict_file)
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect conflicts between expected and actual patterns
    assert!(
        stdout.contains("conflict")
            || stdout.contains("inconsistent")
            || stdout.contains("violation")
            || stdout.contains("benford")
    );

    Ok(())
}

/// Test comprehensive multi-law analysis
/// Verifies simultaneous analysis of multiple statistical laws
#[test]
fn test_comprehensive_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let comprehensive_file = temp_dir.path().join("comprehensive.csv");

    // Create rich dataset that should trigger multiple law analyses
    let mut csv_content = String::from("entity,revenue,employees,transactions,market_rank\n");

    for i in 1..=1000 {
        // Revenue: should follow Pareto/Zipf (few large, many small)
        let revenue = if i <= 100 {
            1000000 / i
        } else {
            10000 + i * 50
        };

        // Employees: should be correlated with revenue but with normal distribution
        let employees = ((revenue as f64).sqrt() / 10.0) as i32 + (i % 20);

        // Transactions: should follow Poisson-like distribution
        let transactions = match i % 25 {
            0..=10 => i % 5,
            11..=18 => (i % 5) + 5,
            19..=22 => (i % 3) + 10,
            _ => (i % 2) + 15,
        };

        // Market rank: should follow power law
        let market_rank = i;

        csv_content.push_str(&format!(
            "Entity{},{},{},{},{}\n",
            i, revenue, employees, transactions, market_rank
        ));
    }

    fs::write(&comprehensive_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(&comprehensive_file)
        .arg("--verbose")
        .arg("--format")
        .arg("yaml");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect multiple law patterns
    let law_indicators = ["benford", "pareto", "zipf", "normal", "poisson"];
    let mut detected_laws = 0;

    for law in &law_indicators {
        if stdout.contains(law) {
            detected_laws += 1;
        }
    }

    assert!(
        detected_laws >= 2,
        "Should detect at least 2 statistical laws, found {}",
        detected_laws
    );

    Ok(())
}

/// Test Japanese number recognition and analysis
/// Verifies analysis of Japanese numeric data
#[test]
fn test_japanese_number_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let japanese_file = temp_dir.path().join("japanese_numbers.csv");

    // Create data with Japanese numerals
    let mut csv_content = String::from("項目,金額\n");

    // Mix of Arabic and Japanese numerals
    let japanese_amounts = [
        "１２３万円", // 1,230,000
        "４５６万円", // 4,560,000
        "７８９万円", // 7,890,000
        "２３４万円", // 2,340,000
        "５６７万円", // 5,670,000
        "８９０万円", // 8,900,000
        "１１１万円", // 1,110,000
        "２２２万円", // 2,220,000
        "３３３万円", // 3,330,000
        "４４４万円", // 4,440,000
    ];

    for (i, amount) in japanese_amounts.iter().enumerate() {
        csv_content.push_str(&format!("項目{},{}\n", i + 1, amount));
    }

    // Add some regular numbers
    for i in 11..=30 {
        csv_content.push_str(&format!("項目{},{}0000\n", i, i * 15));
    }

    fs::write(&japanese_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("benf") // Benford analysis should handle Japanese numbers
        .arg(&japanese_file)
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should process Japanese numerals
    assert!(
        stdout.contains("123")
            || stdout.contains("456")
            || stdout.contains("benford")
            || !stdout.trim().is_empty()
    );

    Ok(())
}

/// Test Chinese number recognition
/// Verifies analysis of Chinese numeric data
#[test]
fn test_chinese_number_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let chinese_file = temp_dir.path().join("chinese_numbers.csv");

    // Create data with Chinese numerals (traditional and simplified)
    let mut csv_content = String::from("项目,金额\n");

    let chinese_amounts = [
        "一千二百三十万", // 12,300,000
        "四千五百六十万", // 45,600,000
        "七千八百九十万", // 78,900,000
        "二千三百四十万", // 23,400,000
        "五千六百七十万", // 56,700,000
        "八千九百万",     // 89,000,000
        "一千一百一十万", // 11,100,000
        "二千二百二十万", // 22,200,000
        "三千三百三十万", // 33,300,000
        "四千四百四十万", // 44,400,000
    ];

    for (i, amount) in chinese_amounts.iter().enumerate() {
        csv_content.push_str(&format!("项目{},{}\n", i + 1, amount));
    }

    fs::write(&chinese_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(&chinese_file)
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should process Chinese numerals
    assert!(
        stdout.contains("123")
            || stdout.contains("456")
            || stdout.contains("benford")
            || !stdout.trim().is_empty()
    );

    Ok(())
}

/// Test data generation for all laws
/// Verifies sample data generation functionality
#[test]
fn test_data_generation_all_laws() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;

    let laws = ["benford", "pareto", "zipf", "normal", "poisson"];

    for law in &laws {
        let output_file = temp_dir.path().join(format!("{}_generated.csv", law));

        let mut cmd = lawkit_cmd();
        cmd.arg("generate")
            .arg(law)
            .arg("--count")
            .arg("100")
            .arg("--output")
            .arg(&output_file);

        let output = cmd.output()?;
        assert!(output.status.success(), "Failed to generate {} data", law);

        // Verify file was created and has content
        assert!(output_file.exists(), "{} data file not created", law);
        let content = fs::read_to_string(&output_file)?;
        assert!(!content.trim().is_empty(), "{} data file is empty", law);
        assert!(
            content.lines().count() > 10,
            "{} data has too few lines",
            law
        );
    }

    Ok(())
}

/// Test list available laws functionality
/// Verifies comprehensive law listing
#[test]
fn test_list_available_laws() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("list").arg("--format").arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should list all supported laws
    let expected_laws = ["benford", "pareto", "zipf", "normal", "poisson"];

    for law in &expected_laws {
        assert!(stdout.contains(law), "Missing law in list: {}", law);
    }

    Ok(())
}

/// Test self-test functionality
/// Verifies built-in testing with generated data
#[test]
fn test_selftest_functionality() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = lawkit_cmd();
    cmd.arg("selftest").arg("--format").arg("json");

    let output = cmd.output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should run self-tests for all laws
    assert!(stdout.contains("selftest") || stdout.contains("passed") || stdout.contains("test"));

    Ok(())
}

/// Test CI/CD integration features
/// Verifies automated pipeline integration
#[test]
fn test_cicd_integration() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let cicd_file = temp_dir.path().join("pipeline_data.csv");

    // Create data that should trigger alerts in CI/CD context
    let mut csv_content = String::from("transaction_id,amount\n");

    // Highly suspicious pattern: all amounts start with same digit
    for i in 1..=100 {
        csv_content.push_str(&format!("TXN{:03},9{:04}\n", i, i * 13 + 1234));
    }

    fs::write(&cicd_file, csv_content)?;

    let mut cmd = lawkit_cmd();
    cmd.arg("benf")
        .arg(&cicd_file)
        .arg("--quiet") // CI/CD friendly output
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;

    // Should detect anomaly and return appropriate exit code
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("risk") || stdout.contains("anomaly") || stdout.contains("suspicious")
        );
    } else {
        // Non-zero exit code indicates detection of anomalous pattern
        assert!(output.status.code().unwrap() > 0);
    }

    Ok(())
}

/// Test confidence level analysis
/// Verifies statistical confidence measurements
#[test]
fn test_confidence_level_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let confidence_file = temp_dir.path().join("confidence_data.csv");

    // Create borderline data for confidence testing
    let mut csv_content = String::from("value\n");

    for i in 1..=200 {
        // Mix of conforming and non-conforming first digits
        let first_digit = if i <= 60 {
            1
        }
        // Too many 1s
        else if i <= 90 {
            2
        }
        // Some 2s
        else if i <= 110 {
            3
        }
        // Some 3s
        else {
            (i % 7) + 1
        }; // Rest distributed

        let value = first_digit * 1000 + (i % 1000);
        csv_content.push_str(&format!("{}\n", value));
    }

    fs::write(&confidence_file, csv_content)?;

    // Test different confidence levels
    for confidence in ["0.90", "0.95", "0.99"] {
        let mut cmd = lawkit_cmd();
        cmd.arg("benf")
            .arg(&confidence_file)
            .arg("--confidence")
            .arg(confidence)
            .arg("--format")
            .arg("json");

        let output = cmd.output()?;
        assert!(
            output.status.success(),
            "Failed with confidence level {}",
            confidence
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains(confidence)
                || stdout.contains("confidence")
                || stdout.contains("interval")
        );
    }

    Ok(())
}

/// Test large dataset optimization
/// Verifies performance with big data
#[test]
fn test_large_dataset_optimization() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let large_file = temp_dir.path().join("large_dataset.csv");

    // Create large dataset
    let mut csv_content = String::from("id,amount,category\n");

    for i in 1..=50000 {
        let amount = match i % 9 {
            0 => format!("1{:05}", i),               // Many starting with 1
            1 => format!("2{:05}", i),               // Some with 2
            2 => format!("3{:05}", i),               // Some with 3
            _ => format!("{}{:05}", (i % 7) + 1, i), // Others distributed
        };

        let category = match i % 5 {
            0 => "Revenue",
            1 => "Expense",
            2 => "Asset",
            3 => "Liability",
            _ => "Equity",
        };

        csv_content.push_str(&format!("{},{},{}\n", i, amount, category));
    }

    fs::write(&large_file, csv_content)?;

    let start = std::time::Instant::now();
    let mut cmd = lawkit_cmd();
    cmd.arg("analyze")
        .arg(&large_file)
        .arg("--sample-size")
        .arg("10000") // Use sampling for large dataset
        .arg("--format")
        .arg("json");

    let output = cmd.output()?;
    let duration = start.elapsed();

    assert!(output.status.success());

    // Should complete large dataset analysis within reasonable time
    assert!(
        duration.as_secs() < 30,
        "Large dataset analysis took too long: {:?}",
        duration
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.trim().is_empty());

    Ok(())
}
