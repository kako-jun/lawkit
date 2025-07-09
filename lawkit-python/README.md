# lawkit-python

Python wrapper for the `lawkit` CLI tool - Statistical law analysis toolkit for fraud detection and data quality assessment.

## Installation

```bash
pip install lawkit-python
```

This will automatically download the appropriate `lawkit` binary for your system from GitHub Releases.

## Quick Start

```python
import lawkit

# Analyze financial data with Benford Law
result = lawkit.analyze_benford('financial_data.csv')
print(result)

# Get structured JSON output
json_result = lawkit.analyze_benford(
    'accounting.csv',
    lawkit.LawkitOptions(format='json')
)
print(f"Risk level: {json_result.risk_level}")
print(f"P-value: {json_result.p_value}")

# Check if data follows Pareto principle (80/20 rule)
pareto_result = lawkit.analyze_pareto(
    'sales_data.csv',
    lawkit.LawkitOptions(format='json', gini_coefficient=True)
)
print(f"Gini coefficient: {pareto_result.gini_coefficient}")
print(f"80/20 concentration: {pareto_result.concentration_80_20}")
```

## Features

### Statistical Laws Supported

- **Benford Law**: Detect fraud and anomalies in numerical data
- **Pareto Principle**: Analyze 80/20 distributions and concentration
- **Zipf Law**: Analyze word frequencies and power-law distributions
- **Normal Distribution**: Test for normality and detect outliers
- **Poisson Distribution**: Analyze rare events and count data

### Advanced Analysis

- **Multi-law Comparison**: Compare multiple statistical laws on the same data
- **Outlier Detection**: Advanced anomaly detection algorithms
- **Time Series Analysis**: Trend and seasonality detection
- **International Numbers**: Support for various number formats (Japanese, Chinese, etc.)
- **Memory Efficient**: Handle large datasets with streaming analysis

### File Format Support

- **CSV, JSON, YAML, TOML, XML**: Standard structured data formats
- **Excel Files**: `.xlsx` and `.xls` support
- **PDF Documents**: Extract and analyze numerical data from PDFs
- **Word Documents**: Analyze data from `.docx` files
- **PowerPoint**: Extract data from presentations

## Usage Examples

### Modern API (Recommended)

```python
import lawkit

# Analyze with Benford Law
result = lawkit.analyze_benford('invoice_data.csv')
print(result)

# Get detailed JSON analysis
json_result = lawkit.analyze_benford(
    'financial_statements.xlsx',
    lawkit.LawkitOptions(
        format='excel',
        output='json',
        confidence=0.95,
        verbose=True
    )
)

if json_result.risk_level == "High":
    print("⚠️  High risk of fraud detected!")
    print(f"Chi-square: {json_result.chi_square}")
    print(f"P-value: {json_result.p_value}")
    print(f"MAD: {json_result.mad}%")

# Pareto analysis for business insights
pareto_result = lawkit.analyze_pareto(
    'customer_revenue.csv',
    lawkit.LawkitOptions(
        output='json',
        gini_coefficient=True,
        business_analysis=True,
        percentiles="70,80,90"
    )
)

print(f"Top 20% customers generate {pareto_result.concentration_80_20:.1f}% of revenue")
print(f"Income inequality (Gini): {pareto_result.gini_coefficient:.3f}")

# Normal distribution analysis with outlier detection
normal_result = lawkit.analyze_normal(
    'quality_measurements.csv',
    lawkit.LawkitOptions(
        output='json',
        outlier_detection=True,
        test_type='shapiro'
    )
)

if normal_result.p_value < 0.05:
    print("Data does not follow normal distribution")
    if normal_result.outliers:
        print(f"Found {len(normal_result.outliers)} outliers")

# Multi-law analysis
analysis = lawkit.analyze_laws(
    'complex_dataset.csv',
    lawkit.LawkitOptions(format='json', laws='benf,pareto,zipf')
)
print(f"Analysis results: {analysis.data}")
print(f"Overall risk level: {analysis.risk_level}")

# Data validation
validation = lawkit.validate_laws(
    'complex_dataset.csv',
    lawkit.LawkitOptions(format='json', consistency_check=True)
)
print(f"Validation status: {validation.data}")

# Conflict diagnosis
diagnosis = lawkit.diagnose_laws(
    'complex_dataset.csv',
    lawkit.LawkitOptions(format='json', report='detailed')
)
print(f"Diagnosis: {diagnosis.data}")
```

### Generate Sample Data

```python
import lawkit

# Generate Benford Law compliant data
benford_data = lawkit.generate_data('benf', samples=1000, seed=42)
print(benford_data)

# Generate normal distribution data
normal_data = lawkit.generate_data('normal', samples=500, mean=100, stddev=15)

# Generate Pareto distribution data
pareto_data = lawkit.generate_data('pareto', samples=1000, concentration=0.8)

# Test the pipeline: generate → analyze
data = lawkit.generate_data('benf', samples=10000, seed=42)
result = lawkit.analyze_string(data, 'benf', lawkit.LawkitOptions(output='json'))
print(f"Generated data risk level: {result.risk_level}")
```

### Analyze String Data Directly

```python
import lawkit

# Analyze CSV data from string
csv_data = """amount
123.45
456.78
789.12
234.56
567.89"""

result = lawkit.analyze_string(
    csv_data,
    'benf',
    lawkit.LawkitOptions(format='json')
)
print(f"Risk assessment: {result.risk_level}")

# Analyze JSON data
json_data = '{"values": [12, 23, 34, 45, 56, 67, 78, 89]}'
result = lawkit.analyze_string(
    json_data,
    'normal',
    lawkit.LawkitOptions(format='json')
)
print(f"Is normal: {result.p_value > 0.05}")
```

