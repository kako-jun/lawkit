# Configuration Guide

## Overview

`lawkit` is designed to work out-of-the-box with sensible defaults, but offers extensive configuration options for advanced users.

## Command-Line Options

### Global Options

```bash
# Output format
lawkit benf data.csv --format json
lawkit benf data.csv --format yaml

# International number support (automatic recognition)
echo "１２３４５６" | lawkit benf      # Japanese numbers
echo "一千二百三十四" | lawkit benf    # Chinese numbers

# Verbosity
lawkit benf data.csv --quiet     # Minimal output
lawkit benf data.csv --verbose   # Detailed output
```

### Data Filtering

```bash
# Number range filtering
lawkit benf data.csv --filter ">=100"
lawkit benf data.csv --filter "50-1000"

# Minimum data requirements
lawkit benf data.csv --min-count 50
```

### Analysis Options

```bash
# Custom thresholds
lawkit benf data.csv --threshold high
lawkit normal data.csv --confidence-level 0.99

# Specific analysis modes
lawkit pareto data.csv --business-analysis
lawkit normal data.csv --outliers
lawkit poisson data.csv --predict
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
lawkit analyze data.csv --laws benf,pareto,normal

# Focus on specific analysis
lawkit analyze data.csv --focus quality
lawkit analyze data.csv --focus anomaly

# Conflict detection
lawkit diagnose data.csv --threshold 0.7 --report conflicting

# Recommendation mode
lawkit analyze data.csv --recommend --purpose fraud
```

### Analysis Purposes

| Purpose | Best Laws | Use Case |
|---------|-----------|----------|
| `quality` | Benford + Normal | Data quality audit |
| `fraud` | Benford + Poisson | Fraud detection |
| `concentration` | Pareto + Zipf | Business analysis |
| `anomaly` | Normal + Poisson | Outlier detection |
| `distribution` | All laws | General analysis |

## Environment Variables

```bash
# Default output format
export LAWKIT_FORMAT=json

# Disable colors
export NO_COLOR=1
```

## Advanced Configuration

### Custom Thresholds

```bash
# Benford Law
lawkit benf data.csv --threshold low     # MAD < 4
lawkit benf data.csv --threshold medium  # MAD < 8  
lawkit benf data.csv --threshold high    # MAD < 15

# Normal Distribution
lawkit normal data.csv --confidence-level 0.95  # 95% confidence
lawkit normal data.csv --confidence-level 0.99  # 99% confidence
```

### Batch Processing

```bash
# Process multiple files
for file in *.csv; do
  lawkit benf "$file" --format json > "results_${file%.csv}.json"
done

# Analyze results
lawkit analyze data1.csv data2.csv data3.csv --report summary
```

## Performance Tuning

### Large Datasets

```bash
# Minimum data filtering for performance
lawkit benf large_data.csv --min-count 1000

# Focus on specific analysis
lawkit analyze large_data.csv --focus quality --laws benf,normal
```

### Memory Management

- Files > 1GB: Consider data sampling
- Use `--quiet` for minimal memory usage
- Stream processing with `--` stdin input

## Troubleshooting

### Common Issues

1. **"Insufficient data"** - Increase `--min-count` or provide more data
2. **"No numbers found"** - Check data format and encoding
3. **"Format error"** - Verify file format matches content

### Debug Mode

```bash
# Enable verbose logging
lawkit benf data.csv --verbose

# Check data parsing
lawkit benf data.csv --format json | jq '.numbers_analyzed'
```

## Next Steps

- [Examples](examples.md) - Real-world configuration examples
- [CLI Reference](../reference/cli-reference.md) - Complete command documentation
- [Integration Guide](../guides/integrations.md) - CI/CD automation