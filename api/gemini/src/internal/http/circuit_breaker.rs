//! Circuit breaker implementation for HTTP reliability

use std::sync::{ Arc, Mutex };
use std::time::{ Duration, Instant };
use reqwest::{ Client, Method };
use serde::Serialize;
use serde::Deserialize;

use crate::error::Error;

#[ cfg( feature = "logging" ) ]
use tracing::{ warn, debug, info };

/// Circuit breaker configuration extracted from client for HTTP layer usage
#[ derive( Debug, Clone ) ]
pub struct CircuitBreakerConfig
{
  /// Maximum number of consecutive failures before opening the circuit
  pub failure_threshold : u32,
  /// Time to wait before transitioning from open to half-open
  pub timeout : Duration,
  /// Number of successful requests needed to close the circuit from half-open
  pub success_threshold : u32,
  /// Whether to collect metrics
  pub enable_metrics : bool,
}

/// Circuit breaker state
#[ derive( Debug, Clone, PartialEq ) ]
pub enum CircuitBreakerState
{
  /// Circuit is closed - requests pass through normally
  Closed,
  /// Circuit is open - requests fail immediately until timeout
  Open( Instant ),
  /// Circuit is half-open - testing if service has recovered
  HalfOpen,
}

/// Circuit breaker metrics for monitoring
#[ derive( Debug, Clone ) ]
pub struct CircuitBreakerMetrics
{
  /// Total number of requests processed
  pub total_requests : u64,
  /// Number of requests that failed
  pub failed_requests : u64,
  /// Number of requests blocked by open circuit
  pub blocked_requests : u64,
  /// Number of state transitions
  pub state_transitions : u64,
  /// Current circuit state
  pub current_state : CircuitBreakerState,
  /// Time circuit was last opened
  pub last_opened : Option< Instant >,
}

/// Circuit breaker instance with state management
#[ derive( Debug ) ]
pub struct CircuitBreaker
{
  config : CircuitBreakerConfig,
  state : Arc< Mutex< CircuitBreakerState > >,
  consecutive_failures : Arc< Mutex< u32 > >,
  consecutive_successes : Arc< Mutex< u32 > >,
  metrics : Arc< Mutex< CircuitBreakerMetrics > >,
}

impl CircuitBreaker
{
  /// Create a new circuit breaker with the given configuration
  pub fn new( config : CircuitBreakerConfig ) -> Self
  {
    Self {
      config,
      state : Arc::new( Mutex::new( CircuitBreakerState::Closed ) ),
      consecutive_failures : Arc::new( Mutex::new( 0 ) ),
      consecutive_successes : Arc::new( Mutex::new( 0 ) ),
      metrics : Arc::new( Mutex::new( CircuitBreakerMetrics {
        total_requests : 0,
        failed_requests : 0,
        blocked_requests : 0,
        state_transitions : 0,
        current_state : CircuitBreakerState::Closed,
        last_opened : None,
      } ) ),
    }
  }

  /// Check if a request should be allowed through the circuit breaker
  pub fn should_allow_request( &self ) -> bool
  {
    let mut state = self.state.lock().unwrap();
    let mut metrics = self.metrics.lock().unwrap();

    metrics.total_requests += 1;

    match *state
    {
      CircuitBreakerState::Closed => true,
      CircuitBreakerState::Open( opened_at ) => {
        if opened_at.elapsed() >= self.config.timeout
        {
          // Transition to half-open
          *state = CircuitBreakerState::HalfOpen;
          metrics.current_state = CircuitBreakerState::HalfOpen;
          metrics.state_transitions += 1;

          #[ cfg( feature = "logging" ) ]
          info!( "Circuit breaker transitioning to half-open state" );

          true // Allow the test request
        } else {
          metrics.blocked_requests += 1;

          #[ cfg( feature = "logging" ) ]
          debug!( "Circuit breaker is open, blocking request" );

          false
        }
      },
      CircuitBreakerState::HalfOpen => {
        // In half-open state, allow requests but they will be closely monitored
        true
      }
    }
  }

