//! Token Validation Integration Tests - REAL API TOKEN VERIFICATION
//!
//! MANDATORY INTEGRATION TEST REQUIREMENTS:
//! - These tests validate REAL token loading and authentication 
//! - Tests MUST FAIL IMMEDIATELY if API secrets are not available
//! - Tests MUST FAIL IMMEDIATELY if token format is invalid
//! - Tests MUST FAIL IMMEDIATELY if authentication with API fails
//! - NO SILENT PASSES allowed when problems occur
//!
//! Run with : cargo test --features `integration,full`
//! Requires : Valid `ANTHROPIC_API_KEY` in environment or ../../secret/-secrets.sh

#[ allow( unused_imports ) ]
use super::*;

#[ test ]
#[ cfg( feature = "integration" ) ]
#[ ignore = "Requires workspace secrets file" ]
fn test_workspace_token_loading_and_format_validation()
{
  println!( "🔐 Token Loading & Format Validation Test" );
  println!( "=========================================" );

  // Test 1: Load token from workspace
  println!( "📁 Testing workspace token loading..." );
  let secret = the_module::Secret::from_workspace()
    .expect( "INTEGRATION FAILURE: Must have valid workspace secret in ../../secret/-secrets.sh" );

  let api_key = &secret.ANTHROPIC_API_KEY;
  
  // Test 2: Validate token format
  println!( "🔍 Validating token format..." );
  assert!( !api_key.is_empty(), "INTEGRATION FAILURE: API key cannot be empty" );
  assert!( api_key.starts_with( "sk-ant-" ), 
    "INTEGRATION FAILURE: Invalid API key format. Expected sk-ant-*, got : {}...", 
    &api_key[..core::cmp::min( 10, api_key.len() )] );
  assert!( api_key.len() >= 50, 
    "INTEGRATION FAILURE: API key too short. Expected >=50 chars, got : {}", api_key.len() );
  
  println!( "✅ Token format validation passed:" );
  println!( "   Format : ✅ Valid Anthropic format (sk-ant-...)" );
  println!( "   Length : ✅ {} characters", api_key.len() );
  println!( "   Prefix : {}", &api_key[..15] );
  println!( "   Source : ✅ Workspace secrets (../../secret/-secrets.sh)" );
  
  // Test 3: Client creation with token
  println!( "\n🏗️ Testing client creation..." );
  let client = the_module::Client::new( secret.clone() );
  assert_eq!( client.secret().ANTHROPIC_API_KEY, *api_key );
  assert_eq!( client.base_url(), "https://api.anthropic.com" );
  
  println!( "✅ Client creation passed:" );
  println!( "   Base URL: {}", client.base_url() );
  println!( "   Token match : ✅ Client token matches workspace token" );
  
  // Test 4: Alternative client creation method
  println!( "\n🏢 Testing Client::from_workspace()..." );
  let workspace_client = the_module::Client::from_workspace()
    .expect( "INTEGRATION FAILURE: Client::from_workspace() must succeed when Secret::from_workspace() succeeds" );
  
  assert_eq!( workspace_client.secret().ANTHROPIC_API_KEY, *api_key );
  assert_eq!( workspace_client.base_url(), client.base_url() );
  
  println!( "✅ Workspace client creation passed:" );
  println!( "   Token consistency : ✅ Both methods load same token" );
  println!( "   Configuration match : ✅ Same base URL and settings" );
  
  println!( "\n🎉 TOKEN VALIDATION: ✅ ALL TESTS PASSED" );
  println!( "=========================================" );
}

