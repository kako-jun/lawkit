use anyhow::{anyhow, Result};
use serde_json::Value;
use std::collections::HashMap;

// ============================================================================
// TYPE RE-EXPORTS - From types module
// ============================================================================

// Re-export all types from types module
pub use crate::types::*;


// ============================================================================
// UNIFIED API - Main Function
// ============================================================================

/// Unified law analysis function for lawkit
///
/// This is the single entry point for all lawkit functionality.
/// The first argument specifies the subcommand/analysis type.
pub fn law(
    subcommand: &str,
    data_or_config: &Value,
    options: Option<&LawkitOptions>,
) -> Result<Vec<LawkitResult>> {
    let default_options = LawkitOptions::default();
    let opts = options.unwrap_or(&default_options);

    match subcommand {
        "benf" | "benford" => analyze_benford_law(data_or_config, opts),
        "pareto" => analyze_pareto_principle(data_or_config, opts),
        "zipf" => analyze_zipf_law(data_or_config, opts),
        "normal" => analyze_normal_distribution(data_or_config, opts),
        "poisson" => analyze_poisson_distribution(data_or_config, opts),
        "analyze" => analyze_all_laws(data_or_config, opts),
        "validate" => validate_data(data_or_config, opts),
        "diagnose" => diagnose_data(data_or_config, opts),
        "generate" => generate_sample_data(data_or_config, opts),
        _ => Err(anyhow!("Unknown subcommand: {}", subcommand)),
    }
}

fn analyze_benford_law(data: &Value, _options: &LawkitOptions) -> Result<Vec<LawkitResult>> {
    let numbers = extract_numbers_from_value(data)?;

    if numbers.is_empty() {
        return Err(anyhow!("No valid numbers found in input data"));
    }

    // Calculate first digit distribution
    let mut observed = [0.0; 9];
    let mut total = 0;

    for &num in &numbers {
        if let Some(digit) = get_first_digit(num.abs()) {
            observed[digit as usize - 1] += 1.0;
            total += 1;
        }
    }

    // Normalize to proportions
    for count in &mut observed {
        *count /= total as f64;
    }

    // Expected Benford distribution
    let expected = [
        (1.0_f64 + 1.0).log10() / 1.0_f64.log10(), // log10(1 + 1/1)
        (1.0_f64 + 1.0 / 2.0).log10(),
        (1.0_f64 + 1.0 / 3.0).log10(),
        (1.0_f64 + 1.0 / 4.0).log10(),
        (1.0_f64 + 1.0 / 5.0).log10(),
        (1.0_f64 + 1.0 / 6.0).log10(),
        (1.0_f64 + 1.0 / 7.0).log10(),
        (1.0_f64 + 1.0 / 8.0).log10(),
        (1.0_f64 + 1.0 / 9.0).log10(),
    ];

    // Calculate chi-square test
    let chi_square = calculate_chi_square(&observed, &expected);
    let p_value = calculate_p_value(chi_square, 8);
    let mad = calculate_mad(&observed, &expected);

    // Determine risk level
    let risk_level = if p_value < 0.05 {
        "HIGH"
    } else if p_value < 0.1 {
        "MEDIUM"
    } else {
        "LOW"
    }
    .to_string();

    let analysis_summary =
        format!("Benford's law analysis: p-value={p_value:.4}, MAD={mad:.4}, risk={risk_level}");

    let benford_data = BenfordData {
        observed_distribution: observed,
        expected_distribution: expected,
        chi_square,
        p_value,
        mad,
        risk_level,
        total_numbers: total,
        analysis_summary,
    };

    Ok(vec![LawkitResult::BenfordAnalysis(
        "benford_analysis".to_string(),
        benford_data,
    )])
}

