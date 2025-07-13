use crate::colors;
// Removed unused imports: get_optimized_reader, setup_automatic_optimization_config
use clap::ArgMatches;
use lawkit_core::{
    common::{
        filtering::{apply_number_filter, NumberFilter},
        input::{parse_input_auto, parse_text_input},
        memory::{streaming_pareto_analysis, MemoryConfig},
        risk::RiskLevel,
        streaming_io::OptimizedFileReader,
    },
    error::{BenfError, Result},
    laws::pareto::{analyze_pareto_distribution, ParetoResult},
};

pub fn run(matches: &ArgMatches) -> Result<()> {
    // Determine input source based on arguments
    if matches.get_flag("verbose") {
        eprintln!(
            "Debug: input argument = {:?}",
            matches.get_one::<String>("input")
        );
    }

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
        // Read from stdin - use automatic optimization based on data characteristics
        if matches.get_flag("verbose") {
            eprintln!("Debug: Reading from stdin, using automatic optimization");
        }

        // 自動最適化処理：データ特性に基づいてストリーミング処理を自動選択
        let mut reader = OptimizedFileReader::from_stdin();

        if matches.get_flag("verbose") {
            eprintln!(
                "Debug: Using automatic optimization (streaming + incremental + memory efficiency)"
            );
        }

        // ストリーミング処理でインクリメンタル分析を実行
        let numbers = match reader.read_lines_streaming(|line: String| {
            if matches.get_flag("verbose") {
                eprintln!("Debug: Processing line: '{line}'");
            }
            parse_text_input(&line).map(Some).or(Ok(None))
        }) {
            Ok(nested_numbers) => {
                let flattened: Vec<f64> = nested_numbers.into_iter().flatten().collect();
                if matches.get_flag("verbose") {
                    eprintln!("Debug: Collected {} numbers from stream", flattened.len());
                }
                flattened
            }
            Err(e) => {
                eprintln!("Analysis error: {e}");
                std::process::exit(1);
            }
        };

        if numbers.is_empty() {
            eprintln!("Error: No valid numbers found in input");
            std::process::exit(1);
        }

        // メモリ設定を作成
        let memory_config = MemoryConfig::default();

        // インクリメンタルストリーミング分析を実行
        let chunk_result = match streaming_pareto_analysis(numbers.into_iter(), &memory_config) {
            Ok(result) => {
                if matches.get_flag("verbose") {
                    eprintln!(
                        "Debug: Streaming analysis successful - {} items processed",
                        result.total_items
                    );
                }
                result
            }
            Err(e) => {
                eprintln!("Streaming analysis error: {e}");
                std::process::exit(1);
            }
        };

        if matches.get_flag("verbose") {
            eprintln!(
                "Debug: Processed {} numbers in {} chunks",
                chunk_result.total_items, chunk_result.chunks_processed
            );
            eprintln!("Debug: Memory used: {:.2} MB", chunk_result.memory_used_mb);
            eprintln!(
                "Debug: Processing time: {} ms",
                chunk_result.processing_time_ms
            );
        }

        // IncrementalParetoから通常のパレート結果に変換
        let mut incremental_pareto = chunk_result.result;
        let sorted_values = incremental_pareto.get_sorted_values().to_vec();

        // パレート分析を実行
        let result =
            match analyze_numbers_with_options(matches, "stdin".to_string(), &sorted_values) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Analysis error: {e}");
                    std::process::exit(1);
                }
            };

        // 結果出力
        output_results(matches, &result);
        std::process::exit(result.risk_level.exit_code());
    }
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

    println!();
    println!("Lorenz Curve (Cumulative Distribution):");
    println!("{}", format_lorenz_curve(result));

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
            println!(
                "{}",
                colors::level_pass("Ideal Pareto distribution detected")
            );
            println!("   80/20 principle is maintained");
        }
        RiskLevel::Medium => {
            println!(
                "{}",
                colors::level_warn("Slight deviation from Pareto principle")
            );
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

fn format_lorenz_curve(result: &ParetoResult) -> String {
    let mut output = String::new();
    const CHART_WIDTH: usize = 50;
    const CHART_HEIGHT: usize = 10;

    // ローレンツ曲線の主要ポイントを取得（グラフ用に簡略化）
    let curve_points = &result.cumulative_distribution;
    let total_points = curve_points.len();

    // 10点のサンプルポイントを選択
    let sample_indices: Vec<usize> = (0..CHART_HEIGHT)
        .map(|i| ((i + 1) * total_points) / (CHART_HEIGHT + 1))
        .collect();

    for &sample_idx in sample_indices.iter() {
        let sample_idx = sample_idx.min(total_points - 1);
        let (population_pct, wealth_pct) = curve_points[sample_idx];

        // X軸：人口パーセンタイル（0-100%）
        // Y軸：富のパーセンタイル（0-100%）
        let bar_length = (wealth_pct * CHART_WIDTH as f64).round() as usize;
        let bar_length = bar_length.min(CHART_WIDTH);

        let filled_bar = "█".repeat(bar_length);
        let background_bar = "░".repeat(CHART_WIDTH - bar_length);
        let full_bar = format!("{filled_bar}{background_bar}");

        // パーセンタイルごとに表示
        let pop_percent = population_pct * 100.0;
        let wealth_percent = wealth_pct * 100.0;

        output.push_str(&format!(
            "{pop_percent:3.0}%: {full_bar} {wealth_percent:>5.1}% cumulative\n"
        ));
    }

    // 80/20ラインの表示
    output.push_str(&format!(
        "\n80/20 Rule: Top 20% owns {:.1}% of total wealth",
        result.top_20_percent_share
    ));
    output.push_str(&format!(
        " (Ideal: 80.0%, Ratio: {:.2})",
        result.pareto_ratio
    ));

    output
}
