# lawkit-js

Node.js wrapper for the `lawkit` CLI tool - comprehensive statistical law analysis toolkit for fraud detection, data quality assessment, and business intelligence.

## Installation

```bash
npm install lawkit-js
```

Automatically downloads the appropriate `lawkit` binary for your system from GitHub Releases.

## Quick Start

### CLI Usage
```bash
# Fraud detection with Benford Law
lawkit benf financial-data.csv

# Business analysis with Pareto principle
lawkit pareto sales.csv --business-analysis

# Multi-law analysis
lawkit analyze data.csv --laws benf,pareto,zipf

# Data validation
lawkit validate data.csv --consistency-check

# Conflict diagnosis
lawkit diagnose data.csv --report detailed
```

### Programmatic Usage
```javascript
const { runLawkit } = require('lawkit-js');

async function analyzeFraud() {
  // Benford analysis for fraud detection
  const benf = await runLawkit(['benf', 'data.csv', '--format', 'json']);
  
  // Multi-law analysis for comprehensive insights
  const analysis = await runLawkit(['analyze', 'data.csv', '--format', 'json', '--laws', 'benf,pareto']);
  
  // Data validation for consistency check
  const validation = await runLawkit(['validate', 'data.csv', '--format', 'json']);
  
  if (benf.code === 0) {
    const result = JSON.parse(benf.stdout);
    if (result.risk_level === 'High') {
      console.log('⚠️  Potential fraud detected!');
    }
  }
}
```

## Supported Laws

- **Benford Law**: Fraud detection in financial data
- **Pareto Analysis**: 80/20 rule and business optimization
- **Zipf Law**: Text analysis and frequency patterns
- **Normal Distribution**: Quality control and outlier detection
- **Poisson Distribution**: Event prediction and risk assessment
- **Multi-Law Analysis**: Comprehensive statistical integration
- **Data Validation**: Consistency checking and quality assessment
- **Conflict Diagnosis**: Detailed anomaly detection and reporting

## Platforms

- **Linux**: x86_64, aarch64
- **macOS**: Intel, Apple Silicon  
- **Windows**: x86_64

## Input Formats

CSV, JSON, YAML, TOML, XML, Excel (.xlsx), Text, PDF

## International Support

Supports number formats in English, Japanese, Chinese, Hindi, and Arabic.

## Links

- **GitHub**: https://github.com/kako-jun/lawkit
- **Documentation**: https://github.com/kako-jun/lawkit/tree/main/docs
- **PyPI Package**: https://pypi.org/project/lawkit-python/

## License

MIT License - see [LICENSE](https://github.com/kako-jun/lawkit/blob/main/LICENSE)