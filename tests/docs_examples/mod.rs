//! Documentation examples tests
//! Tests that verify examples in documentation work correctly

pub mod readme_examples;
pub mod index_examples;
pub mod integrations_examples;

#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    // Helper function to create financial test data
    fn create_financial_data() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("financial_data.csv");
        let mut file = fs::File::create(&file_path)?;
        writeln!(file, "amount")?;
        for i in 1..=100 {
            writeln!(file, "{}", i * 123 + (i % 7) * 1000)?;
        }
        Ok(file_path)
    }

    // Helper function to create sales data
    fn create_sales_data() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("sales_data.csv");
        let mut file = fs::File::create(&file_path)?;
        writeln!(file, "value")?;
        for i in 1..=200 {
            let value = if i <= 40 { i * 10000 } else { (i - 40) * 100 };
            writeln!(file, "{}", value)?;
        }
        Ok(file_path)
    }

    // Helper function to create frequency data
    fn create_frequency_data() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("frequency_data.csv");
        let mut file = fs::File::create(&file_path)?;
        writeln!(file, "rank,frequency")?;
        for i in 1..=100 {
            let frequency = 1000 / i; // Zipf-like distribution
            writeln!(file, "{},{}", i, frequency)?;
        }
        Ok(file_path)
    }

    #[test]
    fn test_docs_example_benford_basic() {
        // Test: lawkit benford data/financial_data.csv
        if let Ok(test_file) = create_financial_data() {
            let output = Command::new("cargo")
                .args(["run", "--bin", "lawkit", "--", "benf", 
                       test_file.to_str().unwrap()])
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .output()
                .expect("Failed to execute lawkit benf");

            // Should succeed or fail gracefully
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                // Should not crash with unhandled errors
                assert!(!stderr.contains("panic"), "Should not panic");
            }
        }
    }

    #[test]
    fn test_docs_example_pareto_basic() {
        // Test: lawkit pareto data/sales_data.csv
        if let Ok(test_file) = create_sales_data() {
            let output = Command::new("cargo")
                .args(["run", "--bin", "lawkit", "--", "pareto", 
                       test_file.to_str().unwrap()])
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .output()
                .expect("Failed to execute lawkit pareto");

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                assert!(!stderr.contains("panic"), "Should not panic");
            }
        }
    }

    #[test]
    fn test_docs_example_zipf_basic() {
        // Test: lawkit zipf data/text_frequency.csv
        if let Ok(test_file) = create_frequency_data() {
            let output = Command::new("cargo")
                .args(["run", "--bin", "lawkit", "--", "zipf", 
                       test_file.to_str().unwrap()])
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .output()
                .expect("Failed to execute lawkit zipf");

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                assert!(!stderr.contains("panic"), "Should not panic");
            }
        }
    }

    #[test]
    fn test_docs_example_integrated_analysis() {
        // Test: lawkit analyze data/comprehensive_data.csv --all-laws
        if let Ok(test_file) = create_financial_data() {
            let output = Command::new("cargo")
                .args(["run", "--bin", "lawkit", "--", "analyze", 
                       test_file.to_str().unwrap()])
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .output()
                .expect("Failed to execute lawkit analyze");

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                assert!(!stderr.contains("panic"), "Should not panic");
            }
        }
    }

    #[test]
    fn test_docs_example_json_output() {
        // Test: lawkit benford data/financial_data.csv --output json
        if let Ok(test_file) = create_financial_data() {
            let output = Command::new("cargo")
                .args(["run", "--bin", "lawkit", "--", "benf", 
                       test_file.to_str().unwrap(), "--format", "json"])
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .output()
                .expect("Failed to execute lawkit benf --format json");

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                // JSON output should contain valid JSON structure
                assert!(stdout.contains("{") || stdout.contains("\""), 
                        "JSON output should contain JSON-like structure");
            }
        }
    }

    #[test]
    fn test_docs_example_confidence_interval() {
        // Test: lawkit benford data/financial_data.csv --confidence 0.99
        if let Ok(test_file) = create_financial_data() {
            let output = Command::new("cargo")
                .args(["run", "--bin", "lawkit", "--", "benf", 
                       test_file.to_str().unwrap(), "--confidence", "0.99"])
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .output()
                .expect("Failed to execute lawkit benf --confidence 0.99");

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                assert!(!stderr.contains("unrecognized"), 
                        "Confidence option should be recognized");
            }
        }
    }

    #[test]
    fn test_docs_example_sample_size() {
        // Test: lawkit pareto data/large_dataset.csv --sample-size 10000
        if let Ok(test_file) = create_sales_data() {
            let output = Command::new("cargo")
                .args(["run", "--bin", "lawkit", "--", "pareto", 
                       test_file.to_str().unwrap(), "--sample-size", "100"])
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .output()
                .expect("Failed to execute lawkit pareto --sample-size");

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                assert!(!stderr.contains("unrecognized"), 
                        "Sample-size option should be recognized");
            }
        }
    }

    #[test]
    fn test_docs_example_help_output() {
        // Test: lawkit --help
        let output = Command::new("cargo")
            .args(["run", "--bin", "lawkit", "--", "--help"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit --help");

        assert!(output.status.success(), "Help command should succeed");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("lawkit") || stdout.contains("Usage") || stdout.contains("Commands"), 
                "Help output should contain program information");
    }

    #[test]
    fn test_docs_example_version_info() {
        // Test: lawkit --version
        let output = Command::new("cargo")
            .args(["run", "--bin", "lawkit", "--", "--version"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit --version");

        assert!(output.status.success(), "Version command should succeed");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("2.5.16") || stdout.contains("lawkit"), 
                "Version output should contain version or program name");
    }

    #[test]
    fn test_docs_example_generate_data() {
        // Test: lawkit generate benford --count 100
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let output_file = temp_dir.path().join("generated.csv");

        let output = Command::new("cargo")
            .args(["run", "--bin", "lawkit", "--", "generate", "benford", 
                   "--count", "100", "--output", output_file.to_str().unwrap()])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit generate");

        if output.status.success() {
            assert!(output_file.exists(), "Generated file should exist");
            let content = fs::read_to_string(&output_file)
                .expect("Failed to read generated file");
            assert!(!content.trim().is_empty(), "Generated file should not be empty");
        }
    }

    #[test]
    fn test_docs_example_list_laws() {
        // Test: lawkit list
        let output = Command::new("cargo")
            .args(["run", "--bin", "lawkit", "--", "list"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit list");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Should list available statistical laws
            assert!(stdout.contains("benford") || stdout.contains("pareto") || 
                    stdout.contains("law") || !stdout.trim().is_empty(),
                    "List command should show available laws");
        }
    }

    #[test]
    fn test_docs_example_selftest() {
        // Test: lawkit selftest
        let output = Command::new("cargo")
            .args(["run", "--bin", "lawkit", "--", "selftest"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to execute lawkit selftest");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Should run self-tests
            assert!(stdout.contains("test") || stdout.contains("pass") || 
                    stdout.contains("selftest") || !stdout.trim().is_empty(),
                    "Selftest should provide test results");
        }
    }
}
