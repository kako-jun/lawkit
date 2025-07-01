# benf

A CLI tool for detecting anomalies using Benford's Law with support for Japanese numerals.

## Overview

`benf` analyzes numerical data to check if it follows Benford's Law, which states that in many naturally occurring datasets, the digit 1 appears as the leading digit about 30.1% of the time, 2 appears 17.6% of the time, and so on. Deviations from this law can indicate data manipulation or fraud.

**Unique Features:**
- 🇯🇵 Japanese numeral support (full-width digits: ０１２, kanji numerals: 一二三)
- 📊 Multiple input formats (Excel, PDF, Word, HTML, etc.)
- 🌐 Direct URL analysis with HTML parsing
- 🔍 Fraud detection focus with risk level indicators

## Installation

### From Source
```bash
git clone https://github.com/kako-jun/benf
cd benf
cargo build --release
cp target/release/benf /usr/local/bin/
```

### Binary Releases
Download from [releases page](https://github.com/kako-jun/benf/releases)

## Quick Start

```bash
# Analyze CSV file
benf data.csv

# Analyze website data
benf --url https://example.com/financial-report

# Pipe data
echo "123 456 789 101112" | benf

# JSON output for automation
benf data.csv --format json
```

## Usage

### Basic Syntax
```bash
benf [OPTIONS] [INPUT]
```

### Input Methods
1. **File path**: `benf financial_data.xlsx`
2. **URL**: `benf --url https://api.example.com/data`
3. **String**: `benf "123 456 789 101112"`
4. **Pipe**: `cat data.txt | benf`

Priority: URL > File > String > Pipe

### Options

| Option | Description |
|--------|-------------|
| `--url <URL>` | Fetch data from URL |
| `--format <FORMAT>` | Output format: text, csv, json, yaml, toml, xml |
| `--quiet` | Minimal output (numbers only) |
| `--verbose` | Detailed statistics |
| `--filter <RANGE>` | Filter numbers (e.g., `--filter ">=100"`) |
| `--threshold <LEVEL>` | Alert threshold: low, medium, high, critical |
| `--proxy <URL>` | HTTP proxy server |
| `--insecure` | Skip SSL certificate verification |
| `--timeout <SECONDS>` | Request timeout (default: 30) |
| `--log-level <LEVEL>` | Log level: debug, info, warn, error |
| `--help, -h` | Show help |
| `--version, -V` | Show version |

### Supported File Formats

| Format | Extensions | Notes |
|--------|------------|-------|
| Microsoft Excel | .xlsx, .xls | Spreadsheet data |
| Microsoft Word | .docx, .doc | Document analysis |
| Microsoft PowerPoint | .pptx, .ppt | Presentation data |
| OpenDocument | .ods, .odt | OpenOffice/LibreOffice files |
| PDF | .pdf | Text extraction |
| CSV/TSV | .csv, .tsv | Structured data |
| JSON/XML | .json, .xml | API responses |
| YAML/TOML | .yaml, .toml | Configuration files |
| HTML | .html | Web pages |
| Text | .txt | Plain text |

## Output

### Default Output
```
Benford's Law Analysis Results

Dataset: financial_data.csv
Numbers analyzed: 1,247
Risk Level: HIGH ⚠️

First Digit Distribution:
1: ████████████████████████████ 28.3% (expected: 30.1%)
2: ████████████████████ 20.1% (expected: 17.6%)
3: ██████████ 9.8% (expected: 12.5%)
...

Statistical Tests:
Chi-square: 23.45 (p-value: 0.003)
Mean Absolute Deviation: 2.1%

Verdict: SIGNIFICANT DEVIATION DETECTED
```

### JSON Output
```json
{
  "dataset": "financial_data.csv",
  "numbers_analyzed": 1247,
  "risk_level": "HIGH",
  "digits": {
    "1": {"observed": 28.3, "expected": 30.1, "deviation": -1.8},
    "2": {"observed": 20.1, "expected": 17.6, "deviation": 2.5}
  },
  "statistics": {
    "chi_square": 23.45,
    "p_value": 0.003,
    "mad": 2.1
  },
  "verdict": "SIGNIFICANT_DEVIATION"
}
```

## Examples

### Fraud Detection
```bash
# Monitor sales data
benf sales_report.xlsx --threshold high

# Real-time log monitoring
tail -f transactions.log | benf --format json | jq 'select(.risk_level == "HIGH")'

# Batch analysis
find . -name "*.csv" -exec benf {} \; | grep "HIGH"
```

### Japanese Numerals
```bash
# Full-width digits
echo "１２３ ４５６ ７８９" | benf

# Kanji numerals
echo "一千二百三十四 五千六百七十八" | benf

# Mixed patterns
benf japanese_financial_report.pdf
```

### Web Analysis
```bash
# Financial website
benf --url https://company.com/earnings --format json

# API endpoint
benf --url https://api.finance.com/data --proxy http://proxy:8080
```

### Automation
```bash
# Daily fraud check
#!/bin/bash
RESULT=$(benf daily_sales.csv --format json)
RISK=$(echo $RESULT | jq -r '.risk_level')
if [ "$RISK" = "HIGH" ]; then
    echo $RESULT | mail -s "Fraud Alert" admin@company.com
fi
```

## Risk Levels

| Level | Chi-square p-value | Interpretation |
|-------|-------------------|----------------|
| LOW | p > 0.1 | Normal distribution |
| MEDIUM | 0.05 < p ≤ 0.1 | Slight deviation |
| HIGH | 0.01 < p ≤ 0.05 | Significant deviation |
| CRITICAL | p ≤ 0.01 | Strong evidence of manipulation |

## Common Use Cases

- **Accounting audits**: Detect manipulated financial records
- **Tax investigations**: Identify suspicious income declarations
- **Election monitoring**: Verify vote count authenticity
- **Insurance claims**: Spot fraudulent claim patterns
- **Scientific data**: Validate research results
- **Quality control**: Monitor manufacturing data

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | File/network error |
| 10 | High risk detected |
| 11 | Critical risk detected |

## Configuration

Benf respects standard environment variables:
- `HTTP_PROXY` / `HTTPS_PROXY`: Proxy settings
- `NO_PROXY`: Proxy bypass list

Logs are written to:
- Linux: `~/.local/state/benf/`
- macOS: `~/Library/Logs/benf/`
- Windows: `%APPDATA%\benf\Logs\`

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

## License

MIT License - see [LICENSE](LICENSE) file.

## References

- [Benford's Law - Wikipedia](https://en.wikipedia.org/wiki/Benford%27s_law)
- [Using Benford's Law for Fraud Detection](https://example.com/benford-fraud)