//! HTML Element Module
//!
//! This module contains HTML element functionality split into focused submodules.
//! Each submodule handles specific aspects of element behavior and manipulation.

pub mod attributes;
pub mod class_list;
pub mod content;
pub mod main; // base struct & core methods
pub mod normalize;
pub mod serialize; // to / from html
pub mod text_ops;
pub mod tree; // attribute normalization utilities

pub use main::*;
pub(super) use normalize::normalize_attr_quotes;
