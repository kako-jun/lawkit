# benf - Benford's Law Analysis

A convenience command for Benford's Law analysis, part of the [**lawkit**](https://github.com/kako-jun/lawkit) statistical analysis toolkit.

## Quick Start

```bash
# Install both benf and lawkit
cargo install lawkit benf

# Basic analysis
echo "123 456 789 101 234" | benf

# File analysis
benf data.csv --format json

# Advanced options (requires lawkit)
lawkit benf data.xlsx --verbose --filter ">=1000"
```

## What is Benford's Law?

Benford's Law states that in many naturally occurring datasets, the first digit 1 appears about 30% of the time, 2 appears about 18% of the time, and so on. Deviations from this pattern often indicate:

- **Financial fraud** in accounting data
- **Data manipulation** in scientific studies  
- **Errors** in data collection or processing

## Features

- **International number formats**: Japanese (１２３), Chinese (一二三), Arabic (١٢٣)
- **Multiple file formats**: CSV, JSON, YAML, Excel, PDF, Word documents
- **Statistical rigor**: Chi-square test, p-values, confidence intervals
- **Risk assessment**: Automatic anomaly detection with severity levels

## Full Statistical Toolkit

This `benf` command is a convenience wrapper for `lawkit benf`. For complete statistical analysis capabilities, use **lawkit**:

### Available Statistical Laws
- **Benford's Law**: `lawkit benf` - Fraud detection and anomaly analysis
- **Pareto Principle**: `lawkit pareto` - 80/20 rule and concentration measurement  
- **Zipf's Law**: `lawkit zipf` - Power-law distribution analysis
- **Normal Distribution**: `lawkit normal` - Normality testing and outlier detection
- **Poisson Distribution**: `lawkit poisson` - Rare event analysis

### Advanced Features
- **Multi-law comparison**: `lawkit compare data.csv`
- **Data generation**: `lawkit generate benf --samples 1000`
- **Performance optimization**: `--optimize` flag for large datasets
- **Comprehensive reporting**: Multiple output formats with detailed statistics

## Installation

```bash
# Install the full toolkit (recommended)
cargo install lawkit

# Or install just this convenience command
cargo install benf

# Or download binaries
# https://github.com/kako-jun/lawkit/releases
```

## Examples

```bash
# Basic fraud detection
benf expenses.csv

# Financial audit with detailed output
benf quarterly-report.xlsx --verbose --format json

# Process large datasets efficiently  
lawkit benf big-data.csv --optimize --min-count 1000

# Compare multiple statistical laws
lawkit compare financial-data.csv

# Generate test data for validation
lawkit generate benf --samples 10000 | lawkit benf --format json
```

## Documentation

- **Complete Documentation**: https://github.com/kako-jun/lawkit/tree/main/docs
- **API Reference**: https://docs.rs/lawkit-core
- **Usage Examples**: https://github.com/kako-jun/lawkit/blob/main/docs/user-guide/examples.md

## License

MIT License - see [LICENSE](https://github.com/kako-jun/lawkit/blob/main/LICENSE) for details.

---

**💡 Tip**: For professional statistical analysis with multiple laws and advanced features, use the full `lawkit` command instead of individual law commands.