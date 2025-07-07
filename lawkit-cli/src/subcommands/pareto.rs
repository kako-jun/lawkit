use crate::common_options::{get_optimized_reader, setup_optimization_config};
use clap::ArgMatches;
use lawkit_core::{
    common::{
        filtering::{apply_number_filter, NumberFilter},
        input::parse_text_input,
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
            let language = get_language(matches);
            let error_msg = localized_text("analysis_error", language);
            eprintln!("{error_msg}: {e}");
            std::process::exit(1);
        }
    };

    if numbers.is_empty() {
        let language = get_language(matches);
        let error_msg = localized_text("no_numbers_found", language);
        eprintln!("{error_msg}");
        std::process::exit(1);
    }

    let dataset_name = matches
        .get_one::<String>("input")
        .map(|s| s.to_string())
        .unwrap_or_else(|| "stdin".to_string());

    let result = match analyze_numbers_with_options(matches, dataset_name, &numbers) {
        Ok(result) => result,
        Err(e) => {
            let language = get_language(matches);
            let error_msg = localized_text("analysis_error", language);
            eprintln!("{error_msg}: {e}");
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
    let language = get_language(matches);

    match format.as_str() {
        "text" => print_text_output(result, quiet, verbose, language, matches),
        "json" => print_json_output(result),
        "csv" => print_csv_output(result),
        "yaml" => print_yaml_output(result),
        "toml" => print_toml_output(result),
        "xml" => print_xml_output(result),
        _ => {
            let error_msg = localized_text("unsupported_format", language);
            eprintln!("{error_msg}: {format}");
            std::process::exit(2);
        }
    }
}

fn print_text_output(
    result: &ParetoResult,
    quiet: bool,
    verbose: bool,
    lang: &str,
    matches: &clap::ArgMatches,
) {
    if quiet {
        println!("pareto_ratio: {:.3}", result.pareto_ratio);
        println!("concentration_index: {:.3}", result.concentration_index);
        println!("top_20_percent_share: {:.1}%", result.top_20_percent_share);
        println!("gini_coefficient: {:.3}", result.concentration_index);
        return;
    }

    println!("{}", localized_text("pareto_analysis_results", lang));
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
        println!("{}:", localized_text("pareto_metrics", lang));
        println!(
            "  {}: {:.1}%",
            localized_text("top_20_percent_share", lang),
            result.top_20_percent_share
        );
        println!(
            "  {}: {:.3}",
            localized_text("pareto_ratio", lang),
            result.pareto_ratio
        );
        println!(
            "  {}: {:.3}",
            localized_text("concentration_index", lang),
            result.concentration_index
        );

        // カスタムパーセンタイルの表示
        if let Some(ref percentiles) = result.custom_percentiles {
            println!();
            println!("{}:", localized_text("custom_percentiles", lang));
            for (percentile, share) in percentiles {
                println!("  Top {percentile:.0}%: {share:.1}%");
            }
        }

        println!();
        println!("{}:", localized_text("interpretation", lang));
        print_pareto_interpretation(result, lang);
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
            println!("{}:", localized_text("custom_percentiles", lang));
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

fn print_pareto_interpretation(result: &ParetoResult, lang: &str) {
    use lawkit_core::common::risk::RiskLevel;

    match result.risk_level {
        RiskLevel::Low => {
            println!("✅ {}", localized_text("ideal_pareto", lang));
            println!("   {}", localized_text("pareto_80_20_maintained", lang));
        }
        RiskLevel::Medium => {
            println!("⚠️  {}", localized_text("slight_pareto_deviation", lang));
            println!(
                "   {}",
                localized_text("pareto_monitoring_recommended", lang)
            );
        }
        RiskLevel::High => {
            println!(
                "🚨 {}",
                localized_text("significant_pareto_deviation", lang)
            );
            println!("   {}", localized_text("pareto_rebalancing_needed", lang));
        }
        RiskLevel::Critical => {
            println!("🔍 {}", localized_text("critical_pareto_deviation", lang));
            println!(
                "   {}",
                localized_text("pareto_strategy_review_needed", lang)
            );
        }
    }

    // 80/20原則からの偏差説明
    if result.top_20_percent_share > 85.0 {
        println!(
            "   💡 {}",
            localized_text("high_concentration_insight", lang)
        );
    } else if result.top_20_percent_share < 70.0 {
        println!(
            "   💡 {}",
            localized_text("low_concentration_insight", lang)
        );
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
        ("en", "pareto_monitoring_recommended") => {
            "Monitoring recommended for distribution balance"
        }
        ("en", "significant_pareto_deviation") => "Significant deviation from 80/20 principle",
        ("en", "pareto_rebalancing_needed") => "Consider rebalancing strategy",
        ("en", "critical_pareto_deviation") => "Critical deviation from Pareto principle",
        ("en", "pareto_strategy_review_needed") => "Strategy review needed",
        ("en", "high_concentration_insight") => {
            "High concentration may indicate efficiency but also risk"
        }
        ("en", "low_concentration_insight") => {
            "Low concentration may indicate missed optimization opportunities"
        }
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
        ("ja", "custom_percentiles") => "カスタムパーセンタイル",

        // Default English
        (_, "pareto_analysis_results") => "Pareto Principle (80/20 Rule) Analysis Results",
        (_, "custom_percentiles") => "Custom Percentiles",
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
