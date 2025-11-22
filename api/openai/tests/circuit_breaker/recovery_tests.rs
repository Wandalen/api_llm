//! Circuit Breaker Recovery Tests
//!
//! Tests for circuit breaker recovery behavior, half-open state, and recovery timeouts.

#[ cfg( feature = "circuit_breaker" ) ]
mod recovery_tests
{
  use crate::inc::circuit_breaker_test_support::*;
  use api_openai::error::{ OpenAIError, Result };
  use tokio::time::sleep;
  use core::time::Duration;

  #[ tokio::test ]
  async fn test_circuit_breaker_recovery_timeout()
  {
    let config = EnhancedCircuitBreakerConfig::new()
      .with_failure_threshold( 1 )
      .with_recovery_timeout( 50 ) // Very short timeout for testing
      .with_success_threshold( 1 );
    let circuit_breaker = EnhancedCircuitBreaker::new( config ).unwrap();

    // Trigger circuit breaker to open
    let result : Result< &str > = circuit_breaker.execute( || async {
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() )
    } ).await;
    assert!( result.is_err() );

    let state = circuit_breaker.get_state();
    assert_eq!( state.state, CircuitBreakerState::Open );

    // Wait for recovery timeout
    sleep( Duration::from_millis( 60 ) ).await;

    // Next request should transition to half-open and succeed
    let result = circuit_breaker.execute( || async { Ok( "success" ) } ).await;
    assert!( result.is_ok() );

    // Should now be closed due to successful request
    let state = circuit_breaker.get_state();
    assert_eq!( state.state, CircuitBreakerState::Closed );
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_half_open_state()
  {
    let config = EnhancedCircuitBreakerConfig::new()
      .with_failure_threshold( 1 )
      .with_recovery_timeout( 50 )
      .with_success_threshold( 2 ) // Need 2 successes to close
      .with_half_open_max_requests( 3 );
    let circuit_breaker = EnhancedCircuitBreaker::new( config ).unwrap();

    // Open circuit
    let result : Result< &str > = circuit_breaker.execute( || async {
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() )
    } ).await;
    assert!( result.is_err() );

    // Wait for recovery
    sleep( Duration::from_millis( 60 ) ).await;

    // First success in half-open
    let result = circuit_breaker.execute( || async { Ok( "success1" ) } ).await;
    assert!( result.is_ok() );

    let state = circuit_breaker.get_state();
    assert_eq!( state.state, CircuitBreakerState::HalfOpen );
    assert_eq!( state.success_count, 1 );

    // Second success - should close circuit
    let result = circuit_breaker.execute( || async { Ok( "success2" ) } ).await;
    assert!( result.is_ok() );

    let state = circuit_breaker.get_state();
    assert_eq!( state.state, CircuitBreakerState::Closed );
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_half_open_failure()
  {
    let config = EnhancedCircuitBreakerConfig::new()
      .with_failure_threshold( 1 )
      .with_recovery_timeout( 50 )
      .with_success_threshold( 2 );
    let circuit_breaker = EnhancedCircuitBreaker::new( config ).unwrap();

    // Open circuit
    let result : Result< &str > = circuit_breaker.execute( || async {
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() )
    } ).await;
    assert!( result.is_err() );

    // Wait for recovery
    sleep( Duration::from_millis( 60 ) ).await;

    // Success in half-open
    let result = circuit_breaker.execute( || async { Ok( "success" ) } ).await;
    assert!( result.is_ok() );

    let state = circuit_breaker.get_state();
    assert_eq!( state.state, CircuitBreakerState::HalfOpen );

    // Failure in half-open - should go back to open
    let result : Result< &str > = circuit_breaker.execute( || async {
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() )
    } ).await;
    assert!( result.is_err() );

    let state = circuit_breaker.get_state();
    assert_eq!( state.state, CircuitBreakerState::Open );
    assert_eq!( state.trip_count, 2 ); // Should increment trip count again
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_half_open_max_requests()
  {
    let config = EnhancedCircuitBreakerConfig::new()
      .with_failure_threshold( 1 )
      .with_recovery_timeout( 50 )
      .with_half_open_max_requests( 2 )
      .with_success_threshold( 5 ); // Keep circuit in half-open longer
    let circuit_breaker = EnhancedCircuitBreaker::new( config ).unwrap();

    // Open circuit
    let result : Result< &str > = circuit_breaker.execute( || async {
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() )
    } ).await;
    assert!( result.is_err() );

    // Wait for recovery
    sleep( Duration::from_millis( 60 ) ).await;

    // Two requests in half-open (should be allowed)
    let result1 = circuit_breaker.execute( || async { Ok( "success1" ) } ).await;
    assert!( result1.is_ok() );

    let result2 = circuit_breaker.execute( || async { Ok( "success2" ) } ).await;
    assert!( result2.is_ok() );

    // Third request should be rejected (exceeds max)
    let result3 = circuit_breaker.execute( || async { Ok( "success3" ) } ).await;
    assert!( result3.is_err() );
    let error_msg = result3.unwrap_err().to_string();
    assert!( error_msg.contains( "Circuit breaker is open" ) );
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_half_open_timeout()
  {
    let config = EnhancedCircuitBreakerConfig::new()
      .with_failure_threshold( 1 )
      .with_recovery_timeout( 50 )
      .with_half_open_timeout( 100 ) // Timeout for half-open state
      .with_half_open_max_requests( 10 );
    let circuit_breaker = EnhancedCircuitBreaker::new( config ).unwrap();

    // Open circuit
    let result : Result< &str > = circuit_breaker.execute( || async {
      Err( OpenAIError::Network( "Connection failed".to_string() ).into() )
    } ).await;
    assert!( result.is_err() );

    // Wait for recovery
    sleep( Duration::from_millis( 60 ) ).await;

    // One request in half-open
    let result = circuit_breaker.execute( || async { Ok( "success" ) } ).await;
    assert!( result.is_ok() );

    let state = circuit_breaker.get_state();
    assert_eq!( state.state, CircuitBreakerState::HalfOpen );

    // Wait for half-open timeout
    sleep( Duration::from_millis( 120 ) ).await;

    // Next request should be rejected (half-open timeout elapsed)
    let result = circuit_breaker.execute( || async { Ok( "success" ) } ).await;
    assert!( result.is_err() );
    let error_msg = result.unwrap_err().to_string();
    assert!( error_msg.contains( "Circuit breaker is open" ) );
  }
}
