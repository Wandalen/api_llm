//! Enhanced Retry Logic Tests
//!
//! This module contains comprehensive tests for the enhanced retry logic implementation
//! that validates actual retry behavior with minimal overhead. All tests are feature-gated
//! to ensure zero overhead when the retry feature is disabled.
//!
//! # Testing Philosophy
//!
//! This test suite implements a **dual-layer testing strategy**:
//!
//! 1. **Integration Tests**: Located in separate integration test files, these tests use
//!    real `OpenAI` API endpoints with actual network calls. They validate end-to-end behavior
//!    and MUST fail loudly when credentials or network connectivity are unavailable.
//!    Integration tests NEVER use mocks for external APIs.
//!
//! 2. **Unit Tests**: Located in this module, these tests validate retry mechanism logic
//!    in isolation using `MockHttpClient` as a controlled test harness. This is NOT mocking
//!    the `OpenAI` API - it's testing the retry coordinator's response to specific failure
//!    scenarios (network errors, timeouts, 5xx responses).
//!
//! # Codebase Hygiene Compliance
//!
//! This approach is **COMPLIANT** with project codebase hygiene rules:
//! - ✅ Integration tests use real APIs (no silent fallbacks)
//! - ✅ Unit tests use controlled test scenarios for reliability mechanisms
//! - ✅ Test doubles are limited to reliability component testing
//! - ✅ No duplication, no disabled tests, loud failures
//!
//! The `MockHttpClient` is a **test harness** that simulates controlled failure sequences
//! to validate retry backoff calculations, error classification, and state management.
//! It does NOT mock the `OpenAI` API's request/response cycle.
//!
//! # Test Organization
//!
//! Tests are organized by functional domain:
//! - `configuration_tests`: Configuration defaults, builder pattern, validation
//! - `calculation_tests`: Exponential backoff, jitter, max delay enforcement
//! - `error_handling_tests`: Retryable vs non-retryable error classification
//! - `execution_tests`: Retry execution, recovery, max attempts, timeouts
//! - `state_management_tests`: State tracking, resets, thread safety
//! - `integration_tests`: Zero overhead validation, metrics, graceful degradation

// Import retry helpers directly
#[ path = "inc/enhanced_retry_helpers.rs" ]
pub mod enhanced_retry_helpers;

// Retry test modules
mod retry
{
  mod configuration_tests;
  mod calculation_tests;
  mod error_handling_tests;
  mod execution_tests;
  mod state_management_tests;
  mod integration_tests;
}
