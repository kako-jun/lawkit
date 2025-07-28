use lawkit_core::*;
use serde_json::{json, Value};

mod fixtures;
use fixtures::*;

// ============================================================================
// UNIFIED API TESTS - Core Functionality
// ============================================================================

#[test]
fn test_law_benford_analysis() {
    let data = TestFixtures::benford_compliant_data();

    let results = law("benford", &data, None).unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        LawkitResult::BenfordAnalysis(name, benford_data) => {
            assert_eq!(name, "benford_analysis");
            assert_eq!(benford_data.observed_distribution.len(), 9);
            assert_eq!(benford_data.expected_distribution.len(), 9);
            assert!(benford_data.chi_square >= 0.0);
            assert!(benford_data.p_value >= 0.0 && benford_data.p_value <= 1.0);
            assert!(benford_data.total_numbers > 0);
            assert!(!benford_data.analysis_summary.is_empty());
        }
        _ => panic!("Expected BenfordAnalysis result"),
    }
}

#[test]
fn test_law_pareto_analysis() {
    let data = TestFixtures::pareto_compliant_data();

    let results = law("pareto", &data, None).unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        LawkitResult::ParetoAnalysis(name, pareto_data) => {
            assert_eq!(name, "pareto_analysis");
            assert!(pareto_data.top_20_percent_contribution > 0.0);
            assert!(pareto_data.pareto_ratio > 0.0);
            assert!(pareto_data.concentration_index >= 0.0);
            assert!(pareto_data.total_items > 0);
            assert!(!pareto_data.analysis_summary.is_empty());
            // Should be high contribution for compliant data
            assert!(pareto_data.top_20_percent_contribution > 60.0);
        }
        _ => panic!("Expected ParetoAnalysis result"),
    }
}

#[test]
fn test_law_zipf_analysis() {
    let data = TestFixtures::zipf_compliant_data();

    let results = law("zipf", &data, None).unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        LawkitResult::ZipfAnalysis(name, zipf_data) => {
            assert_eq!(name, "zipf_analysis");
            assert!(zipf_data.zipf_coefficient != 0.0);
            assert!(
                zipf_data.correlation_coefficient >= -1.0
                    && zipf_data.correlation_coefficient <= 1.0
            );
            assert!(zipf_data.deviation_score >= 0.0);
            assert!(zipf_data.total_items > 0);
            assert!(!zipf_data.analysis_summary.is_empty());
        }
        _ => panic!("Expected ZipfAnalysis result"),
    }
}

#[test]
fn test_law_normal_analysis() {
    let data = TestFixtures::normal_distribution_data();

    let results = law("normal", &data, None).unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        LawkitResult::NormalAnalysis(name, normal_data) => {
            assert_eq!(name, "normal_analysis");
            assert!(normal_data.std_dev > 0.0);
            assert!(normal_data.normality_test_p >= 0.0 && normal_data.normality_test_p <= 1.0);
            assert!(normal_data.total_numbers > 0);
            assert!(!normal_data.analysis_summary.is_empty());
        }
        _ => panic!("Expected NormalAnalysis result"),
    }
}

#[test]
fn test_law_poisson_analysis() {
    let data = TestFixtures::poisson_distribution_data();

    let results = law("poisson", &data, None).unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        LawkitResult::PoissonAnalysis(name, poisson_data) => {
            assert_eq!(name, "poisson_analysis");
            assert!(poisson_data.lambda > 0.0);
            assert!(poisson_data.variance_ratio > 0.0);
            assert!(poisson_data.poisson_test_p >= 0.0 && poisson_data.poisson_test_p <= 1.0);
            assert!(poisson_data.total_events > 0);
            assert!(!poisson_data.analysis_summary.is_empty());
        }
        _ => panic!("Expected PoissonAnalysis result"),
    }
}

#[test]
fn test_law_validate_data() {
    let data = TestFixtures::validation_test_data();

    let results = law("validate", &data["valid_dataset"], None).unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        LawkitResult::ValidationResult(name, validation_data) => {
            assert_eq!(name, "validation_result");
            assert!(validation_data.validation_passed);
            assert!(validation_data.data_quality_score > 0.0);
            assert!(!validation_data.analysis_summary.is_empty());
        }
        _ => panic!("Expected ValidationResult"),
    }
}

