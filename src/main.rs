use clap::{Arg, Command};
use benf::{
    core::{international::extract_numbers_international, RiskLevel, BenfordResult},
    VERSION
};
use std::io::{self, Read};

#[tokio::main]
async fn main() {
    let matches = Command::new("benf")
        .version(VERSION)
        .about("A CLI tool for detecting anomalies using Benford's Law with international numeral support")
        .arg(Arg::new("input")
            .help("Input data (file path, URL, or string)")
            .index(1))
        .arg(Arg::new("url")
            .long("url")
            .value_name("URL")
            .help("Fetch data from URL"))
        .arg(Arg::new("format")
            .long("format")
            .value_name("FORMAT")
            .help("Output format: text, csv, json, yaml, toml, xml")
            .default_value("text"))
        .arg(Arg::new("quiet")
            .long("quiet")
            .short('q')
            .help("Minimal output (numbers only)")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("verbose")
            .long("verbose")
            .short('v')
            .help("Detailed statistics")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("lang")
            .long("lang")
            .short('l')
            .value_name("LANGUAGE")
            .help("Output language: en, ja, zh, hi, ar")
            .default_value("auto"))
        .get_matches();

    // Determine input source based on arguments
    let input_text = if let Some(url) = matches.get_one::<String>("url") {
        // Fetch URL content
        match fetch_url_content(url).await {
            Ok(content) => {
                if content.trim().is_empty() {
                    eprintln!("Error: No content from URL: {}", url);
                    std::process::exit(2);
                }
                content
            }
            Err(e) => {
                eprintln!("Error fetching URL '{}': {}", url, e);
                std::process::exit(1);
            }
        }
    } else if let Some(input) = matches.get_one::<String>("input") {
        // Check if it's a file path or string data
        if std::path::Path::new(input).exists() {
            // Read file contents
            match std::fs::read_to_string(input) {
                Ok(content) => {
                    if content.trim().is_empty() {
                        eprintln!("Error: File is empty: {}", input);
                        std::process::exit(2);
                    }
                    content
                }
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", input, e);
                    std::process::exit(1);
                }
            }
        } else {
            // Treat as string data
            input.clone()
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
                buffer
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                std::process::exit(1);
            }
        }
    };

    // Extract numbers from input text with international numeral support
    let numbers = extract_numbers_international(&input_text);
    
    if numbers.is_empty() {
        let language = get_language(&matches);
        let error_msg = localized_text("no_numbers_found", language);
        eprintln!("{}", error_msg);
        std::process::exit(1);
    }

    // Calculate Benford's Law analysis
    let result = match BenfordResult::new("stdin".to_string(), &numbers) {
        Ok(result) => result,
        Err(e) => {
            let language = get_language(&matches);
            let error_msg = localized_text("analysis_error", language);
            eprintln!("{}: {}", error_msg, e);
            std::process::exit(1);
        }
    };

    // Output results based on format
    let format = matches.get_one::<String>("format").unwrap();
    let quiet = matches.get_flag("quiet");
    let verbose = matches.get_flag("verbose");
    let language = get_language(&matches);

    match format.as_str() {
        "text" => print_text_output(&result, quiet, verbose, language),
        "json" => print_json_output(&result),
        _ => {
            let error_msg = localized_text("unsupported_format", language);
            eprintln!("{}: {}", error_msg, format);
            std::process::exit(2);
        }
    }

    // Exit with appropriate code based on risk level
    let exit_code = match result.risk_level {
        RiskLevel::Low | RiskLevel::Medium => 0,
        RiskLevel::High => 10,
        RiskLevel::Critical => 11,
    };
    
    std::process::exit(exit_code);
}

