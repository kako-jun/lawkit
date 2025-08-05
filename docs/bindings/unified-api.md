# lawkit Unified API Reference

*Language bindings API documentation for lawkit-python and lawkit-js*

## Overview

lawkit provides a unified API for statistical law analysis, particularly useful for fraud detection, data quality assessment, and audit compliance. It implements various statistical laws including Benford's Law, Pareto distribution, Zipf's Law, and more.

## Main Function

### `law(subcommand, data_or_config, options)`

Analyzes data using statistical laws or performs utility operations.

#### Parameters

- `subcommand` (String): The analysis type or utility command to execute
- `data_or_config` (Value): Input data or configuration depending on the subcommand
- `options` (LawkitOptions, optional): Configuration options for the analysis

#### Returns

- `Result<LawkitResult, Error>`: Analysis results or utility output

#### Example

```rust
use lawkit_core::{law, LawkitOptions};
use serde_json::json;

// Benford's Law analysis
let data = json!([123, 456, 789, 111, 222, 333]);
let options = LawkitOptions {
    format: Some("json".to_string()),
    min_count: Some(100),
    ..Default::default()
};

let result = law("benford", &data, Some(&options))?;
```

## Subcommands

### Analysis Commands

#### `benford` - Benford's Law Analysis
Analyzes if data follows Benford's Law for first digit distribution.

```rust
let data = json!([/* numerical data */]);
let result = law("benford", &data, None)?;
```

#### `pareto` - Pareto Distribution Analysis
Checks if data follows the 80/20 rule or other Pareto distributions.

```rust
let data = json!({ "values": [/* data points */] });
let result = law("pareto", &data, None)?;
```

#### `zipf` - Zipf's Law Analysis
Analyzes if data follows Zipf's Law (common in natural language and city populations).

```rust
let data = json!({ "frequencies": [/* frequency data */] });
let result = law("zipf", &data, None)?;
```

#### `normal` - Normal Distribution Analysis
Tests if data follows a normal (Gaussian) distribution.

```rust
let data = json!([/* numerical data */]);
let result = law("normal", &data, None)?;
```

#### `poisson` - Poisson Distribution Analysis
Checks if data follows a Poisson distribution (events in fixed intervals).

```rust
let data = json!({ "events": [/* event counts */] });
let result = law("poisson", &data, None)?;
```

#### `analyze` - Automatic Analysis
Automatically detects and applies relevant statistical laws.

```rust
let data = json!({ "dataset": [/* mixed data */] });
let result = law("analyze", &data, None)?;
```

### Utility Commands

#### `validate` - Data Validation
Validates data format and structure for law analysis.

```rust
let data = json!({ "data": [/* data to validate */] });
let result = law("validate", &data, None)?;
```

#### `diagnose` - Diagnostic Information
Provides diagnostic information about the data and potential analyses.

```rust
let data = json!({ "dataset": [/* data */] });
let result = law("diagnose", &data, None)?;
```

#### `generate` - Test Data Generation
Generates test data following specified statistical laws.

```rust
let config = json!({
    "law": "benford",
    "count": 1000,
    "seed": 42
});
let result = law("generate", &config, None)?;
```

#### `list` - List Available Laws
Lists all available statistical laws and their descriptions.

```rust
let empty = json!({});
let result = law("list", &empty, None)?;
```

#### `selftest` - Self-Test
Runs internal tests to verify lawkit functionality.

```rust
let empty = json!({});
let result = law("selftest", &empty, None)?;
```

## Options

### LawkitOptions Structure

```rust
pub struct LawkitOptions {
    // Output control
    pub format: Option<String>,
    pub quiet: Option<bool>,
    pub verbose: Option<bool>,
    pub no_color: Option<bool>,
    
    // Analysis parameters
    pub min_count: Option<usize>,
    pub confidence_level: Option<f64>,
    pub significance_level: Option<f64>,
    
    // Analysis filters
    pub filter: Option<String>,
    pub laws: Option<Vec<String>>,
    pub focus: Option<String>,
    
    // Thresholds
    pub threshold: Option<f64>,
    pub tolerance: Option<f64>,
    
    // Advanced options
    pub statistical_tests: Option<Vec<String>>,
    pub include_metadata: Option<bool>,
    pub detailed_report: Option<bool>,
}
```

### Option Details

#### Output Control
- **`format`**: Output format
  - Options: `"json"`, `"csv"`, `"text"`, `"markdown"`
  - Default: `"text"`
  
- **`quiet`**: Suppress non-essential output
  - Default: `false`
  
- **`verbose`**: Enable detailed output
  - Default: `false`
  
- **`no_color`**: Disable colored output
  - Default: `false`

#### Analysis Parameters
- **`min_count`**: Minimum data points required for analysis
  - Default: `30` (varies by law)
  
- **`confidence_level`**: Statistical confidence level (0.0-1.0)
  - Default: `0.95` (95% confidence)
  
- **`significance_level`**: Alpha level for hypothesis testing
  - Default: `0.05`

#### Analysis Filters
- **`filter`**: Filter data before analysis
  - Example: `"value > 100"` or `"category == 'sales'"`
  
- **`laws`**: Specific laws to check (for `analyze` command)
  - Example: `["benford", "pareto"]`
  
- **`focus`**: Focus analysis on specific aspect
  - Example: `"first_digit"` or `"distribution_tail"`

