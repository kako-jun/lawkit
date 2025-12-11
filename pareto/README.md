# pareto

[![Crates.io](https://img.shields.io/crates/v/pareto.svg)](https://crates.io/crates/pareto)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Pareto Principle (80/20 rule) analysis CLI tool. Analyze wealth distribution, resource concentration, and identify optimization opportunities.

> **Note**: This is a convenience wrapper for `lawkit pareto`. For additional statistical laws (Benford, Zipf, Normal, Poisson) and advanced features, see [lawkit](https://crates.io/crates/lawkit).

## Installation

```bash
cargo install pareto
```

## Usage

```bash
# Analyze a file
pareto sales.csv

# From stdin
cat revenue.txt | pareto -

# Output formats
pareto data.csv --format json
pareto data.csv --format csv
```

## Options

```bash
-f, --format <FORMAT>      Output: text, csv, json, yaml, toml, xml
-q, --quiet                Minimal output
-v, --verbose              Detailed output
--filter <RANGE>           Filter numbers (e.g., >=100, <1000)
--percentiles <LIST>       Custom percentiles (e.g., 10,20,30)
--gini-coefficient         Show Gini coefficient
--business-analysis        Show business interpretation
--no-color                 Disable colors
```

## Output

- **Top 20% share**: Percentage of total owned by top 20%
- **Pareto ratio**: How close to ideal 80/20 distribution
- **Concentration index**: Gini coefficient (0=equal, 1=concentrated)
- **Lorenz curve**: Visual distribution graph

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | LOW/MEDIUM risk - distribution is healthy |
| 10 | HIGH risk - significant imbalance |
| 11 | CRITICAL risk - extreme concentration |

## See Also

- [lawkit](https://crates.io/crates/lawkit) - Full statistical law analysis toolkit
- [benf](https://crates.io/crates/benf) - Benford's Law analysis

## License

MIT
