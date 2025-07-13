# lawkit-js

Node.js wrapper for the `lawkit` CLI tool - comprehensive statistical law analysis toolkit for fraud detection, data quality assessment, and business intelligence.

## Installation

```bash
npm install lawkit-js
```

Automatically downloads the appropriate `lawkit` binary for your system from GitHub Releases.

### Supported Platforms

- **Linux**: x86_64
- **macOS**: x86_64 and Apple Silicon (ARM64)
- **Windows**: x86_64

Binaries are automatically downloaded and stored in platform-specific directories:
```
bin/
├── linux-x64/lawkit
├── darwin-x64/lawkit  
├── darwin-arm64/lawkit
└── win32-x64/lawkit.exe
```

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

# Generate test data
lawkit generate benf --count 1000 --output-file test-data.csv
```

### JavaScript API

```javascript
const { benford, pareto, zipf, normal, poisson, analyze, validate, diagnose, generate, list } = require('lawkit-js');

// Analyze array data
const numbers = [1, 10, 100, 1000, 2000];
const result = await benford(numbers, { output: 'json' });
console.log('Risk level:', result.risk_level);

// Analyze file data
const fileResult = await benford('data.csv', { 
  output: 'json',
  confidence: 0.99 
});

// Generate sample data
const sampleData = await generate('benf', { 
  count: 1000,
  outputFile: 'sample.csv' 
});

// Multi-law analysis
const analysisResult = await analyze('data.csv', {
  output: 'json',
  crossValidation: true
});

// Data validation
const validationResult = await validate('data.csv', {
  consistencyCheck: true,
  report: true
});

// List available laws
const availableLaws = await list({ output: 'json' });
```

## API Reference

### Analysis Functions

#### `benford(data, options)`
Analyze data using Benford's Law for fraud detection.

#### `pareto(data, options)`
Analyze data using the Pareto Principle for business insights.

#### `zipf(data, options)`
Analyze data using Zipf's Law for text and frequency analysis.

#### `normal(data, options)`
Analyze data using Normal Distribution for quality control.

#### `poisson(data, options)`
Analyze data using Poisson Distribution for event analysis.

#### `analyze(data, options)`
Perform comprehensive multi-law analysis.

#### `validate(data, options)`
Validate data quality using statistical tests.

#### `diagnose(data, options)`
Diagnose data anomalies and provide recommendations.

### Utility Functions

#### `generate(law, options)`
Generate sample data for testing statistical laws.

#### `list(options)`
List available statistical laws and commands.

#### `isLawkitAvailable()`
Check if the lawkit binary is available.

### Options

All analysis functions accept these common options:

```typescript
interface LawkitOptions {
  output?: 'text' | 'json' | 'csv' | 'yaml' | 'toml' | 'xml';
  minCount?: number;
  confidence?: number;
  sampleSize?: number;
  minValue?: number;
  quiet?: boolean;
  verbose?: boolean;
  outputFile?: string;
  businessAnalysis?: boolean;
  giniCoefficient?: boolean;
  percentiles?: string;
  crossValidation?: boolean;
  consistencyCheck?: boolean;
  confidenceLevel?: number;
  report?: boolean;
}
```

## Error Handling

```javascript
const { benford, LawkitError } = require('lawkit-js');

try {
  const result = await benford('data.csv', { output: 'json' });
  console.log(result);
} catch (error) {
  if (error instanceof LawkitError) {
    console.error('lawkit error:', error.message);
    console.error('Exit code:', error.exitCode);
    console.error('stderr:', error.stderr);
  } else {
    console.error('Unexpected error:', error);
  }
}
```

## Features

- **Universal Binary Support**: Automatic platform detection and binary download
- **Comprehensive API**: Full JavaScript API with TypeScript definitions
- **Statistical Laws**: Benford, Pareto, Zipf, Normal, Poisson distributions
- **Advanced Analysis**: Multi-law comparison, validation, diagnostics
- **Data Generation**: Create test datasets for validation
- **Multiple Output Formats**: JSON, CSV, YAML, TOML, XML support
- **Business Intelligence**: Built-in business analysis features
- **Cross-platform**: Linux, macOS (Intel & ARM), Windows support

## Requirements

- Node.js 12.0.0 or higher
- Internet connection for initial binary download

## License

MIT

## Links

- [GitHub Repository](https://github.com/kako-jun/lawkit)
- [Documentation](https://github.com/kako-jun/lawkit/tree/main/docs)
- [Issues](https://github.com/kako-jun/lawkit/issues)