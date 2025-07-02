# benf

A CLI tool for detecting anomalies using Benford's Law with support for international numerals (Japanese, Chinese, Hindi, Arabic).

## Overview

`benf` analyzes numerical data to check if it follows Benford's Law, which states that in many naturally occurring datasets, the digit 1 appears as the **first (leading) digit** about 30.1% of the time, 2 appears 17.6% of the time, and so on. Deviations from this law can indicate data manipulation or fraud.

**Note**: This tool analyzes only the **first digit** of each number, not the entire number sequence.

**Unique Features:**
- 🌍 **International numeral support**: English, Japanese (全角・漢数字), Chinese (中文数字), Hindi (हिन्दी अंक), Arabic (الأرقام العربية)
- 📊 Multiple input formats (Microsoft Excel, Word, PowerPoint, PDF, etc.)
- 🌐 Direct URL analysis with HTML parsing
- 🔍 Fraud detection focus with risk level indicators

## International Numeral Support

### Supported Number Formats

#### 1. Full-width Digits
```bash
echo "１２３４５６ ７８９０１２" | benf
```

#### 2. Kanji Numerals (Basic)
```bash
echo "一二三四五六七八九" | benf
```

#### 3. Kanji Numerals (Positional)
```bash
echo "一千二百三十四 五千六百七十八 九万一千二百" | benf
```

#### 4. Mixed Patterns
```bash
echo "売上123万円 経費45万6千円 利益78万９千円" | benf
```

### Conversion Rules

| Kanji | Number | Notes |
|-------|--------|-------|
| 一 | 1 | Basic digit |
| 十 | 10 | Tens place |
| 百 | 100 | Hundreds place |
| 千 | 1000 | Thousands place |
| 万 | 10000 | Ten thousands place |
| 一千二百三十四 | 1234 | Positional notation |

#### Decimal Numbers
```bash
# Only numbers ≥ 1 are analyzed
echo "12.34 0.567 123.45" | benf
# Result: 1, (excluded), 1 (numbers < 1 are excluded)
```

#### Negative Numbers
```bash
# Uses absolute value's first digit
echo "-123 -456 -789" | benf
# Result: 1, 4, 7
```

### Chinese Numeral Compatibility

Current implementation supports basic Chinese numerals that are identical to Japanese kanji:

#### Supported (Basic Forms)
- 一二三四五六七八九 (1-9) - Same as Japanese
- 十百千 (10, 100, 1000) - Positional markers

#### Planned Support
- **Financial forms**: 壹貳參肆伍陸柒捌玖 (anti-fraud variants)
- **Traditional**: 萬 (10,000) vs Japanese 万
- **Regional variants**: Traditional vs Simplified Chinese

### Hindi Numerals (हिन्दी अंक)
```bash
# Devanagari numerals
echo "१२३४५६ ७८९०१२" | benf --lang hi
```

### Arabic Numerals (الأرقام العربية)
```bash  
# Eastern Arabic-Indic numerals
echo "١٢٣٤٥٦ ٧٨٩٠١٢" | benf --lang ar
```

### Other Numeral Systems (Future Support)

#### Additional Scripts (Planned)
- **Persian**: ۰۱۲۳۴۵۶۷۸۹ (Iran, Afghanistan)
- **Bengali**: ০১২৩৪৫৬৭৮৯ (Bangladesh)
- **Tamil**: ௦௧௨௩௪௫௬௭௮௯ (Tamil Nadu)
- **Thai**: ๐๑๒๓๔๕๖๗๘๙ (Thailand)
- **Myanmar**: ၀၁၂၃၄၅၆၇၈၉ (Myanmar)

> **Note**: International numeral support continues expanding based on user demand. Current priority: Japanese/Chinese/Hindi/Arabic financial document analysis.

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
| `--lang <LANGUAGE>` | Output language: en, ja, zh, hi, ar (default: auto) |
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
| OpenDocument | ods, .odt | OpenOffice/LibreOffice files |
| PDF | .pdf | Text extraction |
| CSV/TSV | .csv, .tsv | Structured data |
| JSON/XML | .json, .xml | API responses |
| YAML/TOML | .yaml, .toml | Configuration files |
| HTML | .html | Web pages |
| Text | .txt | Plain text |

## Real-World Usage Examples

Benf follows Unix philosophy and works excellently with standard Unix tools for processing multiple files:

### Financial Audit Workflows

