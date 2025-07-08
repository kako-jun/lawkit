use crate::common_options::{self, get_optimized_reader, setup_optimization_config};
use clap::{ArgMatches, Command};
use lawkit_core::common::output::{create_output_writer, OutputConfig};
use lawkit_core::error::Result;
use lawkit_core::laws::integration::{
    analyze_all_laws, analyze_selected_laws, apply_focus_analysis, compare_laws,
    cross_validate_laws, detect_conflicts_detailed, generate_detailed_recommendations,
    AnalysisPurpose,
};
use std::io::Write;

pub fn command() -> Command {
    common_options::add_compare_options(common_options::add_common_options(
        common_options::add_input_arg(
            Command::new("compare").about("複数の統計法則を比較・統合分析"),
        ),
    ))
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    // 分析モード判定
    if matches.get_flag("cross-validation") {
        return run_cross_validation_mode(matches);
    }

    if matches.get_flag("consistency-check") {
        return run_consistency_check_mode(matches);
    }

    if matches.get_flag("recommend") {
        return run_recommendation_mode(matches);
    }

    let report_type = matches.get_one::<String>("report").unwrap();
    match report_type.as_str() {
        "conflicting" => run_conflict_analysis_mode(matches),
        "detailed" => run_detailed_analysis_mode(matches),
        _ => run_summary_analysis_mode(matches),
    }
}

fn get_numbers_from_input(matches: &ArgMatches) -> Result<Vec<f64>> {
    let (use_optimize, _parallel_config, _memory_config) = setup_optimization_config(matches);

    let buffer = if let Some(input) = matches.get_one::<String>("input") {
        if input == "-" {
            get_optimized_reader(None, use_optimize)
        } else {
            get_optimized_reader(Some(input), use_optimize)
        }
    } else {
        get_optimized_reader(None, use_optimize)
    };

    let data = buffer.map_err(|e| lawkit_core::error::BenfError::IoError(e.to_string()))?;

    if data.trim().is_empty() {
        return Err(lawkit_core::error::BenfError::ParseError(
            "No input data provided".to_string(),
        ));
    }

    lawkit_core::common::input::parse_text_input(&data)
}

fn run_summary_analysis_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = get_numbers_from_input(matches)?;
    let dataset_name = get_dataset_name(matches);

    let result = if let Some(laws_str) = matches.get_one::<String>("laws") {
        let selected_laws: Vec<String> =
            laws_str.split(',').map(|s| s.trim().to_string()).collect();
        let mut result = analyze_selected_laws(&numbers, &dataset_name, &selected_laws)?;

        // Apply focus if provided
        if let Some(focus) = matches.get_one::<String>("focus") {
            apply_focus_analysis(&mut result, focus);
        }

        result
    } else if let Some(focus) = matches.get_one::<String>("focus") {
        compare_laws(&numbers, &dataset_name, Some(focus))?
    } else {
        analyze_all_laws(&numbers, &dataset_name)?
    };

    let mut writer = create_output_writer(matches)?;
    let output_config = OutputConfig::from_matches(matches);

    output_integration_result(&mut writer, &result, &output_config)?;

    std::process::exit(result.risk_level.exit_code());
}

fn run_detailed_analysis_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = get_numbers_from_input(matches)?;
    let dataset_name = get_dataset_name(matches);

    let result = analyze_all_laws(&numbers, &dataset_name)?;

    let mut writer = create_output_writer(matches)?;
    let output_config = OutputConfig::from_matches(matches);

    output_detailed_integration_result(&mut writer, &result, &output_config)?;

    std::process::exit(result.risk_level.exit_code());
}

fn run_conflict_analysis_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = get_numbers_from_input(matches)?;
    let dataset_name = get_dataset_name(matches);
    let threshold = *matches.get_one::<f64>("threshold").unwrap();

    let conflict_result = detect_conflicts_detailed(&numbers, &dataset_name, threshold)?;

    let mut writer = create_output_writer(matches)?;
    let output_config = OutputConfig::from_matches(matches);

    output_conflict_analysis_result(&mut writer, &conflict_result, &output_config)?;

    std::process::exit(conflict_result.integration_result.risk_level.exit_code());
}

