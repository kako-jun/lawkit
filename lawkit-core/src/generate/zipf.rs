use super::{DataGenerator, GenerateConfig};
use crate::error::Result;
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct ZipfGenerator {
    pub exponent: f64,
    pub vocabulary_size: usize,
}

impl ZipfGenerator {
    pub fn new(exponent: f64, vocabulary_size: usize) -> Self {
        Self {
            exponent,
            vocabulary_size,
        }
    }
}

impl DataGenerator for ZipfGenerator {
    type Output = Vec<usize>;

    fn generate(&self, config: &GenerateConfig) -> Result<Self::Output> {
        let mut rng = config.create_rng();
        let mut numbers = Vec::with_capacity(config.samples);

        // Pre-calculate probabilities for each rank
        let mut probabilities = Vec::with_capacity(self.vocabulary_size);
        let mut total_weight = 0.0;

        for rank in 1..=self.vocabulary_size {
            let weight = 1.0 / (rank as f64).powf(self.exponent);
            probabilities.push(weight);
            total_weight += weight;
        }

        // Normalize probabilities
        for prob in &mut probabilities {
            *prob /= total_weight;
        }

        // Generate samples using inverse transform sampling
        for _ in 0..config.samples {
            let u: f64 = rng.gen();
            let mut cumulative = 0.0;

            for (rank, &prob) in probabilities.iter().enumerate() {
                cumulative += prob;
                if u <= cumulative {
                    numbers.push(rank + 1); // ranks are 1-indexed
                    break;
                }
            }
        }

        // Inject fraud if specified (flatten the distribution)
        if config.fraud_rate > 0.0 {
            inject_zipf_fraud(
                &mut numbers,
                config.fraud_rate,
                self.vocabulary_size,
                &mut rng,
            );
        }

        Ok(numbers)
    }
}

fn inject_zipf_fraud(
    numbers: &mut [usize],
    fraud_rate: f64,
    vocab_size: usize,
    rng: &mut impl Rng,
) {
    let fraud_count = (numbers.len() as f64 * fraud_rate) as usize;

    // Fraud: inject more uniform distribution (less Zipf-like)
    for _ in 0..fraud_count {
        let index = rng.gen_range(0..numbers.len());
        // Replace with a more uniformly distributed rank
        numbers[index] = rng.gen_range(1..=vocab_size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_zipf_generator() {
        let generator = ZipfGenerator::new(1.0, 1000);
        let config = GenerateConfig::new(10000).with_seed(42);

        let result = generator.generate(&config).unwrap();
        assert_eq!(result.len(), 10000);

        // Count frequencies
        let mut frequencies = HashMap::new();
        for &rank in &result {
            *frequencies.entry(rank).or_insert(0) += 1;
        }

        // Check that rank 1 appears most frequently
        let rank1_freq = frequencies.get(&1).unwrap_or(&0);
        let rank2_freq = frequencies.get(&2).unwrap_or(&0);

        assert!(rank1_freq > rank2_freq);

        // All ranks should be within vocabulary size
        for &rank in &result {
            assert!((1..=1000).contains(&rank));
        }
    }
}
