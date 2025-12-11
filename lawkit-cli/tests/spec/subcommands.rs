//! Subcommand tests based on docs/specs/cli.md
//!
//! Analysis commands: benf, pareto, zipf, normal, poisson
//! Integration commands: analyze, validate, diagnose
//!
//! Note: Exit codes 0, 10, 11 are all valid (LOW/MEDIUM, HIGH, CRITICAL risk)

use assert_cmd::Command;
use predicates::prelude::*;

fn lawkit() -> Command {
    Command::cargo_bin("lawkit").unwrap()
}

const SAMPLE_DATA: &str = "1\n2\n3\n10\n20\n30\n100\n200\n300\n1000\n1\n2\n3\n10\n20\n30\n100\n200\n300\n1000\n1\n2\n3\n10\n20\n30\n100\n200\n300\n1000\n";
const INTEGER_DATA: &str = "0\n1\n2\n3\n1\n0\n2\n1\n3\n2\n0\n1\n2\n3\n1\n0\n2\n1\n3\n2\n0\n1\n2\n3\n1\n0\n2\n1\n3\n2\n";

fn valid_exit_codes() -> impl predicates::Predicate<i32> {
    predicate::in_iter([0, 10, 11])
}

// ============================================================================
// benf (Benford's Law)
// ============================================================================

#[test]
fn test_benf_basic() {
    let mut cmd = lawkit();
    cmd.args(["benf"]).write_stdin(SAMPLE_DATA);
    cmd.assert()
        .code(valid_exit_codes())
        .stdout(predicate::str::contains("Benford"));
}

