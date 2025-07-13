use crate::colors;
use crate::common_options::{get_optimized_reader, setup_automatic_optimization_config};
use chrono;
use clap::ArgMatches;
use lawkit_core::common::output::OutputConfig;
use lawkit_core::error::Result;
use lawkit_core::laws::integration::AnalysisPurpose;
use std::io::Write;

pub fn get_numbers_from_input(matches: &ArgMatches) -> Result<Vec<f64>> {
    let (_parallel_config, _memory_config) = setup_automatic_optimization_config();

    let buffer = if let Some(input) = matches.get_one::<String>("input") {
        if input == "-" {
            get_optimized_reader(None)
        } else {
            get_optimized_reader(Some(input))
        }
    } else {
        get_optimized_reader(None)
    };

    let data = buffer.map_err(|e| lawkit_core::error::BenfError::IoError(e.to_string()))?;

    if data.trim().is_empty() {
        return Err(lawkit_core::error::BenfError::ParseError(
            "No input data provided".to_string(),
        ));
    }

    lawkit_core::common::input::parse_text_input(&data)
}

pub fn get_dataset_name(matches: &ArgMatches) -> String {
    matches
        .get_one::<String>("input")
        .cloned()
        .unwrap_or_else(|| "stdin".to_string())
}

pub fn parse_analysis_purpose(purpose_str: &str) -> AnalysisPurpose {
    match purpose_str {
        "quality" => AnalysisPurpose::QualityAudit,
        "fraud" => AnalysisPurpose::FraudDetection,
        "concentration" => AnalysisPurpose::ConcentrationAnalysis,
        "anomaly" => AnalysisPurpose::AnomalyDetection,
        "distribution" => AnalysisPurpose::DistributionFitting,
        _ => AnalysisPurpose::GeneralAnalysis,
    }
}

// Output functions
pub fn output_integration_result(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    config: &OutputConfig,
) -> Result<()> {
    match config.format.as_str() {
        "json" => output_integration_json(writer, result),
        "csv" => output_integration_csv(writer, result),
        "yaml" => output_integration_yaml(writer, result),
        _ => output_integration_text(writer, result, config),
    }
}

fn output_integration_text(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    config: &OutputConfig,
) -> Result<()> {
    if config.quiet {
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
            writeln!(
                writer,
                "  {}",
                colors::level_conflict(&conflict.description)
            )?;
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

    if config.verbose {
        output_verbose_integration_details(writer, result, "en")?;
    }

    Ok(())
}

fn output_integration_json(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
) -> Result<()> {
    let _json_value = serde_json::json!({
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

    let enhanced_json = create_enhanced_integration_json(result);
    writeln!(writer, "{}", serde_json::to_string_pretty(&enhanced_json)?)?;
    Ok(())
}

/// diffx-coreを活用してより構造化されたJSON出力を生成
fn create_enhanced_integration_json(
    result: &lawkit_core::laws::integration::IntegrationResult,
) -> serde_json::Value {
    // 基本のJSON構造を作成
    let basic_json = serde_json::json!({
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

    // diffx-core拡張情報を追加
    let mut enhanced_json = basic_json;

    // 法則スコアの詳細解釈を追加
    if let Some(law_scores_obj) = enhanced_json
        .get_mut("law_scores")
        .and_then(|v| v.as_object_mut())
    {
        for (law, score) in &result.law_scores {
            if let Some(score_val) = law_scores_obj.get_mut(law) {
                *score_val = serde_json::json!({
                    "score": score,
                    "tier": classify_score_tier(*score),
                    "interpretation": interpret_score(*score)
                });
            }
        }
    }

    // 矛盾の重要度分類を追加
    if let Some(conflicts_array) = enhanced_json
        .get_mut("conflicts")
        .and_then(|v| v.as_array_mut())
    {
        for (i, conflict) in result.conflicts.iter().enumerate() {
            if let Some(conflict_obj) = conflicts_array.get_mut(i).and_then(|v| v.as_object_mut()) {
                conflict_obj.insert(
                    "severity".to_string(),
                    serde_json::Value::String(
                        classify_conflict_severity(conflict.conflict_score).to_string(),
                    ),
                );

                // diffx-core由来の矛盾には特別な情報を追加
                if matches!(
                    conflict.conflict_type,
                    lawkit_core::laws::integration::ConflictType::ScoreDeviation
                        | lawkit_core::laws::integration::ConflictType::UnexpectedConsistency
                ) {
                    conflict_obj.insert(
                        "detection_method".to_string(),
                        serde_json::Value::String("diffx-core structural analysis".to_string()),
                    );
                }
            }
        }
    }

    // メタデータを追加
    enhanced_json.as_object_mut().unwrap().insert(
        "metadata".to_string(),
        serde_json::json!({
            "tool": "lawkit",
            "enhanced_with": "diffx-core",
            "format_version": "2.2.0",
            "generation_timestamp": chrono::Utc::now().to_rfc3339()
        }),
    );

    enhanced_json
}

// ヘルパー関数群
fn classify_score_tier(score: f64) -> &'static str {
    match score {
        s if s >= 0.9 => "excellent",
        s if s >= 0.7 => "good",
        s if s >= 0.5 => "fair",
        s if s >= 0.3 => "poor",
        _ => "very_poor",
    }
}

fn interpret_score(score: f64) -> &'static str {
    match score {
        s if s >= 0.9 => "Strong adherence to statistical law",
        s if s >= 0.7 => "Good fit with expected distribution",
        s if s >= 0.5 => "Moderate alignment with pattern",
        s if s >= 0.3 => "Weak correlation with expected behavior",
        _ => "Minimal or no adherence to law",
    }
}

fn classify_conflict_severity(score: f64) -> &'static str {
    match score {
        s if s >= 0.8 => "critical",
        s if s >= 0.6 => "high",
        s if s >= 0.4 => "medium",
        s if s >= 0.2 => "low",
        _ => "minimal",
    }
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

// Helper functions for verbose output
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

// Text localization
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
        "detailed_metrics" => "Detailed Metrics",
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
        "benf" => "Benford Law",
        "pareto" => "Pareto Principle",
        "zipf" => "Zipf Law",
        "normal" => "Normal Distribution",
        "poisson" => "Poisson Distribution",
        _ => law,
    }
    .to_string()
}
