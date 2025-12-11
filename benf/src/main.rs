use clap::{Arg, ArgAction, Command};
use lawkit_core::{
    common::{
        filtering::{apply_number_filter, NumberFilter, RiskThreshold},
        input::{parse_input_auto, parse_text_input},
        memory::{streaming_benford_analysis, IncrementalBenford, MemoryConfig},
        risk::RiskLevel,
        statistics,
        streaming_io::OptimizedFileReader,
    },
    laws::benford::BenfordResult,
};
use std::str::FromStr;

const VERSION: &str = "2.5.16";

fn main() {
    let matches = Command::new("benf")
        .about("Benford's Law analysis CLI")
        .version(VERSION)
        .after_help("For more statistical laws, see: https://crates.io/crates/lawkit")
        .arg(
            Arg::new("input")
                .help("Input data (file path, URL, or '-' for stdin)")
                .index(1),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .help("Output format: text, csv, json, yaml, toml, xml")
                .default_value("text"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Minimal output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Detailed output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("filter")
                .long("filter")
                .help("Filter numbers by range (e.g., >=100, <1000, 50-500)"),
        )
        .arg(
            Arg::new("min-count")
                .short('c')
                .long("min-count")
                .help("Minimum number of data points required")
                .default_value("10"),
        )
        .arg(
            Arg::new("no-color")
                .long("no-color")
                .help("Disable colored output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("threshold")
                .short('t')
                .long("threshold")
                .help("Anomaly detection threshold: low, medium, high, critical")
                .default_value("auto"),
        )
        .arg(
            Arg::new("confidence")
                .long("confidence")
                .help("Statistical confidence level (0.01-0.99)")
                .default_value("0.95"),
        )
        .arg(
            Arg::new("sample-size")
                .long("sample-size")
                .help("Maximum sample size for large datasets"),
        )
        .arg(
            Arg::new("min-value")
                .long("min-value")
                .help("Minimum value to include in analysis"),
        )
        .get_matches();

    if let Err(e) = run(&matches) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(input) = matches.get_one::<String>("input") {
        if input == "-" {
            run_stdin(matches)
        } else {
            run_file(matches, input)
        }
    } else {
        run_stdin(matches)
    }
}

fn run_file(matches: &clap::ArgMatches, input: &str) -> Result<(), Box<dyn std::error::Error>> {
    match parse_input_auto(input) {
        Ok(numbers) => {
            if numbers.is_empty() {
                eprintln!("Error: No valid numbers found in input");
                std::process::exit(1);
            }

            let result = analyze_numbers_with_options(matches, input.to_string(), &numbers)?;
            output_results(matches, &result);
            std::process::exit(result.risk_level.exit_code());
        }
        Err(e) => {
            eprintln!("Error processing input '{input}': {e}");
            std::process::exit(1);
        }
    }
}

fn run_stdin(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = OptimizedFileReader::from_stdin();

    let numbers: Vec<f64> = reader
        .read_lines_streaming(|line| parse_text_input(&line).map(Some).or(Ok(None)))?
        .into_iter()
        .flatten()
        .collect();

    // Apply minimum value filter
    let filtered_numbers = if let Some(min_value_str) = matches.get_one::<String>("min-value") {
        let min_val: f64 = min_value_str.parse()?;
        numbers.into_iter().filter(|&x| x >= min_val).collect()
    } else {
        numbers
    };

    let memory_config = MemoryConfig::default();
    let chunk_result = streaming_benford_analysis(filtered_numbers.into_iter(), &memory_config)?;

    if chunk_result.total_items == 0 {
        eprintln!("Error: No valid numbers found in input");
        std::process::exit(1);
    }

    let result = convert_incremental_to_result(&chunk_result.result, "stdin".to_string());
    output_results(matches, &result);
    std::process::exit(result.risk_level.exit_code());
}

fn analyze_numbers_with_options(
    matches: &clap::ArgMatches,
    dataset_name: String,
    numbers: &[f64],
) -> Result<BenfordResult, Box<dyn std::error::Error>> {
    // Apply number filtering if specified
    let filtered_numbers = if let Some(filter_str) = matches.get_one::<String>("filter") {
        let filter = NumberFilter::parse(filter_str)?;
        apply_number_filter(numbers, &filter)
    } else {
        numbers.to_vec()
    };

    // Parse custom threshold
    let threshold = if let Some(threshold_str) = matches.get_one::<String>("threshold") {
        if threshold_str == "auto" {
            RiskThreshold::Auto
        } else {
            RiskThreshold::from_str(threshold_str)?
        }
    } else {
        RiskThreshold::Auto
    };

    // Parse minimum count
    let min_count: usize = matches
        .get_one::<String>("min-count")
        .unwrap()
        .parse()
        .unwrap_or(5);

    // Apply sample size limit
    let mut working_numbers = filtered_numbers;
    if let Some(sample_size_str) = matches.get_one::<String>("sample-size") {
        let max_size: usize = sample_size_str.parse()?;
        if working_numbers.len() > max_size {
            let step = working_numbers.len() / max_size;
            working_numbers = working_numbers
                .iter()
                .step_by(step.max(1))
                .cloned()
                .take(max_size)
                .collect();
        }
    }

    // Apply minimum value filter
    if let Some(min_value_str) = matches.get_one::<String>("min-value") {
        let min_val: f64 = min_value_str.parse()?;
        working_numbers.retain(|&x| x >= min_val);
    }

    Ok(BenfordResult::new_with_threshold(
        dataset_name,
        &working_numbers,
        &threshold,
        min_count,
    )?)
}

fn convert_incremental_to_result(
    incremental: &IncrementalBenford,
    dataset_name: String,
) -> BenfordResult {
    let digit_distribution = incremental.get_distribution();
    let expected_distribution = [
        30.103, 17.609, 12.494, 9.691, 7.918, 6.695, 5.799, 5.115, 4.576,
    ];

    let chi_square = statistics::calculate_chi_square(&digit_distribution, &expected_distribution);
    let p_value = statistics::calculate_p_value(chi_square, 8);
    let mad = incremental.calculate_mad();

    let risk_level = if mad > 15.0 || p_value < 0.01 {
        RiskLevel::Critical
    } else if mad > 10.0 || p_value < 0.05 {
        RiskLevel::High
    } else if mad > 5.0 || p_value < 0.10 {
        RiskLevel::Medium
    } else {
        RiskLevel::Low
    };

    let verdict = format!("Risk Level: {risk_level:?}");

    BenfordResult {
        dataset_name,
        numbers_analyzed: incremental.total_count(),
        digit_distribution,
        expected_distribution,
        chi_square,
        p_value,
        mean_absolute_deviation: mad,
        risk_level,
        verdict,
    }
}

fn output_results(matches: &clap::ArgMatches, result: &BenfordResult) {
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

fn print_text_output(result: &BenfordResult, quiet: bool, verbose: bool) {
    if quiet {
        for (i, &observed) in result.digit_distribution.iter().enumerate() {
            println!("{}: {:.1}%", i + 1, observed);
        }
        return;
    }

    println!("Benford's Law Analysis");
    println!();
    println!("Dataset: {}", result.dataset_name);
    println!("Numbers analyzed: {}", result.numbers_analyzed);
    println!("Risk Level: {:?}", result.risk_level);
    println!();
    println!("First Digit Distribution:");

    for i in 0..9 {
        let digit = i + 1;
        let observed = result.digit_distribution[i];
        let expected = result.expected_distribution[i];
        let bar_len = (observed / 2.0).round() as usize;
        let bar: String = "█".repeat(bar_len.min(25));
        println!("{digit}: {bar:25} {observed:>5.1}% (expected: {expected:>5.1}%)");
    }

    if verbose {
        println!();
        println!("Statistical Tests:");
        println!("Chi-square: {:.2}", result.chi_square);
        println!("P-value: {:.6}", result.p_value);
        println!("MAD: {:.2}", result.mean_absolute_deviation);
    }
}

fn print_json_output(result: &BenfordResult) {
    let output = serde_json::json!({
        "dataset": result.dataset_name,
        "numbers_analyzed": result.numbers_analyzed,
        "risk_level": format!("{:?}", result.risk_level),
        "chi_square": result.chi_square,
        "p_value": result.p_value,
        "mad": result.mean_absolute_deviation
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
