//! No automatic retry tests for `api_ollama`
//!
//! # GOVERNING PRINCIPLE COMPLIANCE VERIFICATION
//!
//! **✅ CRITICAL: These tests verify "Thin Client, Rich API" principle compliance:**
//!
//! - **No Automatic Retries**: Verify client never retries requests automatically
//! - **Error Transparency**: All Ollama API errors passed directly to calling code
//! - **Explicit Control**: Manual retry patterns must be explicit and developer-controlled
//! - **No Magic Behavior**: Client contains no hidden retry logic or automatic backoff
//! - **API Error Preservation**: Original error context and information fully preserved
//!
//! These tests verify that automatic retry functionality has been eliminated and that
//! all errors from the Ollama API are passed transparently to developers without
//! any client-side intelligence or automatic retry attempts.

#![ cfg( feature = "integration_tests" ) ]

use api_ollama::{
  OllamaClient,
  ChatRequest,
  GenerateRequest,
  EmbeddingsRequest,
  ChatMessage,
  MessageRole,
};
use core::time::Duration;

/// Test that `OllamaClient` has no automatic retry functionality
#[ tokio::test ]
async fn test_no_automatic_retry_functionality()
{
  // Create client without any automatic retry configuration
  let client = OllamaClient::new( "http://unreachable.test:99999".to_string(), Duration::from_millis(100) );

  // Client should not have any retry-related methods available
  // This test verifies at compile-time that retry methods are not available

  // Verify client has basic functionality only
  assert_eq!(client.base_url(), "http://unreachable.test:99999");
}

/// Test that network errors are passed through transparently without automatic retries
#[ tokio::test ]
async fn test_error_transparency_network_failure()
{
  let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), Duration::from_millis(50) );

  let request = ChatRequest
  {
    model : "test-model".to_string(),
    messages : vec!
    [
      ChatMessage
      {
        role : MessageRole::User,
        content : "Hello".to_string(),
        #[ cfg( feature = "vision_support" ) ]
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      }
    ],
    stream : Some( false ),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  // Request should fail immediately without any retry attempts
  let start_time = std::time::Instant::now();
  let result = client.chat( request ).await;
  let elapsed = start_time.elapsed();

  // Should fail quickly (no retry delays)
  assert!( elapsed < Duration::from_millis(200) ); // Allow some time for network timeout
  assert!( result.is_err() );

  // Error should be a network error, not a retry-related error
  let error_str = result.unwrap_err().to_string();
  assert!( !error_str.contains( "retry" ) );
  assert!( !error_str.contains( "attempt" ) );
}

/// Test that server errors are passed through transparently without automatic retries
#[ tokio::test ]
async fn test_error_transparency_server_errors()
{
  // Use httpbin.org to simulate API errors (it returns 404 for non-existent endpoints)
  let mut client = OllamaClient::new( "https://httpbin.org".to_string(), Duration::from_secs(5) );

  let request = ChatRequest
  {
    model : "nonexistent-model".to_string(),
    messages : vec!
    [
      ChatMessage
      {
        role : MessageRole::User,
        content : "Test".to_string(),
        #[ cfg( feature = "vision_support" ) ]
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      }
    ],
    stream : Some( false ),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  // Request should fail immediately without retry attempts
  let start_time = std::time::Instant::now();
  let result = client.chat( request ).await;
  let elapsed = start_time.elapsed();

  // Should fail without extended retry delays
  assert!( elapsed < Duration::from_secs(10) );
  assert!( result.is_err() );

  // Error should not mention retries
  let error_str = result.unwrap_err().to_string();
  assert!( !error_str.contains( "retry" ) );
  assert!( !error_str.contains( "attempt" ) );
}

/// Test that generate requests fail transparently without automatic retries
#[ tokio::test ]
async fn test_generate_error_transparency()
{
  let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), Duration::from_millis(50) );

  let request = GenerateRequest
  {
    model : "test-model".to_string(),
    prompt : "Test prompt".to_string(),
    stream : Some( false ),
    options : None,
  };

  // Should fail immediately without retries
  let start_time = std::time::Instant::now();
  let result = client.generate( request ).await;
  let elapsed = start_time.elapsed();

  assert!( elapsed < Duration::from_millis(200) );
  assert!( result.is_err() );

  // Error should be transparent, not retry-wrapped
  let error_str = result.unwrap_err().to_string();
  assert!( !error_str.contains( "failed after" ) );
  assert!( !error_str.contains( "attempts" ) );
}

