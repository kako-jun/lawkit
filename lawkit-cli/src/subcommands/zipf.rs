use clap::ArgMatches;
use lawkit_core::{
    common::{
        filtering::{apply_number_filter, NumberFilter},
        input::{parse_input_auto, parse_text_input},
    },
    error::{BenfError, Result},
    laws::zipf::{analyze_numeric_zipf, analyze_text_zipf, ZipfResult},
};
use std::io::{self, Read};

pub fn run(matches: &ArgMatches) -> Result<()> {
    // Determine input source based on arguments
    if let Some(input) = matches.get_one::<String>("input") {
        // Check if input should be treated as text for word frequency analysis
        let is_text_mode = matches.get_flag("text");

        if is_text_mode {
            // Text analysis mode
            match analyze_text_zipf(input, input) {
                Ok(result) => {
                    output_results(matches, &result);
                    std::process::exit(result.risk_level.exit_code());
                }
                Err(e) => {
                    let language = get_language(matches);
                    let error_msg = localized_text("analysis_error", language);
                    eprintln!("{}: {}", error_msg, e);
                    std::process::exit(1);
                }
            }
        } else {
            // Numeric analysis mode
            match parse_input_auto(input) {
                Ok(numbers) => {
                    if numbers.is_empty() {
                        let language = get_language(matches);
                        let error_msg = localized_text("no_numbers_found", language);
                        eprintln!("{}", error_msg);
                        std::process::exit(1);
                    }

                    // Apply filtering and custom analysis
                    let result =
                        match analyze_numbers_with_options(matches, input.to_string(), &numbers) {
                            Ok(result) => result,
                            Err(e) => {
                                let language = get_language(matches);
                                let error_msg = localized_text("analysis_error", language);
                                eprintln!("{}: {}", error_msg, e);
                                std::process::exit(1);
                            }
                        };

                    // Output results and exit
                    output_results(matches, &result);
                    std::process::exit(result.risk_level.exit_code());
                }
                Err(e) => {
                    eprintln!("Error processing input '{}': {}", input, e);
                    std::process::exit(1);
                }
            }
        }
    } else {
        // Read from stdin
        let mut buffer = String::new();
        match io::stdin().read_to_string(&mut buffer) {
            Ok(_) => {
                if buffer.trim().is_empty() {
                    eprintln!("Error: No input provided. Use --help for usage information.");
                    std::process::exit(2);
                }

                let is_text_mode = matches.get_flag("text");

                if is_text_mode {
                    // Text analysis mode
                    match analyze_text_zipf(&buffer, "stdin") {
                        Ok(result) => {
                            output_results(matches, &result);
                            std::process::exit(result.risk_level.exit_code());
                        }
                        Err(e) => {
                            let language = get_language(matches);
                            let error_msg = localized_text("analysis_error", language);
                            eprintln!("{}: {}", error_msg, e);
                            std::process::exit(1);
                        }
                    }
                } else {
                    // Numeric analysis mode
                    let numbers = match parse_text_input(&buffer) {
                        Ok(numbers) => numbers,
                        Err(e) => {
                            let language = get_language(matches);
                            let error_msg = localized_text("analysis_error", language);
                            eprintln!("{}: {}", error_msg, e);
                            std::process::exit(1);
                        }
                    };

                    // Apply filtering and custom analysis
                    let result =
                        match analyze_numbers_with_options(matches, "stdin".to_string(), &numbers)
                        {
                            Ok(result) => result,
                            Err(e) => {
                                let language = get_language(matches);
                                let error_msg = localized_text("analysis_error", language);
                                eprintln!("{}: {}", error_msg, e);
                                std::process::exit(1);
                            }
                        };

                    // Output results and exit
                    output_results(matches, &result);
                    std::process::exit(result.risk_level.exit_code());
                }
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn output_results(matches: &clap::ArgMatches, result: &ZipfResult) {
    let format = matches.get_one::<String>("format").unwrap();
    let quiet = matches.get_flag("quiet");
    let verbose = matches.get_flag("verbose");
    let language = get_language(matches);

    match format.as_str() {
        "text" => print_text_output(result, quiet, verbose, language),
        "json" => print_json_output(result),
        "csv" => print_csv_output(result),
        "yaml" => print_yaml_output(result),
        "toml" => print_toml_output(result),
        "xml" => print_xml_output(result),
        _ => {
            let error_msg = localized_text("unsupported_format", language);
            eprintln!("{}: {}", error_msg, format);
            std::process::exit(2);
        }
    }
}

fn print_text_output(result: &ZipfResult, quiet: bool, verbose: bool, lang: &str) {
    if quiet {
        println!("zipf_exponent: {:.3}", result.zipf_exponent);
        println!("correlation: {:.3}", result.correlation_coefficient);
        println!("distribution_quality: {:.3}", result.distribution_quality);
        return;
    }

    println!("{}", localized_text("zipf_analysis_results", lang));
    println!();
    println!(
        "{}: {}",
        localized_text("dataset", lang),
        result.dataset_name
    );
    println!(
        "{}: {}",
        localized_text("numbers_analyzed", lang),
        result.numbers_analyzed
    );
    println!(
        "{}: {:?}",
        localized_text("risk_level", lang),
        result.risk_level
    );

    if verbose {
        println!();
        println!("{}:", localized_text("zipf_metrics", lang));
        println!(
            "  {}: {:.3}",
            localized_text("zipf_exponent", lang),
            result.zipf_exponent
        );
        println!(
            "  {}: {:.3}",
            localized_text("correlation_coefficient", lang),
            result.correlation_coefficient
        );
        println!(
            "  {}: {:.3}",
            localized_text("distribution_quality", lang),
            result.distribution_quality
        );
        println!(
            "  {}: {:.3}",
            localized_text("power_law_fit", lang),
            result.power_law_fit
        );

        println!();
        println!("{}:", localized_text("distribution_stats", lang));
        println!(
            "  {}: {}",
            localized_text("total_observations", lang),
            result.total_observations
        );
        println!(
            "  {}: {}",
            localized_text("unique_items", lang),
            result.unique_items
        );
        println!(
            "  {}: {:.1}%",
            localized_text("top_item_frequency", lang),
            result.top_item_frequency
        );
        println!(
            "  {}: {:.3}",
            localized_text("concentration_index", lang),
            result.concentration_index
        );
        println!(
            "  {}: {:.3}",
            localized_text("diversity_index", lang),
            result.diversity_index
        );

        println!();
        println!("{}:", localized_text("interpretation", lang));
        print_zipf_interpretation(result, lang);
    }
}

fn print_zipf_interpretation(result: &ZipfResult, lang: &str) {
    use lawkit_core::common::risk::RiskLevel;

    match result.risk_level {
        RiskLevel::Low => {
            println!("✅ {}", localized_text("ideal_zipf", lang));
            println!("   {}", localized_text("zipf_law_followed", lang));
        }
        RiskLevel::Medium => {
            println!("⚠️  {}", localized_text("slight_zipf_deviation", lang));
            println!("   {}", localized_text("zipf_monitoring_recommended", lang));
        }
        RiskLevel::High => {
            println!("🚨 {}", localized_text("significant_zipf_deviation", lang));
            println!("   {}", localized_text("zipf_rebalancing_needed", lang));
        }
        RiskLevel::Critical => {
            println!("🔍 {}", localized_text("critical_zipf_deviation", lang));
            println!("   {}", localized_text("zipf_strategy_review_needed", lang));
        }
    }

    // Zipf指数に基づく解釈
    if result.zipf_exponent > 1.5 {
        println!("   💡 {}", localized_text("high_concentration_zipf", lang));
    } else if result.zipf_exponent < 0.5 {
        println!("   💡 {}", localized_text("low_concentration_zipf", lang));
    }

    // 相関係数に基づく解釈
    if result.correlation_coefficient < 0.5 {
        println!("   📊 {}", localized_text("poor_zipf_fit", lang));
    } else if result.correlation_coefficient > 0.8 {
        println!("   📊 {}", localized_text("excellent_zipf_fit", lang));
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

fn get_language(matches: &clap::ArgMatches) -> &str {
    match matches.get_one::<String>("language").map(|s| s.as_str()) {
        Some("auto") | None => {
            let lang = std::env::var("LANG").unwrap_or_default();
            if lang.starts_with("ja") {
                "ja"
            } else if lang.starts_with("zh") {
                "zh"
            } else if lang.starts_with("hi") {
                "hi"
            } else if lang.starts_with("ar") {
                "ar"
            } else {
                "en"
            }
        }
        Some("en") => "en",
        Some("ja") => "ja",
        Some("zh") => "zh",
        Some("hi") => "hi",
        Some("ar") => "ar",
        Some(_) => "en",
    }
}

fn localized_text(key: &str, lang: &str) -> &'static str {
    match (lang, key) {
        // English
        ("en", "zipf_analysis_results") => "Zipf's Law Analysis Results",
        ("en", "dataset") => "Dataset",
        ("en", "numbers_analyzed") => "Numbers analyzed",
        ("en", "risk_level") => "Attention Level",
        ("en", "zipf_metrics") => "Zipf Metrics",
        ("en", "zipf_exponent") => "Zipf exponent",
        ("en", "correlation_coefficient") => "Correlation coefficient",
        ("en", "distribution_quality") => "Distribution quality",
        ("en", "power_law_fit") => "Power law fit",
        ("en", "distribution_stats") => "Distribution Statistics",
        ("en", "total_observations") => "Total observations",
        ("en", "unique_items") => "Unique items",
        ("en", "top_item_frequency") => "Top item frequency",
        ("en", "concentration_index") => "Concentration index",
        ("en", "diversity_index") => "Diversity index (Shannon)",
        ("en", "interpretation") => "Interpretation",
        ("en", "ideal_zipf") => "Ideal Zipf distribution - follows Zipf's law",
        ("en", "zipf_law_followed") => "Distribution follows the expected 1/rank pattern",
        ("en", "slight_zipf_deviation") => "Slight deviation from Zipf's law",
        ("en", "zipf_monitoring_recommended") => "Monitoring recommended for distribution pattern",
        ("en", "significant_zipf_deviation") => "Significant deviation from Zipf's law",
        ("en", "zipf_rebalancing_needed") => "Consider rebalancing distribution",
        ("en", "critical_zipf_deviation") => "Critical deviation from Zipf's law",
        ("en", "zipf_strategy_review_needed") => "Distribution strategy review needed",
        ("en", "high_concentration_zipf") => "High concentration - extreme dominance pattern",
        ("en", "low_concentration_zipf") => "Low concentration - more uniform distribution",
        ("en", "poor_zipf_fit") => "Poor fit to Zipf's law - irregular distribution",
        ("en", "excellent_zipf_fit") => "Excellent fit to Zipf's law",
        ("en", "unsupported_format") => "Error: Unsupported output format",
        ("en", "no_numbers_found") => "Error: No valid numbers found in input",
        ("en", "analysis_error") => "Analysis error",

        // 日本語
        ("ja", "zipf_analysis_results") => "ジップの法則解析結果",
        ("ja", "dataset") => "データセット",
        ("ja", "numbers_analyzed") => "解析した数値数",
        ("ja", "risk_level") => "注意レベル",
        ("ja", "zipf_metrics") => "ジップ指標",
        ("ja", "zipf_exponent") => "ジップ指数",
        ("ja", "correlation_coefficient") => "相関係数",
        ("ja", "distribution_quality") => "分布品質",
        ("ja", "power_law_fit") => "べき乗法則適合度",
        ("ja", "distribution_stats") => "分布統計",
        ("ja", "total_observations") => "総観測数",
        ("ja", "unique_items") => "ユニーク項目数",
        ("ja", "top_item_frequency") => "最頻項目出現率",
        ("ja", "concentration_index") => "集中度指数",
        ("ja", "diversity_index") => "多様性指数（シャノン）",
        ("ja", "interpretation") => "解釈",
        ("ja", "ideal_zipf") => "理想的なジップ分布 - ジップの法則に従っています",
        ("ja", "zipf_law_followed") => "分布は期待される1/rankパターンに従っています",
        ("ja", "slight_zipf_deviation") => "ジップの法則からの軽微な偏差",
        ("ja", "zipf_monitoring_recommended") => "分布パターンの監視を推奨",
        ("ja", "significant_zipf_deviation") => "ジップの法則からの有意な偏差",
        ("ja", "zipf_rebalancing_needed") => "分布の再バランスを検討",
        ("ja", "critical_zipf_deviation") => "ジップの法則からの重大な偏差",
        ("ja", "zipf_strategy_review_needed") => "分布戦略の見直しが必要",
        ("ja", "high_concentration_zipf") => "高集中度 - 極端な優位性パターン",
        ("ja", "low_concentration_zipf") => "低集中度 - より均等な分布",
        ("ja", "poor_zipf_fit") => "ジップの法則への適合度が低い - 不規則な分布",
        ("ja", "excellent_zipf_fit") => "ジップの法則への優れた適合",
        ("ja", "unsupported_format") => "エラー: サポートされていない出力形式",
        ("ja", "no_numbers_found") => "エラー: 入力に有効な数値が見つかりません",
        ("ja", "analysis_error") => "解析エラー",

        // Default English
        (_, "zipf_analysis_results") => "Zipf's Law Analysis Results",
        (_, "dataset") => "Dataset",
        (_, "numbers_analyzed") => "Numbers analyzed",
        (_, "risk_level") => "Attention Level",
        (_, "unsupported_format") => "Error: Unsupported output format",
        (_, "no_numbers_found") => "Error: No valid numbers found in input",
        (_, "analysis_error") => "Analysis error",
        (_, _) => "Unknown message",
    }
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
            .map_err(|e| BenfError::ParseError(format!("無効なフィルタ: {}", e)))?;

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
