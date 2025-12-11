use clap::{Arg, ArgAction, Command};
use lawkit_core::{
    common::{
        filtering::{apply_number_filter, NumberFilter},
        input::{parse_input_auto, parse_text_input},
        memory::{streaming_pareto_analysis, MemoryConfig},
        streaming_io::OptimizedFileReader,
    },
    laws::pareto::{analyze_pareto_distribution, ParetoResult},
};

const VERSION: &str = "2.5.16";

fn main() {
    let matches = Command::new("pareto")
        .about("Pareto Principle (80/20 rule) analysis CLI")
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
            Arg::new("percentiles")
                .long("percentiles")
                .help("Custom percentiles to calculate (comma-separated, e.g., 10,20,30)"),
        )
        .arg(
            Arg::new("gini-coefficient")
                .long("gini-coefficient")
                .help("Show Gini coefficient explicitly")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("business-analysis")
                .long("business-analysis")
                .help("Show business interpretation")
                .action(ArgAction::SetTrue),
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

    if numbers.is_empty() {
        eprintln!("Error: No valid numbers found in input");
        std::process::exit(1);
    }

    let memory_config = MemoryConfig::default();
    let chunk_result = streaming_pareto_analysis(numbers.into_iter(), &memory_config)?;

    let mut incremental_pareto = chunk_result.result;
    let sorted_values = incremental_pareto.get_sorted_values().to_vec();

    let result = analyze_numbers_with_options(matches, "stdin".to_string(), &sorted_values)?;
    output_results(matches, &result);
    std::process::exit(result.risk_level.exit_code());
}

fn analyze_numbers_with_options(
    matches: &clap::ArgMatches,
    dataset_name: String,
    numbers: &[f64],
) -> Result<ParetoResult, Box<dyn std::error::Error>> {
    // Apply number filtering if specified
    let filtered_numbers = if let Some(filter_str) = matches.get_one::<String>("filter") {
        let filter = NumberFilter::parse(filter_str)?;
        apply_number_filter(numbers, &filter)
    } else {
        numbers.to_vec()
    };

    // Parse minimum count
    let min_count: usize = matches
        .get_one::<String>("min-count")
        .unwrap()
        .parse()
        .unwrap_or(5);

    if filtered_numbers.len() < min_count {
        return Err(format!(
            "Insufficient data: {} points (minimum: {})",
            filtered_numbers.len(),
            min_count
        )
        .into());
    }

    // Perform Pareto analysis
    let mut result = analyze_pareto_distribution(&filtered_numbers, &dataset_name)?;

    // Custom percentiles
    if let Some(percentiles_str) = matches.get_one::<String>("percentiles") {
        let percentiles: Vec<f64> = percentiles_str
            .split(',')
            .map(|s| s.trim().parse::<f64>())
            .collect::<Result<Vec<f64>, _>>()?;

        result = result.with_custom_percentiles(&percentiles, &filtered_numbers);
    }

    Ok(result)
}

fn output_results(matches: &clap::ArgMatches, result: &ParetoResult) {
    let format = matches.get_one::<String>("format").unwrap();
    let quiet = matches.get_flag("quiet");
    let verbose = matches.get_flag("verbose");

    match format.as_str() {
        "text" => print_text_output(result, quiet, verbose, matches),
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

fn print_text_output(
    result: &ParetoResult,
    quiet: bool,
    verbose: bool,
    matches: &clap::ArgMatches,
) {
    if quiet {
        println!("pareto_ratio: {:.3}", result.pareto_ratio);
        println!("concentration_index: {:.3}", result.concentration_index);
        println!("top_20_percent_share: {:.1}%", result.top_20_percent_share);
        return;
    }

    println!("Pareto Principle (80/20 Rule) Analysis");
    println!();
    println!("Dataset: {}", result.dataset_name);
    println!("Numbers analyzed: {}", result.numbers_analyzed);
    println!("Risk Level: {:?}", result.risk_level);
    println!();

    // Lorenz curve visualization
    println!("Distribution Summary:");
    println!(
        "  Top 20% owns: {:.1}% of total (ideal: 80%)",
        result.top_20_percent_share
    );
    println!("  Pareto ratio: {:.3}", result.pareto_ratio);
    println!("  Gini coefficient: {:.3}", result.concentration_index);

    if verbose {
        println!();
        println!("Lorenz Curve Points:");
        let points = &result.cumulative_distribution;
        let step = (points.len() / 10).max(1);
        for i in (0..points.len()).step_by(step) {
            let (pop, wealth) = points[i];
            println!(
                "  {:.0}% population → {:.1}% wealth",
                pop * 100.0,
                wealth * 100.0
            );
        }

        if let Some(ref percentiles) = result.custom_percentiles {
            println!();
            println!("Custom Percentiles:");
            for (percentile, share) in percentiles {
                println!("  Top {percentile:.0}%: {share:.1}%");
            }
        }
    }

    // Gini coefficient option
    if matches.get_flag("gini-coefficient") {
        println!();
        println!("Gini coefficient: {:.3}", result.concentration_index);
    }

    // Business analysis option
    if matches.get_flag("business-analysis") {
        println!();
        println!("Business Analysis:");
        println!(
            "  Concentration level: {:.1}%",
            result.concentration_index * 100.0
        );
        if result.top_20_percent_share > 80.0 {
            println!("  Status: High concentration - good focus on high-value items");
        } else if result.top_20_percent_share > 60.0 {
            println!("  Status: Moderate concentration - room for optimization");
        } else {
            println!("  Status: Low concentration - consider focusing efforts");
        }
    }

    // Custom percentiles (show even without verbose if requested)
    if !verbose && result.custom_percentiles.is_some() {
        if let Some(ref percentiles) = result.custom_percentiles {
            println!();
            println!("Custom Percentiles:");
            for (percentile, share) in percentiles {
                println!("  Top {percentile:.0}%: {share:.1}%");
            }
        }
    }
}

fn print_json_output(result: &ParetoResult) {
    let mut output = serde_json::json!({
        "dataset": result.dataset_name,
        "numbers_analyzed": result.numbers_analyzed,
        "risk_level": format!("{:?}", result.risk_level),
        "pareto_ratio": result.pareto_ratio,
        "concentration_index": result.concentration_index,
        "gini_coefficient": result.concentration_index,
        "top_20_percent_share": result.top_20_percent_share
    });

    if let Some(ref percentiles) = result.custom_percentiles {
        output["custom_percentiles"] = serde_json::json!(percentiles);
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
