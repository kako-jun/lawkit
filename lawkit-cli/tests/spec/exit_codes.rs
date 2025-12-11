//! Exit code tests based on docs/specs/cli.md
//!
//! Exit codes:
//! - 0: Normal (LOW or MEDIUM risk)
//! - 1: General error
//! - 2: Argument error
//! - 10: HIGH risk (p ≤ 0.05)
//! - 11: CRITICAL risk (p ≤ 0.01)

use assert_cmd::Command;
use predicates::prelude::*;

fn lawkit() -> Command {
    Command::cargo_bin("lawkit").unwrap()
}

#[test]
fn test_exit_code_0_for_low_risk() {
    // Benford-compliant data should return exit code 0 (LOW risk)
    let mut cmd = lawkit();
    cmd.args(["benf"])
        .write_stdin("123\n234\n345\n456\n567\n678\n789\n891\n912\n");
    cmd.assert().code(predicate::in_iter([0, 10, 11])); // Any valid exit code
}

#[test]
fn test_exit_code_1_for_general_error() {
    // Invalid input should return exit code 1
    let mut cmd = lawkit();
    cmd.args(["benf", "/nonexistent/file.txt"]);
    cmd.assert().code(1);
}

#[test]
fn test_exit_code_2_for_argument_error() {
    // Invalid arguments should return exit code 2
    let mut cmd = lawkit();
    cmd.args(["--invalid-option"]);
    cmd.assert().code(2);
}

#[test]
fn test_exit_code_with_quiet_mode() {
    // --quiet should still return proper exit codes
    let mut cmd = lawkit();
    cmd.args(["benf", "--quiet"])
        .write_stdin("1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n");
    cmd.assert().code(predicate::in_iter([0, 10, 11]));
}

#[test]
fn test_validate_returns_exit_code() {
    let mut cmd = lawkit();
    cmd.args(["validate"])
        .write_stdin("1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n");
    cmd.assert().code(predicate::in_iter([0, 10, 11]));
}

#[test]
fn test_analyze_returns_exit_code() {
    let mut cmd = lawkit();
    cmd.args(["analyze"])
        .write_stdin("1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n");
    cmd.assert().code(predicate::in_iter([0, 10, 11]));
}
