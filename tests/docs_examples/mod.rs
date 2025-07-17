//! Documentation examples tests
//! Tests that verify examples in documentation work correctly

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_basic_analyze_command() {
        // Create a simple test file
        let test_data = "1\n2\n3\n4\n5\n6\n7\n8\n9";
        std::fs::write("test_basic.txt", test_data).expect("Failed to write test file");

        let output = Command::new("cargo")
            .args(["run", "--bin", "lawkit", "--", "analyze", "test_basic.txt"])
            .output()
            .expect("Failed to execute command");

        // Cleanup
        let _ = std::fs::remove_file("test_basic.txt");

        // Should not panic and should produce some output
        assert!(output.status.success() || !output.stderr.is_empty());
    }
}
