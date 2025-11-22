//! Enhanced Circuit Breaker Module
//!
//! This module provides enhanced circuit breaker functionality for HTTP requests with
//! configurable failure thresholds, recovery timeouts, and state management. All functionality
//! is feature-gated to ensure zero overhead when disabled.

#![ allow( clippy::missing_inline_in_public_items, clippy::unused_async ) ]

#[ cfg( feature = "circuit_breaker" ) ]
mod private
{
  use crate::
  {
    error ::{ OpenAIError, Result },
  };

  use std::
  {
    sync ::{ Arc, Mutex },
    time ::Instant,
  };
  use core::time::Duration;

  use serde::{ Serialize, Deserialize };

  /// Circuit breaker state enumeration
  #[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
  pub enum CircuitBreakerState
  {
    /// Circuit is closed - requests flow normally
    Closed,
    /// Circuit is open - all requests are rejected immediately
    Open,
    /// Circuit is half-open - limited requests allowed to test recovery
    HalfOpen,
  }

  /// Enhanced circuit breaker configuration
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct EnhancedCircuitBreakerConfig
  {
    /// Number of consecutive failures required to open the circuit
    pub failure_threshold : u32,
    /// Duration to wait before transitioning from open to half-open
    pub recovery_timeout_ms : u64,
    /// Number of successful requests in half-open state to close circuit
    pub success_threshold : u32,
    /// Maximum number of requests allowed in half-open state
    pub half_open_max_requests : u32,
    /// Timeout for half-open testing period in milliseconds
    pub half_open_timeout_ms : u64,
  }

  impl Default for EnhancedCircuitBreakerConfig
  {
    fn default() -> Self
    {
      Self
      {
        failure_threshold : 5,
        recovery_timeout_ms : 30000,
        success_threshold : 3,
        half_open_max_requests : 5,
        half_open_timeout_ms : 10000,
      }
    }
  }

  impl EnhancedCircuitBreakerConfig
  {
    /// Create a new circuit breaker configuration
    #[ must_use ]
    pub fn new() -> Self
    {
      Self::default()
    }

    /// Set failure threshold
    #[ must_use ]
    pub fn with_failure_threshold( mut self, threshold : u32 ) -> Self
    {
      self.failure_threshold = threshold;
      self
    }

    /// Set recovery timeout
    #[ must_use ]
    pub fn with_recovery_timeout( mut self, timeout_ms : u64 ) -> Self
    {
      self.recovery_timeout_ms = timeout_ms;
      self
    }

    /// Set success threshold for half-open state
    #[ must_use ]
    pub fn with_success_threshold( mut self, threshold : u32 ) -> Self
    {
      self.success_threshold = threshold;
      self
    }

    /// Set max requests allowed in half-open state
    #[ must_use ]
    pub fn with_half_open_max_requests( mut self, max_requests : u32 ) -> Self
    {
      self.half_open_max_requests = max_requests;
      self
    }

    /// Set half-open state timeout
    #[ must_use ]
    pub fn with_half_open_timeout( mut self, timeout_ms : u64 ) -> Self
    {
      self.half_open_timeout_ms = timeout_ms;
      self
    }

    /// Check if an error should trigger the circuit breaker
    #[ must_use ]
    pub fn is_circuit_breaker_error( &self, error : &OpenAIError ) -> bool
    {
      match error
      {
        // Errors that should trigger circuit breaker
        OpenAIError::Network( _ ) | OpenAIError::Timeout( _ ) | OpenAIError::Stream( _ ) | OpenAIError::Ws( _ ) => true,
        // 5xx server errors should trigger circuit breaker
        OpenAIError::Http( message ) =>
        {
          message.contains( "500" ) || message.contains( "502" ) || message.contains( "503" ) || message.contains( "504" )
        },
        // Errors that should not trigger circuit breaker (client issues, config issues, etc)
        OpenAIError::RateLimit( _ ) | OpenAIError::Api( _ ) | OpenAIError::WsInvalidMessage( _ ) |
        OpenAIError::Internal( _ ) | OpenAIError::InvalidArgument( _ ) | OpenAIError::MissingArgument( _ ) |
        OpenAIError::MissingEnvironment( _ ) | OpenAIError::MissingHeader( _ ) | OpenAIError::MissingFile( _ ) |
        OpenAIError::File( _ ) | OpenAIError::Unknown( _ ) => false,
      }
    }

    /// Validate configuration parameters
    ///
    /// # Errors
    ///
    /// Returns an error if any configuration parameter is invalid (e.g., zero values where positive values are required).
    pub fn validate( &self ) -> core::result::Result< (), String >
    {
      if self.failure_threshold == 0
      {
        return Err( "failure_threshold must be greater than 0".to_string() );
      }

      if self.recovery_timeout_ms == 0
      {
        return Err( "recovery_timeout_ms must be greater than 0".to_string() );
      }

      if self.success_threshold == 0
      {
        return Err( "success_threshold must be greater than 0".to_string() );
      }

      if self.half_open_max_requests == 0
      {
        return Err( "half_open_max_requests must be greater than 0".to_string() );
      }

      if self.half_open_timeout_ms == 0
      {
        return Err( "half_open_timeout_ms must be greater than 0".to_string() );
      }

      Ok( () )
    }
  }

