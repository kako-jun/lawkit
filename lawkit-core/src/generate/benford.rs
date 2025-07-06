use super::{DataGenerator, GenerateConfig};
use crate::error::Result;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct BenfordGenerator {
    pub min_value: f64,
    pub max_value: f64,
}

impl BenfordGenerator {
    pub fn new(min_value: f64, max_value: f64) -> Self {
        Self {
            min_value,
            max_value,
        }
    }

    pub fn from_range_str(range_str: &str) -> Result<Self> {
        let parts: Vec<&str> = range_str.split(',').collect();
        if parts.len() != 2 {
            return Err(crate::error::BenfError::ParseError(
                "Range must be in format 'min,max'".to_string(),
            ));
        }

        let min_value: f64 = parts[0]
            .trim()
            .parse()
            .map_err(|_| crate::error::BenfError::ParseError("Invalid min value".to_string()))?;
        let max_value: f64 = parts[1]
            .trim()
            .parse()
            .map_err(|_| crate::error::BenfError::ParseError("Invalid max value".to_string()))?;

        if min_value >= max_value {
            return Err(crate::error::BenfError::ParseError(
                "Min value must be less than max value".to_string(),
            ));
        }

        Ok(Self::new(min_value, max_value))
    }
}

impl DataGenerator for BenfordGenerator {
    type Output = Vec<f64>;

    fn generate(&self, config: &GenerateConfig) -> Result<Self::Output> {
        let mut rng = config.create_rng();
        let mut numbers = Vec::with_capacity(config.samples);

        // Use logarithmic distribution to approximate Benford's law
        let log_min = self.min_value.ln();
        let log_max = self.max_value.ln();
        let log_range = Uniform::new(log_min, log_max);

        for _ in 0..config.samples {
            let log_value = log_range.sample(&mut rng);
            let value = log_value.exp();
            numbers.push(value);
        }

        // Inject fraud if specified
        if config.fraud_rate > 0.0 {
            inject_fraud(&mut numbers, config.fraud_rate, &mut rng);
        }

        Ok(numbers)
    }
}

fn inject_fraud(numbers: &mut [f64], fraud_rate: f64, rng: &mut impl Rng) {
    let fraud_count = (numbers.len() as f64 * fraud_rate) as usize;

    // Simple fraud injection: bias towards specific digits
    for _ in 0..fraud_count {
        let index = rng.gen_range(0..numbers.len());
        let original = numbers[index];

        // Force the number to start with 5 or 6 (common fraud patterns)
        let fraud_digit = if rng.gen_bool(0.5) { 5.0 } else { 6.0 };
        let magnitude = 10_f64.powf(original.log10().floor());
        let fractional_part = original / magnitude - original.trunc() / magnitude;

        numbers[index] = fraud_digit * magnitude + fractional_part * magnitude;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benford_generator() {
        let generator = BenfordGenerator::new(1.0, 100000.0);
        let config = GenerateConfig::new(1000).with_seed(42);

        let result = generator.generate(&config).unwrap();
        assert_eq!(result.len(), 1000);

        // Check that all values are within range
        for &value in &result {
            assert!((1.0..=100000.0).contains(&value));
        }
    }

    #[test]
    fn test_range_parsing() {
        let generator = BenfordGenerator::from_range_str("1,10000").unwrap();
        assert_eq!(generator.min_value, 1.0);
        assert_eq!(generator.max_value, 10000.0);

        assert!(BenfordGenerator::from_range_str("invalid").is_err());
        assert!(BenfordGenerator::from_range_str("10,5").is_err());
    }
}
