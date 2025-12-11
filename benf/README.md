# benf

[![Crates.io](https://img.shields.io/crates/v/benf.svg)](https://crates.io/crates/benf)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Benford's Law analysis CLI tool. Detect anomalies in numerical data for fraud detection and data quality assessment.

> **Note**: This is a convenience wrapper for `lawkit benf`. For additional statistical laws (Pareto, Zipf, Normal, Poisson) and advanced features, see [lawkit](https://crates.io/crates/lawkit).

## Installation

```bash
cargo install benf
```

## Usage

```bash
# Analyze a file
benf data.csv

# From stdin
cat numbers.txt | benf -

# Output formats
benf data.csv --format json
benf data.csv --format csv
```

## Options

```bash
-f, --format <FORMAT>   Output: text, csv, json, yaml, toml, xml
-q, --quiet             Minimal output
-v, --verbose           Detailed output
--filter <RANGE>        Filter numbers (e.g., >=100, <1000)
--no-color              Disable colors
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | LOW/MEDIUM risk - data conforms to Benford's Law |
| 10 | HIGH risk (p ≤ 0.05) |
| 11 | CRITICAL risk (p ≤ 0.01) |

## See Also

- [lawkit](https://crates.io/crates/lawkit) - Full statistical law analysis toolkit
- [pareto](https://crates.io/crates/pareto) - Pareto principle (80/20 rule) analysis

## License

MIT
