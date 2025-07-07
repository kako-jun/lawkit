use crate::common_options::{get_optimized_reader, setup_optimization_config};
use clap::ArgMatches;
use lawkit_core::{
    common::{
        filtering::{apply_number_filter, NumberFilter},
        input::parse_text_input,
        outliers::{
            detect_outliers_dbscan, detect_outliers_ensemble, detect_outliers_isolation,
            detect_outliers_lof, AdvancedOutlierResult,
        },
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
    // 最適化設定をセットアップ
    let (use_optimize, _parallel_config, _memory_config) = setup_optimization_config(matches);

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
            println!(
                "{}: {}",
                "English Text",
                result.test_name
            );
            println!(
                "{}: {:.6}",
                "English Text",
                result.statistic
            );
            println!(
                "{}: {:.6}",
                "English Text",
                result.p_value
            );
            println!(
                "{}: {}",
                "English Text",
                if result.is_normal {
                    "English Text"
                } else {
                    "English Text"
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
            println!(
                "{}: {}",
                "English Text",
                result.method_name
            );
            println!(
                "{}: {}",
                "English Text",
                result.outliers.len()
            );

            if !result.outliers.is_empty() {
                println!("\n{}:", "English Text");
                for outlier in &result.outliers {
                    println!(
                        "  {}: {} ({}: {:.3})",
                        "English Text",
                        outlier.index,
                        "English Text",
                        outlier.value
                    );
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
            println!("{}", "English Text");
            println!("{}: {:.3}", "English Text", result.mean);
            println!(
                "{}: {:.3}",
                "English Text",
                result.std_dev
            );

            if let (Some(cp), Some(cpk)) = (result.cp, result.cpk) {
                println!("{}: {:.3}", "English Text", cp);
                println!("{}: {:.3}", "English Text", cpk);

                if let Some(ref capability) = result.process_capability {
                    let cap_text = match capability {
                        ProcessCapability::Excellent => "English Text",
                        ProcessCapability::Adequate => "English Text",
                        ProcessCapability::Poor => "English Text",
                        ProcessCapability::Inadequate => "English Text",
                    };
                    println!(
                        "{}: {}",
                        "English Text",
                        cap_text
                    );
                }
            }

            if let Some(within_spec) = result.within_spec_percent {
                println!(
                    "{}: {:.1}%",
                    "English Text",
                    within_spec
                );
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

    println!("{}", "Normal Distribution Analysis Results");
    println!();
    println!(
        "{}: {}",
        "Dataset",
        result.dataset_name
    );
    println!(
        "{}: {}",
        "Numbers analyzed",
        result.numbers_analyzed
    );
    println!(
        "{}: {:?}",
        "Quality Level",
        result.risk_level
    );

    println!();
    println!("Distribution Parameters:");
    println!("  {}: {:.3}", "English Text", result.mean);
    println!(
        "  {}: {:.3}",
        "English Text",
        result.std_dev
    );
    println!(
        "  {}: {:.3}",
        "English Text",
        result.variance
    );
    println!(
        "  {}: {:.3}",
        "English Text",
        result.skewness
    );
    println!(
        "  {}: {:.3}",
        "English Text",
        result.kurtosis
    );

    if verbose {
        println!();
        println!("Distribution Parameters:");
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
        println!("Distribution Parameters:");
        println!(
            "  {}: {:.3}",
            "English Text",
            result.normality_score
        );
        println!(
            "  {}: {:.3}",
            "English Text",
            result.qq_correlation
        );
        println!(
            "  {}: {:.3}",
            "English Text",
            result.distribution_quality
        );

        if !result.outliers_z_score.is_empty() {
            println!();
            println!("Distribution Parameters:");
            println!(
                "  Z-score: {} {}",
                result.outliers_z_score.len(),
                "English Text"
            );
            println!(
                "  Modified Z-score: {} {}",
                result.outliers_modified_z.len(),
                "English Text"
            );
            println!(
                "  IQR method: {} {}",
                result.outliers_iqr.len(),
                "English Text"
            );
        }

        println!();
        println!("Distribution Parameters:");
        println!("  1σ: {:.1}%", result.within_1_sigma_percent);
        println!("  2σ: {:.1}%", result.within_2_sigma_percent);
        println!("  3σ: {:.1}%", result.within_3_sigma_percent);

        println!();
        println!("Distribution Parameters:");
        print_normal_interpretation(result);
    }
}

fn print_normal_interpretation(result: &NormalResult) {
    use lawkit_core::common::risk::RiskLevel;

    match result.risk_level {
        RiskLevel::Low => {
            println!("✅ {}", "English Text");
            println!("   {}", "English Text");
        }
        RiskLevel::Medium => {
            println!("⚠️  {}", "English Text");
            println!("   {}", "English Text");
        }
        RiskLevel::High => {
            println!("🚨 {}", "English Text");
            println!("   {}", "English Text");
        }
        RiskLevel::Critical => {
            println!("🔍 {}", "English Text");
            println!("   {}", "English Text");
        }
    }

    // 歪度・尖度に基づく解釈
    if result.skewness.abs() > 1.0 {
        if result.skewness > 0.0 {
            println!("   📊 {}", "English Text");
        } else {
            println!("   📊 {}", "English Text");
        }
    }

    if result.kurtosis > 1.0 {
        println!("   📈 {}", "English Text");
    } else if result.kurtosis < -1.0 {
        println!("   📉 {}", "English Text");
    }

    // 異常値の解釈
    if !result.outliers_z_score.is_empty() {
        println!(
            "   🎯 {}: {}",
            "English Text",
            result.outliers_z_score.len()
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
fn output_advanced_outlier_result(matches: &ArgMatches, result: &AdvancedOutlierResult) {
    let _lang = matches
        .get_one::<String>("language")
        .map(|s| s.as_str())
        .unwrap_or("auto");

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
