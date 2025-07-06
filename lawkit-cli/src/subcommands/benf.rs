use clap::ArgMatches;
use lawkit_core::{
    common::{
        filtering::{apply_number_filter, NumberFilter, RiskThreshold},
        input::{parse_input_auto, parse_text_input},
        streaming_io::OptimizedFileReader,
    },
    error::{BenfError, Result},
    laws::benford::BenfordResult,
};
use std::io::{self, Read};
use std::str::FromStr;

pub fn run(matches: &ArgMatches) -> Result<()> {
    // Determine input source based on arguments
    if let Some(input) = matches.get_one::<String>("input") {
        // Use auto-detection for file vs string input
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
    } else {
        // Read from stdin - use optimizations only if explicitly requested
        let use_optimize = matches.get_flag("optimize");

        if use_optimize {
            // 最適化処理：--optimize フラグ指定時（ストリーミング+並列+メモリ効率化）
            let mut reader = OptimizedFileReader::from_stdin();

            if std::env::var("LAWKIT_DEBUG").is_ok() {
                eprintln!("Debug: Using optimize mode (streaming + memory efficiency)");
            }

            let numbers = match reader
                .read_lines_streaming(|line| parse_text_input(&line).map(Some).or(Ok(None)))
            {
                Ok(nested_numbers) => nested_numbers.into_iter().flatten().collect::<Vec<_>>(),
                Err(e) => {
                    let language = get_language(matches);
                    let error_msg = localized_text("analysis_error", language);
                    eprintln!("{}: {}", error_msg, e);
                    std::process::exit(1);
                }
            };

            if numbers.is_empty() {
                let language = get_language(matches);
                let error_msg = localized_text("no_numbers_found", language);
                eprintln!("{}", error_msg);
                std::process::exit(1);
            }

            // 分析実行
            let result = match analyze_numbers_with_options(matches, "stdin".to_string(), &numbers)
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
        } else {
            // 従来のメモリ処理：デフォルト
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
                            let language = get_language(matches);
                            let error_msg = localized_text("analysis_error", language);
                            eprintln!("{}: {}", error_msg, e);
                            std::process::exit(1);
                        }
                    };

                    if numbers.is_empty() {
                        let language = get_language(matches);
                        let error_msg = localized_text("no_numbers_found", language);
                        eprintln!("{}", error_msg);
                        std::process::exit(1);
                    }

                    // Apply filtering and custom analysis
                    let result = match analyze_numbers_with_options(
                        matches,
                        "stdin".to_string(),
                        &numbers,
                    ) {
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
                    eprintln!("Error reading from stdin: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn output_results(matches: &clap::ArgMatches, result: &BenfordResult) {
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

fn print_text_output(result: &BenfordResult, quiet: bool, verbose: bool, lang: &str) {
    if quiet {
        for (i, &observed) in result.digit_distribution.iter().enumerate() {
            println!("{}: {:.1}%", i + 1, observed);
        }
        return;
    }

    println!("{}", localized_text("analysis_results", lang));
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
        println!("{}:", localized_text("digit_distribution", lang));
        for (i, &observed) in result.digit_distribution.iter().enumerate() {
            let digit = i + 1;
            let expected = result.expected_distribution[i];
            let deviation = observed - expected;

            println!(
                "{}: {:.1}% ({}: {:.1}%, {}: {:+.1}%)",
                digit,
                observed,
                localized_text("expected", lang),
                expected,
                localized_text("deviation", lang),
                deviation
            );
        }

        println!();
        println!("{}:", localized_text("statistical_tests", lang));
        println!(
            "{}: {:.2} ({}: {:.6})",
            localized_text("chi_square", lang),
            result.chi_square,
            localized_text("p_value", lang),
            result.p_value
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

fn get_language(matches: &clap::ArgMatches) -> &str {
    match matches.get_one::<String>("language").map(|s| s.as_str()) {
        Some("auto") | None => {
            // OSの言語設定を検出
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
        Some(_) => "en", // デフォルトは英語
    }
}

fn localized_text(key: &str, lang: &str) -> &'static str {
    match (lang, key) {
        // English
        ("en", "analysis_results") => "Benford's Law Analysis Results",
        ("en", "dataset") => "Dataset",
        ("en", "numbers_analyzed") => "Numbers analyzed",
        ("en", "risk_level") => "Attention Level",
        ("en", "digit_distribution") => "First Digit Distribution",
        ("en", "expected") => "expected",
        ("en", "deviation") => "deviation",
        ("en", "statistical_tests") => "Statistical Tests",
        ("en", "chi_square") => "Chi-square",
        ("en", "p_value") => "p-value",
        ("en", "mean_absolute_deviation") => "Mean Absolute Deviation",
        ("en", "interpretation") => "Interpretation",
        ("en", "normal_distribution") => "✅ Normal distribution - data appears natural",
        ("en", "slight_deviation") => "⚠️  Slight deviation - worth monitoring",
        ("en", "significant_deviation") => "🚨 Significant deviation - potential anomaly detected",
        ("en", "critical_deviation") => {
            "🔍 Significant attention needed - strong evidence of patterns"
        }
        ("en", "unsupported_format") => "Error: Unsupported output format",
        ("en", "no_numbers_found") => "Error: No valid numbers found in input",
        ("en", "analysis_error") => "Analysis error",

        // 日本語
        ("ja", "analysis_results") => "ベンフォードの法則解析結果",
        ("ja", "dataset") => "データセット",
        ("ja", "numbers_analyzed") => "解析した数値数",
        ("ja", "risk_level") => "注意レベル",
        ("ja", "digit_distribution") => "先頭桁分布",
        ("ja", "expected") => "期待値",
        ("ja", "deviation") => "偏差",
        ("ja", "statistical_tests") => "統計検定",
        ("ja", "chi_square") => "カイ二乗値",
        ("ja", "p_value") => "p値",
        ("ja", "mean_absolute_deviation") => "平均絶対偏差",
        ("ja", "interpretation") => "解釈",
        ("ja", "normal_distribution") => "✅ 正常な分布 - データは自然に見えます",
        ("ja", "slight_deviation") => "⚠️  軽微な偏差 - 監視が必要です",
        ("ja", "significant_deviation") => "🚨 有意な偏差 - 異常の可能性があります",
        ("ja", "critical_deviation") => "🔍 特に注意が必要 - パターンの強い証拠",
        ("ja", "unsupported_format") => "エラー: サポートされていない出力形式",
        ("ja", "no_numbers_found") => "エラー: 入力に有効な数値が見つかりません",
        ("ja", "analysis_error") => "解析エラー",

        // 中国語（简体）
        ("zh", "analysis_results") => "本福德定律分析结果",
        ("zh", "dataset") => "数据集",
        ("zh", "numbers_analyzed") => "分析的数字数量",
        ("zh", "risk_level") => "注意等级",
        ("zh", "digit_distribution") => "首位数字分布",
        ("zh", "expected") => "预期",
        ("zh", "deviation") => "偏差",
        ("zh", "statistical_tests") => "统计检验",
        ("zh", "chi_square") => "卡方值",
        ("zh", "p_value") => "p值",
        ("zh", "mean_absolute_deviation") => "平均绝对偏差",
        ("zh", "interpretation") => "解释",
        ("zh", "normal_distribution") => "✅ 正常分布 - 数据看起来自然",
        ("zh", "slight_deviation") => "⚠️  轻微偏差 - 需要监测",
        ("zh", "significant_deviation") => "🚨 显著偏差 - 可能存在异常",
        ("zh", "critical_deviation") => "🔍 需要特别注意 - 模式的强烈证据",
        ("zh", "unsupported_format") => "错误: 不支持的输出格式",
        ("zh", "no_numbers_found") => "错误: 输入中未找到有效数字",
        ("zh", "analysis_error") => "分析错误",

        // हिन्दी (Hindi)
        ("hi", "analysis_results") => "बेनफोर्ड के नियम का विश्लेषण परिणाम",
        ("hi", "dataset") => "डेटासेट",
        ("hi", "numbers_analyzed") => "विश्लेषित संख्याएँ",
        ("hi", "risk_level") => "ध्यान स्तर",
        ("hi", "digit_distribution") => "पहले अंक का वितरण",
        ("hi", "expected") => "अपेक्षित",
        ("hi", "deviation") => "विचलन",
        ("hi", "statistical_tests") => "सांख्यिकीय परीक्षण",
        ("hi", "chi_square") => "काई-स्क्वायर",
        ("hi", "p_value") => "p-मान",
        ("hi", "mean_absolute_deviation") => "औसत निरपेक्ष विचलन",
        ("hi", "interpretation") => "व्याख्या",
        ("hi", "normal_distribution") => "✅ सामान्य वितरण - डेटा प्राकृतिक दिखता है",
        ("hi", "slight_deviation") => "⚠️  हल्का विचलन - निगरानी आवश्यक",
        ("hi", "significant_deviation") => "🚨 महत्वपूर्ण विचलन - संभावित असामान्यता",
        ("hi", "critical_deviation") => "🔍 विशेष ध्यान आवश्यक - पैटर्न का मजबूत प्रमाण",
        ("hi", "unsupported_format") => "त्रुटि: असमर्थित आउटपुट प्रारूप",
        ("hi", "no_numbers_found") => "त्रुटि: इनपुट में कोई वैध संख्या नहीं मिली",
        ("hi", "analysis_error") => "विश्लेषण त्रुटि",

        // العربية (Arabic)
        ("ar", "analysis_results") => "نتائج تحليل قانون بنفورد",
        ("ar", "dataset") => "مجموعة البيانات",
        ("ar", "numbers_analyzed") => "الأرقام المحللة",
        ("ar", "risk_level") => "مستوى الانتباه",
        ("ar", "digit_distribution") => "توزيع الرقم الأول",
        ("ar", "expected") => "متوقع",
        ("ar", "deviation") => "انحراف",
        ("ar", "statistical_tests") => "الاختبارات الإحصائية",
        ("ar", "chi_square") => "كاي تربيع",
        ("ar", "p_value") => "القيمة الاحتمالية",
        ("ar", "mean_absolute_deviation") => "متوسط الانحراف المطلق",
        ("ar", "interpretation") => "التفسير",
        ("ar", "normal_distribution") => "✅ توزيع طبيعي - البيانات تبدو طبيعية",
        ("ar", "slight_deviation") => "⚠️  انحراف طفيف - يستحق المراقبة",
        ("ar", "significant_deviation") => "🚨 انحراف كبير - شذوذ محتمل مكتشف",
        ("ar", "critical_deviation") => "🔍 يحتاج انتباه خاص - دليل قوي على الأنماط",
        ("ar", "unsupported_format") => "خطأ: تنسيق الإخراج غير مدعوم",
        ("ar", "no_numbers_found") => "خطأ: لم يتم العثور على أرقام صحيحة في الإدخال",
        ("ar", "analysis_error") => "خطأ في التحليل",

        // English (Default)
        (_, "analysis_results") => "Benford's Law Analysis Results",
        (_, "dataset") => "Dataset",
        (_, "numbers_analyzed") => "Numbers analyzed",
        (_, "risk_level") => "Attention Level",
        (_, "digit_distribution") => "First Digit Distribution",
        (_, "expected") => "expected",
        (_, "deviation") => "deviation",
        (_, "statistical_tests") => "Statistical Tests",
        (_, "chi_square") => "Chi-square",
        (_, "p_value") => "p-value",
        (_, "mean_absolute_deviation") => "Mean Absolute Deviation",
        (_, "interpretation") => "Interpretation",
        (_, "normal_distribution") => "✅ Normal distribution - data appears natural",
        (_, "slight_deviation") => "⚠️  Slight deviation - worth monitoring",
        (_, "significant_deviation") => "🚨 Significant deviation - potential anomaly detected",
        (_, "critical_deviation") => {
            "🔍 Significant attention needed - strong evidence of patterns"
        }
        (_, "unsupported_format") => "Error: Unsupported output format",
        (_, "no_numbers_found") => "Error: No valid numbers found in input",
        (_, "analysis_error") => "Analysis error",
        (_, _) => "Unknown message", // catch-all pattern
    }
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

    // Parse custom threshold if specified
    let threshold = if let Some(threshold_str) = matches.get_one::<String>("threshold") {
        if threshold_str == "auto" {
            RiskThreshold::Auto
        } else {
            RiskThreshold::from_str(threshold_str)
                .map_err(|e| BenfError::ParseError(format!("無効な閾値: {}", e)))?
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

    // Perform Benford analysis with custom options
    BenfordResult::new_with_threshold(dataset_name, &filtered_numbers, &threshold, min_count)
}
