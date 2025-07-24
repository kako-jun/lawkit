# API Reference - lawkit-core

Complete API documentation for the `lawkit-core` Rust crate, providing statistical law analysis functionality.

## Overview

The `lawkit-core` crate is the heart of the lawkit ecosystem, providing comprehensive statistical law analysis for data quality assessment and fraud detection. It can be embedded in other Rust applications to add statistical analysis capabilities.

**Unified API Design**: The core API exposes only a single main function `law()` for all analysis operations. All functionality is accessed through this unified interface using the subcommand and options parameters. This design ensures consistency and simplicity across all use cases.

## Installation

Add `lawkit-core` to your `Cargo.toml`:

```toml
[dependencies]
lawkit-core = "0.2.0"
```

### Feature Flags

```toml
[dependencies]
lawkit-core = { version = "0.2.0", features = ["all-laws"] }
```

Available features:
- `benford` (default) - Benford's Law analysis
- `pareto` (default) - Pareto distribution analysis
- `zipf` (default) - Zipf's Law analysis
- `normal` - Normal distribution tests
- `poisson` - Poisson distribution analysis
- `all-laws` - Enable all statistical laws

## Public API

### Core Types

#### `LawkitResult`

Represents the result of a statistical law analysis.

```rust
#[derive(Debug, PartialEq, Serialize)]
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

#### Analysis Data Structures

```rust
pub struct BenfordData {
    pub digit_frequencies: HashMap<u8, f64>,
    pub expected_frequencies: HashMap<u8, f64>,
    pub chi_square_statistic: f64,
    pub p_value: f64,
    pub compliant: bool,
    pub sample_size: usize,
}

pub struct ParetoData {
    pub ratio: (f64, f64),  // e.g., (80.0, 20.0)
    pub gini_coefficient: f64,
    pub lorenz_curve: Vec<(f64, f64)>,
    pub compliant: bool,
}
```

### Core Functions

#### `law()`

Primary function for statistical law analysis and utility operations. This is the unified API entry point for all lawkit operations.

```rust
pub fn law(
    subcommand: &str,
    data_or_config: &Value,
    options: Option<&LawkitOptions>,
) -> Result<LawkitResult, Error>
```

**Parameters:**
- `subcommand`: The analysis type or utility command to execute
- `data_or_config`: Input data or configuration depending on the subcommand
- `options`: Optional configuration options for the analysis

**Returns:** `Result<LawkitResult, Error>` representing the analysis results

#### LawkitOptions Structure

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

**Example:**
```rust
use lawkit_core::{law, LawkitOptions, LawkitResult};
use serde_json::{json, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Financial transaction data for fraud detection
    let transactions = json!([10234, 2341, 45632, 1234, 8765]);
    
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
    
    Ok(())
}
```

## Subcommands

### Analysis Commands

- **`benford`** - Benford's Law analysis for first digit distribution
- **`pareto`** - Pareto distribution (80/20 rule) analysis
- **`zipf`** - Zipf's Law analysis for rank-frequency distribution
- **`normal`** - Normal distribution testing
- **`poisson`** - Poisson distribution analysis
- **`analyze`** - Automatic multi-law analysis

### Utility Commands

- **`validate`** - Validate data format and structure
- **`diagnose`** - Provide diagnostic information about data
- **`generate`** - Generate test data following specified laws
- **`list`** - List available statistical laws
- **`selftest`** - Run internal tests

## Advanced Usage

### Custom Analysis Logic

#### Fraud Detection with Benford's Law

```rust
use lawkit_core::{law, LawkitOptions};
use serde_json::json;

fn detect_fraud(transactions: Vec<f64>) -> Result<bool, Box<dyn std::error::Error>> {
    let data = json!(transactions);
    
    let options = LawkitOptions {
        min_count: Some(1000),
        confidence_level: Some(0.99),
        statistical_tests: Some(vec!["chi_square".to_string(), "kolmogorov_smirnov".to_string()]),
        ..Default::default()
    };
    
    let result = law("benford", &data, Some(&options))?;
    
    match result {
        LawkitResult::BenfordAnalysis(_, data) => {
            Ok(!data.compliant && data.p_value < 0.01)
        }
        _ => Ok(false)
    }
}
```

#### Multi-Law Analysis

```rust
let options = LawkitOptions {
    laws: Some(vec!["benford".to_string(), "pareto".to_string(), "normal".to_string()]),
    verbose: Some(true),
    include_metadata: Some(true),
    ..Default::default()
};

let result = law("analyze", &dataset, Some(&options))?;

match result {
    LawkitResult::MultiLawAnalysis(analyses) => {
        for (law_name, analysis) in analyses {
            println!("{}: {}", law_name, if analysis.compliant { "✓" } else { "✗" });
        }
    }
    _ => {}
}
```

### Working with Different Data Types

#### CSV Data Analysis

```rust
use lawkit_core::{law, LawkitOptions};
use csv::Reader;
use serde_json::json;

