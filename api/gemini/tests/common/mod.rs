//! Common test utilities shared across all integration tests
//!
//! This module provides test helpers that enforce the NO-MOCKUP policy
//! by failing explicitly when API keys are unavailable.

use api_gemini::client::Client;

/// Create client for integration tests - REQUIRES real API key
///
/// # Panics
///
/// Panics immediately with a helpful error message if no valid API key is found.
/// This is intentional - integration tests should fail explicitly, not skip silently.
///
/// # Example
///
/// ```no_run
/// use common::create_integration_client;
///
/// let client = create_integration_client();
/// // Test continues only if API key is valid
/// ```
pub fn create_integration_client() -> Client
{
  Client::new().unwrap_or_else( |err| {
    panic!(
    "\n\n❌ INTEGRATION TEST FAILURE: No valid API key found!\n\
    \n🔑 API Key Required From:\n\
    \n   1. Environment variable: GEMINI_API_KEY\n\
    \n   2. Workspace secret file: secret/gemini_api_key\n\
    \n      (using workspace_tools 0.6.0 for secret loading)\n\
    \n📋 This integration test validates functionality with REAL Gemini API calls\n\
    \n🚫 Integration tests NEVER skip silently - missing API keys cause explicit failures\n\
    \n💡 To run ONLY unit tests (no API required):\n\
    \n   cargo test --no-default-features\n\
    \n📖 See tests/readme.md for complete testing guide and setup instructions\n\
  \n🔍 Original error from Client::new(): {err:?}\n\n"
    );
  })
}
