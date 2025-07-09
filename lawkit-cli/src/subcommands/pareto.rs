use crate::colors;
use crate::common_options::{get_optimized_reader, setup_optimization_config};
use clap::ArgMatches;
use lawkit_core::{
    common::{
        filtering::{apply_number_filter, NumberFilter},
        input::parse_text_input,
        risk::RiskLevel,
    },
    error::{BenfError, Result},
    laws::pareto::{analyze_pareto_distribution, ParetoResult},
};

pub fn run(matches: &ArgMatches) -> Result<()> {
    // 最適化設定をセットアップ
    let (use_optimize, _parallel_config, _memory_config) = setup_optimization_config(matches);

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

    let dataset_name = matches
        .get_one::<String>("input")
        .map(|s| s.to_string())
        .unwrap_or_else(|| "stdin".to_string());

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

fn output_results(matches: &clap::ArgMatches, result: &ParetoResult) {
    let format = matches.get_one::<String>("format").unwrap();
    let quiet = matches.get_flag("quiet");
    let verbose = matches.get_flag("verbose");

    match format.as_str() {
        "text" => print_text_output(result, quiet, verbose, matches),
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

fn print_text_output(
    result: &ParetoResult,
    quiet: bool,
    verbose: bool,
    matches: &clap::ArgMatches,
) {
    if quiet {
        println!("pareto_ratio: {:.3}", result.pareto_ratio);
        println!("concentration_index: {:.3}", result.concentration_index);
        println!("top_20_percent_share: {:.1}%", result.top_20_percent_share);
        println!("gini_coefficient: {:.3}", result.concentration_index);
        return;
    }

    println!("Pareto Principle (80/20 Rule) Analysis Results");
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
        println!("Pareto Metrics:");
        println!("  Top 20% share: {:.1}%", result.top_20_percent_share);
        println!("  Pareto ratio: {:.3}", result.pareto_ratio);
        println!("  Concentration index: {:.3}", result.concentration_index);

        // カスタムパーセンタイルの表示
        if let Some(ref percentiles) = result.custom_percentiles {
            println!();
            println!("Custom Percentiles:");
            for (percentile, share) in percentiles {
                println!("  Top {percentile:.0}%: {share:.1}%");
            }
        }

        println!();
        println!("Interpretation:");
        print_pareto_interpretation(result);
    }

    // --gini-coefficient オプションが指定されたときにGini係数を明示的に表示
    if matches.get_flag("gini-coefficient") {
        println!();
        println!("Gini coefficient: {:.3}", result.concentration_index);
    }

    // --percentiles オプションが指定されたときは常に表示（verboseでなくても）
    if !verbose && result.custom_percentiles.is_some() {
        if let Some(ref percentiles) = result.custom_percentiles {
            println!();
            println!("Custom Percentiles:");
            for (percentile, share) in percentiles {
                println!("  Top {percentile:.0}%: {share:.1}%");
            }
        }
    }

    // --business-analysis オプションが指定されたときにビジネス分析を表示
    if matches.get_flag("business-analysis") {
        println!();
        println!("Business Analysis:");
        println!(
            "  Concentration level: {:.1}%",
            result.concentration_index * 100.0
        );
        println!("  Business efficiency: {:.1}%", result.pareto_ratio * 100.0);
        if result.top_20_percent_share > 80.0 {
            println!("  Recommendation: High concentration indicates good focus");
        } else {
            println!("  Recommendation: Consider focusing efforts on high-value activities");
        }
    }
}

fn print_pareto_interpretation(result: &ParetoResult) {
    use lawkit_core::common::risk::RiskLevel;

    match result.risk_level {
        RiskLevel::Low => {
            println!("{}", colors::level_pass("Ideal Pareto distribution detected"));
            println!("   80/20 principle is maintained");
        }
        RiskLevel::Medium => {
            println!("{}", colors::level_warn("Slight deviation from Pareto principle"));
            println!("   Monitoring recommended");
        }
        RiskLevel::High => {
            println!(
                "{}",
                colors::level_fail("Significant deviation from Pareto principle")
            );
            println!("   Rebalancing needed");
        }
        RiskLevel::Critical => {
            println!(
                "{}",
                colors::level_critical("Critical deviation from Pareto principle")
            );
            println!("   Strategy review needed");
        }
    }

    // 80/20原則からの偏差説明
    if result.top_20_percent_share > 85.0 {
        println!("   INFO: High concentration indicates good focus");
    } else if result.top_20_percent_share < 70.0 {
        println!("   ALERT: Low concentration suggests distribution inefficiency");
    }
}

fn print_json_output(result: &ParetoResult) {
    use serde_json::json;

    let mut output = json!({
        "dataset": result.dataset_name,
        "numbers_analyzed": result.numbers_analyzed,
        "risk_level": format!("{:?}", result.risk_level),
        "pareto_ratio": result.pareto_ratio,
        "concentration_index": result.concentration_index,
        "gini_coefficient": result.concentration_index,
        "top_20_percent_share": result.top_20_percent_share,
        "cumulative_distribution_points": result.cumulative_distribution.len()
    });

    // カスタムパーセンタイルがある場合は追加
    if let Some(ref percentiles) = result.custom_percentiles {
        output["custom_percentiles"] = json!(percentiles);
    }

    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

fn print_csv_output(result: &ParetoResult) {
    println!(
        "dataset,numbers_analyzed,risk_level,pareto_ratio,concentration_index,top_20_percent_share"
    );
    println!(
        "{},{},{:?},{:.3},{:.3},{:.1}",
        result.dataset_name,
        result.numbers_analyzed,
        result.risk_level,
        result.pareto_ratio,
        result.concentration_index,
        result.top_20_percent_share
    );
}

fn print_yaml_output(result: &ParetoResult) {
    println!("dataset: \"{}\"", result.dataset_name);
    println!("numbers_analyzed: {}", result.numbers_analyzed);
    println!("risk_level: \"{:?}\"", result.risk_level);
    println!("pareto_ratio: {:.3}", result.pareto_ratio);
    println!("concentration_index: {:.3}", result.concentration_index);
    println!("top_20_percent_share: {:.1}", result.top_20_percent_share);
}

fn print_toml_output(result: &ParetoResult) {
    println!("dataset = \"{}\"", result.dataset_name);
    println!("numbers_analyzed = {}", result.numbers_analyzed);
    println!("risk_level = \"{:?}\"", result.risk_level);
    println!("pareto_ratio = {:.3}", result.pareto_ratio);
    println!("concentration_index = {:.3}", result.concentration_index);
    println!("top_20_percent_share = {:.1}", result.top_20_percent_share);
}

fn print_xml_output(result: &ParetoResult) {
    println!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    println!("<pareto_analysis>");
    println!("  <dataset>{}</dataset>", result.dataset_name);
    println!(
        "  <numbers_analyzed>{}</numbers_analyzed>",
        result.numbers_analyzed
    );
    println!("  <risk_level>{:?}</risk_level>", result.risk_level);
    println!("  <pareto_ratio>{:.3}</pareto_ratio>", result.pareto_ratio);
    println!(
        "  <concentration_index>{:.3}</concentration_index>",
        result.concentration_index
    );
    println!(
        "  <top_20_percent_share>{:.1}</top_20_percent_share>",
        result.top_20_percent_share
    );
    println!("</pareto_analysis>");
}

/// Analyze numbers with filtering and custom options
fn analyze_numbers_with_options(
    matches: &clap::ArgMatches,
    dataset_name: String,
    numbers: &[f64],
) -> Result<ParetoResult> {
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

    // Perform Pareto analysis
    let mut result = analyze_pareto_distribution(&filtered_numbers, &dataset_name)?;

    // カスタムパーセンタイルの処理
    if let Some(percentiles_str) = matches.get_one::<String>("percentiles") {
        let percentiles: Vec<f64> = percentiles_str
            .split(',')
            .map(|s| s.trim().parse::<f64>())
            .collect::<std::result::Result<Vec<f64>, _>>()
            .map_err(|_| BenfError::ParseError("Invalid percentiles format".to_string()))?;

        result = result.with_custom_percentiles(&percentiles, &filtered_numbers);
    }

    Ok(result)
}
