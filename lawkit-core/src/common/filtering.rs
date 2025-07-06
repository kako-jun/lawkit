use std::str::FromStr;

/// Represents different types of number filters
#[derive(Debug, Clone, PartialEq)]
pub enum NumberFilter {
    /// Greater than or equal to a value (>=N)
    GreaterThanOrEqual(f64),
    /// Less than a value (<N)
    LessThan(f64),
    /// Greater than a value (>N)
    GreaterThan(f64),
    /// Less than or equal to a value (<=N)
    LessThanOrEqual(f64),
    /// Equal to a value (=N or N)
    Equal(f64),
    /// Range between two values (N-M)
    Range(f64, f64),
    /// No filter (pass all numbers)
    None,
}

impl NumberFilter {
    /// Parse a filter string into a NumberFilter
    pub fn parse(filter_str: &str) -> Result<Self, String> {
        let filter_str = filter_str.trim();

        if filter_str.is_empty() {
            return Ok(NumberFilter::None);
        }

        // Check for range (N-M)
        if let Some(dash_pos) = filter_str.find('-') {
            // Make sure it's not a negative number
            if dash_pos > 0 {
                let start_str = &filter_str[..dash_pos];
                let end_str = &filter_str[dash_pos + 1..];

                let start = start_str
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid start number in range: {start_str}"))?;
                let end = end_str
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid end number in range: {end_str}"))?;

                if start >= end {
                    return Err("Range start must be less than end".to_string());
                }

                return Ok(NumberFilter::Range(start, end));
            }
        }

        // Check for comparison operators
        if let Some(value_str) = filter_str.strip_prefix(">=") {
            let value = value_str
                .parse::<f64>()
                .map_err(|_| format!("Invalid number after '>=': {value_str}"))?;
            return Ok(NumberFilter::GreaterThanOrEqual(value));
        }

        if let Some(value_str) = filter_str.strip_prefix("<=") {
            let value = value_str
                .parse::<f64>()
                .map_err(|_| format!("Invalid number after '<=': {value_str}"))?;
            return Ok(NumberFilter::LessThanOrEqual(value));
        }

        if let Some(value_str) = filter_str.strip_prefix('>') {
            let value = value_str
                .parse::<f64>()
                .map_err(|_| format!("Invalid number after '>': {value_str}"))?;
            return Ok(NumberFilter::GreaterThan(value));
        }

        if let Some(value_str) = filter_str.strip_prefix('<') {
            let value = value_str
                .parse::<f64>()
                .map_err(|_| format!("Invalid number after '<': {value_str}"))?;
            return Ok(NumberFilter::LessThan(value));
        }

        if let Some(value_str) = filter_str.strip_prefix('=') {
            let value = value_str
                .parse::<f64>()
                .map_err(|_| format!("Invalid number after '=': {value_str}"))?;
            return Ok(NumberFilter::Equal(value));
        }

        // Try to parse as a direct number (equal)
        match filter_str.parse::<f64>() {
            Ok(value) => Ok(NumberFilter::Equal(value)),
            Err(_) => Err(format!(
                "Invalid filter format: {filter_str}. Use formats like: >=100, <1000, 50-500"
            )),
        }
    }

    /// Check if a number passes this filter
    pub fn matches(&self, number: f64) -> bool {
        match self {
            NumberFilter::GreaterThanOrEqual(threshold) => number >= *threshold,
            NumberFilter::LessThan(threshold) => number < *threshold,
            NumberFilter::GreaterThan(threshold) => number > *threshold,
            NumberFilter::LessThanOrEqual(threshold) => number <= *threshold,
            NumberFilter::Equal(target) => (number - target).abs() < f64::EPSILON,
            NumberFilter::Range(start, end) => number >= *start && number <= *end,
            NumberFilter::None => true,
        }
    }

