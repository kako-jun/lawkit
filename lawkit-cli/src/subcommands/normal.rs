use clap::ArgMatches;
use lawkit_core::{
    common::{
        filtering::{apply_number_filter, NumberFilter},
        input::{parse_input_auto, parse_text_input},
        outliers::{
            detect_outliers_ensemble, detect_outliers_isolation, detect_outliers_lof,
            detect_outliers_dbscan, AdvancedOutlierResult,
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
use std::io::{self, Read};

pub fn run(matches: &ArgMatches) -> Result<()> {
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
        if test_type != "all" {  // "all"はデフォルトなので通常分析モードで処理
            return run_normality_test_mode(matches, test_type);
        }
    }

    // 通常の正規分布分析モード
    if let Some(input) = matches.get_one::<String>("input") {
        match parse_input_auto(input) {
            Ok(numbers) => {
                if numbers.is_empty() {
                    let language = get_language(matches);
                    let error_msg = localized_text("no_numbers_found", language);
                    eprintln!("{}", error_msg);
                    std::process::exit(1);
                }

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

                output_results(matches, &result);
                std::process::exit(result.risk_level.exit_code());
            }
            Err(e) => {
                eprintln!("Error processing input '{}': {}", input, e);
                std::process::exit(1);
            }
        }
    } else {
        // 標準入力処理
        let mut buffer = String::new();
        match io::stdin().read_to_string(&mut buffer) {
            Ok(_) => {
                if buffer.trim().is_empty() {
                    eprintln!("Error: No input provided. Use --help for usage information.");
                    std::process::exit(2);
                }

                let numbers = match parse_text_input(&buffer) {
                    Ok(numbers) => numbers,
                    Err(e) => {
                        let language = get_language(matches);
                        let error_msg = localized_text("analysis_error", language);
                        eprintln!("{}: {}", error_msg, e);
                        std::process::exit(1);
                    }
                };

                let result =
                    match analyze_numbers_with_options(matches, "stdin".to_string(), &numbers) {
                        Ok(result) => result,
                        Err(e) => {
                            let language = get_language(matches);
                            let error_msg = localized_text("analysis_error", language);
                            eprintln!("{}: {}", error_msg, e);
                            std::process::exit(1);
                        }
                    };

                output_results(matches, &result);
                std::process::exit(result.risk_level.exit_code());
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                std::process::exit(1);
            }
        }
    }
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
                "Error: Unknown test type '{}'. Available: shapiro, anderson, ks, all",
                test_type
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
                        "Error: Unknown outlier detection method '{}'. Available: zscore, modified_zscore, iqr, lof, isolation, dbscan, ensemble",
                        method_str
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
    if let Some(input) = matches.get_one::<String>("input") {
        parse_input_auto(input)
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| BenfError::ParseError(e.to_string()))?;
        parse_text_input(&buffer)
    }
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

fn output_normality_test_result(matches: &clap::ArgMatches, result: &NormalityTestResult) {
    let language = get_language(matches);
    let format_str = matches
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("text");

    match format_str {
        "text" => {
            println!(
                "{}: {}",
                localized_text("normality_test_result", language),
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
            println!(
                "{}: {}",
                localized_text("is_normal", language),
                if result.is_normal {
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
                "is_normal": result.is_normal
            });
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        _ => print_text_output(
            &NormalResult::new("test".to_string(), &[0.0; 10]).unwrap(),
            false,
            false,
            language,
        ),
    }
}

fn output_outlier_detection_result(matches: &clap::ArgMatches, result: &OutlierDetectionResult) {
    let language = get_language(matches);
    let format_str = matches
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("text");

    match format_str {
        "text" => {
            println!(
                "{}: {}",
                localized_text("outlier_detection_result", language),
                result.method_name
            );
            println!(
                "{}: {}",
                localized_text("outliers_found", language),
                result.outliers.len()
            );

            if !result.outliers.is_empty() {
                println!("\n{}:", localized_text("outlier_details", language));
                for outlier in &result.outliers {
                    println!(
                        "  {}: {} ({}: {:.3})",
                        localized_text("index", language),
                        outlier.index,
                        localized_text("value", language),
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
    let language = get_language(matches);
    let format_str = matches
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("text");

    match format_str {
        "text" => {
            println!("{}", localized_text("quality_control_result", language));
            println!("{}: {:.3}", localized_text("mean", language), result.mean);
            println!(
                "{}: {:.3}",
                localized_text("std_dev", language),
                result.std_dev
            );

            if let (Some(cp), Some(cpk)) = (result.cp, result.cpk) {
                println!("{}: {:.3}", localized_text("cp_index", language), cp);
                println!("{}: {:.3}", localized_text("cpk_index", language), cpk);

                if let Some(ref capability) = result.process_capability {
                    let cap_text = match capability {
                        ProcessCapability::Excellent => localized_text("excellent", language),
                        ProcessCapability::Adequate => localized_text("adequate", language),
                        ProcessCapability::Poor => localized_text("poor", language),
                        ProcessCapability::Inadequate => localized_text("inadequate", language),
                    };
                    println!(
                        "{}: {}",
                        localized_text("process_capability", language),
                        cap_text
                    );
                }
            }

            if let Some(within_spec) = result.within_spec_percent {
                println!(
                    "{}: {:.1}%",
                    localized_text("within_spec", language),
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

fn print_text_output(result: &NormalResult, quiet: bool, verbose: bool, lang: &str) {
    if quiet {
        println!("mean: {:.3}", result.mean);
        println!("std_dev: {:.3}", result.std_dev);
        println!("normality_score: {:.3}", result.normality_score);
        return;
    }

    println!("{}", localized_text("normal_analysis_results", lang));
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
    println!("{}:", localized_text("distribution_parameters", lang));
    println!("  {}: {:.3}", localized_text("mean", lang), result.mean);
    println!(
        "  {}: {:.3}",
        localized_text("std_dev", lang),
        result.std_dev
    );
    println!(
        "  {}: {:.3}",
        localized_text("variance", lang),
        result.variance
    );
    println!(
        "  {}: {:.3}",
        localized_text("skewness", lang),
        result.skewness
    );
    println!(
        "  {}: {:.3}",
        localized_text("kurtosis", lang),
        result.kurtosis
    );

    if verbose {
        println!();
        println!("{}:", localized_text("normality_tests", lang));
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
        println!("{}:", localized_text("fit_assessment", lang));
        println!(
            "  {}: {:.3}",
            localized_text("normality_score", lang),
            result.normality_score
        );
        println!(
            "  {}: {:.3}",
            localized_text("qq_correlation", lang),
            result.qq_correlation
        );
        println!(
            "  {}: {:.3}",
            localized_text("distribution_quality", lang),
            result.distribution_quality
        );

        if !result.outliers_z_score.is_empty() {
            println!();
            println!("{}:", localized_text("outliers_detected", lang));
            println!(
                "  Z-score: {} {}",
                result.outliers_z_score.len(),
                localized_text("outliers", lang)
            );
            println!(
                "  Modified Z-score: {} {}",
                result.outliers_modified_z.len(),
                localized_text("outliers", lang)
            );
            println!(
                "  IQR method: {} {}",
                result.outliers_iqr.len(),
                localized_text("outliers", lang)
            );
        }

        println!();
        println!("{}:", localized_text("sigma_coverage", lang));
        println!("  1σ: {:.1}%", result.within_1_sigma_percent);
        println!("  2σ: {:.1}%", result.within_2_sigma_percent);
        println!("  3σ: {:.1}%", result.within_3_sigma_percent);

        println!();
        println!("{}:", localized_text("interpretation", lang));
        print_normal_interpretation(result, lang);
    }
}

fn print_normal_interpretation(result: &NormalResult, lang: &str) {
    use lawkit_core::common::risk::RiskLevel;

    match result.risk_level {
        RiskLevel::Low => {
            println!("✅ {}", localized_text("excellent_normality", lang));
            println!("   {}", localized_text("data_follows_normal", lang));
        }
        RiskLevel::Medium => {
            println!("⚠️  {}", localized_text("good_normality", lang));
            println!("   {}", localized_text("minor_deviations", lang));
        }
        RiskLevel::High => {
            println!("🚨 {}", localized_text("poor_normality", lang));
            println!("   {}", localized_text("significant_deviations", lang));
        }
        RiskLevel::Critical => {
            println!("🔍 {}", localized_text("very_poor_normality", lang));
            println!("   {}", localized_text("major_deviations", lang));
        }
    }

    // 歪度・尖度に基づく解釈
    if result.skewness.abs() > 1.0 {
        if result.skewness > 0.0 {
            println!("   📊 {}", localized_text("right_skewed", lang));
        } else {
            println!("   📊 {}", localized_text("left_skewed", lang));
        }
    }

    if result.kurtosis > 1.0 {
        println!("   📈 {}", localized_text("heavy_tailed", lang));
    } else if result.kurtosis < -1.0 {
        println!("   📉 {}", localized_text("light_tailed", lang));
    }

    // 異常値の解釈
    if !result.outliers_z_score.is_empty() {
        println!(
            "   🎯 {}: {}",
            localized_text("outliers_detected", lang),
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
        ("en", "normal_analysis_results") => "Normal Distribution Analysis Results",
        ("en", "dataset") => "Dataset",
        ("en", "numbers_analyzed") => "Numbers analyzed",
        ("en", "risk_level") => "Quality Level",
        ("en", "distribution_parameters") => "Distribution Parameters",
        ("en", "mean") => "Mean",
        ("en", "std_dev") => "Standard deviation",
        ("en", "variance") => "Variance",
        ("en", "skewness") => "Skewness",
        ("en", "kurtosis") => "Kurtosis",
        ("en", "normality_tests") => "Normality Tests",
        ("en", "fit_assessment") => "Fit Assessment",
        ("en", "normality_score") => "Normality score",
        ("en", "qq_correlation") => "Q-Q correlation",
        ("en", "distribution_quality") => "Distribution quality",
        ("en", "outliers_detected") => "Outliers detected",
        ("en", "outliers") => "outliers",
        ("en", "sigma_coverage") => "Sigma Coverage",
        ("en", "interpretation") => "Interpretation",
        ("en", "excellent_normality") => "Excellent normal distribution fit",
        ("en", "data_follows_normal") => "Data closely follows normal distribution",
        ("en", "good_normality") => "Good normal distribution fit",
        ("en", "minor_deviations") => "Minor deviations from normality",
        ("en", "poor_normality") => "Poor normal distribution fit",
        ("en", "significant_deviations") => "Significant deviations from normality",
        ("en", "very_poor_normality") => "Very poor normal distribution fit",
        ("en", "major_deviations") => "Major deviations from normality",
        ("en", "right_skewed") => "Distribution is right-skewed",
        ("en", "left_skewed") => "Distribution is left-skewed",
        ("en", "heavy_tailed") => "Distribution has heavy tails",
        ("en", "light_tailed") => "Distribution has light tails",
        ("en", "normality_test_result") => "Normality Test Result",
        ("en", "test_statistic") => "Test statistic",
        ("en", "p_value") => "P-value",
        ("en", "is_normal") => "Is normal",
        ("en", "yes") => "Yes",
        ("en", "no") => "No",
        ("en", "outlier_detection_result") => "Outlier Detection Result",
        ("en", "outliers_found") => "Outliers found",
        ("en", "outlier_details") => "Outlier Details",
        ("en", "index") => "Index",
        ("en", "value") => "Value",
        ("en", "quality_control_result") => "Quality Control Analysis",
        ("en", "cp_index") => "Cp index",
        ("en", "cpk_index") => "Cpk index",
        ("en", "process_capability") => "Process capability",
        ("en", "excellent") => "Excellent",
        ("en", "adequate") => "Adequate",
        ("en", "poor") => "Poor",
        ("en", "inadequate") => "Inadequate",
        ("en", "within_spec") => "Within specifications",
        ("en", "unsupported_format") => "Error: Unsupported output format",
        ("en", "no_numbers_found") => "Error: No valid numbers found in input",
        ("en", "analysis_error") => "Analysis error",

        // 日本語
        ("ja", "normal_analysis_results") => "正規分布解析結果",
        ("ja", "dataset") => "データセット",
        ("ja", "numbers_analyzed") => "解析した数値数",
        ("ja", "risk_level") => "品質レベル",
        ("ja", "distribution_parameters") => "分布パラメータ",
        ("ja", "mean") => "平均",
        ("ja", "std_dev") => "標準偏差",
        ("ja", "variance") => "分散",
        ("ja", "skewness") => "歪度",
        ("ja", "kurtosis") => "尖度",
        ("ja", "normality_tests") => "正規性検定",
        ("ja", "fit_assessment") => "適合度評価",
        ("ja", "normality_score") => "正規性スコア",
        ("ja", "qq_correlation") => "Q-Q相関",
        ("ja", "distribution_quality") => "分布品質",
        ("ja", "outliers_detected") => "検出された外れ値",
        ("ja", "outliers") => "個の外れ値",
        ("ja", "sigma_coverage") => "σ範囲カバー率",
        ("ja", "interpretation") => "解釈",
        ("ja", "excellent_normality") => "優れた正規分布適合",
        ("ja", "data_follows_normal") => "データは正規分布に良く従っています",
        ("ja", "good_normality") => "良い正規分布適合",
        ("ja", "minor_deviations") => "正規性からの軽微な偏差",
        ("ja", "poor_normality") => "不十分な正規分布適合",
        ("ja", "significant_deviations") => "正規性からの有意な偏差",
        ("ja", "very_poor_normality") => "非常に不十分な正規分布適合",
        ("ja", "major_deviations") => "正規性からの重大な偏差",
        ("ja", "right_skewed") => "分布は右に偏っています",
        ("ja", "left_skewed") => "分布は左に偏っています",
        ("ja", "heavy_tailed") => "分布は裾が重い",
        ("ja", "light_tailed") => "分布は裾が軽い",
        ("ja", "normality_test_result") => "正規性検定結果",
        ("ja", "test_statistic") => "検定統計量",
        ("ja", "p_value") => "p値",
        ("ja", "is_normal") => "正規分布か",
        ("ja", "yes") => "はい",
        ("ja", "no") => "いいえ",
        ("ja", "outlier_detection_result") => "外れ値検出結果",
        ("ja", "outliers_found") => "検出された外れ値数",
        ("ja", "outlier_details") => "外れ値詳細",
        ("ja", "index") => "インデックス",
        ("ja", "value") => "値",
        ("ja", "quality_control_result") => "品質管理分析",
        ("ja", "cp_index") => "Cp指数",
        ("ja", "cpk_index") => "Cpk指数",
        ("ja", "process_capability") => "工程能力",
        ("ja", "excellent") => "優秀",
        ("ja", "adequate") => "適切",
        ("ja", "poor") => "不十分",
        ("ja", "inadequate") => "不適切",
        ("ja", "within_spec") => "規格内率",
        ("ja", "unsupported_format") => "エラー: サポートされていない出力形式",
        ("ja", "no_numbers_found") => "エラー: 入力に有効な数値が見つかりません",
        ("ja", "analysis_error") => "解析エラー",

        // Default English
        (_, "normal_analysis_results") => "Normal Distribution Analysis Results",
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
) -> Result<NormalResult> {
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
    let lang = matches
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
            println!("  {}: {:.3}", param, value);
        }
    }
}

/// 標準偏差を計算するヘルパー関数
fn calculate_std_dev(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    
    let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
    let variance = numbers.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / numbers.len() as f64;
    variance.sqrt()
}

/// 時系列分析結果の出力
fn output_timeseries_result(matches: &ArgMatches, result: &TimeSeriesAnalysis) {
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
            println!("  Period: {:.1}", period);
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
                i + 1, cp.index, cp.significance, cp.change_type
            );
        }
    }
    
    // 予測
    if !result.forecasts.is_empty() {
        println!("\nForecasts (next {} points):", result.forecasts.len());
        for (i, forecast) in result.forecasts.iter().enumerate() {
            println!(
                "  {}: {:.3} (uncertainty: {:.3})",
                i + 1, forecast.predicted_value, forecast.uncertainty
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
    println!("  Completeness: {:.1}%", result.statistics.data_quality.completeness * 100.0);
    println!("  Consistency: {:.1}%", result.statistics.data_quality.consistency * 100.0);
    println!("  Outlier ratio: {:.1}%", result.statistics.data_quality.outlier_ratio * 100.0);
    println!("  Noise level: {:.3}", result.statistics.noise_level);
}
