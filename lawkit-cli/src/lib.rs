pub mod colors;
pub mod common_options;
pub mod subcommands;

// 明示的なre-exportで曖昧さを回避
pub use lawkit_core::{common, error, laws, VERSION as CORE_VERSION};
pub use subcommands::{analyze, benf, diagnose, normal, pareto, poisson, validate, zipf};

pub const VERSION: &str = "2.0.1";