#[test]
fn test_law_diagnose_data() {
    let data = TestFixtures::diagnostic_test_data();

    let results = law("diagnose", &data["normal_with_outliers"], None).unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        LawkitResult::DiagnosticResult(name, diagnostic_data) => {
            assert_eq!(name, "diagnostic_result");
            assert_eq!(diagnostic_data.diagnostic_type, "General");
            assert!(!diagnostic_data.findings.is_empty());
            assert!(
                diagnostic_data.confidence_level > 0.0 && diagnostic_data.confidence_level <= 1.0
            );
            assert!(!diagnostic_data.analysis_summary.is_empty());
        }
        _ => panic!("Expected DiagnosticResult"),
    }
}

#[test]
fn test_law_generate_data() {
    let config = TestFixtures::generation_configs();

    let results = law("generate", &config["benford_config"], None).unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        LawkitResult::GeneratedData(name, generated_info) => {
            assert_eq!(name, "generated_data");
            assert_eq!(generated_info.data_type, "benford");
            assert_eq!(generated_info.count, 1000);
            assert!(!generated_info.sample_data.is_empty());
            assert!(!generated_info.parameters.is_empty());
        }
        _ => panic!("Expected GeneratedData result"),
    }
}

#[test]
fn test_law_analyze_all() {
    let data = TestFixtures::integration_analysis_data();

    let results = law("analyze", &data, None).unwrap();

    // Should have multiple analysis results plus integration
    assert!(results.len() > 1);

    // Last result should be integration analysis
    let integration_result = results.last().unwrap();
    match integration_result {
        LawkitResult::IntegrationAnalysis(name, integration_data) => {
            assert_eq!(name, "integration_analysis");
            assert!(!integration_data.laws_analyzed.is_empty());
            assert!(!integration_data.overall_risk.is_empty());
            assert!(!integration_data.recommendations.is_empty());
            assert!(!integration_data.analysis_summary.is_empty());
        }
        _ => panic!("Expected IntegrationAnalysis as last result"),
    }
}

#[test]
fn test_law_unknown_subcommand() {
    let data = json!([1, 2, 3]);

    let result = law("unknown", &data, None);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Unknown subcommand"));
}

// ============================================================================
// STATISTICAL LAW SPECIFIC TESTS
// ============================================================================

#[test]
fn test_benford_risk_levels() {
    // Test compliant data (should be LOW risk)
    let compliant_data = TestFixtures::benford_compliant_data();
    let results = law("benford", &compliant_data, None).unwrap();

    match &results[0] {
        LawkitResult::BenfordAnalysis(_, benford_data) => {
            // This might be LOW or MEDIUM depending on exact data, but shouldn't be HIGH for compliant data
            assert!(benford_data.risk_level == "LOW" || benford_data.risk_level == "MEDIUM");
        }
        _ => panic!("Expected BenfordAnalysis result"),
    }

    // Test non-compliant data (should be higher risk)
    let non_compliant_data = TestFixtures::benford_non_compliant_data();
    let results = law("benford", &non_compliant_data, None).unwrap();

    match &results[0] {
        LawkitResult::BenfordAnalysis(_, benford_data) => {
            // Non-compliant data should show higher risk
            assert!(benford_data.risk_level == "MEDIUM" || benford_data.risk_level == "HIGH");
        }
        _ => panic!("Expected BenfordAnalysis result"),
    }
}

