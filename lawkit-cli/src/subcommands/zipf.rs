use crate::common_options::{get_optimized_reader, setup_optimization_config};
use clap::ArgMatches;
use lawkit_core::{
    common::{
        filtering::{apply_number_filter, NumberFilter},
        input::parse_text_input,
    },
    error::{BenfError, Result},
    laws::zipf::{analyze_numeric_zipf, analyze_text_zipf, ZipfResult},
};

pub fn run(matches: &ArgMatches) -> Result<()> {
    // 最適化設定をセットアップ
    let (use_optimize, _parallel_config, _memory_config) = setup_optimization_config(matches);
    let is_text_mode = matches.get_flag("text");

    // 最適化された入力読み込み
    let input_data = if let Some(input) = matches.get_one::<String>("input") {
        if input == "-" {
            get_optimized_reader(None, use_optimize)
        } else {
            get_optimized_reader(Some(input), use_optimize)
        }
    } else {
        get_optimized_reader(None, use_optimize)
    };

    let buffer = match input_data {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading input: {e}");
            std::process::exit(1);
        }
    };

    if buffer.trim().is_empty() {
        eprintln!("Error: No input provided. Use --help for usage information.");
        std::process::exit(2);
    }

    let dataset_name = matches
        .get_one::<String>("input")
        .map(|s| s.to_string())
        .unwrap_or_else(|| "stdin".to_string());

    if is_text_mode {
        // Text analysis mode
        match analyze_text_zipf(&buffer, &dataset_name) {
            Ok(result) => {
                output_results(matches, &result);
                std::process::exit(result.risk_level.exit_code());
            }
            Err(e) => {
                eprintln!("Analysis error: {e}");
                std::process::exit(1);
            }
        }
    } else {
        // Numeric analysis mode
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

        let result = match analyze_numbers_with_options(matches, dataset_name, &numbers) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Analysis error: {e}");
                std::process::exit(1);
            }
        };

        output_results(matches, &result);
        std::process::exit(result.risk_level.exit_code())
    }
}