fn analyze_pareto_principle(data: &Value, _options: &LawkitOptions) -> Result<Vec<LawkitResult>> {
    let numbers = extract_numbers_from_value(data)?;

    if numbers.is_empty() {
        return Err(anyhow!("No valid numbers found in input data"));
    }

    // Sort in descending order
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let total_sum: f64 = sorted_numbers.iter().sum();
    let total_count = sorted_numbers.len();
    let top_20_count = (total_count as f64 * 0.2).ceil() as usize;

    let top_20_sum: f64 = sorted_numbers.iter().take(top_20_count).sum();
    let top_20_percent_contribution = (top_20_sum / total_sum) * 100.0;

    // Calculate Pareto ratio (how much the top 20% contributes)
    let pareto_ratio = top_20_percent_contribution / 80.0; // Ideal is 1.0 (80/20 rule)

    // Calculate concentration index (Gini-like measure)
    let mut cumulative_sum = 0.0;
    let mut concentration_index = 0.0;
    for (i, &value) in sorted_numbers.iter().enumerate() {
        cumulative_sum += value;
        let proportion = (i + 1) as f64 / total_count as f64;
        let cumulative_proportion = cumulative_sum / total_sum;
        concentration_index += (proportion - cumulative_proportion).abs();
    }
    concentration_index /= total_count as f64;

    // Determine risk level
    let risk_level = if top_20_percent_contribution < 60.0 {
        "LOW" // Not following Pareto principle
    } else if top_20_percent_contribution > 95.0 {
        "HIGH" // Extreme concentration
    } else {
        "MEDIUM"
    }
    .to_string();

    let analysis_summary = format!(
        "Pareto analysis: top 20% contributes {top_20_percent_contribution:.1}%, concentration index={concentration_index:.3}, risk={risk_level}"
    );

    let pareto_data = ParetoData {
        top_20_percent_contribution,
        pareto_ratio,
        concentration_index,
        risk_level,
        total_items: total_count,
        analysis_summary,
    };

    Ok(vec![LawkitResult::ParetoAnalysis(
        "pareto_analysis".to_string(),
        pareto_data,
    )])
}

fn analyze_zipf_law(data: &Value, _options: &LawkitOptions) -> Result<Vec<LawkitResult>> {
    let numbers = extract_numbers_from_value(data)?;

    if numbers.is_empty() {
        return Err(anyhow!("No valid numbers found in input data"));
    }

    // Count frequencies and sort by frequency (descending)
    let mut frequency_map: HashMap<String, f64> = HashMap::new();
    for &num in &numbers {
        let key = format!("{num:.6}"); // Use string representation for grouping
        *frequency_map.entry(key).or_insert(0.0) += 1.0;
    }

    let mut frequencies: Vec<f64> = frequency_map.values().cloned().collect();
    frequencies.sort_by(|a, b| b.partial_cmp(a).unwrap());

    if frequencies.len() < 2 {
        return Err(anyhow!("Insufficient unique values for Zipf analysis"));
    }

    // Calculate Zipf coefficient (log-log slope)
    let mut log_ranks: Vec<f64> = Vec::new();
    let mut log_frequencies: Vec<f64> = Vec::new();

    for (rank, &freq) in frequencies.iter().enumerate() {
        if freq > 0.0 {
            log_ranks.push((rank + 1) as f64);
            log_frequencies.push(freq);
        }
    }

    let zipf_coefficient = calculate_zipf_coefficient(&log_ranks, &log_frequencies);
    let correlation = calculate_correlation(&log_ranks, &log_frequencies);
    let deviation_score = (zipf_coefficient - 1.0).abs(); // Ideal Zipf has coefficient = -1

    // Determine risk level
    let risk_level = if deviation_score < 0.2 {
        "LOW" // Close to ideal Zipf
    } else if deviation_score > 0.8 {
        "HIGH" // Far from Zipf distribution
    } else {
        "MEDIUM"
    }
    .to_string();

    let analysis_summary = format!(
        "Zipf analysis: coefficient={zipf_coefficient:.3}, correlation={correlation:.3}, deviation={deviation_score:.3}, risk={risk_level}"
    );

    let zipf_data = ZipfData {
        zipf_coefficient,
        correlation_coefficient: correlation,
        deviation_score,
        risk_level,
        total_items: frequencies.len(),
        analysis_summary,
    };

    Ok(vec![LawkitResult::ZipfAnalysis(
        "zipf_analysis".to_string(),
        zipf_data,
    )])
}

