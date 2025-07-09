# pareto - Pareto Principle (80/20 Rule) Analysis

A convenience command for Pareto Principle analysis, part of the [**lawkit**](https://github.com/kako-jun/lawkit) statistical analysis toolkit.

## Quick Start

```bash
# Install both pareto and lawkit
cargo install lawkit pareto

# Basic 80/20 analysis
echo "100 50 30 20 10 5 3 2 1" | pareto

# Business analysis
pareto sales-data.csv --business-analysis

# Advanced analysis (requires lawkit)
lawkit pareto revenue.xlsx --gini-coefficient --percentiles 70,80,90,95
```

## What is the Pareto Principle?

The Pareto Principle (80/20 rule) states that roughly 80% of effects come from 20% of causes. This pattern appears in:

- **Business**: 80% of revenue from 20% of customers
- **Quality Control**: 80% of problems from 20% of defects
- **Economics**: 80% of wealth held by 20% of population
- **Productivity**: 80% of results from 20% of effort

## Features

- **Gini Coefficient**: Measure inequality and concentration
- **Custom Percentiles**: Analyze any ratio (70/30, 90/10, etc.)
- **Business Insights**: Automated recommendations for resource allocation
- **International Support**: Japanese (１２３), Chinese (一二三), Arabic (١٢٣) numbers
- **Multiple Formats**: CSV, JSON, YAML, Excel, PDF, Word documents

## Usage Examples

```bash
# Identify top revenue drivers
pareto customer-revenue.csv --business-analysis

# Quality control analysis
pareto defect-counts.csv --gini-coefficient

# Custom concentration analysis
pareto wealth-distribution.csv --percentiles 80,90,95,99

# Resource allocation planning
lawkit pareto project-efforts.xlsx --verbose --format json
```

## Full Statistical Toolkit

This `pareto` command is a convenience wrapper for `lawkit pareto`. For complete statistical analysis capabilities, use **lawkit**:

### Available Statistical Laws
- **Benford Law**: `lawkit benf` - Fraud detection and anomaly analysis
- **Pareto Principle**: `lawkit pareto` - 80/20 rule and concentration measurement  
- **Zipf Law**: `lawkit zipf` - Power-law distribution analysis
- **Normal Distribution**: `lawkit normal` - Normality testing and outlier detection
- **Poisson Distribution**: `lawkit poisson` - Rare event analysis

### Advanced Features
- **Multi-law comparison**: `lawkit compare data.csv`
- **Data generation**: `lawkit generate pareto --samples 1000 --concentration 0.8`
- **Performance optimization**: `--optimize` flag for large datasets
- **Comprehensive reporting**: Multiple output formats with detailed statistics

## Business Intelligence Applications

```bash
# Customer Revenue Analysis
pareto customer-sales.csv --business-analysis
# Output: "Focus on top 23% customers generating 80% revenue"

# Product Performance Review
pareto product-profits.csv --gini-coefficient
# Output: Gini = 0.65 (high concentration, few star products)

# Resource Allocation Planning
pareto task-impacts.csv --percentiles 70,80,90
# Output: Top 20% tasks deliver 78% of value

# Inventory Optimization
lawkit pareto inventory-turnover.xlsx --verbose --optimize
# Comprehensive analysis for warehouse management
```

## Installation

```bash
# Install the full toolkit (recommended)
cargo install lawkit

# Or install just this convenience command
cargo install pareto

# Or download binaries
# https://github.com/kako-jun/lawkit/releases
```

## Output Interpretation

- **Concentration Ratio**: How much the top 20% contributes to the total
- **Gini Coefficient**: 0 = perfect equality, 1 = maximum inequality
- **Business Recommendations**: Automated insights for decision making
- **Risk Assessment**: Identifies over-dependence on few elements

## Documentation

- **Complete Documentation**: https://github.com/kako-jun/lawkit/tree/main/docs
- **API Reference**: https://docs.rs/lawkit-core
- **Business Examples**: https://github.com/kako-jun/lawkit/blob/main/docs/user-guide/examples.md

## License

MIT License - see [LICENSE](https://github.com/kako-jun/lawkit/blob/main/LICENSE) for details.

---

**💡 Tip**: For comprehensive business intelligence with multiple statistical laws and advanced analytics, use the full `lawkit` command instead of individual law commands.