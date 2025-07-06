pub mod common;
pub mod error;
pub mod generate;
pub mod laws;

pub use common::*;
pub use error::*;
pub use laws::*;

// Re-export commonly used types
pub use common::filtering::RiskThreshold;
pub use common::risk::RiskLevel;
pub use laws::benford::BenfordResult;

pub const VERSION: &str = "2.0.1";
