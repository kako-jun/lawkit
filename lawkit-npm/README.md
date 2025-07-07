# lawkit-js

A Node.js wrapper for the `lawkit` CLI tool - a comprehensive statistical law analysis toolkit for fraud detection, data quality assessment, and audit compliance.

## Installation

```bash
npm install lawkit-js
```

This will automatically download the appropriate `lawkit` binary for your system from GitHub Releases.

## Supported Platforms

- **Linux**: x86_64, aarch64
- **macOS**: x86_64 (Intel), aarch64 (Apple Silicon)
- **Windows**: x86_64

## Usage

### Command Line Interface

After installation, you can use `lawkit` directly from the command line:

```bash
# Analyze data with Benford's Law
lawkit benf data.csv

# Pareto analysis (80/20 rule)
lawkit pareto sales.json --business-analysis

# Normal distribution analysis
lawkit normal measurements.txt --verbose

# Poisson distribution analysis
lawkit poisson events.csv --predict 30

# Zipf's law analysis
lawkit zipf document.txt --language en

# Compare multiple statistical laws
lawkit compare data.csv
```

### Programmatic Usage

You can also use lawkit programmatically in your Node.js applications:

```javascript
const { runLawkit } = require('lawkit-js');

async function analyzeFraud() {
  // Analyze financial data with Benford's Law
  const result = await runLawkit(['benf', 'financial-data.csv', '--format', 'json']);
  
  if (result.code === 0) {
    const analysis = JSON.parse(result.stdout);
    console.log('Benford Analysis:', analysis);
    
    // Check for potential fraud indicators
    if (analysis.risk_level === 'High') {
      console.log('⚠️  Potential fraud detected!');
    }
  } else {
    console.error('Analysis failed:', result.stderr);
  }
}

async function qualityAssessment() {
  // Pareto analysis for quality control
  const result = await runLawkit(['pareto', 'defects.csv', '--gini-coefficient']);
  
  if (result.code === 0) {
    console.log('Quality Analysis Results:');
    console.log(result.stdout);
  }
}

async function eventAnalysis() {
  // Poisson analysis for event prediction
  const result = await runLawkit(['poisson', 'incidents.csv', '--predict', '30', '--format', 'json']);
  
  if (result.code === 0) {
    const prediction = JSON.parse(result.stdout);
    console.log('Event Prediction:', prediction);
  }
}

// Run analyses
analyzeFraud();
qualityAssessment();
eventAnalysis();
```

## Statistical Laws Supported

### 1. Benford's Law (`lawkit benf`)
- **Use Cases**: Fraud detection, data quality assessment, audit compliance
- **Input**: Financial data, accounting records, population data
- **Output**: Conformity analysis, risk assessment, anomaly detection

### 2. Pareto Principle (`lawkit pareto`)
- **Use Cases**: Business analysis, quality control, resource optimization
- **Input**: Sales data, defect counts, customer metrics
- **Output**: 80/20 analysis, Gini coefficient, business insights

### 3. Zipf's Law (`lawkit zipf`)
- **Use Cases**: Text analysis, language processing, content optimization
- **Input**: Text documents, word frequencies, linguistic data
- **Output**: Word frequency analysis, language patterns, content insights

### 4. Normal Distribution (`lawkit normal`)
- **Use Cases**: Quality control, process monitoring, statistical validation
- **Input**: Measurements, process data, experimental results
- **Output**: Normality tests, outlier detection, process capability

### 5. Poisson Distribution (`lawkit poisson`)
- **Use Cases**: Event prediction, risk assessment, capacity planning
- **Input**: Event counts, incident data, arrival times
- **Output**: Rate estimation, probability calculations, predictions

### 6. Comparative Analysis (`lawkit compare`)
- **Use Cases**: Multi-law validation, comprehensive analysis
- **Input**: Any numerical data
- **Output**: Best-fit law identification, comparative metrics

## Input Formats Supported

- **CSV**: Comma-separated values
- **JSON**: JavaScript Object Notation
- **YAML**: YAML Ain't Markup Language
- **TOML**: Tom's Obvious, Minimal Language
- **XML**: eXtensible Markup Language
- **Excel**: .xlsx files
- **Text**: Plain text files
- **PDF**: Portable Document Format (text extraction)

## International Number Support

lawkit supports international number formats:
- **English**: 1,234.56
- **Japanese**: １，２３４．５６
- **Chinese**: 一千二百三十四点五六
- **Hindi**: १,२३४.५६
- **Arabic**: ١٬٢٣٤٫٥٦

## Examples

### Fraud Detection with Benford's Law
```bash
# Basic analysis
lawkit benf accounting-data.csv

# Detailed analysis with risk assessment
lawkit benf financial-records.xlsx --format json --verbose

# Multi-column analysis
lawkit benf transactions.csv --columns amount,balance --threshold 0.05
```

### Business Intelligence with Pareto Analysis
```bash
# Sales analysis
lawkit pareto sales-data.csv --business-analysis

# Quality control
lawkit pareto defects.csv --gini-coefficient --percentiles 70,80,90

# Customer analysis
lawkit pareto customers.json --format yaml
```

### Content Analysis with Zipf's Law
```bash
# Document analysis
lawkit zipf document.txt --language en

# Website content analysis
lawkit zipf content.html --min-count 5

# Multi-language support
lawkit zipf text-ja.txt --language ja
```

## Development

To link for local development:

```bash
npm link
```

To test the package:

```bash
node index.js --help
```

## Contributing

Contributions are welcome! Please visit the [lawkit repository](https://github.com/kako-jun/lawkit) for more information.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/kako-jun/lawkit/blob/main/LICENSE) file for details.

## Links

- **GitHub Repository**: https://github.com/kako-jun/lawkit
- **Documentation**: https://github.com/kako-jun/lawkit/tree/main/docs
- **Issues**: https://github.com/kako-jun/lawkit/issues
- **Releases**: https://github.com/kako-jun/lawkit/releases