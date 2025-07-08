# Getting Started with lawkit

Welcome to `lawkit`! This guide will help you get started with the statistical law analysis toolkit.

## What is lawkit?

`lawkit` is a comprehensive toolkit for analyzing data using multiple statistical laws:

- **Benford's Law** - Detect anomalies and assess data quality
- **Pareto Principle** - 80/20 analysis and concentration measurement
- **Zipf's Law** - Frequency distribution and ranking analysis
- **Normal Distribution** - Normality testing and outlier detection
- **Poisson Distribution** - Event occurrence and rare event analysis

## Quick Start

### Basic Analysis

```bash
# Analyze with Benford's Law
lawkit benf data.csv

# Analyze with Pareto Principle
lawkit pareto sales_data.json

# Compare multiple laws
lawkit analyze financial_data.yaml
```

### Real-World Example

```bash
# Analyze financial transactions for fraud detection
lawkit benf transactions.csv --purpose fraud --verbose

# Check sales concentration (80/20 rule)
lawkit pareto sales.json --business-analysis

# Compare all laws for comprehensive analysis
lawkit analyze data.csv --report detailed
```

## Core Concepts

### Statistical Laws
Each law provides unique insights:
- **Quality assurance** → Benford's Law
- **Business efficiency** → Pareto Principle  
- **Frequency patterns** → Zipf's Law
- **Anomaly detection** → Normal Distribution
- **Event modeling** → Poisson Distribution

### Integration Analysis
Use `lawkit analyze` to:
- Run multiple laws simultaneously
- Detect conflicts between results
- Get recommendations for best approach
- Generate comprehensive reports

## Next Steps

- [Installation Guide](installation.md) - Platform-specific setup
- [CLI Reference](../reference/cli-reference.md) - Complete command documentation
- [Examples](examples.md) - Real-world use cases
- [Integration Guide](../guides/integrations.md) - CI/CD and automation