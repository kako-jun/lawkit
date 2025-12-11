# lawkit

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
[LOW] Dataset analysis

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
```

## Main Options

```bash
-f, --format <FORMAT>     # Output: text, json, yaml, csv, toml, xml
-q, --quiet               # Minimal output
-v, --verbose             # Detailed output
--filter <RANGE>          # Filter numbers (e.g., >=100, <1000, 50-500)
-c, --min-count <N>       # Minimum data count (default: 10)
--no-color                # Disable colors
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Normal (LOW/MEDIUM risk) |
| 1 | General error |
| 2 | Argument error |
| 10 | HIGH risk (p ≤ 0.05) |
| 11 | CRITICAL risk (p ≤ 0.01) |

## CI/CD Usage

```bash
# Detect anomalies in financial data
if ! lawkit benf transactions.csv --quiet; then
  echo "Anomaly detected"
  lawkit benf transactions.csv --format json > report.json
fi

# Validate data quality
lawkit validate data.csv --cross-validation
```

## Documentation

- [CLI Specification](docs/specs/cli.md)
- [Core API Specification](docs/specs/core.md)

## License

MIT