```bash
# Quarterly financial audit - check all Excel reports
find ./Q4-2024 -name "*.xlsx" | while read file; do
    echo "Auditing: $file"
    benf "$file" --filter ">=1000" --threshold critical --verbose
    echo "---"
done

# Monthly expense report validation
for dept in accounting marketing sales; do
    echo "Department: $dept"
    find "./expenses/$dept" -name "*.xlsx" -exec benf {} --format json \; | \
    jq '.risk_level' | sort | uniq -c
done

# Tax document verification (high-precision analysis)
find ./tax-filings -name "*.pdf" | parallel benf {} --min-count 50 --format csv | \
awk -F, '$3=="Critical" {print "🚨 CRITICAL:", $1}'
```

### Automated Monitoring & Alerts

```bash
# Daily monitoring script for accounting system exports
#!/bin/bash
ALERT_EMAIL="audit@company.com"
find /exports/daily -name "*.csv" -newer /var/log/last-benf-check | while read file; do
    benf "$file" --format json | jq -r 'select(.risk_level=="Critical" or .risk_level=="High") | "\(.dataset): \(.risk_level)"'
done | mail -s "Daily Benford Alert" $ALERT_EMAIL

# Continuous integration fraud detection
find ./uploaded-reports -name "*.xlsx" -mtime -1 | \
xargs -I {} sh -c 'benf "$1" || echo "FRAUD ALERT: $1" >> /var/log/fraud-alerts.log' _ {}

# Real-time folder monitoring with inotify
inotifywait -m ./financial-uploads -e create --format '%f' | while read file; do
    if [[ "$file" =~ \.(xlsx|csv|pdf)$ ]]; then
        echo "$(date): Analyzing $file" >> /var/log/benf-monitor.log
        benf "./financial-uploads/$file" --threshold high || \
        echo "$(date): ALERT - Suspicious file: $file" >> /var/log/fraud-alerts.log
    fi
done
```

### Large-Scale Data Processing

```bash
# Process entire corporate filesystem for compliance audit
find /corporate-data -type f \( -name "*.xlsx" -o -name "*.csv" -o -name "*.pdf" \) | \
parallel -j 16 'echo "{}: $(benf {} --format json 2>/dev/null | jq -r .risk_level // "ERROR")"' | \
tee compliance-audit-$(date +%Y%m%d).log

# Archive analysis - process historical data efficiently
find ./archives/2020-2024 -name "*.xlsx" -print0 | \
xargs -0 -n 100 -P 8 -I {} benf {} --filter ">=10000" --format csv | \
awk -F, 'BEGIN{OFS=","} NR>1 && $3~/High|Critical/ {sum++} END{print "High-risk files:", sum}'

# Network storage scanning with progress tracking
total_files=$(find /nfs/financial-data -name "*.xlsx" | wc -l)
find /nfs/financial-data -name "*.xlsx" | nl | while read num file; do
    echo "[$num/$total_files] Processing: $(basename "$file")"
    benf "$file" --format json | jq -r '"File: \(.dataset), Risk: \(.risk_level), Numbers: \(.numbers_analyzed)"'
done | tee network-scan-report.txt
```

### Advanced Reporting & Analytics

```bash
# Risk distribution analysis across departments
for dept in */; do
    echo "=== Department: $dept ==="
    find "$dept" -name "*.xlsx" | xargs -I {} benf {} --format json 2>/dev/null | \
    jq -r '.risk_level' | sort | uniq -c | awk '{print $2": "$1" files"}'
    echo
done

# Time-series risk analysis (requires date-sorted files)
find ./monthly-reports -name "202[0-4]-*.xlsx" | sort | while read file; do
    month=$(basename "$file" .xlsx)
    risk=$(benf "$file" --format json 2>/dev/null | jq -r '.risk_level // "N/A"')
    echo "$month,$risk"
done > risk-timeline.csv

# Statistical summary generation
{
    echo "file,risk_level,numbers_count,chi_square,p_value"
    find ./audit-sample -name "*.xlsx" | while read file; do
        benf "$file" --format json 2>/dev/null | \
        jq -r '"\(.dataset),\(.risk_level),\(.numbers_analyzed),\(.statistics.chi_square),\(.statistics.p_value)"'
    done
} > statistical-analysis.csv

# Comparative analysis between periods
echo "Comparing Q3 vs Q4 risk levels..."
q3_high=$(find ./Q3-2024 -name "*.xlsx" | xargs -I {} benf {} --format json 2>/dev/null | jq -r 'select(.risk_level=="High" or .risk_level=="Critical")' | wc -l)
q4_high=$(find ./Q4-2024 -name "*.xlsx" | xargs -I {} benf {} --format json 2>/dev/null | jq -r 'select(.risk_level=="High" or .risk_level=="Critical")' | wc -l)
echo "Q3 high-risk files: $q3_high"
echo "Q4 high-risk files: $q4_high"
echo "Change: $((q4_high - q3_high))"
```