  /// Record a successful request
  pub fn record_success( &self )
  {
    let mut state = self.state.lock().unwrap();
    let mut consecutive_failures = self.consecutive_failures.lock().unwrap();
    let mut consecutive_successes = self.consecutive_successes.lock().unwrap();
    let mut metrics = self.metrics.lock().unwrap();

    *consecutive_failures = 0;
    *consecutive_successes += 1;

    match *state
    {
      CircuitBreakerState::HalfOpen => {
        if *consecutive_successes >= self.config.success_threshold
        {
          // Close the circuit
          *state = CircuitBreakerState::Closed;
          *consecutive_successes = 0;
          metrics.current_state = CircuitBreakerState::Closed;
          metrics.state_transitions += 1;

          #[ cfg( feature = "logging" ) ]
          info!( "Circuit breaker closed after successful recovery" );
        }
      },
      _ => {
        // Reset success counter in closed state
        if matches!( *state, CircuitBreakerState::Closed )
        {
          *consecutive_successes = 0;
        }
      }
    }
  }

  /// Record a failed request
  pub fn record_failure( &self )
  {
    let mut state = self.state.lock().unwrap();
    let mut consecutive_failures = self.consecutive_failures.lock().unwrap();
    let mut consecutive_successes = self.consecutive_successes.lock().unwrap();
    let mut metrics = self.metrics.lock().unwrap();

    *consecutive_failures += 1;
    *consecutive_successes = 0;
    metrics.failed_requests += 1;

    // Check if we should open the circuit
    if *consecutive_failures >= self.config.failure_threshold
    {
      match *state
      {
        CircuitBreakerState::Closed | CircuitBreakerState::HalfOpen => {
          let now = Instant::now();
          *state = CircuitBreakerState::Open( now );
          metrics.current_state = CircuitBreakerState::Open( now );
          metrics.last_opened = Some( now );
          metrics.state_transitions += 1;

          #[ cfg( feature = "logging" ) ]
          warn!(
            "Circuit breaker opened after {} consecutive failures",
            *consecutive_failures
          );
        },
        _ => {}
      }
    }
  }

  /// Get current circuit breaker metrics
  pub fn get_metrics( &self ) -> CircuitBreakerMetrics
  {
    self.metrics.lock().unwrap().clone()
  }
}

/// Classify if an error should trigger circuit breaker failure counting
pub fn is_circuit_breaker_error( error : &Error ) -> bool
{
  matches!( error,
    Error::NetworkError( _ ) |
    Error::ServerError( _ ) |
    Error::RateLimitError( _ ) |
    Error::TimeoutError( _ )
  )
}

/// Execute an HTTP request with circuit breaker protection
pub async fn execute_with_circuit_breaker< T, R >
(
  client : &Client,
  method : Method,
  url : &str,
  api_key : &str,
  body : Option< &T >,
  config : &super::HttpConfig,
  circuit_breaker : Option< &CircuitBreaker >,
)
-> Result< R, Error >
where
  T: Serialize,
  R: for< 'de > Deserialize< 'de >,
{
  let Some( cb ) = circuit_breaker else {
    // No circuit breaker configured, use normal execution
    return super::execute( client, method, url, api_key, body, config ).await;
  };

  // Check if request should be allowed
  if !cb.should_allow_request()
  {
    return Err( Error::CircuitBreakerOpen(
      "Circuit breaker is open".to_string()
    ) );
  }

  // Execute the request
  match super::execute( client, method, url, api_key, body, config ).await
  {
    Ok( response ) => {
      cb.record_success();
      Ok( response )
    },
    Err( error ) => {
      // Only count errors that should trigger circuit breaker
      if is_circuit_breaker_error( &error )
      {
        cb.record_failure();
      }
      Err( error )
    }
  }
}
