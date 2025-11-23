//! Comprehensive token validation test for `workspace_tools` 0.3.0
//! 
//! This test demonstrates that token loading and validation works correctly
//! with the updated `workspace_tools` version.

#![ cfg( all( feature = "workspace", feature = "secret_management" ) ) ]

use api_ollama::{ OllamaClient, SecretStore };
use std::env;
use std::fs;
#[ tokio::test ]
async fn test_comprehensive_token_validation()
{
  // Create a temporary workspace with realistic tokens
  let temp_dir = env::temp_dir().join("comprehensive_token_test");
  let secret_dir = temp_dir.join("secret");
  fs ::create_dir_all(&secret_dir).expect("Failed to create secret directory");

  // Create a realistic secret file with multiple tokens
  let secret_content = r#"#!/bin/bash
# API Keys
export OLLAMA_API_KEY="sk-proj-abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
export OPENAI_API_KEY="sk-1234567890abcdefghijklmnopqrstuvwxyz1234567890abcdefghijklmnopqr"
export ANTHROPIC_API_KEY="sk-ant-api03-1234567890abcdefghijklmnopqrstuvwxyz1234567890abcdef"

# Service URLs
export OLLAMA_URL="https://api.ollama.com/v1"
export DATABASE_URL="postgresql://user:securepass123@localhost:5432/myapp"

# Short tokens for testing edge cases
export SHORT_TOKEN="abc123"
"#;

  let secret_file = secret_dir.join("-secrets.sh");
  fs ::write(&secret_file, secret_content).expect("Failed to write secrets file");

  // Set workspace path
  let original_workspace = env::var("WORKSPACE_PATH").ok();
  env ::set_var("WORKSPACE_PATH", &temp_dir);

  // Test 1: Load secrets and validate they're correct
  println!( "🔑 Testing token loading..." );
  let secret_store = SecretStore::from_path(&temp_dir)
    .expect("Failed to create secret store");

  // Test valid API keys
  let ollama_key = secret_store.get_with_fallback("OLLAMA_API_KEY")
    .expect("Failed to get OLLAMA_API_KEY");
  assert_eq!(ollama_key, "sk-proj-abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890");
  assert!(ollama_key.starts_with("sk-proj-"));
  assert!(ollama_key.len() > 50, "API key should be long enough");
  println!( "✅ OLLAMA_API_KEY loaded and validated" );

  let openai_key = secret_store.get_with_fallback("OPENAI_API_KEY")
    .expect("Failed to get OPENAI_API_KEY");
  assert!(openai_key.starts_with("sk-"));
  assert!(openai_key.len() > 40, "OpenAI API key should be long enough");
  println!( "✅ OPENAI_API_KEY loaded and validated" );

  // Test URL validation
  let ollama_url = secret_store.get_with_fallback("OLLAMA_URL")
    .expect("Failed to get OLLAMA_URL");
  assert!(ollama_url.starts_with("https://"));
  println!( "✅ OLLAMA_URL loaded and validated" );

  // Test 2: Validate token format requirements
  println!( "🔍 Testing token validation..." );
  
  // Test short token handling
  let short_token = secret_store.get_with_fallback("SHORT_TOKEN")
    .expect("Failed to get SHORT_TOKEN");
  assert_eq!(short_token, "abc123");
  println!( "✅ Short token handled correctly" );

  // Test 3: Error handling for missing tokens
  println!( "❌ Testing error handling..." );
  let missing_result = secret_store.get_with_fallback("NONEXISTENT_TOKEN");
  assert!(missing_result.is_err(), "Should return error for missing token");
  println!( "✅ Missing token error handling works" );

  // Test 4: Environment fallback
  println!( "🌍 Testing environment fallback..." );
  env ::set_var("ENV_ONLY_TOKEN", "env-fallback-value");
  let env_token = secret_store.get_with_fallback("ENV_ONLY_TOKEN")
    .expect("Failed to get environment fallback token");
  assert_eq!(env_token, "env-fallback-value");
  println!( "✅ Environment fallback works" );

  // Test 5: Client creation with valid tokens
  println!( "🚀 Testing client creation with tokens..." );
  let client = OllamaClient::from_workspace_secrets_at(&temp_dir)
    .expect("Failed to create client from workspace secrets");
  
  // Verify client has proper configuration
  assert_eq!(client.base_url(), "https://api.ollama.com/v1");
  println!( "✅ Client created successfully with workspace tokens" );

  // Test 6: Token security (masking in debug output)
  println!( "🔒 Testing token security..." );
  let debug_output = format!( "{secret_store:?}" );
  assert!(!debug_output.contains("sk-proj-abcdef1234567890"));
  assert!(!debug_output.contains("sk-1234567890abcdef"));
  assert!(debug_output.contains("***"));
  println!( "✅ Tokens are properly masked in debug output" );

  // Cleanup
  env ::remove_var("ENV_ONLY_TOKEN");
  if let Some(original) = original_workspace
  {
    env ::set_var("WORKSPACE_PATH", original);
  }
  else
  {
    env ::remove_var("WORKSPACE_PATH");
  }
  let _ = fs::remove_dir_all(&temp_dir);

  println!( "🎉 All token validation tests passed!" );
}
#[ tokio::test ]
async fn test_token_validation_edge_cases()
{
  // Test edge cases for token validation
  let temp_dir = env::temp_dir().join("token_edge_cases");
  let secret_dir = temp_dir.join("secret");
  fs ::create_dir_all(&secret_dir).expect("Failed to create secret directory");

  // Create secret file with edge cases
  let secret_content = r#"#!/bin/bash
# Edge case tokens
export MALFORMED_KEY="not-a-real-api-key"
export VERY_LONG_KEY="sk-proj-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
export SPECIAL_CHARS_KEY="sk-test-special"
export WHITESPACE_KEY="  sk-test-with-whitespace  "
"#;

  let secret_file = secret_dir.join("-secrets.sh");
  fs ::write(&secret_file, secret_content).expect("Failed to write secrets file");

  let original_workspace = env::var("WORKSPACE_PATH").ok();
  env ::set_var("WORKSPACE_PATH", &temp_dir);

  let secret_store = SecretStore::from_path(&temp_dir)
    .expect("Failed to create secret store");

  // Test malformed key
  let malformed = secret_store.get_with_fallback("MALFORMED_KEY")
    .expect("Failed to get malformed key");
  assert_eq!(malformed, "not-a-real-api-key");
  
  // Test whitespace handling
  let whitespace_key = secret_store.get_with_fallback("WHITESPACE_KEY")
    .expect("Failed to get whitespace key");
  assert_eq!(whitespace_key.trim(), "sk-test-with-whitespace");

  // Cleanup
  if let Some(original) = original_workspace
  {
    env ::set_var("WORKSPACE_PATH", original);
  }
  else
  {
    env ::remove_var("WORKSPACE_PATH");
  }
  let _ = fs::remove_dir_all(&temp_dir);

  println!( "✅ Token edge case validation passed" );
}

#[ cfg( not( all( feature = "workspace", feature = "secret_management" ) ) ) ]
fn main()
{
  println!( "⚠️  Token validation tests require 'workspace' and 'secret_management' features" );
  println!( "Run with : cargo test --features 'workspace,secret_management'" );
}
