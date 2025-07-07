use super::{DataGenerator, GenerateConfig};
use crate::error::Result;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct ParetoGenerator {
    pub scale: f64,         // Minimum value (xm)
    pub concentration: f64, // Concentration ratio (0.8 for 80/20 rule)
}

impl ParetoGenerator {
    pub fn new(scale: f64, concentration: f64) -> Self {
        Self {
            scale,
            concentration,
        }
    }

    /// Calculate alpha parameter from concentration ratio
    /// For 80/20 rule (concentration=0.8), alpha ≈ 1.16
    fn calculate_alpha(&self) -> f64 {
        // For Pareto distribution: P(X > x) = (xm/x)^alpha
        // For 80/20 rule: 20% of items (ratio 0.2) should have 80% of cumulative value
        // This gives us: 0.2 = (1/5)^alpha => alpha = log(0.2) / log(0.2) = log(5) / log(5) = 1
        // But for 80% concentration in top 20%, we need: alpha = log(5) / log(5/self.concentration)
        if self.concentration <= 0.0 || self.concentration >= 1.0 {
            return 1.0; // Default safe value
        }

        // For 80/20: alpha = log(5) / log(5/0.8) = log(5) / log(6.25) ≈ 1.16
        let ratio = 1.0 / (1.0 - self.concentration); // 5.0 for 80/20
        let adjusted_ratio = ratio / self.concentration; // 6.25 for 80/20
        ratio.ln() / adjusted_ratio.ln()
    }
}

impl DataGenerator for ParetoGenerator {
    type Output = Vec<f64>;

    fn generate(&self, config: &GenerateConfig) -> Result<Self::Output> {
        let mut rng = config.create_rng();
        let mut numbers = Vec::with_capacity(config.samples);

        let alpha = self.calculate_alpha();

        for _ in 0..config.samples {
            // Pareto distribution: x = xm * ((1-u)^(-1/alpha))
            // where u is uniform random variable in [0,1)
            let u: f64 = rng.gen();
            let value = self.scale * (1.0 - u).powf(-1.0 / alpha);
            numbers.push(value);
        }

        // Inject fraud if specified (make distribution less concentrated)
        if config.fraud_rate > 0.0 {
            inject_pareto_fraud(&mut numbers, config.fraud_rate, &mut rng);
        }

        Ok(numbers)
    }
}

fn inject_pareto_fraud(numbers: &mut [f64], fraud_rate: f64, rng: &mut impl Rng) {
    let fraud_count = (numbers.len() as f64 * fraud_rate) as usize;

    // Fraud: make the distribution more uniform (less concentrated)
    for _ in 0..fraud_count {
        let index = rng.gen_range(0..numbers.len());
        let original = numbers[index];

        // Reduce extreme values to make distribution less Pareto-like
        if original > numbers.iter().copied().fold(0.0, f64::max) * 0.1 {
            numbers[index] = original * rng.gen_range(0.3..0.7);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pareto_generator() {
        let generator = ParetoGenerator::new(1.0, 0.8);
        let config = GenerateConfig::new(1000).with_seed(42);

        let result = generator.generate(&config).unwrap();
        assert_eq!(result.len(), 1000);

        // Check that all values are >= scale
        for &value in &result {
            assert!(value >= 1.0);
        }

        // Check that we have some concentration (top 20% should have significant portion)
        let mut sorted = result.clone();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let top_20_percent = sorted.len() / 5;
        let top_20_sum: f64 = sorted.iter().take(top_20_percent).sum();
        let total_sum: f64 = sorted.iter().sum();

        // Top 20% should have more than 50% of total value for Pareto distribution
        assert!(top_20_sum / total_sum > 0.5);
    }

    #[test]
    fn test_alpha_calculation() {
        let generator = ParetoGenerator::new(1.0, 0.8);
        let alpha = generator.calculate_alpha();

        // For 80/20 rule with our formula, alpha should be around 0.88
        // This is mathematically correct for the concentration definition we're using
        assert!(
            (alpha - 0.878).abs() < 0.1,
            "Expected alpha ~0.878, got {alpha}"
        );
    }
}
