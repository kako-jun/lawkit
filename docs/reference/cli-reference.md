# CLI Reference

Complete command-line interface documentation for `lawkit`.

## Global Commands

### `lawkit --help`
Display help information for the main command or subcommands.

### `lawkit --version`
Show version information.

### `lawkit list`
List all available statistical laws and their descriptions.

```bash
$ lawkit list
Available statistical laws:
  benf    - Benford Law analysis
  pareto  - Pareto Principle (80/20 rule) analysis
  zipf    - Zipf Law analysis
  normal  - Normal Distribution analysis
  poisson - Poisson Distribution analysis

Integration commands:
  analyze  - Multi-law basic analysis and recommendations
  validate - Data validation and consistency checks  
  diagnose - Conflict detection and detailed diagnostics
```

## Statistical Law Commands

### `lawkit benf` - Benford Law Analysis

Detect anomalies and assess data quality using Benford Law.

```bash
lawkit benf [OPTIONS] [INPUT]
```

#### Options
- `--format <FORMAT>` - Output format: text, json, csv, yaml, toml, xml (default: text)
- `--quiet, -q` - Minimal output (numbers only)
- `--verbose, -v` - Enable verbose debugging output with detailed analysis insights
- `--filter <RANGE>` - Filter numbers by range (e.g., >=100, <1000, 50-500)
- `--min-count <NUMBER>` - Minimum number of data points required for analysis (default: 10)
- `--optimize` - Enable memory and processing optimizations for large datasets
- `--threshold <LEVEL>` - Anomaly detection threshold: low, medium, high, critical (default: auto)
- `--confidence <LEVEL>` - Statistical confidence level for tests (0.01-0.99, default: 0.95)
- `--sample-size <NUMBER>` - Maximum sample size for large datasets (improves performance)
- `--min-value <VALUE>` - Minimum value to include in analysis (filters small values that add noise)

#### Verbose Output
The `--verbose` flag provides comprehensive debugging and analysis information:

**Debug Information:**
- Input argument detection and validation
- Data processing strategy (automatic optimization, streaming)
- Filter application with before/after statistics
- Data collection and parsing details

**Performance Metrics:**
- Processing time in milliseconds
- Memory usage in MB
- Number of chunks processed for large datasets
- Items processed counts

**Analysis Insights:**
- Statistical computation steps
- Confidence interval details
- Algorithm selection reasoning
- Data quality assessment

Example verbose output:
```bash
$ echo "123 456 789" | lawkit benf --verbose
Debug: input argument = None
Debug: Reading from stdin, using automatic optimization
Debug: Using automatic optimization (streaming + incremental + memory efficiency)
Debug: Collected 3 numbers from stream
Debug: Streaming analysis successful - 3 items processed
Debug: Processed 3 numbers in 1 chunks
Debug: Memory used: 0.00 MB
Debug: Processing time: 1 ms

# Standard analysis output follows...
```

#### Examples
```bash
# Basic analysis
lawkit benf data.csv

# Detailed output with JSON format
lawkit benf transactions.json --verbose --format json

# Quiet mode for minimal output
lawkit benf data.csv --quiet

# Filter large transactions with high threshold
lawkit benf accounts.csv --filter ">=1000" --threshold high

# High confidence analysis for auditing (99% confidence level)
lawkit benf audit_data.csv --confidence 0.99 --verbose

# Performance optimization for large datasets
lawkit benf big_data.csv --sample-size 50000 --optimize

# Filter out small values that add noise to analysis
lawkit benf financial_data.csv --min-value 100
```

### `lawkit pareto` - Pareto Principle Analysis

Analyze concentration and apply the 80/20 rule.

```bash
lawkit pareto [OPTIONS] [INPUT]
```

#### Specific Options
- `--concentration <THRESHOLD>` - Concentration threshold (0.0-1.0) (default: 0.8)
- `--gini-coefficient` - Calculate Gini coefficient for inequality measurement
- `--percentiles <PERCENTILES>` - Custom percentiles to calculate (e.g., 70,80,90)
- `--business-analysis` - Enable business analysis insights

#### Examples
```bash
# Basic pareto analysis
lawkit pareto sales.csv

# Custom threshold
lawkit pareto data.csv --concentration 0.9

# Business analysis with Gini coefficient
lawkit pareto customers.csv --business-analysis --gini-coefficient

# Custom percentiles
lawkit pareto revenue.csv --percentiles 70,80,90,95
```

### `lawkit zipf` - Zipf Law Analysis

Analyze frequency distributions and ranking patterns.

```bash
lawkit zipf [OPTIONS] [INPUT]
```

#### Examples
```bash
# Basic zipf analysis
lawkit zipf frequency_data.csv

# Verbose output
lawkit zipf rankings.csv --verbose

# JSON output format
lawkit zipf data.csv --format json
```

### `lawkit normal` - Normal Distribution Analysis

Test for normality and detect outliers.

```bash
lawkit normal [OPTIONS] [INPUT]
```

#### Examples
```bash
# Basic normality testing
lawkit normal data.csv

# Detailed output
lawkit normal measurements.csv --verbose

# JSON output format
lawkit normal quality_data.csv --format json
```

### `lawkit poisson` - Poisson Distribution Analysis

Analyze event occurrences and rare events.

```bash
lawkit poisson [OPTIONS] [INPUT]
```

#### Options
- `--format <FORMAT>` - Output format: text, json, csv, yaml, toml, xml (default: text)
- `--quiet, -q` - Minimal output (numbers only)
- `--verbose, -v` - Enable verbose debugging output with detailed analysis insights
- `--min-count <NUMBER>` - Minimum number of data points required for analysis (default: 10)
- `--optimize` - Enable memory and processing optimizations for large datasets
- `--confidence <LEVEL>` - Statistical confidence level for tests (0.01-0.99, default: 0.95)

