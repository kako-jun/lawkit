# Examples

Practical usage examples of lawkit based on real-world use cases.

## 1. Fraud Detection in Financial Auditing

### Case: Expense Report Verification

```bash
# Basic analysis of expense data
lawkit benf expenses_2024.csv --columns "Amount" --output json

# Detailed analysis with verbose output
lawkit benf expenses_2024.csv --verbose

# Comprehensive analysis with multiple laws
lawkit analyze expenses_2024.csv --laws benford,normal --detect-conflicts
```

**Expected Results**: 
- Deviations from Benford Law may indicate artificial manipulation
- Normal Distribution analysis identifies outliers
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
lawkit zipf website_content.txt --text --words 100

# Content distribution analysis
lawkit zipf blog_posts.txt --text --min-count 10
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
lawkit analyze financial_data.csv --laws benford,pareto,normal --focus quality

# Generate detailed quality report
lawkit analyze data.csv --laws all --output json > quality_report.json
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
lawkit analyze daily_data.csv --laws benford,pareto,normal --output json > daily_report.json
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
          lawkit analyze data/sales.csv --laws benford,pareto --output json
```

### Jenkins Pipeline Integration

```groovy
pipeline {
    agent any
    stages {
        stage('Data Quality Check') {
            steps {
                sh 'lawkit benf ${WORKSPACE}/data/*.csv --min-count 50'
                sh 'lawkit analyze ${WORKSPACE}/data/sales.csv --laws all --output json > quality_report.json'
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
lawkit benf large_dataset.csv --optimize --min-count 1000

# Parallel processing for multiple files
find data/ -name "*.csv" | xargs -P 4 -I {} lawkit benf {}
```

### Memory-Efficient Analysis

```bash
# Process 100GB+ datasets efficiently
lawkit benf huge_data.csv --format json | jq '.risk_level'

# Streaming analysis with real-time output
tail -f live_data.log | lawkit benf --quiet
```

## 5. Generate and Self-Testing Examples

### Case: Data Generation for Testing and Education

```bash
# Generate Benford's Law samples for fraud detection testing
lawkit generate benf --samples 10000 --seed 42 > benf_test_data.txt

# Test our detection capability
lawkit benf benf_test_data.txt --format json

# Generate with fraud injection for validation
lawkit generate benf --samples 5000 --fraud-rate 0.3 > fraud_data.txt
lawkit benf fraud_data.txt --threshold critical
```

### Case: Statistical Education and Demonstration

```bash
# Demonstrate Central Limit Theorem
for i in {1..5}; do
  echo "Sample $i:"
  lawkit generate normal --samples 1000 --mean 100 --stddev 15 | 
  lawkit normal --verbose | grep "Mean:"
done

# Show Pareto Principle (80/20 rule)
lawkit generate pareto --samples 10000 --concentration 0.8 | 
lawkit pareto --business-analysis --format json | 
jq '.pareto_analysis.concentration_ratio'

# Validate Zipf's Law in generated text
lawkit generate zipf --samples 2000 --exponent 1.0 --vocabulary-size 1000 | 
lawkit zipf --text --format json | 
jq '.correlation_coefficient'
```

### Case: Method Validation and Cross-Testing

```bash
# Generate and immediately analyze pipeline
lawkit generate poisson --samples 1000 --lambda 2.5 | 
lawkit poisson --predict --format json

# Cross-validation between laws
lawkit generate normal --samples 5000 --mean 50 --stddev 10 > normal_data.txt
lawkit analyze normal_data.txt --laws normal,benf,zipf --detect-conflicts

# Comprehensive self-testing
lawkit selftest --comprehensive

# Performance testing with generated data
time lawkit generate benf --samples 100000 | lawkit benf --quiet
```

### Case: Continuous Integration Testing

```bash
#!/bin/bash
# CI/CD test script using generated data

echo "Running statistical accuracy tests..."

# Test 1: Benford's Law accuracy
BENF_RESULT=$(lawkit generate benf --samples 10000 --seed 123 | 
              lawkit benf --format json | jq -r '.risk_level')
if [ "$BENF_RESULT" != "Low" ]; then
    echo "❌ Benford test failed: Expected Low risk, got $BENF_RESULT"
    exit 1
fi

# Test 2: Normal distribution detection
NORMAL_RESULT=$(lawkit generate normal --samples 1000 --mean 0 --stddev 1 | 
                lawkit normal --test shapiro --format json | jq -r '.is_normal')
if [ "$NORMAL_RESULT" != "true" ]; then
    echo "❌ Normal test failed: Expected true, got $NORMAL_RESULT"
    exit 1
fi

# Test 3: Poisson parameter estimation
LAMBDA_RESULT=$(lawkit generate poisson --samples 5000 --lambda 3.0 | 
                lawkit poisson --format json | jq -r '.lambda')
LAMBDA_DIFF=$(echo "$LAMBDA_RESULT - 3.0" | bc -l | tr -d '-')
if (( $(echo "$LAMBDA_DIFF > 0.2" | bc -l) )); then
    echo "❌ Poisson test failed: Lambda estimation off by $LAMBDA_DIFF"
    exit 1
fi

echo "✅ All statistical accuracy tests passed"
```

## Configuration Examples

See [Configuration Guide](configuration.md) for detailed setup instructions and advanced configuration options.

## Next Steps

- [Installation Guide](installation.md) - Setup and installation instructions
- [CLI Reference](../reference/cli-reference.md) - Complete command documentation
- [Integration Guide](../guides/integrations.md) - CI/CD automation
- [Performance Guide](../guides/performance.md) - Optimization techniques