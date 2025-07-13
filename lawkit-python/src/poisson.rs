use crate::colors;
use crate::common_options::{get_optimized_reader, setup_automatic_optimization_config};
use clap::ArgMatches;
use lawkit_core::{
    common::{
        filtering::{apply_number_filter, NumberFilter},
        input::{parse_input_auto, parse_text_input},
        memory::{streaming_poisson_analysis, MemoryConfig},
        streaming_io::OptimizedFileReader,
    },
    error::{BenfError, Result},
    laws::poisson::{
        analyze_poisson_distribution, analyze_rare_events, predict_event_probabilities,
        test_poisson_fit, EventProbabilityResult, PoissonResult, PoissonTest, PoissonTestResult,
        RareEventAnalysis,
    },
};

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

    // 自動最適化設定をセットアップ
    let (_parallel_config, _memory_config) = setup_automatic_optimization_config();

    // Determine input source based on arguments
    if matches.get_flag("verbose") {
        eprintln!(
            "Debug: input argument = {:?}",
            matches.get_one::<String>("input")
        );
    }

    // 入力データ処理
    let numbers = if let Some(input) = matches.get_one::<String>("input") {
        // ファイル入力の場合
        match parse_input_auto(input) {
            Ok(numbers) => {
                if numbers.is_empty() {
                    eprintln!("Error: No valid numbers found in input");
                    std::process::exit(1);
                }
                numbers
            }
            Err(e) => {
                eprintln!("Error processing input '{input}': {e}");
                std::process::exit(1);
            }
        }
    } else {
        // stdin入力の場合：ストリーミング処理を使用
        if matches.get_flag("verbose") {
            eprintln!("Debug: Reading from stdin, using automatic optimization");
        }

        let mut reader = OptimizedFileReader::from_stdin();

        if matches.get_flag("verbose") {
            eprintln!(
                "Debug: Using automatic optimization (streaming + incremental + memory efficiency)"
            );
        }

        let numbers = match reader
            .read_lines_streaming(|line: String| parse_text_input(&line).map(Some).or(Ok(None)))
        {
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

        // ポアソン分布は非負整数データを想定しているので、データを整数化
        let event_counts: Vec<usize> = numbers
            .iter()
            .filter_map(|&x| if x >= 0.0 { Some(x as usize) } else { None })
            .collect();

        // インクリメンタルストリーミング分析を実行（大量データの場合）
        if event_counts.len() > 10000 {
            let memory_config = MemoryConfig::default();
            let chunk_result =
                match streaming_poisson_analysis(event_counts.into_iter(), &memory_config) {
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
            }

            // usize から f64 に変換
            chunk_result
                .result
                .event_counts()
                .iter()
                .map(|&x| x as f64)
                .collect()
        } else {
            // 小さなデータセットの場合は直接処理
            if matches.get_flag("verbose") {
                eprintln!("Debug: Memory used: 0.00 MB");
            }
            event_counts.iter().map(|&x| x as f64).collect()
        }
    };

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

fn get_numbers_from_input(matches: &ArgMatches) -> Result<Vec<f64>> {
    let (_parallel_config, _memory_config) = setup_automatic_optimization_config();

    let buffer = if let Some(input) = matches.get_one::<String>("input") {
        if input == "-" {
            get_optimized_reader(None)
        } else {
            get_optimized_reader(Some(input))
        }
    } else {
        get_optimized_reader(None)
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

fn output_poisson_test_result(matches: &clap::ArgMatches, result: &PoissonTestResult) {
    let format_str = matches
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("text");

    match format_str {
        "text" => {
            println!("Poisson Test Result: {}", result.test_name);
            println!("Test statistic: {:.6}", result.statistic);
            println!("P-value: {:.6}", result.p_value);
            println!("λ: {:.3}", result.parameter_lambda);
            println!(
                "Is Poisson: {}",
                if result.is_poisson { "Yes" } else { "No" }
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
    let format_str = matches
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("text");

    match format_str {
        "text" => {
            println!("Event Probability Prediction (λ = {:.3})", result.lambda);
            println!("Most likely count: {}", result.most_likely_count);
            println!();

            for prob in &result.probabilities {
                println!(
                    "P(X = {}) = {:.6} (cumulative: {:.6})",
                    prob.event_count, prob.probability, prob.cumulative_probability
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
    let format_str = matches
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("text");

    match format_str {
        "text" => {
            println!("Rare Events Analysis (λ = {:.3})", result.lambda);
            println!("Total observations: {}", result.total_observations);
            println!();

            println!("Rare Event Thresholds:");
            println!(
                "  95%: {} ({} events)",
                result.threshold_95, result.rare_events_95
            );
            println!(
                "  99%: {} ({} events)",
                result.threshold_99, result.rare_events_99
            );
            println!(
                "  99.9%: {} ({} events)",
                result.threshold_999, result.rare_events_999
            );

            if !result.extreme_events.is_empty() {
                println!();
                println!("Extreme Events:");
                for event in &result.extreme_events {
                    println!(
                        "  Index: {} {} (P = {:.6})",
                        event.index, event.event_count, event.probability
                    );
                }
            }

            if result.clustering_detected {
                println!();
                println!("ALERT: Event clustering detected");
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

fn print_text_output(result: &PoissonResult, quiet: bool, verbose: bool) {
    if quiet {
        println!("lambda: {:.3}", result.lambda);
        println!("variance_ratio: {:.3}", result.variance_ratio);
        println!("goodness_of_fit: {:.3}", result.goodness_of_fit_score);
        return;
    }

    println!("Poisson Distribution Analysis Results");
    println!();
    println!("Dataset: {}", result.dataset_name);
    println!("Numbers analyzed: {}", result.numbers_analyzed);
    println!("Quality Level: {:?}", result.risk_level);

    println!();
    println!("Probability Distribution:");
    println!("{}", format_poisson_probability_chart(result));

    println!();
    println!("Poisson Parameters:");
    println!("  λ (rate parameter): {:.3}", result.lambda);
    println!("  Sample mean: {:.3}", result.sample_mean);
    println!("  Sample variance: {:.3}", result.sample_variance);
    println!("  Variance/Mean ratio: {:.3}", result.variance_ratio);

    if verbose {
        println!();
        println!("Goodness of Fit Tests:");
        println!(
            "  Chi-Square: χ²={:.3}, p={:.3}",
            result.chi_square_statistic, result.chi_square_p_value
        );
        println!(
            "  Kolmogorov-Smirnov: D={:.3}, p={:.3}",
            result.kolmogorov_smirnov_statistic, result.kolmogorov_smirnov_p_value
        );

        println!();
        println!("Fit Assessment:");
        println!(
            "  Goodness of fit score: {:.3}",
            result.goodness_of_fit_score
        );
        println!("  Poisson quality: {:.3}", result.poisson_quality);
        println!(
            "  Distribution assessment: {:?}",
            result.distribution_assessment
        );

        println!();
        println!("Event Probabilities:");
        println!("  P(X = 0) = {:.3}", result.probability_zero);
        println!("  P(X = 1) = {:.3}", result.probability_one);
        println!("  P(X ≥ 2) = {:.3}", result.probability_two_or_more);

        if result.rare_events_count > 0 {
            println!();
            println!(
                "Rare events: {} (events ≥ {})",
                result.rare_events_count, result.rare_events_threshold
            );
        }

        println!();
        println!("Interpretation:");
        print_poisson_interpretation(result);
    }
}

fn print_poisson_interpretation(result: &PoissonResult) {
    use lawkit_core::laws::poisson::result::PoissonAssessment;

    match result.distribution_assessment {
        PoissonAssessment::Excellent => {
            println!(
                "{}",
                colors::level_pass("Excellent Poisson distribution fit")
            );
            println!("   Data closely follows Poisson distribution");
        }
        PoissonAssessment::Good => {
            println!("{}", colors::level_pass("Good Poisson distribution fit"));
            println!("   Acceptable fit to Poisson distribution");
        }
        PoissonAssessment::Moderate => {
            println!(
                "{}",
                colors::level_warn("Moderate Poisson distribution fit")
            );
            println!("   Some deviations from Poisson distribution");
        }
        PoissonAssessment::Poor => {
            println!("{}", colors::level_fail("Poor Poisson distribution fit"));
            println!("   Significant deviations from Poisson distribution");
        }
        PoissonAssessment::NonPoisson => {
            println!("{}", colors::level_critical("Non-Poisson distribution"));
            println!("   Data does not follow Poisson distribution");
        }
    }

    // 分散/平均比に基づく解釈
    if result.variance_ratio > 1.5 {
        println!("   INFO: Distribution is overdispersed");
    } else if result.variance_ratio < 0.7 {
        println!("   INFO: Distribution is underdispersed");
    }

    // 稀少事象の解釈
    if result.rare_events_count > 0 {
        println!(
            "   ALERT: Rare events detected: {}",
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

    // Parse confidence level
    let confidence = if let Some(confidence_str) = matches.get_one::<String>("confidence") {
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

    // Perform Poisson distribution analysis
    // TODO: Integrate confidence level into analysis
    let mut result = analyze_poisson_distribution(&filtered_numbers, &dataset_name)?;

    // For now, just store confidence level as a comment in the dataset name
    if confidence != 0.95 {
        result.dataset_name = format!("{} (confidence: {:.2})", result.dataset_name, confidence);
    }

    Ok(result)
}

fn format_poisson_probability_chart(result: &PoissonResult) -> String {
    let mut output = String::new();
    const CHART_WIDTH: usize = 50;

    let lambda = result.lambda;

    // 確率が十分小さくなるまでの範囲を計算（通常λ + 3√λ程度）
    let max_k = ((lambda + 3.0 * lambda.sqrt()).ceil() as u32).clamp(10, 20);

    // 各値の理論確率を計算
    let mut probabilities = Vec::new();
    let mut max_prob: f64 = 0.0;

    for k in 0..=max_k {
        // ポアソン確率質量関数: P(X=k) = (λ^k * e^(-λ)) / k!
        let prob = poisson_pmf(k, lambda);
        probabilities.push((k, prob));
        max_prob = max_prob.max(prob);
    }

    // 確率分布を表示
    for (k, prob) in &probabilities {
        if max_prob > 0.0 {
            let normalized_prob = prob / max_prob;
            let bar_length = (normalized_prob * CHART_WIDTH as f64).round() as usize;
            let bar_length = bar_length.min(CHART_WIDTH);

            let filled_bar = "█".repeat(bar_length);
            let background_bar = "░".repeat(CHART_WIDTH - bar_length);
            let full_bar = format!("{filled_bar}{background_bar}");

            output.push_str(&format!("P(X={k:2}): {full_bar} {prob:>6.3}\n"));
        }
    }

    // 重要な確率値を表示
    output.push_str(&format!(
        "\nKey Probabilities: P(X=0)={:.3}, P(X=1)={:.3}, P(X≥2)={:.3}",
        result.probability_zero, result.probability_one, result.probability_two_or_more
    ));

    // λとポアソン性の評価
    output.push_str(&format!(
        "\nλ={:.2}, Variance/Mean={:.3} (ideal: 1.0), Fit Score={:.3}",
        lambda, result.variance_ratio, result.goodness_of_fit_score
    ));

    output
}

// ポアソン確率質量関数
fn poisson_pmf(k: u32, lambda: f64) -> f64 {
    if lambda <= 0.0 {
        return 0.0;
    }

    // P(X=k) = (λ^k * e^(-λ)) / k!
    // 対数計算で数値的安定性を確保
    let log_prob = k as f64 * lambda.ln() - lambda - log_factorial(k);
    log_prob.exp()
}

// 対数階乗の計算（数値的安定性のため）
fn log_factorial(n: u32) -> f64 {
    if n <= 1 {
        0.0
    } else {
        (2..=n).map(|i| (i as f64).ln()).sum()
    }
}