fn analyze_csv_column(
    file_path: &str,
    column_index: usize
) -> Result<LawkitResult, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let mut values = Vec::new();
    
    for result in reader.records() {
        let record = result?;
        if let Some(value) = record.get(column_index) {
            if let Ok(num) = value.parse::<f64>() {
                values.push(num);
            }
        }
    }
    
    let data = json!(values);
    Ok(law("benford", &data, None)?)
}
```

### Integration Patterns

#### Continuous Monitoring

```rust
use lawkit_core::{law, LawkitOptions, LawkitResult};
use serde_json::Value;
use std::time::Duration;
use tokio;

struct DataMonitor {
    pub threshold: f64,
    pub min_sample_size: usize,
}

impl DataMonitor {
    pub async fn monitor_data_quality(
        &self,
        data_source: impl Fn() -> Value
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Hourly
        
        loop {
            interval.tick().await;
            
            let data = data_source();
            let options = LawkitOptions {
                min_count: Some(self.min_sample_size),
                threshold: Some(self.threshold),
                ..Default::default()
            };
            
            let result = law("analyze", &data, Some(&options))?;
            
            match result {
                LawkitResult::MultiLawAnalysis(analyses) => {
                    let non_compliant: Vec<_> = analyses
                        .iter()
                        .filter(|(_, a)| !a.compliant)
                        .collect();
                    
                    if !non_compliant.is_empty() {
                        println!("Data quality alert: {} laws violated", 
                                 non_compliant.len());
                    }
                }
                _ => {}
            }
        }
    }
}
```

#### Test Data Generation

```rust
use lawkit_core::{law, LawkitResult};
use serde_json::json;

fn generate_benford_data(count: usize, seed: u64) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let config = json!({
        "law": "benford",
        "count": count,
        "seed": seed
    });
    
    let result = law("generate", &config, None)?;
    
    match result {
        LawkitResult::GeneratedData(values) => {
            Ok(values.into_iter()
                .filter_map(|v| v.as_f64())
                .collect())
        }
        _ => Err("Unexpected result type".into())
    }
}
```

## Error Handling

### Error Types

The library uses `anyhow::Error` for error handling:

```rust
use lawkit_core::{law, LawkitOptions};
use anyhow::Result;

fn handle_analysis_errors(data: &Value) -> Result<()> {
    match law("benford", data, None) {
        Ok(result) => {
            println!("Analysis completed successfully");
        }
        Err(e) => {
            eprintln!("Analysis error: {}", e);
            
            // Check for specific error conditions
            if e.to_string().contains("insufficient data") {
                eprintln!("Need at least 30 data points for analysis");
            } else if e.to_string().contains("invalid format") {
                eprintln!("Data must be numeric values");
            }
        }
    }
    
    Ok(())
}
```

## Performance Considerations

### Large Dataset Analysis

```rust
use lawkit_core::{law, LawkitOptions};

fn analyze_large_dataset(data: &Value) -> Result<LawkitResult, Box<dyn std::error::Error>> {
    let options = LawkitOptions {
        // Filter to reduce dataset size
        filter: Some("value > 100 AND value < 1000000".to_string()),
        
        // Use sampling for very large datasets
        statistical_tests: Some(vec!["chi_square".to_string()]), // Faster test
        
        // Disable detailed reporting for performance
        detailed_report: Some(false),
        include_metadata: Some(false),
        
        ..Default::default()
    };
    
    Ok(law("benford", data, Some(&options))?)
}
```

### Optimization Tips

1. **Pre-filter data** using the `filter` option
2. **Set appropriate min_count** to avoid unnecessary computation
3. **Disable detailed reports** for large-scale analysis
4. **Use specific laws** instead of `analyze` when possible
5. **Cache results** for repeated analyses on the same dataset

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_benford_compliance() {
        // Known Benford-compliant data
        let fibonacci = json!([1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]);
        
        let result = law("benford", &fibonacci, None).unwrap();
        
        match result {
            LawkitResult::BenfordAnalysis(_, data) => {
                assert!(data.sample_size == 12);
                // Fibonacci sequence follows Benford's Law
                assert!(data.compliant);
            }
            _ => panic!("Unexpected result type")
        }
    }
    
    #[test]
    fn test_data_generation() {
        let config = json!({
            "law": "zipf",
            "count": 100,
            "parameters": {"s": 1.2, "n": 50},
            "seed": 42
        });
        
        let result = law("generate", &config, None).unwrap();
        
        match result {
            LawkitResult::GeneratedData(values) => {
                assert_eq!(values.len(), 100);
            }
            _ => panic!("Unexpected result type")
        }
    }
}
```

## Version Compatibility

- **0.2.x**: Current stable version
- **Minimum Rust version**: 1.70.0
- **Dependencies**: See `Cargo.toml` for current versions

## See Also

- [CLI Reference](cli-reference.md) for command-line usage
- [Getting Started Guide](../user-guide/getting-started.md) for basic concepts
- [Unified API Reference](../bindings/unified-api.md) for language bindings
- [Advanced Analysis Guide](../guides/advanced-analysis.md) for complex use cases