fn run_cross_validation_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = get_numbers_from_input(matches)?;
    let dataset_name = get_dataset_name(matches);
    let confidence_level = *matches.get_one::<f64>("confidence-level").unwrap();

    let cv_result = cross_validate_laws(&numbers, &dataset_name, confidence_level)?;

    let mut writer = create_output_writer(matches)?;
    let output_config = OutputConfig::from_matches(matches);

    output_cross_validation_result(&mut writer, &cv_result, &output_config)?;

    Ok(())
}

fn run_consistency_check_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = get_numbers_from_input(matches)?;
    let dataset_name = get_dataset_name(matches);
    let threshold = *matches.get_one::<f64>("threshold").unwrap();

    let result = analyze_all_laws(&numbers, &dataset_name)?;

    let mut writer = create_output_writer(matches)?;
    let output_config = OutputConfig::from_matches(matches);

    output_consistency_check_result(&mut writer, &result, threshold, &output_config)?;

    std::process::exit(result.risk_level.exit_code());
}

fn run_recommendation_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = get_numbers_from_input(matches)?;
    let dataset_name = get_dataset_name(matches);

    let analysis_purpose = matches
        .get_one::<String>("purpose")
        .map(|p| parse_analysis_purpose(p))
        .unwrap_or(AnalysisPurpose::GeneralAnalysis);

    let recommendation_result =
        generate_detailed_recommendations(&numbers, &dataset_name, analysis_purpose)?;

    let mut writer = create_output_writer(matches)?;
    let output_config = OutputConfig::from_matches(matches);

    output_recommendation_result(&mut writer, &recommendation_result, &output_config)?;

    std::process::exit(
        recommendation_result
            .integration_result
            .risk_level
            .exit_code(),
    );
}

fn get_dataset_name(matches: &ArgMatches) -> String {
    matches
        .get_one::<String>("input")
        .cloned()
        .unwrap_or_else(|| "stdin".to_string())
}

fn parse_analysis_purpose(purpose_str: &str) -> AnalysisPurpose {
    match purpose_str {
        "quality" => AnalysisPurpose::QualityAudit,
        "fraud" => AnalysisPurpose::FraudDetection,
        "concentration" => AnalysisPurpose::ConcentrationAnalysis,
        "anomaly" => AnalysisPurpose::AnomalyDetection,
        "distribution" => AnalysisPurpose::DistributionFitting,
        _ => AnalysisPurpose::GeneralAnalysis,
    }
}

// 出力関数群

fn output_integration_result(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    _config: &OutputConfig,
) -> Result<()> {
    match _config.format.as_str() {
        "json" => output_integration_json(writer, result),
        "csv" => output_integration_csv(writer, result),
        "yaml" => output_integration_yaml(writer, result),
        _ => output_integration_text(writer, result, _config),
    }
}

fn output_integration_text(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    _config: &OutputConfig,
) -> Result<()> {
    if _config.quiet {
        writeln!(writer, "{:.3}", result.overall_quality_score)?;
        return Ok(());
    }

    writeln!(writer, "Statistical Laws Integration Analysis")?;
    writeln!(writer)?;

    writeln!(
        writer,
        "{}: {}",
        get_text("dataset", "en"),
        result.dataset_name
    )?;
    writeln!(
        writer,
        "{}: {}",
        get_text("numbers_analyzed", "en"),
        result.numbers_analyzed
    )?;
    writeln!(
        writer,
        "{}: {} ({})",
        get_text("laws_executed", "en"),
        result.laws_executed.len(),
        result.laws_executed.join(", ")
    )?;

    if let Some(ref focus) = result.focus {
        writeln!(writer, "{}: {}", get_text("focus", "en"), focus)?;
    }

    writeln!(writer)?;

    writeln!(writer, "{}:", get_text("integration_metrics", "en"))?;
    writeln!(
        writer,
        "  {}: {:.3}",
        get_text("overall_quality", "en"),
        result.overall_quality_score
    )?;
    writeln!(
        writer,
        "  {}: {:.3}",
        get_text("consistency", "en"),
        result.consistency_score
    )?;
    writeln!(
        writer,
        "  {}: {}",
        get_text("conflicts_detected", "en"),
        result.conflicts_detected
    )?;
    writeln!(
        writer,
        "  {}: {:.3}",
        get_text("recommendation_confidence", "en"),
        result.recommendation_confidence
    )?;
    writeln!(writer)?;

    writeln!(writer, "{}:", get_text("law_results", "en"))?;
    for (law, score) in &result.law_scores {
        let law_name = get_law_name(law, "en");
        writeln!(writer, "  {law_name}: {score:.3}")?;
    }
    writeln!(writer)?;

    if !result.conflicts.is_empty() {
        writeln!(writer, "{}:", get_text("conflicts", "en"))?;
        for conflict in &result.conflicts {
            writeln!(writer, "  CONFLICT: {}", conflict.description)?;
            writeln!(
                writer,
                "     {}: {}",
                get_text("cause", "en"),
                conflict.likely_cause
            )?;
            writeln!(
                writer,
                "     {}: {}",
                get_text("suggestion", "en"),
                conflict.resolution_suggestion
            )?;
        }
        writeln!(writer)?;
    }

    writeln!(writer, "{}:", get_text("recommendations", "en"))?;
    writeln!(
        writer,
        "  FOCUS: {}: {}",
        get_text("primary_law", "en"),
        get_law_name(&result.recommendations.primary_law, "en")
    )?;

    if !result.recommendations.secondary_laws.is_empty() {
        let secondary_names: Vec<String> = result
            .recommendations
            .secondary_laws
            .iter()
            .map(|law| get_law_name(law, "en"))
            .collect();
        writeln!(
            writer,
            "  DETAIL: {}: {}",
            get_text("secondary_laws", "en"),
            secondary_names.join(", ")
        )?;
    }

    writeln!(
        writer,
        "  METRIC: {}: {}",
        get_text("rationale", "en"),
        result.recommendations.rationale
    )?;
    writeln!(writer)?;

    if _config.verbose {
        output_verbose_integration_details(writer, result, "en")?;
    }

    Ok(())
}

