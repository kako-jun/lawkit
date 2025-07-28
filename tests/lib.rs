/// Lawkit test suite
/// Unified test structure across all test categories

// Allow common clippy warnings in test files
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::useless_vec)]
#![allow(clippy::single_match)]
#![allow(clippy::len_zero)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::needless_borrow)]

mod basic;
mod cli;
mod core;
mod docs_examples;
mod errors;
mod features;
mod formats;
mod integration;