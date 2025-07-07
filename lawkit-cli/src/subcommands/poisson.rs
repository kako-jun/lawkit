use clap::ArgMatches;
use lawkit_core::{
    common::{
        filtering::{apply_number_filter, NumberFilter},
        input::parse_text_input,
    },
    error::{BenfError, Result},
    laws::poisson::{
        analyze_poisson_distribution, analyze_rare_events, predict_event_probabilities,
        test_poisson_fit, EventProbabilityResult, PoissonResult, PoissonTest, PoissonTestResult,
        RareEventAnalysis,
    },
};
use crate::common_options::{get_optimized_reader, setup_optimization_config};

pub fn run(matches: &ArgMatches) -> Result<()> {
    // 特殊モードの確認（フラグが明示的に指定された場合を優先）
    if matches.get_flag("predict") {
        return run_prediction_mode(matches);
    }

    if matches.get_flag("rare-events") {
        return run_rare_events_mode(matches);
    }

    // testパラメータが明示的に指定されている場合（デフォルト値"all"は通常分析で処理）
    if let Some(test_type) = matches.get_one::<String>("test") {
        if test_type != "all" {
            // "all"以外が明示的に指定された場合のみテストモード
            return run_poisson_test_mode(matches, test_type);
        }
    }

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

fn get_numbers_from_input(matches: &ArgMatches) -> Result<Vec<f64>> {
    let (use_optimize, _parallel_config, _memory_config) = setup_optimization_config(matches);

    let buffer = if let Some(input) = matches.get_one::<String>("input") {
        if input == "-" {
            get_optimized_reader(None, use_optimize)
        } else {
            get_optimized_reader(Some(input), use_optimize)
        }
    } else {
        get_optimized_reader(None, use_optimize)
    };

    let data = buffer.map_err(|e| BenfError::ParseError(e.to_string()))?;
    parse_text_input(&data)
}

fn run_poisson_test_mode(matches: &ArgMatches, test_type: &str) -> Result<()> {
    let numbers = get_numbers_from_input(matches)?;

    let test = match test_type {
        "chi-square" => PoissonTest::ChiSquare,
        "ks" => PoissonTest::KolmogorovSmirnov,
        "variance" => PoissonTest::VarianceTest,
        "all" => PoissonTest::All,
        _ => {
            eprintln!(
                "Error: Unknown test type '{test_type}'. Available: chi-square, ks, variance, all"
            );
            std::process::exit(2);
        }
    };

    let test_result = test_poisson_fit(&numbers, test)?;
    output_poisson_test_result(matches, &test_result);

    let exit_code = if test_result.is_poisson { 0 } else { 1 };
    std::process::exit(exit_code);
}

fn run_prediction_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = get_numbers_from_input(matches)?;
    let result = analyze_poisson_distribution(&numbers, "prediction")?;

    let max_events = matches
        .get_one::<String>("max-events")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(10);

    let prediction_result = predict_event_probabilities(result.lambda, max_events);
    output_prediction_result(matches, &prediction_result);

    std::process::exit(0);
}

fn run_rare_events_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = get_numbers_from_input(matches)?;
    let result = analyze_poisson_distribution(&numbers, "rare_events")?;

    let rare_analysis = analyze_rare_events(&numbers, result.lambda);
    output_rare_events_result(matches, &rare_analysis);

    let exit_code = if rare_analysis.clustering_detected {
        2
    } else {
        0
    };
    std::process::exit(exit_code);
}


fn output_results(matches: &clap::ArgMatches, result: &PoissonResult) {
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
            eprintln!("{error_msg}: {format}");
            std::process::exit(2);
        }
    }
}

