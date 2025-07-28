use std::process::Command;

/// Test verbose output functionality across all lawkit commands
#[cfg(test)]
mod verbose_output_tests {
    use super::*;

    fn run_lawkit_verbose(
        subcommand: &str,
        args: &[&str],
        input_data: Option<&str>,
    ) -> std::process::Output {
        let mut command = Command::new("cargo");
        command.args(["run", "--bin", "lawkit", "--", subcommand]);
        command.args(args);
        command.args(["--verbose"]);
        command.env("LANG", "en_US.UTF-8");

        if let Some(data) = input_data {
            use std::process::Stdio;
            command.stdin(Stdio::piped());
            command.stdout(Stdio::piped());
            command.stderr(Stdio::piped());

            let mut child = command.spawn().expect("Failed to spawn command");

            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write;
                stdin
                    .write_all(data.as_bytes())
                    .expect("Failed to write to stdin");
            }

            child.wait_with_output().expect("Failed to read output")
        } else {
            command.output().expect("Failed to execute command")
        }
    }

    #[test]
    fn test_benf_verbose_output_format() {
        let test_data = "123 456 789 101 234 567 890 123 456";
        let output = run_lawkit_verbose("benf", &[], Some(test_data));
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Verify debug information is present
        assert!(stderr.contains("Debug: input argument = None"));
        assert!(stderr.contains("Debug: Reading from stdin"));
        assert!(stderr.contains("Debug: Using automatic optimization"));
        assert!(stderr.contains("Debug: Collected"));
        assert!(stderr.contains("numbers from"));

        // Verify performance metrics are present
        assert!(stderr.contains("Debug: Memory used:"));
        assert!(stderr.contains("MB"));
        assert!(stderr.contains("Debug: Processing time:"));
        assert!(stderr.contains("ms"));
    }

    #[test]
    fn test_pareto_verbose_output_format() {
        let test_data = "100 200 300 400 500 600 700 800 900 1000";
        let output = run_lawkit_verbose("pareto", &[], Some(test_data));
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Verify consistent debug output format
        assert!(stderr.contains("Debug: input argument = None"));
        assert!(stderr.contains("Debug: Reading from stdin"));
        assert!(stderr.contains("Debug: Using automatic optimization"));
        assert!(stderr.contains("Debug: Collected"));
        assert!(stderr.contains("Debug: Memory used:"));
        assert!(stderr.contains("Debug: Processing time:"));
    }

    #[test]
    fn test_zipf_verbose_output_format() {
        let test_data = "1 2 3 4 5 6 7 8 9 10";
        let output = run_lawkit_verbose("zipf", &[], Some(test_data));
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Verify zipf-specific debug output
        assert!(stderr.contains("Debug: input argument = None"));
        assert!(stderr.contains("Debug: text mode = false"));
        assert!(stderr.contains("Debug: Reading from stdin"));
        assert!(stderr.contains("Debug: Collected"));
        assert!(stderr.contains("numbers from input"));
    }

    #[test]
    fn test_zipf_text_mode_verbose() {
        let test_data = "the quick brown fox jumps over the lazy dog";
        let output = run_lawkit_verbose("zipf", &["--text"], Some(test_data));
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Verify text mode specific output
        assert!(stderr.contains("Debug: text mode = true"));
        assert!(stderr.contains("Debug: Collected"));
        assert!(stderr.contains("words from stream"));
        assert!(stderr.contains("Debug: Streaming analysis successful"));
    }

    #[test]
    fn test_normal_verbose_output_format() {
        let test_data = "50 51 49 52 48 53 47 54 46 55";
        let output = run_lawkit_verbose("normal", &[], Some(test_data));
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Verify normal distribution verbose output
        assert!(stderr.contains("Debug: input argument = None"));
        assert!(stderr.contains("Debug: Reading from stdin"));
        assert!(stderr.contains("Debug: Collected"));
        assert!(stderr.contains("Debug: Memory used:"));
    }

    #[test]
    fn test_poisson_verbose_output_format() {
        let test_data = "2 3 1 4 2 3 1 2 3 2";
        let output = run_lawkit_verbose("poisson", &[], Some(test_data));
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Verify poisson distribution verbose output
        assert!(stderr.contains("Debug: input argument = None"));
        assert!(stderr.contains("Debug: Reading from stdin"));
        assert!(stderr.contains("Debug: Collected"));
        assert!(stderr.contains("Debug: Memory used:"));
    }

    #[test]
    fn test_verbose_filter_application() {
        let test_data = "1 10 100 1000 10000";
        let output = run_lawkit_verbose("benf", &["--min-value", "100"], Some(test_data));
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Verify filter application is logged in verbose mode
        assert!(stderr.contains("Debug: Min-value filter applied"));
        assert!(stderr.contains("Debug: Filter removed"));
        assert!(stderr.contains("%"));
    }

    #[test]
    fn test_verbose_consistency_across_commands() {
        let test_data = "1 2 3 4 5 6 7 8 9 10";

        let commands = ["benf", "pareto", "zipf", "normal", "poisson"];

        for command in &commands {
            let output = run_lawkit_verbose(command, &[], Some(test_data));
            let stderr = String::from_utf8_lossy(&output.stderr);

            // All commands should have consistent basic debug info
            assert!(
                stderr.contains("Debug: input argument = None"),
                "Command {command} missing input argument debug"
            );
            assert!(
                stderr.contains("Debug: Reading from stdin"),
                "Command {command} missing stdin debug"
            );
        }
    }

    #[test]
    fn test_verbose_with_file_input() {
        use std::fs::File;
        use std::io::Write;
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_data.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "123 456 789").unwrap();

        let output = run_lawkit_verbose("benf", &[file_path.to_str().unwrap()], None);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Should show file input debugging
        assert!(stderr.contains("Debug: input argument = Some"));
        assert!(stderr.contains(&*file_path.to_string_lossy()));
    }

    #[test]
    fn test_verbose_performance_metrics_format() {
        let test_data = "1 2 3 4 5 6 7 8 9 10 11 12 13 14 15";
        let output = run_lawkit_verbose("benf", &[], Some(test_data));
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Verify performance metrics have correct format
        assert!(stderr.contains("Debug: Memory used:"));
        assert!(stderr.contains("MB"));
        assert!(stderr.contains("Debug: Processing time:"));
        assert!(stderr.contains("ms"));

        // Check for reasonable values (should be small for this test)
        let memory_line = stderr
            .lines()
            .find(|line| line.contains("Debug: Memory used:"))
            .expect("Memory usage line not found");

        // Memory should be reported as a number with MB
        assert!(memory_line.matches(" MB").count() == 1);

        let time_line = stderr
            .lines()
            .find(|line| line.contains("Debug: Processing time:"))
            .expect("Processing time line not found");

        // Time should be reported as a number with ms
        assert!(time_line.matches(" ms").count() == 1);
    }
}
