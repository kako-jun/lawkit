use clap::ArgMatches;
use lawkit_core::{
    common::{
        input::{parse_input_auto, parse_text_input},
        filtering::{NumberFilter, apply_number_filter},
    },
    laws::pareto::{analyze_pareto_distribution, analyze_business_pareto, ParetoResult},
    error::{BenfError, Result},
};
use std::io::{self, Read};

pub fn run(matches: &ArgMatches) -> Result<()> {
    // Determine input source based on arguments
    if let Some(input) = matches.get_one::<String>("input") {
        // Use auto-detection for file vs string input
        match parse_input_auto(input) {
            Ok(numbers) => {
                if numbers.is_empty() {
                    let language = get_language(&matches);
                    let error_msg = localized_text("no_numbers_found", language);
                    eprintln!("{}", error_msg);
                    std::process::exit(1);
                }
                
                // Apply filtering and custom analysis
                let result = match analyze_numbers_with_options(&matches, input.to_string(), &numbers) {
                    Ok(result) => result,
                    Err(e) => {
                        let language = get_language(&matches);
                        let error_msg = localized_text("analysis_error", language);
                        eprintln!("{}: {}", error_msg, e);
                        std::process::exit(1);
                    }
                };

                // Output results and exit
                output_results(&matches, &result);
                std::process::exit(result.risk_level.exit_code());
            }
            Err(e) => {
                eprintln!("Error processing input '{}': {}", input, e);
                std::process::exit(1);
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
                
                // Extract numbers from stdin input text with international numeral support
                let numbers = match parse_text_input(&buffer) {
                    Ok(numbers) => numbers,
                    Err(e) => {
                        let language = get_language(&matches);
                        let error_msg = localized_text("analysis_error", language);
                        eprintln!("{}: {}", error_msg, e);
                        std::process::exit(1);
                    }
                };
                
                // Apply filtering and custom analysis
                let result = match analyze_numbers_with_options(&matches, "stdin".to_string(), &numbers) {
                    Ok(result) => result,
                    Err(e) => {
                        let language = get_language(&matches);
                        let error_msg = localized_text("analysis_error", language);
                        eprintln!("{}: {}", error_msg, e);
                        std::process::exit(1);
                    }
                };

                // Output results and exit
                output_results(&matches, &result);
                std::process::exit(result.risk_level.exit_code());
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn output_results(matches: &clap::ArgMatches, result: &ParetoResult) {
    let format = matches.get_one::<String>("format").unwrap();
    let quiet = matches.get_flag("quiet");
    let verbose = matches.get_flag("verbose");
    let language = get_language(&matches);

    match format.as_str() {
        "text" => print_text_output(&result, quiet, verbose, language),
        "json" => print_json_output(&result),
        "csv" => print_csv_output(&result),
        "yaml" => print_yaml_output(&result),
        "toml" => print_toml_output(&result),
        "xml" => print_xml_output(&result),
        _ => {
            let error_msg = localized_text("unsupported_format", language);
            eprintln!("{}: {}", error_msg, format);
            std::process::exit(2);
        }
    }
}

fn print_text_output(result: &ParetoResult, quiet: bool, verbose: bool, lang: &str) {
    if quiet {
        println!("pareto_ratio: {:.3}", result.pareto_ratio);
        println!("concentration_index: {:.3}", result.concentration_index);
        println!("top_20_percent_share: {:.1}%", result.top_20_percent_share);
        return;
    }

    println!("{}", localized_text("pareto_analysis_results", lang));
    println!();
    println!("{}: {}", localized_text("dataset", lang), result.dataset_name);
    println!("{}: {}", localized_text("numbers_analyzed", lang), result.numbers_analyzed);
    println!("{}: {:?}", localized_text("risk_level", lang), result.risk_level);
    
    if verbose {
        println!();
        println!("{}:", localized_text("pareto_metrics", lang));
        println!("  {}: {:.1}%", localized_text("top_20_percent_share", lang), result.top_20_percent_share);
        println!("  {}: {:.3}", localized_text("pareto_ratio", lang), result.pareto_ratio);
        println!("  {}: {:.3}", localized_text("concentration_index", lang), result.concentration_index);
        
        println!();
        println!("{}:", localized_text("interpretation", lang));
        print_pareto_interpretation(result, lang);
    }
}

fn print_pareto_interpretation(result: &ParetoResult, lang: &str) {
    use lawkit_core::common::risk::RiskLevel;
    
    match result.risk_level {
        RiskLevel::Low => {
            println!("✅ {}", localized_text("ideal_pareto", lang));
            println!("   {}", localized_text("pareto_80_20_maintained", lang));
        },
        RiskLevel::Medium => {
            println!("⚠️  {}", localized_text("slight_pareto_deviation", lang));
            println!("   {}", localized_text("pareto_monitoring_recommended", lang));
        },
        RiskLevel::High => {
            println!("🚨 {}", localized_text("significant_pareto_deviation", lang));
            println!("   {}", localized_text("pareto_rebalancing_needed", lang));
        },
        RiskLevel::Critical => {
            println!("🔍 {}", localized_text("critical_pareto_deviation", lang));
            println!("   {}", localized_text("pareto_strategy_review_needed", lang));
        }
    }
    
    // 80/20原則からの偏差説明
    if result.top_20_percent_share > 85.0 {
        println!("   💡 {}", localized_text("high_concentration_insight", lang));
    } else if result.top_20_percent_share < 70.0 {
        println!("   💡 {}", localized_text("low_concentration_insight", lang));
    }
}

fn print_json_output(result: &ParetoResult) {
    use serde_json::json;
    
    let output = json!({
        "dataset": result.dataset_name,
        "numbers_analyzed": result.numbers_analyzed,
        "risk_level": format!("{:?}", result.risk_level),
        "pareto_ratio": result.pareto_ratio,
        "concentration_index": result.concentration_index,
        "top_20_percent_share": result.top_20_percent_share,
        "cumulative_distribution_points": result.cumulative_distribution.len()
    });
    
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

fn print_csv_output(result: &ParetoResult) {
    println!("dataset,numbers_analyzed,risk_level,pareto_ratio,concentration_index,top_20_percent_share");
    println!("{},{},{:?},{:.3},{:.3},{:.1}",
             result.dataset_name,
             result.numbers_analyzed,
             result.risk_level,
             result.pareto_ratio,
             result.concentration_index,
             result.top_20_percent_share);
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
    println!("  <numbers_analyzed>{}</numbers_analyzed>", result.numbers_analyzed);
    println!("  <risk_level>{:?}</risk_level>", result.risk_level);
    println!("  <pareto_ratio>{:.3}</pareto_ratio>", result.pareto_ratio);
    println!("  <concentration_index>{:.3}</concentration_index>", result.concentration_index);
    println!("  <top_20_percent_share>{:.1}</top_20_percent_share>", result.top_20_percent_share);
    println!("</pareto_analysis>");
}

fn get_language(matches: &clap::ArgMatches) -> &str {
    match matches.get_one::<String>("lang").map(|s| s.as_str()) {
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
        },
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
        ("en", "pareto_analysis_results") => "Pareto Principle (80/20 Rule) Analysis Results",
        ("en", "dataset") => "Dataset",
        ("en", "numbers_analyzed") => "Numbers analyzed",
        ("en", "risk_level") => "Attention Level",
        ("en", "pareto_metrics") => "Pareto Metrics",
        ("en", "top_20_percent_share") => "Top 20% share",
        ("en", "pareto_ratio") => "Pareto ratio",
        ("en", "concentration_index") => "Concentration index (Gini)",
        ("en", "interpretation") => "Interpretation",
        ("en", "ideal_pareto") => "Ideal Pareto distribution - follows 80/20 principle",
        ("en", "pareto_80_20_maintained") => "Top 20% controls approximately 80% of value",
        ("en", "slight_pareto_deviation") => "Slight deviation from Pareto principle",
        ("en", "pareto_monitoring_recommended") => "Monitoring recommended for distribution balance",
        ("en", "significant_pareto_deviation") => "Significant deviation from 80/20 principle",
        ("en", "pareto_rebalancing_needed") => "Consider rebalancing strategy",
        ("en", "critical_pareto_deviation") => "Critical deviation from Pareto principle",
        ("en", "pareto_strategy_review_needed") => "Strategy review needed",
        ("en", "high_concentration_insight") => "High concentration may indicate efficiency but also risk",
        ("en", "low_concentration_insight") => "Low concentration may indicate missed optimization opportunities",
        ("en", "unsupported_format") => "Error: Unsupported output format",
        ("en", "no_numbers_found") => "Error: No valid numbers found in input",
        ("en", "analysis_error") => "Analysis error",
        
        // 日本語
        ("ja", "pareto_analysis_results") => "パレートの法則（80/20の法則）解析結果",
        ("ja", "dataset") => "データセット",
        ("ja", "numbers_analyzed") => "解析した数値数",
        ("ja", "risk_level") => "注意レベル",
        ("ja", "pareto_metrics") => "パレート指標",
        ("ja", "top_20_percent_share") => "上位20%の占有率",
        ("ja", "pareto_ratio") => "パレート比率",
        ("ja", "concentration_index") => "集中度指数（ジニ係数）",
        ("ja", "interpretation") => "解釈",
        ("ja", "ideal_pareto") => "理想的なパレート分布 - 80/20の法則に従っています",
        ("ja", "pareto_80_20_maintained") => "上位20%が約80%の価値をコントロール",
        ("ja", "slight_pareto_deviation") => "パレートの法則からの軽微な偏差",
        ("ja", "pareto_monitoring_recommended") => "分布バランスの監視を推奨",
        ("ja", "significant_pareto_deviation") => "80/20の法則からの有意な偏差",
        ("ja", "pareto_rebalancing_needed") => "戦略の再バランスを検討",
        ("ja", "critical_pareto_deviation") => "パレートの法則からの重大な偏差",
        ("ja", "pareto_strategy_review_needed") => "戦略の見直しが必要",
        ("ja", "high_concentration_insight") => "高い集中度は効率性を示すが、リスクも伴います",
        ("ja", "low_concentration_insight") => "低い集中度は最適化機会を逃している可能性",
        ("ja", "unsupported_format") => "エラー: サポートされていない出力形式",
        ("ja", "no_numbers_found") => "エラー: 入力に有効な数値が見つかりません",
        ("ja", "analysis_error") => "解析エラー",
        
        // Default English
        (_, "pareto_analysis_results") => "Pareto Principle (80/20 Rule) Analysis Results",
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
fn analyze_numbers_with_options(matches: &clap::ArgMatches, dataset_name: String, numbers: &[f64]) -> Result<ParetoResult> {
    // Apply number filtering if specified
    let filtered_numbers = if let Some(filter_str) = matches.get_one::<String>("filter") {
        let filter = NumberFilter::parse(filter_str)
            .map_err(|e| BenfError::ParseError(format!("無効なフィルタ: {}", e)))?;
        
        let filtered = apply_number_filter(numbers, &filter);
        
        // Inform user about filtering results
        if filtered.len() != numbers.len() {
            eprintln!("フィルタリング結果: {} 個の数値が {} 個に絞り込まれました ({})", 
                     numbers.len(), filtered.len(), filter.description());
        }
        
        filtered
    } else {
        numbers.to_vec()
    };
    
    // Parse minimum count requirement
    let min_count = if let Some(min_count_str) = matches.get_one::<String>("min-count") {
        min_count_str.parse::<usize>()
            .map_err(|_| BenfError::ParseError("無効な最小数値数".to_string()))?
    } else {
        5
    };
    
    // Check minimum count requirement
    if filtered_numbers.len() < min_count {
        return Err(BenfError::InsufficientData(filtered_numbers.len()));
    }
    
    // Perform Pareto analysis
    analyze_pareto_distribution(&filtered_numbers, &dataset_name)
}