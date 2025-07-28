use crate::colors;
use crate::common_options::{get_optimized_reader, setup_automatic_optimization_config};
use clap::ArgMatches;
use lawkit_core::common::input::parse_text_input;
use lawkit_core::common::output::OutputConfig;
use lawkit_core::error::{BenfError, Result};
use lawkit_core::laws::integration::IntegrationResult;
use lawkit_core::{IntegrationData, LawkitResult};
use std::io::Write;

#[allow(dead_code)]
pub fn print_integration_header(matches: &ArgMatches) {
    let no_color = matches.get_flag("no-color");

    if no_color {
        println!("LAWKIT INTEGRATED ANALYSIS");
        println!("==========================");
    } else {
        println!("{}", colors::bold("LAWKIT INTEGRATED ANALYSIS"));
        println!("{}", colors::bold("=========================="));
    }
    println!();
}

#[allow(dead_code)]
pub fn print_integration_summary(data: &IntegrationData, no_color: bool) {
    // Analysis summary
    println!("{}", data.analysis_summary);
    println!();

    // Laws analyzed
    if no_color {
        println!("LAWS ANALYZED");
        println!("-------------");
    } else {
        println!("{}", colors::bold("LAWS ANALYZED"));
        println!("{}", colors::bold("-------------"));
    }

    for law in &data.laws_analyzed {
        println!("✓ {law}");
    }
    println!();

    // Overall risk
    let risk_display = if no_color {
        format!("Overall Risk Level: {}", data.overall_risk)
    } else {
        match data.overall_risk.as_str() {
            "HIGH" => format!("Overall Risk Level: {}", colors::red(&data.overall_risk)),
            "MEDIUM" => format!("Overall Risk Level: {}", colors::yellow(&data.overall_risk)),
            _ => format!("Overall Risk Level: {}", colors::green(&data.overall_risk)),
        }
    };
    println!("{risk_display}");
    println!();

    // Conflicting results if any
    if !data.conflicting_results.is_empty() {
        if no_color {
            println!("CONFLICTING RESULTS");
            println!("-------------------");
        } else {
            println!("{}", colors::bold("CONFLICTING RESULTS"));
            println!("{}", colors::bold("-------------------"));
        }

        for conflict in &data.conflicting_results {
            let msg = format!("⚠ {conflict}");
            println!("{}", if no_color { msg } else { colors::yellow(&msg) });
        }
        println!();
    }

    // Recommendations
    if !data.recommendations.is_empty() {
        if no_color {
            println!("RECOMMENDATIONS");
            println!("---------------");
        } else {
            println!("{}", colors::bold("RECOMMENDATIONS"));
            println!("{}", colors::bold("---------------"));
        }

        for recommendation in &data.recommendations {
            println!("• {recommendation}");
        }
        println!();
    }
}

