pub mod benford;
pub mod pareto;
pub mod zipf;
pub mod normal;
pub mod poisson;

pub use benford::*;
pub use pareto::*;
pub use zipf::*;
pub use normal::*;
pub use poisson::*;

use crate::error::Result;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Debug, Clone)]
pub struct GenerateConfig {
    pub samples: usize,
    pub seed: Option<u64>,
    pub output_format: String,
    pub fraud_rate: f64,
}

impl GenerateConfig {
    pub fn new(samples: usize) -> Self {
        Self {
            samples,
            seed: None,
            output_format: "text".to_string(),
            fraud_rate: 0.0,
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn with_fraud_rate(mut self, rate: f64) -> Self {
        self.fraud_rate = rate.clamp(0.0, 1.0);
        self
    }

    pub fn create_rng(&self) -> StdRng {
        match self.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        }
    }
}

pub trait DataGenerator {
    type Output;
    
    fn generate(&self, config: &GenerateConfig) -> Result<Self::Output>;
}