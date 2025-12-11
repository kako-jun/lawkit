use anyhow::{anyhow, Result};
use serde::Serialize;
use serde_json::Value;

use crate::OutputFormat;

// ============================================================================
// UTILITY FUNCTIONS - FOR INTERNAL USE ONLY
// ============================================================================
// These functions are public only for CLI and language bindings.
// External users should use the main law() function.

/// Format output to string - FOR INTERNAL USE ONLY
pub fn format_output<T: Serialize>(results: &[T], format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Json => serde_json::to_string_pretty(results)
            .map_err(|e| anyhow!("JSON serialization error: {}", e)),
        OutputFormat::Yaml => {
            serde_yaml::to_string(results).map_err(|e| anyhow!("YAML serialization error: {}", e))
        }
        OutputFormat::Lawkit => {
            let mut output = String::new();
            for result in results {
                let json = serde_json::to_string(result)?;
                output.push_str(&json);
                output.push('\n');
            }
            Ok(output)
        }
        OutputFormat::Csv => {
            // Simple CSV output
            let mut output = String::new();
            output.push_str("type,summary\n");
            for result in results {
                let json = serde_json::to_string(result)?;
                let escaped_json = json.replace("\"", "\"\"");
                output.push_str(&format!("result,\"{escaped_json}\"\n"));
            }
            Ok(output)
        }
        OutputFormat::Text => {
            let mut output = String::new();
            for result in results {
                let json = serde_json::to_string_pretty(result)?;
                output.push_str(&json);
                output.push_str("\n\n");
            }
            Ok(output)
        }
    }
}

// Helper functions
pub(crate) fn extract_numbers_from_value(value: &Value) -> Result<Vec<f64>> {
    let mut numbers = Vec::new();
    extract_numbers_recursive(value, &mut numbers);
    Ok(numbers)
}

fn extract_numbers_recursive(value: &Value, numbers: &mut Vec<f64>) {
    match value {
        Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                numbers.push(f);
            }
        }
        Value::String(s) => {
            // Try to parse as number
            if let Ok(f) = s.parse::<f64>() {
                numbers.push(f);
            }
        }
        Value::Array(arr) => {
            for item in arr {
                extract_numbers_recursive(item, numbers);
            }
        }
        Value::Object(obj) => {
            for value in obj.values() {
                extract_numbers_recursive(value, numbers);
            }
        }
        _ => {} // Ignore null, bool
    }
}

pub(crate) fn get_first_digit(number: f64) -> Option<u8> {
    if number <= 0.0 {
        return None;
    }

    let mut num = number;
    while num >= 10.0 {
        num /= 10.0;
    }
    while num < 1.0 {
        num *= 10.0;
    }

    Some(num.floor() as u8)
}

pub(crate) fn calculate_chi_square(observed: &[f64], expected: &[f64]) -> f64 {
    observed
        .iter()
        .zip(expected.iter())
        .map(|(obs, exp)| (obs - exp).powi(2) / exp)
        .sum()
}

pub(crate) fn calculate_p_value(chi_square: f64, degrees_of_freedom: i32) -> f64 {
    // P-value approximation based on chi-square critical values
    // For more accuracy, use a proper chi-square distribution function
    if chi_square <= 0.0 {
        return 1.0;
    }

    match degrees_of_freedom {
        8 => {
            // Critical values for df=8: χ²(0.01)=20.09, χ²(0.05)=15.51, χ²(0.10)=13.36
            if chi_square >= 20.09 {
                0.01 // p <= 0.01
            } else if chi_square >= 15.51 {
                0.05 // p <= 0.05
            } else if chi_square >= 13.36 {
                0.10 // p <= 0.10
            } else {
                0.50 // p > 0.10
            }
        }
        _ => {
            // Fallback approximation
            if chi_square >= 20.0 {
                0.01
            } else if chi_square >= 15.0 {
                0.05
            } else if chi_square >= 12.0 {
                0.10
            } else {
                0.50
            }
        }
    }
}

pub(crate) fn calculate_mad(observed: &[f64], expected: &[f64]) -> f64 {
    observed
        .iter()
        .zip(expected.iter())
        .map(|(obs, exp)| (obs - exp).abs())
        .sum::<f64>()
        / observed.len() as f64
}

pub(crate) fn calculate_zipf_coefficient(_ranks: &[f64], _frequencies: &[f64]) -> f64 {
    // Simplified Zipf coefficient calculation
    // In a real implementation, use linear regression on log-log plot
    -1.0 // Default Zipf coefficient
}

pub(crate) fn calculate_correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.is_empty() {
        return 0.0;
    }

    let mean_x = x.iter().sum::<f64>() / x.len() as f64;
    let mean_y = y.iter().sum::<f64>() / y.len() as f64;

    let numerator: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
        .sum();

    let denom_x: f64 = x.iter().map(|xi| (xi - mean_x).powi(2)).sum();
    let denom_y: f64 = y.iter().map(|yi| (yi - mean_y).powi(2)).sum();

    if denom_x == 0.0 || denom_y == 0.0 {
        0.0
    } else {
        numerator / (denom_x * denom_y).sqrt()
    }
}