fn get_language(matches: &clap::ArgMatches) -> &str {
    match matches.get_one::<String>("lang").map(|s| s.as_str()) {
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
        },
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
        ("en", "risk_level") => "Risk Level",
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
        ("en", "critical_deviation") => "💀 Critical deviation - strong evidence of manipulation",
        ("en", "unsupported_format") => "Error: Unsupported output format",
        ("en", "no_numbers_found") => "Error: No valid numbers found in input",
        ("en", "analysis_error") => "Analysis error",
        
        // 日本語
        ("ja", "analysis_results") => "ベンフォードの法則解析結果",
        ("ja", "dataset") => "データセット",
        ("ja", "numbers_analyzed") => "解析した数値数",
        ("ja", "risk_level") => "リスクレベル",
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
        ("ja", "critical_deviation") => "💀 致命的偏差 - 操作の強い証拠",
        ("ja", "unsupported_format") => "エラー: サポートされていない出力形式",
        ("ja", "no_numbers_found") => "エラー: 入力に有効な数値が見つかりません",
        ("ja", "analysis_error") => "解析エラー",
        
        // 中国語（简体）
        ("zh", "analysis_results") => "本福德定律分析结果",
        ("zh", "dataset") => "数据集",
        ("zh", "numbers_analyzed") => "分析的数字数量",
        ("zh", "risk_level") => "风险等级",
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
        ("zh", "critical_deviation") => "💀 严重偏差 - 有操作的强烈证据",
        ("zh", "unsupported_format") => "错误: 不支持的输出格式",
        ("zh", "no_numbers_found") => "错误: 输入中未找到有效数字",
        ("zh", "analysis_error") => "分析错误",
        
        // हिन्दी (Hindi)
        ("hi", "analysis_results") => "बेनफोर्ड के नियम का विश्लेषण परिणाम",
        ("hi", "dataset") => "डेटासेट",
        ("hi", "numbers_analyzed") => "विश्लेषित संख्याएँ",
        ("hi", "risk_level") => "जोखिम स्तर",
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
        ("hi", "critical_deviation") => "💀 गंभीर विचलन - हेराफेरी का मजबूत प्रमाण",
        ("hi", "unsupported_format") => "त्रुटि: असमर्थित आउटपुट प्रारूप",
        ("hi", "no_numbers_found") => "त्रुटि: इनपुट में कोई वैध संख्या नहीं मिली",
        ("hi", "analysis_error") => "विश्लेषण त्रुटि",
        
        // العربية (Arabic)
        ("ar", "analysis_results") => "نتائج تحليل قانون بنفورد",
        ("ar", "dataset") => "مجموعة البيانات",
        ("ar", "numbers_analyzed") => "الأرقام المحللة",
        ("ar", "risk_level") => "مستوى المخاطر",
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
        ("ar", "critical_deviation") => "💀 انحراف حرج - دليل قوي على التلاعب",
        ("ar", "unsupported_format") => "خطأ: تنسيق الإخراج غير مدعوم",
        ("ar", "no_numbers_found") => "خطأ: لم يتم العثور على أرقام صحيحة في الإدخال",
        ("ar", "analysis_error") => "خطأ في التحليل",
        
        // English (Default)
        (_, "analysis_results") => "Benford's Law Analysis Results",
        (_, "dataset") => "Dataset",
        (_, "numbers_analyzed") => "Numbers analyzed",
        (_, "risk_level") => "Risk Level",
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
        (_, "critical_deviation") => "💀 Critical deviation - strong evidence of manipulation",
        (_, "unsupported_format") => "Error: Unsupported output format",
        (_, "no_numbers_found") => "Error: No valid numbers found in input",
        (_, "analysis_error") => "Analysis error",
        (_, _) => "Unknown message", // catch-all pattern
    }
}