fn output_integration_json(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
) -> Result<()> {
    let json_value = serde_json::json!({
        "dataset": result.dataset_name,
        "numbers_analyzed": result.numbers_analyzed,
        "laws_executed": result.laws_executed,
        "focus": result.focus,
        "integration_metrics": {
            "overall_quality_score": result.overall_quality_score,
            "consistency_score": result.consistency_score,
            "conflicts_detected": result.conflicts_detected,
            "recommendation_confidence": result.recommendation_confidence
        },
        "law_scores": result.law_scores,
        "conflicts": result.conflicts.iter().map(|c| {
            serde_json::json!({
                "type": format!("{:?}", c.conflict_type),
                "laws_involved": c.laws_involved,
                "conflict_score": c.conflict_score,
                "description": c.description,
                "likely_cause": c.likely_cause,
                "resolution_suggestion": c.resolution_suggestion
            })
        }).collect::<Vec<_>>(),
        "recommendations": {
            "primary_law": result.recommendations.primary_law,
            "secondary_laws": result.recommendations.secondary_laws,
            "confidence": result.recommendations.confidence,
            "rationale": result.recommendations.rationale
        },
        "overall_assessment": format!("{:?}", result.overall_assessment),
        "risk_level": format!("{:?}", result.risk_level)
    });

    writeln!(writer, "{}", serde_json::to_string_pretty(&json_value)?)?;
    Ok(())
}

fn output_integration_csv(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
) -> Result<()> {
    writeln!(writer, "dataset,numbers_analyzed,laws_executed,focus,overall_quality_score,consistency_score,conflicts_detected,primary_law,overall_assessment,risk_level")?;
    writeln!(
        writer,
        "{},{},{},{},{:.3},{:.3},{},{},{:?},{:?}",
        result.dataset_name,
        result.numbers_analyzed,
        result.laws_executed.len(),
        result.focus.as_deref().unwrap_or(""),
        result.overall_quality_score,
        result.consistency_score,
        result.conflicts_detected,
        result.recommendations.primary_law,
        result.overall_assessment,
        result.risk_level
    )?;
    Ok(())
}

fn output_integration_yaml(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
) -> Result<()> {
    writeln!(writer, "dataset: \"{}\"", result.dataset_name)?;
    writeln!(writer, "numbers_analyzed: {}", result.numbers_analyzed)?;
    writeln!(writer, "laws_executed:")?;
    for law in &result.laws_executed {
        writeln!(writer, "  - \"{law}\"")?;
    }
    if let Some(ref focus) = result.focus {
        writeln!(writer, "focus: \"{focus}\"")?;
    }
    writeln!(writer, "integration_metrics:")?;
    writeln!(
        writer,
        "  overall_quality_score: {:.3}",
        result.overall_quality_score
    )?;
    writeln!(
        writer,
        "  consistency_score: {:.3}",
        result.consistency_score
    )?;
    writeln!(
        writer,
        "  conflicts_detected: {}",
        result.conflicts_detected
    )?;
    writeln!(writer, "law_scores:")?;
    for (law, score) in &result.law_scores {
        writeln!(writer, "  {law}: {score:.3}")?;
    }
    writeln!(writer, "recommendations:")?;
    writeln!(
        writer,
        "  primary_law: \"{}\"",
        result.recommendations.primary_law
    )?;
    writeln!(
        writer,
        "  confidence: {:.3}",
        result.recommendations.confidence
    )?;
    Ok(())
}

