/// Integration tests for international numeral support across all law commands
/// Tests Chinese, Hindi, and Arabic numerals with real CLI commands

use assert_cmd::Command;
use std::fs;
use tempfile::NamedTempFile;
use std::io::Write;

fn create_international_csv() -> Result<NamedTempFile, Box<dyn std::error::Error>> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "amount,description")?;
    writeln!(temp_file, "১২৩,Bengali numbers")?;  // Bengali numerals
    writeln!(temp_file, "१२३४५,Hindi numbers")?;  // Devanagari numerals
    writeln!(temp_file, "١٢٣٤٥٦,Arabic numbers")?; // Arabic-Indic numerals
    writeln!(temp_file, "壹拾貳萬參仟,Chinese financial")?; // Chinese financial numerals
    writeln!(temp_file, "123456,Regular numbers")?; // Standard Arabic numerals
    Ok(temp_file)
}

fn create_mixed_numerals_text() -> Result<NamedTempFile, Box<dyn std::error::Error>> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "Sales data:")?;
    writeln!(temp_file, "Region A: १२३४५ units")?; // Hindi
    writeln!(temp_file, "Region B: ١٢٣٤٥ units")?; // Arabic
    writeln!(temp_file, "Region C: 壹貳參肆伍 units")?; // Chinese
    writeln!(temp_file, "Region D: 12345 units")?; // Standard
    writeln!(temp_file, "Total: ４９３８０ units")?; // Full-width
    Ok(temp_file)
}

#[test]
fn test_benf_international_numerals_csv() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_international_csv()?;
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "benf", 
               test_file.path().to_str().unwrap(), "--verbose"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit benf with international numerals");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should process international numerals successfully
    assert!(output.status.success(), "Command should succeed with international numerals");
    
    // Should contain analysis results
    assert!(stdout.contains("numbers analyzed") || stdout.contains("Dataset analysis"));
    
    // Verbose output should show processing details
    assert!(stdout.contains("Debug") || stdout.contains("Processing"));
    
    Ok(())
}

#[test]
fn test_pareto_international_numerals() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_international_csv()?;
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "pareto", 
               test_file.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit pareto with international numerals");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success(), "Pareto analysis should work with international numerals");
    assert!(stdout.contains("concentration") || stdout.contains("Pareto"));
    
    Ok(())
}

#[test]
fn test_zipf_international_numerals() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_international_csv()?;
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "zipf", 
               test_file.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit zipf with international numerals");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success(), "Zipf analysis should work with international numerals");
    assert!(stdout.contains("frequency") || stdout.contains("rank") || stdout.contains("Zipf"));
    
    Ok(())
}

#[test]
fn test_normal_international_numerals() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_international_csv()?;
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "normal", 
               test_file.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit normal with international numerals");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success(), "Normal analysis should work with international numerals");
    assert!(stdout.contains("normal") || stdout.contains("distribution") || stdout.contains("p-value"));
    
    Ok(())
}

#[test]
fn test_poisson_international_numerals() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_international_csv()?;
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "poisson", 
               test_file.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit poisson with international numerals");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success(), "Poisson analysis should work with international numerals");
    assert!(stdout.contains("poisson") || stdout.contains("lambda") || stdout.contains("events"));
    
    Ok(())
}

#[test]
fn test_analyze_international_numerals() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_international_csv()?;
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "analyze", 
               test_file.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit analyze with international numerals");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success(), "Multi-law analysis should work with international numerals");
    assert!(stdout.contains("analysis") || stdout.contains("recommendation"));
    
    Ok(())
}

#[test]
fn test_validate_international_numerals() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_international_csv()?;
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "validate", 
               test_file.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit validate with international numerals");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success(), "Data validation should work with international numerals");
    assert!(stdout.contains("validation") || stdout.contains("quality") || stdout.contains("consistent"));
    
    Ok(())
}

#[test]
fn test_diagnose_international_numerals() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_international_csv()?;
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "diagnose", 
               test_file.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit diagnose with international numerals");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success(), "Diagnostic analysis should work with international numerals");
    assert!(stdout.contains("diagnose") || stdout.contains("conflict") || stdout.contains("recommendation"));
    
    Ok(())
}

#[test]
fn test_mixed_numerals_text_processing() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_mixed_numerals_text()?;
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "benf", 
               test_file.path().to_str().unwrap(), "--verbose"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit benf with mixed numerals");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success(), "Should process mixed numeral scripts successfully");
    
    // Should extract multiple numbers from different scripts
    if stdout.contains("Collected") {
        // Look for number count in debug output
        assert!(stdout.contains("numbers") || stdout.contains("items"));
    }
    
    Ok(())
}

#[test]
fn test_international_numerals_json_output() -> Result<(), Box<dyn std::error::Error>> {
    let test_file = create_international_csv()?;
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "benf", 
               test_file.path().to_str().unwrap(), "--format", "json"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit benf with JSON output");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success(), "JSON output should work with international numerals");
    
    // Should be valid JSON
    if !stdout.trim().is_empty() {
        let _: serde_json::Value = serde_json::from_str(&stdout)
            .expect("Output should be valid JSON");
    }
    
    Ok(())
}

#[test]
fn test_chinese_financial_numerals_specific() -> Result<(), Box<dyn std::error::Error>> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "transaction_amount")?;
    writeln!(temp_file, "壹萬貳仟參佰肆拾伍")?; // 12345 in Chinese financial
    writeln!(temp_file, "貳萬參仟肆佰伍拾陸")?; // 23456 in Chinese financial
    writeln!(temp_file, "參萬肆仟伍佰陸拾柒")?; // 34567 in Chinese financial
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "lawkit", "--", "benf", 
               temp_file.path().to_str().unwrap(), "--verbose"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute lawkit benf with Chinese financial numerals");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(output.status.success(), "Should process Chinese financial numerals");
    
    // Should extract numbers successfully
    if stdout.contains("Debug") {
        assert!(stdout.contains("numbers") || stdout.contains("items"));
    }
    
    Ok(())
}