### Integration with Other Tools

```bash
# Git pre-commit hook for data validation
#!/bin/bash
# .git/hooks/pre-commit
changed_files=$(git diff --cached --name-only --diff-filter=A | grep -E '\.(xlsx|csv|pdf)$')
for file in $changed_files; do
    if ! benf "$file" --min-count 10 >/dev/null 2>&1; then
        echo "⚠️  Warning: $file may contain suspicious data patterns"
        benf "$file" --format json | jq '.risk_level'
    fi
done

# Database import validation
psql -c "COPY suspicious_files FROM STDIN CSV HEADER" <<< $(
    echo "filename,risk_level,chi_square,p_value"
    find ./import-data -name "*.csv" | while read file; do
        benf "$file" --format json 2>/dev/null | \
        jq -r '"\(.dataset),\(.risk_level),\(.statistics.chi_square),\(.statistics.p_value)"'
    done
)

# Slack/Teams webhook integration
high_risk_files=$(find ./daily-uploads -name "*.xlsx" -mtime -1 | \
    xargs -I {} benf {} --format json 2>/dev/null | \
    jq -r 'select(.risk_level=="High" or .risk_level=="Critical") | .dataset')

if [ -n "$high_risk_files" ]; then
    curl -X POST -H 'Content-type: application/json' \
    --data "{\"text\":\"🚨 High-risk files detected:\n$high_risk_files\"}" \
    $SLACK_WEBHOOK_URL
fi

# Excel macro integration (save as macro-enabled workbook)
# VBA code to call benf from Excel:
# Shell "benf """ & ActiveWorkbook.FullName & """ --format json > benf-result.json"
```

### Specialized Use Cases

```bash
# Election audit (checking vote counts)
find ./election-data -name "*.csv" | parallel benf {} --min-count 100 --threshold low | \
grep -E "(HIGH|CRITICAL)" > election-anomalies.txt

# Scientific data validation
find ./research-data -name "*.xlsx" | while read file; do
    lab=$(dirname "$file" | xargs basename)
    result=$(benf "$file" --format json | jq -r '.risk_level')
    echo "$lab,$file,$result"
done | grep -E "(High|Critical)" > data-integrity-issues.csv

# Supply chain invoice verification
find ./invoices/2024 -name "*.pdf" | parallel 'vendor=$(dirname {} | xargs basename); benf {} --format json | jq --arg v "$vendor" '"'"'{vendor: $v, file: .dataset, risk: .risk_level}'"'"' > invoice-analysis.jsonl

# Insurance claim analysis  
find ./claims -name "*.xlsx" | while read file; do
    claim_id=$(basename "$file" .xlsx)
    benf "$file" --filter ">=1000" --format json | \
    jq --arg id "$claim_id" '{claim_id: $id, risk_assessment: .risk_level, total_numbers: .numbers_analyzed}'
done | jq -s '.' > claims-risk-assessment.json
```

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

## ⚠️ Important Limitations

**Benford's Law does NOT apply to:**
- **Constrained ranges**: Adult heights (1.5-2.0m), ages (0-100), temperatures
- **Sequential data**: Invoice numbers, employee IDs, zip codes
- **Assigned numbers**: Phone numbers, social security numbers, lottery numbers
- **Small datasets**: Less than 30-50 numbers (insufficient for statistical analysis)
- **Single-source data**: All from same process/machine with similar magnitudes
- **Rounded data**: Heavily rounded amounts (e.g., all ending in 00)

**Best suited for:**
- **Multi-scale natural data**: Financial transactions, populations, physical measurements
- **Diverse sources**: Mixed data from different processes/timeframes
- **Large datasets**: 100+ numbers for reliable analysis
- **Unmanipulated data**: Naturally occurring, not artificially constrained

## Historical Background

**Discovery and Development:**
- **1881**: Simon Newcomb first observed the phenomenon while studying logarithm tables
- **1938**: Physicist Frank Benford rediscovered and formalized the law with extensive research
- **1972**: First application to accounting and fraud detection in academic literature
- **1980s**: Major accounting firms began using Benford's Law as a standard audit tool
- **1990s**: Mark Nigrini popularized its use in forensic accounting and tax fraud detection
- **2000s+**: Expanded to election monitoring, scientific data validation, and financial crime investigation

**Modern Applications:**
- Used by IRS for tax audit screening
- Standard tool in Big Four accounting firms
- Applied in election fraud detection (notably 2009 Iran election analysis)
- Employed in anti-money laundering investigations

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