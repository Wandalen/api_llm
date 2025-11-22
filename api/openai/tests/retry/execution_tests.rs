//! Retry Execution Tests
//!
//! Tests for `EnhancedRetryExecutor` behavior including:
//! - Successful operations without retry
//! - Recovery from transient failures
//! - Max attempts enforcement
//! - Non-retryable error handling
//! - Max elapsed time enforcement

#[ cfg( feature = "retry" ) ]
mod retry_execution_tests
{
  use crate::enhanced_retry_helpers::*;
  use api_openai::error::OpenAIError;
  use core::time::Duration;
  use std::time::Instant;

  #[ tokio::test ]
  async fn test_retry_executor_successful_operation()
  {
    let config = EnhancedRetryConfig::new().with_max_attempts( 3 );
    let executor = EnhancedRetryExecutor::new( config ).unwrap();

    // Operation that succeeds immediately
    let result = executor.execute( || async { Ok( "success" ) } ).await;

    assert!( result.is_ok() );
    assert_eq!( result.unwrap(), "success" );

    // Check that only one attempt was made
    let state = executor.get_state();
    assert_eq!( state.attempt, 1 );
    assert_eq!( state.total_attempts, 1 );
  }

  #[ tokio::test ]
  async fn test_retry_executor_with_transient_failures()
  {
    let config = EnhancedRetryConfig::new()
      .with_max_attempts( 3 )
      .with_base_delay( 10 ) // Short delay for testing
      .with_jitter( 0 );
    let executor = EnhancedRetryExecutor::new( config ).unwrap();

    // Mock client that fails twice then succeeds
    let mock_client = MockHttpClient::new( vec![
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Err( OpenAIError::Http( "HTTP error with status 500: Server error".to_string() ).into() ),
      Ok( "success".to_string() ),
    ] );

    let result = executor.execute( || mock_client.make_request() ).await;

    assert!( result.is_ok() );
    assert_eq!( result.unwrap(), "success" );

    // Check that three attempts were made
    let state = executor.get_state();
    assert_eq!( state.attempt, 3 );
    assert_eq!( state.total_attempts, 3 );
    assert_eq!( mock_client.get_call_count(), 3 );
  }

  #[ tokio::test ]
  async fn test_retry_executor_exceeds_max_attempts()
  {
    let config = EnhancedRetryConfig::new()
      .with_max_attempts( 2 )
      .with_base_delay( 10 )
      .with_jitter( 0 );
    let executor = EnhancedRetryExecutor::new( config ).unwrap();

    // Mock client that always fails with retryable error
    let mock_client = MockHttpClient::new( vec![
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
    ] );

    let result = executor.execute( || mock_client.make_request() ).await;

    assert!( result.is_err() );

    // Check that max attempts were made
    let state = executor.get_state();
    assert_eq!( state.attempt, 2 );
    assert_eq!( state.total_attempts, 2 );
    assert_eq!( mock_client.get_call_count(), 2 );
  }

  #[ tokio::test ]
  async fn test_retry_executor_non_retryable_error()
  {
    let config = EnhancedRetryConfig::new().with_max_attempts( 3 );
    let executor = EnhancedRetryExecutor::new( config ).unwrap();

    // Mock client with non-retryable error
    let mock_client = MockHttpClient::new( vec![
      Err( OpenAIError::InvalidArgument( "Invalid API key".to_string() ).into() ),
    ] );

    let result = executor.execute( || mock_client.make_request() ).await;

    assert!( result.is_err() );

    // Check that only one attempt was made (no retry for non-retryable error)
    let state = executor.get_state();
    assert_eq!( state.attempt, 1 );
    assert_eq!( state.total_attempts, 1 );
    assert_eq!( mock_client.get_call_count(), 1 );
  }

  #[ tokio::test ]
  async fn test_retry_executor_max_elapsed_time()
  {
    let config = EnhancedRetryConfig::new()
      .with_max_attempts( 10 )
      .with_base_delay( 200 )  // Longer delay to ensure timeout
      .with_max_elapsed_time( 250 ) // Short max elapsed time that should trigger first
      .with_jitter( 0 );
    let executor = EnhancedRetryExecutor::new( config ).unwrap();

    // Mock client that always fails with many failures to ensure timeout
    let mock_client = MockHttpClient::new( vec![
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
    ] );

    let start_time = Instant::now();
    let result = executor.execute( || mock_client.make_request() ).await;
    let elapsed = start_time.elapsed();

    assert!( result.is_err() );

    // Should fail due to max elapsed time, not max attempts
    let error_msg = result.unwrap_err().to_string();
    assert!( error_msg.contains( "Max elapsed time exceeded" ), "Expected timeout error, got : {error_msg}" );

    // Should have taken at least the max elapsed time
    assert!( elapsed >= Duration::from_millis( 250 ) );

    // Should have made fewer attempts than max_attempts due to time limit
    let state = executor.get_state();
    assert!( state.attempt < 10 );
  }
}
