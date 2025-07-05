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
  benf    - Benford's law analysis
  pareto  - Pareto principle (80/20 rule) analysis
  zipf    - Zipf's law analysis
  normal  - Normal distribution analysis
  poisson - Poisson distribution analysis

Integration commands:
  compare - Compare and integrate multiple statistical laws
```

## Statistical Law Commands

### `lawkit benf` - Benford's Law Analysis

Detect anomalies and assess data quality using Benford's Law.

```bash
lawkit benf [OPTIONS] [INPUT]
```

#### Options
- `--format <FORMAT>` - Output format: text, json, csv, yaml, toml, xml (default: text)
- `--quiet, -q` - Minimal output (numbers only)
- `--verbose, -v` - Detailed statistics
- `--language <LANGUAGE>` - Output language: en, ja, zh, hi, ar (default: auto)
- `--filter <RANGE>` - Filter numbers by range (e.g., >=100, <1000, 50-500)
- `--threshold <LEVEL>` - Custom anomaly detection threshold: low, medium, high, critical (default: auto)
- `--min-count <NUMBER>` - Minimum number of data points required for analysis (default: 5)

#### Examples
```bash
# Basic analysis
lawkit benf data.csv

# Fraud detection mode
lawkit benf transactions.json --threshold high --verbose

# Filter and format output
lawkit benf data.csv --filter ">=1000" --format json
```

### `lawkit pareto` - Pareto Principle Analysis

Analyze concentration and apply the 80/20 rule.

```bash
lawkit pareto [OPTIONS] [INPUT]
```

#### Specific Options
- `--business-analysis` - Enable business-specific analysis
- `--gini-coefficient` - Calculate Gini coefficient
- `--percentiles <LIST>` - Custom percentiles (default: 80,90,95)

#### Examples
```bash
# Business analysis
lawkit pareto sales.csv --business-analysis

# Custom percentiles
lawkit pareto data.csv --percentiles "70,80,90"
```

### `lawkit zipf` - Zipf's Law Analysis

Analyze frequency distributions and ranking patterns.

```bash
lawkit zipf [OPTIONS] [INPUT]
```

#### Specific Options
- `--text-analysis` - Enable text/word frequency analysis
- `--ranking` - Analyze ranking distributions
- `--correlation-method <METHOD>` - Correlation calculation method

#### Examples
```bash
# Text frequency analysis
lawkit zipf document.txt --text-analysis

# Numeric ranking
lawkit zipf rankings.csv --ranking
```

### `lawkit normal` - Normal Distribution Analysis

Test for normality and detect outliers.

```bash
lawkit normal [OPTIONS] [INPUT]
```

#### Specific Options
- `--test <TYPE>` - Normality test: shapiro, anderson, ks, all (default: all)
- `--outliers` - Enable outlier detection
- `--outlier-method <METHOD>` - Detection method: zscore, modified_zscore, iqr, lof, isolation, dbscan, ensemble (default: zscore)
- `--quality-control` - Enable quality control analysis
- `--spec-limits <LIMITS>` - Specification limits: "lower,upper"
- `--confidence-level <LEVEL>` - Confidence level (default: 0.95)
- `--enable-timeseries` - Enable time series analysis
- `--timeseries-window <SIZE>` - Time series analysis window size (default: 10)

#### Examples
```bash
# Normality testing
lawkit normal data.csv --test shapiro

# Advanced outlier detection
lawkit normal data.csv --outliers --outlier-method ensemble

# Time series analysis
lawkit normal timeseries.csv --enable-timeseries --timeseries-window 20

# Quality control
lawkit normal measurements.csv --quality-control --spec-limits "10,20"
```

### `lawkit poisson` - Poisson Distribution Analysis

Analyze event occurrences and rare events.

```bash
lawkit poisson [OPTIONS] [INPUT]
```

#### Specific Options
- `--test <TYPE>` - Poisson test: chi-square, ks, variance, all (default: all)
- `--predict` - Enable event probability prediction
- `--max-events <COUNT>` - Maximum events for prediction (default: 10)
- `--rare-events` - Enable rare event analysis
- `--confidence-level <LEVEL>` - Confidence level (default: 0.95)

#### Examples
```bash
# Basic Poisson analysis
lawkit poisson events.csv

# Prediction mode
lawkit poisson data.csv --predict --max-events 15

# Rare event analysis
lawkit poisson incidents.csv --rare-events
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
- `--fraud-rate <RATE>` - Fraud injection rate (0.0-1.0) for testing

#### Law-Specific Options

**Benford Generation:**
- `--range <MIN,MAX>` - Number range (default: 1,100000)

