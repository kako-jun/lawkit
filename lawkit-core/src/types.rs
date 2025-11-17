// ============================================================================
// UNIFIED API - Core Types
// ============================================================================
// Type definitions for lawkit's unified API
// Extracted from lib.rs for better organization (following diffx reboot structure)

use anyhow::{anyhow, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize)]
pub enum LawkitResult {
    // Benford's law results
    BenfordAnalysis(String, BenfordData),

    // Pareto principle results
    ParetoAnalysis(String, ParetoData),

    // Zipf's law results
    ZipfAnalysis(String, ZipfData),

    // Normal distribution results
    NormalAnalysis(String, NormalData),

    // Poisson distribution results
    PoissonAnalysis(String, PoissonData),

    // Integration analysis results
    IntegrationAnalysis(String, IntegrationData),

    // Validation results
    ValidationResult(String, ValidationData),

    // Diagnostic results
    DiagnosticResult(String, DiagnosticData),

    // Generated data
    GeneratedData(String, GeneratedDataInfo),
}

#[derive(Debug, PartialEq, Serialize)]
pub struct BenfordData {
    pub observed_distribution: [f64; 9],
    pub expected_distribution: [f64; 9],
    pub chi_square: f64,
    pub p_value: f64,
    pub mad: f64,
    pub risk_level: String,
    pub total_numbers: usize,
    pub analysis_summary: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ParetoData {
    pub top_20_percent_contribution: f64,
    pub pareto_ratio: f64,
    pub concentration_index: f64,
    pub risk_level: String,
    pub total_items: usize,
    pub analysis_summary: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ZipfData {
    pub zipf_coefficient: f64,
    pub correlation_coefficient: f64,
    pub deviation_score: f64,
    pub risk_level: String,
    pub total_items: usize,
    pub analysis_summary: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct NormalData {
    pub mean: f64,
    pub std_dev: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    pub normality_test_p: f64,
    pub risk_level: String,
    pub total_numbers: usize,
    pub analysis_summary: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct PoissonData {
    pub lambda: f64,
    pub variance_ratio: f64,
    pub poisson_test_p: f64,
    pub risk_level: String,
    pub total_events: usize,
    pub analysis_summary: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct IntegrationData {
    pub laws_analyzed: Vec<String>,
    pub overall_risk: String,
    pub conflicting_results: Vec<String>,
    pub recommendations: Vec<String>,
    pub analysis_summary: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ValidationData {
    pub validation_passed: bool,
    pub issues_found: Vec<String>,
    pub data_quality_score: f64,
    pub analysis_summary: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct DiagnosticData {
    pub diagnostic_type: String,
    pub findings: Vec<String>,
    pub confidence_level: f64,
    pub analysis_summary: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct GeneratedDataInfo {
    pub data_type: String,
    pub count: usize,
    pub parameters: HashMap<String, f64>,
    pub sample_data: Vec<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum OutputFormat {
    #[serde(rename = "lawkit")]
    #[default]
    Lawkit,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "yaml")]
    Yaml,
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "text")]
    Text,
}

impl OutputFormat {
    pub fn value_variants() -> &'static [Self] {
        &[Self::Lawkit, Self::Json, Self::Yaml, Self::Csv, Self::Text]
    }

    pub fn parse_format(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "lawkit" => Ok(Self::Lawkit),
            "json" => Ok(Self::Json),
            "yaml" | "yml" => Ok(Self::Yaml),
            "csv" => Ok(Self::Csv),
            "text" | "txt" => Ok(Self::Text),
            _ => Err(anyhow!("Invalid output format: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LawkitSpecificOptions {
    // Risk assessment options
    pub risk_threshold: Option<String>, // "low", "medium", "high"
    pub confidence_level: Option<f64>,
    pub analysis_threshold: Option<f64>,

    // Statistical analysis options
    pub significance_level: Option<f64>,
    pub min_sample_size: Option<usize>,
    pub enable_outlier_detection: Option<bool>,

    // Benford-specific options
    pub benford_digits: Option<String>, // "first", "second", "both"
    pub benford_base: Option<u32>,      // default 10

    // Pareto-specific options
    pub pareto_ratio: Option<f64>, // default 0.8 (80/20)
    pub pareto_category_limit: Option<usize>,

    // Zipf-specific options
    pub zipf_rank_limit: Option<usize>,
    pub zipf_frequency_cutoff: Option<f64>,

    // Generation options (for generate subcommand)
    pub generate_count: Option<usize>,
    pub generate_range_min: Option<f64>,
    pub generate_range_max: Option<f64>,
    pub generate_seed: Option<u64>,

    // International support
    pub enable_japanese_numerals: Option<bool>,
    pub enable_international_numerals: Option<bool>,

    // Performance options
    pub enable_parallel_processing: Option<bool>,
    pub memory_limit_mb: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct LawkitOptions {
    // Core options
    pub ignore_keys_regex: Option<Regex>,
    pub path_filter: Option<String>,

    // Output control
    pub output_format: Option<OutputFormat>,
    pub show_details: Option<bool>,
    pub show_recommendations: Option<bool>,

    // Memory optimization
    pub use_memory_optimization: Option<bool>,
    pub batch_size: Option<usize>,

    // lawkit-specific options
    pub lawkit_options: Option<LawkitSpecificOptions>,
}