    /// Get a human-readable description of this filter
    pub fn description(&self) -> String {
        match self {
            NumberFilter::GreaterThanOrEqual(n) => format!("≥ {n}"),
            NumberFilter::LessThan(n) => format!("< {n}"),
            NumberFilter::GreaterThan(n) => format!("> {n}"),
            NumberFilter::LessThanOrEqual(n) => format!("≤ {n}"),
            NumberFilter::Equal(n) => format!("= {n}"),
            NumberFilter::Range(start, end) => format!("{start} - {end}"),
            NumberFilter::None => "All numbers".to_string(),
        }
    }
}

/// Apply a number filter to a vector of numbers
pub fn apply_number_filter(numbers: &[f64], filter: &NumberFilter) -> Vec<f64> {
    match filter {
        NumberFilter::None => numbers.to_vec(),
        _ => numbers
            .iter()
            .filter(|&&num| filter.matches(num))
            .copied()
            .collect(),
    }
}

/// Custom risk threshold levels
#[derive(Debug, Clone, PartialEq)]
pub enum RiskThreshold {
    /// Automatic thresholds based on statistical analysis
    Auto,
    /// Low threshold (p > 0.2)
    Low,
    /// Medium threshold (p > 0.1)
    Medium,
    /// High threshold (p > 0.05)
    High,
    /// Critical threshold (p > 0.01)
    Critical,
    /// Custom p-value threshold
    Custom(f64),
}

impl FromStr for RiskThreshold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(RiskThreshold::Auto),
            "low" => Ok(RiskThreshold::Low),
            "medium" => Ok(RiskThreshold::Medium),
            "high" => Ok(RiskThreshold::High),
            "critical" => Ok(RiskThreshold::Critical),
            _ => {
                // Try to parse as custom p-value
                match s.parse::<f64>() {
                    Ok(p_value) if (0.0..=1.0).contains(&p_value) => {
                        Ok(RiskThreshold::Custom(p_value))
                    },
                    Ok(_) => Err("Custom p-value must be between 0.0 and 1.0".to_string()),
                    Err(_) => Err(format!("Invalid threshold: {s}. Use: auto, low, medium, high, critical, or a p-value (0.0-1.0)")),
                }
            }
        }
    }
}

impl RiskThreshold {
    /// Get the p-value threshold for this risk level
    pub fn p_value_threshold(&self) -> Option<f64> {
        match self {
            RiskThreshold::Auto => None,
            RiskThreshold::Low => Some(0.2),
            RiskThreshold::Medium => Some(0.1),
            RiskThreshold::High => Some(0.05),
            RiskThreshold::Critical => Some(0.01),
            RiskThreshold::Custom(p) => Some(*p),
        }
    }