pub(crate) fn calculate_skewness(numbers: &[f64], mean: f64, std_dev: f64) -> f64 {
    if std_dev == 0.0 {
        return 0.0;
    }

    let n = numbers.len() as f64;
    let sum_cubed = numbers
        .iter()
        .map(|x| ((x - mean) / std_dev).powi(3))
        .sum::<f64>();

    sum_cubed / n
}

pub(crate) fn calculate_kurtosis(numbers: &[f64], mean: f64, std_dev: f64) -> f64 {
    if std_dev == 0.0 {
        return 0.0;
    }

    let n = numbers.len() as f64;
    let sum_fourth = numbers
        .iter()
        .map(|x| ((x - mean) / std_dev).powi(4))
        .sum::<f64>();

    (sum_fourth / n) - 3.0 // Excess kurtosis
}

pub(crate) fn calculate_normality_p_value(numbers: &[f64]) -> f64 {
    // Simplified normality test
    // In a real implementation, use Shapiro-Wilk or similar
    if numbers.len() < 3 {
        return 0.0;
    }

    let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
    let variance = numbers.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / numbers.len() as f64;
    let std_dev = variance.sqrt();

    let skewness = calculate_skewness(numbers, mean, std_dev);
    let kurtosis = calculate_kurtosis(numbers, mean, std_dev);

    // Simple heuristic
    if skewness.abs() < 0.5 && kurtosis.abs() < 1.0 {
        0.8 // Likely normal
    } else if skewness.abs() > 2.0 || kurtosis.abs() > 3.0 {
        0.01 // Clearly non-normal
    } else {
        0.3 // Moderate deviation
    }
}

pub(crate) fn calculate_poisson_p_value(variance_ratio: f64, _sample_size: usize) -> f64 {
    // Simplified Poisson test
    let deviation = (variance_ratio - 1.0).abs();

    if deviation < 0.1 {
        0.8 // Likely Poisson
    } else if deviation > 0.5 {
        0.01 // Clearly non-Poisson
    } else {
        0.3 // Moderate deviation
    }
}

pub(crate) fn calculate_percentile(numbers: &[f64], percentile: f64) -> f64 {
    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let index = (percentile * (sorted.len() - 1) as f64).round() as usize;
    sorted[index.min(sorted.len() - 1)]
}

pub(crate) fn generate_recommendations(laws: &[String], risks: &[String]) -> Vec<String> {
    let mut recommendations = Vec::new();

    let high_risk_count = risks.iter().filter(|&r| r == "HIGH").count();

    if high_risk_count == 0 {
        recommendations.push("Data appears to follow expected statistical patterns".to_string());
    } else if high_risk_count == 1 {
        recommendations
            .push("One statistical law shows high risk - investigate specific area".to_string());
    } else {
        recommendations.push(
            "Multiple statistical laws show high risk - comprehensive review needed".to_string(),
        );
    }

    if laws.contains(&"Benford".to_string()) {
        recommendations.push("Consider fraud detection analysis if financial data".to_string());
    }

    if laws.contains(&"Normal".to_string()) {
        recommendations.push("Check for outliers and data transformation needs".to_string());
    }

    recommendations
}

// Data generation functions
pub(crate) fn generate_benford_data(count: usize) -> Vec<f64> {
    use std::f64::consts::E;

    let mut data = Vec::new();
    for i in 1..=count {
        // Generate numbers that follow Benford's law approximately
        let base = E.powf(((i as f64 * 7.0) % 10.0) + 1.0);
        let multiplier = 10.0_f64.powf((i % 5) as f64);
        data.push(base * multiplier);
    }
    data
}

pub(crate) fn generate_normal_data(count: usize, mean: f64, std_dev: f64) -> Vec<f64> {
    let mut data = Vec::new();
    for i in 0..count {
        // Simple Box-Muller transformation approximation
        let u1 = (i as f64 + 1.0) / (count as f64 + 1.0);
        let u2 = ((i * 7 + 3) % count) as f64 / count as f64;

        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        data.push(mean + std_dev * z);
    }
    data
}

pub(crate) fn generate_poisson_data(count: usize, lambda: f64) -> Vec<f64> {
    let mut data = Vec::new();
    for i in 0..count {
        // Simple Poisson approximation
        let u = (i as f64 + 0.5) / (count as f64 + 1.0);
        let k = (-lambda * u.ln()).floor();
        data.push(k.max(0.0));
    }
    data
}

pub(crate) fn generate_pareto_data(count: usize, alpha: f64) -> Vec<f64> {
    // Generate Pareto distribution: P(X > x) = (x_m / x)^alpha
    // Using inverse transform: X = x_m / U^(1/alpha) where U ~ Uniform(0,1)
    let x_m = 1.0; // minimum value
    let mut data = Vec::new();
    for i in 0..count {
        let u = (i as f64 + 1.0) / (count as f64 + 1.0);
        let value = x_m / u.powf(1.0 / alpha);
        data.push(value);
    }
    data
}

pub(crate) fn generate_zipf_data(count: usize, s: f64) -> Vec<f64> {
    // Generate Zipf distribution: f(k) = 1 / (k^s * H_N,s)
    // Returns frequency counts for ranks 1 to count
    let mut data = Vec::new();
    let base_freq = 1000.0; // Base frequency for rank 1
    for k in 1..=count {
        let freq = base_freq / (k as f64).powf(s);
        data.push(freq.round());
    }
    data
}
