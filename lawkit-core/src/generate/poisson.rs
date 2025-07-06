use super::{DataGenerator, GenerateConfig};
use crate::error::Result;
use rand::prelude::*;
use rand_distr::{Distribution, Poisson};

#[derive(Debug, Clone)]
pub struct PoissonGenerator {
    pub lambda: f64,
    pub time_series: bool,
}

impl PoissonGenerator {
    pub fn new(lambda: f64, time_series: bool) -> Self {
        Self {
            lambda,
            time_series,
        }
    }
}

impl DataGenerator for PoissonGenerator {
    type Output = Vec<u32>;

    fn generate(&self, config: &GenerateConfig) -> Result<Self::Output> {
        let mut rng = config.create_rng();
        let mut numbers = Vec::with_capacity(config.samples);

        let poisson = Poisson::new(self.lambda).map_err(|e| {
            crate::error::BenfError::ParseError(format!("Invalid lambda parameter: {e}"))
        })?;

        for _ in 0..config.samples {
            let value = poisson.sample(&mut rng) as u32;
            numbers.push(value);
        }

        // Inject fraud if specified (add non-Poisson patterns)
        if config.fraud_rate > 0.0 {
            inject_poisson_fraud(&mut numbers, config.fraud_rate, &mut rng);
        }

        Ok(numbers)
    }
}

fn inject_poisson_fraud(numbers: &mut [u32], fraud_rate: f64, rng: &mut impl Rng) {
    let fraud_count = (numbers.len() as f64 * fraud_rate) as usize;

    // Fraud: add artificially high values or clustering
    for _ in 0..fraud_count {
        let index = rng.gen_range(0..numbers.len());

        if rng.gen_bool(0.5) {
            // Add artificially high values
            numbers[index] = rng.gen_range(50..100);
        } else {
            // Force clustering around specific values
            numbers[index] = if rng.gen_bool(0.3) { 0 } else { 1 };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poisson_generator() {
        let generator = PoissonGenerator::new(2.5, false);
        let config = GenerateConfig::new(1000).with_seed(42);

        let result = generator.generate(&config).unwrap();
        assert_eq!(result.len(), 1000);

        // Check that mean is approximately lambda
        let mean = result.iter().sum::<u32>() as f64 / result.len() as f64;
        assert!((mean - 2.5).abs() < 0.5);

        // Poisson distribution should have variance ≈ mean
        let variance = result
            .iter()
            .map(|&x| (x as f64 - mean).powi(2))
            .sum::<f64>()
            / result.len() as f64;

        assert!((variance - mean).abs() < 1.0);
    }
}