    /// Determine risk level based on p-value and this threshold
    pub fn evaluate_risk(&self, p_value: f64) -> crate::common::risk::RiskLevel {
        if let Some(threshold) = self.p_value_threshold() {
            // Custom threshold evaluation
            if p_value <= threshold {
                crate::common::risk::RiskLevel::Critical
            } else {
                crate::common::risk::RiskLevel::Low
            }
        } else {
            // Auto mode: use default Benford analysis
            if p_value <= 0.01 {
                crate::common::risk::RiskLevel::Critical
            } else if p_value <= 0.05 {
                crate::common::risk::RiskLevel::High
            } else if p_value <= 0.1 {
                crate::common::risk::RiskLevel::Medium
            } else {
                crate::common::risk::RiskLevel::Low
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_filter_parsing() {
        // Test various filter formats
        assert_eq!(
            NumberFilter::parse(">=100").unwrap(),
            NumberFilter::GreaterThanOrEqual(100.0)
        );
        assert_eq!(
            NumberFilter::parse("<=50").unwrap(),
            NumberFilter::LessThanOrEqual(50.0)
        );
        assert_eq!(
            NumberFilter::parse(">0").unwrap(),
            NumberFilter::GreaterThan(0.0)
        );
        assert_eq!(
            NumberFilter::parse("<1000").unwrap(),
            NumberFilter::LessThan(1000.0)
        );
        assert_eq!(
            NumberFilter::parse("=42").unwrap(),
            NumberFilter::Equal(42.0)
        );
        assert_eq!(
            NumberFilter::parse("100-500").unwrap(),
            NumberFilter::Range(100.0, 500.0)
        );
        assert_eq!(
            NumberFilter::parse("123").unwrap(),
            NumberFilter::Equal(123.0)
        );
        assert_eq!(NumberFilter::parse("").unwrap(), NumberFilter::None);

        // Test error cases
        assert!(NumberFilter::parse("invalid").is_err());
        assert!(NumberFilter::parse(">=abc").is_err());
        assert!(NumberFilter::parse("500-100").is_err()); // Invalid range
    }

    #[test]
    fn test_number_filter_matching() {
        let ge100 = NumberFilter::GreaterThanOrEqual(100.0);
        assert!(ge100.matches(100.0));
        assert!(ge100.matches(150.0));
        assert!(!ge100.matches(99.0));

        let range = NumberFilter::Range(10.0, 20.0);
        assert!(range.matches(15.0));
        assert!(range.matches(10.0));
        assert!(range.matches(20.0));
        assert!(!range.matches(5.0));
        assert!(!range.matches(25.0));

        let none = NumberFilter::None;
        assert!(none.matches(0.0));
        assert!(none.matches(-100.0));
        assert!(none.matches(1000000.0));
    }

    #[test]
    fn test_apply_number_filter() {
        let numbers = vec![5.0, 15.0, 25.0, 35.0, 45.0];

        let ge20 = NumberFilter::GreaterThanOrEqual(20.0);
        let filtered = apply_number_filter(&numbers, &ge20);
        assert_eq!(filtered, vec![25.0, 35.0, 45.0]);

        let range = NumberFilter::Range(10.0, 30.0);
        let filtered = apply_number_filter(&numbers, &range);
        assert_eq!(filtered, vec![15.0, 25.0]);

        let none = NumberFilter::None;
        let filtered = apply_number_filter(&numbers, &none);
        assert_eq!(filtered, numbers);
    }

    #[test]
    fn test_risk_threshold_parsing() {
        assert_eq!(
            "auto".parse::<RiskThreshold>().unwrap(),
            RiskThreshold::Auto
        );
        assert_eq!("low".parse::<RiskThreshold>().unwrap(), RiskThreshold::Low);
        assert_eq!(
            "medium".parse::<RiskThreshold>().unwrap(),
            RiskThreshold::Medium
        );
        assert_eq!(
            "high".parse::<RiskThreshold>().unwrap(),
            RiskThreshold::High
        );
        assert_eq!(
            "critical".parse::<RiskThreshold>().unwrap(),
            RiskThreshold::Critical
        );
        assert_eq!(
            "0.05".parse::<RiskThreshold>().unwrap(),
            RiskThreshold::Custom(0.05)
        );

        assert!("invalid".parse::<RiskThreshold>().is_err());
        assert!("2.0".parse::<RiskThreshold>().is_err()); // Out of range
    }

    #[test]
    fn test_risk_threshold_evaluation() {
        let auto = RiskThreshold::Auto;
        assert_eq!(
            auto.evaluate_risk(0.005),
            crate::common::risk::RiskLevel::Critical
        );
        assert_eq!(
            auto.evaluate_risk(0.03),
            crate::common::risk::RiskLevel::High
        );
        assert_eq!(
            auto.evaluate_risk(0.07),
            crate::common::risk::RiskLevel::Medium
        );
        assert_eq!(
            auto.evaluate_risk(0.15),
            crate::common::risk::RiskLevel::Low
        );

        let custom = RiskThreshold::Custom(0.02);
        assert_eq!(
            custom.evaluate_risk(0.01),
            crate::common::risk::RiskLevel::Critical
        );
        assert_eq!(
            custom.evaluate_risk(0.03),
            crate::common::risk::RiskLevel::Low
        );
    }
}