#[test]
fn test_pareto_principle_compliance() {
    // Test compliant data
    let compliant_data = TestFixtures::pareto_compliant_data();
    let results = law("pareto", &compliant_data, None).unwrap();

    match &results[0] {
        LawkitResult::ParetoAnalysis(_, pareto_data) => {
            // Should show strong Pareto principle (top 20% contributes significantly)
            assert!(pareto_data.top_20_percent_contribution > 60.0);
            assert!(pareto_data.concentration_index > 0.0);
        }
        _ => panic!("Expected ParetoAnalysis result"),
    }

    // Test non-compliant data
    let non_compliant_data = TestFixtures::pareto_non_compliant_data();
    let results = law("pareto", &non_compliant_data, None).unwrap();

    match &results[0] {
        LawkitResult::ParetoAnalysis(_, pareto_data) => {
            // Uniform data should not follow Pareto principle
            assert!(pareto_data.top_20_percent_contribution < 60.0);
        }
        _ => panic!("Expected ParetoAnalysis result"),
    }
}

#[test]
fn test_normal_distribution_detection() {
    // Test normal data
    let normal_data = TestFixtures::normal_distribution_data();
    let results = law("normal", &normal_data, None).unwrap();

    match &results[0] {
        LawkitResult::NormalAnalysis(_, normal_analysis) => {
            // Should show signs of normality
            assert!(normal_analysis.skewness.abs() < 2.0); // Not too skewed
            assert!(normal_analysis.std_dev > 0.0);
            assert!(normal_analysis.risk_level == "LOW" || normal_analysis.risk_level == "MEDIUM");
        }
        _ => panic!("Expected NormalAnalysis result"),
    }

    // Test non-normal data
    let non_normal_data = TestFixtures::non_normal_distribution_data();
    let results = law("normal", &non_normal_data, None).unwrap();

    match &results[0] {
        LawkitResult::NormalAnalysis(_, normal_analysis) => {
            // Skewed data should show deviation from normality
            assert!(normal_analysis.risk_level == "MEDIUM" || normal_analysis.risk_level == "HIGH");
        }
        _ => panic!("Expected NormalAnalysis result"),
    }
}

#[test]
fn test_poisson_distribution_detection() {
    // Test Poisson data
    let poisson_data = TestFixtures::poisson_distribution_data();
    let results = law("poisson", &poisson_data, None).unwrap();

    match &results[0] {
        LawkitResult::PoissonAnalysis(_, poisson_analysis) => {
            // Should show Poisson characteristics
            assert!(poisson_analysis.lambda > 0.0);
            assert!(poisson_analysis.variance_ratio > 0.0);
            // For Poisson, variance should approximately equal mean
            assert!(
                poisson_analysis.risk_level == "LOW" || poisson_analysis.risk_level == "MEDIUM"
            );
        }
        _ => panic!("Expected PoissonAnalysis result"),
    }

    // Test non-Poisson data
    let non_poisson_data = TestFixtures::non_poisson_data();
    let results = law("poisson", &non_poisson_data, None).unwrap();

    match &results[0] {
        LawkitResult::PoissonAnalysis(_, poisson_analysis) => {
            // High variance data should deviate from Poisson
            assert!(
                poisson_analysis.risk_level == "MEDIUM" || poisson_analysis.risk_level == "HIGH"
            );
        }
        _ => panic!("Expected PoissonAnalysis result"),
    }
}

// ============================================================================
// OPTIONS TESTING - lawkit Specific Options
// ============================================================================

#[test]
fn test_lawkit_specific_options() {
    let data = TestFixtures::benford_compliant_data();

    let lawkit_options = LawkitSpecificOptions {
        risk_threshold: Some("medium".to_string()),
        confidence_level: Some(0.95),
        significance_level: Some(0.05),
        min_sample_size: Some(20),
        enable_outlier_detection: Some(true),
        ..Default::default()
    };

    let options = LawkitOptions {
        lawkit_options: Some(lawkit_options),
        ..Default::default()
    };

    let results = law("benford", &data, Some(&options)).unwrap();

    assert_eq!(results.len(), 1);
    // Should still work with options
    match &results[0] {
        LawkitResult::BenfordAnalysis(_, benford_data) => {
            assert!(benford_data.total_numbers > 0);
        }
        _ => panic!("Expected BenfordAnalysis result"),
    }
}