#[allow(dead_code)]
pub fn print_individual_results(results: &[LawkitResult], matches: &ArgMatches) {
    let no_color = matches.get_flag("no-color");
    let verbose = matches.get_flag("verbose");

    for result in results {
        match result {
            LawkitResult::BenfordAnalysis(name, data) => {
                println!();
                if no_color {
                    println!("--- {} ---", name.to_uppercase());
                } else {
                    println!("--- {} ---", colors::cyan(&name.to_uppercase()));
                }
                println!("Risk Level: {}", data.risk_level);
                if verbose {
                    println!(
                        "Chi-square: {:.4}, P-value: {:.4}, MAD: {:.4}",
                        data.chi_square, data.p_value, data.mad
                    );
                }
            }
            LawkitResult::ParetoAnalysis(name, data) => {
                println!();
                if no_color {
                    println!("--- {} ---", name.to_uppercase());
                } else {
                    println!("--- {} ---", colors::cyan(&name.to_uppercase()));
                }
                println!("Risk Level: {}", data.risk_level);
                if verbose {
                    println!(
                        "Top 20% contribution: {:.1}%, Pareto ratio: {:.2}",
                        data.top_20_percent_contribution, data.pareto_ratio
                    );
                }
            }
            LawkitResult::ZipfAnalysis(name, data) => {
                println!();
                if no_color {
                    println!("--- {} ---", name.to_uppercase());
                } else {
                    println!("--- {} ---", colors::cyan(&name.to_uppercase()));
                }
                println!("Risk Level: {}", data.risk_level);
                if verbose {
                    println!(
                        "Zipf coefficient: {:.3}, Correlation: {:.3}",
                        data.zipf_coefficient, data.correlation_coefficient
                    );
                }
            }
            LawkitResult::NormalAnalysis(name, data) => {
                println!();
                if no_color {
                    println!("--- {} ---", name.to_uppercase());
                } else {
                    println!("--- {} ---", colors::cyan(&name.to_uppercase()));
                }
                println!("Risk Level: {}", data.risk_level);
                if verbose {
                    println!(
                        "Mean: {:.3}, Std Dev: {:.3}, Skewness: {:.3}, Kurtosis: {:.3}",
                        data.mean, data.std_dev, data.skewness, data.kurtosis
                    );
                }
            }
            LawkitResult::PoissonAnalysis(name, data) => {
                println!();
                if no_color {
                    println!("--- {} ---", name.to_uppercase());
                } else {
                    println!("--- {} ---", colors::cyan(&name.to_uppercase()));
                }
                println!("Risk Level: {}", data.risk_level);
                if verbose {
                    println!(
                        "Lambda: {:.3}, Variance ratio: {:.3}, P-value: {:.4}",
                        data.lambda, data.variance_ratio, data.poisson_test_p
                    );
                }
            }
            _ => {} // Skip other result types for now
        }
    }
}

pub fn get_dataset_name(matches: &ArgMatches) -> String {
    matches
        .get_one::<String>("input")
        .map(|s| s.to_string())
        .unwrap_or_else(|| "stdin".to_string())
}

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

    let data = buffer.map_err(|e| BenfError::ParseError(e.to_string()))?;
    parse_text_input(&data)
}

pub fn output_integration_result(
    writer: &mut Box<dyn Write>,
    result: &IntegrationResult,
    config: &OutputConfig,
) -> Result<()> {
    match config.format.as_str() {
        "text" => output_text_integration_result(writer, result, config),
        "json" => output_json_integration_result(writer, result),
        "csv" => output_csv_integration_result(writer, result),
        "yaml" => output_yaml_integration_result(writer, result),
        "toml" => output_toml_integration_result(writer, result),
        "xml" => output_xml_integration_result(writer, result),
        _ => output_text_integration_result(writer, result, config),
    }
}

fn output_text_integration_result(
    writer: &mut Box<dyn Write>,
    result: &IntegrationResult,
    config: &OutputConfig,
) -> Result<()> {
    writeln!(writer, "Integration Analysis Results")?;
    writeln!(writer, "=============================")?;
    writeln!(writer)?;

    writeln!(writer, "Dataset: {}", result.dataset_name)?;
    writeln!(writer, "Overall Risk Level: {:?}", result.risk_level)?;
    writeln!(writer)?;

    if !config.quiet {
        writeln!(writer, "Laws Executed: {}", result.laws_executed.join(", "))?;
        writeln!(
            writer,
            "Overall Quality Score: {:.3}",
            result.overall_quality_score
        )?;
        writeln!(writer, "Consistency Score: {:.3}", result.consistency_score)?;
        writeln!(writer, "Conflicts Detected: {}", result.conflicts_detected)?;
        writeln!(writer)?;

        if let Some(ref benford) = result.benford_result {
            writeln!(
                writer,
                "- Benford Law: {} (Chi-square: {:.4})",
                benford.risk_level, benford.chi_square
            )?;
        }
        if let Some(ref pareto) = result.pareto_result {
            writeln!(
                writer,
                "- Pareto Principle: {} (Top 20%: {:.1}%)",
                pareto.risk_level, pareto.top_20_percent_share
            )?;
        }
        if let Some(ref zipf) = result.zipf_result {
            writeln!(
                writer,
                "- Zipf Law: {} (Exponent: {:.3})",
                zipf.risk_level, zipf.zipf_exponent
            )?;
        }
        if let Some(ref normal) = result.normal_result {
            writeln!(
                writer,
                "- Normal Distribution: {} (Mean: {:.3})",
                normal.risk_level, normal.mean
            )?;
        }
        if let Some(ref poisson) = result.poisson_result {
            writeln!(
                writer,
                "- Poisson Distribution: {} (Lambda: {:.3})",
                poisson.risk_level, poisson.lambda
            )?;
        }
        writeln!(writer)?;
    }

    if !result.conflicts.is_empty() {
        writeln!(writer, "Conflicts:")?;
        for conflict in &result.conflicts {
            writeln!(writer, "• {}", conflict.description)?;
        }
        writeln!(writer)?;
    }

    Ok(())
}

