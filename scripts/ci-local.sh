#!/bin/bash
set -euo pipefail

# Exactly match GitHub Actions CI environment
export CARGO_TERM_COLOR=always
export RUST_BACKTRACE=1

# Stricter error handling to match CI
trap 'echo "Error occurred on line $LINENO. Exit code: $?" >&2' ERR

echo "Running complete CI simulation locally (matching GitHub Actions exactly)..."

echo "Step 1: Check formatting"
cargo fmt --all --check

echo "Step 2: Run Clippy"
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo "Step 3: Build"
cargo build --workspace --verbose

echo "Step 4: Run unit tests"
cargo test --workspace --lib --verbose

echo "Step 5: Run integration tests"
cargo test --workspace --test "*" --verbose

echo "Step 6: Test generate functionality"
# Test basic generate functionality
cargo run --bin lawkit -- generate benf --samples 35 --seed 12345 > /tmp/test_benf.txt
cargo run --bin lawkit -- benf /tmp/test_benf.txt

# Test generate→analyze pipeline (allow exit codes 0-12 for normal)
cargo run --bin lawkit -- generate normal --samples 100 --mean 50 --stddev 10 --seed 42 > /tmp/test_normal.txt
set +e
cargo run --bin lawkit -- normal /tmp/test_normal.txt
exit_code=$?
set -e
if [ $exit_code -gt 12 ]; then
  echo "Normal analysis failed with exit code $exit_code"
  exit $exit_code
fi

# Test selftest functionality
cargo run --bin lawkit -- selftest

echo "Step 7: Test CLI examples from README"
# Basic analysis examples (use --min-count to allow small datasets)
set +e
echo "1234 5678 9012 3456" | cargo run --bin lawkit -- benf --min-count 4
exit_code=$?
set -e
if [ $exit_code -gt 12 ]; then
  echo "Benf analysis failed with exit code $exit_code"
  exit $exit_code
fi

# Generate and analyze workflow
cargo run --bin lawkit -- generate benf --samples 100 --seed readme > /tmp/readme_test.txt
cargo run --bin lawkit -- benf /tmp/readme_test.txt --format json

# Multi-law comparison (allow exit codes 0-12 for analysis results)
set +e
cargo run --bin lawkit -- analyze /tmp/readme_test.txt
exit_code=$?
set -e
if [ $exit_code -gt 12 ]; then
  echo "Analyze command failed with exit code $exit_code"
  exit $exit_code
fi

# List functionality
cargo run --bin lawkit -- list

echo "All CI steps completed successfully!"
echo "Ready to push to remote repository"