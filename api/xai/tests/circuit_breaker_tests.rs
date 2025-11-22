//! Unit tests for circuit breaker implementation.
//!
//! # Purpose
//!
//! Validates the three-state circuit breaker pattern for preventing
//! cascading failures in distributed systems.
//!
//! # Key Insights
//!
//! - **State Transitions**:
//!   ```text
//!   Closed --[failures >= threshold]--> Open
//!   Open --[timeout elapsed]--> HalfOpen
//!   HalfOpen --[success]--> Closed
//!   HalfOpen --[failure]--> Open (immediate)
//!   ```
//!
//! - **Design Decision**: Circuit opens AFTER threshold failures (not at threshold).
//!   Example : threshold=3 means circuit opens on the 3rd consecutive failure.
//!
//! - **`HalfOpen` Behavior**: Any single failure in `HalfOpen` immediately reopens
//!   the circuit. This prevents flapping and ensures service has truly recovered.
//!
//! - **Success Reset**: In Closed state, any success resets failure counter to zero.
//!   This prevents old failures from accumulating across long time periods.
//!
//! - **Timeout Transition**: Circuit transitions from `Open` to `HalfOpen` only when
//!   `is_request_allowed()` is called AND timeout has elapsed. It's lazy, not automatic.
//!
//! # Running Tests
//!
//! ```bash
//! cargo test --features circuit_breaker --test circuit_breaker_tests
//! ```

#![ cfg( feature = "circuit_breaker" ) ]

use api_xai::{ CircuitBreaker, CircuitBreakerConfig, CircuitState };
use core::time::Duration;

#[ test ]
fn circuit_breaker_starts_closed()
{
  let breaker = CircuitBreaker::default();
  assert_eq!( breaker.state(), CircuitState::Closed );
  assert!( breaker.is_request_allowed() );
}

#[ test ]
fn circuit_breaker_opens_after_threshold_failures()
{
  let config = CircuitBreakerConfig::default()
    .with_failure_threshold( 3 );

  let breaker = CircuitBreaker::new( config );

  // Record failures
  breaker.record_failure();
  assert_eq!( breaker.state(), CircuitState::Closed, "Should stay closed after 1 failure" );

  breaker.record_failure();
  assert_eq!( breaker.state(), CircuitState::Closed, "Should stay closed after 2 failures" );

  breaker.record_failure();
  assert_eq!( breaker.state(), CircuitState::Open, "Should open after 3 failures" );

  // Requests should be blocked
  assert!( !breaker.is_request_allowed(), "Requests should be blocked when open" );
}

#[ test ]
fn circuit_breaker_transitions_to_half_open_after_timeout()
{
  let config = CircuitBreakerConfig::default()
    .with_failure_threshold( 2 )
    .with_timeout( Duration::from_millis( 100 ) );

  let breaker = CircuitBreaker::new( config );

  // Open the circuit
  breaker.record_failure();
  breaker.record_failure();
  assert_eq!( breaker.state(), CircuitState::Open );

  // Wait for timeout
  std::thread::sleep( Duration::from_millis( 150 ) );

  // Check if request is allowed (should transition to HalfOpen)
  assert!( breaker.is_request_allowed(), "Should allow request after timeout" );
  assert_eq!( breaker.state(), CircuitState::HalfOpen, "Should be in HalfOpen state" );
}

#[ test ]
fn circuit_breaker_closes_after_success_threshold_in_half_open()
{
  let config = CircuitBreakerConfig::default()
    .with_failure_threshold( 2 )
    .with_success_threshold( 2 )
    .with_timeout( Duration::from_millis( 100 ) );

  let breaker = CircuitBreaker::new( config );

  // Open the circuit
  breaker.record_failure();
  breaker.record_failure();
  assert_eq!( breaker.state(), CircuitState::Open );

  // Wait for timeout and transition to HalfOpen
  std::thread::sleep( Duration::from_millis( 150 ) );
  assert!( breaker.is_request_allowed() );
  assert_eq!( breaker.state(), CircuitState::HalfOpen );

  // Record successes
  breaker.record_success();
  assert_eq!( breaker.state(), CircuitState::HalfOpen, "Should stay HalfOpen after 1 success" );

  breaker.record_success();
  assert_eq!( breaker.state(), CircuitState::Closed, "Should close after 2 successes" );
}