  /// Circuit breaker state management
  #[ derive( Debug ) ]
  pub struct CircuitBreakerStateManager
  {
    /// Current state of the circuit breaker
    pub state : CircuitBreakerState,
    /// Number of consecutive failures
    pub failure_count : u32,
    /// Number of consecutive successes in half-open state
    pub success_count : u32,
    /// Number of requests made in half-open state
    pub half_open_requests : u32,
    /// Timestamp when circuit was opened
    pub opened_at : Option< Instant >,
    /// Timestamp when circuit entered half-open state
    pub half_open_at : Option< Instant >,
    /// Total number of requests processed
    pub total_requests : u64,
    /// Total number of failures recorded
    pub total_failures : u64,
    /// Total number of circuit breaker trips
    pub trip_count : u64,
  }

  impl Default for CircuitBreakerStateManager
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  impl CircuitBreakerStateManager
  {
    /// Create new circuit breaker state
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        state : CircuitBreakerState::Closed,
        failure_count : 0,
        success_count : 0,
        half_open_requests : 0,
        opened_at : None,
        half_open_at : None,
        total_requests : 0,
        total_failures : 0,
        trip_count : 0,
      }
    }

    /// Record a successful request
    pub fn record_success( &mut self )
    {
      self.total_requests += 1;
      self.failure_count = 0; // Reset failure count on success

      // In half-open state, track success count
      if self.state == CircuitBreakerState::HalfOpen
      {
        self.success_count += 1;
      }
      // Success in other states just resets failure count
    }

    /// Record a failed request
    pub fn record_failure( &mut self )
    {
      self.total_requests += 1;
      self.total_failures += 1;
      self.failure_count += 1;

      // Reset half-open state on failure
      if self.state == CircuitBreakerState::HalfOpen
      {
        self.success_count = 0;
        self.half_open_requests = 0;
      }
      // Failure in other states just increments counters
    }

    /// Transition to open state
    pub fn open( &mut self )
    {
      if self.state != CircuitBreakerState::Open
      {
        self.state = CircuitBreakerState::Open;
        self.opened_at = Some( Instant::now() );
        self.trip_count += 1;
        self.success_count = 0;
        self.half_open_requests = 0;
        self.half_open_at = None;
      }
    }

    /// Transition to half-open state
    pub fn half_open( &mut self )
    {
      if self.state != CircuitBreakerState::HalfOpen
      {
        self.state = CircuitBreakerState::HalfOpen;
        self.half_open_at = Some( Instant::now() );
        self.success_count = 0;
        self.half_open_requests = 0;
      }
    }

    /// Transition to closed state
    pub fn close( &mut self )
    {
      if self.state != CircuitBreakerState::Closed
      {
        self.state = CircuitBreakerState::Closed;
        self.failure_count = 0;
        self.success_count = 0;
        self.half_open_requests = 0;
        self.opened_at = None;
        self.half_open_at = None;
      }
    }

    /// Check if circuit breaker should allow request
    #[ must_use ]
    pub fn should_allow_request( &self ) -> bool
    {
      match self.state
      {
        CircuitBreakerState::Open => false,
        CircuitBreakerState::Closed | CircuitBreakerState::HalfOpen => true, // Allow requests in closed and half-open states
      }
    }

    /// Get current state as string for logging
    #[ must_use ]
    pub fn state_str( &self ) -> &'static str
    {
      match self.state
      {
        CircuitBreakerState::Closed => "closed",
        CircuitBreakerState::Open => "open",
        CircuitBreakerState::HalfOpen => "half-open",
      }
    }
  }

  /// Enhanced circuit breaker executor
  #[ derive( Debug, Clone ) ]
  pub struct EnhancedCircuitBreaker
  {
    config : EnhancedCircuitBreakerConfig,
    state : Arc< Mutex< CircuitBreakerStateManager > >,
  }

  impl EnhancedCircuitBreaker
  {
    /// Create new circuit breaker with configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration validation fails.
    pub fn new( config : EnhancedCircuitBreakerConfig ) -> core::result::Result< Self, String >
    {
      config.validate()?;

      Ok( Self
      {
        config,
        state : Arc::new( Mutex::new( CircuitBreakerStateManager::new() ) ),
      } )
    }

    /// Execute operation with circuit breaker protection
    ///
    /// # Errors
    ///
    /// Returns an error if the circuit breaker is open (rejecting requests) or if the operation itself fails.
    pub async fn execute< F, Fut, T >( &self, operation : F ) -> Result< T >
    where
      F : Fn() -> Fut,
      Fut : core::future::Future< Output = Result< T > >,
    {
      // Check if request should be allowed
      if !self.should_allow_request()
      {
        return Err( error_tools::untyped::Error::msg( "Circuit breaker is open - request rejected" ) );
      }

      // Execute the operation
      match operation().await
      {
        Ok( result ) =>
        {
          self.record_success().await;
          Ok( result )
        },
        Err( error ) =>
        {
          // Check if this error should trigger circuit breaker
          if let Some( openai_error ) = error.downcast_ref::< OpenAIError >()
          {
            if self.config.is_circuit_breaker_error( openai_error )
            {
              self.record_failure().await?;
            }
          }
          Err( error )
        }
      }
    }

    /// Check if circuit breaker should allow request and update state
    fn should_allow_request( &self ) -> bool
    {
      let mut state = self.state.lock().unwrap();

      match state.state
      {
        CircuitBreakerState::Closed => true,
        CircuitBreakerState::Open =>
        {
          // Check if recovery timeout has elapsed
          if let Some( opened_at ) = state.opened_at
          {
            let elapsed = opened_at.elapsed();
            if elapsed >= Duration::from_millis( self.config.recovery_timeout_ms )
            {
              // Transition to half-open
              state.half_open();
              state.half_open_requests += 1; // Count this request
              true
            }
            else
            {
              false
            }
          }
          else
          {
            false
          }
        },
        CircuitBreakerState::HalfOpen =>
        {
          // Check if half-open timeout has elapsed
          if let Some( half_open_at ) = state.half_open_at
          {
            let elapsed = half_open_at.elapsed();
            if elapsed >= Duration::from_millis( self.config.half_open_timeout_ms )
            {
              // Timeout elapsed, go back to open
              state.open();
              false
            }
            else if state.half_open_requests + 1 > self.config.half_open_max_requests
            {
              // Too many requests in half-open state
              false
            }
            else
            {
              state.half_open_requests += 1;
              true
            }
          }
          else
          {
            state.half_open_requests += 1;
            true
          }
        }
      }
    }

    /// Record successful operation
    async fn record_success( &self )
    {
      let mut state = self.state.lock().unwrap();
      state.record_success();

      // Check if we should close the circuit in half-open state
      if state.state == CircuitBreakerState::HalfOpen && state.success_count >= self.config.success_threshold
      {
        state.close();
      }
    }

    /// Record failed operation and update state
    async fn record_failure( &self ) -> Result< () >
    {
      let mut state = self.state.lock().unwrap();
      state.record_failure();

      // Check if we should open the circuit
      match state.state
      {
        CircuitBreakerState::Closed =>
        {
          if state.failure_count >= self.config.failure_threshold
          {
            state.open();
          }
        },
        CircuitBreakerState::HalfOpen =>
        {
          // Any failure in half-open goes back to open
          state.open();
        },
        CircuitBreakerState::Open => {} // Already open
      }

      Ok( () )
    }

    /// Get current circuit breaker state (for testing and metrics)
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    #[ must_use ]
    pub fn get_state( &self ) -> CircuitBreakerStateManager
    {
      let state = self.state.lock().unwrap();
      CircuitBreakerStateManager
      {
        state : state.state.clone(),
        failure_count : state.failure_count,
        success_count : state.success_count,
        half_open_requests : state.half_open_requests,
        opened_at : state.opened_at,
        half_open_at : state.half_open_at,
        total_requests : state.total_requests,
        total_failures : state.total_failures,
        trip_count : state.trip_count,
      }
    }

    /// Get circuit breaker configuration
    #[ must_use ]
    pub fn config( &self ) -> &EnhancedCircuitBreakerConfig
    {
      &self.config
    }
  }
}

