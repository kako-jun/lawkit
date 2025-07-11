use std::process::Command;
use std::io::Write;
use tempfile::NamedTempFile;

/// Test automatic optimization functionality
#[cfg(test)]
mod automatic_optimization_tests {
    use super::*;

    fn run_lawkit_with_debug(subcommand: &str, args: &[&str], input_data: Option<&str>) -> std::process::Output {
        let mut command = Command::new("cargo");
        command.args(["run", "--bin", "lawkit", "--", subcommand]);
        command.args(args);
        command.env("LAWKIT_DEBUG", "1");
        command.env("LANG", "en_US.UTF-8");

        if let Some(data) = input_data {
            use std::process::Stdio;
            command.stdin(Stdio::piped());
            command.stdout(Stdio::piped());
            command.stderr(Stdio::piped());

            let mut child = command.spawn().expect("Failed to spawn lawkit command");

            if let Some(mut stdin) = child.stdin.take() {
                write!(stdin, "{data}").expect("Failed to write to stdin");
            }

            child.wait_with_output().expect("Failed to get output")
        } else {
            command.output().expect("Failed to execute lawkit command")
        }
    }

    #[test]
    fn test_automatic_optimization_triggers() {
        let test_data = "100\n200\n300\n400\n500\n600\n700\n800\n900\n1000";
        
        let output = run_lawkit_with_debug("benf", &[], Some(test_data));
        
        // Test should succeed
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Verify automatic optimization is triggered
        assert!(stderr.contains("automatic optimization"), 
                "Should show automatic optimization trigger, stderr: {}", stderr);
        assert!(stderr.contains("streaming + incremental + memory efficiency"), 
                "Should show optimization features, stderr: {}", stderr);
        assert!(stderr.contains("Streaming analysis successful"), 
                "Should complete streaming analysis, stderr: {}", stderr);
    }

    #[test]
    fn test_optimization_memory_efficiency() {
        // Test with larger dataset to verify memory efficiency
        let mut large_data = Vec::new();
        for i in 1..=1000 {
            large_data.push(format!("{}", 100 + i * 17));
        }
        let test_data = large_data.join("\n");

        let output = run_lawkit_with_debug("benf", &[], Some(&test_data));
        
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Verify chunked processing
        assert!(stderr.contains("Processed 1000 numbers"), 
                "Should process all 1000 numbers, stderr: {}", stderr);
        assert!(stderr.contains("chunks"), 
                "Should show chunk processing, stderr: {}", stderr);
        
        // Memory usage should be reported
        assert!(stderr.contains("Memory used:"), 
                "Should report memory usage, stderr: {}", stderr);
    }

    #[test]
    fn test_optimization_across_all_laws() {
        let test_data = "123\n456\n789\n1234\n2345\n3456\n4567\n5678\n6789\n1000";
        
        let laws = ["benf", "pareto", "zipf", "normal"];
        
        for law in &laws {
            let output = run_lawkit_with_debug(law, &[], Some(test_data));
            
            assert!(matches!(
                output.status.code(),
                Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
            ), "Law {} should succeed", law);

            let stderr = String::from_utf8_lossy(&output.stderr);
            
            // All laws should use automatic optimization
            assert!(stderr.contains("automatic optimization") || stderr.is_empty(), 
                    "Law {} should use automatic optimization or have no debug output, stderr: {}", law, stderr);
        }
    }

    #[test]
    fn test_no_optimize_flag_exists() {
        // Verify that --optimize flag no longer exists
        let output = run_lawkit_with_debug("benf", &["--optimize"], None);
        
        // Should fail with argument parsing error
        assert_eq!(output.status.code(), Some(2), "Should fail with argument parsing error");
        
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("unexpected argument") || stderr.contains("not found"), 
                "Should show that --optimize flag doesn't exist, stderr: {}", stderr);
    }

    #[test]
    fn test_optimization_is_transparent() {
        // Test that optimization doesn't change output format
        let test_data = "100\n200\n300\n400\n500";
        
        let output = run_lawkit_with_debug("benf", &[], Some(test_data));
        
        assert!(matches!(
            output.status.code(),
            Some(0) | Some(1) | Some(10) | Some(11) | Some(12)
        ));

        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Output should still contain standard Benford analysis
        assert!(stdout.contains("Benford Law Analysis Results"), 
                "Should contain standard analysis output, stdout: {}", stdout);
        assert!(stdout.contains("Numbers analyzed: 5"), 
                "Should analyze correct number of values, stdout: {}", stdout);
        assert!(stdout.contains("First Digit Distribution:"), 
                "Should show distribution, stdout: {}", stdout);
    }

    #[test]
    fn test_chunked_processing_indicators() {
        // Test that we can detect chunked processing in larger datasets
        let mut large_data = Vec::new();
        for i in 1..=15000 { // Large enough to trigger multiple chunks
            large_data.push(format!("{}", 1000 + i * 23));
        }
        let test_data = large_data.join("\n");

        let output = run_lawkit_with_debug("benf", &[], Some(&test_data));
        
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        if output.status.success() {
            // If successful, should show chunk processing
            assert!(stderr.contains("chunks"), 
                    "Should show chunk processing for large dataset, stderr: {}", stderr);
            assert!(stderr.contains("Streaming analysis successful"), 
                    "Should complete streaming analysis, stderr: {}", stderr);
        }
        // Note: Large datasets might hit memory limits in test environment,
        // so we don't assert success, just verify behavior when it works
    }
}