use clap::{command, Command, ArgMatches};

mod common;
mod laws;
mod subcommands;
mod error;

use crate::error::LawkitError;

const VERSION: &str = "2.0.0";


fn main() {
    let matches = command!()
        .name("lawkit")
        .about("Statistical law analysis toolkit")
        .version(VERSION)
        .subcommand(
            Command::new("benf")
                .about("Benford's law analysis")
                .arg(clap::Arg::new("input")
                    .help("Input data (file path or string)")
                    .index(1))
                .arg(clap::Arg::new("format")
                    .long("format")
                    .value_name("FORMAT")
                    .help("Output format: text, csv, json, yaml, toml, xml")
                    .default_value("text"))
                .arg(clap::Arg::new("quiet")
                    .long("quiet")
                    .short('q')
                    .help("Minimal output (numbers only)")
                    .action(clap::ArgAction::SetTrue))
                .arg(clap::Arg::new("verbose")
                    .long("verbose")
                    .short('v')
                    .help("Detailed statistics")
                    .action(clap::ArgAction::SetTrue))
                .arg(clap::Arg::new("lang")
                    .long("lang")
                    .short('l')
                    .value_name("LANGUAGE")
                    .help("Output language: en, ja, zh, hi, ar")
                    .default_value("auto"))
                .arg(clap::Arg::new("filter")
                    .long("filter")
                    .value_name("RANGE")
                    .help("Filter numbers by range (e.g., >=100, <1000, 50-500)"))
                .arg(clap::Arg::new("threshold")
                    .long("threshold")
                    .value_name("LEVEL")
                    .help("Custom anomaly detection threshold (low, medium, high, critical)")
                    .default_value("auto"))
                .arg(clap::Arg::new("min-count")
                    .long("min-count")
                    .value_name("NUMBER")
                    .help("Minimum number of data points required for analysis")
                    .default_value("5"))
        )
        .subcommand(
            Command::new("pareto")
                .about("Pareto principle (80/20 rule) analysis")
                .arg(clap::Arg::new("input")
                    .help("Input data (file path or string)")
                    .index(1))
                .arg(clap::Arg::new("format")
                    .long("format")
                    .value_name("FORMAT")
                    .help("Output format: text, csv, json, yaml, toml, xml")
                    .default_value("text"))
                .arg(clap::Arg::new("quiet")
                    .long("quiet")
                    .short('q')
                    .help("Minimal output (metrics only)")
                    .action(clap::ArgAction::SetTrue))
                .arg(clap::Arg::new("verbose")
                    .long("verbose")
                    .short('v')
                    .help("Detailed analysis with interpretation")
                    .action(clap::ArgAction::SetTrue))
                .arg(clap::Arg::new("lang")
                    .long("lang")
                    .short('l')
                    .value_name("LANGUAGE")
                    .help("Output language: en, ja, zh, hi, ar")
                    .default_value("auto"))
                .arg(clap::Arg::new("filter")
                    .long("filter")
                    .value_name("RANGE")
                    .help("Filter numbers by range (e.g., >=100, <1000, 50-500)"))
                .arg(clap::Arg::new("min-count")
                    .long("min-count")
                    .value_name("NUMBER")
                    .help("Minimum number of data points required for analysis")
                    .default_value("5"))
        )
        .subcommand(
            Command::new("zipf")
                .about("Zipf's law analysis")
                .arg(clap::Arg::new("input")
                    .help("Input data (file path or string)")
                    .index(1))
                .arg(clap::Arg::new("format")
                    .long("format")
                    .value_name("FORMAT")
                    .help("Output format: text, csv, json, yaml, toml, xml")
                    .default_value("text"))
                .arg(clap::Arg::new("quiet")
                    .long("quiet")
                    .short('q')
                    .help("Minimal output (metrics only)")
                    .action(clap::ArgAction::SetTrue))
                .arg(clap::Arg::new("verbose")
                    .long("verbose")
                    .short('v')
                    .help("Detailed analysis with interpretation")
                    .action(clap::ArgAction::SetTrue))
                .arg(clap::Arg::new("lang")
                    .long("lang")
                    .short('l')
                    .value_name("LANGUAGE")
                    .help("Output language: en, ja, zh, hi, ar")
                    .default_value("auto"))
                .arg(clap::Arg::new("filter")
                    .long("filter")
                    .value_name("RANGE")
                    .help("Filter numbers by range (e.g., >=100, <1000, 50-500)"))
                .arg(clap::Arg::new("min-count")
                    .long("min-count")
                    .value_name("NUMBER")
                    .help("Minimum number of data points required for analysis")
                    .default_value("5"))
                .arg(clap::Arg::new("text")
                    .long("text")
                    .help("Treat input as text for word frequency analysis")
                    .action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            Command::new("list")
                .about("List available statistical laws")
        )
        .get_matches();

    let result = match matches.subcommand() {
        Some(("benf", sub_matches)) => subcommands::benf::run(sub_matches),
        Some(("pareto", sub_matches)) => subcommands::pareto::run(sub_matches),
        Some(("zipf", sub_matches)) => subcommands::zipf::run(sub_matches),
        Some(("list", _)) => list_laws(),
        _ => {
            show_help();
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn list_laws() -> Result<(), LawkitError> {
    println!("Available statistical laws:");
    println!("  benf    - Benford's law analysis");
    println!("  pareto  - Pareto principle (80/20 rule) analysis");
    println!("  zipf    - Zipf's law analysis");
    Ok(())
}

fn show_help() {
    println!("lawkit - Statistical law analysis toolkit");
    println!("Usage: lawkit <SUBCOMMAND>");
    println!("Run 'lawkit --help' for more information.");
}



