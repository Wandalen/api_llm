//! Circuit breaker implementation for preventing cascading failures.

#[ cfg( feature = "circuit_breaker" ) ]
mod private
{
  use core::time::Duration;
  use std::sync::{ Arc, Mutex };
  use std::time::Instant;

  /// Circuit breaker states
  #[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
  pub enum CircuitBreakerState
  {
    /// Circuit is closed - requests are allowed through
    Closed,
    /// Circuit is open - requests are rejected immediately
    Open,
    /// Circuit is half-open - limited requests are allowed to test recovery
    HalfOpen,
  }

  /// Configuration for circuit breaker behavior
  #[ derive( Debug, Clone ) ]
  pub struct CircuitBreakerConfig
  {
    failure_threshold : u32,
    recovery_timeout : Duration,
    half_open_max_calls : u32,
  }

  /// Circuit breaker implementation for preventing cascading failures
  #[ derive( Debug ) ]
  pub struct CircuitBreaker
  {
    config : CircuitBreakerConfig,
    state : Arc< Mutex< CircuitBreakerInternalState > >,
  }

  /// Internal state for circuit breaker
  #[ derive( Debug ) ]
  struct CircuitBreakerInternalState
  {
    current_state : CircuitBreakerState,
    failure_count : u32,
    last_failure_time : Option< Instant >,
    half_open_calls : u32,
  }

  impl CircuitBreakerConfig
  {
    /// Create a new circuit breaker configuration
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        failure_threshold : 5,
        recovery_timeout : Duration::from_secs( 60 ),
        half_open_max_calls : 3,
      }
    }

    /// Set the failure threshold (number of consecutive failures before opening circuit)
    #[ inline ]
    #[ must_use ]
    pub fn with_failure_threshold( mut self, threshold : u32 ) -> Self
    {
      self.failure_threshold = threshold;
      self
    }

    /// Set the recovery timeout (how long to wait before transitioning to half-open)
    #[ inline ]
    #[ must_use ]
    pub fn with_recovery_timeout( mut self, timeout : Duration ) -> Self
    {
      self.recovery_timeout = timeout;
      self
    }

    /// Set the maximum number of calls allowed in half-open state
    #[ inline ]
    #[ must_use ]
    pub fn with_half_open_max_calls( mut self, max_calls : u32 ) -> Self
    {
      self.half_open_max_calls = max_calls;
      self
    }

    /// Get the failure threshold
    #[ inline ]
    #[ must_use ]
    pub fn failure_threshold( &self ) -> u32
    {
      self.failure_threshold
    }

    /// Get the recovery timeout
    #[ inline ]
    #[ must_use ]
    pub fn recovery_timeout( &self ) -> Duration
    {
      self.recovery_timeout
    }

    /// Get the maximum calls in half-open state
    #[ inline ]
    #[ must_use ]
    pub fn half_open_max_calls( &self ) -> u32
    {
      self.half_open_max_calls
    }
  }

  impl Default for CircuitBreakerConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  impl CircuitBreaker
  {
    /// Create a new circuit breaker with the given configuration
    #[ inline ]
    #[ must_use ]
    pub fn new( config : CircuitBreakerConfig ) -> Self
    {
      Self
      {
        config,
        state : Arc::new( Mutex::new( CircuitBreakerInternalState
        {
          current_state : CircuitBreakerState::Closed,
          failure_count : 0,
          last_failure_time : None,
          half_open_calls : 0,
        })),
      }
    }

    /// Check if the circuit breaker allows execution
    #[ inline ]
    #[ must_use ]
    pub fn can_execute( &self ) -> bool
    {
      let mut state = self.state.lock().unwrap();

      match state.current_state
      {
        CircuitBreakerState::Closed => true,
        CircuitBreakerState::Open =>
        {
          // Check if recovery timeout has elapsed
          if let Some( last_failure ) = state.last_failure_time
          {
            if last_failure.elapsed() >= self.config.recovery_timeout
            {
              // Transition to half-open
              state.current_state = CircuitBreakerState::HalfOpen;
              state.half_open_calls = 0;
              return true;
            }
          }
          false
        },
        CircuitBreakerState::HalfOpen =>
        {
          // Allow limited calls in half-open state
          state.half_open_calls < self.config.half_open_max_calls
        },
      }
    }

    /// Record a successful call
    #[ inline ]
    pub fn record_success( &self )
    {
      let mut state = self.state.lock().unwrap();

      match state.current_state
      {
        CircuitBreakerState::Closed =>
        {
          // Reset failure count on success
          state.failure_count = 0;
        },
        CircuitBreakerState::HalfOpen =>
        {
          state.half_open_calls += 1;
          // If we've completed enough successful calls, transition to closed
          if state.half_open_calls >= self.config.half_open_max_calls
          {
            state.current_state = CircuitBreakerState::Closed;
            state.failure_count = 0;
            state.last_failure_time = None;
            state.half_open_calls = 0;
          }
        },
        CircuitBreakerState::Open =>
        {
          // Success in open state should not happen, but reset if it does
          state.current_state = CircuitBreakerState::Closed;
          state.failure_count = 0;
          state.last_failure_time = None;
        },
      }
    }

    /// Record a failed call
    #[ inline ]
    pub fn record_failure( &self )
    {
      let mut state = self.state.lock().unwrap();

      match state.current_state
      {
        CircuitBreakerState::Closed =>
        {
          state.failure_count += 1;
          state.last_failure_time = Some( Instant::now() );

          // Check if we should open the circuit
          if state.failure_count >= self.config.failure_threshold
          {
            state.current_state = CircuitBreakerState::Open;
          }
        },
        CircuitBreakerState::HalfOpen =>
        {
          // Failure in half-open state - reopen the circuit
          state.current_state = CircuitBreakerState::Open;
          state.failure_count += 1;
          state.last_failure_time = Some( Instant::now() );
          state.half_open_calls = 0;
        },
        CircuitBreakerState::Open =>
        {
          // Already open, just update failure time
          state.last_failure_time = Some( Instant::now() );
        },
      }
    }

    /// Get the current state of the circuit breaker
    #[ inline ]
    #[ must_use ]
    pub fn state( &self ) -> CircuitBreakerState
    {
      let state = self.state.lock().unwrap();

      // Check for automatic state transitions
      match state.current_state
      {
        CircuitBreakerState::Open =>
        {
          if let Some( last_failure ) = state.last_failure_time
          {
            if last_failure.elapsed() >= self.config.recovery_timeout
            {
              return CircuitBreakerState::HalfOpen;
            }
          }
        },
        _ => {},
      }

      state.current_state
    }

    /// Get the current failure count
    #[ inline ]
    #[ must_use ]
    pub fn failure_count( &self ) -> u32
    {
      let state = self.state.lock().unwrap();
      state.failure_count
    }
  }

  impl Clone for CircuitBreaker
  {
    #[ inline ]
    fn clone( &self ) -> Self
    {
      Self
      {
        config : self.config.clone(),
        state : Arc::clone( &self.state ),
      }
    }
  }

  impl core::fmt::Display for CircuitBreaker
  {
    #[ inline ]
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      let state = self.state.lock().unwrap();
      write!( f, "Circuit breaker [state : {:?}, failures : {}]", state.current_state, state.failure_count )
    }
  }
}

#[ cfg( feature = "circuit_breaker" ) ]
crate ::mod_interface!
{
  exposed use private::CircuitBreakerState;
  exposed use private::CircuitBreakerConfig;
  exposed use private::CircuitBreaker;
}