fn output_json_integration_result(
    writer: &mut Box<dyn Write>,
    result: &IntegrationResult,
) -> Result<()> {
    use serde_json::json;

    let output = json!({
        "dataset_name": result.dataset_name,
        "risk_level": format!("{:?}", result.risk_level),
        "numbers_analyzed": result.numbers_analyzed,
        "laws_executed": result.laws_executed,
        "overall_quality_score": result.overall_quality_score,
        "consistency_score": result.consistency_score,
        "conflicts_detected": result.conflicts_detected
    });

    writeln!(writer, "{}", serde_json::to_string_pretty(&output).unwrap())?;
    Ok(())
}

fn output_csv_integration_result(
    writer: &mut Box<dyn Write>,
    result: &IntegrationResult,
) -> Result<()> {
    writeln!(
        writer,
        "dataset,risk_level,quality_score,consistency_score,conflicts"
    )?;
    writeln!(
        writer,
        "{},{:?},{:.3},{:.3},{}",
        result.dataset_name,
        result.risk_level,
        result.overall_quality_score,
        result.consistency_score,
        result.conflicts_detected
    )?;
    Ok(())
}

fn output_yaml_integration_result(
    writer: &mut Box<dyn Write>,
    result: &IntegrationResult,
) -> Result<()> {
    writeln!(writer, "dataset_name: \"{}\"", result.dataset_name)?;
    writeln!(writer, "risk_level: \"{:?}\"", result.risk_level)?;
    writeln!(
        writer,
        "primary_recommendation: {}",
        result.recommendations.primary_law
    )?;
    writeln!(
        writer,
        "recommendation_confidence: {:.3}",
        result.recommendations.confidence
    )?;
    Ok(())
}

fn output_toml_integration_result(
    writer: &mut Box<dyn Write>,
    result: &IntegrationResult,
) -> Result<()> {
    writeln!(writer, "dataset_name = \"{}\"", result.dataset_name)?;
    writeln!(writer, "risk_level = \"{:?}\"", result.risk_level)?;
    writeln!(
        writer,
        "primary_recommendation = \"{}\"",
        result.recommendations.primary_law
    )?;
    writeln!(
        writer,
        "recommendation_confidence = {:.3}",
        result.recommendations.confidence
    )?;
    Ok(())
}

fn output_xml_integration_result(
    writer: &mut Box<dyn Write>,
    result: &IntegrationResult,
) -> Result<()> {
    writeln!(writer, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
    writeln!(writer, "<integration_analysis>")?;
    writeln!(
        writer,
        "  <dataset_name>{}</dataset_name>",
        result.dataset_name
    )?;
    writeln!(writer, "  <risk_level>{:?}</risk_level>", result.risk_level)?;
    writeln!(
        writer,
        "  <primary_recommendation>{}</primary_recommendation>",
        result.recommendations.primary_law
    )?;
    writeln!(
        writer,
        "  <recommendation_confidence>{:.3}</recommendation_confidence>",
        result.recommendations.confidence
    )?;
    writeln!(writer, "</integration_analysis>")?;
    Ok(())
}

pub fn parse_analysis_purpose(purpose: &str) -> lawkit_core::laws::integration::AnalysisPurpose {
    use lawkit_core::laws::integration::AnalysisPurpose;
    match purpose {
        "quality" => AnalysisPurpose::QualityAudit,
        "fraud" => AnalysisPurpose::FraudDetection,
        "concentration" => AnalysisPurpose::ConcentrationAnalysis,
        "anomaly" => AnalysisPurpose::AnomalyDetection,
        "distribution" => AnalysisPurpose::DistributionFitting,
        _ => AnalysisPurpose::GeneralAnalysis,
    }
}
