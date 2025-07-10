# Examples

Practical usage examples of lawkit based on real-world use cases.

## 1. Fraud Detection in Financial Auditing

### Case: Expense Report Verification

```bash
# Basic analysis of expense data
lawkit benf expenses_2024.csv --format json

# Detailed analysis with verbose output
lawkit benf expenses_2024.csv --verbose

# High-confidence audit analysis (99% confidence level)
lawkit benf expenses_2024.csv --confidence 0.99 --verbose

# Filter out small amounts that add noise
lawkit benf expenses_2024.csv --min-value 50 --threshold high

# Performance optimization for large datasets
lawkit benf expenses_2024.csv --sample-size 10000 --optimize

# Comprehensive analysis with multiple laws
lawkit analyze expenses_2024.csv --laws benford,normal
```

**Expected Results**: 
- Deviations from Benford Law may indicate artificial manipulation
- Normal Distribution analysis identifies outliers
- Multiple law analysis provides comprehensive insights

### Case: Sales Data Reliability Verification

```bash
# Monthly sales analysis
lawkit benf monthly_sales.csv --verbose

# Regional analysis
lawkit benf sales_by_region.csv --verbose
```

## 2. Business Analysis

### Case: Customer Sales Pareto Analysis

```bash
# 80/20 analysis
lawkit pareto customer_sales.csv --threshold 0.8

# 90/10 analysis (stricter top customer identification)
lawkit pareto customer_sales.csv --threshold 0.9

# Export visualization data
lawkit pareto customer_sales.csv --format csv > pareto_results.csv
```

**Applications**:
- Identify key customers contributing 80% of revenue
- Focus sales efforts on high-value segments
- Optimize customer service resource allocation

### Case: Inventory Management Analysis

```bash
# Inventory turnover analysis
lawkit pareto inventory_turnover.csv --verbose

# Seasonal pattern detection
lawkit normal seasonal_demand.csv --verbose
```

## 3. Text Analysis and Content Management

### Case: Website Content Analysis

```bash
# Word frequency analysis
lawkit zipf website_content.txt --verbose

# Content distribution analysis
lawkit zipf blog_posts.txt --verbose
```

**Use Cases**:
- SEO keyword optimization
- Content strategy planning
- Natural vs. artificial content detection

### Case: Social Media Analytics

```bash
# Hashtag distribution analysis
lawkit zipf hashtags.csv --verbose

# Engagement pattern analysis
lawkit poisson post_engagements.csv --verbose
```

## 4. Quality Control and Manufacturing

### Case: Manufacturing Process Control

```bash
# Product dimension quality control
lawkit normal product_dimensions.csv --verbose

# Defect rate analysis with high confidence
lawkit poisson defect_rates.csv --confidence 0.99 --verbose
```

**Quality Metrics**:
- Statistical process control
- Defect pattern analysis
- Quality distribution assessment

### Case: Service Response Time Analysis

```bash
# Response time distribution
lawkit normal response_times.csv --verbose

# Incident frequency analysis with confidence level
lawkit poisson incidents.csv --confidence 0.95 --verbose
```

## 5. Integrated Analysis Workflows

### Case: Comprehensive Data Quality Assessment

```bash
# Multi-law comparison for data quality
lawkit analyze financial_data.csv --laws benford,pareto,normal --purpose audit

# Generate detailed quality report
lawkit analyze data.csv --laws all --format json > quality_report.json
```

### Case: Automated Anomaly Detection Pipeline

```bash
#!/bin/bash
# Daily data quality pipeline

# Step 1: Basic quality check
lawkit benf daily_transactions.csv --verbose || exit 1

# Step 2: Concentration analysis
lawkit pareto daily_sales.csv --verbose

# Step 3: Statistical validation
lawkit normal process_metrics.csv --verbose

# Step 4: Comprehensive report
lawkit analyze daily_data.csv --laws benford,pareto,normal --format json > daily_report.json
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
          lawkit analyze data/sales.csv --laws benford,pareto --format json
```

### Jenkins Pipeline Integration

