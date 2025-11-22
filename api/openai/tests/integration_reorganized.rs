//! Reorganized integration tests for `OpenAI` API client
//!
//! This file replaces the monolithic integration.rs with a well-organized
//! modular structure split by functionality.
//!
//! These tests make real API calls to `OpenAI` and require a valid API key.
//! They are gated behind the "integration" feature flag.
//!
//! # MANDATORY FAILING BEHAVIOR
//!
//! Integration tests in this file and its modules MUST fail hard when:
//! - Real API credentials are not available in environment or workspace secrets
//! - Network connectivity issues prevent API access
//! - API authentication or authorization fails
//! - Any other real API access issues occur
//!
//! **IMPORTANT**: These tests NEVER silently fall back to mocks or dummy data.
//! Test failures indicate real issues that must be addressed:
//! - Missing `OPENAI_API_KEY` in environment or ../../secret/-secrets.sh
//! - Invalid/expired API credentials
//! - Network connectivity problems
//! - `OpenAI` API service issues
//!
//! This ensures integration test results are meaningful and reliable.
//!
//! All tests use the test isolation framework to ensure proper test isolation
//! and prevent shared state issues.

#![ cfg( feature = "integration" ) ]

pub use api_openai as the_module;

mod test_isolation;

// Import organized test modules
mod integration_tests;

// Re-export test modules for easy access
pub use integration_tests::{
  response_creation,
  response_management,
  environment,
  shared,
};