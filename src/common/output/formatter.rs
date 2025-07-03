use crate::laws::benford::BenfordResult;

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Text,
    Json,
    Csv,
    Yaml,
    Toml,
    Xml,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> crate::Result<Self> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            "csv" => Ok(OutputFormat::Csv),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            "xml" => Ok(OutputFormat::Xml),
            _ => Err(crate::BenfError::InvalidInput(format!("Unsupported format: {}", s))),
        }
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
            crate::RiskLevel::Critical => "⚠️",
            crate::RiskLevel::High => "⚠️",
            _ => "",
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
    
    for i in 0..9 {
        let digit = i + 1;
        let observed = result.digit_distribution[i];
        let expected = result.expected_distribution[i];
        let bar_length = ((observed / 100.0) * 40.0) as usize;
        let bar = "█".repeat(bar_length);
        
        output.push_str(&format!(
            "{}: {:30} {:.1}% (expected: {:.1}%)\n",
            digit, bar, observed, expected
        ));
    }
    
    output
}

fn format_json(result: &BenfordResult) -> String {
    format!(
        r#"{{
  "dataset": "{}",
  "numbers_analyzed": {},
  "risk_level": "{}",
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
        result.digit_distribution[0], result.expected_distribution[0], result.digit_distribution[0] - result.expected_distribution[0],
        result.digit_distribution[1], result.expected_distribution[1], result.digit_distribution[1] - result.expected_distribution[1],
        result.digit_distribution[2], result.expected_distribution[2], result.digit_distribution[2] - result.expected_distribution[2],
        result.digit_distribution[3], result.expected_distribution[3], result.digit_distribution[3] - result.expected_distribution[3],
        result.digit_distribution[4], result.expected_distribution[4], result.digit_distribution[4] - result.expected_distribution[4],
        result.digit_distribution[5], result.expected_distribution[5], result.digit_distribution[5] - result.expected_distribution[5],
        result.digit_distribution[6], result.expected_distribution[6], result.digit_distribution[6] - result.expected_distribution[6],
        result.digit_distribution[7], result.expected_distribution[7], result.digit_distribution[7] - result.expected_distribution[7],
        result.digit_distribution[8], result.expected_distribution[8], result.digit_distribution[8] - result.expected_distribution[8],
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