#### Examples
```bash
# Basic Poisson analysis
lawkit poisson events.csv

# Detailed output
lawkit poisson incidents.csv --verbose

# JSON output format
lawkit poisson data.csv --format json

# High confidence level for critical analysis
lawkit poisson server_errors.csv --confidence 0.99 --verbose
```

## Generation Commands

### `lawkit generate` - Sample Data Generation

Generate sample data following specific statistical laws for testing and validation.

```bash
lawkit generate <LAW> [OPTIONS]
```

#### Available Laws
- `benf` - Generate Benford's law compliant data
- `pareto` - Generate Pareto distribution data
- `zipf` - Generate Zipf's law data
- `normal` - Generate normal distribution data
- `poisson` - Generate Poisson distribution data

#### Common Generation Options
- `--samples <NUMBER>` - Number of samples to generate (default: 1000)
- `--seed <NUMBER>` - Random seed for reproducible generation
- `--output-file <FILE>` - Output file path (default: stdout)

#### Law-Specific Options

**Benford Generation:**
- `--fraud-rate <RATE>` - Fraud injection rate (0.0-1.0) for testing (default: 0.0)
- `--range <MIN,MAX>` - Number range for generation (e.g., 1,10000) (default: 1,100000)

#### Examples
```bash
# Generate Benford's law data
lawkit generate benf --samples 5000

# Generate Benford data with fraud injection
lawkit generate benf --samples 2000 --fraud-rate 0.1

# Generate reproducible data with custom range
lawkit generate benf --samples 1000 --seed 42 --range 1,50000

# Generate and save to file
lawkit generate normal --samples 1000 --output-file test_data.csv
```

## Integration Commands

### `lawkit analyze` - Multi-Law Analysis

Perform basic multi-law analysis with recommendations for comprehensive data assessment.

```bash
lawkit analyze [OPTIONS] [INPUT]
```

### `lawkit validate` - Data Validation

Validate data consistency and quality across multiple statistical patterns.

```bash
lawkit validate [OPTIONS] [INPUT]
```

### `lawkit diagnose` - Conflict Detection

Detect conflicts and provide detailed diagnostics between statistical law results.

```bash
lawkit diagnose [OPTIONS] [INPUT]
```

#### Options
- `--laws <LAWS>` - Specific laws to analyze: benf,pareto,zipf,normal,poisson
- `--focus <FOCUS>` - Analysis focus: quality, concentration, distribution, anomaly
- `--purpose <PURPOSE>` - Analysis purpose: quality, fraud, concentration, anomaly, distribution, general
- `--recommend` - Enable optimal law recommendation mode
- `--threshold <THRESHOLD>` - Conflict detection threshold (0.0-1.0) (default: 0.5)
- `--report <TYPE>` - Integration report type: summary, detailed, conflicting (default: summary)
- `--consistency-check` - Enable consistency check
- `--cross-validation` - Enable cross-validation analysis
- `--confidence-level <LEVEL>` - Confidence level (default: 0.95)

#### Examples
```bash
# Compare all laws
lawkit analyze data.csv

# Focus on fraud detection
lawkit analyze transactions.csv --purpose fraud --recommend

# Custom law selection
lawkit analyze data.csv --laws benf,normal --focus quality

# Verbose output with JSON format
lawkit analyze dataset.csv --verbose --format json
```

## Common Options

All commands support these common options:

### Input/Output
- `[INPUT]` - Input data (file path, URL, or '-' for stdin)
- `--format <FORMAT>` - Output format: text, json, csv, yaml, toml, xml
- `--quiet, -q` - Minimal output
- `--verbose, -v` - Detailed output
- `--optimize` - Enable memory and processing optimizations for large datasets

### Data Processing
- `--filter <RANGE>` - Number filtering (>=100, <1000, 50-500)
- `--min-count <NUMBER>` - Minimum data points required (default: 10)

## Input Formats

`lawkit` supports multiple input formats:

- **Text files** - Numbers separated by whitespace/commas
- **CSV** - Comma-separated values
- **JSON** - Structured data
- **YAML** - YAML configuration files
- **TOML** - TOML configuration files
- **XML** - XML data files

## Output Formats

### Text Format (Default)
Human-readable output with analysis results, interpretations, and recommendations.

### JSON Format
Machine-readable structured output for APIs and automation:
```json
{
  "dataset": "data.csv",
  "numbers_analyzed": 1000,
  "risk_level": "Low",
  "analysis_results": {...}
}
```

### CSV Format
Tabular format for spreadsheet import:
```csv
dataset,numbers_analyzed,risk_level,score
data.csv,1000,Low,0.85
```

## Exit Codes

- `0` - Success, low risk
- `10` - Medium risk detected
- `11` - High risk detected
- `12` - Critical risk detected
- `1` - Analysis error
- `2` - Invalid arguments
- `3` - File/network error

## Examples by Use Case

### Fraud Detection
```bash
# Financial transaction analysis
lawkit benf transactions.csv --verbose

# Multi-law fraud detection
lawkit analyze suspicious_data.csv --purpose fraud --recommend
```

### Data Quality Assessment
```bash
# Comprehensive quality check
lawkit analyze dataset.csv --purpose quality --verbose

# Focus on normality
lawkit normal dataset.csv --verbose
```

### Business Intelligence
```bash
# 80/20 analysis
lawkit pareto sales.csv --threshold 0.8

# Customer analysis
lawkit zipf customer_frequency.csv --verbose
```

### Anomaly Detection
```bash
# Normality and outlier analysis
lawkit normal data.csv --verbose

# Event analysis
lawkit poisson incidents.csv --verbose
```