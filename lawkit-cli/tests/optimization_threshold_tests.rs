use std::process::Command;

/// Test optimization thresholds to ensure streaming processing activates
/// at appropriate data sizes and remains disabled for small datasets

#[test]
fn test_normal_below_threshold_no_streaming() {
    // Test with 9,999 items - should NOT use streaming processing
    let small_data: Vec<String> = (1..=9999).map(|i| i.to_string()).collect();
    let test_data = small_data.join(" ");

    let output = run_lawkit_command_with_debug("normal", &[&test_data]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should NOT contain streaming debug messages
    assert!(!stderr.contains("Streaming analysis successful"));
    assert!(!stderr.contains("chunks processed"));
}

#[test]
fn test_normal_above_threshold_uses_streaming() {
    // Test with 10,001 items - should use streaming processing
    let large_data: Vec<String> = (1..=10001).map(|i| i.to_string()).collect();
    let test_data = large_data.join(" ");

    let output = run_lawkit_command_with_debug("normal", &[&test_data]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should contain streaming debug messages
    assert!(
        stderr.contains("Streaming analysis successful") || stderr.contains("chunks processed")
    );
}

#[test]
fn test_poisson_below_threshold_no_streaming() {
    // Test with 9,999 items - should NOT use streaming processing
    // Using 0-9 values (non-negative integers for Poisson)
    let small_data: Vec<String> = (0..9999).map(|i| (i % 10).to_string()).collect();
    let test_data = small_data.join(" ");

    let output = run_lawkit_command_with_debug("poisson", &[&test_data]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should NOT contain streaming debug messages
    assert!(!stderr.contains("Streaming analysis successful"));
    assert!(!stderr.contains("chunks processed"));
}

#[test]
fn test_poisson_above_threshold_uses_streaming() {
    // Test with 12,000 items - should use streaming processing
    // Using 0-9 values (non-negative integers for Poisson)
    let large_data: Vec<String> = (0..12000).map(|i| (i % 10).to_string()).collect();
    let test_data = large_data.join(" ");

    let output = run_lawkit_command_with_debug("poisson", &[&test_data]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should contain streaming debug messages
    assert!(
        stderr.contains("Streaming analysis successful") || stderr.contains("chunks processed")
    );
}

#[test]
fn test_benf_stdin_always_uses_streaming() {
    // Benford analysis with stdin should always use streaming (regardless of size)
    let test_data: Vec<String> = (100..=200).map(|i| i.to_string()).collect();
    let small_test_data = test_data.join(" ");

    let output = run_lawkit_command_with_debug("benf", &[&small_test_data]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should contain streaming debug messages even for small data
    assert!(stderr.contains("Using automatic optimization") || stderr.contains("Debug: Collected"));
}

#[test]
fn test_pareto_stdin_always_uses_streaming() {
    // Pareto analysis with stdin should always use streaming (regardless of size)
    let test_data: Vec<String> = (100..=200).map(|i| i.to_string()).collect();
    let small_test_data = test_data.join(" ");

    let output = run_lawkit_command_with_debug("pareto", &[&small_test_data]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should contain streaming debug messages even for small data
    assert!(stderr.contains("Using automatic optimization") || stderr.contains("Debug: Collected"));
}

#[test]
fn test_exact_threshold_boundary() {
    // Test exactly at the 10,000 threshold
    let boundary_data: Vec<String> = (1..=10000).map(|i| i.to_string()).collect();
    let test_data = boundary_data.join(" ");

    let output = run_lawkit_command_with_debug("normal", &[&test_data]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // At exactly 10,000 items, should NOT use streaming (> 10000 condition)
    assert!(!stderr.contains("Streaming analysis successful"));
}

#[test]
fn test_memory_efficiency_large_dataset() {
    // Verify memory efficiency with a large dataset
    let large_data: Vec<String> = (1..=15000).map(|i| i.to_string()).collect();
    let test_data = large_data.join(" ");

    let output = run_lawkit_command_with_debug("normal", &[&test_data]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should show memory usage information
    assert!(stderr.contains("Memory used") || stderr.contains("chunks processed"));

    // Command should complete successfully despite large size
    assert!(output.status.success() || matches!(output.status.code(), Some(10..=13)));
}

// Helper function to run lawkit with debug output enabled
fn run_lawkit_command_with_debug(subcommand: &str, args: &[&str]) -> std::process::Output {
    let mut command = Command::new("cargo");
    command.args(["run", "--bin", "lawkit", "--", subcommand]);
    command.env("LAWKIT_DEBUG", "1"); // Enable debug output

    // Handle test data similar to integration tests
    let mut test_data = String::new();
    let mut command_args = Vec::new();

    for arg in args {
        if arg.chars().any(|c| c.is_ascii_digit()) && arg.len() > 20 {
            test_data = arg.to_string();
        } else {
            command_args.push(*arg);
        }
    }

    command.args(&command_args);

    if !test_data.is_empty() {
        command
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        let mut child = command.spawn().expect("Failed to spawn command");

        if let Some(stdin) = child.stdin.take() {
            use std::io::Write;
            let mut stdin = stdin;
            stdin
                .write_all(test_data.as_bytes())
                .expect("Failed to write to stdin");
        }

        child.wait_with_output().expect("Failed to read stdout")
    } else {
        command.output().expect("Failed to execute command")
    }
}