**Pareto Generation:**
- `--concentration <RATIO>` - Concentration ratio (default: 0.8)
- `--scale <NUMBER>` - Scale parameter (default: 1.0)

**Zipf Generation:**
- `--exponent <NUMBER>` - Zipf exponent (default: 1.0)
- `--vocabulary-size <NUMBER>` - Vocabulary size (default: 10000)

**Normal Generation:**
- `--mean <NUMBER>` - Mean (default: 0.0)
- `--stddev <NUMBER>` - Standard deviation (default: 1.0)

**Poisson Generation:**
- `--lambda <NUMBER>` - Lambda parameter (default: 2.0)
- `--time-series` - Generate time-series event data

#### Examples
```bash
# Generate Benford's law data
lawkit generate benf --samples 5000 --range 1,1000000

# Generate Pareto data with 20% fraud
lawkit generate pareto --samples 2000 --fraud-rate 0.2

# Generate reproducible normal data
lawkit generate normal --samples 1000 --seed 42 --mean 100 --stddev 15
```

## Integration Commands

### `lawkit compare` - Multi-Law Analysis

Compare and integrate multiple statistical laws for comprehensive analysis.

```bash
lawkit compare [OPTIONS] [INPUT]
```

#### Options
- `--laws <LAWS>` - Specific laws to compare: benf,pareto,zipf,normal,poisson
- `--focus <FOCUS>` - Analysis focus: quality, concentration, distribution, anomaly
- `--threshold <THRESHOLD>` - Conflict detection threshold: 0.0-1.0 (default: 0.5)
- `--recommend` - Enable optimal law recommendation mode
- `--report <TYPE>` - Report type: summary, detailed, conflicting (default: summary)
- `--consistency-check` - Run consistency check
- `--cross-validation` - Enable cross-validation analysis
- `--confidence-level <LEVEL>` - Confidence level (default: 0.95)
- `--purpose <PURPOSE>` - Analysis purpose: quality, fraud, concentration, anomaly, distribution, general

#### Examples
```bash
# Compare all laws
lawkit compare data.csv

# Focus on fraud detection
lawkit compare transactions.csv --purpose fraud --recommend

# Conflict analysis
lawkit compare data.csv --report conflicting --threshold 0.7

# Custom law selection
lawkit compare data.csv --laws benf,normal --focus quality
```

## Common Options

All commands support these common options:

### Input/Output
- `[INPUT]` - Input data (file path, URL, or '-' for stdin)
- `--format <FORMAT>` - Output format: text, json, csv, yaml, toml, xml
- `--quiet, -q` - Minimal output
- `--verbose, -v` - Detailed output
- `--language <LANG>` - Output language: en, ja, zh, hi, ar, auto
- `--parallel` - Enable parallel processing
- `--threads <NUMBER>` - Number of threads for parallel processing (0 = auto)
- `--chunk-size <SIZE>` - Chunk size for memory-efficient processing (default: 10000)
- `--streaming` - Enable streaming mode for large datasets

### Data Processing
- `--filter <RANGE>` - Number filtering (>=100, <1000, 50-500)
- `--min-count <NUMBER>` - Minimum data points required

## Input Formats

`lawkit` supports multiple input formats:

- **Text files** - Numbers separated by whitespace/commas
- **CSV** - Comma-separated values
- **JSON** - Structured data
- **YAML** - YAML configuration files
- **TOML** - TOML configuration files
- **XML** - XML data files
- **Excel** - .xlsx spreadsheet files
- **OpenDocument** - .ods/.odt files
- **PDF** - Text extraction from PDF
- **Word** - .docx document files
- **PowerPoint** - .pptx presentation files

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
lawkit benf transactions.csv --purpose fraud --threshold high

# Multi-law fraud detection
lawkit compare suspicious_data.csv --purpose fraud --recommend
```

### Data Quality Assessment
```bash
# Comprehensive quality check
lawkit compare dataset.csv --purpose quality --report detailed

# Focus on normality
lawkit normal dataset.csv --test all --confidence-level 0.99
```

### Business Intelligence
```bash
# 80/20 analysis
lawkit pareto sales.csv --business-analysis --percentiles "80,90,95"

# Customer analysis
lawkit zipf customer_frequency.csv --ranking
```

### Anomaly Detection
```bash
# Multi-method outlier detection
lawkit normal data.csv --outliers --outlier-method ensemble

# LOF outlier detection for complex patterns
lawkit normal data.csv --outliers --outlier-method lof

# Event anomaly analysis
lawkit poisson incidents.csv --rare-events
```