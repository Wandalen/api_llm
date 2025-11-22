//! Integration test modules organized by functionality
//!
//! This module provides a clean organization of integration tests split by
//! functionality rather than having everything in a single monolithic file.

pub mod response_creation;
pub mod response_management;
pub mod environment;
pub mod shared;

// Re-export commonly used test utilities
// Note : shared::* not re-exported here to avoid unused import warnings
// Each module imports from shared directly as needed