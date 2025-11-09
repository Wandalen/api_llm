//! Integration tests demonstrating enhanced retry functionality
//!
//! These tests validate that the enhanced retry logic works correctly
//! with the actual `OllamaClient` implementation.

#![ cfg( all( feature = "retry", feature = "integration_tests" ) ) ]

use api_ollama::{
  OllamaClient,
  ChatRequest,
  ChatMessage,
  MessageRole,
  RetryConfig,
};
use core::time::Duration;
use std::time::Instant;

/// Test that retry-enabled client can be configured correctly
#[ tokio::test ]
async fn test_retry_client_configuration()
{
  let retry_config = RetryConfig::new()
    .with_max_attempts( 3 )
    .with_base_delay_ms( 100 )
    .with_jitter_ms( 50 )
    .with_max_elapsed_time( Duration::from_secs( 10 ) )
    .with_backoff_multiplier( 2.0 )
    .with_logging( false ); // Disable logging for clean test output

  let client = OllamaClient::new(
    "http://unreachable.test:99999".to_string(),
    Duration::from_millis( 100 )
  ).with_retry_config( retry_config );

  // Verify retry stats are available
  let stats = client.retry_stats();
  assert!( stats.is_some() );

  let stats = stats.unwrap();
  assert_eq!( stats.total_attempts, 0 ); // No attempts yet

  println!( "✓ Retry client configured successfully" );
}

/// Test that default retry configuration works
#[ tokio::test ]
async fn test_default_retry_configuration()
{
  let client = OllamaClient::new(
    "http://localhost:11434".to_string(),
    Duration::from_secs( 5 )
  ).with_default_retry();

  // Verify retry stats are available with default config
  let stats = client.retry_stats();
  assert!( stats.is_some() );

  println!( "✓ Default retry configuration works" );
}

/// Test retry functionality with unreachable endpoint
#[ tokio::test ]
async fn test_retry_with_unreachable_endpoint()
{
  let retry_config = RetryConfig::new()
    .with_max_attempts( 2 ) // Low attempt count for quick test
    .with_base_delay_ms( 10 ) // Short delay for quick test
    .with_jitter_ms( 5 )
    .with_max_elapsed_time( Duration::from_secs( 5 ) )
    .with_logging( false ); // Disable logging for clean test

  let mut client = OllamaClient::new(
    "http://unreachable.test:99999".to_string(),
    Duration::from_millis( 50 )
  ).with_retry_config( retry_config );

  let request = ChatRequest
  {
    model: "test-model".to_string(),
    messages: vec!
    [
      ChatMessage
      {
        role: MessageRole::User,
        content: "Test retry".to_string(),
        #[ cfg( feature = "vision_support" ) ]
        images: None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls: None,
      }
    ],
    stream: Some( false ),
    options: None,
    #[ cfg( feature = "tool_calling" ) ]
    tools: None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages: None,
  };

  // Test with retry - should take longer due to retry delays
  let start_time = Instant::now();
  let result = client.chat_with_retries( request ).await;
  let elapsed_with_retry = start_time.elapsed();

  assert!( result.is_err() );

  // Should take longer than a single request due to retry delays
  // Note: Timing can be variable in test environments, so we just check that retries occurred
  assert!( elapsed_with_retry >= Duration::from_millis( 5 ) ); // Some additional time for retries

  // Check retry stats
  let stats = client.retry_stats().unwrap();
  assert!( stats.total_attempts >= 2 ); // Should have made multiple attempts
  assert_eq!( stats.successful_retries, 0 ); // No success on unreachable endpoint
  assert_eq!( stats.failed_operations, 1 ); // One failed operation

  println!( "✓ Retry functionality validated: {} attempts in {:?}", stats.total_attempts, elapsed_with_retry );
}

