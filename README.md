# lawkit

[日本語](README.ja.md)

[![CI](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/lawkit.svg)](https://crates.io/crates/lawkit)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Statistical law analysis toolkit. Analyze data for Benford's law, Pareto principle, Zipf's law, and more. Detect anomalies and assess data quality.

## Why lawkit?

Traditional tools analyze one pattern at a time. lawkit analyzes multiple statistical laws simultaneously:

```bash
$ lawkit benf financial_data.csv
Benford Law Analysis Results

Dataset: financial_data.csv
Numbers analyzed: 1000
[LOW] Dataset conforms to Benford's Law

First Digit Distribution:
1: ██████████████████████████████  30.1% (expected: 30.1%)
2: ██████████████████              17.8% (expected: 17.6%)
3: ████████████                    12.5% (expected: 12.5%)
...

$ lawkit pareto sales.csv
80/20 Rule: Top 20% owns 79.2% of total
```

## Installation

```bash
# As CLI tool
cargo install lawkit

# As library (Cargo.toml)
[dependencies]
lawkit-core = "2.5"
```

## Usage

```bash
# Analysis commands
lawkit benf data.csv          # Benford's law (fraud detection)
lawkit pareto data.csv        # Pareto principle (80/20 rule)
lawkit zipf data.csv          # Zipf's law (frequency distribution)
lawkit normal data.csv        # Normal distribution (quality control)
lawkit poisson data.csv       # Poisson distribution (rare events)

# Integration commands
lawkit analyze data.csv       # Multi-law analysis
lawkit validate data.csv      # Data validation
lawkit diagnose data.csv      # Detailed diagnostics

# Generate test data
lawkit generate benf -s 1000
lawkit generate pareto -s 500

# Utility commands
lawkit list                   # List available laws
lawkit selftest               # Run self-test
```

## Supported Input

- File: `lawkit benf data.csv`
- URL: `lawkit benf https://example.com/data.json`
- stdin: `cat data.csv | lawkit benf -`

Formats: CSV, JSON, YAML, plain text (one number per line)

## Main Options

```bash
-f, --format <FORMAT>      # Output: text, csv, json, yaml, toml, xml
-q, --quiet                # Minimal output
-v, --verbose              # Detailed output
--filter <RANGE>           # Filter numbers (e.g., >=100, <1000, 50-500)
-c, --min-count <N>        # Minimum data count (default: 10)
--no-color                 # Disable colors
-t, --threshold <LEVEL>    # Detection threshold: low, medium, high, critical
--confidence <LEVEL>       # Confidence level (0.01-0.99, default: 0.95)
--sample-size <N>          # Max sample size for large datasets
--min-value <VALUE>        # Minimum value to include
```

## Risk Levels

| Level | Meaning | Exit Code |
|-------|---------|-----------|
| LOW | Data conforms to expected distribution | 0 |
| MEDIUM | Minor deviation, likely normal | 0 |
| HIGH | Significant deviation (p ≤ 0.05) | 10 |
| CRITICAL | Severe anomaly (p ≤ 0.01) | 11 |

Other exit codes: 1 (general error), 2 (argument error)

## CI/CD Usage

```bash
# Detect anomalies in financial data
if ! lawkit benf transactions.csv --quiet; then
  echo "Anomaly detected"
  lawkit benf transactions.csv --format json > report.json
fi

# Validate data quality with cross-validation
lawkit validate data.csv --cross-validation
```

## Documentation

- [CLI Specification](docs/specs/cli.md)
- [Core API Specification](docs/specs/core.md)
- [Test Examples](lawkit-cli/tests/)

## License

MIT
