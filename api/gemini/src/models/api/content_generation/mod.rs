//! Content generation API implementation.
//!
//! This module provides comprehensive content generation capabilities including
//! text generation, conversation handling, streaming, and batch processing.

// Public modules to ensure impl blocks are visible
#[ doc(hidden) ]
pub mod api_impl;

mod builder;

pub use builder::GenerationRequestBuilder;