#[ test ]
fn circuit_breaker_reopens_on_failure_in_half_open()
{
  let config = CircuitBreakerConfig::default()
    .with_failure_threshold( 2 )
    .with_timeout( Duration::from_millis( 100 ) );

  let breaker = CircuitBreaker::new( config );

  // Open the circuit
  breaker.record_failure();
  breaker.record_failure();
  assert_eq!( breaker.state(), CircuitState::Open );

  // Wait for timeout and transition to HalfOpen
  std::thread::sleep( Duration::from_millis( 150 ) );
  assert!( breaker.is_request_allowed() );
  assert_eq!( breaker.state(), CircuitState::HalfOpen );

  // Record failure in HalfOpen
  breaker.record_failure();
  assert_eq!( breaker.state(), CircuitState::Open, "Should reopen on failure in HalfOpen" );
}

#[ test ]
fn circuit_breaker_reset_works()
{
  let config = CircuitBreakerConfig::default()
    .with_failure_threshold( 2 );

  let breaker = CircuitBreaker::new( config );

  // Open the circuit
  breaker.record_failure();
  breaker.record_failure();
  assert_eq!( breaker.state(), CircuitState::Open );

  // Reset
  breaker.reset();
  assert_eq!( breaker.state(), CircuitState::Closed, "Should be closed after reset" );
  assert!( breaker.is_request_allowed(), "Should allow requests after reset" );
}

#[ test ]
fn circuit_breaker_success_resets_failure_count_in_closed()
{
  let config = CircuitBreakerConfig::default()
    .with_failure_threshold( 3 );

  let breaker = CircuitBreaker::new( config );

  // Record some failures
  breaker.record_failure();
  breaker.record_failure();
  assert_eq!( breaker.state(), CircuitState::Closed );

  // Record success (should reset failure count)
  breaker.record_success();

  // Now need 3 more failures to open
  breaker.record_failure();
  breaker.record_failure();
  assert_eq!( breaker.state(), CircuitState::Closed, "Should still be closed" );

  breaker.record_failure();
  assert_eq!( breaker.state(), CircuitState::Open, "Should open after 3 new failures" );
}

#[ tokio::test ]
async fn circuit_breaker_call_with_success()
{
  use api_xai::Result;

  let breaker = CircuitBreaker::default();

  let result = breaker.call( || async {
    Ok::< i32, _ >( 42 ) as Result< i32 >
  } ).await;

  assert!( result.is_ok() );
  assert_eq!( result.unwrap(), 42 );
  assert_eq!( breaker.state(), CircuitState::Closed );
}

#[ tokio::test ]
async fn circuit_breaker_call_with_failure()
{
  use api_xai::{ XaiError, Result };

  let config = CircuitBreakerConfig::default()
    .with_failure_threshold( 2 );

  let breaker = CircuitBreaker::new( config );

  // First failure
  let result = breaker.call( || async {
    Err::< i32, _ >( XaiError::Http( "test error".to_string() ).into() ) as Result< i32 >
  } ).await;

  assert!( result.is_err() );
  assert_eq!( breaker.state(), CircuitState::Closed );

  // Second failure (should open circuit)
  let result = breaker.call( || async {
    Err::< i32, _ >( XaiError::Http( "test error".to_string() ).into() ) as Result< i32 >
  } ).await;

  assert!( result.is_err() );
  assert_eq!( breaker.state(), CircuitState::Open );

  // Third call should be rejected immediately
  let result = breaker.call( || async {
    Ok::< i32, _ >( 42 ) as Result< i32 >
  } ).await;

  assert!( result.is_err() );
  if let Err( e ) = result {
    if let Some( xai_err ) = e.downcast_ref::< XaiError >() {
      assert!(
        matches!( xai_err, XaiError::CircuitBreakerOpen( _ ) ),
        "Should be CircuitBreakerOpen error"
      );
    } else {
      panic!( "Expected XaiError" );
    }
  }
}

#[ test ]
fn circuit_breaker_config_builder_works()
{
  let config = CircuitBreakerConfig::default()
    .with_failure_threshold( 10 )
    .with_timeout( Duration::from_secs( 60 ) )
    .with_success_threshold( 3 );

  assert_eq!( config.failure_threshold, 10 );
  assert_eq!( config.timeout, Duration::from_secs( 60 ) );
  assert_eq!( config.success_threshold, 3 );
}
