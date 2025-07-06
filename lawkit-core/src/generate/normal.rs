use super::{DataGenerator, GenerateConfig};
use crate::error::Result;
use rand::prelude::*;
use rand_distr::{Distribution, Normal};

#[derive(Debug, Clone)]
pub struct NormalGenerator {
    pub mean: f64,
    pub stddev: f64,
}

impl NormalGenerator {
    pub fn new(mean: f64, stddev: f64) -> Self {
        Self { mean, stddev }
    }
}

impl DataGenerator for NormalGenerator {
    type Output = Vec<f64>;

    fn generate(&self, config: &GenerateConfig) -> Result<Self::Output> {
        let mut rng = config.create_rng();
        let mut numbers = Vec::with_capacity(config.samples);

        let normal = Normal::new(self.mean, self.stddev).map_err(|e| {
            crate::error::BenfError::ParseError(format!("Invalid normal parameters: {e}"))
        })?;

        for _ in 0..config.samples {
            let value = normal.sample(&mut rng);
            numbers.push(value);
        }

        // Inject fraud if specified (add non-normal outliers)
        if config.fraud_rate > 0.0 {
            inject_normal_fraud(&mut numbers, config.fraud_rate, &mut rng);
        }

        Ok(numbers)
    }
}

fn inject_normal_fraud(numbers: &mut [f64], fraud_rate: f64, rng: &mut impl Rng) {
    let fraud_count = (numbers.len() as f64 * fraud_rate) as usize;
    let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
    let stddev =
        (numbers.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / numbers.len() as f64).sqrt();

    // Add outliers beyond 3 standard deviations
    for _ in 0..fraud_count {
        let index = rng.gen_range(0..numbers.len());
        let outlier_multiplier = rng.gen_range(3.5..6.0);
        let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        numbers[index] = mean + sign * outlier_multiplier * stddev;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_generator() {
        let generator = NormalGenerator::new(100.0, 15.0);
        let config = GenerateConfig::new(1000).with_seed(42);

        let result = generator.generate(&config).unwrap();
        assert_eq!(result.len(), 1000);

        // Check mean and standard deviation are approximately correct
        let mean = result.iter().sum::<f64>() / result.len() as f64;
        let variance = result.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / result.len() as f64;
        let stddev = variance.sqrt();

        assert!((mean - 100.0).abs() < 5.0); // Within 5 units of target mean
        assert!((stddev - 15.0).abs() < 3.0); // Within 3 units of target stddev
    }
}
