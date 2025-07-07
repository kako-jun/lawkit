pub mod benford;
pub mod japanese;
pub mod international;
pub mod statistics;
pub mod filtering;

pub use benford::*;
pub use japanese::*;
pub use international::*;
pub use statistics::*;
pub use filtering::*;

#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,        // p > 0.1 - Normal distribution
    Medium,     // 0.05 < p ≤ 0.1 - Moderate attention
    High,       // 0.01 < p ≤ 0.05 - Notable deviation
    Critical,   // p ≤ 0.01 - Significant attention needed
}

impl RiskLevel {
    pub fn from_p_value(p_value: f64) -> Self {
        if p_value <= 0.01 {
            RiskLevel::Critical
        } else if p_value <= 0.05 {
            RiskLevel::High
        } else if p_value <= 0.1 {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        }
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            RiskLevel::Low | RiskLevel::Medium => 0,
            RiskLevel::High => 10,
            RiskLevel::Critical => 11,
        }
    }
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "LOW"),
            RiskLevel::Medium => write!(f, "MEDIUM"),
            RiskLevel::High => write!(f, "HIGH"),
            RiskLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BenfordResult {
    pub dataset_name: String,
    pub numbers_analyzed: usize,
    pub digit_distribution: [f64; 9], // Observed percentages for digits 1-9
    pub expected_distribution: [f64; 9], // Benford's Law expected percentages
    pub chi_square: f64,
    pub p_value: f64,
    pub mean_absolute_deviation: f64,
    pub risk_level: RiskLevel,
    pub verdict: String,
}

impl BenfordResult {
    pub fn new(dataset_name: String, numbers: &[f64]) -> crate::error::Result<Self> {
        Self::new_with_threshold(dataset_name, numbers, &RiskThreshold::Auto, 5)
    }

    pub fn new_with_threshold(
        dataset_name: String, 
        numbers: &[f64], 
        threshold: &RiskThreshold,
        min_count: usize
    ) -> crate::error::Result<Self> {
        if numbers.is_empty() {
            return Err(crate::error::BenfError::NoNumbersFound);
        }
        
        // Check minimum count requirement
        if numbers.len() < min_count {
            return Err(crate::error::BenfError::InsufficientData(numbers.len()));
        }
        
        // Issue warning for small datasets but continue analysis
        if numbers.len() < 30 {
            let num_len = numbers.len();
            eprintln!("Warning: {num_len} numbers analyzed. For reliable Benford's Law analysis, 30+ numbers recommended.");
        }

        let digit_distribution = benford::calculate_digit_distribution(numbers);
        let expected_distribution = benford::BENFORD_EXPECTED_PERCENTAGES;
        let chi_square = statistics::calculate_chi_square(&digit_distribution, &expected_distribution);
        let p_value = statistics::calculate_p_value(chi_square, 8); // 8 degrees of freedom
        let mean_absolute_deviation = statistics::calculate_mad(&digit_distribution, &expected_distribution);
        
        // Use custom threshold if provided, otherwise use default logic
        let risk_level = threshold.evaluate_risk(p_value);
        
        let verdict = match risk_level {
            RiskLevel::Low => "NORMAL_DISTRIBUTION".to_string(),
            RiskLevel::Medium => "SLIGHT_DEVIATION".to_string(),
            RiskLevel::High => "SIGNIFICANT_DEVIATION".to_string(),
            RiskLevel::Critical => "STRONG_EVIDENCE_OF_MANIPULATION".to_string(),
        };

        Ok(BenfordResult {
            dataset_name,
            numbers_analyzed: numbers.len(),
            digit_distribution,
            expected_distribution,
            chi_square,
            p_value,
            mean_absolute_deviation,
            risk_level,
            verdict,
        })
    }
}