#[test]
fn test_benford_specific_options() {
    let data = TestFixtures::benford_compliant_data();

    let lawkit_options = LawkitSpecificOptions {
        benford_digits: Some("first".to_string()),
        benford_base: Some(10),
        ..Default::default()
    };

    let options = LawkitOptions {
        lawkit_options: Some(lawkit_options),
        ..Default::default()
    };

    let results = law("benford", &data, Some(&options)).unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        LawkitResult::BenfordAnalysis(_, benford_data) => {
            // Should analyze first digits (9 digits: 1-9)
            assert_eq!(benford_data.observed_distribution.len(), 9);
            assert_eq!(benford_data.expected_distribution.len(), 9);
        }
        _ => panic!("Expected BenfordAnalysis result"),
    }
}

#[test]
fn test_pareto_specific_options() {
    let data = TestFixtures::pareto_compliant_data();

    let lawkit_options = LawkitSpecificOptions {
        pareto_ratio: Some(0.8), // 80/20 rule
        pareto_category_limit: Some(100),
        ..Default::default()
    };

    let options = LawkitOptions {
        lawkit_options: Some(lawkit_options),
        ..Default::default()
    };

    let results = law("pareto", &data, Some(&options)).unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        LawkitResult::ParetoAnalysis(_, pareto_data) => {
            assert!(pareto_data.total_items > 0);
            assert!(pareto_data.top_20_percent_contribution > 0.0);
        }
        _ => panic!("Expected ParetoAnalysis result"),
    }
}

#[test]
fn test_generation_options() {
    let config = json!({
        "type": "normal",
        "count": 500,
        "mean": 50.0,
        "std_dev": 10.0
    });

    let lawkit_options = LawkitSpecificOptions {
        generate_count: Some(500),
        generate_range_min: Some(0.0),
        generate_range_max: Some(100.0),
        generate_seed: Some(12345),
        ..Default::default()
    };

    let options = LawkitOptions {
        lawkit_options: Some(lawkit_options),
        ..Default::default()
    };

    let results = law("generate", &config, Some(&options)).unwrap();

    assert_eq!(results.len(), 1);
    match &results[0] {
        LawkitResult::GeneratedData(_, generated_info) => {
            assert_eq!(generated_info.data_type, "normal");
            assert_eq!(generated_info.count, 500);
            assert!(generated_info.parameters.contains_key("mean"));
            assert!(generated_info.parameters.contains_key("std_dev"));
            assert_eq!(generated_info.sample_data.len(), 500);
        }
        _ => panic!("Expected GeneratedData result"),
    }
}

// ============================================================================
// OUTPUT FORMAT TESTS
// ============================================================================

#[test]
fn test_lawkit_output_format() {
    let data = json!([1, 2, 3]);
    let results = law("validate", &data, None).unwrap();

    // Test lawkit-specific format
    let formatted = format_output(&results, OutputFormat::Lawkit).unwrap();
    assert!(!formatted.is_empty());
    assert!(formatted.contains("ValidationResult"));

    // Test all supported formats
    for format in OutputFormat::value_variants() {
        let output = format_output(&results, *format).unwrap();
        assert!(!output.is_empty());
    }
}

#[test]
fn test_output_format_parsing() {
    assert_eq!(
        OutputFormat::parse_format("lawkit").unwrap(),
        OutputFormat::Lawkit
    );
    assert_eq!(
        OutputFormat::parse_format("json").unwrap(),
        OutputFormat::Json
    );
    assert_eq!(
        OutputFormat::parse_format("yaml").unwrap(),
        OutputFormat::Yaml
    );
    assert_eq!(
        OutputFormat::parse_format("csv").unwrap(),
        OutputFormat::Csv
    );
    assert_eq!(
        OutputFormat::parse_format("text").unwrap(),
        OutputFormat::Text
    );

    assert!(OutputFormat::parse_format("invalid").is_err());
}

// ============================================================================
// DATA VALIDATION AND ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_empty_data_handling() {
    let empty_data = json!([]);

    let result = law("benford", &empty_data, None);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No valid numbers found"));
}

#[test]
fn test_invalid_data_handling() {
    let invalid_data = json!({"not": "numbers"});

    let result = law("benford", &invalid_data, None);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No valid numbers found"));
}

