//! CSS Selector Engine Module
//!
//! This module provides CSS selector functionality for querying HTML elements.
//! It includes compilation, selection, and matching capabilities.

pub mod api;
pub mod attributes;
pub mod compile;
pub mod convert;
pub mod general;
pub mod helpers;
pub mod legacy;
pub mod types;

// Re-export public API
pub use api::*;