#### Thresholds
- **`threshold`**: Custom threshold for law compliance
  - Default: Varies by law
  
- **`tolerance`**: Acceptable deviation from expected values
  - Default: Varies by law

#### Advanced Options
- **`statistical_tests`**: Additional statistical tests to perform
  - Options: `["chi_square", "kolmogorov_smirnov", "anderson_darling"]`
  
- **`include_metadata`**: Include detailed metadata in results
  - Default: `false`
  
- **`detailed_report`**: Generate comprehensive analysis report
  - Default: `false`

## Result Types

### LawkitResult Enum

```rust
pub enum LawkitResult {
    // Analysis results
    BenfordAnalysis(String, BenfordData),
    ParetoAnalysis(String, ParetoData),
    ZipfAnalysis(String, ZipfData),
    NormalAnalysis(String, NormalData),
    PoissonAnalysis(String, PoissonData),
    
    // Utility results
    ValidationResult(bool, Vec<String>),
    DiagnosticInfo(DiagnosticData),
    GeneratedData(Vec<Value>),
    LawList(Vec<LawInfo>),
    SelfTestResult(TestResults),
    
    // Multi-law analysis
    MultiLawAnalysis(Vec<(String, AnalysisResult)>),
}
```

### Analysis Data Structures

#### BenfordData
```rust
pub struct BenfordData {
    pub digit_frequencies: HashMap<u8, f64>,
    pub expected_frequencies: HashMap<u8, f64>,
    pub chi_square_statistic: f64,
    pub p_value: f64,
    pub compliant: bool,
    pub sample_size: usize,
}
```

#### ParetoData
```rust
pub struct ParetoData {
    pub ratio: (f64, f64),  // e.g., (80.0, 20.0)
    pub gini_coefficient: f64,
    pub lorenz_curve: Vec<(f64, f64)>,
    pub compliant: bool,
}
```

## Language Bindings

### Python

```python
import lawkit_python

# Benford's Law analysis
data = [123, 456, 789, 111, 222, 333]
result = lawkit_python.law("benford", data)

# With options
result = lawkit_python.law(
    "benford",
    data,
    format="json",
    min_count=100,
    confidence_level=0.99,
    detailed_report=True
)

# Automatic analysis
result = lawkit_python.law(
    "analyze",
    {"dataset": data},
    laws=["benford", "pareto", "normal"],
    verbose=True
)
```

### TypeScript/JavaScript

```typescript
import { law, LawkitOptions } from 'lawkit-js';

// Benford's Law analysis
const data = [123, 456, 789, 111, 222, 333];
const result = law('benford', data);

// With options
const options: LawkitOptions = {
    outputFormat: 'json',
    minSampleSize: 100,
    confidenceLevel: 0.99,
    detailedReport: true
};
const result = law('benford', data, options);

// Automatic analysis
const analysisOptions: LawkitOptions = {
    lawsToCheck: ['benford', 'pareto', 'normal'],
    includeMetadata: true
};
const result = law('analyze', { dataset: data }, analysisOptions);
```

## Examples

### Fraud Detection with Benford's Law

```rust
use lawkit_core::{law, LawkitOptions};
use serde_json::json;

// Analyze financial transactions
let transactions = json!([10234, 2341, 45632, 1234, 8765, /*...*/]);

let options = LawkitOptions {
    min_count: Some(1000),
    confidence_level: Some(0.99),
    detailed_report: Some(true),
    ..Default::default()
};

let result = law("benford", &transactions, Some(&options))?;

match result {
    LawkitResult::BenfordAnalysis(_, data) => {
        if !data.compliant {
            println!("Warning: Potential fraud detected!");
            println!("Chi-square: {}, p-value: {}", 
                     data.chi_square_statistic, 
                     data.p_value);
        }
    }
    _ => {}
}
```

### Data Quality Assessment

```rust
// Check multiple laws for data quality
let options = LawkitOptions {
    laws: Some(vec!["benford".to_string(), 
                    "pareto".to_string(), 
                    "normal".to_string()]),
    verbose: Some(true),
    ..Default::default()
};

let result = law("analyze", &dataset, Some(&options))?;
```

### Generating Test Data

```rust
let config = json!({
    "law": "zipf",
    "count": 10000,
    "parameters": {
        "s": 1.2,
        "n": 100
    },
    "seed": 12345
});

let result = law("generate", &config, None)?;
```

## Performance Considerations

- **Large Datasets**: lawkit automatically uses streaming analysis for datasets >100MB
- **Memory Usage**: Use `filter` option to reduce dataset size before analysis
- **Parallel Processing**: Multiple laws in `analyze` are processed in parallel
- **Caching**: Results are cached for repeated analyses on the same dataset

## Error Handling

Detailed errors are provided for:
- Insufficient data (below `min_count`)
- Invalid data format
- Statistical computation failures
- Memory allocation issues
- Invalid subcommands or options

## Best Practices

1. **Data Size**: Ensure sufficient data points (typically >1000 for Benford's Law)
2. **Data Cleaning**: Use `validate` before analysis to check data quality
3. **Multiple Laws**: Use `analyze` to automatically detect applicable laws
4. **Confidence Levels**: Adjust based on your use case (fraud detection: 0.99, general analysis: 0.95)
5. **Filtering**: Pre-filter data to relevant subsets for more accurate analysis