#[test]
fn test_small_sample_size() {
    let small_data = json!([1.0, 2.0]);

    // Normal analysis should fail with insufficient data
    let result = law("normal", &small_data, None);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Insufficient data points"));
}

#[test]
fn test_validation_with_issues() {
    let problematic_data = TestFixtures::validation_test_data();

    let results = law("validate", &problematic_data["small_dataset"], None).unwrap();

    match &results[0] {
        LawkitResult::ValidationResult(_, validation_data) => {
            assert!(!validation_data.validation_passed);
            assert!(!validation_data.issues_found.is_empty());
            assert!(validation_data.data_quality_score < 1.0);
        }
        _ => panic!("Expected ValidationResult"),
    }
}

// ============================================================================
// INTEGRATION TESTS WITH FIXTURES
// ============================================================================

#[test]
fn test_comprehensive_analysis_workflow() {
    let integration_data = TestFixtures::integration_analysis_data();

    let options = LawkitOptions {
        show_details: Some(true),
        show_recommendations: Some(true),
        output_format: Some(OutputFormat::Json),
        ..Default::default()
    };

    let results = law("analyze", &integration_data, Some(&options)).unwrap();

    // Should have multiple analyses
    assert!(results.len() > 1);

    // Check that we get different types of analysis
    let result_types: std::collections::HashSet<_> = results
        .iter()
        .map(|r| match r {
            LawkitResult::BenfordAnalysis(_, _) => "benford",
            LawkitResult::ParetoAnalysis(_, _) => "pareto",
            LawkitResult::ZipfAnalysis(_, _) => "zipf",
            LawkitResult::NormalAnalysis(_, _) => "normal",
            LawkitResult::PoissonAnalysis(_, _) => "poisson",
            LawkitResult::IntegrationAnalysis(_, _) => "integration",
            _ => "other",
        })
        .collect();

    // Should have multiple analysis types
    assert!(result_types.len() >= 3);
    assert!(result_types.contains("integration"));
}

#[test]
fn test_data_generation_and_analysis_cycle() {
    // Generate Benford data
    let config = json!({
        "type": "benford",
        "count": 100
    });

    let generation_results = law("generate", &config, None).unwrap();

    let generated_data = match &generation_results[0] {
        LawkitResult::GeneratedData(_, generated_info) => {
            json!(generated_info.sample_data)
        }
        _ => panic!("Expected GeneratedData result"),
    };

    // Analyze the generated data
    let analysis_results = law("benford", &generated_data, None).unwrap();

    match &analysis_results[0] {
        LawkitResult::BenfordAnalysis(_, benford_data) => {
            // Generated Benford data should show low risk
            assert!(benford_data.risk_level == "LOW" || benford_data.risk_level == "MEDIUM");
            assert_eq!(benford_data.total_numbers, 100);
        }
        _ => panic!("Expected BenfordAnalysis result"),
    }
}

// ============================================================================
// PERFORMANCE TESTS
// ============================================================================

#[test]
fn test_large_dataset_performance() {
    // Generate large dataset
    let large_data = law_generators::generate_benford_data(10000, "low");

    let start = std::time::Instant::now();
    let results = law("benford", &large_data, None).unwrap();
    let duration = start.elapsed();

    assert_eq!(results.len(), 1);
    assert!(duration.as_secs() < 5); // Should complete within 5 seconds

    match &results[0] {
        LawkitResult::BenfordAnalysis(_, benford_data) => {
            assert_eq!(benford_data.total_numbers, 10000);
        }
        _ => panic!("Expected BenfordAnalysis result"),
    }
}

#[test]
fn test_comprehensive_analysis_performance() {
    let integration_data = law_generators::generate_integration_test_data();

    let start = std::time::Instant::now();
    let results = law("analyze", &integration_data, None).unwrap();
    let duration = start.elapsed();

    assert!(results.len() > 1);
    assert!(duration.as_secs() < 10); // Should complete within 10 seconds

    // Should include integration analysis
    let has_integration = results
        .iter()
        .any(|r| matches!(r, LawkitResult::IntegrationAnalysis(_, _)));
    assert!(has_integration);
}