```groovy
pipeline {
    agent any
    stages {
        stage('Data Quality Check') {
            steps {
                sh 'lawkit benf ${WORKSPACE}/data/*.csv --verbose'
                sh 'lawkit analyze ${WORKSPACE}/data/sales.csv --laws all --format json > quality_report.json'
                archiveArtifacts artifacts: 'quality_report.json'
            }
        }
    }
}
```

## 7. Performance Optimization Examples

### Large Dataset Processing

```bash
# Optimized processing for large files
lawkit benf large_dataset.csv --quiet

# Parallel processing for multiple files
find data/ -name "*.csv" | xargs -P 4 -I {} lawkit benf {}
```

### Memory-Efficient Analysis

```bash
# Process large datasets efficiently
lawkit benf huge_data.csv --format json | jq '.risk_level'

# Streaming analysis with real-time output
tail -f live_data.log | lawkit benf --quiet
```

## 8. Data Generation and Testing Examples

### Case: Data Generation for Testing and Education

```bash
# Generate Benford Law samples for fraud detection testing
lawkit generate benf --samples 10000 > benf_test_data.csv

# Test our detection capability
lawkit benf benf_test_data.csv --format json

# Generate different types of data
lawkit generate pareto --samples 5000 > pareto_data.csv
lawkit generate zipf --samples 2000 > zipf_data.txt
lawkit generate normal --samples 1000 > normal_data.csv
lawkit generate poisson --samples 1000 > poisson_data.csv
```

### Case: Statistical Education and Demonstration

```bash
# Demonstrate different statistical laws
for law in benf pareto zipf normal poisson; do
  echo "Testing $law law:"
  lawkit generate $law --samples 1000 > test_data.csv
  lawkit $law test_data.csv --verbose
  echo "---"
done

# Show validation capabilities
lawkit generate benf --samples 5000 > test_benf.csv
lawkit validate test_benf.csv --laws benford
```

### Case: Method Validation and Cross-Testing

```bash
# Generate and immediately analyze pipeline
lawkit generate poisson --samples 1000 > poisson_test.csv
lawkit poisson poisson_test.csv --format json

# Cross-validation between laws
lawkit generate normal --samples 5000 > normal_data.csv
lawkit analyze normal_data.csv --laws normal,benford,zipf

# Comprehensive testing
lawkit list --help  # Show available commands
```

### Case: Continuous Integration Testing

```bash
#!/bin/bash
# CI/CD test script using generated data

echo "Running statistical accuracy tests..."

# Test 1: Benford Law accuracy
lawkit generate benf --samples 10000 > benf_test.csv
BENF_RESULT=$(lawkit benf benf_test.csv --format json | jq -r '.risk_level')
if [ "$BENF_RESULT" != "Low" ]; then
    echo "❌ Benford test failed: Expected Low risk, got $BENF_RESULT"
    exit 1
fi

# Test 2: Normal distribution detection
lawkit generate normal --samples 1000 > normal_test.csv
lawkit normal normal_test.csv --verbose

# Test 3: Poisson analysis
lawkit generate poisson --samples 5000 > poisson_test.csv
lawkit poisson poisson_test.csv --format json

echo "✅ All statistical accuracy tests passed"
```

## 9. Command Reference Examples

### Available Commands

```bash
# List all available commands
lawkit --help

# Individual command help
lawkit benf --help
lawkit pareto --help
lawkit zipf --help
lawkit normal --help
lawkit poisson --help
lawkit analyze --help
lawkit validate --help
lawkit diagnose --help
lawkit generate --help
lawkit list --help
```

### Output Format Examples

```bash
# Different output formats
lawkit benf data.csv --format json
lawkit benf data.csv --format csv
lawkit benf data.csv --format yaml
lawkit benf data.csv --format toml
lawkit benf data.csv --format xml

# Verbosity control
lawkit benf data.csv --quiet
lawkit benf data.csv --verbose
```

## Configuration Examples

See [Configuration Guide](configuration.md) for detailed setup instructions and advanced configuration options.

## Next Steps

- [Installation Guide](installation.md) - Setup and installation instructions
- [CLI Reference](../reference/cli-reference.md) - Complete command documentation
- [Integration Guide](../guides/integrations.md) - CI/CD automation
- [Performance Guide](../guides/performance.md) - Optimization techniques