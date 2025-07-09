use crate::colors;
use clap::ArgMatches;
use lawkit_core::{
    common::{
        filtering::{apply_number_filter, NumberFilter, RiskThreshold},
        input::{parse_input_auto, parse_text_input},
        risk::RiskLevel,
        streaming_io::OptimizedFileReader,
    },
    error::{BenfError, Result},
    laws::benford::BenfordResult,
};
use std::io::{self, Read};
use std::str::FromStr;

pub fn run(matches: &ArgMatches) -> Result<()> {
    // Determine input source based on arguments
    if let Some(input) = matches.get_one::<String>("input") {
        // Use auto-detection for file vs string input
        match parse_input_auto(input) {
            Ok(numbers) => {
                if numbers.is_empty() {
                    eprintln!("Error: No valid numbers found in input");
                    std::process::exit(1);
                }

                // Apply filtering and custom analysis
                let result =
                    match analyze_numbers_with_options(matches, input.to_string(), &numbers) {
                        Ok(result) => result,
                        Err(e) => {
                            eprintln!("Analysis error: {e}");
                            std::process::exit(1);
                        }
                    };

                // Output results and exit
                output_results(matches, &result);
                std::process::exit(result.risk_level.exit_code());
            }
            Err(e) => {
                eprintln!("Error processing input '{input}': {e}");
                std::process::exit(1);
            }
        }
    } else {
        // Read from stdin - use optimizations only if explicitly requested
        let use_optimize = matches.get_flag("optimize");

        if use_optimize {
            // 最適化処理：--optimize フラグ指定時（ストリーミング+並列+メモリ効率化）
            let mut reader = OptimizedFileReader::from_stdin();

            if std::env::var("LAWKIT_DEBUG").is_ok() {
                eprintln!("Debug: Using optimize mode (streaming + memory efficiency)");
            }

            let numbers = match reader
                .read_lines_streaming(|line| parse_text_input(&line).map(Some).or(Ok(None)))
            {
                Ok(nested_numbers) => nested_numbers.into_iter().flatten().collect::<Vec<_>>(),
                Err(e) => {
                    eprintln!("Analysis error: {e}");
                    std::process::exit(1);
                }
            };

            if numbers.is_empty() {
                eprintln!("Error: No valid numbers found in input");
                std::process::exit(1);
            }

            // 分析実行
            let result = match analyze_numbers_with_options(matches, "stdin".to_string(), &numbers)
            {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Analysis error: {e}");
                    std::process::exit(1);
                }
            };

            // Output results and exit
            output_results(matches, &result);
            std::process::exit(result.risk_level.exit_code());
        } else {
            // 従来のメモリ処理：デフォルト
            let mut buffer = String::new();
            match io::stdin().read_to_string(&mut buffer) {
                Ok(_) => {
                    if buffer.trim().is_empty() {
                        eprintln!("Error: No input provided. Use --help for usage information.");
                        std::process::exit(2);
                    }

                    // Extract numbers from stdin input text with international numeral support
                    let numbers = match parse_text_input(&buffer) {
                        Ok(numbers) => numbers,
                        Err(e) => {
                            eprintln!("Analysis error: {e}");
                            std::process::exit(1);
                        }
                    };

                    if numbers.is_empty() {
                        eprintln!("Error: No valid numbers found in input");
                        std::process::exit(1);
                    }

                    // Apply filtering and custom analysis
                    let result = match analyze_numbers_with_options(
                        matches,
                        "stdin".to_string(),
                        &numbers,
                    ) {
                        Ok(result) => result,
                        Err(e) => {
                            eprintln!("Analysis error: {e}");
                            std::process::exit(1);
                        }
                    };

                    // Output results and exit
                    output_results(matches, &result);
                    std::process::exit(result.risk_level.exit_code());
                }
                Err(e) => {
                    eprintln!("Error reading from stdin: {e}");
                    std::process::exit(1);
                }
            }
        }
    }
}

fn output_results(matches: &clap::ArgMatches, result: &BenfordResult) {
    let format = matches.get_one::<String>("format").unwrap();
    let quiet = matches.get_flag("quiet");
    let verbose = matches.get_flag("verbose");

    match format.as_str() {
        "text" => print_text_output(result, quiet, verbose),
        "json" => print_json_output(result),
        "csv" => print_csv_output(result),
        "yaml" => print_yaml_output(result),
        "toml" => print_toml_output(result),
        "xml" => print_xml_output(result),
        _ => {
            eprintln!("Error: Unsupported output format: {format}");
            std::process::exit(2);
        }
    }
}