// ============================================================================
// PARSER FUNCTION TESTS (Internal Use)
// ============================================================================

#[test]
fn test_parse_json() {
    let content = r#"{"numbers": [1, 2, 3, 4, 5]}"#;
    let result = parse_json(content).unwrap();

    assert!(result["numbers"].is_array());
    assert_eq!(result["numbers"].as_array().unwrap().len(), 5);
}

#[test]
fn test_parse_csv() {
    let content = "value,amount\n123.45,100\n234.56,200\n345.67,300";
    let result = parse_csv(content).unwrap();

    if let Value::Array(records) = result {
        assert_eq!(records.len(), 3);
        assert!(records[0]["value"].as_str().unwrap().contains("123.45"));
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_parse_yaml() {
    let content = "numbers:\n  - 123.45\n  - 234.56\n  - 345.67";
    let result = parse_yaml(content).unwrap();

    assert!(result["numbers"].is_array());
    assert_eq!(result["numbers"].as_array().unwrap().len(), 3);
}

#[test]
fn test_parse_invalid_json() {
    let content = "invalid json {";
    let result = parse_json(content);
    assert!(result.is_err());
}

// ============================================================================
// UTILITY FUNCTION TESTS
// ============================================================================

#[test]
fn test_format_output_all_formats() {
    let results = vec![LawkitResult::ValidationResult(
        "test".to_string(),
        ValidationData {
            validation_passed: true,
            issues_found: vec![],
            data_quality_score: 1.0,
            analysis_summary: "Test validation".to_string(),
        },
    )];

    // Test all output formats
    for format in OutputFormat::value_variants() {
        let output = format_output(&results, *format).unwrap();
        assert!(!output.is_empty());

        match format {
            OutputFormat::Json => assert!(output.contains("{")),
            OutputFormat::Yaml => assert!(output.contains("ValidationResult")),
            OutputFormat::Lawkit => assert!(output.contains("ValidationResult")),
            OutputFormat::Csv => assert!(output.contains("type,summary")),
            OutputFormat::Text => assert!(output.contains("ValidationResult")),
        }
    }
}

// ============================================================================
// EDGE CASES AND BOUNDARY TESTS
// ============================================================================

#[test]
fn test_single_value_data() {
    let single_value = json!([42.0]);

    // Most analyses should fail or handle gracefully with single value
    let benford_result = law("benford", &single_value, None);
    // Should work but with limited data
    if benford_result.is_ok() {
        let results = benford_result.unwrap();
        if let LawkitResult::BenfordAnalysis(_, benford_data) = &results[0] {
            assert_eq!(benford_data.total_numbers, 1);
        }
    }

    // Normal analysis should fail
    let normal_result = law("normal", &single_value, None);
    assert!(normal_result.is_err());
}

#[test]
fn test_extreme_values() {
    let extreme_data = json!([1e-10, 1e10, -1e5, 0.0, f64::MAX]);

    let results = law("diagnose", &extreme_data, None).unwrap();

    match &results[0] {
        LawkitResult::DiagnosticResult(_, diagnostic_data) => {
            assert!(!diagnostic_data.findings.is_empty());
            // Should detect extreme range
            assert!(diagnostic_data.findings.iter().any(|f| f.contains("Range")));
        }
        _ => panic!("Expected DiagnosticResult"),
    }
}

#[test]
fn test_mixed_data_types_in_json() {
    let mixed_data = json!({
        "numbers": [1, 2, 3],
        "strings": ["4", "5", "6"], // Should be parsed as numbers
        "mixed": [7, "8", 9.5],
        "objects": [{"value": 10}, {"value": 11}]
    });

    let results = law("validate", &mixed_data, None).unwrap();

    match &results[0] {
        LawkitResult::ValidationResult(_, validation_data) => {
            // Should extract numbers from various sources
            assert!(validation_data.validation_passed);
        }
        _ => panic!("Expected ValidationResult"),
    }
}
