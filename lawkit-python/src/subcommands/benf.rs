use crate::colors;
use clap::ArgMatches;
use lawkit_core::{
    common::{
        filtering::{apply_number_filter, NumberFilter, RiskThreshold},
        input::{parse_input_auto, parse_text_input},
        memory::{streaming_benford_analysis, MemoryConfig},
        risk::RiskLevel,
        streaming_io::OptimizedFileReader,
    },
    error::{BenfError, Result},
    laws::benford::BenfordResult,
};
use std::str::FromStr;

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
        let numbers = match reader.read_lines_streaming(|line| {
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

        // Apply minimum value filter for streaming analysis
        let filtered_numbers = if let Some(min_value_str) = matches.get_one::<String>("min-value") {
            let min_val = min_value_str
                .parse::<f64>()
                .map_err(|_| {
                    eprintln!("Error: Invalid minimum value");
                    std::process::exit(2);
                })
                .unwrap();

            let original_len = numbers.len();
            let filtered: Vec<f64> = numbers.into_iter().filter(|&x| x >= min_val).collect();

            if matches.get_flag("verbose") {
                eprintln!(
                    "Debug: Min-value filter applied: {} → {} numbers (>= {})",
                    original_len,
                    filtered.len(),
                    min_val
                );
                eprintln!(
                    "Debug: Filter removed {} values ({:.1}%)",
                    original_len - filtered.len(),
                    100.0 * (original_len - filtered.len()) as f64 / original_len as f64
                );
            }
            filtered
        } else {
            numbers
        };

        // メモリ設定を作成
        let memory_config = MemoryConfig::default();

        // インクリメンタルストリーミング分析を実行
        let chunk_result =
            match streaming_benford_analysis(filtered_numbers.into_iter(), &memory_config) {
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

        if chunk_result.total_items == 0 {
            if matches.get_flag("verbose") {
                eprintln!(
                    "Debug: Total items in chunk_result: {}",
                    chunk_result.total_items
                );
            }
            eprintln!("Error: No valid numbers found in input");
            std::process::exit(1);
        }

        // IncrementalBenford を BenfordResult に変換
        let benford_result =
            convert_incremental_to_result(&chunk_result.result, "stdin".to_string(), matches);

        // デバッグ情報を出力
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

        // Output results and exit
        output_results(matches, &benford_result);
        std::process::exit(benford_result.risk_level.exit_code());
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

    println!();
    println!("First Digit Distribution:");
    println!("{}", format_distribution_bars(result));

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

    // Parse confidence level
    let _confidence = if let Some(confidence_str) = matches.get_one::<String>("confidence") {
        let conf = confidence_str
            .parse::<f64>()
            .map_err(|_| BenfError::ParseError("無効な信頼度レベル".to_string()))?;
        if !(0.01..=0.99).contains(&conf) {
            return Err(BenfError::ParseError(
                "信頼度レベルは0.01から0.99の間である必要があります".to_string(),
            ));
        }
        conf
    } else {
        0.95
    };

    // Parse sample size limit
    let mut working_numbers = filtered_numbers.clone();
    if let Some(sample_size_str) = matches.get_one::<String>("sample-size") {
        let max_size = sample_size_str
            .parse::<usize>()
            .map_err(|_| BenfError::ParseError("無効なサンプルサイズ".to_string()))?;

        if working_numbers.len() > max_size {
            eprintln!(
                "大規模データセット: {}個の数値を{}個にサンプリングしました",
                working_numbers.len(),
                max_size
            );
            // Simple random sampling by taking every nth element
            let step = working_numbers.len() / max_size;
            working_numbers = working_numbers
                .iter()
                .step_by(step.max(1))
                .cloned()
                .take(max_size)
                .collect();
        }
    }

    // Apply minimum value filter
    if let Some(min_value_str) = matches.get_one::<String>("min-value") {
        let min_val = min_value_str
            .parse::<f64>()
            .map_err(|_| BenfError::ParseError("無効な最小値".to_string()))?;

        let original_len = working_numbers.len();
        working_numbers.retain(|&x| x >= min_val);

        if working_numbers.len() != original_len {
            if matches.get_flag("verbose") {
                eprintln!(
                    "Debug: Min-value filter applied: {} → {} numbers (>= {})",
                    original_len,
                    working_numbers.len(),
                    min_val
                );
                eprintln!(
                    "Debug: Filter removed {} values ({:.1}%)",
                    original_len - working_numbers.len(),
                    100.0 * (original_len - working_numbers.len()) as f64 / original_len as f64
                );
            } else {
                eprintln!(
                    "最小値フィルタ適用: {}個の数値が{}個に絞り込まれました (>= {})",
                    original_len,
                    working_numbers.len(),
                    min_val
                );
            }
        }
    }

    // Perform Benford analysis with custom options
    BenfordResult::new_with_threshold(dataset_name, &working_numbers, &threshold, min_count)
}

/// IncrementalBenford を BenfordResult に変換
fn convert_incremental_to_result(
    incremental: &lawkit_core::common::memory::IncrementalBenford,
    dataset_name: String,
    _matches: &clap::ArgMatches,
) -> BenfordResult {
    use lawkit_core::common::statistics;

    // 分布データを取得
    let digit_distribution = incremental.get_distribution();
    let expected_distribution = [
        30.103, 17.609, 12.494, 9.691, 7.918, 6.695, 5.799, 5.115, 4.576,
    ];

    // 統計値を計算
    let chi_square = statistics::calculate_chi_square(&digit_distribution, &expected_distribution);
    let p_value = statistics::calculate_p_value(chi_square, 8);
    let mean_absolute_deviation = incremental.calculate_mad();

    // リスクレベルを決定
    let risk_level = determine_risk_level(mean_absolute_deviation, p_value);

    // 判定を生成
    let verdict = format!("Risk Level: {risk_level:?}");

    BenfordResult {
        dataset_name,
        numbers_analyzed: incremental.total_count(),
        digit_distribution,
        expected_distribution,
        chi_square,
        p_value,
        mean_absolute_deviation,
        risk_level,
        verdict,
    }
}

/// リスクレベルを決定
fn determine_risk_level(mad: f64, p_value: f64) -> RiskLevel {
    if mad > 15.0 || p_value < 0.01 {
        RiskLevel::Critical
    } else if mad > 10.0 || p_value < 0.05 {
        RiskLevel::High
    } else if mad > 5.0 || p_value < 0.10 {
        RiskLevel::Medium
    } else {
        RiskLevel::Low
    }
}

fn format_distribution_bars(result: &BenfordResult) -> String {
    let mut output = String::new();
    const BAR_WIDTH: usize = 40;

    for i in 0..9 {
        let digit = i + 1;
        let observed = result.digit_distribution[i];
        let expected = result.expected_distribution[i];
        let bar_length = ((observed / 100.0) * BAR_WIDTH as f64).round() as usize;
        let bar_length = bar_length.min(BAR_WIDTH); // Ensure we don't exceed max width

        // Create bar with filled and background portions
        let filled_bar = "█".repeat(bar_length);
        let background_bar = "░".repeat(BAR_WIDTH - bar_length);
        let full_bar = format!("{filled_bar}{background_bar}");

        output.push_str(&format!(
            "{digit:1}: {full_bar} {observed:>5.1}% (expected: {expected:>5.1}%)\n"
        ));
    }

    output
}