fn output_detailed_integration_result(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    _config: &OutputConfig,
) -> Result<()> {
    output_integration_result(writer, result, _config)?;

    if _config.format == "text" {
        writeln!(writer)?;
        writeln!(writer, "=== {} ===", get_text("detailed_analysis", "en"))?;

        output_detailed_law_results(writer, result, "en")?;
        output_data_characteristics(writer, result, "en")?;
        output_alternative_combinations(writer, result, "en")?;
    }

    Ok(())
}

fn output_conflict_analysis_result(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::ConflictAnalysisResult,
    _config: &OutputConfig,
) -> Result<()> {
    writeln!(writer, "{}", get_text("conflict_analysis_title", "en"))?;
    writeln!(writer)?;
    writeln!(
        writer,
        "{}: {}",
        get_text("dataset", "en"),
        result.dataset_name
    )?;
    writeln!(
        writer,
        "{}: {:.3}",
        get_text("threshold", "en"),
        result.threshold
    )?;
    writeln!(
        writer,
        "{}: {:?}",
        get_text("conflict_severity", "en"),
        result.conflict_severity
    )?;
    writeln!(writer)?;

    if !result.detailed_conflicts.is_empty() {
        writeln!(writer, "{}:", get_text("detailed_conflicts", "en"))?;
        for (i, conflict) in result.detailed_conflicts.iter().enumerate() {
            writeln!(writer, "{}. {}", i + 1, conflict.base_conflict.description)?;
            writeln!(
                writer,
                "   {}: {:.3}",
                get_text("significance", "en"),
                conflict.statistical_significance
            )?;
            writeln!(
                writer,
                "   {}: {:?}",
                get_text("impact", "en"),
                conflict.impact_assessment
            )?;
            writeln!(
                writer,
                "   {}: {}",
                get_text("root_cause", "en"),
                conflict.root_cause_analysis
            )?;
            writeln!(writer)?;
        }
    }

    if !result.resolution_strategies.is_empty() {
        writeln!(writer, "{}:", get_text("resolution_strategies", "en"))?;
        for strategy in &result.resolution_strategies {
            writeln!(
                writer,
                "• {} ({:?})",
                strategy.strategy_name, strategy.priority
            )?;
            writeln!(
                writer,
                "  {}: {}",
                get_text("expected_outcome", "en"),
                strategy.expected_outcome
            )?;
            writeln!(
                writer,
                "  {}: {:.3}",
                get_text("confidence", "en"),
                strategy.confidence
            )?;
            writeln!(writer)?;
        }
    }

    Ok(())
}

fn output_cross_validation_result(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::CrossValidationResult,
    _config: &OutputConfig,
) -> Result<()> {
    writeln!(writer, "{}", get_text("cross_validation_title", "en"))?;
    writeln!(writer)?;
    writeln!(
        writer,
        "{}: {}",
        get_text("dataset", "en"),
        result.dataset_name
    )?;
    writeln!(
        writer,
        "{}: {:.3}",
        get_text("confidence_level", "en"),
        result.confidence_level
    )?;
    writeln!(
        writer,
        "{}: {:.3}",
        get_text("overall_stability", "en"),
        result.overall_stability
    )?;
    writeln!(
        writer,
        "{}: {:?}",
        get_text("stability_assessment", "en"),
        result.stability_assessment
    )?;
    writeln!(writer)?;

    writeln!(writer, "{}:", get_text("validation_folds", "en"))?;
    for fold in &result.validation_folds {
        writeln!(
            writer,
            "  {} {}: {:.3}",
            get_text("fold", "en"),
            fold.fold_number,
            fold.consistency_score
        )?;
    }

    Ok(())
}

