# lawkit-js

Node.js wrapper for the `lawkit` CLI tool - comprehensive statistical law analysis toolkit for fraud detection, data quality assessment, and business intelligence.

## Installation

```bash
npm install lawkit-js
```

Includes all platform binaries (universal bundle) - no download required.

### Supported Platforms

- **Linux**: x86_64
- **macOS**: x86_64 and Apple Silicon (ARM64)  
- **Windows**: x86_64

All binaries are pre-bundled in the package for immediate use:
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

### JavaScript API (Unified API)

```javascript
const { law } = require('lawkit-js');

// Benford's Law analysis on JavaScript array
const numbers = [123, 187, 234, 298, 345, 456, 567, 678, 789, 1234];
const benfordResult = law("benford", numbers);

console.log(`Benford analysis: ${benfordResult[0].data.analysis_summary}`);
console.log(`Risk level: ${benfordResult[0].data.risk_level}`);
console.log(`P-value: ${benfordResult[0].data.p_value}`);

// Pareto analysis
const salesData = [10000, 9500, 9000, 8500, 1000, 950, 900, 850, 800, 750];
const paretoResult = law("pareto", salesData);

console.log(`Top 20% contribution: ${paretoResult[0].data.top_20_percent_contribution}%`);

// Normal distribution analysis
const qualityScores = [98.5, 99.2, 100.1, 99.8, 100.4, 99.6, 100.8, 99.9];
const normalResult = law("normal", qualityScores);

console.log(`Mean: ${normalResult[0].data.mean}`);
console.log(`Standard deviation: ${normalResult[0].data.std_dev}`);

// Data validation
const testData = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const validationResult = law("validate", testData);

console.log(`Validation passed: ${validationResult[0].data.validation_passed}`);
console.log(`Data quality score: ${validationResult[0].data.data_quality_score}`);

// Generate sample data
const config = { type: "benford", count: 100 };
const generatedResult = law("generate", config);

console.log(`Generated ${generatedResult[0].data.count} data points`);
console.log(`Sample data: ${generatedResult[0].data.sample_data.slice(0, 5)}...`);

// Comprehensive analysis
const comprehensiveData = {
  financial_data: [123, 187, 234, 298, 345],
  quality_scores: [98.5, 99.2, 100.1, 99.8, 100.4]
};
const analysisResult = law("analyze", comprehensiveData);

console.log(`Found ${analysisResult.length} analyses`);
analysisResult.forEach(result => {
  console.log(`${result.type}: ${result.data.analysis_summary}`);
});
```

## API Reference

### Unified API

#### `law(subcommand, data, options?)`
The unified function for all statistical law analysis.

**Parameters:**
- `subcommand`: String - The analysis type ("benford", "pareto", "zipf", "normal", "poisson", "analyze", "validate", "diagnose", "generate")
- `data`: Array or Object - The data to analyze (or configuration object for "generate")
- `options`: Object (optional) - Analysis options

**Supported Subcommands:**
- `"benford"` - Benford's Law analysis for fraud detection
- `"pareto"` - Pareto Principle analysis for business insights  
- `"zipf"` - Zipf's Law analysis for frequency distributions
- `"normal"` - Normal distribution analysis for quality control
- `"poisson"` - Poisson distribution analysis for event analysis
- `"validate"` - Data quality validation
- `"diagnose"` - Data anomaly diagnosis
- `"generate"` - Sample data generation
- `"analyze"` - Comprehensive multi-law analysis

**Return Value:**
Returns an array of analysis result objects, each containing:
- `type`: The analysis type (e.g., 'BenfordAnalysis', 'ParetoAnalysis')
- `data`: Analysis results with statistical measures and summary

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
const { law } = require('lawkit-js');

try {
  const result = law("benford", [1, 2, 3]);
  console.log(result);
} catch (error) {
  console.error('lawkit error:', error.message);
  // Handle error appropriately
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