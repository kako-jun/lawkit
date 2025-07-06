# Examples

Practical usage examples of lawkit based on real-world use cases.

## 1. Fraud Detection in Financial Auditing

### Case: Expense Report Verification

```bash
# Basic analysis of expense data
lawkit benf expenses_2024.csv --columns "Amount" --output json

# Detailed analysis with verbose output
lawkit benf expenses_2024.csv --language en --verbose

# Comprehensive analysis with multiple laws
lawkit compare expenses_2024.csv --laws benford,normal --detect-conflicts
```

**Expected Results**: 
- Deviations from Benford's Law may indicate artificial manipulation
- Normal distribution analysis identifies outliers
- Conflict detection highlights items requiring further investigation

### Case: Sales Data Reliability Verification

```bash
# Monthly sales analysis
lawkit benf monthly_sales.csv --min-value 1000 --confidence 0.99

# Regional analysis
lawkit benf sales_by_region.csv --columns "North,South,East,West,Central"
```

## 2. Business Analysis

### Case: Customer Sales Pareto Analysis

```bash
# 80/20 analysis
lawkit pareto customer_sales.csv --threshold 0.8 --gini

# 90/10 analysis (stricter top customer identification)
lawkit pareto customer_sales.csv --threshold 0.9

# Export visualization data
lawkit pareto customer_sales.csv --output csv > pareto_results.csv
```

**Applications**:
- Identify key customers contributing 80% of revenue
- Focus sales efforts on high-value segments
- Optimize customer service resource allocation

### Case: Inventory Management Analysis

```bash
# Inventory turnover analysis
lawkit pareto inventory_turnover.csv --business-analysis

# Seasonal pattern detection
lawkit normal seasonal_demand.csv --quality-control --verbose
```

## 3. Text Analysis and Content Management

### Case: Website Content Analysis

```bash
# Word frequency analysis
lawkit zipf website_content.txt --language en --top-words 100

# Content distribution analysis
lawkit zipf blog_posts.txt --min-count 5 --correlation 0.8
```

**Use Cases**:
- SEO keyword optimization
- Content strategy planning
- Natural vs. artificial content detection

### Case: Social Media Analytics

```bash
# Hashtag distribution analysis
lawkit zipf hashtags.csv --normalize --top-words 50

# Engagement pattern analysis
lawkit poisson post_engagements.csv --predict 30 --confidence 0.95
```

## 4. Quality Control and Manufacturing

### Case: Manufacturing Process Control

```bash
# Product dimension quality control
lawkit normal product_dimensions.csv --quality-control --control-limits

# Defect rate analysis
lawkit poisson defect_rates.csv --lambda auto --goodness-of-fit
```

**Quality Metrics**:
- Process capability indices (Cp, Cpk)
- Control chart limits
- Defect prediction models

### Case: Service Response Time Analysis

```bash
# Response time distribution
lawkit normal response_times.csv --outlier-detection --verbose

# Incident frequency analysis
lawkit poisson incidents.csv --time-series --predict 7
```

## 5. Integrated Analysis Workflows

### Case: Comprehensive Data Quality Assessment

```bash
# Multi-law comparison for data quality
lawkit compare financial_data.csv --laws benford,pareto,normal --focus quality

# Generate detailed quality report
lawkit compare data.csv --laws all --output json > quality_report.json
```

### Case: Automated Anomaly Detection Pipeline

```bash
#!/bin/bash
# Daily data quality pipeline

# Step 1: Basic quality check
lawkit benf daily_transactions.csv --min-count 100 || exit 1

# Step 2: Concentration analysis
lawkit pareto daily_sales.csv --gini-coefficient

# Step 3: Statistical validation
lawkit normal process_metrics.csv --quality-control

# Step 4: Comprehensive report
lawkit compare daily_data.csv --laws benford,pareto,normal --output json > daily_report.json
```

## 6. CI/CD Integration Examples

### GitHub Actions Integration

```yaml
name: Data Quality Check
on: [push, pull_request]

jobs:
  quality-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install lawkit
        run: cargo install lawkit
      - name: Run quality checks
        run: |
          lawkit benf data/transactions.csv --format json
          lawkit compare data/sales.csv --laws benford,pareto --output json
```

### Jenkins Pipeline Integration

```groovy
pipeline {
    agent any
    stages {
        stage('Data Quality Check') {
            steps {
                sh 'lawkit benf ${WORKSPACE}/data/*.csv --min-count 50'
                sh 'lawkit compare ${WORKSPACE}/data/sales.csv --laws all --output json > quality_report.json'
                archiveArtifacts artifacts: 'quality_report.json'
            }
        }
    }
}
```

## 7. Performance Optimization Examples

### Large Dataset Processing

```bash
# Enable streaming for large files
lawkit benf large_dataset.csv --optimize --min-count 1000

# Parallel processing for multiple files
find data/ -name "*.csv" | xargs -P 4 -I {} lawkit benf {} --optimize
```

### Memory-Efficient Analysis

```bash
# Process 100GB+ datasets efficiently
lawkit benf huge_data.csv --optimize --format json | jq '.risk_level'

# Streaming analysis with real-time output
tail -f live_data.log | lawkit benf --optimize --quiet
```

## Configuration Examples

See [Configuration Guide](configuration.md) for detailed setup instructions and advanced configuration options.

## Next Steps

- [Installation Guide](installation.md) - Setup and installation instructions
- [CLI Reference](../reference/cli-reference.md) - Complete command documentation
- [Integration Guide](../guides/integrations.md) - CI/CD automation
- [Performance Guide](../guides/performance.md) - Optimization techniques