//! Output format tests based on docs/specs/cli.md
//!
//! Supported formats: text, csv, json, yaml, toml, xml
//!
//! Note: Exit codes 0, 10, 11 are all valid (LOW/MEDIUM, HIGH, CRITICAL risk)

use assert_cmd::Command;
use predicates::prelude::*;

fn lawkit() -> Command {
    Command::cargo_bin("lawkit").unwrap()
}

const SAMPLE_DATA: &str = "123\n234\n345\n456\n567\n678\n789\n891\n912\n1023\n1134\n1245\n1356\n1467\n1578\n1689\n1790\n1801\n1912\n2023\n2134\n2245\n2356\n2467\n2578\n2689\n2790\n2801\n2912\n3023\n";

// Valid exit codes: 0 (LOW/MEDIUM), 10 (HIGH), 11 (CRITICAL)
fn valid_exit_codes() -> impl predicates::Predicate<i32> {
    predicate::in_iter([0, 10, 11])
}

#[test]
fn test_format_text_default() {
    let mut cmd = lawkit();
    cmd.args(["benf"]).write_stdin(SAMPLE_DATA);
    cmd.assert()
        .code(valid_exit_codes())
        .stdout(predicate::str::contains("Benford"));
}

#[test]
fn test_format_json() {
    let mut cmd = lawkit();
    cmd.args(["benf", "-f", "json"]).write_stdin(SAMPLE_DATA);
    cmd.assert()
        .code(valid_exit_codes())
        .stdout(predicate::str::starts_with("{"));
}

#[test]
fn test_format_yaml() {
    let mut cmd = lawkit();
    cmd.args(["benf", "-f", "yaml"]).write_stdin(SAMPLE_DATA);
    cmd.assert()
        .code(valid_exit_codes())
        .stdout(predicate::str::contains(":"));
}

#[test]
fn test_format_csv() {
    let mut cmd = lawkit();
    cmd.args(["benf", "-f", "csv"]).write_stdin(SAMPLE_DATA);
    cmd.assert()
        .code(valid_exit_codes())
        .stdout(predicate::str::contains(","));
}

#[test]
fn test_format_toml() {
    let mut cmd = lawkit();
    cmd.args(["benf", "-f", "toml"]).write_stdin(SAMPLE_DATA);
    cmd.assert()
        .code(valid_exit_codes())
        .stdout(predicate::str::contains("="));
}

#[test]
fn test_format_xml() {
    let mut cmd = lawkit();
    cmd.args(["benf", "-f", "xml"]).write_stdin(SAMPLE_DATA);
    cmd.assert()
        .code(valid_exit_codes())
        .stdout(predicate::str::starts_with("<?xml"));
}

#[test]
fn test_format_long_option() {
    let mut cmd = lawkit();
    cmd.args(["benf", "--format", "json"])
        .write_stdin(SAMPLE_DATA);
    cmd.assert()
        .code(valid_exit_codes())
        .stdout(predicate::str::starts_with("{"));
}
