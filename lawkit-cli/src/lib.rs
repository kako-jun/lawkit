pub mod colors;
pub mod common_options;
pub mod subcommands;

// 明示的なre-exportで曖昧さを回避
pub use lawkit_core::{common, error, laws, VERSION as CORE_VERSION};
pub use subcommands::{benf, compare, normal, pareto, poisson, zipf};

pub const VERSION: &str = "2.0.1";