fn output_results(matches: &clap::ArgMatches, result: &ZipfResult) {
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

fn print_text_output(result: &ZipfResult, quiet: bool, verbose: bool) {
    if quiet {
        println!("zipf_exponent: {:.3}", result.zipf_exponent);
        println!("correlation: {:.3}", result.correlation_coefficient);
        println!("distribution_quality: {:.3}", result.distribution_quality);
        return;
    }

    println!("Zipf's Law Analysis Results");
    println!();
    println!("Dataset: {}", result.dataset_name);
    println!("Numbers analyzed: {}", result.numbers_analyzed);
    println!("Attention Level: {:?}", result.risk_level);

    if verbose {
        println!();
        println!("Zipf Metrics:");
        println!("  Zipf exponent: {:.3}", result.zipf_exponent);
        println!(
            "  Correlation coefficient: {:.3}",
            result.correlation_coefficient
        );
        println!("  Distribution quality: {:.3}", result.distribution_quality);
        println!("  Power law fit: {:.3}", result.power_law_fit);

        println!();
        println!("Distribution Statistics:");
        println!("  Total observations: {}", result.total_observations);
        println!("  Unique items: {}", result.unique_items);
        println!("  Top item frequency: {:.1}%", result.top_item_frequency);
        println!("  Concentration index: {:.3}", result.concentration_index);
        println!("  Diversity index (Shannon): {:.3}", result.diversity_index);

        println!();
        println!("Interpretation:");
        print_zipf_interpretation(result);
    }
}

fn print_zipf_interpretation(result: &ZipfResult) {
    use lawkit_core::common::risk::RiskLevel;

    match result.risk_level {
        RiskLevel::Low => {
            println!("[PASS] Ideal Zipf distribution - follows Zipf's law");
            println!("   Distribution follows the expected 1/rank pattern");
        }
        RiskLevel::Medium => {
            println!("[WARN] Slight deviation from Zipf's law");
            println!("   Monitoring recommended for distribution pattern");
        }
        RiskLevel::High => {
            println!("[FAIL] Significant deviation from Zipf's law");
            println!("   Consider rebalancing distribution");
        }
        RiskLevel::Critical => {
            println!("[CRITICAL] Critical deviation from Zipf's law");
            println!("   Distribution strategy review needed");
        }
    }

    // Zipf指数に基づく解釈
    if result.zipf_exponent > 1.5 {
        println!("   INFO: High concentration - extreme dominance pattern");
    } else if result.zipf_exponent < 0.5 {
        println!("   INFO: Low concentration - more uniform distribution");
    }

    // 相関係数に基づく解釈
    if result.correlation_coefficient < 0.5 {
        println!("   ALERT: Poor fit to Zipf's law - irregular distribution");
    } else if result.correlation_coefficient > 0.8 {
        println!("   INFO: Excellent fit to Zipf's law");
    }
}

fn print_json_output(result: &ZipfResult) {
    use serde_json::json;

    let output = json!({
        "dataset": result.dataset_name,
        "numbers_analyzed": result.numbers_analyzed,
        "risk_level": format!("{:?}", result.risk_level),
        "zipf_exponent": result.zipf_exponent,
        "correlation_coefficient": result.correlation_coefficient,
        "distribution_quality": result.distribution_quality,
        "total_observations": result.total_observations,
        "unique_items": result.unique_items,
        "top_item_frequency": result.top_item_frequency,
        "concentration_index": result.concentration_index,
        "diversity_index": result.diversity_index,
        "power_law_fit": result.power_law_fit,
        "rank_frequency_pairs": result.rank_frequency_pairs
    });

    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

fn print_csv_output(result: &ZipfResult) {
    println!("dataset,numbers_analyzed,risk_level,zipf_exponent,correlation_coefficient,distribution_quality,power_law_fit");
    println!(
        "{},{},{:?},{:.3},{:.3},{:.3},{:.3}",
        result.dataset_name,
        result.numbers_analyzed,
        result.risk_level,
        result.zipf_exponent,
        result.correlation_coefficient,
        result.distribution_quality,
        result.power_law_fit
    );
}

fn print_yaml_output(result: &ZipfResult) {
    println!("dataset: \"{}\"", result.dataset_name);
    println!("numbers_analyzed: {}", result.numbers_analyzed);
    println!("risk_level: \"{:?}\"", result.risk_level);
    println!("zipf_exponent: {:.3}", result.zipf_exponent);
    println!(
        "correlation_coefficient: {:.3}",
        result.correlation_coefficient
    );
    println!("distribution_quality: {:.3}", result.distribution_quality);
    println!("power_law_fit: {:.3}", result.power_law_fit);
}

fn print_toml_output(result: &ZipfResult) {
    println!("dataset = \"{}\"", result.dataset_name);
    println!("numbers_analyzed = {}", result.numbers_analyzed);
    println!("risk_level = \"{:?}\"", result.risk_level);
    println!("zipf_exponent = {:.3}", result.zipf_exponent);
    println!(
        "correlation_coefficient = {:.3}",
        result.correlation_coefficient
    );
    println!("distribution_quality = {:.3}", result.distribution_quality);
    println!("power_law_fit = {:.3}", result.power_law_fit);
}

fn print_xml_output(result: &ZipfResult) {
    println!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    println!("<zipf_analysis>");
    println!("  <dataset>{}</dataset>", result.dataset_name);
    println!(
        "  <numbers_analyzed>{}</numbers_analyzed>",
        result.numbers_analyzed
    );
    println!("  <risk_level>{:?}</risk_level>", result.risk_level);
    println!(
        "  <zipf_exponent>{:.3}</zipf_exponent>",
        result.zipf_exponent
    );
    println!(
        "  <correlation_coefficient>{:.3}</correlation_coefficient>",
        result.correlation_coefficient
    );
    println!(
        "  <distribution_quality>{:.3}</distribution_quality>",
        result.distribution_quality
    );
    println!(
        "  <power_law_fit>{:.3}</power_law_fit>",
        result.power_law_fit
    );
    println!("</zipf_analysis>");
}

/// Analyze numbers with filtering and custom options
fn analyze_numbers_with_options(
    matches: &clap::ArgMatches,
    dataset_name: String,
    numbers: &[f64],
) -> Result<ZipfResult> {
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

    // Parse minimum count requirement
    let min_count = if let Some(min_count_str) = matches.get_one::<String>("min-count") {
        min_count_str
            .parse::<usize>()
            .map_err(|_| BenfError::ParseError("無効な最小数値数".to_string()))?
    } else {
        5
    };

    // Check minimum count requirement
    if filtered_numbers.len() < min_count {
        return Err(BenfError::InsufficientData(filtered_numbers.len()));
    }

    // Perform Zipf analysis
    analyze_numeric_zipf(&filtered_numbers, &dataset_name)
}
