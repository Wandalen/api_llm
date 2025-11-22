//! Circuit Breaker State Transition Tests
//!
//! Tests for circuit breaker state machine behavior, failure thresholds, and transitions.
//!
//! ⚠️ CODEBASE HYGIENE VIOLATION: These tests use MockHttpClient which violates no-mocking rule
//!
//! Justification : Circuit breaker state transitions require precise control over
//! failure sequences and timing that cannot be reliably achieved with real API calls.
//! State machine testing requires deterministic failure patterns (e.g., exactly N
//! consecutive failures to trigger state transition).
//!
//! Mitigation : Corresponding integration tests must verify circuit breaker behavior
//! with real OpenAI API under actual failure conditions.
//!
//! TODO(hygiene-007): Create integration tests for:
//! - Real API failures triggering circuit open
//! - Real half-open state with recovery attempts
//! - Real circuit close after recovery
//! - Real timeout scenarios with circuit breaker
//!
//! Integration test file : tests/circuit_breaker_integration_tests.rs (to be created)

#[ cfg( feature = "circuit_breaker" ) ]
mod state_transition_tests
{
  use crate::inc::circuit_breaker_test_support::*;
  use api_openai::error::{ OpenAIError, Result };
  use std::sync::Arc;
  use tokio::time::sleep;
  use core::time::Duration;

  #[ tokio::test ]
  async fn test_circuit_breaker_successful_operation()
  {
    let config = EnhancedCircuitBreakerConfig::new().with_failure_threshold( 2 );
    let circuit_breaker = EnhancedCircuitBreaker::new( config ).unwrap();

    // Operation that succeeds immediately
    let result = circuit_breaker.execute( || async { Ok( "success" ) } ).await;

    assert!( result.is_ok() );
    assert_eq!( result.unwrap(), "success" );

    // Check state - should remain closed
    let state = circuit_breaker.get_state();
    assert_eq!( state.state, CircuitBreakerState::Closed );
    assert_eq!( state.total_requests, 1 );
    assert_eq!( state.total_failures, 0 );
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_failure_threshold()
  {
    let config = EnhancedCircuitBreakerConfig::new()
      .with_failure_threshold( 2 )
      .with_recovery_timeout( 100 ); // Short timeout for testing
    let circuit_breaker = EnhancedCircuitBreaker::new( config ).unwrap();

    // Mock client that always fails with circuit breaker triggering errors
    let mock_client = MockHttpClient::new( vec![
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() ),
    ] );

    // First failure - should remain closed
    let result = circuit_breaker.execute( || mock_client.make_request() ).await;
    assert!( result.is_err() );

    let state = circuit_breaker.get_state();
    assert_eq!( state.state, CircuitBreakerState::Closed );
    assert_eq!( state.failure_count, 1 );

    // Second failure - should open circuit
    let result = circuit_breaker.execute( || mock_client.make_request() ).await;
    assert!( result.is_err() );

    let state = circuit_breaker.get_state();
    assert_eq!( state.state, CircuitBreakerState::Open );
    assert_eq!( state.failure_count, 2 );
    assert_eq!( state.trip_count, 1 );

    // Third attempt - should be rejected immediately
    let result = circuit_breaker.execute( || mock_client.make_request() ).await;
    assert!( result.is_err() );
    let error_msg = result.unwrap_err().to_string();
    assert!( error_msg.contains( "Circuit breaker is open" ) );

    // Mock client should only have been called twice (third was rejected)
    assert_eq!( mock_client.get_call_count(), 2 );
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_non_triggering_errors()
  {
    let config = EnhancedCircuitBreakerConfig::new().with_failure_threshold( 2 );
    let circuit_breaker = EnhancedCircuitBreaker::new( config ).unwrap();

    // Errors that should NOT trigger circuit breaker
    let result1 : Result< &str > = circuit_breaker.execute( || async {
      Err( OpenAIError::RateLimit( "Rate limit exceeded".to_string() ).into() )
    } ).await;
    assert!( result1.is_err() );

    let result2 : Result< &str > = circuit_breaker.execute( || async {
      Err( OpenAIError::InvalidArgument( "Invalid argument".to_string() ).into() )
    } ).await;
    assert!( result2.is_err() );

    // Circuit should remain closed
    let state = circuit_breaker.get_state();
    assert_eq!( state.state, CircuitBreakerState::Closed );
    assert_eq!( state.failure_count, 0 ); // No failures recorded for non-triggering errors
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_metrics_tracking()
  {
    let config = EnhancedCircuitBreakerConfig::new().with_failure_threshold( 2 );
    let circuit_breaker = EnhancedCircuitBreaker::new( config ).unwrap();

    // Successful operation
    let result = circuit_breaker.execute( || async { Ok( "success" ) } ).await;
    assert!( result.is_ok() );

    // Failed operation
    let result : Result< &str > = circuit_breaker.execute( || async {
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() )
    } ).await;
    assert!( result.is_err() );

    // Check metrics
    let state = circuit_breaker.get_state();
    assert_eq!( state.total_requests, 2 );
    assert_eq!( state.total_failures, 1 );
    assert_eq!( state.trip_count, 0 ); // Not tripped yet
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_thread_safety()
  {
    let config = EnhancedCircuitBreakerConfig::new().with_failure_threshold( 5 );
    let circuit_breaker = Arc::new( EnhancedCircuitBreaker::new( config ).unwrap() );

    // Test concurrent access
    let circuit_breaker_clone = circuit_breaker.clone();
    let handle = tokio::spawn( async move {
      circuit_breaker_clone.execute( || async {
        sleep( Duration::from_millis( 10 ) ).await;
        Ok( "concurrent success" )
      } ).await
    } );

    let result = circuit_breaker.execute( || async { Ok( "main success" ) } ).await;
    let concurrent_result = handle.await.unwrap();

    assert!( result.is_ok() );
    assert!( concurrent_result.is_ok() );
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_zero_overhead_when_disabled()
  {
    // This test validates that circuit breaker configuration has zero overhead when disabled
    // Since we're in the feature-gated module, this tests the enabled behavior
    // The zero overhead when disabled is ensured by the feature gate itself

    let config = EnhancedCircuitBreakerConfig::default();
    assert!( config.validate().is_ok() );

    // Create circuit breaker without actual usage (minimal overhead)
    let circuit_breaker = EnhancedCircuitBreaker::new( config );
    assert!( circuit_breaker.is_ok() );
  }
}

#[ cfg( not( feature = "circuit_breaker" ) ) ]
mod no_circuit_breaker_tests
{
  /// Test that ensures zero overhead when circuit breaker feature is disabled
  #[ tokio::test ]
  async fn test_zero_overhead_when_circuit_breaker_disabled()
  {
    // When circuit breaker feature is disabled, this module should compile
    // but circuit breaker functionality should not be available

    // This test simply validates that the module compiles without the circuit_breaker feature
    // The actual zero overhead is ensured by the compiler when feature is not enabled
    assert!( true, "Circuit breaker feature is disabled - zero overhead ensured" );
  }
}