// Re-export circuit breaker functionality only when feature is enabled
#[ cfg( feature = "circuit_breaker" ) ]
pub use private::
{
  EnhancedCircuitBreakerConfig,
  CircuitBreakerState,
  CircuitBreakerStateManager,
  EnhancedCircuitBreaker,
};

// Provide no-op implementations when circuit breaker feature is disabled
#[ cfg( not( feature = "circuit_breaker" ) ) ]
/// No-op circuit breaker configuration module when circuit breaker feature is disabled
pub mod private
{
  /// No-op circuit breaker configuration when feature is disabled
  #[ derive( Debug, Clone ) ]
  pub struct EnhancedCircuitBreakerConfig;

  impl EnhancedCircuitBreakerConfig
  {
    /// Create a new no-op configuration
    pub fn new() -> Self
    {
      Self
    }
  }

  impl Default for EnhancedCircuitBreakerConfig
  {
    fn default() -> Self
    {
      Self
    }
  }
}

#[ cfg( not( feature = "circuit_breaker" ) ) ]
pub use private::EnhancedCircuitBreakerConfig;

// Export for mod_interface
crate ::mod_interface!
{
  #[ cfg( feature = "circuit_breaker" ) ]
  exposed use
  {
    EnhancedCircuitBreakerConfig,
    CircuitBreakerState,
    CircuitBreakerStateManager,
    EnhancedCircuitBreaker,
  };

  #[ cfg( not( feature = "circuit_breaker" ) ) ]
  exposed use
  {
    EnhancedCircuitBreakerConfig,
  };
}