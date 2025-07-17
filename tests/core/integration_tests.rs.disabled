use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

fn run_benf_command(args: &[&str]) -> std::process::Output {
    let mut command = Command::new("cargo");
    command.args(&["run", "--bin", "benf", "--", "--lang", "en"]); // Force English output for consistent tests
    command.args(args);
    command.output().expect("Failed to execute benf command")
}

fn create_temp_file_with_content(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to temp file");
    file
}

#[cfg(test)]
mod cli_integration_tests {
    use super::*;

    #[test]
    fn test_help_command() {
        let output = run_benf_command(&["--help"]);
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("benf"));
        assert!(stdout.contains("Benford"));
        assert!(stdout.contains("--format"));
        // assert!(stdout.contains("--url")); // URL functionality removed
    }

    #[test]
    fn test_version_command() {
        let output = run_benf_command(&["--version"]);
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("benf"));
        assert!(stdout.contains("1.0.0"));
    }

    #[test]
    fn test_no_arguments_shows_help() {
        let output = run_benf_command(&[]);
        // Should either show help or prompt for input
        // Exit code might be 0 (help) or 2 (missing args)
        assert!(output.status.code() == Some(0) || output.status.code() == Some(2));
    }

    #[test]
    fn test_pipe_input() {
        let mut child = Command::new("cargo")
            .args(&["run", "--bin", "benf", "--"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to spawn benf process");

        if let Some(stdin) = child.stdin.as_mut() {
            // Need sufficient data for analysis (minimum 30 numbers)
            let large_input = (100..150)
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            stdin
                .write_all(large_input.as_bytes())
                .expect("Failed to write to stdin");
        }

        let output = child.wait_with_output().expect("Failed to read stdout");
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Benford") || stdout.contains("ベンフォード"));
        assert!(stdout.contains("Numbers analyzed") || stdout.contains("解析した数値数"));
    }

    #[test]
    fn test_string_argument() {
        // Generate data with diverse first digits to avoid extreme distributions
        let diverse_data = vec![
            "123", "234", "345", "456", "567", "678", "789", "891", "912", "123", "124", "235",
            "346", "457", "568", "679", "780", "892", "913", "124", "125", "236", "347", "458",
            "569", "670", "781", "893", "914", "125", "126", "237", "348", "459", "560", "671",
            "782", "894", "915", "126", "127", "238", "349", "450", "561", "672", "783", "895",
            "916", "127",
        ]
        .join(" ");
        let output = run_benf_command(&[&diverse_data]);

        // Debug output
        if !output.status.success() {
            println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
            println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
            println!("Exit code: {:?}", output.status.code());
        }

        // Accept exit codes 0, 10, or 11 (normal operation with different risk levels)
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(10) | Some(11)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        // Check for either English or Japanese output (depends on system locale)
        assert!(stdout.contains("Numbers analyzed") || stdout.contains("解析した数値数"));
        assert!(stdout.contains("Attention Level") || stdout.contains("注意レベル"));
    }

    #[test]
    fn test_file_input() {
        // Generate sufficient data for analysis (minimum 30 numbers)
        let numbers: Vec<String> = (100..140).map(|i| i.to_string()).collect();
        let content = numbers.join("\n");
        let temp_file = create_temp_file_with_content(&content);

        let output = run_benf_command(&[temp_file.path().to_str().unwrap()]);
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Numbers analyzed"));
        assert!(stdout.contains("40")); // Should analyze 40 numbers
    }

    #[test]
    fn test_csv_file_input() {
        let csv_content = "Name,Amount,Date\nSales,1234.56,2023-01-01\nExpenses,567.89,2023-01-02\nRevenue,9876.54,2023-01-03";
        let temp_file = create_temp_file_with_content(csv_content);

        let output = run_benf_command(&[temp_file.path().to_str().unwrap()]);
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Numbers analyzed"));
    }

    #[test]
    fn test_yaml_file_input() {
        let yaml_content = r#"
financial_report:
  revenue: 1234567
  expenses: 567890
transactions:
  - amount: 123.45
  - amount: 234.56
"#;
        let temp_file = create_temp_file_with_content(yaml_content);

        let output = run_benf_command(&[temp_file.path().to_str().unwrap()]);
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Numbers analyzed"));
    }

    #[test]
    fn test_opendocument_file_input() {
        // Test ODS (OpenDocument Spreadsheet) file
        let _output = run_benf_command(&["tests/fixtures/sample_data.ods"]);
        // Note: This test will need actual ODS parsing implementation
        // For now, we expect it to either succeed or fail gracefully

        // Test ODT (OpenDocument Text) file
        let _output = run_benf_command(&["tests/fixtures/sample_data.odt"]);
        // Similar expectation for ODT files
    }

    #[test]
    fn test_toml_file_input() {
        let toml_content = r#"
[financial_report]
revenue = 1234567
expenses = 567890

[[transactions]]
amount = 123.45

[[transactions]]
amount = 234.56
"#;
        let temp_file = create_temp_file_with_content(toml_content);

        let output = run_benf_command(&[temp_file.path().to_str().unwrap()]);
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Numbers analyzed"));
    }

    #[test]
    fn test_json_output_format() {
        // Generate sufficient data for analysis
        let large_data = (100..140)
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let output = run_benf_command(&["--format", "json", &large_data]);
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("{"));
        assert!(stdout.contains("dataset"));
        assert!(stdout.contains("numbers_analyzed"));
        assert!(stdout.contains("risk_level"));

        // Verify it's valid JSON
        let parsed: serde_json::Value =
            serde_json::from_str(&stdout).expect("Output should be valid JSON");
        assert!(parsed.get("numbers_analyzed").is_some());
    }

    // TODO: Implement CSV output format
    // #[test]
    // fn test_csv_output_format() {
    //     let output = run_benf_command(&["--format", "csv", "123 456 789"]);
    //     assert!(output.status.success());
    //
    //     let stdout = String::from_utf8_lossy(&output.stdout);
    //     assert!(stdout.contains(","));
    //     // Should have CSV headers
    //     assert!(stdout.contains("dataset") || stdout.contains("numbers_analyzed"));
    // }

    #[test]
    fn test_quiet_mode() {
        let output = run_benf_command(&["--quiet", "123 456 789"]);
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        // In quiet mode, output should be minimal
        assert!(stdout.len() < 100); // Arbitrary threshold for "quiet"
    }

    #[test]
    fn test_verbose_mode() {
        let output = run_benf_command(&["--verbose", "123 456 789"]);
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        // In verbose mode, should have detailed statistics
        assert!(stdout.contains("Chi-square") || stdout.contains("p-value"));
        assert!(stdout.contains("distribution"));
    }

    #[test]
    fn test_japanese_numerals_cli() {
        // Mix of different numeral types with sufficient quantity
        let mixed_data = vec![
            "１２３",
            "４５６",
            "７８９",
            "一二三",
            "四五六",
            "七八九",
            "1234",
            "5678",
            "9012",
            "3456",
            "7890",
            "2345",
            "6789",
            "1012",
            "3456",
            "7890",
            "2134",
            "5678",
            "9012",
            "3456",
            "7890",
            "1234",
            "5678",
            "9012",
            "3456",
            "7890",
            "1234",
            "5678",
            "9012",
            "3456",
            "7890",
            "1234",
            "5678",
            "9012",
            "3456",
            "7890",
        ];
        let test_data = mixed_data.join(" ");
        let output = run_benf_command(&[&test_data]);
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Numbers analyzed"));
        // Should extract and analyze both full-width and kanji numbers
    }

    #[test]
    fn test_language_support() {
        // Create test data that roughly follows Benford's law to avoid triggering high risk exit codes
        let benford_data = vec![
            "123", "234", "345", "456", "567", "678", "789", "890", "901",
            "112", // digit 1,2,3,4,5,6,7,8,9,1
            "134", "245", "356", "467", "578", "689", "790", "801", "912",
            "123", // more 1s,2s,3s,4s,5s,6s,7s,8s,9s,1s
            "156", "267", "378", "489", "590", "601", "712", "823", "934",
            "145", // mixed distribution
            "178", "289", "390", "401", "512", "623", "734", "845", "956",
            "167", // more mixed numbers
        ];
        let large_data = benford_data.join(" ");

        // Test Japanese output
        let output = run_benf_command(&["--lang", "ja", &large_data]);
        // Language support test - focus on output content, not exit code (risk detection may trigger non-zero exit)
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("ベンフォード") || stdout.contains("解析した数値数"));

        // Test Chinese output
        let output = run_benf_command(&["--lang", "zh", &large_data]);
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("本福德") || stdout.contains("分析的数字"));

        // Test Hindi output
        let output = run_benf_command(&["--lang", "hi", &large_data]);
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("बेनफोर्ड") || stdout.contains("विश्लेषित"));

        // Test Arabic output
        let output = run_benf_command(&["--lang", "ar", &large_data]);
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("بنفورد") || stdout.contains("المحللة"));
    }

    // TODO: Implement filter option
    // #[test]
    // fn test_filter_option() {
    //     let output = run_benf_command(&["--filter", ">=100", "50 150 250 350"]);
    //     assert!(output.status.success());
    //
    //     let stdout = String::from_utf8_lossy(&output.stdout);
    //     // Should only analyze numbers >= 100, so 3 numbers
    //     assert!(stdout.contains("3"));
    // }

    // TODO: Implement threshold option
    // #[test]
    // fn test_threshold_option() {
    //     let output = run_benf_command(&["--threshold", "high", "123 456 789"]);
    //     assert!(output.status.success());
    //
    //     let stdout = String::from_utf8_lossy(&output.stdout);
    //     // Should run without error (specific behavior depends on implementation)
    //     assert!(stdout.contains("Risk Level") || stdout.contains("Numbers analyzed"));
    // }

    #[test]
    fn test_invalid_arguments() {
        let output = run_benf_command(&["--invalid-option"]);
        assert!(!output.status.success());
        assert_eq!(output.status.code(), Some(2)); // Argument parsing error
    }

    #[test]
    fn test_invalid_format() {
        let output = run_benf_command(&["--format", "invalid", "123 456"]);
        assert!(!output.status.success());
        assert_eq!(output.status.code(), Some(2)); // Invalid format error
    }

    #[test]
    fn test_nonexistent_file() {
        let output = run_benf_command(&["/path/that/does/not/exist.txt"]);
        assert!(!output.status.success());
        assert_eq!(output.status.code(), Some(3)); // File error
    }

    #[test]
    fn test_empty_input() {
        let output = run_benf_command(&[""]);
        assert!(!output.status.success());
        // Should exit with error for empty input
    }

    #[test]
    fn test_no_numbers_in_input() {
        let output = run_benf_command(&["no numbers here at all"]);
        assert!(!output.status.success());
        // Should exit with error when no numbers found
    }

    #[test]
    fn test_exit_codes() {
        // Test normal case - accept valid exit codes (0, 10, 11 for different risk levels)
        let diverse_data = vec![
            "123", "234", "345", "456", "567", "678", "789", "891", "912",
        ]
        .join(" ");
        let output = run_benf_command(&[&diverse_data]);
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(10) | Some(11)
        ));

        // Test high risk case (should exit with 10)
        // Note: This depends on the actual data producing high risk
        // We might need synthetic data that reliably produces high risk

        // Test invalid arguments (should exit with 2)
        let output = run_benf_command(&["--invalid"]);
        assert_eq!(output.status.code(), Some(2));
    }
}

