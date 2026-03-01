//! HTML Parser Module
//!
//! This module provides HTML parsing functionality with high performance
//! zero-copy optimizations. The parser is organized into specialized
//! submodules for different concerns.

// Core modules
mod api;
mod attrs;
mod core_parser;
mod fast_parser;
mod types;
mod utils;
mod validator;

// Public re-exports
pub use api::parse;
pub use core_parser::parse_with_options;
pub use types::Options;
pub use validator::valid;
