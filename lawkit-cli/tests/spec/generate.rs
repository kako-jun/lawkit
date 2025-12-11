//! Generate command tests based on docs/specs/cli.md
//!
//! generate <law> - Generate sample data
//! Supports: benf, pareto, zipf, normal, poisson
//!
//! Note: Exit codes 0, 10, 11 are all valid (LOW/MEDIUM, HIGH, CRITICAL risk)

use assert_cmd::Command;
use predicates::prelude::*;

fn lawkit() -> Command {
    Command::cargo_bin("lawkit").unwrap()
}

fn valid_exit_codes() -> impl predicates::Predicate<i32> {
    predicate::in_iter([0, 10, 11])
}

// ============================================================================
// generate benf
// ============================================================================

#[test]
fn test_generate_benf_default() {
    let mut cmd = lawkit();
    cmd.args(["generate", "benf"]);
    cmd.assert()
        .code(valid_exit_codes())
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn test_generate_benf_samples() {
    let mut cmd = lawkit();
    cmd.args(["generate", "benf", "-s", "100"]);
    cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_generate_benf_seed() {
    let mut cmd = lawkit();
    cmd.args(["generate", "benf", "--seed", "42"]);
    let output1 = cmd
        .assert()
        .code(valid_exit_codes())
        .get_output()
        .stdout
        .clone();

    let mut cmd2 = lawkit();
    cmd2.args(["generate", "benf", "--seed", "42"]);
    let output2 = cmd2
        .assert()
        .code(valid_exit_codes())
        .get_output()
        .stdout
        .clone();

    assert_eq!(output1, output2, "Same seed should produce same output");
}

#[test]
fn test_generate_benf_fraud_rate() {
    let mut cmd = lawkit();
    cmd.args(["generate", "benf", "--fraud-rate", "0.1"]);
    cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_generate_benf_range() {
    let mut cmd = lawkit();
    cmd.args(["generate", "benf", "--range", "100,10000"]);
    cmd.assert().code(valid_exit_codes());
}

// ============================================================================
// generate pareto
// ============================================================================

#[test]
fn test_generate_pareto_default() {
    let mut cmd = lawkit();
    cmd.args(["generate", "pareto"]);
    cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_generate_pareto_samples() {
    let mut cmd = lawkit();
    cmd.args(["generate", "pareto", "-s", "50"]);
    cmd.assert().code(valid_exit_codes());
}

// ============================================================================
// generate zipf
// ============================================================================

#[test]
fn test_generate_zipf_default() {
    let mut cmd = lawkit();
    cmd.args(["generate", "zipf"]);
    cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_generate_zipf_samples() {
    let mut cmd = lawkit();
    cmd.args(["generate", "zipf", "-s", "50"]);
    cmd.assert().code(valid_exit_codes());
}

// ============================================================================
// generate normal
// ============================================================================

#[test]
fn test_generate_normal_default() {
    let mut cmd = lawkit();
    cmd.args(["generate", "normal"]);
    cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_generate_normal_samples() {
    let mut cmd = lawkit();
    cmd.args(["generate", "normal", "-s", "100"]);
    cmd.assert().code(valid_exit_codes());
}

// ============================================================================
// generate poisson
// ============================================================================

#[test]
fn test_generate_poisson_default() {
    let mut cmd = lawkit();
    cmd.args(["generate", "poisson"]);
    cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_generate_poisson_samples() {
    let mut cmd = lawkit();
    cmd.args(["generate", "poisson", "-s", "100"]);
    cmd.assert().code(valid_exit_codes());
}

// ============================================================================
// Roundtrip tests (generate -> analyze)
// ============================================================================

#[test]
fn test_roundtrip_benf() {
    // Generate Benford data, then analyze it
    let mut gen_cmd = lawkit();
    let gen_output = gen_cmd
        .args(["generate", "benf", "-s", "1000"])
        .assert()
        .code(valid_exit_codes())
        .get_output()
        .stdout
        .clone();

    let mut analyze_cmd = lawkit();
    analyze_cmd.args(["benf"]).write_stdin(gen_output);
    analyze_cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_roundtrip_pareto() {
    let mut gen_cmd = lawkit();
    let gen_output = gen_cmd
        .args(["generate", "pareto", "-s", "100"])
        .assert()
        .code(valid_exit_codes())
        .get_output()
        .stdout
        .clone();

    let mut analyze_cmd = lawkit();
    analyze_cmd.args(["pareto"]).write_stdin(gen_output);
    analyze_cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_roundtrip_normal() {
    let mut gen_cmd = lawkit();
    let gen_output = gen_cmd
        .args(["generate", "normal", "-s", "100"])
        .assert()
        .code(valid_exit_codes())
        .get_output()
        .stdout
        .clone();

    let mut analyze_cmd = lawkit();
    analyze_cmd.args(["normal"]).write_stdin(gen_output);
    analyze_cmd.assert().code(valid_exit_codes());
}
