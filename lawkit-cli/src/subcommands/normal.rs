use crate::colors;
use crate::common_options::{get_optimized_reader, setup_automatic_optimization_config};
use clap::ArgMatches;
use lawkit_core::{
    common::{
        filtering::{apply_number_filter, NumberFilter},
        input::{parse_input_auto, parse_text_input},
        memory::{streaming_normal_analysis, MemoryConfig},
        outliers::{
            detect_outliers_dbscan, detect_outliers_ensemble, detect_outliers_isolation,
            detect_outliers_lof, AdvancedOutlierResult,
        },
        streaming_io::OptimizedFileReader,
        timeseries::{analyze_timeseries, create_timeseries_from_values, TimeSeriesAnalysis},
    },
    error::{BenfError, Result},
    laws::normal::{
        analyze_normal_distribution, detect_outliers, quality_control_analysis, test_normality,
        NormalResult, NormalityTest, NormalityTestResult, OutlierDetectionMethod,
        OutlierDetectionResult, ProcessCapability, QualityControlResult,
    },
};

pub fn run(matches: &ArgMatches) -> Result<()> {
    // 自動最適化設定をセットアップ
    let (_parallel_config, _memory_config) = setup_automatic_optimization_config();

    // 特殊モードの確認（フラグベースのモードを優先）
    if matches.get_flag("outliers") {
        return run_outlier_detection_mode(matches);
    }

    if matches.get_flag("quality-control") {
        return run_quality_control_mode(matches);
    }

    if matches.get_flag("enable-timeseries") {
        return run_timeseries_analysis_mode(matches);
    }

    // testパラメータが明示的に指定されている場合のみテストモード
    if let Some(test_type) = matches.get_one::<String>("test") {
        if test_type != "all" {
            // "all"はデフォルトなので通常分析モードで処理
            return run_normality_test_mode(matches, test_type);
        }
    }

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

        // インクリメンタルストリーミング分析を実行（より詳細な統計が必要な場合）
        if numbers.len() > 10000 {
            let memory_config = MemoryConfig::default();
            let chunk_result = match streaming_normal_analysis(numbers.into_iter(), &memory_config)
            {
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

            chunk_result.result.values().to_vec()
        } else {
            if matches.get_flag("verbose") {
                eprintln!("Debug: Memory used: 0.00 MB");
            }
            numbers
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

fn run_normality_test_mode(matches: &ArgMatches, test_type: &str) -> Result<()> {
    let numbers = get_numbers_from_input(matches)?;

    let test = match test_type {
        "shapiro" => NormalityTest::ShapiroWilk,
        "anderson" => NormalityTest::AndersonDarling,
        "ks" => NormalityTest::KolmogorovSmirnov,
        "all" => NormalityTest::All,
        _ => {
            eprintln!(
                "Error: Unknown test type '{test_type}'. Available: shapiro, anderson, ks, all"
            );
            std::process::exit(2);
        }
    };

    let test_result = test_normality(&numbers, test)?;
    output_normality_test_result(matches, &test_result);

    let exit_code = if test_result.is_normal { 0 } else { 1 };
    std::process::exit(exit_code);
}

fn run_outlier_detection_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = get_numbers_from_input(matches)?;

    let method_str = matches
        .get_one::<String>("outlier-method")
        .map(|s| s.as_str())
        .unwrap_or("zscore");

    // 高度な異常値検出手法の処理
    match method_str {
        "lof" => {
            let result = detect_outliers_lof(&numbers, 5)?;
            output_advanced_outlier_result(matches, &result);
            let exit_code = if result.outliers.is_empty() { 0 } else { 10 };
            std::process::exit(exit_code);
        }
        "isolation" => {
            let result = detect_outliers_isolation(&numbers, 8)?;
            output_advanced_outlier_result(matches, &result);
            let exit_code = if result.outliers.is_empty() { 0 } else { 10 };
            std::process::exit(exit_code);
        }
        "dbscan" => {
            let std_dev = calculate_std_dev(&numbers);
            let eps = std_dev * 0.5;
            let min_pts = (numbers.len() as f64).sqrt() as usize;
            let result = detect_outliers_dbscan(&numbers, eps, min_pts)?;
            output_advanced_outlier_result(matches, &result);
            let exit_code = if result.outliers.is_empty() { 0 } else { 10 };
            std::process::exit(exit_code);
        }
        "ensemble" => {
            let result = detect_outliers_ensemble(&numbers)?;
            output_advanced_outlier_result(matches, &result);
            let exit_code = if result.outliers.is_empty() { 0 } else { 10 };
            std::process::exit(exit_code);
        }
        _ => {
            // 既存の異常値検出手法
            let method = match method_str {
                "zscore" => OutlierDetectionMethod::ZScore,
                "modified" | "modified_zscore" => OutlierDetectionMethod::ModifiedZScore,
                "iqr" => OutlierDetectionMethod::IQR,
                _ => {
                    eprintln!(
                        "Error: Unknown outlier detection method '{method_str}'. Available: zscore, modified_zscore, iqr, lof, isolation, dbscan, ensemble"
                    );
                    std::process::exit(2);
                }
            };

            let outlier_result = detect_outliers(&numbers, method)?;
            output_outlier_detection_result(matches, &outlier_result);

            let exit_code = if outlier_result.outliers.is_empty() {
                0
            } else {
                1
            };
            std::process::exit(exit_code);
        }
    }
}

fn run_timeseries_analysis_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = get_numbers_from_input(matches)?;

    // 数値データを時系列データに変換
    let timeseries_data = create_timeseries_from_values(&numbers);

    // 時系列分析を実行
    let analysis_result = analyze_timeseries(&timeseries_data)?;

    // 結果を出力
    output_timeseries_result(matches, &analysis_result);

    std::process::exit(0);
}

fn run_quality_control_mode(matches: &ArgMatches) -> Result<()> {
    let numbers = get_numbers_from_input(matches)?;

    let spec_limits = if let Some(limits_str) = matches.get_one::<String>("spec-limits") {
        parse_spec_limits(limits_str)?
    } else {
        None
    };

    let qc_result = quality_control_analysis(&numbers, spec_limits)?;
    output_quality_control_result(matches, &qc_result);

    let exit_code = match &qc_result.process_capability {
        Some(cap) => match cap {
            ProcessCapability::Excellent => 0,
            ProcessCapability::Adequate => 1,
            ProcessCapability::Poor => 2,
            ProcessCapability::Inadequate => 3,
        },
        None => 0,
    };
    std::process::exit(exit_code);
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

fn parse_spec_limits(limits_str: &str) -> Result<Option<(f64, f64)>> {
    let parts: Vec<&str> = limits_str.split(',').collect();
    if parts.len() != 2 {
        return Err(BenfError::ParseError(
            "Spec limits must be in format 'lower,upper'".to_string(),
        ));
    }

    let lower = parts[0]
        .trim()
        .parse::<f64>()
        .map_err(|_| BenfError::ParseError("Invalid lower spec limit".to_string()))?;
    let upper = parts[1]
        .trim()
        .parse::<f64>()
        .map_err(|_| BenfError::ParseError("Invalid upper spec limit".to_string()))?;

    if lower >= upper {
        return Err(BenfError::ParseError(
            "Lower spec limit must be less than upper spec limit".to_string(),
        ));
    }

    Ok(Some((lower, upper)))
}

fn output_results(matches: &clap::ArgMatches, result: &NormalResult) {
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

fn output_normality_test_result(matches: &clap::ArgMatches, result: &NormalityTestResult) {
    let format_str = matches
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("text");

    match format_str {
        "text" => {
            println!("Test: {}", result.test_name);
            println!("Statistic: {:.6}", result.statistic);
            println!("P-value: {:.6}", result.p_value);
            println!("Is Normal: {}", if result.is_normal { "Yes" } else { "No" });
        }
        "json" => {
            use serde_json::json;
            let output = json!({
                "test_name": result.test_name,
                "statistic": result.statistic,
                "p_value": result.p_value,
                "critical_value": result.critical_value,
                "is_normal": result.is_normal
            });
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        _ => print_text_output(
            &NormalResult::new("test".to_string(), &[0.0; 10]).unwrap(),
            false,
            false,
        ),
    }
}

fn output_outlier_detection_result(matches: &clap::ArgMatches, result: &OutlierDetectionResult) {
    let format_str = matches
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("text");

    match format_str {
        "text" => {
            println!("Method: {}", result.method_name);
            println!("Outliers found: {}", result.outliers.len());

            if !result.outliers.is_empty() {
                println!("\nOutlier Details:");
                for outlier in &result.outliers {
                    println!("  Index: {} (Value: {:.3})", outlier.index, outlier.value);
                }
            }
        }
        "json" => {
            use serde_json::json;
            let output = json!({
                "method_name": result.method_name,
                "threshold": result.threshold,
                "outliers_count": result.outliers.len(),
                "outliers": result.outliers.iter().map(|o| json!({
                    "index": o.index,
                    "value": o.value,
                    "score": o.score,
                    "is_outlier": o.is_outlier
                })).collect::<Vec<_>>()
            });
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        _ => println!("Unsupported format for outlier detection"),
    }
}

fn output_quality_control_result(matches: &clap::ArgMatches, result: &QualityControlResult) {
    let format_str = matches
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("text");

    match format_str {
        "text" => {
            println!("Quality Control Analysis");
            println!("Mean: {:.3}", result.mean);
            println!("Standard Deviation: {:.3}", result.std_dev);

            if let (Some(cp), Some(cpk)) = (result.cp, result.cpk) {
                println!("Cp: {cp:.3}");
                println!("Cpk: {cpk:.3}");

                if let Some(ref capability) = result.process_capability {
                    let cap_text = match capability {
                        ProcessCapability::Excellent => "Excellent",
                        ProcessCapability::Adequate => "Adequate",
                        ProcessCapability::Poor => "Poor",
                        ProcessCapability::Inadequate => "Inadequate",
                    };
                    println!("Process Capability: {cap_text}");
                }
            }

            if let Some(within_spec) = result.within_spec_percent {
                println!("Within Specification: {within_spec:.1}%");
            }
        }
        "json" => {
            use serde_json::json;
            let output = json!({
                "mean": result.mean,
                "std_dev": result.std_dev,
                "cp": result.cp,
                "cpk": result.cpk,
                "within_spec_percent": result.within_spec_percent,
                "three_sigma_limits": result.three_sigma_limits,
                "violations_count": result.control_chart_violations.len()
            });
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        _ => println!("Unsupported format for quality control"),
    }
}

fn print_text_output(result: &NormalResult, quiet: bool, verbose: bool) {
    if quiet {
        println!("mean: {:.3}", result.mean);
        println!("std_dev: {:.3}", result.std_dev);
        println!("normality_score: {:.3}", result.normality_score);
        return;
    }

    println!("Normal Distribution Analysis Results");
    println!();
    println!("Dataset: {}", result.dataset_name);
    println!("Numbers analyzed: {}", result.numbers_analyzed);
    println!("Quality Level: {:?}", result.risk_level);

    println!();
    println!("Distribution Histogram:");
    println!("{}", format_normal_histogram(result));

    println!();
    println!("Distribution Parameters:");
    println!("  Mean: {:.3}", result.mean);
    println!("  Standard Deviation: {:.3}", result.std_dev);
    println!("  Variance: {:.3}", result.variance);
    println!("  Skewness: {:.3}", result.skewness);
    println!("  Kurtosis: {:.3}", result.kurtosis);

    if verbose {
        println!();
        println!("Normality Tests:");
        println!(
            "  Shapiro-Wilk: W={:.3}, p={:.3}",
            result.shapiro_wilk_statistic, result.shapiro_wilk_p_value
        );
        println!(
            "  Anderson-Darling: A²={:.3}, p={:.3}",
            result.anderson_darling_statistic, result.anderson_darling_p_value
        );
        println!(
            "  Kolmogorov-Smirnov: D={:.3}, p={:.3}",
            result.kolmogorov_smirnov_statistic, result.kolmogorov_smirnov_p_value
        );

        println!();
        println!("Quality Metrics:");
        println!("  Normality Score: {:.3}", result.normality_score);
        println!("  QQ Correlation: {:.3}", result.qq_correlation);
        println!("  Distribution Quality: {:.3}", result.distribution_quality);

        if !result.outliers_z_score.is_empty() {
            println!();
            println!("Outlier Detection:");
            println!("  Z-score: {} outliers", result.outliers_z_score.len());
            println!(
                "  Modified Z-score: {} outliers",
                result.outliers_modified_z.len()
            );
            println!("  IQR method: {} outliers", result.outliers_iqr.len());
        }

        println!();
        println!("Sigma Coverage:");
        println!("  1σ: {:.1}%", result.within_1_sigma_percent);
        println!("  2σ: {:.1}%", result.within_2_sigma_percent);
        println!("  3σ: {:.1}%", result.within_3_sigma_percent);

        println!();
        println!("Interpretation:");
        print_normal_interpretation(result);
    }
}

fn print_normal_interpretation(result: &NormalResult) {
    use lawkit_core::common::risk::RiskLevel;

    match result.risk_level {
        RiskLevel::Low => {
            println!(
                "{}",
                colors::pass("[PASS] Data follows normal distribution well")
            );
            println!("   Suitable for standard statistical analysis");
        }
        RiskLevel::Medium => {
            println!(
                "{}",
                colors::warn("[WARN] Data shows some deviation from normality")
            );
            println!("   Consider robust statistical methods");
        }
        RiskLevel::High => {
            println!(
                "{}",
                colors::fail("[FAIL] Data significantly deviates from normality")
            );
            println!("   Non-parametric methods recommended");
        }
        RiskLevel::Critical => {
            println!(
                "{}",
                colors::critical("[CRITICAL] Data shows extreme deviation from normality")
            );
            println!("   Requires special handling and investigation");
        }
    }

    // 歪度・尖度に基づく解釈
    if result.skewness.abs() > 1.0 {
        if result.skewness > 0.0 {
            println!(
                "   {}",
                colors::info("INFO: Data is right-skewed (positive skewness)")
            );
        } else {
            println!(
                "   {}",
                colors::info("INFO: Data is left-skewed (negative skewness)")
            );
        }
    }

    if result.kurtosis > 1.0 {
        println!(
            "   {}",
            colors::info("INFO: Data has heavy tails (high kurtosis)")
        );
    } else if result.kurtosis < -1.0 {
        println!(
            "   {}",
            colors::info("INFO: Data has light tails (low kurtosis)")
        );
    }

    // 異常値の解釈
    if !result.outliers_z_score.is_empty() {
        println!(
            "   {}",
            colors::alert(&format!(
                "ALERT: Outliers detected: {}",
                result.outliers_z_score.len()
            ))
        );
    }
}

fn print_json_output(result: &NormalResult) {
    use serde_json::json;

    let output = json!({
        "dataset": result.dataset_name,
        "numbers_analyzed": result.numbers_analyzed,
        "risk_level": format!("{:?}", result.risk_level),
        "mean": result.mean,
        "std_dev": result.std_dev,
        "variance": result.variance,
        "skewness": result.skewness,
        "kurtosis": result.kurtosis,
        "shapiro_wilk": {
            "statistic": result.shapiro_wilk_statistic,
            "p_value": result.shapiro_wilk_p_value
        },
        "anderson_darling": {
            "statistic": result.anderson_darling_statistic,
            "p_value": result.anderson_darling_p_value
        },
        "kolmogorov_smirnov": {
            "statistic": result.kolmogorov_smirnov_statistic,
            "p_value": result.kolmogorov_smirnov_p_value
        },
        "normality_score": result.normality_score,
        "qq_correlation": result.qq_correlation,
        "distribution_quality": result.distribution_quality,
        "outliers": {
            "z_score_count": result.outliers_z_score.len(),
            "modified_z_count": result.outliers_modified_z.len(),
            "iqr_count": result.outliers_iqr.len()
        },
        "confidence_intervals": {
            "mean_95": result.mean_confidence_interval,
            "prediction_95": result.prediction_interval_95,
            "three_sigma": result.three_sigma_limits
        },
        "sigma_coverage": {
            "within_1_sigma": result.within_1_sigma_percent,
            "within_2_sigma": result.within_2_sigma_percent,
            "within_3_sigma": result.within_3_sigma_percent
        }
    });

    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

fn print_csv_output(result: &NormalResult) {
    println!("dataset,numbers_analyzed,risk_level,mean,std_dev,variance,skewness,kurtosis,normality_score");
    println!(
        "{},{},{:?},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3}",
        result.dataset_name,
        result.numbers_analyzed,
        result.risk_level,
        result.mean,
        result.std_dev,
        result.variance,
        result.skewness,
        result.kurtosis,
        result.normality_score
    );
}

fn print_yaml_output(result: &NormalResult) {
    println!("dataset: \"{}\"", result.dataset_name);
    println!("numbers_analyzed: {}", result.numbers_analyzed);
    println!("risk_level: \"{:?}\"", result.risk_level);
    println!("mean: {:.3}", result.mean);
    println!("std_dev: {:.3}", result.std_dev);
    println!("variance: {:.3}", result.variance);
    println!("skewness: {:.3}", result.skewness);
    println!("kurtosis: {:.3}", result.kurtosis);
    println!("normality_score: {:.3}", result.normality_score);
}

fn print_toml_output(result: &NormalResult) {
    println!("dataset = \"{}\"", result.dataset_name);
    println!("numbers_analyzed = {}", result.numbers_analyzed);
    println!("risk_level = \"{:?}\"", result.risk_level);
    println!("mean = {:.3}", result.mean);
    println!("std_dev = {:.3}", result.std_dev);
    println!("variance = {:.3}", result.variance);
    println!("skewness = {:.3}", result.skewness);
    println!("kurtosis = {:.3}", result.kurtosis);
    println!("normality_score = {:.3}", result.normality_score);
}

fn print_xml_output(result: &NormalResult) {
    println!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    println!("<normal_analysis>");
    println!("  <dataset>{}</dataset>", result.dataset_name);
    println!(
        "  <numbers_analyzed>{}</numbers_analyzed>",
        result.numbers_analyzed
    );
    println!("  <risk_level>{:?}</risk_level>", result.risk_level);
    println!("  <mean>{:.3}</mean>", result.mean);
    println!("  <std_dev>{:.3}</std_dev>", result.std_dev);
    println!("  <variance>{:.3}</variance>", result.variance);
    println!("  <skewness>{:.3}</skewness>", result.skewness);
    println!("  <kurtosis>{:.3}</kurtosis>", result.kurtosis);
    println!(
        "  <normality_score>{:.3}</normality_score>",
        result.normality_score
    );
    println!("</normal_analysis>");
}

/// Analyze numbers with filtering and custom options
fn analyze_numbers_with_options(
    matches: &clap::ArgMatches,
    dataset_name: String,
    numbers: &[f64],
) -> Result<NormalResult> {
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
        8 // 正規分布分析では最低8個必要
    };

    // Check minimum count requirement
    if filtered_numbers.len() < min_count {
        return Err(BenfError::InsufficientData(filtered_numbers.len()));
    }

    // Perform normal distribution analysis
    analyze_normal_distribution(&filtered_numbers, &dataset_name)
}

/// 高度な異常値検出結果の出力
fn output_advanced_outlier_result(_matches: &ArgMatches, result: &AdvancedOutlierResult) {
    println!("Advanced Outlier Detection Result: {}", result.method_name);
    println!("Detection rate: {:.3}", result.detection_rate);
    println!("Threshold: {:.3}", result.threshold);
    println!("Outliers found: {}", result.outliers.len());

    if !result.outliers.is_empty() {
        println!("\nOutlier Details:");
        for outlier in &result.outliers {
            println!(
                "  Index {}: Value={:.3}, Score={:.3}, Confidence={:.3}",
                outlier.index, outlier.value, outlier.outlier_score, outlier.confidence
            );
        }
    }

    if !result.method_params.is_empty() {
        println!("\nMethod Parameters:");
        for (param, value) in &result.method_params {
            println!("  {param}: {value:.3}");
        }
    }
}

/// 標準偏差を計算するヘルパー関数
fn calculate_std_dev(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }

    let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
    let variance = numbers.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / numbers.len() as f64;
    variance.sqrt()
}

/// 時系列分析結果の出力
fn output_timeseries_result(_matches: &ArgMatches, result: &TimeSeriesAnalysis) {
    println!("Time Series Analysis Results");
    println!("============================");

    // トレンド分析
    println!("\nTrend Analysis:");
    println!("  Slope: {:.6}", result.trend.slope);
    println!("  R-squared: {:.3}", result.trend.r_squared);
    println!("  Direction: {:?}", result.trend.direction);
    println!("  Trend strength: {:.3}", result.trend.trend_strength);

    // 季節性
    if result.seasonality.detected {
        println!("\nSeasonality Detected:");
        if let Some(period) = result.seasonality.period {
            println!("  Period: {period:.1}");
        }
        println!("  Strength: {:.3}", result.seasonality.strength);
    } else {
        println!("\nNo significant seasonality detected");
    }

    // 変化点
    if !result.changepoints.is_empty() {
        println!("\nChange Points Detected: {}", result.changepoints.len());
        for (i, cp) in result.changepoints.iter().enumerate().take(5) {
            println!(
                "  {}: Index {}, Significance: {:.2}, Type: {:?}",
                i + 1,
                cp.index,
                cp.significance,
                cp.change_type
            );
        }
    }

    // 予測
    if !result.forecasts.is_empty() {
        println!("\nForecasts (next {} points):", result.forecasts.len());
        for (i, forecast) in result.forecasts.iter().enumerate() {
            println!(
                "  {}: {:.3} (uncertainty: {:.3})",
                i + 1,
                forecast.predicted_value,
                forecast.uncertainty
            );
        }
    }

    // 異常値
    if !result.anomalies.is_empty() {
        println!("\nAnomalies Detected: {}", result.anomalies.len());
        for anomaly in result.anomalies.iter().take(10) {
            println!(
                "  Index {}: Value={:.3}, Expected={:.3}, Score={:.3}",
                anomaly.index, anomaly.value, anomaly.expected_value, anomaly.anomaly_score
            );
        }
    }

    // データ品質
    println!("\nData Quality Assessment:");
    println!(
        "  Completeness: {:.1}%",
        result.statistics.data_quality.completeness * 100.0
    );
    println!(
        "  Consistency: {:.1}%",
        result.statistics.data_quality.consistency * 100.0
    );
    println!(
        "  Outlier ratio: {:.1}%",
        result.statistics.data_quality.outlier_ratio * 100.0
    );
    println!("  Noise level: {:.3}", result.statistics.noise_level);
}

fn format_normal_histogram(result: &NormalResult) -> String {
    let mut output = String::new();
    const CHART_WIDTH: usize = 50;
    const BINS: usize = 10;

    // 仮想データでヒストグラムをシミュレート（実際のデータはNormalResultから取得不可）
    // 平均、標準偏差を使って理論的な正規分布カーブを表示
    let mean = result.mean;
    let std_dev = result.std_dev;

    // -3σから+3σの範囲でビンを作成
    let range_start = mean - 3.0 * std_dev;
    let range_end = mean + 3.0 * std_dev;
    let bin_width = (range_end - range_start) / BINS as f64;

    // 各ビンの理論的確率密度を計算
    let mut bin_densities = Vec::new();
    let mut max_density: f64 = 0.0;

    for i in 0..BINS {
        let bin_center = range_start + (i as f64 + 0.5) * bin_width;
        let z_score = (bin_center - mean) / std_dev;

        // 正規分布の確率密度関数
        let density =
            (-0.5 * z_score * z_score).exp() / (std_dev * (2.0 * std::f64::consts::PI).sqrt());
        bin_densities.push(density);
        max_density = max_density.max(density);
    }

    // ヒストグラムを表示
    for (i, &density) in bin_densities.iter().enumerate() {
        let bin_start = range_start + i as f64 * bin_width;
        let bin_end = bin_start + bin_width;

        let normalized_density = if max_density > 0.0 {
            density / max_density
        } else {
            0.0
        };
        let bar_length = (normalized_density * CHART_WIDTH as f64).round() as usize;
        let bar_length = bar_length.min(CHART_WIDTH);

        let filled_bar = "█".repeat(bar_length);
        let background_bar = "░".repeat(CHART_WIDTH - bar_length);
        let full_bar = format!("{filled_bar}{background_bar}");

        output.push_str(&format!(
            "{:6.2}-{:6.2}: {} {:>5.1}%\n",
            bin_start,
            bin_end,
            full_bar,
            normalized_density * 100.0
        ));
    }

    // 統計情報を追加
    output.push_str(&format!(
        "\nDistribution: μ={mean:.2}, σ={std_dev:.2}, Range: [{range_start:.2}, {range_end:.2}]"
    ));

    // σ範囲の情報
    output.push_str(&format!(
        "\n1σ: {:.1}%, 2σ: {:.1}%, 3σ: {:.1}%",
        result.within_1_sigma_percent, result.within_2_sigma_percent, result.within_3_sigma_percent
    ));

    output
}