fn output_consistency_check_result(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    threshold: f64,
    _config: &OutputConfig,
) -> Result<()> {
    writeln!(writer, "{}", get_text("consistency_check_title", "en"))?;
    writeln!(writer)?;

    output_integration_result(writer, result, _config)?;

    writeln!(writer)?;
    writeln!(writer, "=== {} ===", get_text("consistency_analysis", "en"))?;
    writeln!(writer, "{}: {:.3}", get_text("threshold", "en"), threshold)?;
    writeln!(
        writer,
        "{}: {:.3}",
        get_text("consistency_score", "en"),
        result.consistency_score
    )?;

    if result.consistency_score >= threshold {
        writeln!(writer, "[PASS] {}", get_text("consistent_results", "en"))?;
    } else {
        writeln!(writer, "[WARN] {}", get_text("inconsistent_results", "en"))?;
    }

    Ok(())
}

fn output_recommendation_result(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::DetailedRecommendationResult,
    _config: &OutputConfig,
) -> Result<()> {
    writeln!(writer, "{}", get_text("recommendation_title", "en"))?;
    writeln!(writer)?;
    writeln!(
        writer,
        "{}: {}",
        get_text("dataset", "en"),
        result.dataset_name
    )?;
    writeln!(
        writer,
        "{}: {:?}",
        get_text("analysis_purpose", "en"),
        result.analysis_purpose
    )?;
    writeln!(writer)?;

    writeln!(writer, "{}:", get_text("purpose_recommendations", "en"))?;
    for rec in &result.purpose_specific_recommendations {
        writeln!(
            writer,
            "• {:?}: {}",
            rec.purpose,
            rec.recommended_laws.join(", ")
        )?;
        writeln!(
            writer,
            "  {}: {}",
            get_text("rationale", "en"),
            rec.rationale
        )?;
        writeln!(
            writer,
            "  {}: {:.3}",
            get_text("effectiveness", "en"),
            rec.effectiveness
        )?;
        writeln!(writer)?;
    }

    if !result.combination_analysis.is_empty() {
        writeln!(writer, "{}:", get_text("combination_analysis", "en"))?;
        for combo in result.combination_analysis.iter().take(3) {
            writeln!(
                writer,
                "• {}: {:.3}",
                combo.laws.join(" + "),
                combo.synergy_score
            )?;
        }
        writeln!(writer)?;
    }

    Ok(())
}

// 詳細出力ヘルパー関数

fn output_verbose_integration_details(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    _lang: &str,
) -> Result<()> {
    writeln!(writer, "=== {} ===", get_text("detailed_metrics", "en"))?;

    output_data_characteristics(writer, result, "en")?;

    if !result.recommendations.alternative_combinations.is_empty() {
        output_alternative_combinations(writer, result, "en")?;
    }

    Ok(())
}

fn output_detailed_law_results(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    _lang: &str,
) -> Result<()> {
    writeln!(writer, "{}:", get_text("individual_law_results", "en"))?;

    if let Some(ref benf_result) = result.benford_result {
        writeln!(
            writer,
            "• {}: {:.3} ({:?})",
            get_law_name("benf", "en"),
            1.0 - (benf_result.mean_absolute_deviation / 100.0),
            benf_result.risk_level
        )?;
    }

    if let Some(ref pareto_result) = result.pareto_result {
        writeln!(
            writer,
            "• {}: {:.3} ({:?})",
            get_law_name("pareto", "en"),
            pareto_result.concentration_index,
            pareto_result.risk_level
        )?;
    }

    if let Some(ref zipf_result) = result.zipf_result {
        writeln!(
            writer,
            "• {}: {:.3} ({:?})",
            get_law_name("zipf", "en"),
            zipf_result.distribution_quality,
            zipf_result.risk_level
        )?;
    }

    if let Some(ref normal_result) = result.normal_result {
        writeln!(
            writer,
            "• {}: {:.3} ({:?})",
            get_law_name("normal", "en"),
            normal_result.normality_score,
            normal_result.risk_level
        )?;
    }

    if let Some(ref poisson_result) = result.poisson_result {
        writeln!(
            writer,
            "• {}: {:.3} ({:?})",
            get_law_name("poisson", "en"),
            poisson_result.goodness_of_fit_score,
            poisson_result.risk_level
        )?;
    }

    writeln!(writer)?;
    Ok(())
}

