# Configuration Guide

## Overview

`lawkit` is designed to work out-of-the-box with sensible defaults, but offers several configuration options for different use cases.

## Command-Line Options

### Global Options

```bash
# Output format
lawkit benf data.csv --format json
lawkit benf data.csv --format yaml
lawkit benf data.csv --format csv
lawkit benf data.csv --format toml
lawkit benf data.csv --format xml

# International number support (automatic recognition)
echo "１２３４５６" | lawkit benf      # Japanese numbers
echo "一千二百三十四" | lawkit benf    # Chinese numbers

# Verbosity
lawkit benf data.csv --quiet     # Minimal output
lawkit benf data.csv --verbose   # Detailed output
```

### Analysis Options

```bash
# Pareto analysis with threshold
lawkit pareto data.csv --threshold 0.8

# Multi-law analysis
lawkit analyze data.csv --laws benford,pareto,normal

# Analysis with focus
lawkit analyze data.csv --laws benford --focus accuracy

# Purpose-specific analysis
lawkit analyze data.csv --laws all --purpose audit

# Recommendations
lawkit analyze data.csv --laws all --recommend
```

## Output Formats

### Supported Formats

| Format | Description | Best For |
|--------|-------------|----------|
| `text` | Human-readable (default) | Terminal display |
| `json` | Machine-readable | APIs, automation |
| `csv` | Tabular data | Spreadsheets |
| `yaml` | Structured config | Config files |
| `toml` | Rust-friendly | Rust integration |
| `xml` | Legacy systems | Enterprise |

### Format Examples

#### JSON Output
```bash
lawkit benf data.csv --format json
```
```json
{
  "dataset": "data.csv",
  "numbers_analyzed": 1000,
  "risk_level": "Low",
  "mean_absolute_deviation": 2.3,
  "chi_square_p_value": 0.85
}
```

#### CSV Output
```bash
lawkit benf data.csv --format csv
```
```csv
dataset,numbers_analyzed,risk_level,mad,chi_square_p
data.csv,1000,Low,2.3,0.85
```

## Multi-Language Support

### Supported Languages

- **English** (`en`) - Default
- **Japanese** (`ja`) - 日本語
- **Chinese** (`zh`) - 中文
- **Hindi** (`hi`) - हिन्दी
- **Arabic** (`ar`) - العربية

### International Number Support

`lawkit` automatically recognizes various number formats:

```bash
# Japanese numbers
echo "１２３４ ５６７８" | lawkit benf

# Chinese financial numbers  
echo "壹万贰千 三千四百" | lawkit benf

# Mixed formats
echo "123 ４５６ 七八九" | lawkit benf
```

## Integration Analysis

### Multi-Law Analysis Configuration

```bash
# Select specific laws
lawkit analyze data.csv --laws benford,pareto,normal

# Focus on specific analysis type
lawkit analyze data.csv --laws benford --focus accuracy

# Purpose-specific analysis
lawkit analyze data.csv --laws all --purpose audit

# Recommendation mode
lawkit analyze data.csv --laws all --recommend

# Validation mode
lawkit validate data.csv --laws all

# Diagnosis mode
lawkit diagnose data.csv --laws all
```

### Analysis Purposes

| Purpose | Best Laws | Use Case |
|---------|-----------|----------|
| `audit` | Benford + Normal | Data quality audit |
| `fraud` | Benford + Poisson | Fraud detection |
| `business` | Pareto + Zipf | Business analysis |
| `research` | All laws | General analysis |

## Batch Processing

```bash
# Process multiple files
for file in *.csv; do
  lawkit benf "$file" --format json > "results_${file%.csv}.json"
done

# Analyze with different laws
lawkit analyze data1.csv --laws benford --format json
lawkit analyze data2.csv --laws pareto --format json
lawkit analyze data3.csv --laws normal --format json
```

## Performance Tuning

### Large Datasets

```bash
# Use quiet mode for better performance
lawkit benf large_data.csv --quiet

# Focus on specific analysis
lawkit analyze large_data.csv --laws benford --quiet
```

### Memory Management

- Files > 1GB: Consider data preprocessing
- Use `--quiet` for minimal memory usage
- Stream processing with stdin input

## Troubleshooting

### Common Issues

1. **"Insufficient data"** - Provide more data or check file format
2. **"No numbers found"** - Check data format and encoding
3. **"Format error"** - Verify file format matches content

### Debug Mode

```bash
# Enable verbose logging
lawkit benf data.csv --verbose

# Check data parsing
lawkit benf data.csv --format json | jq '.numbers_analyzed'
```

## Future Configuration Features

The following features are planned for future versions:

- Configuration file support (`lawkit.toml`)
- Environment variable settings
- Custom threshold configuration
- Profile-based settings
- Data filtering options
- Advanced analysis options

## Next Steps

- [Examples](examples.md) - Real-world configuration examples
- [CLI Reference](../reference/cli-reference.md) - Complete command documentation
- [Integration Guide](../guides/integrations.md) - CI/CD automation