#[cfg(test)]
#[cfg(feature = "url")] // Disabled since URL functionality was removed
mod url_integration_tests {
    use super::*;
    use mockito::Server;

    #[test]
    fn test_url_input_success() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/data")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body>Revenue: $123,456 Expenses: $78,901</body></html>")
            .create();

        let url = format!("{}/data", server.url());
        let output = run_benf_command(&["--url", &url]);

        mock.assert();
        assert!(output.status.success());

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Numbers analyzed"));
    }

    #[test]
    fn test_url_input_failure() {
        let output = run_benf_command(&["--url", "http://nonexistent.invalid.url"]);
        assert!(!output.status.success());
        assert_eq!(output.status.code(), Some(3)); // Network error
    }

    #[test]
    fn test_url_with_proxy() {
        // Test proxy option (might not actually connect)
        let output = run_benf_command(&[
            "--url",
            "http://example.com",
            "--proxy",
            "http://proxy.example.com:8080",
        ]);
        // This will likely fail, but should fail with network error, not argument error
        assert_eq!(output.status.code(), Some(3)); // Network error, not argument error
    }

    #[test]
    fn test_url_with_timeout() {
        let output = run_benf_command(&[
            "--url",
            "http://httpbin.org/delay/10", // Slow endpoint
            "--timeout",
            "1", // 1 second timeout
        ]);
        assert!(!output.status.success());
        // Should timeout and exit with network error
        assert_eq!(output.status.code(), Some(3));
    }

    #[test]
    fn test_insecure_option() {
        let output = run_benf_command(&["--url", "https://self-signed.badssl.com/", "--insecure"]);
        // This should attempt to connect despite certificate issues
        // Result depends on whether the site is actually accessible
        // We're mainly testing that the option is accepted
        assert!(output.status.code() != Some(2)); // Not an argument error
    }
}

#[cfg(test)]
mod performance_integration_tests {
    use super::*;

    #[test]
    fn test_large_dataset_performance() {
        // Generate a large dataset
        let large_data: Vec<String> = (1..10000)
            .map(|i| (i as f64 * 1.618).to_string()) // Golden ratio for varied digits
            .collect();
        let large_content = large_data.join(" ");

        let temp_file = create_temp_file_with_content(&large_content);

        let start = std::time::Instant::now();
        let output = run_benf_command(&[temp_file.path().to_str().unwrap()]);
        let duration = start.elapsed();

        assert!(output.status.success());
        assert!(duration.as_secs() < 10); // Should complete within 10 seconds

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("9999")); // Should analyze 9999 numbers (excluding 0)
    }

    #[test]
    fn test_memory_usage() {
        // Test with moderately large dataset to ensure reasonable memory usage
        let data: Vec<String> = (1..1000).map(|i| format!("{}.{}", i, i)).collect();
        let content = data.join("\n");

        let temp_file = create_temp_file_with_content(&content);
        let output = run_benf_command(&[temp_file.path().to_str().unwrap()]);

        assert!(output.status.success());
        // If this completes without OOM, we're good
    }
}
