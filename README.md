# lawkit

> **🔍 Multi-law statistical analysis toolkit - Uncover hidden patterns and continuously detect anomalies automatically**

[English README](README.md) | [日本語版 README](README_ja.md) | [中文版 README](README_zh.md)

[![CI](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/lawkit/actions/workflows/ci.yml)
[![Crates.io CLI](https://img.shields.io/crates/v/lawkit.svg?label=lawkit-cli)](https://crates.io/crates/lawkit)
[![Docs.rs Core](https://docs.rs/lawkit-core/badge.svg)](https://docs.rs/lawkit-core)
[![npm](https://img.shields.io/npm/v/lawkit-js.svg?label=lawkit-js)](https://www.npmjs.com/package/lawkit-js)
[![PyPI](https://img.shields.io/pypi/v/lawkit-python.svg?label=lawkit-python)](https://pypi.org/project/lawkit-python/)
[![Documentation](https://img.shields.io/badge/📚%20User%20Guide-Documentation-green)](https://github.com/kako-jun/lawkit/tree/main/docs/index.md)
[![API Reference](https://img.shields.io/badge/🔧%20API%20Reference-docs.rs-blue)](https://docs.rs/lawkit-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Why lawkit?

Traditional tools analyze one pattern at a time. lawkit analyzes multiple statistical laws together to give you the complete picture. It automatically detects conflicts, runs faster with parallel processing, and provides clear insights.

Designed for modern automation with JSON, CSV, and other structured outputs that work perfectly with AI tools and automated workflows. Ideal for fraud detection, data quality checks, and business intelligence.

```bash
# Single law analysis - Benford's Law
$ lawkit benf financial_data.csv
Benford Law Analysis Results

Dataset: financial_data.csv
Numbers analyzed: 2500
Chi-square: 12.834
p-value: 0.117
Attention: PASS

# Generate data following Pareto distribution
$ lawkit generate pareto --size 100 | head -5
4.312
1.827
12.543
2.156
6.789

# Comprehensive multi-law analysis
$ lawkit analyze --laws all data.csv
Statistical Laws Integration Analysis

Dataset: data.csv
Numbers analyzed: 1000
Laws executed: 5 (benford, pareto, zipf, normal, poisson)

Integration Metrics:
  Overall Quality: 0.743
  Consistency: 0.823
  Conflicts Detected: 2
  Recommendation Confidence: 0.892

Law Results:
  Benford Law: 0.652
  Pareto Analysis: 0.845
  Zipf Law: 0.423
  Normal Distribution: 0.912
  Poisson Distribution: 0.634

Conflicts:
  ⚠️ Benford and Zipf laws show conflicting patterns
     Cause: Different distribution assumptions
     Suggestion: Focus on Zipf analysis for frequency data

Risk Assessment: MEDIUM (Score: 0.743)
```

## ✨ Key Features

- **🎯 Multi-Law Analysis**: Benford, Pareto, Zipf, Normal, Poisson distributions with smart integration
- **🌍 International Support**: Parse numbers in 5 languages (EN, JP, CN, HI, AR) with rich output formats
- **📈 Advanced Analytics**: Time series analysis, outlier detection (LOF, Isolation Forest, DBSCAN), meta-chaining
- **⚡ High Performance**: Rust-powered parallel processing optimized for large datasets

## 📊 Performance

Real benchmark results on AMD Ryzen 5 PRO 4650U:

```bash
# Traditional tools analyze one pattern at a time
$ other-tool data.csv         # Single analysis: ~2.1s
$ lawkit benf data.csv        # Same analysis: ~180ms (11.7x faster)
$ lawkit analyze data.csv     # Multi-law analysis: ~850ms
```


## 🏗️ How It Works

```mermaid
graph TB
    A[📄 Input Data<br/>CSV, JSON, Excel, PDF...] --> B[🔍 Parse & Validate<br/>5 Language Support]
    
    B --> C1[🕵️ Benford's Law<br/>Fraud Detection]
    B --> C2[📊 Pareto Analysis<br/>80/20 Rule]
    B --> C3[🔤 Zipf's Law<br/>Frequency Analysis]
    B --> C4[📈 Normal Distribution<br/>Quality Control]
    B --> C5[⚡ Poisson Distribution<br/>Rare Events]
    
    C1 --> D1[📊 Statistical Scores]
    C2 --> D2[📊 Gini Coefficient]
    C3 --> D3[📊 Correlation Analysis]
    C4 --> D4[📊 Normality Tests]
    C5 --> D5[📊 Event Modeling]
    
    D1 --> E[🧠 Integration Engine<br/>Conflict Detection]
    D2 --> E
    D3 --> E
    D4 --> E
    D5 --> E
    
    E --> F1[⚠️ Risk Assessment<br/>Critical/High/Medium/Low]
    E --> F2[🎯 Smart Recommendations<br/>Primary/Secondary Laws]
    E --> F3[🔍 Advanced Outliers<br/>LOF, Isolation Forest, DBSCAN]
    E --> F4[📈 Time Series Analysis<br/>Trends, Seasonality, Anomalies]
    
    F1 --> G[📋 Comprehensive Report<br/>Diffai/JSON/CSV/YAML/XML]
    F2 --> G
    F3 --> G
    F4 --> G
```

lawkit analyzes your data through multiple statistical lenses simultaneously, then integrates the results to provide comprehensive insights and recommendations.

## Specification

### Supported Statistical Laws

#### 🕵️ Benford's Law - Fraud Detection
The first digit of naturally occurring numbers follows a specific distribution (1 appears ~30%, 2 appears ~18%, etc.). Deviations often indicate data manipulation, making it invaluable for:
- **Financial auditing**: Detecting manipulated accounting records
- **Election monitoring**: Identifying vote count irregularities  
- **Scientific data validation**: Spotting fabricated research data
- **Tax fraud detection**: Finding altered income/expense reports

#### 📊 Pareto Analysis - 80/20 Principle
The famous "80/20 rule" where 80% of effects come from 20% of causes. Essential for:
- **Business optimization**: Identifying top customers, products, or revenue sources
- **Resource allocation**: Focusing effort on high-impact areas
- **Quality management**: Finding the few defects causing most problems
- **Wealth distribution analysis**: Understanding economic inequality patterns

#### 🔤 Zipf's Law - Frequency Power Laws
Word frequencies follow a predictable pattern where the nth most common word appears 1/n as often as the most common word. Useful for:
- **Content analysis**: Analyzing text patterns and authenticity
- **Market research**: Understanding brand mention distributions
- **Language processing**: Detecting artificial or generated text
- **Social media analysis**: Identifying unusual posting patterns

#### 📈 Normal Distribution - Statistical Foundation
The bell-curve distribution that appears throughout nature and human behavior. Critical for:
- **Quality control**: Detecting manufacturing defects and process variations
- **Performance analysis**: Evaluating test scores, measurements, and metrics
- **Risk assessment**: Understanding natural variation vs. anomalies
- **Process improvement**: Establishing control limits and specifications

#### ⚡ Poisson Distribution - Rare Event Modeling
Models the probability of rare events occurring in fixed time/space intervals. Essential for:
- **System reliability**: Predicting failure rates and maintenance needs
- **Customer service**: Modeling call center traffic and wait times
- **Network analysis**: Understanding packet loss and connection patterns
- **Healthcare monitoring**: Tracking disease outbreaks and incident rates

### Types of Analysis

- Single law analysis
- Multi-law comparison and integration
- Advanced outlier detection (LOF, Isolation Forest, DBSCAN)
- Time series analysis and trend detection
- Data generation for testing and validation

### Output Formats

`lawkit` outputs results in multiple formats for different use cases:

- **Diffai Format (Default)**: Human-readable analysis results (superset of [diffx](https://github.com/kako-jun/diffx) format)
- **JSON/CSV/YAML/TOML/XML**: Machine-readable structured formats for automation, integration, and data processing

## Installation

### CLI Tool

```bash
# From crates.io (recommended)
cargo install lawkit

# From releases
wget https://github.com/kako-jun/lawkit/releases/latest/download/lawkit-linux-x86_64.tar.gz
tar -xzf lawkit-linux-x86_64.tar.gz
```

### Rust Library

```toml
# In your Cargo.toml
[dependencies]
lawkit-core = "2.1"
```

```rust
use lawkit_core::laws::benford::analyze_benford;
use lawkit_core::common::input::parse_text_input;

let numbers = parse_text_input("123 456 789")?;
let result = analyze_benford(&numbers, "data.txt", false)?;
println!("Chi-square: {}", result.chi_square);
```

### Package Integrations

```bash
# Node.js integration
npm install lawkit-js

# Python integration  
pip install lawkit-python
lawkit-download-binary  # Download CLI binary (available after pip install)
```

## Basic Usage

```bash
# Single law analysis with various input formats
lawkit benf financial_data.csv
lawkit pareto sales_report.json
lawkit zipf word_frequencies.txt
lawkit normal measurements.xlsx
lawkit poisson server_logs.tsv

# Multi-law analysis with different outputs
lawkit analyze --laws all transactions.csv
lawkit validate --laws all inventory.json --format yaml
lawkit diagnose --laws benf,zipf document.txt --format json

# Generate test data
lawkit generate pareto --samples 1000 > test_data.txt
lawkit generate normal --mean 100 --stddev 15 --samples 500

# Built-in time series analysis
lawkit normal monthly_sales.csv --enable-timeseries --timeseries-window 12
# Returns: trend analysis, seasonality detection, changepoints, forecasts

# Advanced filtering and analysis
lawkit analyze --laws all --filter ">=1000" financial_data.xlsx
lawkit benf sales_data.csv --format xml

# Pipeline usage
cat raw_numbers.txt | lawkit benf -
lawkit generate zipf --samples 10000 | lawkit analyze --laws all -

# Meta-chaining with diffx for time series analysis
lawkit benf sales_2023.csv > analysis_2023.txt
lawkit benf sales_2024.csv > analysis_2024.txt
diffx analysis_2023.txt analysis_2024.txt  # Detect changes in statistical patterns

# Continuous monitoring pipeline
for month in {01..12}; do
  lawkit analyze --laws all sales_2024_${month}.csv > analysis_${month}.txt
done
diffx analysis_*.txt --chain  # Visualize pattern evolution over time
```

## 🔗 Meta-Chaining: Tracking Pattern Evolution

Meta-chaining combines lawkit's built-in time series analysis with diffx for long-term pattern tracking:

```mermaid
graph LR
    A[Jan Data] -->|lawkit| B[Jan Analysis]
    C[Feb Data] -->|lawkit| D[Feb Analysis]
    E[Mar Data] -->|lawkit| F[Mar Analysis]
    
    B -->|diffx| G[Pattern Changes<br/>Jan→Feb]
    D -->|diffx| G
    D -->|diffx| H[Pattern Changes<br/>Feb→Mar]
    F -->|diffx| H
    
    G -->|trend| I[Pattern<br/>Evolution]
    H -->|trend| I
    
    style I stroke:#0288d1,stroke-width:3px
```

**Built-in Time Series Analysis** (single dataset):
- Trend detection with R-squared analysis
- Automatic seasonality detection and decomposition
- Changepoint identification (level, trend, variance shifts)
- Forecasting with confidence intervals
- Anomaly detection and data quality assessment

**Meta-chaining with diffx** (multiple time periods):
- Gradual drift in Benford compliance (potential fraud buildup)
- Long-term evolution of statistical patterns
- Cross-period anomaly comparison
- Historical pattern baseline establishment

## Documentation

For comprehensive guides, examples, and API documentation:

📚 **[User Guide](https://github.com/kako-jun/lawkit/tree/main/docs/index.md)** - Installation, usage, and examples  
🔧 **[CLI Reference](https://github.com/kako-jun/lawkit/tree/main/docs/reference/cli-reference.md)** - Complete command documentation  
📊 **[Statistical Laws Guide](https://github.com/kako-jun/lawkit/tree/main/docs/user-guide/examples.md)** - Detailed analysis examples  
⚡ **[Performance Guide](https://github.com/kako-jun/lawkit/tree/main/docs/guides/performance.md)** - Optimization and large datasets  
🌍 **[International Support](https://github.com/kako-jun/lawkit/tree/main/docs/user-guide/configuration.md)** - Multi-language number parsing

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) for details.

