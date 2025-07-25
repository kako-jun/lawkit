use crate::colors;
use clap::ArgMatches;
use lawkit_core::{IntegrationData, LawkitResult};

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
        println!("✓ {}", law);
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
    println!("{}", risk_display);
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
            let msg = format!("⚠ {}", conflict);
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
            println!("• {}", recommendation);
        }
        println!();
    }
}

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
