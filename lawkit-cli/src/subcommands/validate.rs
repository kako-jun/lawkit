use crate::common_options;
use crate::subcommands::compare_common::{get_dataset_name, get_numbers_from_input, output_integration_result};
use clap::{ArgMatches, Command};
use lawkit_core::common::output::{create_output_writer, OutputConfig};
use lawkit_core::error::Result;
use lawkit_core::laws::integration::{
    analyze_all_laws, cross_validate_laws,
};
use std::io::Write;

pub fn command() -> Command {
    common_options::add_compare_options(common_options::add_common_options(
        common_options::add_input_arg(
            Command::new("validate").about("データ検証と一貫性チェック"),
        ),
    ))
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    let _numbers = get_numbers_from_input(matches)?;
    let _dataset_name = get_dataset_name(matches);

    if matches.get_flag("cross-validation") {
        return run_cross_validation_mode(matches);
    }

    if matches.get_flag("consistency-check") {
        return run_consistency_check_mode(matches);
    }

    // Default: consistency check
    run_consistency_check_mode(matches)
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
    config: &OutputConfig,
) -> Result<()> {
    writeln!(writer, "{}", get_text("consistency_check_title", "en"))?;
    writeln!(writer)?;

    output_integration_result(writer, result, config)?;

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

fn get_text(key: &str, _lang: &str) -> String {
    match key {
        "cross_validation_title" => "Cross-Validation Analysis",
        "confidence_level" => "Confidence Level",
        "overall_stability" => "Overall Stability",
        "stability_assessment" => "Stability Assessment",
        "validation_folds" => "Validation Folds",
        "fold" => "Fold",
        "consistency_check_title" => "Consistency Check",
        "consistency_analysis" => "Consistency Analysis",
        "threshold" => "Threshold",
        "consistency_score" => "Consistency Score",
        "consistent_results" => "Results are consistent",
        "inconsistent_results" => "Results show inconsistencies",
        "dataset" => "Dataset",
        _ => key,
    }
    .to_string()
}