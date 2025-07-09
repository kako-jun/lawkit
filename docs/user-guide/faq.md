# Frequently Asked Questions

## General Questions

### What is lawkit?

lawkit is a comprehensive statistical analysis toolkit that implements multiple statistical laws including Benford's Law, Pareto Principle, Zipf's Law, Normal Distribution, and Poisson Distribution. It's designed for fraud detection, data quality assessment, business analytics, and scientific research.

### What makes lawkit different from other statistical tools?

- **Multi-law Integration**: Compare multiple statistical laws in a single analysis
- **International Support**: Handles numbers in multiple languages and formats
- **Built-in Risk Assessment**: Automatic anomaly detection and risk evaluation
- **Comprehensive Input Support**: Reads from CSV, Excel, PDF, Word, JSON, and more
- **Professional Output**: Multiple output formats including JSON, CSV, YAML, XML

### Is lawkit free to use?

Yes, lawkit is open-source software released under the MIT License. You can use it freely for both personal and commercial purposes.

## Installation and Setup

### How do I install lawkit?

**Option 1: Download Binary**
Download pre-built binaries from [GitHub Releases](https://github.com/kako-jun/lawkit/releases).

**Option 2: Build from Source**
```bash
git clone https://github.com/kako-jun/lawkit.git
cd lawkit
cargo build --release
```

**Option 3: As a Library**
Add to your `Cargo.toml`:
```toml
[dependencies]
lawkit-core = "2.0"
```

### What are the system requirements?

- **Operating Systems**: Linux, macOS, Windows
- **Memory**: Minimum 512MB RAM (more for large datasets)
- **Disk Space**: 50MB for installation
- **Dependencies**: None (statically linked binaries)

### Why am I getting "command not found" error?

Ensure the lawkit binary is in your system's PATH. After downloading:

```bash
# Make executable (Unix/Linux/macOS)
chmod +x lawkit

# Move to PATH (example)
sudo mv lawkit /usr/local/bin/

# Or add to PATH in your shell profile
export PATH="/path/to/lawkit:$PATH"
```

## Usage Questions

### What file formats does lawkit support?

**Input Formats:**
- Spreadsheets: CSV, TSV, Excel (.xlsx, .xls), OpenDocument (.ods)
- Documents: PDF, Word (.docx), PowerPoint (.pptx), plain text
- Data: JSON, YAML, TOML, XML
- Web: HTML (table extraction)

**Output Formats:**
- text, json, csv, yaml, toml, xml

### How do I analyze data from a specific column in a CSV file?

lawkit automatically extracts numerical data from all columns. For specific columns, pre-process your data:

```bash
# Extract specific column using standard tools
cut -d',' -f2 data.csv | lawkit benf

# Or use awk for more complex extraction
awk -F',' '{print $2}' data.csv | lawkit pareto
```

### How many data points do I need for reliable analysis?

Minimum requirements vary by statistical law:
- **Benford's Law**: 5+ points (recommended: 100+)
- **Pareto Analysis**: 5+ points (recommended: 20+)
- **Zipf's Law**: 5+ points (recommended: 50+)
- **Normal Distribution**: 8+ points (recommended: 30+)
- **Poisson Distribution**: 10+ points (recommended: 50+)

You can adjust minimums with `--min-count`:
```bash
lawkit benf --min-count 100 data.csv
```

### What does the risk assessment mean?

lawkit categorizes results into risk levels:
- **Low**: Data follows expected statistical patterns
- **Medium**: Some deviation, worth investigating
- **High**: Significant deviation, likely issues
- **Critical**: Extreme deviation, immediate attention needed

Customize thresholds with:
```bash
lawkit benf --threshold high data.csv
```

## Statistical Analysis Questions

### When should I use Benford's Law?

Benford's Law is ideal for:
- **Financial fraud detection**: Transaction amounts, accounting data
- **Data quality assessment**: Naturally occurring numerical data
- **Scientific validation**: Experimental measurements
- **Election auditing**: Vote counts and demographics

**Not suitable for:**
- Assigned numbers (phone numbers, IDs)
- Constrained ranges (percentages, ratings)
- Uniform distributions

### What's the difference between Pareto analysis and Zipf's law?

**Pareto Analysis (80/20 Rule):**
- Focuses on business applications
- Calculates Gini coefficient for inequality
- Provides business insights and recommendations
- Best for: sales analysis, customer segmentation, resource allocation

**Zipf's Law (Power-law Distribution):**
- Focuses on frequency distributions
- Analyzes rank-frequency relationships
- Supports text analysis and word frequencies
- Best for: linguistic analysis, city populations, website traffic

### How accurate is the normal distribution testing?

lawkit implements multiple normality tests:
- **Shapiro-Wilk**: Most powerful for small samples (n < 50)
- **Anderson-Darling**: Good for moderate samples
- **Kolmogorov-Smirnov**: General purpose, less powerful

Use `--test all` to run all tests:
```bash
lawkit normal --test all data.csv
```

### What does Poisson distribution analysis tell me?

Poisson analysis is useful for:
- **Event counting**: Defects per unit, calls per hour
- **Rare events**: Accidents, equipment failures
- **Quality control**: Process monitoring
- **Capacity planning**: Server load, customer arrivals

The analysis provides:
- Lambda parameter (average rate)
- Goodness-of-fit testing
- Event probability predictions
- Rare event assessment

## International and Language Support

### How does international number support work?

lawkit automatically recognizes various number formats:

```bash
# These are all recognized as 1234.56:
echo "1,234.56" | lawkit benf    # English
echo "１，２３４．５６" | lawkit benf  # Japanese numbers (auto-detected)
echo "१,२३४.५६" | lawkit benf    # Hindi numbers (auto-detected)
```

### Can I analyze text in non-English languages?

Yes! lawkit supports Unicode text analysis with automatic language detection:

```bash
# Japanese text analysis (automatic language detection)
lawkit zipf --text japanese_document.txt

# Chinese text analysis (automatic language detection)
lawkit zipf --text chinese_text.txt

# Multilingual documents (automatic language detection)
lawkit zipf --text multilingual_doc.txt
```

### What languages does lawkit support?

lawkit provides English output unified across all analysis, with automatic recognition of international number formats:
```bash
# English output (unified)
lawkit benf data.csv

# International numbers automatically recognized
echo "１２３４５６" | lawkit benf      # Japanese numbers
echo "一千二百三十四" | lawkit benf    # Chinese numbers
echo "१२३४५६" | lawkit benf        # Hindi numbers
echo "١٢٣٤٥٦" | lawkit benf        # Arabic numbers
```

## Integration and Advanced Features

### How does multi-law comparison work?

The `analyze` command analyzes data with multiple statistical laws:

```bash
# Analyze with specific laws
lawkit analyze --laws benf,pareto data.csv

# Analyze with all applicable laws
lawkit analyze --laws all data.csv
```

Features include:
- **Contradiction Detection**: Identifies conflicting results
- **Confidence Scoring**: Rates reliability of each analysis
- **Recommendations**: Suggests most appropriate law
- **Meta-Analysis**: Combines insights from multiple perspectives

### Can I use lawkit in my own software?

Yes! Use the `lawkit-core` library:

```rust
use lawkit_core::laws::benford::BenfordResult;

fn analyze_data(numbers: &[f64]) {
    let result = BenfordResult::analyze(numbers);
    println!("Chi-square: {}", result.chi_square);
}
```

### How do I integrate lawkit with CI/CD pipelines?

See our [Integration Guide](../guides/integrations.md) for examples with:
- GitHub Actions
- GitLab CI
- Jenkins
- Docker containers

## Performance and Troubleshooting

### lawkit is slow on large datasets. How can I improve performance?

**Optimization strategies:**
```bash
# Use quiet mode for faster processing
lawkit benf --quiet large_data.csv

# Filter data to reduce size
lawkit benf --filter ">=1000" large_data.csv

# Use appropriate minimum counts
lawkit benf --min-count 1000 large_data.csv

# Choose efficient output formats
lawkit benf --format json large_data.csv  # Faster than text
```

### I'm getting "insufficient data" errors. What should I do?

This error occurs when your dataset doesn't meet minimum requirements:

```bash
# Check your data size
wc -l data.csv

# Reduce minimum count requirement
lawkit benf --min-count 5 small_data.csv

# Use appropriate law for your data size
lawkit pareto small_data.csv  # Lower requirements than normal
```

### The analysis results seem incorrect. What could be wrong?

**Common issues:**
1. **Wrong statistical law**: Not all data fits all laws
2. **Data preprocessing needed**: Remove headers, filter outliers
3. **Insufficient data**: Too few data points for reliable analysis
4. **Wrong format**: Ensure numerical data is properly formatted

**Debugging steps:**
```bash
# Verbose output for more details
lawkit benf --verbose data.csv

# Check data with different laws
lawkit analyze --laws all data.csv

# Validate data format
head -10 data.csv
```

### Can I analyze data in real-time?

Yes, lawkit supports piped input:

```bash
# Pipe data from other commands
tail -f logfile.txt | grep "amount:" | lawkit benf

# Process continuous data
while true; do
    get_latest_data | lawkit benf --quiet
    sleep 60
done
```

## Error Messages

### "Failed to parse input data"

This usually means:
- Non-numerical data in input
- Incorrect file format
- Encoding issues

**Solutions:**
```bash
# Check file encoding
file data.csv

# Validate CSV format
csvlint data.csv

# Extract only numerical columns
cut -d',' -f2 data.csv | lawkit benf
```

### "No statistical law applicable"

This occurs when:
- Dataset is too small
- Data doesn't fit any implemented law
- All laws failed their applicability tests

**Solutions:**
```bash
# Try with lower minimum counts
lawkit analyze --laws all --min-count 5 data.csv

# Check data characteristics
lawkit analyze --laws all --verbose data.csv
```

### "Permission denied" or "Access denied"

File permission issues:
```bash
# Check file permissions
ls -la data.csv

# Make file readable
chmod 644 data.csv

# Check if file exists
test -f data.csv && echo "File exists" || echo "File not found"
```

## Getting Help

### Where can I get more help?

- **Documentation**: [docs/](index.md)
- **Issues**: [GitHub Issues](https://github.com/kako-jun/lawkit/issues)
- **Discussions**: [GitHub Discussions](https://github.com/kako-jun/lawkit/discussions)
- **Examples**: Check the `tests/fixtures/` directory in the repository

### How do I report a bug?

1. Check existing issues on GitHub
2. Provide minimal reproduction case
3. Include system information and lawkit version
4. Attach sample data if possible (anonymized)

### How do I request a new feature?

1. Open a GitHub Discussion to discuss the idea
2. Explain the use case and expected behavior
3. Consider contributing the implementation
4. Check our [Contributing Guide](../CONTRIBUTING.md)

### Is there a community or forum?

- **GitHub Discussions**: General questions and ideas
- **GitHub Issues**: Bug reports and feature requests
- **Email**: Direct contact for sensitive issues

We welcome contributions and feedback from the community!