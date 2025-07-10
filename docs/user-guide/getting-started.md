# Getting Started with lawkit

Welcome to `lawkit`! This guide will help you get started with the statistical law analysis toolkit.

## What is lawkit?

`lawkit` is a comprehensive toolkit for analyzing data using multiple statistical laws:

- **Benford Law** - Detect anomalies and assess data quality
- **Pareto Principle** - 80/20 analysis and concentration measurement
- **Zipf Law** - Frequency distribution and ranking analysis
- **Normal Distribution** - Normality testing and outlier detection
- **Poisson Distribution** - Event occurrence and rare event analysis

## Quick Start

### Basic Analysis

```bash
# Analyze with Benford Law
lawkit benf data.csv

# Analyze with Pareto Principle
lawkit pareto sales_data.json

# Analyze with multiple laws
lawkit analyze financial_data.yaml
```

### Real-World Example

```bash
# Analyze financial transactions for fraud detection
lawkit benf transactions.csv --threshold high --verbose

# Check sales concentration (80/20 rule)
lawkit pareto sales.json --business-analysis

# Generate detailed diagnostic report
lawkit diagnose data.csv --focus all
```

## Core Concepts

### Statistical Laws
Each law provides unique insights:
- **Quality assurance** → Benford Law
- **Business efficiency** → Pareto Principle  
- **Frequency patterns** → Zipf Law
- **Anomaly detection** → Normal Distribution
- **Event modeling** → Poisson Distribution

### Integration Analysis
Use the following commands for different analysis needs:
- `lawkit analyze` - Run multiple laws and get recommendations
- `lawkit validate` - Check data consistency across laws
- `lawkit diagnose` - Detect conflicts and generate detailed reports

## Next Steps

- [Installation Guide](installation.md) - Platform-specific setup
- [CLI Reference](../reference/cli-reference.md) - Complete command documentation
- [Examples](examples.md) - Real-world use cases
- [Integration Guide](../guides/integrations.md) - CI/CD and automation