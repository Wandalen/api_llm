//! Circuit Breaker Configuration Tests
//!
//! Tests for circuit breaker configuration, validation, error classification, and state management.

#[ cfg( feature = "circuit_breaker" ) ]
mod config_tests
{
  use crate::inc::circuit_breaker_test_support::*;
  use api_openai::error::OpenAIError;

  #[ tokio::test ]
  async fn test_circuit_breaker_config_default_values()
  {
    let config = EnhancedCircuitBreakerConfig::default();

    assert_eq!( config.failure_threshold, 5 );
    assert_eq!( config.recovery_timeout_ms, 30000 );
    assert_eq!( config.success_threshold, 3 );
    assert_eq!( config.half_open_max_requests, 5 );
    assert_eq!( config.half_open_timeout_ms, 10000 );
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_config_builder_pattern()
  {
    let config = EnhancedCircuitBreakerConfig::new()
      .with_failure_threshold( 3 )
      .with_recovery_timeout( 15000 )
      .with_success_threshold( 2 )
      .with_half_open_max_requests( 3 )
      .with_half_open_timeout( 5000 );

    assert_eq!( config.failure_threshold, 3 );
    assert_eq!( config.recovery_timeout_ms, 15000 );
    assert_eq!( config.success_threshold, 2 );
    assert_eq!( config.half_open_max_requests, 3 );
    assert_eq!( config.half_open_timeout_ms, 5000 );
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_config_validation()
  {
    // Valid configuration
    let valid_config = EnhancedCircuitBreakerConfig::default();
    assert!( valid_config.validate().is_ok() );

    // Invalid : failure_threshold = 0
    let invalid_config = EnhancedCircuitBreakerConfig::default().with_failure_threshold( 0 );
    assert!( invalid_config.validate().is_err() );

    // Invalid : recovery_timeout_ms = 0
    let invalid_config = EnhancedCircuitBreakerConfig::default().with_recovery_timeout( 0 );
    assert!( invalid_config.validate().is_err() );

    // Invalid : success_threshold = 0
    let invalid_config = EnhancedCircuitBreakerConfig::default().with_success_threshold( 0 );
    assert!( invalid_config.validate().is_err() );

    // Invalid : half_open_max_requests = 0
    let invalid_config = EnhancedCircuitBreakerConfig::default().with_half_open_max_requests( 0 );
    assert!( invalid_config.validate().is_err() );

    // Invalid : half_open_timeout_ms = 0
    let invalid_config = EnhancedCircuitBreakerConfig::default().with_half_open_timeout( 0 );
    assert!( invalid_config.validate().is_err() );
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_error_classification()
  {
    let config = EnhancedCircuitBreakerConfig::default();

    // Errors that should trigger circuit breaker
    let network_error = OpenAIError::Network( "Connection failed".to_string() );
    assert!( config.is_circuit_breaker_error( &network_error ) );

    let timeout_error = OpenAIError::Timeout( "Request timeout".to_string() );
    assert!( config.is_circuit_breaker_error( &timeout_error ) );

    let server_error = OpenAIError::Http( "HTTP error with status 500: Internal Server Error".to_string() );
    assert!( config.is_circuit_breaker_error( &server_error ) );

    // Errors that should NOT trigger circuit breaker
    let rate_limit_error = OpenAIError::RateLimit( "Rate limit exceeded".to_string() );
    assert!( !config.is_circuit_breaker_error( &rate_limit_error ) );

    let client_error = OpenAIError::Http( "HTTP error with status 400: Bad Request".to_string() );
    assert!( !config.is_circuit_breaker_error( &client_error ) );

    let invalid_arg_error = OpenAIError::InvalidArgument( "Invalid argument".to_string() );
    assert!( !config.is_circuit_breaker_error( &invalid_arg_error ) );
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_state_management()
  {
    let mut state = CircuitBreakerStateManager::new();

    // Initial state
    assert_eq!( state.state, CircuitBreakerState::Closed );
    assert_eq!( state.failure_count, 0 );
    assert_eq!( state.success_count, 0 );
    assert!( state.should_allow_request() );

    // Record failures
    state.record_failure();
    assert_eq!( state.failure_count, 1 );
    assert_eq!( state.total_failures, 1 );
    assert_eq!( state.total_requests, 1 );

    // Record success (should reset failure count)
    state.record_success();
    assert_eq!( state.failure_count, 0 );
    assert_eq!( state.total_requests, 2 );

    // Test state transitions
    state.open();
    assert_eq!( state.state, CircuitBreakerState::Open );
    assert!( state.opened_at.is_some() );
    assert!( !state.should_allow_request() );
    assert_eq!( state.trip_count, 1 );

    state.half_open();
    assert_eq!( state.state, CircuitBreakerState::HalfOpen );
    assert!( state.half_open_at.is_some() );
    assert!( state.should_allow_request() );

    state.close();
    assert_eq!( state.state, CircuitBreakerState::Closed );
    assert!( state.opened_at.is_none() );
    assert!( state.half_open_at.is_none() );
  }
}