/// Test retry vs non-retry behavior comparison
#[ tokio::test ]
async fn test_retry_vs_normal_behavior()
{
  let base_url = "http://unreachable.test:99999".to_string();
  let timeout = Duration::from_millis( 50 );

  // Client without retry
  let mut normal_client = OllamaClient::new( base_url.clone(), timeout );

  // Client with retry
  let retry_config = RetryConfig::new()
    .with_max_attempts( 2 )
    .with_base_delay_ms( 10 )
    .with_logging( false );

  let mut retry_client = OllamaClient::new( base_url, timeout )
    .with_retry_config( retry_config );

  let request = ChatRequest
  {
    model: "test".to_string(),
    messages: vec!
    [
      ChatMessage
      {
        role: MessageRole::User,
        content: "Test".to_string(),
        #[ cfg( feature = "vision_support" ) ]
        images: None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls: None,
      }
    ],
    stream: Some( false ),
    options: None,
    #[ cfg( feature = "tool_calling" ) ]
    tools: None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages: None,
  };

  // Test normal behavior (should be fast)
  let start_time = Instant::now();
  let normal_result = normal_client.chat( request.clone() ).await;
  let normal_elapsed = start_time.elapsed();

  // Test retry behavior (should be slower)
  let start_time = Instant::now();
  let retry_result = retry_client.chat_with_retries( request ).await;
  let retry_elapsed = start_time.elapsed();

  // Both should fail
  assert!( normal_result.is_err() );
  assert!( retry_result.is_err() );

  // Retry should take longer due to multiple attempts
  assert!( retry_elapsed > normal_elapsed );

  println!( "✓ Normal: {normal_elapsed:?}, Retry: {retry_elapsed:?} - retry took longer as expected" );
}

/// Test that retry methods gracefully handle no retry configuration
#[ tokio::test ]
async fn test_retry_methods_without_configuration()
{
  let mut client = OllamaClient::new(
    "http://unreachable.test:99999".to_string(),
    Duration::from_millis( 50 )
  );
  // Note: No retry configuration set

  let request = ChatRequest
  {
    model: "test".to_string(),
    messages: vec!
    [
      ChatMessage
      {
        role: MessageRole::User,
        content: "Test".to_string(),
        #[ cfg( feature = "vision_support" ) ]
        images: None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls: None,
      }
    ],
    stream: Some( false ),
    options: None,
    #[ cfg( feature = "tool_calling" ) ]
    tools: None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages: None,
  };

  // Should work like normal method when no retry configured
  let start_time = Instant::now();
  let result = client.chat_with_retries( request ).await;
  let elapsed = start_time.elapsed();

  assert!( result.is_err() );
  assert!( elapsed < Duration::from_millis( 100 ) ); // Should be quick like normal method

  // Retry stats should be None when no retry configured
  assert!( client.retry_stats().is_none() );

  println!( "✓ Retry methods gracefully handle no configuration" );
}

/// Test retry statistics reset functionality
#[ tokio::test ]
async fn test_retry_statistics_reset()
{
  let retry_config = RetryConfig::new()
    .with_max_attempts( 2 )
    .with_base_delay_ms( 5 )
    .with_logging( false );

  let mut client = OllamaClient::new(
    "http://unreachable.test:99999".to_string(),
    Duration::from_millis( 50 )
  ).with_retry_config( retry_config );

  let request = ChatRequest
  {
    model: "test".to_string(),
    messages: vec!
    [
      ChatMessage
      {
        role: MessageRole::User,
        content: "Test".to_string(),
        #[ cfg( feature = "vision_support" ) ]
        images: None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls: None,
      }
    ],
    stream: Some( false ),
    options: None,
    #[ cfg( feature = "tool_calling" ) ]
    tools: None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages: None,
  };

  // Make a request to generate some stats
  let _ = client.chat_with_retries( request ).await;

  // Check that stats exist
  let stats_before = client.retry_stats().unwrap();
  assert!( stats_before.total_attempts > 0 );

  // Reset stats
  client.reset_retry_stats();

  // Check that stats are reset
  let stats_after = client.retry_stats().unwrap();
  assert_eq!( stats_after.total_attempts, 0 );
  assert_eq!( stats_after.successful_retries, 0 );
  assert_eq!( stats_after.failed_operations, 0 );

  println!( "✓ Retry statistics reset functionality works" );
}

/// Test different retry methods (chat, generate, `list_models`, `model_info`)
#[ tokio::test ]
async fn test_different_retry_methods()
{
  let retry_config = RetryConfig::new()
    .with_max_attempts( 2 )
    .with_base_delay_ms( 5 )
    .with_logging( false );

  let mut client = OllamaClient::new(
    "http://unreachable.test:99999".to_string(),
    Duration::from_millis( 30 )
  ).with_retry_config( retry_config );

  // Test list_models_with_retries
  let list_result = client.list_models_with_retries().await;
  assert!( list_result.is_err() );

  // Test model_info_with_retries
  let info_result = client.model_info_with_retries( "test-model".to_string() ).await;
  assert!( info_result.is_err() );

  // Check that stats accumulated from multiple retry method calls
  let stats = client.retry_stats().unwrap();
  assert!( stats.total_attempts >= 4 ); // At least 2 attempts per method × 2 methods

  println!( "✓ Different retry methods work: {} total attempts", stats.total_attempts );
}
