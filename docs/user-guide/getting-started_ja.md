# Getting Started

Learn the basic usage of lawkit. This guide explains the main features using actual sample data.

## Basic Usage

### 1. Benford's Law Analysis

Used for fraud detection in accounting data and natural data:

```bash
# Basic analysis
lawkit benf data.csv

# Detailed output
lawkit benf data.csv --verbose

# Output in JSON format
lawkit benf data.csv --format json

# Analyze with specified threshold
lawkit benf data.csv --threshold high

# Audit-level analysis (99% confidence)
lawkit benf audit_data.csv --confidence 0.99 --verbose

# Optimization for large datasets
lawkit benf large_data.csv --sample-size 10000 --optimize

# Filter small values (noise removal)
lawkit benf financial_data.csv --min-value 100
```

### 2. Pareto Analysis (80/20 Rule)

Used for sales analysis and inventory management:

```bash
# Basic Pareto analysis
lawkit pareto sales.csv

# Specify threshold (90/10 analysis instead of 80/20)
lawkit pareto sales.csv --concentration 0.9

# Also calculate Gini coefficient
lawkit pareto sales.csv --gini-coefficient
```

### 3. Zipf's Law Analysis

Word frequency analysis for text data:

```bash
# Analyze text files
lawkit zipf document.txt

# Japanese text analysis
lawkit zipf japanese_text.txt

# Specify minimum occurrence count
lawkit zipf text.txt --min-count 5
```

### 4. Normal Distribution Analysis

Quality control and outlier detection:

```bash
# Normality testing
lawkit normal measurements.csv

# Detailed normality testing
lawkit normal quality_data.csv --verbose

# Also detect outliers
lawkit normal process_data.csv --outliers
```

### 5. Poisson Distribution Analysis

Event occurrence prediction and rare event analysis:

```bash
# Basic Poisson analysis
lawkit poisson events.csv

# Detailed Poisson analysis
lawkit poisson events.csv --verbose

# High confidence analysis
lawkit poisson critical_events.csv --confidence 0.99 --verbose
```

### 6. Multi-Law Comparison

Comprehensive analysis by applying multiple statistical laws simultaneously:

```bash
# Multi-law analysis and recommendations
lawkit analyze data.csv --laws benf,pareto,normal

# Data consistency check
lawkit validate data.csv --laws all

# Detailed diagnostic report
lawkit diagnose data.csv --focus conflict
```

## Hands-on Practice with Sample Data

### Fraud Detection in Accounting Data

```bash
# Create sample accounting data
echo "TransactionID,Amount,Date
1,1234,2024-01-01
2,2345,2024-01-02
3,3456,2024-01-03" > accounting.csv

# Analyze with Benford's Law
lawkit benf accounting.csv
```

### Pareto Analysis of Sales Data

```bash
# Create sample sales data
echo "Product,Sales
Product A,80000
Product B,12000
Product C,5000
Product D,2000
Product E,1000" > sales.csv

# Execute Pareto analysis
lawkit pareto sales.csv --threshold 0.8
```

## Output Formats

lawkit supports multiple output formats:

```bash
# Text format (default)
lawkit benf data.csv

# JSON format
lawkit benf data.csv --format json

# CSV format
lawkit benf data.csv --format csv

# YAML format
lawkit benf data.csv --format yaml

# XML format
lawkit benf data.csv --format xml
```

## Multi-language Support

lawkit supports the following languages:

```bash
# English output (default unified)
lawkit benf data.csv

# Japanese numbers are automatically recognized
echo "１２３４５６ ７８９０" | lawkit benf

# Chinese numbers are also automatically recognized
echo "一千二百三十四" | lawkit benf

# Traditional Chinese (old style) financial numbers are also supported
echo "壹萬貳仟參佰肆拾伍" | lawkit benf

# Japanese kanji numerals are automatically recognized
echo "五万六千七百八十九" | lawkit benf
```

## Advanced Features

### Filtering

```bash
# Analyze only data within specific range
lawkit benf data.csv --filter ">=1000"

# Threshold setting
lawkit pareto sales.csv --concentration 0.95
```

### Performance Tuning

```bash
# For large files, use sampling
lawkit benf large_data.csv --optimize

# Specify number of threads for parallel processing
lawkit analyze data.csv --recommend
```

## Common Workflows

### 1. Data Quality Check
```bash
# Comprehensive data quality assessment
lawkit validate financial_data.csv --laws benf,normal
```

### 2. Fraud Detection Pipeline
```bash
# Initial screening with Benford's Law
lawkit benf transactions.csv --format json > results.json

# Detailed analysis of anomalies
lawkit normal suspicious_data.csv --verbose
```

### 3. Business Analysis
```bash
# Pareto analysis of sales
lawkit pareto monthly_sales.csv --gini-coefficient

# Customer analysis
lawkit zipf customer_feedback.txt
```

## Next Steps

- [Examples](examples.md) - Real-world use cases
- [CLI Reference](../reference/cli-reference.md) - All command details
- [Architecture](../guides/architecture.md) - System design