#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,      // p > 0.1 - Normal distribution
    Medium,   // 0.05 < p ≤ 0.1 - Moderate attention
    High,     // 0.01 < p ≤ 0.05 - Notable deviation
    Critical, // p ≤ 0.01 - Significant attention needed
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "Low"),
            RiskLevel::Medium => write!(f, "Medium"),
            RiskLevel::High => write!(f, "High"),
            RiskLevel::Critical => write!(f, "Critical"),
        }
    }
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
            RiskLevel::Low => 0,
            RiskLevel::Medium => 0,
            RiskLevel::High => 10,
            RiskLevel::Critical => 11,
        }
    }
}
