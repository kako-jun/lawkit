//! Output control tests
//! Spec: docs/specs/cli.md ## 共通オプション
//!
//! -f, --format: text (default), csv, json, yaml, toml, xml
//! --no-color: Disable ANSI colors
//! -q, --quiet: Minimal output
//! -v, --verbose: Detailed output
//! --help: Show help
//! --version: Show version

use assert_cmd::Command;
use predicates::prelude::*;

fn lawkit() -> Command {
    Command::cargo_bin("lawkit").unwrap()
}

// Use generate command which always returns exit code 0 for output format tests
fn generate_sample() -> Command {
    let mut cmd = Command::cargo_bin("lawkit").unwrap();
    cmd.args(["generate", "benf", "--samples", "50"]);
    cmd
}

// =============================================================================
// generate command outputs plain text
// =============================================================================

#[test]
fn output_generate_plain_text() {
    // generate command outputs 1 number per line
    generate_sample()
        .assert()
        .success()
        .stdout(predicate::str::is_empty().not());
}

// =============================================================================
// --format (not available for generate, tested via formats.rs for analysis)
// =============================================================================

#[test]
fn output_format_not_available_for_generate() {
    // generate command should NOT accept --format option
    generate_sample()
        .args(["--format", "json"])
        .assert()
        .failure()
        .code(2); // argument error
}

// =============================================================================
// --no-color
// =============================================================================

#[test]
fn output_no_color() {
    let output = generate_sample().arg("--no-color").output().unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("\x1b["),
        "Output should not contain ANSI codes: {stdout}"
    );
}

// =============================================================================
// --quiet
// =============================================================================

#[test]
fn output_quiet() {
    let output = generate_sample().arg("--quiet").output().unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Generate with quiet still outputs data
    assert!(output.status.success());
}

#[test]
fn output_quiet_short_form() {
    generate_sample().arg("-q").assert().success();
}

// =============================================================================
// --verbose
// =============================================================================

#[test]
fn output_verbose() {
    generate_sample().arg("--verbose").assert().success();
}

#[test]
fn output_verbose_short_form() {
    generate_sample().arg("-v").assert().success();
}

// =============================================================================
// --help and --version
// =============================================================================

#[test]
fn output_help() {
    lawkit()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"))
        .stdout(predicate::str::contains("Options"));
}

#[test]
fn output_help_short() {
    lawkit()
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn output_version() {
    lawkit()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("lawkit"));
}

#[test]
fn output_version_short() {
    lawkit()
        .arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("lawkit"));
}

// =============================================================================
// Subcommand help
// =============================================================================

#[test]
fn output_subcommand_help_benf() {
    lawkit()
        .arg("benf")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Benford"));
}

#[test]
fn output_subcommand_help_pareto() {
    lawkit()
        .arg("pareto")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Pareto"));
}

#[test]
fn output_subcommand_help_zipf() {
    lawkit()
        .arg("zipf")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Zipf"));
}

#[test]
fn output_subcommand_help_normal() {
    lawkit()
        .arg("normal")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Normal").or(predicate::str::contains("normal")));
}

#[test]
fn output_subcommand_help_poisson() {
    lawkit()
        .arg("poisson")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Poisson").or(predicate::str::contains("poisson")));
}
