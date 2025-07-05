pub mod common;
pub mod laws;
pub mod error;

pub use common::*;
pub use laws::*;
pub use error::*;

// Re-export commonly used types
pub use common::risk::RiskLevel;
pub use common::filtering::RiskThreshold;
pub use laws::benford::BenfordResult;

pub const VERSION: &str = "2.0.1";