fn output_poisson_test_result(matches: &clap::ArgMatches, result: &PoissonTestResult) {
    let language = get_language(matches);
    let format_str = matches
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("text");

    match format_str {
        "text" => {
            println!(
                "{}: {}",
                localized_text("poisson_test_result", language),
                result.test_name
            );
            println!(
                "{}: {:.6}",
                localized_text("test_statistic", language),
                result.statistic
            );
            println!(
                "{}: {:.6}",
                localized_text("p_value", language),
                result.p_value
            );
            println!("λ: {:.3}", result.parameter_lambda);
            println!(
                "{}: {}",
                localized_text("is_poisson", language),
                if result.is_poisson {
                    localized_text("yes", language)
                } else {
                    localized_text("no", language)
                }
            );
        }
        "json" => {
            use serde_json::json;
            let output = json!({
                "test_name": result.test_name,
                "statistic": result.statistic,
                "p_value": result.p_value,
                "critical_value": result.critical_value,
                "lambda": result.parameter_lambda,
                "is_poisson": result.is_poisson
            });
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        _ => println!("Unsupported format for Poisson test"),
    }
}

fn output_prediction_result(matches: &clap::ArgMatches, result: &EventProbabilityResult) {
    let language = get_language(matches);
    let format_str = matches
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("text");

    match format_str {
        "text" => {
            println!(
                "{} (λ = {:.3})",
                localized_text("event_probability_prediction", language),
                result.lambda
            );
            println!(
                "{}: {}",
                localized_text("most_likely_count", language),
                result.most_likely_count
            );
            println!();

            for prob in &result.probabilities {
                println!(
                    "P(X = {}) = {:.6} ({}累積: {:.6})",
                    prob.event_count,
                    prob.probability,
                    localized_text("cumulative", language),
                    prob.cumulative_probability
                );
            }

            if result.tail_probability > 0.001 {
                println!(
                    "P(X > {}) = {:.6}",
                    result.max_events, result.tail_probability
                );
            }
        }
        "json" => {
            use serde_json::json;
            let output = json!({
                "lambda": result.lambda,
                "max_events": result.max_events,
                "most_likely_count": result.most_likely_count,
                "expected_value": result.expected_value,
                "variance": result.variance,
                "tail_probability": result.tail_probability,
                "probabilities": result.probabilities.iter().map(|p| json!({
                    "event_count": p.event_count,
                    "probability": p.probability,
                    "cumulative_probability": p.cumulative_probability
                })).collect::<Vec<_>>()
            });
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        _ => println!("Unsupported format for prediction"),
    }
}

fn output_rare_events_result(matches: &clap::ArgMatches, result: &RareEventAnalysis) {
    let language = get_language(matches);
    let format_str = matches
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("text");

    match format_str {
        "text" => {
            println!(
                "{} (λ = {:.3})",
                localized_text("rare_events_analysis", language),
                result.lambda
            );
            println!(
                "{}: {}",
                localized_text("total_observations", language),
                result.total_observations
            );
            println!();

            println!("{}:", localized_text("rare_event_thresholds", language));
            println!(
                "  95%: {} ({} {})",
                result.threshold_95,
                result.rare_events_95,
                localized_text("events", language)
            );
            println!(
                "  99%: {} ({} {})",
                result.threshold_99,
                result.rare_events_99,
                localized_text("events", language)
            );
            println!(
                "  99.9%: {} ({} {})",
                result.threshold_999,
                result.rare_events_999,
                localized_text("events", language)
            );

            if !result.extreme_events.is_empty() {
                println!();
                println!("{}:", localized_text("extreme_events", language));
                for event in &result.extreme_events {
                    println!(
                        "  {}: {} {} (P = {:.6})",
                        localized_text("index", language),
                        event.index,
                        event.event_count,
                        event.probability
                    );
                }
            }

            if result.clustering_detected {
                println!();
                println!("⚠️ {}", localized_text("clustering_detected", language));
            }
        }
        "json" => {
            use serde_json::json;
            let output = json!({
                "lambda": result.lambda,
                "total_observations": result.total_observations,
                "thresholds": {
                    "95_percent": result.threshold_95,
                    "99_percent": result.threshold_99,
                    "99_9_percent": result.threshold_999
                },
                "rare_event_counts": {
                    "95_percent": result.rare_events_95,
                    "99_percent": result.rare_events_99,
                    "99_9_percent": result.rare_events_999
                },
                "extreme_events": result.extreme_events.iter().map(|e| json!({
                    "index": e.index,
                    "event_count": e.event_count,
                    "probability": e.probability,
                    "rarity_level": format!("{:?}", e.rarity_level)
                })).collect::<Vec<_>>(),
                "clustering_detected": result.clustering_detected
            });
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        _ => println!("Unsupported format for rare events analysis"),
    }
}

fn print_text_output(result: &PoissonResult, quiet: bool, verbose: bool, lang: &str) {
    if quiet {
        println!("lambda: {:.3}", result.lambda);
        println!("variance_ratio: {:.3}", result.variance_ratio);
        println!("goodness_of_fit: {:.3}", result.goodness_of_fit_score);
        return;
    }

    println!("{}", localized_text("poisson_analysis_results", lang));
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

    println!();
    println!("{}:", localized_text("poisson_parameters", lang));
    println!(
        "  λ ({}): {:.3}",
        localized_text("lambda", lang),
        result.lambda
    );
    println!(
        "  {}: {:.3}",
        localized_text("sample_mean", lang),
        result.sample_mean
    );
    println!(
        "  {}: {:.3}",
        localized_text("sample_variance", lang),
        result.sample_variance
    );
    println!(
        "  {}: {:.3}",
        localized_text("variance_ratio", lang),
        result.variance_ratio
    );

    if verbose {
        println!();
        println!("{}:", localized_text("goodness_of_fit_tests", lang));
        println!(
            "  Chi-Square: χ²={:.3}, p={:.3}",
            result.chi_square_statistic, result.chi_square_p_value
        );
        println!(
            "  Kolmogorov-Smirnov: D={:.3}, p={:.3}",
            result.kolmogorov_smirnov_statistic, result.kolmogorov_smirnov_p_value
        );

        println!();
        println!("{}:", localized_text("fit_assessment", lang));
        println!(
            "  {}: {:.3}",
            localized_text("goodness_of_fit_score", lang),
            result.goodness_of_fit_score
        );
        println!(
            "  {}: {:.3}",
            localized_text("poisson_quality", lang),
            result.poisson_quality
        );
        println!(
            "  {}: {:?}",
            localized_text("distribution_assessment", lang),
            result.distribution_assessment
        );

        println!();
        println!("{}:", localized_text("event_probabilities", lang));
        println!("  P(X = 0) = {:.3}", result.probability_zero);
        println!("  P(X = 1) = {:.3}", result.probability_one);
        println!("  P(X ≥ 2) = {:.3}", result.probability_two_or_more);

        if result.rare_events_count > 0 {
            println!();
            println!(
                "{}: {} ({} ≥ {})",
                localized_text("rare_events", lang),
                result.rare_events_count,
                localized_text("events", lang),
                result.rare_events_threshold
            );
        }

        println!();
        println!("{}:", localized_text("interpretation", lang));
        print_poisson_interpretation(result, lang);
    }
}

fn print_poisson_interpretation(result: &PoissonResult, lang: &str) {
    use lawkit_core::laws::poisson::result::PoissonAssessment;

    match result.distribution_assessment {
        PoissonAssessment::Excellent => {
            println!("✅ {}", localized_text("excellent_poisson_fit", lang));
            println!("   {}", localized_text("data_follows_poisson", lang));
        }
        PoissonAssessment::Good => {
            println!("✅ {}", localized_text("good_poisson_fit", lang));
            println!("   {}", localized_text("acceptable_poisson_fit", lang));
        }
        PoissonAssessment::Moderate => {
            println!("⚠️  {}", localized_text("moderate_poisson_fit", lang));
            println!(
                "   {}",
                localized_text("some_deviations_from_poisson", lang)
            );
        }
        PoissonAssessment::Poor => {
            println!("🚨 {}", localized_text("poor_poisson_fit", lang));
            println!(
                "   {}",
                localized_text("significant_deviations_from_poisson", lang)
            );
        }
        PoissonAssessment::NonPoisson => {
            println!("🔍 {}", localized_text("non_poisson_distribution", lang));
            println!("   {}", localized_text("data_not_poisson", lang));
        }
    }

    // 分散/平均比に基づく解釈
    if result.variance_ratio > 1.5 {
        println!("   📊 {}", localized_text("overdispersed", lang));
    } else if result.variance_ratio < 0.7 {
        println!("   📊 {}", localized_text("underdispersed", lang));
    }

    // 稀少事象の解釈
    if result.rare_events_count > 0 {
        println!(
            "   🎯 {}: {}",
            localized_text("rare_events_detected", lang),
            result.rare_events_count
        );
    }
}

fn print_json_output(result: &PoissonResult) {
    use serde_json::json;

    let output = json!({
        "dataset": result.dataset_name,
        "numbers_analyzed": result.numbers_analyzed,
        "risk_level": format!("{:?}", result.risk_level),
        "lambda": result.lambda,
        "sample_mean": result.sample_mean,
        "sample_variance": result.sample_variance,
        "variance_ratio": result.variance_ratio,
        "chi_square_test": {
            "statistic": result.chi_square_statistic,
            "p_value": result.chi_square_p_value
        },
        "kolmogorov_smirnov_test": {
            "statistic": result.kolmogorov_smirnov_statistic,
            "p_value": result.kolmogorov_smirnov_p_value
        },
        "goodness_of_fit_score": result.goodness_of_fit_score,
        "poisson_quality": result.poisson_quality,
        "distribution_assessment": format!("{:?}", result.distribution_assessment),
        "event_probabilities": {
            "zero": result.probability_zero,
            "one": result.probability_one,
            "two_or_more": result.probability_two_or_more
        },
        "rare_events": {
            "threshold": result.rare_events_threshold,
            "count": result.rare_events_count
        },
        "confidence_interval_lambda": result.confidence_interval_lambda
    });

    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

fn print_csv_output(result: &PoissonResult) {
    println!("dataset,numbers_analyzed,risk_level,lambda,sample_mean,sample_variance,variance_ratio,goodness_of_fit_score");
    println!(
        "{},{},{:?},{:.3},{:.3},{:.3},{:.3},{:.3}",
        result.dataset_name,
        result.numbers_analyzed,
        result.risk_level,
        result.lambda,
        result.sample_mean,
        result.sample_variance,
        result.variance_ratio,
        result.goodness_of_fit_score
    );
}

fn print_yaml_output(result: &PoissonResult) {
    println!("dataset: \"{}\"", result.dataset_name);
    println!("numbers_analyzed: {}", result.numbers_analyzed);
    println!("risk_level: \"{:?}\"", result.risk_level);
    println!("lambda: {:.3}", result.lambda);
    println!("sample_mean: {:.3}", result.sample_mean);
    println!("sample_variance: {:.3}", result.sample_variance);
    println!("variance_ratio: {:.3}", result.variance_ratio);
    println!("goodness_of_fit_score: {:.3}", result.goodness_of_fit_score);
}

fn print_toml_output(result: &PoissonResult) {
    println!("dataset = \"{}\"", result.dataset_name);
    println!("numbers_analyzed = {}", result.numbers_analyzed);
    println!("risk_level = \"{:?}\"", result.risk_level);
    println!("lambda = {:.3}", result.lambda);
    println!("sample_mean = {:.3}", result.sample_mean);
    println!("sample_variance = {:.3}", result.sample_variance);
    println!("variance_ratio = {:.3}", result.variance_ratio);
    println!(
        "goodness_of_fit_score = {:.3}",
        result.goodness_of_fit_score
    );
}

fn print_xml_output(result: &PoissonResult) {
    println!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    println!("<poisson_analysis>");
    println!("  <dataset>{}</dataset>", result.dataset_name);
    println!(
        "  <numbers_analyzed>{}</numbers_analyzed>",
        result.numbers_analyzed
    );
    println!("  <risk_level>{:?}</risk_level>", result.risk_level);
    println!("  <lambda>{:.3}</lambda>", result.lambda);
    println!("  <sample_mean>{:.3}</sample_mean>", result.sample_mean);
    println!(
        "  <sample_variance>{:.3}</sample_variance>",
        result.sample_variance
    );
    println!(
        "  <variance_ratio>{:.3}</variance_ratio>",
        result.variance_ratio
    );
    println!(
        "  <goodness_of_fit_score>{:.3}</goodness_of_fit_score>",
        result.goodness_of_fit_score
    );
    println!("</poisson_analysis>");
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
        ("en", "poisson_analysis_results") => "Poisson Distribution Analysis Results",
        ("en", "dataset") => "Dataset",
        ("en", "numbers_analyzed") => "Numbers analyzed",
        ("en", "risk_level") => "Quality Level",
        ("en", "poisson_parameters") => "Poisson Parameters",
        ("en", "lambda") => "rate parameter",
        ("en", "sample_mean") => "Sample mean",
        ("en", "sample_variance") => "Sample variance",
        ("en", "variance_ratio") => "Variance/Mean ratio",
        ("en", "goodness_of_fit_tests") => "Goodness of Fit Tests",
        ("en", "fit_assessment") => "Fit Assessment",
        ("en", "goodness_of_fit_score") => "Goodness of fit score",
        ("en", "poisson_quality") => "Poisson quality",
        ("en", "distribution_assessment") => "Distribution assessment",
        ("en", "event_probabilities") => "Event Probabilities",
        ("en", "rare_events") => "Rare events",
        ("en", "events") => "events",
        ("en", "interpretation") => "Interpretation",
        ("en", "excellent_poisson_fit") => "Excellent Poisson distribution fit",
        ("en", "data_follows_poisson") => "Data closely follows Poisson distribution",
        ("en", "good_poisson_fit") => "Good Poisson distribution fit",
        ("en", "acceptable_poisson_fit") => "Acceptable fit to Poisson distribution",
        ("en", "moderate_poisson_fit") => "Moderate Poisson distribution fit",
        ("en", "some_deviations_from_poisson") => "Some deviations from Poisson distribution",
        ("en", "poor_poisson_fit") => "Poor Poisson distribution fit",
        ("en", "significant_deviations_from_poisson") => {
            "Significant deviations from Poisson distribution"
        }
        ("en", "non_poisson_distribution") => "Non-Poisson distribution",
        ("en", "data_not_poisson") => "Data does not follow Poisson distribution",
        ("en", "overdispersed") => "Distribution is overdispersed",
        ("en", "underdispersed") => "Distribution is underdispersed",
        ("en", "rare_events_detected") => "Rare events detected",
        ("en", "poisson_test_result") => "Poisson Test Result",
        ("en", "test_statistic") => "Test statistic",
        ("en", "p_value") => "P-value",
        ("en", "is_poisson") => "Is Poisson",
        ("en", "yes") => "Yes",
        ("en", "no") => "No",
        ("en", "event_probability_prediction") => "Event Probability Prediction",
        ("en", "most_likely_count") => "Most likely count",
        ("en", "cumulative") => "cumulative",
        ("en", "rare_events_analysis") => "Rare Events Analysis",
        ("en", "total_observations") => "Total observations",
        ("en", "rare_event_thresholds") => "Rare Event Thresholds",
        ("en", "extreme_events") => "Extreme Events",
        ("en", "index") => "Index",
        ("en", "clustering_detected") => "Event clustering detected",
        ("en", "unsupported_format") => "Error: Unsupported output format",
        ("en", "no_numbers_found") => "Error: No valid numbers found in input",
        ("en", "analysis_error") => "Analysis error",

        // 日本語
        ("ja", "poisson_analysis_results") => "ポアソン分布解析結果",
        ("ja", "dataset") => "データセット",
        ("ja", "numbers_analyzed") => "解析した数値数",
        ("ja", "risk_level") => "品質レベル",
        ("ja", "poisson_parameters") => "ポアソンパラメータ",
        ("ja", "lambda") => "発生率パラメータ",
        ("ja", "sample_mean") => "標本平均",
        ("ja", "sample_variance") => "標本分散",
        ("ja", "variance_ratio") => "分散/平均比",
        ("ja", "goodness_of_fit_tests") => "適合度検定",
        ("ja", "fit_assessment") => "適合度評価",
        ("ja", "goodness_of_fit_score") => "適合度スコア",
        ("ja", "poisson_quality") => "ポアソン品質",
        ("ja", "distribution_assessment") => "分布評価",
        ("ja", "event_probabilities") => "イベント発生確率",
        ("ja", "rare_events") => "稀少事象",
        ("ja", "events") => "事象",
        ("ja", "interpretation") => "解釈",
        ("ja", "excellent_poisson_fit") => "優れたポアソン分布適合",
        ("ja", "data_follows_poisson") => "データはポアソン分布に良く従っています",
        ("ja", "good_poisson_fit") => "良いポアソン分布適合",
        ("ja", "acceptable_poisson_fit") => "ポアソン分布への許容できる適合",
        ("ja", "moderate_poisson_fit") => "中程度のポアソン分布適合",
        ("ja", "some_deviations_from_poisson") => "ポアソン分布からの軽微な偏差",
        ("ja", "poor_poisson_fit") => "不十分なポアソン分布適合",
        ("ja", "significant_deviations_from_poisson") => "ポアソン分布からの有意な偏差",
        ("ja", "non_poisson_distribution") => "非ポアソン分布",
        ("ja", "data_not_poisson") => "データはポアソン分布に従いません",
        ("ja", "overdispersed") => "分布は過分散です",
        ("ja", "underdispersed") => "分布は過少分散です",
        ("ja", "rare_events_detected") => "稀少事象が検出されました",
        ("ja", "poisson_test_result") => "ポアソン検定結果",
        ("ja", "test_statistic") => "検定統計量",
        ("ja", "p_value") => "p値",
        ("ja", "is_poisson") => "ポアソン分布か",
        ("ja", "yes") => "はい",
        ("ja", "no") => "いいえ",
        ("ja", "event_probability_prediction") => "イベント発生確率予測",
        ("ja", "most_likely_count") => "最頻発生数",
        ("ja", "cumulative") => "累積",
        ("ja", "rare_events_analysis") => "稀少事象分析",
        ("ja", "total_observations") => "総観測数",
        ("ja", "rare_event_thresholds") => "稀少事象閾値",
        ("ja", "extreme_events") => "極端事象",
        ("ja", "index") => "インデックス",
        ("ja", "clustering_detected") => "事象クラスタリングが検出されました",
        ("ja", "unsupported_format") => "エラー: サポートされていない出力形式",
        ("ja", "no_numbers_found") => "エラー: 入力に有効な数値が見つかりません",
        ("ja", "analysis_error") => "解析エラー",

        // Default English
        (_, "poisson_analysis_results") => "Poisson Distribution Analysis Results",
        (_, "dataset") => "Dataset",
        (_, "numbers_analyzed") => "Numbers analyzed",
        (_, "risk_level") => "Quality Level",
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
) -> Result<PoissonResult> {
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
        10 // ポアソン分布分析では最低10個必要
    };

    // Check minimum count requirement
    if filtered_numbers.len() < min_count {
        return Err(BenfError::InsufficientData(filtered_numbers.len()));
    }

    // Perform Poisson distribution analysis
    analyze_poisson_distribution(&filtered_numbers, &dataset_name)
}