fn print_text_output(result: &BenfordResult, quiet: bool, verbose: bool, lang: &str) {
    if quiet {
        // Just print the numbers
        for (i, &observed) in result.digit_distribution.iter().enumerate() {
            println!("{}: {:.1}%", i + 1, observed * 100.0);
        }
        return;
    }

    println!("{}", localized_text("analysis_results", lang));
    println!();
    println!("{}: {}", localized_text("dataset", lang), result.dataset_name);
    println!("{}: {}", localized_text("numbers_analyzed", lang), result.numbers_analyzed);
    println!("{}: {:?} {}", localized_text("risk_level", lang), result.risk_level, get_risk_emoji(&result.risk_level));
    println!();
    
    println!("{}:", localized_text("digit_distribution", lang));
    for (i, &observed) in result.digit_distribution.iter().enumerate() {
        let digit = i + 1;
        let expected = result.expected_distribution[i];
        let deviation = observed - expected;
        let bar = generate_bar(observed / 100.0, 0.3);
        
        println!("{}: {} {:.1}% ({}: {:.1}%, {}: {:+.1}%)", 
                 digit, bar, observed, 
                 localized_text("expected", lang), expected,
                 localized_text("deviation", lang), deviation);
    }
    
    println!();
    println!("{}:", localized_text("statistical_tests", lang));
    println!("{}: {:.2} ({}: {:.6})", 
             localized_text("chi_square", lang), result.chi_square,
             localized_text("p_value", lang), result.p_value);
    
    if verbose {
        println!("{}: {:.2}%", localized_text("mean_absolute_deviation", lang), result.mean_absolute_deviation);
        println!();
        println!("{}:", localized_text("interpretation", lang));
        match result.risk_level {
            RiskLevel::Low => println!("{}", localized_text("normal_distribution", lang)),
            RiskLevel::Medium => println!("{}", localized_text("slight_deviation", lang)),
            RiskLevel::High => println!("{}", localized_text("significant_deviation", lang)),
            RiskLevel::Critical => println!("{}", localized_text("critical_deviation", lang)),
        }
    }
}

fn print_json_output(result: &BenfordResult) {
    use serde_json::json;
    
    let output = json!({
        "dataset": result.dataset_name,
        "numbers_analyzed": result.numbers_analyzed,
        "risk_level": format!("{:?}", result.risk_level),
        "digits": {
            "1": {"observed": result.digit_distribution[0], "expected": result.expected_distribution[0], "deviation": result.digit_distribution[0] - result.expected_distribution[0]},
            "2": {"observed": result.digit_distribution[1], "expected": result.expected_distribution[1], "deviation": result.digit_distribution[1] - result.expected_distribution[1]},
            "3": {"observed": result.digit_distribution[2], "expected": result.expected_distribution[2], "deviation": result.digit_distribution[2] - result.expected_distribution[2]},
            "4": {"observed": result.digit_distribution[3], "expected": result.expected_distribution[3], "deviation": result.digit_distribution[3] - result.expected_distribution[3]},
            "5": {"observed": result.digit_distribution[4], "expected": result.expected_distribution[4], "deviation": result.digit_distribution[4] - result.expected_distribution[4]},
            "6": {"observed": result.digit_distribution[5], "expected": result.expected_distribution[5], "deviation": result.digit_distribution[5] - result.expected_distribution[5]},
            "7": {"observed": result.digit_distribution[6], "expected": result.expected_distribution[6], "deviation": result.digit_distribution[6] - result.expected_distribution[6]},
            "8": {"observed": result.digit_distribution[7], "expected": result.expected_distribution[7], "deviation": result.digit_distribution[7] - result.expected_distribution[7]},
            "9": {"observed": result.digit_distribution[8], "expected": result.expected_distribution[8], "deviation": result.digit_distribution[8] - result.expected_distribution[8]}
        },
        "statistics": {
            "chi_square": result.chi_square,
            "p_value": result.p_value,
            "mad": result.mean_absolute_deviation
        }
    });
    
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
}

fn get_risk_emoji(risk: &RiskLevel) -> &'static str {
    match risk {
        RiskLevel::Low => "✅",
        RiskLevel::Medium => "⚠️",
        RiskLevel::High => "🚨",
        RiskLevel::Critical => "💀",
    }
}

fn generate_bar(value: f64, max_value: f64) -> String {
    let bar_length = 20;
    let filled = ((value / max_value) * bar_length as f64) as usize;
    let filled = filled.min(bar_length);
    
    let mut bar = String::new();
    for _ in 0..filled {
        bar.push('█');
    }
    for _ in filled..bar_length {
        bar.push('░');
    }
    bar
}

async fn fetch_url_content(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    
    if response.status().is_success() {
        let text = response.text().await?;
        Ok(text)
    } else {
        Err(reqwest::Error::from(response.error_for_status().unwrap_err()))
    }
}