fn print_text_output(result: &BenfordResult, quiet: bool, verbose: bool) {
    if quiet {
        for (i, &observed) in result.digit_distribution.iter().enumerate() {
            println!("{}: {:.1}%", i + 1, observed);
        }
        return;
    }

    println!("Benford Law Analysis Results");
    println!();
    println!("Dataset: {}", result.dataset_name);
    println!("Numbers analyzed: {}", result.numbers_analyzed);
    match result.risk_level {
        RiskLevel::Critical => println!("{}", colors::level_critical("Dataset analysis")),
        RiskLevel::High => println!("{}", colors::level_high("Dataset analysis")),
        RiskLevel::Medium => println!("{}", colors::level_medium("Dataset analysis")),
        RiskLevel::Low => println!("{}", colors::level_low("Dataset analysis")),
    }

    if verbose {
        println!();
        println!("First Digit Distribution:");
        for (i, &observed) in result.digit_distribution.iter().enumerate() {
            let digit = i + 1;
            let expected = result.expected_distribution[i];
            let deviation = observed - expected;

            println!(
                "{digit}: {observed:.1}% (expected: {expected:.1}%, deviation: {deviation:+.1}%)"
            );
        }

        println!();
        println!("Statistical Tests:");
        println!(
            "Chi-square: {:.2} (p-value: {:.6})",
            result.chi_square, result.p_value
        );
    }
}

fn print_json_output(result: &BenfordResult) {
    use serde_json::json;

    let output = json!({
        "dataset": result.dataset_name,
        "numbers_analyzed": result.numbers_analyzed,
        "risk_level": format!("{:?}", result.risk_level),
        "chi_square": result.chi_square,
        "p_value": result.p_value,
        "mean_absolute_deviation": result.mean_absolute_deviation
    });

    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

fn print_csv_output(result: &BenfordResult) {
    println!("dataset,numbers_analyzed,risk_level,chi_square,p_value,mad");
    println!(
        "{},{},{:?},{:.6},{:.6},{:.2}",
        result.dataset_name,
        result.numbers_analyzed,
        result.risk_level,
        result.chi_square,
        result.p_value,
        result.mean_absolute_deviation
    );
}

fn print_yaml_output(result: &BenfordResult) {
    println!("dataset: \"{}\"", result.dataset_name);
    println!("numbers_analyzed: {}", result.numbers_analyzed);
    println!("risk_level: \"{:?}\"", result.risk_level);
    println!("chi_square: {:.6}", result.chi_square);
    println!("p_value: {:.6}", result.p_value);
    println!("mad: {:.2}", result.mean_absolute_deviation);
}

fn print_toml_output(result: &BenfordResult) {
    println!("dataset = \"{}\"", result.dataset_name);
    println!("numbers_analyzed = {}", result.numbers_analyzed);
    println!("risk_level = \"{:?}\"", result.risk_level);
    println!("chi_square = {:.6}", result.chi_square);
    println!("p_value = {:.6}", result.p_value);
    println!("mad = {:.2}", result.mean_absolute_deviation);
}

fn print_xml_output(result: &BenfordResult) {
    println!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    println!("<benford_analysis>");
    println!("  <dataset>{}</dataset>", result.dataset_name);
    println!(
        "  <numbers_analyzed>{}</numbers_analyzed>",
        result.numbers_analyzed
    );
    println!("  <risk_level>{:?}</risk_level>", result.risk_level);
    println!("  <chi_square>{:.6}</chi_square>", result.chi_square);
    println!("  <p_value>{:.6}</p_value>", result.p_value);
    println!("  <mad>{:.2}</mad>", result.mean_absolute_deviation);
    println!("</benford_analysis>");
}

/// Analyze numbers with filtering and custom options
fn analyze_numbers_with_options(
    matches: &clap::ArgMatches,
    dataset_name: String,
    numbers: &[f64],
) -> Result<BenfordResult> {
    // Apply number filtering if specified
    let filtered_numbers = if let Some(filter_str) = matches.get_one::<String>("filter") {
        let filter = NumberFilter::parse(filter_str)
            .map_err(|e| BenfError::ParseError(format!("無効なフィルタ: {e}")))?;

        let filtered = apply_number_filter(numbers, &filter);

        // Inform user about filtering results
        if filtered.len() != numbers.len() {
            eprintln!(
                "フィルタリング結果: {} 個の数値が {} 個に絞り込まれました ({})",
                numbers.len(),
                filtered.len(),
                filter.description()
            );
        }

        filtered
    } else {
        numbers.to_vec()
    };

    // Parse custom threshold if specified
    let threshold = if let Some(threshold_str) = matches.get_one::<String>("threshold") {
        if threshold_str == "auto" {
            RiskThreshold::Auto
        } else {
            RiskThreshold::from_str(threshold_str)
                .map_err(|e| BenfError::ParseError(format!("無効な閾値: {e}")))?
        }
    } else {
        RiskThreshold::Auto
    };

    // Parse minimum count requirement
    let min_count = if let Some(min_count_str) = matches.get_one::<String>("min-count") {
        min_count_str
            .parse::<usize>()
            .map_err(|_| BenfError::ParseError("無効な最小数値数".to_string()))?
    } else {
        5
    };

    // Perform Benford analysis with custom options
    BenfordResult::new_with_threshold(dataset_name, &filtered_numbers, &threshold, min_count)
}
