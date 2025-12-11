# lawkit

[日本語](README.ja.md)

[![CI](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/lawkit.svg)](https://crates.io/crates/lawkit)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Statistical law analysis toolkit. Analyze data for Benford's law, Pareto principle, Zipf's law, Normal and Poisson distributions. Detect anomalies and assess data quality.

## Installation

```bash
cargo install lawkit
```

## Supported Laws

### Benford's Law (Fraud Detection)

```bash
$ lawkit benf financial_data.csv
Benford Law Analysis Results

Dataset: financial_data.csv
Numbers analyzed: 1000
[LOW] Dataset analysis

First Digit Distribution:
1: ███████████████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  30.1% (expected:  30.1%)
2: █████████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  17.6% (expected:  17.6%)
3: ██████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  12.5% (expected:  12.5%)
...
```

### Pareto Principle (80/20 Rule)

```bash
$ lawkit pareto sales.csv
Pareto Principle (80/20 Rule) Analysis Results

Dataset: sales.csv
Numbers analyzed: 500
[LOW] Dataset analysis

Lorenz Curve (Cumulative Distribution):
 20%: ███████████████████████████████████████┃░░░░░░░░░░  79.2% cumulative (80/20 point)
 40%: █████████████████████████████████████████████░░░░░  91.5% cumulative
...

80/20 Rule: Top 20% owns 79.2% of total wealth (Ideal: 80.0%, Ratio: 0.99)
```

### Zipf's Law (Frequency Distribution)

```bash
$ lawkit zipf word_frequencies.csv
Zipf Law Analysis Results

Dataset: word_frequencies.csv
Numbers analyzed: 1000
[LOW] Dataset analysis

Rank-Frequency Distribution:
# 1: █████████████████████████████████████████████████┃  11.50% (expected: 11.50%)
# 2: █████████████████████████┃░░░░░░░░░░░░░░░░░░░░░░░   5.75% (expected: 5.75%)
# 3: █████████████████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   3.83% (expected: 3.83%)
...

Zipf Exponent: 1.02 (ideal: 1.0), Correlation: 0.998
```

### Normal Distribution (Quality Control)

```bash
$ lawkit normal measurements.csv
Normal Distribution Analysis Results

Dataset: measurements.csv
Numbers analyzed: 200
Quality Level: High

Distribution Histogram:
 -2.50- -1.89: ██████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  11.5%
 -1.89- -1.28: █████████████████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  34.0%
 -1.28- -0.67: ███████████████████████████████████┃░░░░░░░░░░░░░░  69.8%
...

Distribution: μ=0.02, σ=1.01, Range: [-2.89, 3.12]
1σ: 68.5%, 2σ: 95.5%, 3σ: 99.7%
```

### Poisson Distribution (Rare Events)

```bash
$ lawkit poisson events.csv
Poisson Distribution Analysis Results

Dataset: events.csv
Numbers analyzed: 100
Quality Level: High

Probability Distribution:
P(X= 0): ██████████████████┃░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  0.095
P(X= 1): ███████████████████████████████████████████┃░░░░░░  0.224
P(X= 2): █████████████████████████████████████████████████┃  0.263
...

λ=2.35, Variance/Mean=1.02 (ideal: 1.0), Fit Score=0.95
```

## Usage

```bash
# Single law analysis
lawkit benf data.csv          # Benford's law
lawkit pareto data.csv        # Pareto principle
lawkit zipf data.csv          # Zipf's law
lawkit normal data.csv        # Normal distribution
lawkit poisson data.csv       # Poisson distribution

# Multi-law analysis
lawkit analyze data.csv       # Run all applicable laws
lawkit validate data.csv      # Data validation
lawkit diagnose data.csv      # Detailed diagnostics

# Generate test data
lawkit generate benf -s 1000
lawkit generate pareto -s 500

# Utility
lawkit list                   # List available laws
lawkit selftest               # Run self-test
```

## Input Sources

```bash
lawkit benf data.csv                        # File
lawkit benf https://example.com/data.json   # URL
cat data.csv | lawkit benf -                # stdin
```

Formats: CSV, JSON, YAML, plain text (one number per line)

## Main Options

```bash
-f, --format <FORMAT>      # Output: text, csv, json, yaml, toml, xml
-q, --quiet                # Minimal output
-v, --verbose              # Detailed output
--filter <RANGE>           # Filter numbers (e.g., >=100, <1000, 50-500)
-c, --min-count <N>        # Minimum data count (default: 10)
--no-color                 # Disable colors
```

## Risk Levels & Exit Codes

| Level | Meaning | Exit Code |
|-------|---------|-----------|
| LOW | Data conforms to expected distribution | 0 |
| MEDIUM | Minor deviation, likely normal | 0 |
| HIGH | Significant deviation (p ≤ 0.05) | 10 |
| CRITICAL | Severe anomaly (p ≤ 0.01) | 11 |

## CI/CD Usage

```bash
# Detect anomalies in financial data
if ! lawkit benf transactions.csv --quiet; then
  echo "Anomaly detected"
  lawkit benf transactions.csv --format json > report.json
fi

# Validate distribution
lawkit validate data.csv --cross-validation
```

## Standalone Tools

For focused single-law analysis:
- [benf](https://crates.io/crates/benf) - Benford's Law only
- [pareto](https://crates.io/crates/pareto) - Pareto Principle only

## Documentation

- [CLI Specification](docs/specs/cli.md)
- [Core API Specification](docs/specs/core.md)

## License

MIT