fn output_data_characteristics(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    _lang: &str,
) -> Result<()> {
    let chars = &result.data_characteristics;

    writeln!(writer, "{}:", get_text("data_characteristics", "en"))?;
    writeln!(
        writer,
        "  {}: {:?}",
        get_text("data_type", "en"),
        chars.data_type
    )?;
    writeln!(
        writer,
        "  {}: {:?}",
        get_text("distribution_shape", "en"),
        chars.distribution_shape
    )?;
    writeln!(
        writer,
        "  {}: {:?}",
        get_text("outlier_presence", "en"),
        chars.outlier_presence
    )?;
    writeln!(
        writer,
        "  {}: {:?}",
        get_text("scale_range", "en"),
        chars.scale_range
    )?;
    writeln!(
        writer,
        "  {}: {:?}",
        get_text("sample_size_category", "en"),
        chars.sample_size_category
    )?;
    writeln!(writer)?;

    Ok(())
}

fn output_alternative_combinations(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    _lang: &str,
) -> Result<()> {
    writeln!(writer, "{}:", get_text("alternative_combinations", "en"))?;

    for combo in &result.recommendations.alternative_combinations {
        writeln!(writer, "• {} ({})", combo.purpose, combo.laws.join(" + "))?;
        writeln!(
            writer,
            "  {}: {:.3}",
            get_text("effectiveness", "en"),
            combo.effectiveness_score
        )?;
        writeln!(
            writer,
            "  {}: {}",
            get_text("description", "en"),
            combo.description
        )?;
        writeln!(writer)?;
    }

    Ok(())
}

// 多言語対応ヘルパー関数

fn get_text(key: &str, _lang: &str) -> String {
    match key {
        "integration_title" => "Integration Analysis Result",
        "dataset" => "Dataset",
        "numbers_analyzed" => "Numbers Analyzed",
        "laws_executed" => "Laws Executed",
        "integration_metrics" => "Integration Metrics",
        "overall_quality" => "Overall Quality Score",
        "consistency" => "Consistency Score",
        "conflicts_detected" => "Conflicts Detected",
        "recommendation_confidence" => "Recommendation Confidence",
        "law_results" => "Law Results",
        "conflicts" => "Conflicts",
        "cause" => "Likely Cause",
        "suggestion" => "Suggestion",
        "recommendations" => "Recommendations",
        "primary_law" => "Primary Law",
        "secondary_laws" => "Secondary Laws",
        "rationale" => "Rationale",
        "focus" => "Focus",
        "detailed_analysis" => "Detailed Analysis",
        "conflict_analysis_title" => "Conflict Analysis",
        "threshold" => "Threshold",
        "conflict_severity" => "Conflict Severity",
        "detailed_conflicts" => "Detailed Conflicts",
        "significance" => "Significance",
        "impact" => "Impact",
        "root_cause" => "Root Cause",
        "resolution_strategies" => "Resolution Strategies",
        "expected_outcome" => "Expected Outcome",
        "confidence" => "Confidence",
        "cross_validation_title" => "Cross-Validation Analysis",
        "confidence_level" => "Confidence Level",
        "overall_stability" => "Overall Stability",
        "stability_assessment" => "Stability Assessment",
        "validation_folds" => "Validation Folds",
        "fold" => "Fold",
        "consistency_check_title" => "Consistency Check",
        "consistency_analysis" => "Consistency Analysis",
        "consistency_score" => "Consistency Score",
        "consistent_results" => "Results are consistent",
        "inconsistent_results" => "Results show inconsistencies",
        "recommendation_title" => "Recommendations",
        "analysis_purpose" => "Analysis Purpose",
        "purpose_recommendations" => "Purpose-Based Recommendations",
        "combination_analysis" => "Combination Analysis",
        "detailed_metrics" => "Detailed Metrics",
        "individual_law_results" => "Individual Law Results",
        "data_characteristics" => "Data Characteristics",
        "data_type" => "Data Type",
        "distribution_shape" => "Distribution Shape",
        "outlier_presence" => "Outlier Presence",
        "scale_range" => "Scale Range",
        "sample_size_category" => "Sample Size Category",
        "alternative_combinations" => "Alternative Combinations",
        "effectiveness" => "Effectiveness",
        "description" => "Description",
        _ => key,
    }
    .to_string()
}

fn get_law_name(law: &str, _lang: &str) -> String {
    match law {
        "benf" => "Benford's Law",
        "pareto" => "Pareto Principle",
        "zipf" => "Zipf's Law",
        "normal" => "Normal Distribution",
        "poisson" => "Poisson Distribution",
        _ => law,
    }
    .to_string()
}
