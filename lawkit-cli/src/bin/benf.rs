// 後方互換性のための個別benfコマンド
// 既存のmain.rsロジックをそのまま使用

use clap::{Arg, Command};
use lawkit::{
    common::{
        filtering::{apply_number_filter, NumberFilter, RiskThreshold},
        input::{parse_input_auto, parse_text_input},
    },
    error::{BenfError, Result},
    laws::benford::BenfordResult,
};
use std::io::{self, Read};
use std::str::FromStr;

const VERSION: &str = "2.0.0";

fn main() {
    let matches = Command::new("benf")
        .version(VERSION)
        .about("A CLI tool for detecting anomalies using Benford's Law with international numeral support")
        .arg(Arg::new("input")
            .help("Input data (file path or string)")
            .index(1))
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
        .arg(Arg::new("filter")
            .long("filter")
            .value_name("RANGE")
            .help("Filter numbers by range (e.g., >=100, <1000, 50-500)"))
        .arg(Arg::new("threshold")
            .long("threshold")
            .value_name("LEVEL")
            .help("Custom anomaly detection threshold (low, medium, high, critical)")
            .default_value("auto"))
        .arg(Arg::new("min-count")
            .long("min-count")
            .value_name("NUMBER")
            .help("Minimum number of data points required for analysis")
            .default_value("5"))
        .get_matches();

    // 既存の処理ロジック
    if let Some(input) = matches.get_one::<String>("input") {
        match parse_input_auto(input) {
            Ok(numbers) => {
                if numbers.is_empty() {
                    eprintln!("Error: No valid numbers found in input");
                    std::process::exit(1);
                }

                let result =
                    match analyze_numbers_with_options(&matches, input.to_string(), &numbers) {
                        Ok(result) => result,
                        Err(e) => {
                            eprintln!("Analysis error: {}", e);
                            std::process::exit(1);
                        }
                    };

                output_results(&matches, &result);
                std::process::exit(result.risk_level.exit_code());
            }
            Err(e) => {
                eprintln!("Error processing input '{}': {}", input, e);
                std::process::exit(1);
            }
        }
    } else {
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
                        eprintln!("Analysis error: {}", e);
                        std::process::exit(1);
                    }
                };

                let result =
                    match analyze_numbers_with_options(&matches, "stdin".to_string(), &numbers) {
                        Ok(result) => result,
                        Err(e) => {
                            eprintln!("Analysis error: {}", e);
                            std::process::exit(1);
                        }
                    };

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

fn output_results(matches: &clap::ArgMatches, result: &BenfordResult) {
    // 簡易版の出力（main.rsから移植必要）
    let format = matches.get_one::<String>("format").unwrap();

    match format.as_str() {
        "json" => {
            println!(
                "{{\"risk_level\": \"{:?}\", \"numbers_analyzed\": {}}}",
                result.risk_level, result.numbers_analyzed
            );
        }
        _ => {
            println!("Benford's Law Analysis Results");
            println!("Numbers analyzed: {}", result.numbers_analyzed);
            println!("Risk level: {:?}", result.risk_level);
        }
    }
}

fn analyze_numbers_with_options(
    matches: &clap::ArgMatches,
    dataset_name: String,
    numbers: &[f64],
) -> Result<BenfordResult> {
    let filtered_numbers = if let Some(filter_str) = matches.get_one::<String>("filter") {
        let filter = NumberFilter::parse(filter_str)
            .map_err(|e| BenfError::ParseError(format!("無効なフィルタ: {}", e)))?;
        apply_number_filter(numbers, &filter)
    } else {
        numbers.to_vec()
    };

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

    let min_count = if let Some(min_count_str) = matches.get_one::<String>("min-count") {
        min_count_str
            .parse::<usize>()
            .map_err(|_| BenfError::ParseError("無効な最小数値数".to_string()))?
    } else {
        5
    };

    BenfordResult::new_with_threshold(dataset_name, &filtered_numbers, &threshold, min_count)
}
