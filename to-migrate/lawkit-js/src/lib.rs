use lawkit_core::{
    law as core_law, LawkitOptions, LawkitResult, LawkitSpecificOptions, OutputFormat,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use regex::Regex;

#[napi(object)]
pub struct JsLawkitOptions {
    /// Regex pattern for keys to ignore
    pub ignore_keys_regex: Option<String>,

    /// Only show differences in paths containing this string
    pub path_filter: Option<String>,

    /// Output format
    pub output_format: Option<String>,

    /// Show detailed analysis information
    pub show_details: Option<bool>,

    /// Show recommendations in output
    pub show_recommendations: Option<bool>,

    /// Enable memory optimization for large datasets
    pub use_memory_optimization: Option<bool>,

    /// Batch size for memory optimization
    pub batch_size: Option<u32>,

    // lawkit-specific options
    /// Risk threshold level: "low", "medium", "high"
    pub risk_threshold: Option<String>,

    /// Confidence level for statistical tests
    pub confidence_level: Option<f64>,

    /// Analysis threshold value
    pub analysis_threshold: Option<f64>,

    /// Statistical significance level
    pub significance_level: Option<f64>,

    /// Minimum sample size required
    pub min_sample_size: Option<u32>,

    /// Enable outlier detection
    pub enable_outlier_detection: Option<bool>,

    /// Digits to analyze for Benford's law: "first", "second", "both"
    pub benford_digits: Option<String>,

    /// Benford analysis base (default 10)
    pub benford_base: Option<u32>,

    /// Pareto ratio (default 0.8 for 80/20 rule)
    pub pareto_ratio: Option<f64>,

    /// Maximum categories for Pareto analysis
    pub pareto_category_limit: Option<u32>,

    /// Maximum rank limit for Zipf analysis
    pub zipf_rank_limit: Option<u32>,

    /// Frequency cutoff for Zipf analysis
    pub zipf_frequency_cutoff: Option<f64>,

    /// Count for data generation
    pub generate_count: Option<u32>,

    /// Minimum range for data generation
    pub generate_range_min: Option<f64>,

    /// Maximum range for data generation
    pub generate_range_max: Option<f64>,

    /// Seed for random data generation
    pub generate_seed: Option<String>,

    /// Enable Japanese numeral support
    pub enable_japanese_numerals: Option<bool>,

    /// Enable international numeral support
    pub enable_international_numerals: Option<bool>,

    /// Enable parallel processing
    pub enable_parallel_processing: Option<bool>,

    /// Memory limit in MB
    pub memory_limit_mb: Option<u32>,
}

#[napi(object)]
pub struct JsLawkitResult {
    /// Type of analysis result
    pub result_type: String,

    /// Path to the analyzed data
    pub path: String,

    /// Analysis-specific data fields
    pub observed_distribution: Option<Vec<f64>>,
    pub expected_distribution: Option<Vec<f64>>,
    pub chi_square: Option<f64>,
    pub p_value: Option<f64>,
    pub mad: Option<f64>,
    pub risk_level: Option<String>,
    pub total_numbers: Option<u32>,
    pub analysis_summary: Option<String>,

    // Pareto-specific fields
    pub top_20_percent_contribution: Option<f64>,
    pub pareto_ratio: Option<f64>,
    pub concentration_index: Option<f64>,
    pub total_items: Option<u32>,

    // Zipf-specific fields
    pub zipf_coefficient: Option<f64>,
    pub correlation_coefficient: Option<f64>,
    pub deviation_score: Option<f64>,

    // Normal distribution fields
    pub mean: Option<f64>,
    pub std_dev: Option<f64>,
    pub skewness: Option<f64>,
    pub kurtosis: Option<f64>,
    pub normality_test_p: Option<f64>,

    // Poisson distribution fields
    pub lambda: Option<f64>,
    pub variance_ratio: Option<f64>,
    pub poisson_test_p: Option<f64>,
    pub total_events: Option<u32>,

    // Integration analysis fields
    pub laws_analyzed: Option<Vec<String>>,
    pub overall_risk: Option<String>,
    pub conflicting_results: Option<Vec<String>>,
    pub recommendations: Option<Vec<String>>,

    // Validation fields
    pub validation_passed: Option<bool>,
    pub issues_found: Option<Vec<String>>,
    pub data_quality_score: Option<f64>,

    // Diagnostic fields
    pub diagnostic_type: Option<String>,
    pub findings: Option<Vec<String>>,
    pub confidence_level: Option<f64>,

    // Generated data fields
    pub data_type: Option<String>,
    pub count: Option<u32>,
    pub parameters: Option<serde_json::Value>,
    pub sample_data: Option<Vec<f64>>,
}

/// Unified law analysis function for JavaScript/Node.js
///
/// Perform statistical law analysis on JavaScript data using various statistical laws.
///
/// # Arguments
///
/// * `subcommand` - The analysis subcommand ("benf", "pareto", "zipf", "normal", "poisson", "analyze", "validate", "diagnose", "generate")
/// * `data_or_config` - The data to analyze or configuration for generation
/// * `options` - Optional configuration object
///
/// # Returns
///
/// Array of analysis result objects specific to the statistical law used
///
/// # Example
///
/// ```javascript
/// const { law } = require('lawkit-js');
///
/// // Benford's law analysis
/// const data = [123, 456, 789, 1234, 5678];
/// const result = law("benf", data);
/// console.log(result); // [{ type: 'BenfordAnalysis', risk_level: 'LOW', ... }]
///
/// // Pareto analysis
/// const values = [100, 200, 300, 1000, 2000];
/// const result = law("pareto", values);
/// console.log(result); // [{ type: 'ParetoAnalysis', pareto_ratio: 0.8, ... }]
/// ```
#[napi]
pub fn law(
    subcommand: String,
    data_or_config: serde_json::Value,
    options: Option<JsLawkitOptions>,
) -> Result<Vec<JsLawkitResult>> {
    // Convert options
    let rust_options = options.map(build_lawkit_options).transpose()?;

    // Perform law analysis
    let results = core_law(&subcommand, &data_or_config, rust_options.as_ref())
        .map_err(|e| Error::new(Status::GenericFailure, format!("Law analysis error: {e}")))?;

    // Convert results to JavaScript objects
    let js_results = results
        .into_iter()
        .map(convert_lawkit_result)
        .collect::<Result<Vec<_>>>()?;

    Ok(js_results)
}

// Helper functions

fn build_lawkit_options(js_options: JsLawkitOptions) -> Result<LawkitOptions> {
    let mut options = LawkitOptions::default();

    // Core options
    if let Some(ignore_keys_regex) = js_options.ignore_keys_regex {
        let regex = Regex::new(&ignore_keys_regex)
            .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid regex: {e}")))?;
        options.ignore_keys_regex = Some(regex);
    }

    if let Some(path_filter) = js_options.path_filter {
        options.path_filter = Some(path_filter);
    }

    if let Some(output_format) = js_options.output_format {
        let format = OutputFormat::parse_format(&output_format)
            .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid output format: {e}")))?;
        options.output_format = Some(format);
    }

    if let Some(show_details) = js_options.show_details {
        options.show_details = Some(show_details);
    }

    if let Some(show_recommendations) = js_options.show_recommendations {
        options.show_recommendations = Some(show_recommendations);
    }

    if let Some(use_memory_optimization) = js_options.use_memory_optimization {
        options.use_memory_optimization = Some(use_memory_optimization);
    }

    if let Some(batch_size) = js_options.batch_size {
        options.batch_size = Some(batch_size as usize);
    }

    // lawkit-specific options
    let mut lawkit_options = LawkitSpecificOptions::default();
    let mut has_lawkit_options = false;

    if let Some(risk_threshold) = js_options.risk_threshold {
        lawkit_options.risk_threshold = Some(risk_threshold);
        has_lawkit_options = true;
    }

    if let Some(confidence_level) = js_options.confidence_level {
        lawkit_options.confidence_level = Some(confidence_level);
        has_lawkit_options = true;
    }

    if let Some(analysis_threshold) = js_options.analysis_threshold {
        lawkit_options.analysis_threshold = Some(analysis_threshold);
        has_lawkit_options = true;
    }

    if let Some(significance_level) = js_options.significance_level {
        lawkit_options.significance_level = Some(significance_level);
        has_lawkit_options = true;
    }

    if let Some(min_sample_size) = js_options.min_sample_size {
        lawkit_options.min_sample_size = Some(min_sample_size as usize);
        has_lawkit_options = true;
    }

    if let Some(enable_outlier_detection) = js_options.enable_outlier_detection {
        lawkit_options.enable_outlier_detection = Some(enable_outlier_detection);
        has_lawkit_options = true;
    }

    if let Some(benford_digits) = js_options.benford_digits {
        lawkit_options.benford_digits = Some(benford_digits);
        has_lawkit_options = true;
    }

    if let Some(benford_base) = js_options.benford_base {
        lawkit_options.benford_base = Some(benford_base);
        has_lawkit_options = true;
    }

    if let Some(pareto_ratio) = js_options.pareto_ratio {
        lawkit_options.pareto_ratio = Some(pareto_ratio);
        has_lawkit_options = true;
    }

    if let Some(pareto_category_limit) = js_options.pareto_category_limit {
        lawkit_options.pareto_category_limit = Some(pareto_category_limit as usize);
        has_lawkit_options = true;
    }

    if let Some(zipf_rank_limit) = js_options.zipf_rank_limit {
        lawkit_options.zipf_rank_limit = Some(zipf_rank_limit as usize);
        has_lawkit_options = true;
    }

    if let Some(zipf_frequency_cutoff) = js_options.zipf_frequency_cutoff {
        lawkit_options.zipf_frequency_cutoff = Some(zipf_frequency_cutoff);
        has_lawkit_options = true;
    }

    if let Some(generate_count) = js_options.generate_count {
        lawkit_options.generate_count = Some(generate_count as usize);
        has_lawkit_options = true;
    }

    if let Some(generate_range_min) = js_options.generate_range_min {
        lawkit_options.generate_range_min = Some(generate_range_min);
        has_lawkit_options = true;
    }

    if let Some(generate_range_max) = js_options.generate_range_max {
        lawkit_options.generate_range_max = Some(generate_range_max);
        has_lawkit_options = true;
    }

    if let Some(generate_seed) = js_options.generate_seed {
        if let Ok(seed) = generate_seed.parse::<u64>() {
            lawkit_options.generate_seed = Some(seed);
            has_lawkit_options = true;
        }
    }

    if let Some(enable_japanese_numerals) = js_options.enable_japanese_numerals {
        lawkit_options.enable_japanese_numerals = Some(enable_japanese_numerals);
        has_lawkit_options = true;
    }

    if let Some(enable_international_numerals) = js_options.enable_international_numerals {
        lawkit_options.enable_international_numerals = Some(enable_international_numerals);
        has_lawkit_options = true;
    }

    if let Some(enable_parallel_processing) = js_options.enable_parallel_processing {
        lawkit_options.enable_parallel_processing = Some(enable_parallel_processing);
        has_lawkit_options = true;
    }

    if let Some(memory_limit_mb) = js_options.memory_limit_mb {
        lawkit_options.memory_limit_mb = Some(memory_limit_mb as usize);
        has_lawkit_options = true;
    }

    if has_lawkit_options {
        options.lawkit_options = Some(lawkit_options);
    }

    Ok(options)
}

fn convert_lawkit_result(result: LawkitResult) -> Result<JsLawkitResult> {
    let mut js_result = JsLawkitResult {
        result_type: String::new(),
        path: String::new(),
        observed_distribution: None,
        expected_distribution: None,
        chi_square: None,
        p_value: None,
        mad: None,
        risk_level: None,
        total_numbers: None,
        analysis_summary: None,
        top_20_percent_contribution: None,
        pareto_ratio: None,
        concentration_index: None,
        total_items: None,
        zipf_coefficient: None,
        correlation_coefficient: None,
        deviation_score: None,
        mean: None,
        std_dev: None,
        skewness: None,
        kurtosis: None,
        normality_test_p: None,
        lambda: None,
        variance_ratio: None,
        poisson_test_p: None,
        total_events: None,
        laws_analyzed: None,
        overall_risk: None,
        conflicting_results: None,
        recommendations: None,
        validation_passed: None,
        issues_found: None,
        data_quality_score: None,
        diagnostic_type: None,
        findings: None,
        confidence_level: None,
        data_type: None,
        count: None,
        parameters: None,
        sample_data: None,
    };

    match result {
        LawkitResult::BenfordAnalysis(path, data) => {
            js_result.result_type = "BenfordAnalysis".to_string();
            js_result.path = path;
            js_result.observed_distribution = Some(data.observed_distribution.to_vec());
            js_result.expected_distribution = Some(data.expected_distribution.to_vec());
            js_result.chi_square = Some(data.chi_square);
            js_result.p_value = Some(data.p_value);
            js_result.mad = Some(data.mad);
            js_result.risk_level = Some(data.risk_level);
            js_result.total_numbers = Some(data.total_numbers as u32);
            js_result.analysis_summary = Some(data.analysis_summary);
        }
        LawkitResult::ParetoAnalysis(path, data) => {
            js_result.result_type = "ParetoAnalysis".to_string();
            js_result.path = path;
            js_result.top_20_percent_contribution = Some(data.top_20_percent_contribution);
            js_result.pareto_ratio = Some(data.pareto_ratio);
            js_result.concentration_index = Some(data.concentration_index);
            js_result.risk_level = Some(data.risk_level);
            js_result.total_items = Some(data.total_items as u32);
            js_result.analysis_summary = Some(data.analysis_summary);
        }
        LawkitResult::ZipfAnalysis(path, data) => {
            js_result.result_type = "ZipfAnalysis".to_string();
            js_result.path = path;
            js_result.zipf_coefficient = Some(data.zipf_coefficient);
            js_result.correlation_coefficient = Some(data.correlation_coefficient);
            js_result.deviation_score = Some(data.deviation_score);
            js_result.risk_level = Some(data.risk_level);
            js_result.total_items = Some(data.total_items as u32);
            js_result.analysis_summary = Some(data.analysis_summary);
        }
        LawkitResult::NormalAnalysis(path, data) => {
            js_result.result_type = "NormalAnalysis".to_string();
            js_result.path = path;
            js_result.mean = Some(data.mean);
            js_result.std_dev = Some(data.std_dev);
            js_result.skewness = Some(data.skewness);
            js_result.kurtosis = Some(data.kurtosis);
            js_result.normality_test_p = Some(data.normality_test_p);
            js_result.risk_level = Some(data.risk_level);
            js_result.total_numbers = Some(data.total_numbers as u32);
            js_result.analysis_summary = Some(data.analysis_summary);
        }
        LawkitResult::PoissonAnalysis(path, data) => {
            js_result.result_type = "PoissonAnalysis".to_string();
            js_result.path = path;
            js_result.lambda = Some(data.lambda);
            js_result.variance_ratio = Some(data.variance_ratio);
            js_result.poisson_test_p = Some(data.poisson_test_p);
            js_result.risk_level = Some(data.risk_level);
            js_result.total_events = Some(data.total_events as u32);
            js_result.analysis_summary = Some(data.analysis_summary);
        }
        LawkitResult::IntegrationAnalysis(path, data) => {
            js_result.result_type = "IntegrationAnalysis".to_string();
            js_result.path = path;
            js_result.laws_analyzed = Some(data.laws_analyzed);
            js_result.overall_risk = Some(data.overall_risk);
            js_result.conflicting_results = Some(data.conflicting_results);
            js_result.recommendations = Some(data.recommendations);
            js_result.analysis_summary = Some(data.analysis_summary);
        }
        LawkitResult::ValidationResult(path, data) => {
            js_result.result_type = "ValidationResult".to_string();
            js_result.path = path;
            js_result.validation_passed = Some(data.validation_passed);
            js_result.issues_found = Some(data.issues_found);
            js_result.data_quality_score = Some(data.data_quality_score);
            js_result.analysis_summary = Some(data.analysis_summary);
        }
        LawkitResult::DiagnosticResult(path, data) => {
            js_result.result_type = "DiagnosticResult".to_string();
            js_result.path = path;
            js_result.diagnostic_type = Some(data.diagnostic_type);
            js_result.findings = Some(data.findings);
            js_result.confidence_level = Some(data.confidence_level);
            js_result.analysis_summary = Some(data.analysis_summary);
        }
        LawkitResult::GeneratedData(path, data) => {
            js_result.result_type = "GeneratedData".to_string();
            js_result.path = path;
            js_result.data_type = Some(data.data_type);
            js_result.count = Some(data.count as u32);
            js_result.parameters =
                Some(serde_json::to_value(data.parameters).unwrap_or(serde_json::Value::Null));
            js_result.sample_data = Some(data.sample_data);
        }
    }

    Ok(js_result)
}
