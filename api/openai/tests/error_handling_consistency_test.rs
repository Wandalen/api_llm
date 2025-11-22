//! Test error handling consistency after standardization
//!
//! This test verifies that all error handling patterns use the untyped Result< T >
//! approach consistently throughout the codebase.

use api_openai::
{
  secret ::Secret,
  environment ::{ OpenaiEnvironmentImpl, OpenaiEnvironment },
};

/// Test that Secret creation and validation uses untyped Result properly
#[ test ]
fn test_secret_error_handling_consistency()
{
  // Test invalid API key format - should return untyped Result with OpenAIError
  let result = Secret::new( "invalid-key".to_string() );
  assert!( result.is_err() );

  // Test valid API key format - should succeed
  let result = Secret::new( "sk-1234567890123456789012345678901234567890123456789012".to_string() );
  assert!( result.is_ok() );
}

/// Test that environment configuration uses untyped Result properly
#[ test ]
fn test_environment_error_handling_consistency()
{
  // Test invalid base URL - should return untyped Result with OpenAIError
  let secret = Secret::new_unchecked( "sk-test123".to_string() );
  let result = OpenaiEnvironmentImpl::build(
    secret,
    None,
    None,
    "invalid-url".to_string(),
    api_openai ::environment::OpenAIRecommended::realtime_base_url().to_string()
  );
  assert!( result.is_err() );
}

/// Test that environment methods use untyped Result properly
#[ test ]
fn test_environment_methods_error_handling_consistency()
{
  let secret = Secret::new_unchecked( "sk-test123".to_string() );
  let env = OpenaiEnvironmentImpl::build(
    secret,
    None,
    None,
    api_openai ::environment::OpenAIRecommended::base_url().to_string(),
    api_openai ::environment::OpenAIRecommended::realtime_base_url().to_string()
  ).expect( "Should build environment successfully" );

  // Test headers method uses untyped Result
  let headers_result = env.headers();
  assert!( headers_result.is_ok() );

  // Test URL joining methods use untyped Result
  let url_result = env.join_base_url( "chat/completions" );
  assert!( url_result.is_ok() );

  let realtime_url_result = env.join_realtime_base_url( "connect" );
  assert!( realtime_url_result.is_ok() );
}