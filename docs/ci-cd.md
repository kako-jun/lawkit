# CI/CD Integration Guide

This guide shows how to integrate lawkit into your continuous integration and deployment pipelines for automated data quality monitoring and fraud detection.

## Table of Contents

- [GitHub Actions](#github-actions)
- [GitLab CI](#gitlab-ci)
- [Jenkins](#jenkins)
- [Docker Integration](#docker-integration)
- [Data Quality Gates](#data-quality-gates)
- [Monitoring and Alerting](#monitoring-and-alerting)

## GitHub Actions

### Basic Data Quality Check

```yaml
name: Data Quality Check

on:
  push:
    paths:
      - 'data/**'
  pull_request:
    paths:
      - 'data/**'

jobs:
  data-quality:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Download lawkit
      run: |
        wget -O lawkit https://github.com/kako-jun/lawkit/releases/latest/download/lawkit-linux-x86_64.tar.gz
        tar -xzf lawkit-linux-x86_64.tar.gz
        chmod +x lawkit
        sudo mv lawkit /usr/local/bin/
    
    - name: Run Benford's Law Analysis
      run: |
        lawkit benf --format json data/transactions.csv > benford_results.json
        
    - name: Check for Anomalies
      run: |
        # Extract chi-square value and check threshold
        chi_square=$(jq '.chi_square' benford_results.json)
        if (( $(echo "$chi_square > 20.0" | bc -l) )); then
          echo "::error::High chi-square value detected: $chi_square"
          exit 1
        fi
    
    - name: Upload Results
      uses: actions/upload-artifact@v3
      with:
        name: analysis-results
        path: '*_results.json'
```

### Multi-Dataset Analysis

```yaml
name: Comprehensive Data Analysis

on:
  schedule:
    - cron: '0 2 * * *'  # Daily at 2 AM
  workflow_dispatch:

jobs:
  analyze-datasets:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        dataset: [transactions, customers, inventory, sales]
        law: [benf, pareto, normal]
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup lawkit
      run: |
        wget -O lawkit.tar.gz https://github.com/kako-jun/lawkit/releases/latest/download/lawkit-linux-x86_64.tar.gz
        tar -xzf lawkit.tar.gz
        chmod +x lawkit
        sudo mv lawkit /usr/local/bin/
    
    - name: Run Analysis
      run: |
        lawkit ${{ matrix.law }} --format json --verbose \
          data/${{ matrix.dataset }}.csv > results_${{ matrix.dataset }}_${{ matrix.law }}.json
    
    - name: Quality Gate Check
      run: |
        python scripts/quality_gate.py results_${{ matrix.dataset }}_${{ matrix.law }}.json
    
    - name: Store Results
      uses: actions/upload-artifact@v3
      with:
        name: analysis-${{ matrix.dataset }}-${{ matrix.law }}
        path: results_*.json
```

### Integration with Pull Requests

```yaml
name: PR Data Validation

on:
  pull_request:
    types: [opened, synchronize]

jobs:
  validate-data-changes:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0  # Full history for comparison
    
    - name: Setup lawkit
      run: |
        # Install lawkit (same as above)
        
    - name: Get changed files
      id: changed-files
      run: |
        echo "files=$(git diff --name-only origin/main...HEAD | grep '\.csv$' | tr '\n' ' ')" >> $GITHUB_OUTPUT
    
    - name: Analyze changed datasets
      if: steps.changed-files.outputs.files != ''
      run: |
        for file in ${{ steps.changed-files.outputs.files }}; do
          echo "Analyzing $file"
          lawkit compare --laws all --format json "$file" > "analysis_$(basename $file .csv).json"
          
          # Check for high-risk indicators
          risk_level=$(jq -r '.overall_risk_level' "analysis_$(basename $file .csv).json")
          if [ "$risk_level" = "high" ] || [ "$risk_level" = "critical" ]; then
            echo "::error file=$file::High risk detected in dataset: $risk_level"
          fi
        done
    
    - name: Comment PR with results
      uses: actions/github-script@v6
      with:
        script: |
          const fs = require('fs');
          const files = '${{ steps.changed-files.outputs.files }}'.split(' ').filter(f => f);
          
          let comment = '## Data Quality Analysis Results\n\n';
          
          for (const file of files) {
            const resultFile = `analysis_${file.replace('.csv', '')}.json`;
            if (fs.existsSync(resultFile)) {
              const result = JSON.parse(fs.readFileSync(resultFile, 'utf8'));
              comment += `### ${file}\n`;
              comment += `- Risk Level: ${result.overall_risk_level}\n`;
              comment += `- Applicable Laws: ${result.applicable_laws.join(', ')}\n\n`;
            }
          }
          
          github.rest.issues.createComment({
            issue_number: context.issue.number,
            owner: context.repo.owner,
            repo: context.repo.repo,
            body: comment
          });
```

## GitLab CI

### Basic Pipeline

```yaml
# .gitlab-ci.yml
stages:
  - quality-check
  - analysis
  - report

variables:
  LAWKIT_VERSION: "2.0.1"

.install_lawkit: &install_lawkit
  - wget -O lawkit.tar.gz "https://github.com/kako-jun/lawkit/releases/download/v${LAWKIT_VERSION}/lawkit-linux-x86_64.tar.gz"
  - tar -xzf lawkit.tar.gz
  - chmod +x lawkit
  - mv lawkit /usr/local/bin/

data_quality_check:
  stage: quality-check
  image: ubuntu:latest
  before_script:
    - apt-get update && apt-get install -y wget
    - *install_lawkit
  script:
    - lawkit benf --threshold high data/financial_data.csv
    - lawkit pareto --verbose data/customer_data.csv
  rules:
    - changes:
        - data/**/*
  artifacts:
    reports:
      junit: test-results.xml
    when: always

comprehensive_analysis:
  stage: analysis
  image: ubuntu:latest
  before_script:
    - apt-get update && apt-get install -y wget jq
    - *install_lawkit
  script:
    - |
      for dataset in data/*.csv; do
        echo "Analyzing $dataset"
        lawkit compare --laws all --format json "$dataset" > "analysis_$(basename $dataset .csv).json"
      done
    - |
      # Aggregate results
      jq -s '.' analysis_*.json > aggregated_results.json
  artifacts:
    paths:
      - "*.json"
    expire_in: 30 days

generate_report:
  stage: report
  image: ubuntu:latest
  dependencies:
    - comprehensive_analysis
  script:
    - |
      # Generate HTML report
      python3 scripts/generate_report.py aggregated_results.json > data_quality_report.html
  artifacts:
    paths:
      - data_quality_report.html
    expire_in: 7 days
```

### Scheduled Analysis

```yaml
# Scheduled daily analysis
daily_analysis:
  stage: analysis
  image: ubuntu:latest
  before_script:
    - apt-get update && apt-get install -y wget jq bc
    - *install_lawkit
  script:
    - |
      # Download fresh data
      ./scripts/fetch_daily_data.sh
      
      # Run comprehensive analysis
      lawkit compare --laws all --format json daily_data.csv > daily_analysis.json
      
      # Check thresholds
      risk_level=$(jq -r '.overall_risk_level' daily_analysis.json)
      if [ "$risk_level" = "high" ] || [ "$risk_level" = "critical" ]; then
        echo "ALERT: High risk detected in daily data"
        # Send notification
        ./scripts/send_alert.sh "$risk_level"
      fi
  only:
    - schedules
  artifacts:
    paths:
      - daily_analysis.json
```

## Jenkins

### Pipeline Script

```groovy
pipeline {
    agent any
    
    parameters {
        choice(
            name: 'ANALYSIS_TYPE',
            choices: ['benf', 'pareto', 'normal', 'compare'],
            description: 'Type of statistical analysis to perform'
        )
        string(
            name: 'DATA_PATH',
            defaultValue: 'data/transactions.csv',
            description: 'Path to data file'
        )
    }
    
    environment {
        LAWKIT_VERSION = '2.0.1'
    }
    
    stages {
        stage('Setup') {
            steps {
                script {
                    // Download and install lawkit
                    sh '''
                        wget -O lawkit.tar.gz "https://github.com/kako-jun/lawkit/releases/download/v${LAWKIT_VERSION}/lawkit-linux-x86_64.tar.gz"
                        tar -xzf lawkit.tar.gz
                        chmod +x lawkit
                        sudo mv lawkit /usr/local/bin/
                    '''
                }
            }
        }
        
        stage('Data Quality Analysis') {
            steps {
                script {
                    def analysisCmd = "lawkit ${params.ANALYSIS_TYPE} --format json --verbose ${params.DATA_PATH}"
                    
                    // Run analysis
                    sh "${analysisCmd} > analysis_results.json"
                    
                    // Parse results
                    def results = readJSON file: 'analysis_results.json'
                    
                    // Quality gate
                    if (results.risk_level in ['high', 'critical']) {
                        error("High risk detected in data analysis: ${results.risk_level}")
                    }
                    
                    // Store results
                    archiveArtifacts artifacts: 'analysis_results.json'
                    
                    // Publish results
                    publishHTML([
                        allowMissing: false,
                        alwaysLinkToLastBuild: true,
                        keepAll: true,
                        reportDir: '.',
                        reportFiles: 'analysis_results.json',
                        reportName: 'Data Quality Report'
                    ])
                }
            }
        }
        
        stage('Trend Analysis') {
            steps {
                script {
                    // Compare with historical data
                    sh '''
                        # Get historical results
                        cp analysis_results.json "historical/$(date +%Y%m%d)_results.json"
                        
                        # Generate trend report
                        python3 scripts/trend_analysis.py historical/ > trend_report.html
                    '''
                    
                    publishHTML([
                        allowMissing: false,
                        alwaysLinkToLastBuild: true,
                        keepAll: true,
                        reportDir: '.',
                        reportFiles: 'trend_report.html',
                        reportName: 'Trend Analysis'
                    ])
                }
            }
        }
    }
    
    post {
        always {
            // Clean up
            sh 'rm -f lawkit.tar.gz'
        }
        failure {
            // Send notification on failure
            emailext (
                subject: "Data Quality Check Failed - ${env.JOB_NAME} #${env.BUILD_NUMBER}",
                body: "The data quality analysis has detected issues. Please check the build logs.",
                to: "${env.CHANGE_AUTHOR_EMAIL}"
            )
        }
    }
}
```

### Freestyle Job Configuration

```bash
#!/bin/bash
# Jenkins freestyle job build script

# Parameters
DATA_FILE=${DATA_FILE:-"data/transactions.csv"}
ANALYSIS_TYPE=${ANALYSIS_TYPE:-"benf"}
THRESHOLD=${THRESHOLD:-"medium"}

# Setup
wget -O lawkit.tar.gz "https://github.com/kako-jun/lawkit/releases/latest/download/lawkit-linux-x86_64.tar.gz"
tar -xzf lawkit.tar.gz
chmod +x lawkit

# Analysis
./lawkit ${ANALYSIS_TYPE} --threshold ${THRESHOLD} --format json ${DATA_FILE} > results.json

# Quality gate
RISK_LEVEL=$(jq -r '.risk_level' results.json)
case $RISK_LEVEL in
    "critical"|"high")
        echo "FAILED: High risk detected - $RISK_LEVEL"
        exit 1
        ;;
    "medium")
        echo "WARNING: Medium risk detected"
        ;;
    "low")
        echo "PASSED: Low risk"
        ;;
esac

# Archive results
mkdir -p artifacts
cp results.json artifacts/
```

## Docker Integration

### Dockerfile for lawkit

```dockerfile
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    wget \
    ca-certificates \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Install lawkit
ARG LAWKIT_VERSION=2.0.1
RUN wget -O lawkit.tar.gz "https://github.com/kako-jun/lawkit/releases/download/v${LAWKIT_VERSION}/lawkit-linux-x86_64.tar.gz" \
    && tar -xzf lawkit.tar.gz \
    && chmod +x lawkit \
    && mv lawkit /usr/local/bin/ \
    && rm lawkit.tar.gz

WORKDIR /data

ENTRYPOINT ["lawkit"]
CMD ["--help"]
```

### Docker Compose for Data Pipeline

```yaml
version: '3.8'

services:
  data-quality-check:
    build: .
    volumes:
      - ./data:/data
      - ./results:/results
    command: >
      sh -c "
        lawkit benf --format json /data/transactions.csv > /results/benf_results.json &&
        lawkit pareto --format json /data/sales.csv > /results/pareto_results.json &&
        lawkit compare --laws all --format json /data/combined.csv > /results/comparison.json
      "
    
  report-generator:
    image: python:3.9-slim
    depends_on:
      - data-quality-check
    volumes:
      - ./results:/results
      - ./scripts:/scripts
    command: python /scripts/generate_report.py /results
    
  alerting:
    image: alpine:latest
    depends_on:
      - data-quality-check
    volumes:
      - ./results:/results
      - ./scripts:/scripts
    command: /scripts/check_alerts.sh /results
```

### Kubernetes Job

```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: data-quality-analysis
spec:
  template:
    spec:
      containers:
      - name: lawkit-analyzer
        image: lawkit:latest
        command: ["sh", "-c"]
        args:
          - |
            lawkit compare --laws all --format json /data/input.csv > /data/output.json
            if [ $(jq -r '.overall_risk_level' /data/output.json) = "critical" ]; then
              echo "Critical risk detected!"
              exit 1
            fi
        volumeMounts:
        - name: data-volume
          mountPath: /data
      volumes:
      - name: data-volume
        persistentVolumeClaim:
          claimName: data-pvc
      restartPolicy: Never
  backoffLimit: 3
```

## Data Quality Gates

### Python Quality Gate Script

```python
#!/usr/bin/env python3
# scripts/quality_gate.py

import json
import sys
import argparse

def check_quality_gates(result_file, config_file=None):
    """Check data quality gates based on analysis results."""
    
    # Default thresholds
    thresholds = {
        'chi_square_max': 20.0,
        'gini_coefficient_max': 0.8,
        'correlation_min': 0.7,
        'p_value_min': 0.05,
        'risk_levels_fail': ['critical', 'high']
    }
    
    # Load custom thresholds if provided
    if config_file:
        with open(config_file) as f:
            custom_thresholds = json.load(f)
            thresholds.update(custom_thresholds)
    
    # Load analysis results
    with open(result_file) as f:
        results = json.load(f)
    
    failures = []
    warnings = []
    
    # Check risk level
    risk_level = results.get('risk_level', 'unknown')
    if risk_level in thresholds['risk_levels_fail']:
        failures.append(f"Risk level too high: {risk_level}")
    elif risk_level == 'medium':
        warnings.append(f"Medium risk detected: {risk_level}")
    
    # Check chi-square (Benford's law)
    if 'chi_square' in results:
        chi_square = results['chi_square']
        if chi_square > thresholds['chi_square_max']:
            failures.append(f"Chi-square too high: {chi_square} > {thresholds['chi_square_max']}")
    
    # Check Gini coefficient (Pareto)
    if 'gini_coefficient' in results:
        gini = results['gini_coefficient']
        if gini > thresholds['gini_coefficient_max']:
            warnings.append(f"High inequality detected: Gini = {gini}")
    
    # Check correlation (Zipf's law)
    if 'correlation' in results:
        correlation = results['correlation']
        if correlation < thresholds['correlation_min']:
            failures.append(f"Poor correlation: {correlation} < {thresholds['correlation_min']}")
    
    # Check p-values (normality tests)
    if 'normality_tests' in results:
        for test, result in results['normality_tests'].items():
            if result.get('p_value', 1.0) < thresholds['p_value_min']:
                warnings.append(f"Non-normal distribution detected: {test} p-value = {result['p_value']}")
    
    # Report results
    if failures:
        print("QUALITY GATE FAILED:")
        for failure in failures:
            print(f"  ❌ {failure}")
        return 1
    
    if warnings:
        print("QUALITY GATE PASSED WITH WARNINGS:")
        for warning in warnings:
            print(f"  ⚠️  {warning}")
    else:
        print("QUALITY GATE PASSED:")
        print("  ✅ All checks passed")
    
    return 0

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Check data quality gates")
    parser.add_argument("result_file", help="Analysis result JSON file")
    parser.add_argument("--config", help="Custom thresholds configuration file")
    
    args = parser.parse_args()
    
    exit_code = check_quality_gates(args.result_file, args.config)
    sys.exit(exit_code)
```

### Bash Quality Gate Script

```bash
#!/bin/bash
# scripts/check_alerts.sh

RESULTS_DIR=$1
ALERT_THRESHOLD=${ALERT_THRESHOLD:-"high"}

echo "Checking for alerts in $RESULTS_DIR"

for result_file in "$RESULTS_DIR"/*.json; do
    if [ -f "$result_file" ]; then
        echo "Checking $(basename "$result_file")"
        
        # Extract risk level
        risk_level=$(jq -r '.risk_level // .overall_risk_level // "unknown"' "$result_file")
        
        case $risk_level in
            "critical")
                echo "🚨 CRITICAL ALERT: $(basename "$result_file")"
                # Send critical alert
                curl -X POST "$WEBHOOK_URL" -H "Content-Type: application/json" \
                     -d "{\"text\": \"Critical data quality issue detected in $(basename "$result_file")\"}"
                ;;
            "high")
                if [ "$ALERT_THRESHOLD" = "high" ]; then
                    echo "⚠️  HIGH ALERT: $(basename "$result_file")"
                    # Send high priority alert
                fi
                ;;
            "medium")
                echo "ℹ️  Medium risk: $(basename "$result_file")"
                ;;
            "low")
                echo "✅ Normal: $(basename "$result_file")"
                ;;
            *)
                echo "❓ Unknown risk level: $risk_level in $(basename "$result_file")"
                ;;
        esac
    fi
done
```

## Monitoring and Alerting

### Prometheus Metrics Export

```python
#!/usr/bin/env python3
# scripts/export_metrics.py

import json
import sys
from datetime import datetime

def export_prometheus_metrics(result_file):
    """Export lawkit results as Prometheus metrics."""
    
    with open(result_file) as f:
        results = json.load(f)
    
    timestamp = int(datetime.now().timestamp() * 1000)
    
    # Risk level as numeric value
    risk_levels = {'low': 1, 'medium': 2, 'high': 3, 'critical': 4}
    risk_value = risk_levels.get(results.get('risk_level', 'unknown'), 0)
    
    print(f"# HELP lawkit_risk_level Current risk level (1=low, 2=medium, 3=high, 4=critical)")
    print(f"# TYPE lawkit_risk_level gauge")
    print(f"lawkit_risk_level {risk_value} {timestamp}")
    
    # Chi-square value (Benford's law)
    if 'chi_square' in results:
        print(f"# HELP lawkit_chi_square Chi-square test statistic")
        print(f"# TYPE lawkit_chi_square gauge")
        print(f"lawkit_chi_square {results['chi_square']} {timestamp}")
    
    # Gini coefficient (Pareto)
    if 'gini_coefficient' in results:
        print(f"# HELP lawkit_gini_coefficient Gini coefficient of inequality")
        print(f"# TYPE lawkit_gini_coefficient gauge")
        print(f"lawkit_gini_coefficient {results['gini_coefficient']} {timestamp}")
    
    # Correlation (Zipf's law)
    if 'correlation' in results:
        print(f"# HELP lawkit_correlation Correlation coefficient")
        print(f"# TYPE lawkit_correlation gauge")
        print(f"lawkit_correlation {results['correlation']} {timestamp}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: export_metrics.py <result_file.json>")
        sys.exit(1)
    
    export_prometheus_metrics(sys.argv[1])
```

### Grafana Dashboard Configuration

```json
{
  "dashboard": {
    "title": "Data Quality Monitoring",
    "panels": [
      {
        "title": "Risk Level",
        "type": "stat",
        "targets": [
          {
            "expr": "lawkit_risk_level",
            "legendFormat": "Risk Level"
          }
        ],
        "fieldConfig": {
          "defaults": {
            "thresholds": {
              "steps": [
                {"color": "green", "value": 0},
                {"color": "yellow", "value": 2},
                {"color": "red", "value": 3}
              ]
            }
          }
        }
      },
      {
        "title": "Chi-Square Trend",
        "type": "timeseries",
        "targets": [
          {
            "expr": "lawkit_chi_square",
            "legendFormat": "Chi-Square"
          }
        ]
      }
    ]
  }
}
```

This CI/CD integration guide provides comprehensive examples for incorporating lawkit into your automated data quality monitoring workflows. Adapt the examples to your specific infrastructure and requirements.