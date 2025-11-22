//! Environment and configuration integration tests
//!
//! Tests for environment setup, configuration validation,
//! and client initialization functionality.
//!
//! # MANDATORY FAILING BEHAVIOR
//!
//! These integration tests MUST fail hard when real API access is unavailable.
//! Tests NEVER silently fall back to fallbacks. Failures indicate real issues:
//! - Missing `OPENAI_API_KEY` credentials
//! - Network connectivity problems
//! - API authentication/authorization failures
//! - `OpenAI` service unavailability

#![ allow( clippy::uninlined_format_args, clippy::len_zero ) ] // Test code can be more verbose for clarity

use super::shared::{ *, IsolatedClient, should_run_real_api_tests };

/// Tests environment details and configuration.
/// Test Combination : E1.1
/// Uses proper test isolation to prevent shared state issues.
#[ tokio::test ]
#[ allow( deprecated ) ]
async fn test_environment_details()
{
  let isolated_client = IsolatedClient::new("test_environment_details", should_run_real_api_tests())
    .expect("Failed to create isolated client");
  let client = isolated_client.client();

  // Test that the client has proper environment configuration
  let environment = &client.environment;

  // Verify environment has required components
  use api_openai::OpenaiEnvironment;
  assert!(OpenaiEnvironment::api_key(environment).expose_secret().len() > 0, "Environment should have API key");

  // Test environment interface methods
  let base_url = environment.base_url();
  assert!(base_url.scheme() == "https", "Base URL should use HTTPS");
  assert!(base_url.to_string().contains("openai"), "Base URL should contain 'openai'");

  let organization = OpenaiEnvironment::organization_id(environment);
  // Organization can be None, so we just check it doesn't panic

  let project = OpenaiEnvironment::project_id(environment);
  // Project can be None, so we just check it doesn't panic

  println!("Environment test completed successfully");
  println!("Base URL: {}", base_url);
  println!("Organization : {:?}", organization);
  println!("Project : {:?}", project);
}