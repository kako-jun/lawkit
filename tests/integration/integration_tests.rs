// Integration tests for lawkit components
// Test the interaction between different parts of the system

use std::process::Command;
use std::io::Write;
use tempfile::NamedTempFile;

fn run_lawkit_command(args: &[&str]) -> std::process::Output {
    let mut command = Command::new("cargo");
    command.args(&["run", "--bin", "lawkit", "--"]);
    command.args(args);
    command.output().expect("Failed to execute lawkit command")
}

fn create_temp_file_with_content(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    file.write_all(content.as_bytes())
        .expect("Failed to write to temp file");
    file
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_benford_analyze_integration() {
        let data = "123 234 345 456 567 678 789 890 901 112 134 245 356 467 578 689 790 801 912 123";
        let output = run_lawkit_command(&["analyze", "benford", data]);
        
        assert!(output.status.success() || matches!(output.status.code(), Some(10) | Some(11)));
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Benford") || stdout.contains("ベンフォード"));
        assert!(stdout.contains("Numbers analyzed") || stdout.contains("解析した数値数"));
    }

    #[test]
    fn test_pareto_analyze_integration() {
        let data = "10 20 30 40 50 60 70 80 90 100 200 300 400 500 600 700 800 900 1000 2000";
        let output = run_lawkit_command(&["analyze", "pareto", data]);
        
        assert!(output.status.success());
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Pareto") || stdout.contains("パレート"));
    }

    #[test]
    fn test_zipf_analyze_integration() {
        let data = "word1 word2 word3 word1 word2 word1 word4 word5 word6 word7 word8 word9 word10";
        let output = run_lawkit_command(&["analyze", "zipf", data]);
        
        assert!(output.status.success());
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Zipf") || stdout.contains("ジップ"));
    }

    #[test]
    fn test_validate_subcommand_integration() {
        let data = "123 234 345 456 567 678 789 890 901 112 134 245 356 467 578 689 790 801 912 123";
        let output = run_lawkit_command(&["validate", data]);
        
        assert!(output.status.success() || matches!(output.status.code(), Some(10) | Some(11)));
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("validation") || stdout.contains("検証"));
    }

    #[test]
    fn test_diagnose_subcommand_integration() {
        let data = "123 234 345 456 567 678 789 890 901 112 134 245 356 467 578 689 790 801 912 123";
        let output = run_lawkit_command(&["diagnose", data]);
        
        assert!(output.status.success() || matches!(output.status.code(), Some(10) | Some(11)));
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("diagnosis") || stdout.contains("診断"));
    }

    #[test]
    fn test_file_input_integration() {
        let csv_content = "Amount\n123.45\n234.56\n345.67\n456.78\n567.89\n678.90\n789.01\n890.12\n901.23\n112.34\n134.45\n245.56\n356.67\n467.78\n578.89\n689.90\n790.01\n801.12\n912.23\n123.45";
        let temp_file = create_temp_file_with_content(csv_content);
        
        let output = run_lawkit_command(&["analyze", "benford", temp_file.path().to_str().unwrap()]);
        assert!(output.status.success() || matches!(output.status.code(), Some(10) | Some(11)));
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Numbers analyzed") || stdout.contains("解析した数値数"));
    }

    #[test]
    fn test_json_output_integration() {
        let data = "123 234 345 456 567 678 789 890 901 112 134 245 356 467 578 689 790 801 912 123";
        let output = run_lawkit_command(&["analyze", "benford", "--format", "json", data]);
        
        assert!(output.status.success() || matches!(output.status.code(), Some(10) | Some(11)));
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("{"));
        assert!(stdout.contains("numbers_analyzed"));
        
        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&stdout)
            .expect("Output should be valid JSON");
        assert!(parsed.get("numbers_analyzed").is_some());
    }

    #[test]
    fn test_no_color_option_integration() {
        let data = "123 234 345 456 567 678 789 890 901 112 134 245 356 467 578 689 790 801 912 123";
        let output = run_lawkit_command(&["analyze", "benford", "--no-color", data]);
        
        assert!(output.status.success() || matches!(output.status.code(), Some(10) | Some(11)));
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Output should not contain ANSI color codes
        assert!(!stdout.contains("\x1b["));
    }

    #[test]
    fn test_verbose_mode_integration() {
        let data = "123 234 345 456 567 678 789 890 901 112 134 245 356 467 578 689 790 801 912 123";
        let output = run_lawkit_command(&["analyze", "benford", "--verbose", data]);
        
        assert!(output.status.success() || matches!(output.status.code(), Some(10) | Some(11)));
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Chi-square") || stdout.contains("p-value") || stdout.contains("distribution"));
    }

    #[test]
    fn test_quiet_mode_integration() {
        let data = "123 234 345 456 567 678 789 890 901 112 134 245 356 467 578 689 790 801 912 123";
        let output = run_lawkit_command(&["analyze", "benford", "--quiet", data]);
        
        assert!(output.status.success() || matches!(output.status.code(), Some(10) | Some(11)));
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        // In quiet mode, output should be minimal
        assert!(stdout.len() < 200);
    }

    #[test]
    fn test_multi_law_analysis_integration() {
        // Test analysis with multiple statistical laws
        let numeric_data = "123 234 345 456 567 678 789 890 901 112 134 245 356 467 578 689 790 801 912 123";
        
        // Test Benford's law
        let benford_output = run_lawkit_command(&["analyze", "benford", numeric_data]);
        assert!(benford_output.status.success() || matches!(benford_output.status.code(), Some(10) | Some(11)));
        
        // Test Pareto principle
        let pareto_output = run_lawkit_command(&["analyze", "pareto", numeric_data]);
        assert!(pareto_output.status.success());
        
        // Test that both outputs contain relevant information
        let benford_stdout = String::from_utf8_lossy(&benford_output.stdout);
        let pareto_stdout = String::from_utf8_lossy(&pareto_output.stdout);
        
        assert!(benford_stdout.contains("Benford") || benford_stdout.contains("ベンフォード"));
        assert!(pareto_stdout.contains("Pareto") || pareto_stdout.contains("パレート"));
    }
}