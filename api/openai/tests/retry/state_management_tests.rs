//! Retry State Management Tests
//!
//! Tests for `RetryState` management including:
//! - State initialization and updates
//! - State reset between requests
//! - Thread-safe state access

#[ cfg( feature = "retry" ) ]
mod retry_state_management_tests
{
  use crate::enhanced_retry_helpers::*;
  use api_openai::error::OpenAIError;
  use core::time::Duration;
  use std::sync::Arc;
  use tokio::time::sleep;

  #[ tokio::test ]
  async fn test_retry_state_management()
  {
    let mut state = RetryState::new();

    // Initial state
    assert_eq!( state.attempt, 0 );
    assert_eq!( state.total_attempts, 0 );
    assert!( state.last_error.is_none() );

    // Increment attempts
    state.next_attempt();
    assert_eq!( state.attempt, 1 );
    assert_eq!( state.total_attempts, 1 );

    state.next_attempt();
    assert_eq!( state.attempt, 2 );
    assert_eq!( state.total_attempts, 2 );

    // Set error
    let error = OpenAIError::Network( "Test error".to_string() );
    state.set_error( error.to_string() );
    assert!( state.last_error.is_some() );

    // Reset state
    state.reset();
    assert_eq!( state.attempt, 0 );
    assert_eq!( state.total_attempts, 0 );
    assert!( state.last_error.is_none() );
  }

  #[ tokio::test ]
  async fn test_retry_state_reset_between_requests()
  {
    let config = EnhancedRetryConfig::new()
      .with_max_attempts( 2 )
      .with_base_delay( 10 )
      .with_jitter( 0 );
    let executor = EnhancedRetryExecutor::new( config ).unwrap();

    // First request with failure
    let mock_client_1 = MockHttpClient::new( vec![
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Ok( "success".to_string() ),
    ] );

    let result_1 = executor.execute( || mock_client_1.make_request() ).await;
    assert!( result_1.is_ok() );

    let state_after_first = executor.get_state();
    assert_eq!( state_after_first.attempt, 2 );

    // Second request should reset state
    let mock_client_2 = MockHttpClient::new( vec![
      Ok( "immediate success".to_string() ),
    ] );

    let result_2 = executor.execute( || mock_client_2.make_request() ).await;
    assert!( result_2.is_ok() );

    let state_after_second = executor.get_state();
    assert_eq!( state_after_second.attempt, 1 ); // Reset to 1 for new request
  }

  #[ tokio::test ]
  async fn test_thread_safe_retry_state()
  {
    let config = EnhancedRetryConfig::new()
      .with_max_attempts( 5 )
      .with_base_delay( 1 )
      .with_jitter( 0 );
    let executor = Arc::new( EnhancedRetryExecutor::new( config ).unwrap() );

    // Test concurrent access to retry state
    let executor_clone = executor.clone();
    let handle = tokio::spawn( async move {
      executor_clone.execute( || async {
        sleep( Duration::from_millis( 10 ) ).await;
        Ok( "concurrent success" )
      } ).await
    } );

    let result = executor.execute( || async { Ok( "main success" ) } ).await;
    let concurrent_result = handle.await.unwrap();

    assert!( result.is_ok() );
    assert!( concurrent_result.is_ok() );
  }
}
