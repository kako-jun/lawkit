use crate::common_options;
use crate::subcommands::integration_common::{
    get_dataset_name, get_numbers_from_input, output_integration_result, parse_analysis_purpose,
};
use clap::{ArgMatches, Command};
use lawkit_core::common::output::{create_output_writer, OutputConfig};
use lawkit_core::error::Result;
use lawkit_core::laws::integration::{
    analyze_all_laws, detect_conflicts_detailed, generate_detailed_recommendations, AnalysisPurpose,
};
use std::io::Write;

pub fn command() -> Command {
    common_options::add_integration_options(common_options::add_common_options(
        common_options::add_input_arg(Command::new("diagnose").about("矛盾検出と詳細分析レポート")),
    ))
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    if matches.get_flag("recommend") {
        return run_recommendation_mode(matches);
    }

    let report_type = matches.get_one::<String>("report").unwrap();
    match report_type.as_str() {
        "conflicting" => run_conflict_analysis_mode(matches),
        "detailed" => run_detailed_analysis_mode(matches),
        _ => run_detailed_analysis_mode(matches), // Default to detailed
    }
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

fn output_detailed_integration_result(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    config: &OutputConfig,
) -> Result<()> {
    output_integration_result(writer, result, config)?;

    if config.format == "text" {
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

fn get_text(key: &str, _lang: &str) -> String {
    match key {
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
        "recommendation_title" => "Recommendations",
        "analysis_purpose" => "Analysis Purpose",
        "purpose_recommendations" => "Purpose-Based Recommendations",
        "combination_analysis" => "Combination Analysis",
        "individual_law_results" => "Individual Law Results",
        "data_characteristics" => "Data Characteristics",
        "data_type" => "Data Type",
        "distribution_shape" => "Distribution Shape",
        "outlier_presence" => "Outlier Presence",
        "scale_range" => "Scale Range",
        "sample_size_category" => "Sample Size Category",
        "alternative_combinations" => "Alternative Combinations",
        "effectiveness" => "Effectiveness",
        "rationale" => "Rationale",
        "dataset" => "Dataset",
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
