// Allow common clippy warnings in tests
#![cfg_attr(test, allow(clippy::uninlined_format_args))]
#![cfg_attr(test, allow(clippy::useless_vec))]
#![cfg_attr(test, allow(clippy::single_match))]
#![cfg_attr(test, allow(clippy::len_zero))]
#![cfg_attr(test, allow(clippy::redundant_closure))]
#![cfg_attr(test, allow(clippy::needless_borrow))]

pub mod colors;
pub mod common_options;
pub mod subcommands;

// 明示的なre-exportで曖昧さを回避
pub use lawkit_core::{common, error, laws};
pub use subcommands::{analyze, benf, diagnose, normal, pareto, poisson, validate, zipf};

pub const VERSION: &str = "2.0.1";
