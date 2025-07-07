#!/bin/bash
set -e

echo "🔄 Running complete CI simulation locally..."

echo "📝 Step 1: Check formatting"
cargo fmt --all --check

echo "🔍 Step 2: Run Clippy"
cargo clippy --workspace --all-targets --all-features -- -D warnings

echo "🏗️ Step 3: Build"
cargo build --workspace --verbose

echo "🧪 Step 4: Run unit tests"
cargo test --workspace --lib --verbose

echo "🧪 Step 5: Run integration tests"
cargo test --workspace --test "*" --verbose

echo "🎲 Step 6: Test generate functionality"
# Test basic generate functionality
cargo run --bin lawkit -- generate benf --samples 35 --seed 12345 > /tmp/test_benf.txt
cargo run --bin lawkit -- benf /tmp/test_benf.txt

# Test generate→analyze pipeline (allow exit codes 0-12 for normal)
cargo run --bin lawkit -- generate normal --samples 100 --mean 50 --stddev 10 --seed 42 > /tmp/test_normal.txt
cargo run --bin lawkit -- normal /tmp/test_normal.txt || test $? -le 12

# Test selftest functionality
cargo run --bin lawkit -- selftest

echo "📚 Step 7: Test CLI examples from README"
# Basic analysis examples (use --min-count to allow small datasets)
echo "1234 5678 9012 3456" | cargo run --bin lawkit -- benf --min-count 4 || test $? -le 12

# Generate and analyze workflow
cargo run --bin lawkit -- generate benf --samples 100 --seed readme > /tmp/readme_test.txt
cargo run --bin lawkit -- benf /tmp/readme_test.txt --format json

# Multi-law comparison
cargo run --bin lawkit -- compare /tmp/readme_test.txt

# List functionality
cargo run --bin lawkit -- list

echo "✅ All CI steps completed successfully!"
echo "🚀 Ready to push to remote repository"