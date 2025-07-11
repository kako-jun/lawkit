use crate::laws::benford::BenfordResult;
use clap::ArgMatches; // clap::ArgMatches をインポート
use std::io::{self, Write}; // io::Write をインポート

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Text,
    Json,
    Csv,
    Yaml,
    Toml,
    Xml,
}

impl std::str::FromStr for OutputFormat {
    type Err = crate::error::BenfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            "csv" => Ok(OutputFormat::Csv),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            "xml" => Ok(OutputFormat::Xml),
            _ => Err(crate::error::BenfError::InvalidInput(format!(
                "Unsupported format: {s}"
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputConfig {
    pub format: String,
    pub quiet: bool,
    pub verbose: bool,
}

impl OutputConfig {
    pub fn from_matches(matches: &ArgMatches) -> Self {
        OutputConfig {
            format: matches
                .get_one::<String>("format")
                .unwrap_or(&"text".to_string())
                .clone(),
            quiet: *matches.get_one::<bool>("quiet").unwrap_or(&false),
            verbose: *matches.get_one::<bool>("verbose").unwrap_or(&false),
        }
    }
}

pub fn create_output_writer(matches: &ArgMatches) -> crate::error::Result<Box<dyn Write>> {
    // outputオプションが存在する場合のみチェック
    let output_path = matches.try_get_one::<String>("output").ok().flatten();

    if let Some(path) = output_path {
        if path == "-" {
            Ok(Box::new(io::stdout()))
        } else {
            let file = std::fs::File::create(path)?;
            Ok(Box::new(file))
        }
    } else {
        Ok(Box::new(io::stdout()))
    }
}

pub fn format_output(result: &BenfordResult, format: &OutputFormat) -> String {
    match format {
        OutputFormat::Text => format_text(result),
        OutputFormat::Json => format_json(result),
        OutputFormat::Csv => format_csv(result),
        OutputFormat::Yaml => format_yaml(result),
        OutputFormat::Toml => format_toml(result),
        OutputFormat::Xml => format_xml(result),
    }
}

fn format_text(result: &BenfordResult) -> String {
    format!(
        "Benford's Law Analysis Results\n\
        \n\
        Dataset: {}\n\
        Numbers analyzed: {}\n\
        Risk Level: {} {}\n\
        \n\
        First Digit Distribution:\n\
        {}\n\
        Statistical Tests:\n\
        Chi-square: {:.2} (p-value: {:.3})\n\
        Mean Absolute Deviation: {:.1}%\n\
        \n\
        Verdict: {}",
        result.dataset_name,
        result.numbers_analyzed,
        result.risk_level,
        match result.risk_level {
            crate::common::risk::RiskLevel::Critical => "[CRITICAL]",
            crate::common::risk::RiskLevel::High => "[HIGH]",
            crate::common::risk::RiskLevel::Medium => "[MEDIUM]",
            crate::common::risk::RiskLevel::Low => "[LOW]",
        },
        format_distribution_bars(result),
        result.chi_square,
        result.p_value,
        result.mean_absolute_deviation,
        result.verdict
    )
}

fn format_distribution_bars(result: &BenfordResult) -> String {
    let mut output = String::new();
    const BAR_WIDTH: usize = 40;

    for i in 0..9 {
        let digit = i + 1;
        let observed = result.digit_distribution[i];
        let expected = result.expected_distribution[i];
        let bar_length = ((observed / 100.0) * BAR_WIDTH as f64).round() as usize;
        let bar_length = bar_length.min(BAR_WIDTH); // Ensure we don't exceed max width

        // Create bar with filled and background portions
        let filled_bar = "█".repeat(bar_length);
        let background_bar = "░".repeat(BAR_WIDTH - bar_length);
        let full_bar = format!("{filled_bar}{background_bar}");

        output.push_str(&format!(
            "{digit:1}: {full_bar} {observed:>5.1}% (expected: {expected:>5.1}%)\n"
        ));
    }

    output
}

fn format_json(result: &BenfordResult) -> String {
    format!(
        r#"{{
  "dataset": "{}"
  "numbers_analyzed": {},
  "risk_level": "{}"
  "digits": {{
    "1": {{"observed": {:.1}, "expected": {:.1}, "deviation": {:.1}}},
    "2": {{"observed": {:.1}, "expected": {:.1}, "deviation": {:.1}}},
    "3": {{"observed": {:.1}, "expected": {:.1}, "deviation": {:.1}}},
    "4": {{"observed": {:.1}, "expected": {:.1}, "deviation": {:.1}}},
    "5": {{"observed": {:.1}, "expected": {:.1}, "deviation": {:.1}}},
    "6": {{"observed": {:.1}, "expected": {:.1}, "deviation": {:.1}}},
    "7": {{"observed": {:.1}, "expected": {:.1}, "deviation": {:.1}}},
    "8": {{"observed": {:.1}, "expected": {:.1}, "deviation": {:.1}}},
    "9": {{"observed": {:.1}, "expected": {:.1}, "deviation": {:.1}}}
  }},
  "statistics": {{
    "chi_square": {:.2},
    "p_value": {:.3},
    "mad": {:.1}
  }},
  "verdict": "{}"
}}"#,
        result.dataset_name,
        result.numbers_analyzed,
        result.risk_level,
        result.digit_distribution[0],
        result.expected_distribution[0],
        result.digit_distribution[0] - result.expected_distribution[0],
        result.digit_distribution[1],
        result.expected_distribution[1],
        result.digit_distribution[1] - result.expected_distribution[1],
        result.digit_distribution[2],
        result.expected_distribution[2],
        result.digit_distribution[2] - result.expected_distribution[2],
        result.digit_distribution[3],
        result.expected_distribution[3],
        result.digit_distribution[3] - result.expected_distribution[3],
        result.digit_distribution[4],
        result.expected_distribution[4],
        result.digit_distribution[4] - result.expected_distribution[4],
        result.digit_distribution[5],
        result.expected_distribution[5],
        result.digit_distribution[5] - result.expected_distribution[5],
        result.digit_distribution[6],
        result.expected_distribution[6],
        result.digit_distribution[6] - result.expected_distribution[6],
        result.digit_distribution[7],
        result.expected_distribution[7],
        result.digit_distribution[7] - result.expected_distribution[7],
        result.digit_distribution[8],
        result.expected_distribution[8],
        result.digit_distribution[8] - result.expected_distribution[8],
        result.chi_square,
        result.p_value,
        result.mean_absolute_deviation,
        result.verdict
    )
}

fn format_csv(_result: &BenfordResult) -> String {
    "CSV format not yet implemented".to_string()
}

fn format_yaml(_result: &BenfordResult) -> String {
    "YAML format not yet implemented".to_string()
}

fn format_toml(_result: &BenfordResult) -> String {
    "TOML format not yet implemented".to_string()
}

fn format_xml(_result: &BenfordResult) -> String {
    "XML format not yet implemented".to_string()
}
