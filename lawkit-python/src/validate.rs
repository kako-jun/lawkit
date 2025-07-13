use crate::colors;
use crate::common_options;
use crate::subcommands::integration_common::{
    get_dataset_name, get_numbers_from_input, output_integration_result,
};
use clap::{ArgMatches, Command};
use lawkit_core::common::output::{create_output_writer, OutputConfig};
use lawkit_core::error::Result;
use lawkit_core::laws::integration::{analyze_all_laws, cross_validate_laws};
use std::io::Write;

pub fn command() -> Command {
    common_options::add_integration_options(common_options::add_common_options(
        common_options::add_input_arg(Command::new("validate").about("データ検証と一貫性チェック")),
    ))
}

pub fn run(matches: &ArgMatches) -> Result<()> {
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
    writeln!(writer, "Cross-Validation Analysis")?;
    writeln!(writer)?;
    writeln!(writer, "Dataset: {}", result.dataset_name)?;
    writeln!(writer, "Confidence Level: {:.3}", result.confidence_level)?;
    writeln!(writer, "Overall Stability: {:.3}", result.overall_stability)?;
    writeln!(
        writer,
        "Stability Assessment: {:?}",
        result.stability_assessment
    )?;
    writeln!(writer)?;

    writeln!(writer, "Validation Folds:")?;
    for fold in &result.validation_folds {
        writeln!(writer, "  Consistency Score: {:.3}", fold.consistency_score)?;
    }

    Ok(())
}

fn output_consistency_check_result(
    writer: &mut Box<dyn Write>,
    result: &lawkit_core::laws::integration::IntegrationResult,
    threshold: f64,
    config: &OutputConfig,
) -> Result<()> {
    writeln!(writer, "Data Validation and Consistency Check")?;
    writeln!(writer)?;
    writeln!(writer, "Dataset: {}", result.dataset_name)?;
    writeln!(writer, "Threshold: {threshold:.3}")?;
    writeln!(writer, "Consistency Score: {:.3}", result.consistency_score)?;
    writeln!(writer)?;

    if result.consistency_score < threshold {
        writeln!(
            writer,
            "{}",
            colors::level_warning("Consistency below threshold")
        )?;
        writeln!(
            writer,
            "Recommendation: Review data quality and collection methods"
        )?;
    } else {
        writeln!(
            writer,
            "{}",
            colors::level_pass("Data consistency meets requirements")
        )?;
    }

    writeln!(writer)?;
    output_integration_result(writer, result, config)?;

    Ok(())
}