fn analyze_normal_distribution(
    data: &Value,
    _options: &LawkitOptions,
) -> Result<Vec<LawkitResult>> {
    let numbers = extract_numbers_from_value(data)?;

    if numbers.is_empty() {
        return Err(anyhow!("No valid numbers found in input data"));
    }

    if numbers.len() < 3 {
        return Err(anyhow!(
            "Insufficient data points for normal distribution analysis"
        ));
    }

    // Calculate basic statistics
    let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
    let variance =
        numbers.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (numbers.len() - 1) as f64;
    let std_dev = variance.sqrt();

    // Calculate skewness and kurtosis
    let skewness = calculate_skewness(&numbers, mean, std_dev);
    let kurtosis = calculate_kurtosis(&numbers, mean, std_dev);

    // Simple normality test (Shapiro-Wilk approximation)
    let normality_test_p = calculate_normality_p_value(&numbers);

    // Determine risk level
    let risk_level = if normality_test_p > 0.05 && skewness.abs() < 1.0 && kurtosis.abs() < 3.0 {
        "LOW" // Likely normal
    } else if normality_test_p < 0.01 || skewness.abs() > 2.0 || kurtosis.abs() > 5.0 {
        "HIGH" // Clearly non-normal
    } else {
        "MEDIUM"
    }
    .to_string();

    let analysis_summary = format!(
        "Normal distribution analysis: mean={mean:.3}, std={std_dev:.3}, skew={skewness:.3}, kurt={kurtosis:.3}, p={normality_test_p:.4}, risk={risk_level}"
    );

    let normal_data = NormalData {
        mean,
        std_dev,
        skewness,
        kurtosis,
        normality_test_p,
        risk_level,
        total_numbers: numbers.len(),
        analysis_summary,
    };

    Ok(vec![LawkitResult::NormalAnalysis(
        "normal_analysis".to_string(),
        normal_data,
    )])
}

fn analyze_poisson_distribution(
    data: &Value,
    _options: &LawkitOptions,
) -> Result<Vec<LawkitResult>> {
    let numbers = extract_numbers_from_value(data)?;

    if numbers.is_empty() {
        return Err(anyhow!("No valid numbers found in input data"));
    }

    // For Poisson analysis, we need non-negative integer values
    let integers: Vec<u32> = numbers
        .iter()
        .filter_map(|&x| {
            if x >= 0.0 && x.fract() == 0.0 {
                Some(x as u32)
            } else {
                None
            }
        })
        .collect();

    if integers.is_empty() {
        return Err(anyhow!(
            "No valid non-negative integers found for Poisson analysis"
        ));
    }

    // Calculate lambda (mean rate)
    let lambda = integers.iter().sum::<u32>() as f64 / integers.len() as f64;

    // Calculate variance
    let variance = integers
        .iter()
        .map(|&x| (x as f64 - lambda).powi(2))
        .sum::<f64>()
        / integers.len() as f64;

    // For Poisson distribution, variance should equal lambda
    let variance_ratio = variance / lambda;

    // Simple Poisson test (variance-to-mean ratio)
    let poisson_test_p = calculate_poisson_p_value(variance_ratio, integers.len());

    // Determine risk level
    let risk_level = if (variance_ratio - 1.0).abs() < 0.2 && poisson_test_p > 0.05 {
        "LOW" // Likely Poisson
    } else if (variance_ratio - 1.0).abs() > 0.8 || poisson_test_p < 0.01 {
        "HIGH" // Clearly non-Poisson
    } else {
        "MEDIUM"
    }
    .to_string();

    let analysis_summary = format!(
        "Poisson distribution analysis: lambda={lambda:.3}, var/mean={variance_ratio:.3}, p={poisson_test_p:.4}, risk={risk_level}"
    );

    let poisson_data = PoissonData {
        lambda,
        variance_ratio,
        poisson_test_p,
        risk_level,
        total_events: integers.len(),
        analysis_summary,
    };

    Ok(vec![LawkitResult::PoissonAnalysis(
        "poisson_analysis".to_string(),
        poisson_data,
    )])
}