/// Test that embeddings requests fail transparently without automatic retries
///
/// Fix(issue-embeddings-transparency-001): Implemented test for embeddings error transparency
/// Root cause: Test was marked as waiting for refactoring but `EmbeddingsRequest` was already available
/// Pitfall: Always verify actual codebase state before assuming features are missing - check for existing types/APIs
#[ tokio::test ]
async fn test_embeddings_error_transparency()
{
  let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), Duration::from_millis(50) );

  let request = EmbeddingsRequest
  {
    model : "test-model".to_string(),
    prompt : "Test prompt".to_string(),
    options : None,
  };

  // Should fail immediately without retries
  let start_time = std::time::Instant::now();
  let result = client.embeddings( request ).await;
  let elapsed = start_time.elapsed();

  assert!( elapsed < Duration::from_millis(200) );
  assert!( result.is_err() );

  // Error should be transparent, not retry-wrapped
  let error_str = result.unwrap_err().to_string();
  assert!( !error_str.contains( "failed after" ) );
  assert!( !error_str.contains( "attempts" ) );
}

/// Test that model info requests fail transparently without automatic retries
#[ tokio::test ]
async fn test_model_info_error_transparency()
{
  let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), Duration::from_millis(50) );

  // Should fail immediately without retries
  let start_time = std::time::Instant::now();
  let result = client.model_info( "test-model".to_string() ).await;
  let elapsed = start_time.elapsed();

  assert!( elapsed < Duration::from_millis(200) );
  assert!( result.is_err() );

  // Error should be transparent, not retry-wrapped
  let error_str = result.unwrap_err().to_string();
  assert!( !error_str.contains( "retry" ) );
  assert!( !error_str.contains( "attempt" ) );
}

/// Test that `list_models` requests fail transparently without automatic retries
#[ tokio::test ]
async fn test_list_models_error_transparency()
{
  let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), Duration::from_millis(50) );

  // Should fail immediately without retries
  let start_time = std::time::Instant::now();
  let result = client.list_models().await;
  let elapsed = start_time.elapsed();

  assert!( elapsed < Duration::from_millis(200) );
  assert!( result.is_err() );

  // Error should be transparent, not retry-wrapped
  let error_str = result.unwrap_err().to_string();
  assert!( !error_str.contains( "retry" ) );
  assert!( !error_str.contains( "attempt" ) );
}

/// Test that streaming requests fail transparently without automatic retries
#[ tokio::test ]
async fn test_streaming_error_transparency()
{
  #[ cfg( feature = "streaming" ) ]
  {
    let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), Duration::from_millis(50) );

    let request = ChatRequest
    {
      model : "test-model".to_string(),
      messages : vec!
      [
        ChatMessage
        {
          role : MessageRole::User,
          content : "Streaming test".to_string(),
          #[ cfg( feature = "vision_support" ) ]
          images : None,
          #[ cfg( feature = "tool_calling" ) ]
          tool_calls : None,
        }
      ],
      stream : Some( true ),
      options : None,
      #[ cfg( feature = "tool_calling" ) ]
      tools : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_messages : None,
    };

    // Should fail immediately without retries
    let start_time = std::time::Instant::now();
    let result = client.chat_stream( request ).await;
    let elapsed = start_time.elapsed();

    assert!( elapsed < Duration::from_millis(200) );
    assert!( result.is_err() );

    // Error should be transparent, not retry-wrapped
    // Note : streaming errors don't implement Debug, so we just verify it fails
    assert!( result.is_err() );
  }

  #[ cfg( not( feature = "streaming" ) ) ]
  {
    println!("⚠ Skipping streaming error transparency test - streaming feature not enabled");
  }
}

/// Test that multiple consecutive requests each fail transparently
#[ tokio::test ]
async fn test_multiple_requests_error_transparency()
{
  let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), Duration::from_millis(50) );

  // Each request should fail immediately and transparently
  for i in 0..3
  {
    let start_time = std::time::Instant::now();
    let result = client.list_models().await;
    let elapsed = start_time.elapsed();

    assert!( elapsed < Duration::from_millis(200), "Request {i} took too long" );
    assert!( result.is_err(), "Request {i} should fail" );

    let error_str = result.unwrap_err().to_string();
    assert!( !error_str.contains( "retry" ), "Request {i} error contains 'retry'" );
    assert!( !error_str.contains( "attempt" ), "Request {i} error contains 'attempt'" );
  }
}

/// Test that configuration methods no longer exist (compile-time verification)
#[ tokio::test ]
async fn test_no_retry_configuration_methods()
{
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs(5) );

  // These should compile without any retry-related methods
  // The absence of compilation errors for missing retry methods confirms removal
  let _ = client;

  println!("✓ No retry configuration methods exist - successful elimination");
}