### Advanced Options

```python
import lawkit

# High-performance analysis with optimization
result = lawkit.analyze_benford(
    'large_dataset.csv',
    lawkit.LawkitOptions(
        optimize=True,
        parallel=True,
        memory_efficient=True,
        min_count=50,
        threshold=0.001
    )
)

# International number support
result = lawkit.analyze_benford(
    'japanese_accounting.csv',
    lawkit.LawkitOptions(
        international=True,
        format='csv',
        output='json'
    )
)

# Time series analysis
result = lawkit.analyze_normal(
    'sensor_data.csv',
    lawkit.LawkitOptions(
        time_series=True,
        outlier_detection=True,
        output='json'
    )
)
```

### Legacy API (Backward Compatibility)

```python
from lawkit import run_lawkit

# Direct command execution
result = run_lawkit(["benf", "data.csv", "--format", "csv", "--output", "json"])

if result.returncode == 0:
    print("Analysis successful")
    print(result.stdout)
else:
    print("Analysis failed")
    print(result.stderr)

# Legacy analysis functions
from lawkit.compat import run_benford_analysis, run_pareto_analysis

benford_result = run_benford_analysis("financial.csv", format="csv", output="json")
pareto_result = run_pareto_analysis("sales.csv", gini_coefficient=True)
```

## Installation and Setup

### Automatic Installation (Recommended)

```bash
pip install lawkit-python
```

The package will automatically download the appropriate binary for your platform.

### Manual Binary Installation

If automatic download fails:

```bash
lawkit-download-binary
```

### Development Installation

```bash
git clone https://github.com/kako-jun/lawkit
cd lawkit/lawkit-python
pip install -e .[dev]
```

### Verify Installation

```python
import lawkit

# Check if lawkit is available
if lawkit.is_lawkit_available():
    print("✅ lawkit is installed and working")
    print(f"Version: {lawkit.get_version()}")
else:
    print("❌ lawkit is not available")

# Run self-test
if lawkit.selftest():
    print("✅ All tests passed")
else:
    print("❌ Self-test failed")
```

## Use Cases

### Financial Fraud Detection

```python
import lawkit

# Analyze invoice amounts for fraud
result = lawkit.analyze_benford('invoices.csv', 
                               lawkit.LawkitOptions(output='json'))

if result.risk_level in ['High', 'Critical']:
    print("🚨 Potential fraud detected in invoice data")
    print(f"Statistical significance: p={result.p_value:.6f}")
    print(f"Deviation from Benford Law: {result.mad:.2f}%")
```

### Business Intelligence

```python
import lawkit

# Analyze customer revenue distribution
result = lawkit.analyze_pareto('customer_revenue.csv',
                              lawkit.LawkitOptions(
                                  output='json',
                                  business_analysis=True,
                                  gini_coefficient=True
                              ))

print(f"Revenue concentration: {result.concentration_80_20:.1f}%")
print(f"Market inequality: {result.gini_coefficient:.3f}")
```

### Quality Control

```python
import lawkit

# Analyze manufacturing measurements
result = lawkit.analyze_normal('measurements.csv',
                              lawkit.LawkitOptions(
                                  output='json',
                                  outlier_detection=True,
                                  test_type='shapiro'
                              ))

if result.p_value < 0.05:
    print("⚠️  Process out of control - not following normal distribution")
    if result.outliers:
        print(f"Found {len(result.outliers)} outlying measurements")
```

### Text Analysis

```python
import lawkit

# Analyze word frequency in documents
result = lawkit.analyze_zipf('document.txt',
                            lawkit.LawkitOptions(output='json'))

print(f"Text follows Zipf Law: {result.p_value > 0.05}")
print(f"Power law exponent: {result.exponent:.3f}")
```

## API Reference

### Main Functions

- `analyze_benford(input_data, options)` - Benford Law analysis
- `analyze_pareto(input_data, options)` - Pareto principle analysis  
- `analyze_zipf(input_data, options)` - Zipf Law analysis
- `analyze_normal(input_data, options)` - Normal distribution analysis
- `analyze_poisson(input_data, options)` - Poisson distribution analysis
- `analyze_laws(input_data, options)` - Multi-law analysis
- `validate_laws(input_data, options)` - Data validation and consistency check
- `diagnose_laws(input_data, options)` - Conflict diagnosis and detailed reporting
- `compare_laws(input_data, options)` - Alias for analyze_laws (backward compatibility)
- `generate_data(law_type, samples, **kwargs)` - Generate sample data
- `analyze_string(content, law_type, options)` - Analyze string data directly

### Utility Functions

- `is_lawkit_available()` - Check if lawkit CLI is available
- `get_version()` - Get lawkit version
- `selftest()` - Run self-test

### Classes

- `LawkitOptions` - Configuration options for analysis
- `LawkitResult` - Analysis results with structured access
- `LawkitError` - Exception class for lawkit errors

## Platform Support

- **Windows**: x86_64
- **macOS**: x86_64, ARM64 (Apple Silicon)
- **Linux**: x86_64, ARM64

## Requirements

- Python 3.8+
- No additional dependencies required

## License

This project is licensed under the MIT License.

## Support

- GitHub Issues: https://github.com/kako-jun/lawkit/issues
- Documentation: https://github.com/kako-jun/lawkit/tree/main/docs
- Examples: https://github.com/kako-jun/lawkit/tree/main/docs/user-guide/examples.md

## Contributing

Contributions are welcome! Please read the [Contributing Guide](https://github.com/kako-jun/lawkit/blob/main/CONTRIBUTING.md) for details.