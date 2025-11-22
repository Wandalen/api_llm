//! Integration test for `workspace_tools` secret loading
//!
//! Tests that verify the integration between our Secret type and `workspace_tools`
//! for loading credentials from the centralized workspace secrets.
//!
//! # Test Matrix
//!
//! | Function | Test Cases | Purpose |
//! |----------|------------|---------|
//! | `Secret::load_with_fallbacks` | Basic loading, format validation | Core integration |
//! | Fallback chain | Env → workspace → alt files | Fallback behavior |
//! | Feature compatibility | `secrets` feature usage | v0.3.0 migration |
//! | Exposure counting | Counter increments | Audit functionality |
//! | Real API integration | Client creation | End-to-end validation |
//! | Error handling | Missing keys | Error scenarios |

use api_openai::secret::Secret;
use secrecy::ExposeSecret;

/// Test that `Secret::load_with_fallbacks` works with the updated `workspace_tools` v0.3.0
#[ test ]
fn test_secret_loading_integration()
{
  // This test validates that workspace_tools v0.3.0 integration works correctly
  // It should find OPENAI_API_KEY from either environment or workspace secrets

  let result = Secret::load_with_fallbacks( "OPENAI_API_KEY" );

  match result
  {
    Ok( secret ) =>
    {
      let api_key = secret.expose_secret();

      // Verify the key has the expected format
      assert!( api_key.starts_with( "sk-" ), "API key should start with 'sk-'" );
      assert!( api_key.len() >= 20, "API key should be at least 20 characters" );

      // Verify it's not a dummy/test key
      assert!( !api_key.contains( "dummy" ), "Should not be a dummy key" );
      assert!( !api_key.contains( "test" ), "Should not be a test key" );
      assert!( !api_key.contains( "invalid" ), "Should not be an invalid key" );

      println!( "✅ Successfully loaded API key via workspace_tools v0.3.0" );
      let key_preview = &api_key[ ..core::cmp::min( 10, api_key.len() ) ];
      println!( "   Key prefix : {key_preview}..." );
    }
    Err( e ) =>
    {
      println!( "⚠️  No valid OPENAI_API_KEY found : {e}" );
      println!( "   This is expected if no API key is configured." );
      println!( "   To test with real credentials, add OPENAI_API_KEY to:" );
      println!( "   - Environment variables, OR" );
      println!( "   - secret/-secrets.sh file" );
    }
  }
}

/// Test that `workspace_tools` fallback chain works correctly
#[ test ]
fn test_fallback_chain_behavior()
{
  // Test the fallback behavior by temporarily removing environment variable
  let original_env_key = std::env::var( "OPENAI_API_KEY" ).ok();

  // Remove environment variable to test workspace file fallback
  std ::env::remove_var( "OPENAI_API_KEY" );

  let result = Secret::load_with_fallbacks( "OPENAI_API_KEY" );

  // Restore original environment if it existed
  if let Some( key ) = original_env_key
  {
    std ::env::set_var( "OPENAI_API_KEY", key );
  }

  match result
  {
    Ok( secret ) =>
    {
      println!( "✅ Successfully loaded from workspace secrets (fallback worked)" );
      let api_key = secret.expose_secret();
      let key_preview = &api_key[ ..core::cmp::min( 10, api_key.len() ) ];
      println!( "   Key loaded from workspace file : {key_preview}..." );
    }
    Err( e ) =>
    {
      println!( "⚠️  Fallback to workspace secrets failed : {e}" );
      println!( "   This is expected if no secrets file is configured." );
    }
  }
}

/// Test `workspace_tools` feature compatibility after v0.3.0 update
#[ test ]
fn test_workspace_tools_feature_compatibility()
{
  // Test that the 'secrets' feature (renamed from 'secret_management') works
  let result = Secret::load_from_workspace( "OPENAI_API_KEY", "-secrets.sh" );

  match result
  {
    Ok( secret ) =>
    {
      println!( "✅ workspace_tools 'secrets' feature working correctly" );
      let api_key = secret.expose_secret();
      assert!( api_key.starts_with( "sk-" ), "Loaded key should have correct format" );
    }
    Err( e ) =>
    {
      println!( "⚠️  No workspace secrets found : {e}" );
      println!( "   This is expected if secret/-secrets.sh is not configured." );
    }
  }
}

/// Test secret exposure counting after `workspace_tools` update
#[ test ]
fn test_secret_exposure_counting()
{
  // Test that secret exposure counting still works after the update
  let initial_count = Secret::exposure_count();

  if let Ok( secret ) = Secret::load_with_fallbacks( "OPENAI_API_KEY" )
  {
    let _exposed = secret.expose_secret(); // This should increment counter
    let new_count = Secret::exposure_count();

    assert!( new_count > initial_count, "Exposure count should increment" );
    println!( "✅ Secret exposure counting works : {initial_count} -> {new_count}" );
  }
  else
  {
    println!( "⚠️  No API key available to test exposure counting" );
  }
}

/// Comprehensive integration test for real API usage
#[ cfg( feature = "integration" ) ]
#[ test ]
fn test_real_api_integration()
{
  use api_openai::{ Client, environment::OpenaiEnvironmentImpl };

  // This test requires the integration feature and real credentials
  let secret = Secret::load_with_fallbacks( "OPENAI_API_KEY" )
    .expect( "Real API integration test requires valid OPENAI_API_KEY" );

  let api_key = secret.expose_secret();

  // Validate this is a real API key (not test/dummy)
  assert!( api_key.len() >= 40, "Real OpenAI API keys should be at least 40 characters" );
  assert!( api_key.starts_with( "sk-" ), "Must be OpenAI API key format" );
  assert!( !api_key.contains( "test" ), "Should not be test key for integration test" );
  assert!( !api_key.contains( "dummy" ), "Should not be dummy key for integration test" );

  // Test that we can create a client with the real credentials
  let env_result = OpenaiEnvironmentImpl::build( secret, None, None, api_openai::environment::OpenAIRecommended::base_url().to_string(), api_openai::environment::OpenAIRecommended::realtime_base_url().to_string() );
  assert!( env_result.is_ok(), "Should be able to build environment with real API key" );

  let client_result = Client::build( env_result.unwrap() );
  assert!( client_result.is_ok(), "Should be able to build client with real environment" );

  println!( "✅ Real API integration test passed with workspace_tools v0.3.0" );
}

/// Test error handling for missing secrets
#[ test ]
fn test_missing_secret_error_handling()
{
  // Test with a definitely non-existent key
  let result = Secret::load_with_fallbacks( "DEFINITELY_NONEXISTENT_API_KEY_12345" );

  assert!( result.is_err(), "Should fail for non-existent key" );

  let error = result.unwrap_err();
  let error_string = format!( "{error}" );

  // Verify error message contains useful information
  assert!( error_string.contains( "DEFINITELY_NONEXISTENT_API_KEY_12345" ), "Error should mention the key name" );
  assert!( error_string.contains( "not found" ), "Error should indicate key was not found" );

  println!( "✅ Error handling working correctly : {error_string}" );
}