fn analyze_all_laws(data: &Value, _options: &LawkitOptions) -> Result<Vec<LawkitResult>> {
    let mut results = Vec::new();
    let mut laws_analyzed = Vec::new();
    let mut overall_risks = Vec::new();

    // Analyze all applicable laws
    if let Ok(mut benford_results) = analyze_benford_law(data, _options) {
        laws_analyzed.push("Benford".to_string());
        if let Some(LawkitResult::BenfordAnalysis(_, ref benford_data)) = benford_results.first() {
            overall_risks.push(benford_data.risk_level.clone());
        }
        results.append(&mut benford_results);
    }

    if let Ok(mut pareto_results) = analyze_pareto_principle(data, _options) {
        laws_analyzed.push("Pareto".to_string());
        if let Some(LawkitResult::ParetoAnalysis(_, ref pareto_data)) = pareto_results.first() {
            overall_risks.push(pareto_data.risk_level.clone());
        }
        results.append(&mut pareto_results);
    }

    if let Ok(mut zipf_results) = analyze_zipf_law(data, _options) {
        laws_analyzed.push("Zipf".to_string());
        if let Some(LawkitResult::ZipfAnalysis(_, ref zipf_data)) = zipf_results.first() {
            overall_risks.push(zipf_data.risk_level.clone());
        }
        results.append(&mut zipf_results);
    }

    if let Ok(mut normal_results) = analyze_normal_distribution(data, _options) {
        laws_analyzed.push("Normal".to_string());
        if let Some(LawkitResult::NormalAnalysis(_, ref normal_data)) = normal_results.first() {
            overall_risks.push(normal_data.risk_level.clone());
        }
        results.append(&mut normal_results);
    }

    if let Ok(mut poisson_results) = analyze_poisson_distribution(data, _options) {
        laws_analyzed.push("Poisson".to_string());
        if let Some(LawkitResult::PoissonAnalysis(_, ref poisson_data)) = poisson_results.first() {
            overall_risks.push(poisson_data.risk_level.clone());
        }
        results.append(&mut poisson_results);
    }

    // Determine overall risk
    let high_count = overall_risks.iter().filter(|&r| r == "HIGH").count();
    let medium_count = overall_risks.iter().filter(|&r| r == "MEDIUM").count();

    let overall_risk = if high_count > 0 {
        "HIGH"
    } else if medium_count > 0 {
        "MEDIUM"
    } else {
        "LOW"
    }
    .to_string();

    // Generate recommendations
    let recommendations = generate_recommendations(&laws_analyzed, &overall_risks);

    let analysis_summary = format!(
        "Integrated analysis of {} laws completed. Overall risk: {overall_risk}",
        laws_analyzed.len()
    );

    let integration_data = IntegrationData {
        laws_analyzed,
        overall_risk,
        conflicting_results: Vec::new(), // Could be implemented later
        recommendations,
        analysis_summary,
    };

    results.push(LawkitResult::IntegrationAnalysis(
        "integration_analysis".to_string(),
        integration_data,
    ));

    Ok(results)
}

fn validate_data(data: &Value, _options: &LawkitOptions) -> Result<Vec<LawkitResult>> {
    let numbers = extract_numbers_from_value(data)?;

    let mut issues_found = Vec::new();
    let mut validation_passed = true;

    // Check for empty data
    if numbers.is_empty() {
        issues_found.push("No valid numbers found in input data".to_string());
        validation_passed = false;
    }

    // Check for minimum sample size
    if numbers.len() < 10 {
        issues_found.push(format!(
            "Small sample size: {} (recommended: 10+)",
            numbers.len()
        ));
        validation_passed = false;
    }

    // Check for data quality issues
    let infinite_count = numbers.iter().filter(|&&x| x.is_infinite()).count();
    let nan_count = numbers.iter().filter(|&&x| x.is_nan()).count();

    if infinite_count > 0 {
        issues_found.push(format!("Found {infinite_count} infinite values"));
        validation_passed = false;
    }

    if nan_count > 0 {
        issues_found.push(format!("Found {nan_count} NaN values"));
        validation_passed = false;
    }

    // Calculate data quality score
    let total_issues = issues_found.len() as f64;
    let data_quality_score = if numbers.is_empty() {
        0.0
    } else {
        (1.0 - (total_issues / 10.0)).clamp(0.0, 1.0) // Scale to 0-1
    };

    let analysis_summary = if validation_passed {
        "Data validation passed successfully".to_string()
    } else {
        format!("Data validation failed with {} issues", issues_found.len())
    };

    let validation_data = ValidationData {
        validation_passed,
        issues_found,
        data_quality_score,
        analysis_summary,
    };

    Ok(vec![LawkitResult::ValidationResult(
        "validation_result".to_string(),
        validation_data,
    )])
}