#[test]
fn test_benf_threshold_option() {
    let mut cmd = lawkit();
    cmd.args(["benf", "-t", "high"]).write_stdin(SAMPLE_DATA);
    cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_benf_confidence_option() {
    let mut cmd = lawkit();
    cmd.args(["benf", "--confidence", "0.99"])
        .write_stdin(SAMPLE_DATA);
    cmd.assert().code(valid_exit_codes());
}

// ============================================================================
// pareto (Pareto Principle)
// ============================================================================

#[test]
fn test_pareto_basic() {
    let mut cmd = lawkit();
    cmd.args(["pareto"]).write_stdin(SAMPLE_DATA);
    cmd.assert()
        .code(valid_exit_codes())
        .stdout(predicate::str::contains("Pareto").or(predicate::str::contains("80/20")));
}

#[test]
fn test_pareto_concentration_option() {
    let mut cmd = lawkit();
    cmd.args(["pareto", "-C", "0.7"]).write_stdin(SAMPLE_DATA);
    cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_pareto_gini_option() {
    let mut cmd = lawkit();
    cmd.args(["pareto", "--gini-coefficient"])
        .write_stdin(SAMPLE_DATA);
    cmd.assert().code(valid_exit_codes());
}

// ============================================================================
// zipf (Zipf's Law)
// ============================================================================

#[test]
fn test_zipf_basic() {
    let mut cmd = lawkit();
    cmd.args(["zipf"]).write_stdin(SAMPLE_DATA);
    cmd.assert()
        .code(valid_exit_codes())
        .stdout(predicate::str::contains("Zipf"));
}

#[test]
fn test_zipf_text_mode() {
    // Need sufficient words for analysis (minimum 30)
    let text = "hello world hello foo bar hello world the a an is are was were be been being have has had do does did will would could should may might must shall can cannot won't don't isn't aren't wasn't weren't hasn't haven't hadn't won't couldn't shouldn't mightn't mustn't\n";
    let mut cmd = lawkit();
    cmd.args(["zipf", "-T"]).write_stdin(text);
    cmd.assert().code(valid_exit_codes());
}

// ============================================================================
// normal (Normal Distribution)
// ============================================================================

#[test]
fn test_normal_basic() {
    let mut cmd = lawkit();
    cmd.args(["normal"]).write_stdin(SAMPLE_DATA);
    cmd.assert()
        .code(valid_exit_codes())
        .stdout(predicate::str::contains("Normal").or(predicate::str::contains("mean")));
}

#[test]
fn test_normal_outliers_option() {
    // Generate normal data then analyze with outliers option
    let mut gen_cmd = lawkit();
    let output = gen_cmd
        .args(["generate", "normal", "-s", "50"])
        .assert()
        .code(valid_exit_codes())
        .get_output()
        .stdout
        .clone();

    let mut cmd = lawkit();
    cmd.args(["normal", "--outliers"]).write_stdin(output);
    cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_normal_quality_control_option() {
    // Generate normal data then analyze with quality control
    // Quality control has its own exit codes: 0=Excellent, 1=Adequate, 2=Poor, 3=Inadequate
    let mut gen_cmd = lawkit();
    let output = gen_cmd
        .args(["generate", "normal", "-s", "50"])
        .assert()
        .code(valid_exit_codes())
        .get_output()
        .stdout
        .clone();

    let mut cmd = lawkit();
    cmd.args(["normal", "--quality-control", "--spec-limits", "0,5"])
        .write_stdin(output);
    cmd.assert().code(predicate::in_iter([0, 1, 2, 3]));
}

#[test]
fn test_normal_timeseries_option() {
    let mut cmd = lawkit();
    cmd.args(["normal", "--enable-timeseries"])
        .write_stdin(SAMPLE_DATA);
    cmd.assert().code(valid_exit_codes());
}

// ============================================================================
// poisson (Poisson Distribution)
// ============================================================================

#[test]
fn test_poisson_basic() {
    let mut cmd = lawkit();
    cmd.args(["poisson"]).write_stdin(INTEGER_DATA);
    cmd.assert()
        .code(valid_exit_codes())
        .stdout(predicate::str::contains("Poisson").or(predicate::str::contains("lambda")));
}

#[test]
fn test_poisson_predict_option() {
    let mut cmd = lawkit();
    cmd.args(["poisson", "--predict"]).write_stdin(INTEGER_DATA);
    cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_poisson_rare_events_option() {
    let mut cmd = lawkit();
    cmd.args(["poisson", "--rare-events"])
        .write_stdin(INTEGER_DATA);
    cmd.assert().code(valid_exit_codes());
}

// ============================================================================
// analyze (Integration)
// ============================================================================

#[test]
fn test_analyze_basic() {
    let mut cmd = lawkit();
    cmd.args(["analyze"]).write_stdin(SAMPLE_DATA);
    cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_analyze_laws_option() {
    let mut cmd = lawkit();
    cmd.args(["analyze", "--laws", "benf,pareto"])
        .write_stdin(SAMPLE_DATA);
    cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_analyze_recommend_option() {
    let mut cmd = lawkit();
    cmd.args(["analyze", "--recommend"]).write_stdin(SAMPLE_DATA);
    cmd.assert().code(valid_exit_codes());
}

// ============================================================================
// validate
// ============================================================================

#[test]
fn test_validate_basic() {
    let mut cmd = lawkit();
    cmd.args(["validate"]).write_stdin(SAMPLE_DATA);
    cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_validate_consistency_check() {
    let mut cmd = lawkit();
    cmd.args(["validate", "--consistency-check"])
        .write_stdin(SAMPLE_DATA);
    cmd.assert().code(valid_exit_codes());
}

#[test]
fn test_validate_cross_validation() {
    let mut cmd = lawkit();
    cmd.args(["validate", "--cross-validation"])
        .write_stdin(SAMPLE_DATA);
    cmd.assert()
        .code(valid_exit_codes())
        .stdout(predicate::str::contains("Cross-Validation"));
}

// ============================================================================
// diagnose
// ============================================================================

#[test]
fn test_diagnose_basic() {
    let mut cmd = lawkit();
    cmd.args(["diagnose"]).write_stdin(SAMPLE_DATA);
    cmd.assert().code(valid_exit_codes());
}

// ============================================================================
// list
// ============================================================================

#[test]
fn test_list_command() {
    let mut cmd = lawkit();
    cmd.args(["list"]);
    cmd.assert()
        .code(valid_exit_codes())
        .stdout(predicate::str::contains("benf").or(predicate::str::contains("Benford")));
}

// ============================================================================
// selftest
// ============================================================================

#[test]
fn test_selftest_command() {
    let mut cmd = lawkit();
    cmd.args(["selftest"]);
    cmd.assert().code(valid_exit_codes());
}