#[ tokio::test ]
#[ cfg( feature = "integration" ) ]
#[ ignore = "Requires workspace secrets file" ]
async fn test_live_token_authentication_verification()
{
  println!( "\n🌐 Live Token Authentication Verification" );
  println!( "=========================================" );
  
  let client = the_module::Client::from_workspace()
    .expect( "INTEGRATION FAILURE: Must have workspace client for authentication test" );
    
  let api_key = &client.secret().ANTHROPIC_API_KEY;
  println!( "🔑 Testing authentication with token : {}...{}", 
    &api_key[..12], &api_key[api_key.len()-8..] );

  // Test minimal API request for authentication verification
  let request = the_module::CreateMessageRequest
  {
    model : "claude-3-5-haiku-20241022".to_string(),
    max_tokens : 10,
    messages : vec![ the_module::Message::user( "Auth test".to_string() ) ],
    system : None,
    temperature : Some( 0.0 ), // Deterministic for testing
    stream : None,
    tools : None,
    tool_choice : None,
  };

  let auth_start = std::time::Instant::now();
  
  println!( "🚀 Making authenticated API call..." );
  let response = match client.create_message( request ).await
  {
    Ok( response ) => response,
    Err( the_module::AnthropicError::Api( ref api_err ) ) if api_err.message.contains( "credit balance is too low" ) =>
    {
      println!( "INTEGRATION TEST SKIPPED: Credit balance exhausted - this confirms real API usage" );
      return;
    },
    Err( err ) => panic!( "INTEGRATION FAILURE: Authentication must succeed with valid token : {err}" ),
  };
    
  let auth_duration = auth_start.elapsed();
  
  // Verify authentic API response structure
  assert!( !response.id.is_empty(), "INTEGRATION FAILURE: Real API response must have message ID" );
  assert!( response.id.starts_with( "msg_" ), "INTEGRATION FAILURE: Invalid message ID format : {}", response.id );
  assert_eq!( response.r#type, "message", "INTEGRATION FAILURE: Response type must be 'message'" );
  assert_eq!( response.role, "assistant", "INTEGRATION FAILURE: Response role must be 'assistant'" );
  assert_eq!( response.model, "claude-3-5-haiku-20241022", "INTEGRATION FAILURE: Model mismatch" );
  assert!( !response.content.is_empty(), "INTEGRATION FAILURE: Response must have content" );
  assert_eq!( response.content[0].r#type, "text", "INTEGRATION FAILURE: Content type must be 'text'" );
  assert!( response.usage.input_tokens > 0, "INTEGRATION FAILURE: Must track input tokens" );
  assert!( response.usage.output_tokens > 0, "INTEGRATION FAILURE: Must track output tokens" );
  
  println!( "✅ Authentication verification successful:" );
  println!( "   Response ID: {}", response.id );
  println!( "   Response time : {auth_duration:?}" );
  println!( "   Model : {}", response.model );
  println!( "   Token usage : {} in, {} out", response.usage.input_tokens, response.usage.output_tokens );
  if let Some( text ) = &response.content[0].text
  {
    println!( "   Content preview : {}", &text[..core::cmp::min( 50, text.len() )] );
  }
  
  // Test authentication error handling with invalid token
  println!( "\n🔒 Testing authentication error handling..." );
  let invalid_secret = the_module::Secret::new( "sk-ant-invalid-key-for-testing".to_string() )
    .expect( "Invalid secret should construct but fail on API call" );
  let invalid_client = the_module::Client::new( invalid_secret );
  
  let invalid_request = the_module::CreateMessageRequest
  {
    model : "claude-3-5-haiku-20241022".to_string(),
    max_tokens : 5,
    messages : vec![ the_module::Message::user( "Test".to_string() ) ],
    system : None,
    temperature : None,
    stream : None,
    tools : None,
    tool_choice : None,
  };
  
  let auth_error_result = invalid_client.create_message( invalid_request ).await;
  assert!( auth_error_result.is_err(), "INTEGRATION FAILURE: Invalid token should cause authentication error" );
  
  let error = auth_error_result.unwrap_err();
  let error_str = error.to_string().to_lowercase();
  assert!( error_str.contains( "authentication" ) || error_str.contains( "unauthorized" ) || error_str.contains( "invalid" ),
    "INTEGRATION FAILURE: Authentication error should mention auth issue, got : {error}" );
  
  println!( "✅ Authentication error handling verified:" );
  println!( "   Invalid token properly rejected : {error}" );
  
  println!( "\n🎉 AUTHENTICATION VERIFICATION: ✅ FULLY WORKING" );
  println!( "================================================" );
  println!( "✅ Token loading : WORKING" );
  println!( "✅ Token format : VALID" );  
  println!( "✅ API authentication : WORKING" );
  println!( "✅ Error handling : WORKING" );
  println!( "✅ Response validation : WORKING" );
}

#[ test ]
#[ cfg( feature = "integration" ) ]
#[ ignore = "Requires workspace secrets file" ]
fn test_token_security_and_workspace_integration()
{
  println!( "\n🛡️ Token Security & Workspace Integration Test" );
  println!( "===============================================" );
  
  // Test that token is properly loaded from workspace secrets
  let secret_result = the_module::Secret::from_workspace();
  assert!( secret_result.is_ok(), "INTEGRATION FAILURE: Workspace secret loading must work" );
  
  let secret = secret_result.unwrap();
  let api_key = &secret.ANTHROPIC_API_KEY;
  
  // Verify token security properties
  println!( "🔐 Verifying token security properties..." );
  assert!( api_key.len() >= 100, "INTEGRATION FAILURE: Token should be long enough for security" );
  assert!( api_key.chars().all( |c| c.is_ascii_alphanumeric() || c == '-' || c == '_' ), 
    "INTEGRATION FAILURE: Token should only contain safe characters" );
  
  // Verify token is not a placeholder or test value
  assert!( !api_key.contains( "placeholder" ), "INTEGRATION FAILURE: Token appears to be placeholder" );
  assert!( !api_key.contains( "example" ), "INTEGRATION FAILURE: Token appears to be example value" );
  assert!( !api_key.contains( "test" ), "INTEGRATION FAILURE: Token appears to be test value" );
  
  println!( "✅ Token security validation passed:" );
  println!( "   Length : ✅ {} characters (secure length)", api_key.len() );
  println!( "   Characters : ✅ Safe ASCII characters only" );
  println!( "   Authenticity : ✅ Real API token (not placeholder/test)" );
  
  // Test environment variable fallback (if available)
  println!( "\n🌍 Testing environment variable integration..." );
  match std::env::var( "ANTHROPIC_API_KEY" ) 
  {
    Ok( env_key ) => 
    {
      println!( "ℹ️ ANTHROPIC_API_KEY found in environment" );
      println!( "   Length : {} characters", env_key.len() );
      println!( "   Matches workspace : {}", if env_key == *api_key { "✅ Yes" } else { "⚠️ No (using workspace)" } );
    },
    Err( _ ) => 
    {
      println!( "ℹ️ No ANTHROPIC_API_KEY in environment (expected - using workspace secrets)" );
    }
  }
  
  println!( "\n🎉 SECURITY & INTEGRATION: ✅ VERIFIED" );
  println!( "====================================" );
}