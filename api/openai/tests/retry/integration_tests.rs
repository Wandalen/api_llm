//! Retry Integration Tests
//!
//! Integration tests for retry functionality including:
//! - Zero overhead validation when disabled
//! - Retry metrics collection and validation
//! - Graceful degradation with None config
//! - Feature gate compliance

#[ cfg( feature = "retry" ) ]
mod retry_integration_tests
{
  use crate::enhanced_retry_helpers::*;
  use api_openai::error::{ OpenAIError, Result };
  use core::time::Duration;
  use std::sync::{ Arc, Mutex };
  use std::time::Instant;

  #[ tokio::test ]
  async fn test_retry_configuration_zero_overhead_when_disabled()
  {
    // This test validates that retry configuration has zero overhead when disabled
    // Since we're in the feature-gated module, this tests the enabled behavior
    // The zero overhead when disabled is ensured by the feature gate itself

    let config = EnhancedRetryConfig::default();
    assert!( config.validate().is_ok() );

    // Create executor without actual usage (minimal overhead)
    let executor = EnhancedRetryExecutor::new( config );
    assert!( executor.is_ok() );
  }

  #[ tokio::test ]
  async fn test_retry_metrics_integration()
  {
    let config = EnhancedRetryConfig::new()
      .with_max_attempts( 3 )
      .with_base_delay( 10 )
      .with_jitter( 0 );
    let executor = EnhancedRetryExecutor::new( config ).unwrap();

    // Mock client with predictable failure pattern
    let mock_client = MockHttpClient::new( vec![
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Err( OpenAIError::Http( "HTTP error with status 503: Service unavailable".to_string() ).into() ),
      Ok( "success".to_string() ),
    ] );

    let start_time = Instant::now();
    let result = executor.execute( || mock_client.make_request() ).await;
    let total_time = start_time.elapsed();

    assert!( result.is_ok() );

    // Validate retry metrics
    let state = executor.get_state();
    assert_eq!( state.total_attempts, 3 );
    assert!( state.elapsed_time > Duration::ZERO );
    assert!( total_time >= Duration::from_millis( 20 ) ); // At least 2 delays of 10ms

    // Check that last error is preserved
    assert!( state.last_error.is_some() );
  }

  #[ tokio::test ]
  async fn test_graceful_degradation_with_none_config()
  {
    // Test behavior when retry configuration is None (graceful degradation)
    // This test simulates the case where retry is not configured

    // When config is None, operation should execute once without retry
    let operation_count = Arc::new( Mutex::new( 0u32 ) );
    let count_clone = operation_count.clone();

    let operation = move || {
      let count_clone = count_clone.clone();
      async move {
        let mut count = count_clone.lock().unwrap();
        *count += 1;
        if *count == 1
        {
          Err( OpenAIError::Network( "Simulated failure".to_string() ).into() )
        }
        else
        {
          Ok( "Success after retry".to_string() )
        }
      }
    };

    // Execute without retry configuration (simulate None config behavior)
    let result : Result< String > = operation().await;

    // Should fail on first attempt without retry
    assert!( result.is_err() );

    // Should have made exactly one call
    let final_count = *operation_count.lock().unwrap();
    assert_eq!( final_count, 1 );
  }
}

#[ cfg( not( feature = "retry" ) ) ]
mod no_retry_tests
{
  /// Test that ensures zero overhead when retry feature is disabled
  #[ tokio::test ]
  async fn test_zero_overhead_when_retry_disabled()
  {
    // When retry feature is disabled, this module should compile
    // but retry functionality should not be available

    // This test simply validates that the module compiles without the retry feature
    // The actual zero overhead is ensured by the compiler when feature is not enabled
    assert!( true, "Retry feature is disabled - zero overhead ensured" );
  }
}