fn diagnose_data(data: &Value, _options: &LawkitOptions) -> Result<Vec<LawkitResult>> {
    let numbers = extract_numbers_from_value(data)?;

    if numbers.is_empty() {
        return Err(anyhow!("No valid numbers found for diagnosis"));
    }

    let mut findings = Vec::new();

    // Basic statistics
    let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
    let median = {
        let mut sorted = numbers.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted[sorted.len() / 2]
    };

    findings.push(format!("Sample size: {}", numbers.len()));
    findings.push(format!("Mean: {mean:.3}"));
    findings.push(format!("Median: {median:.3}"));

    // Data range analysis
    let min_val = numbers.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = numbers.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max_val - min_val;

    findings.push(format!(
        "Range: {min_val:.3} to {max_val:.3} (span: {range:.3})"
    ));

    // Distribution shape analysis
    if (mean - median).abs() < 0.1 * mean.abs() {
        findings.push("Distribution appears symmetric".to_string());
    } else if mean > median {
        findings.push("Distribution appears right-skewed".to_string());
    } else {
        findings.push("Distribution appears left-skewed".to_string());
    }

    // Outlier detection
    let q1 = calculate_percentile(&numbers, 0.25);
    let q3 = calculate_percentile(&numbers, 0.75);
    let iqr = q3 - q1;
    let outlier_threshold_low = q1 - 1.5 * iqr;
    let outlier_threshold_high = q3 + 1.5 * iqr;

    let outliers: Vec<f64> = numbers
        .iter()
        .cloned()
        .filter(|&x| x < outlier_threshold_low || x > outlier_threshold_high)
        .collect();

    if !outliers.is_empty() {
        findings.push(format!("Found {} potential outliers", outliers.len()));
    }

    let confidence_level = if numbers.len() >= 100 {
        0.95
    } else if numbers.len() >= 30 {
        0.80
    } else {
        0.60
    };

    let analysis_summary = format!(
        "Diagnostic analysis completed with {} findings (confidence: {:.0}%)",
        findings.len(),
        confidence_level * 100.0
    );

    let diagnostic_data = DiagnosticData {
        diagnostic_type: "General".to_string(),
        findings,
        confidence_level,
        analysis_summary,
    };

    Ok(vec![LawkitResult::DiagnosticResult(
        "diagnostic_result".to_string(),
        diagnostic_data,
    )])
}

fn generate_sample_data(config: &Value, _options: &LawkitOptions) -> Result<Vec<LawkitResult>> {
    // Parse generation configuration
    let data_type = config
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("benford");

    let count = config.get("count").and_then(|v| v.as_u64()).unwrap_or(1000) as usize;

    let mut parameters = HashMap::new();
    let sample_data = match data_type {
        "benford" => {
            parameters.insert("base".to_string(), 10.0);
            generate_benford_data(count)
        }
        "normal" => {
            let mean = config.get("mean").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let std_dev = config
                .get("std_dev")
                .and_then(|v| v.as_f64())
                .unwrap_or(1.0);
            parameters.insert("mean".to_string(), mean);
            parameters.insert("std_dev".to_string(), std_dev);
            generate_normal_data(count, mean, std_dev)
        }
        "poisson" => {
            let lambda = config.get("lambda").and_then(|v| v.as_f64()).unwrap_or(5.0);
            parameters.insert("lambda".to_string(), lambda);
            generate_poisson_data(count, lambda)
        }
        _ => return Err(anyhow!("Unknown data type for generation: {}", data_type)),
    };

    let generated_info = GeneratedDataInfo {
        data_type: data_type.to_string(),
        count,
        parameters,
        sample_data,
    };

    Ok(vec![LawkitResult::GeneratedData(
        "generated_data".to_string(),
        generated_info,
    )])
}

// Import helper functions from modules
use helpers::*;

// Re-export parser and format functions for CLI and language bindings
pub use helpers::format_output;
pub use parsers::{parse_csv, parse_ini, parse_json, parse_toml, parse_xml, parse_yaml};

// ============================================================================
// MODULE DECLARATIONS - For CLI and external access
// ============================================================================

// Type definitions (extracted for better organization - following diffx reboot)
pub mod types;

// Helper functions and parsers (extracted from lib.rs for better organization)
pub mod helpers;
pub mod parsers;

// Temporary diffx-core mock until diffx reboot is complete
pub mod diffx_core_mock;

pub mod common;
pub mod core;
pub mod error;
pub mod generate;
pub mod laws;
