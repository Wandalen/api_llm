mod private
{
  use std::sync::{ Arc, Mutex };
  use std::time::{ Duration, Instant };
  use crate::error::{ XaiError, Result };

  /// Circuit breaker states.
  ///
  /// The circuit breaker transitions between states based on success/failure patterns:
  /// - **Closed**: Normal operation, requests pass through
  /// - **Open**: Too many failures, requests rejected immediately
  /// - **`HalfOpen`**: Testing recovery, limited requests allowed
  #[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
  pub enum CircuitState
  {
    /// Circuit is closed, requests pass through normally.
    Closed,

    /// Circuit is open, requests are rejected immediately.
    Open,

    /// Circuit is testing recovery, allowing limited requests.
    HalfOpen,
  }

  /// Circuit breaker configuration.
  ///
  /// Configures the thresholds and timeouts for circuit breaker behavior.
  ///
  /// # Examples
  ///
  /// ```
  /// use api_xai::CircuitBreakerConfig;
  /// use std::time::Duration;
  ///
  /// let config = CircuitBreakerConfig::default()
  ///   .with_failure_threshold( 5 )
  ///   .with_timeout( Duration::from_secs( 30 ) )
  ///   .with_success_threshold( 2 );
  /// ```
  #[ derive( Debug, Clone ) ]
  pub struct CircuitBreakerConfig
  {
    /// Number of consecutive failures before opening circuit.
    pub failure_threshold : usize,

    /// Duration to wait before moving from `Open` to `HalfOpen`.
    pub timeout : Duration,

    /// Number of consecutive successes in `HalfOpen` before closing.
    pub success_threshold : usize,
  }

  impl Default for CircuitBreakerConfig
  {
    fn default() -> Self
    {
      Self
      {
        failure_threshold : 5,
        timeout : Duration::from_secs( 30 ),
        success_threshold : 2,
      }
    }
  }

  impl CircuitBreakerConfig
  {
    /// Sets the failure threshold.
    ///
    /// Number of consecutive failures before the circuit opens.
    #[ must_use ]
    pub fn with_failure_threshold( mut self, threshold : usize ) -> Self
    {
      self.failure_threshold = threshold;
      self
    }

    /// Sets the timeout duration.
    ///
    /// Time to wait before transitioning from `Open` to `HalfOpen`.
    #[ must_use ]
    pub fn with_timeout( mut self, timeout : Duration ) -> Self
    {
      self.timeout = timeout;
      self
    }

    /// Sets the success threshold.
    ///
    /// Number of consecutive successes in `HalfOpen` before closing circuit.
    #[ must_use ]
    pub fn with_success_threshold( mut self, threshold : usize ) -> Self
    {
      self.success_threshold = threshold;
      self
    }
  }

  /// Circuit breaker for protecting against cascading failures.
  ///
  /// Monitors request failures and temporarily blocks requests when failure
  /// rate exceeds thresholds, preventing resource exhaustion.
  ///
  /// # State Transitions
  ///
  /// ```text
  /// Closed --[failures >= threshold]--> Open
  /// Open --[timeout elapsed]--> HalfOpen
  /// HalfOpen --[success]--> Closed
  /// HalfOpen --[failure]--> Open
  /// ```
  ///
  /// # Examples
  ///
  /// ```
  /// use api_xai::{ CircuitBreaker, CircuitBreakerConfig };
  ///
  /// let breaker = CircuitBreaker::new( CircuitBreakerConfig::default() );
  ///
  /// // Check if request is allowed
  /// if breaker.is_request_allowed() {
  ///   // Execute request
  ///   match perform_request() {
  ///     Ok( result ) => {
  ///       breaker.record_success();
  ///       // Use result
  ///     }
  ///     Err( e ) => {
  ///       breaker.record_failure();
  ///       // Handle error
  ///     }
  ///   }
  /// }
  ///
  /// # fn perform_request() -> Result< (), Box< dyn std::error::Error > > { Ok( () ) }
  /// ```
  #[ derive( Debug, Clone ) ]
  pub struct CircuitBreaker
  {
    config : CircuitBreakerConfig,
    state : Arc< Mutex< CircuitBreakerState > >,
  }

  #[ derive( Debug ) ]
  struct CircuitBreakerState
  {
    current_state : CircuitState,
    failure_count : usize,
    success_count : usize,
    last_failure_time : Option< Instant >,
  }

  impl CircuitBreaker
  {
    /// Creates a new circuit breaker with the given configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::{ CircuitBreaker, CircuitBreakerConfig };
    ///
    /// let breaker = CircuitBreaker::new( CircuitBreakerConfig::default() );
    /// ```
    pub fn new( config : CircuitBreakerConfig ) -> Self
    {
      Self
      {
        config,
        state : Arc::new( Mutex::new( CircuitBreakerState
        {
          current_state : CircuitState::Closed,
          failure_count : 0,
          success_count : 0,
          last_failure_time : None,
        } ) ),
      }
    }

    /// Checks if a request is allowed based on current circuit state.
    ///
    /// # Returns
    ///
    /// `true` if the request should proceed, `false` if it should be rejected.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::CircuitBreaker;
    ///
    /// let breaker = CircuitBreaker::default();
    ///
    /// if breaker.is_request_allowed() {
    ///   // Proceed with request
    /// } else {
    ///   // Circuit is open, reject request
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn is_request_allowed( &self ) -> bool
    {
      let mut state = self.state.lock().unwrap();

      match state.current_state
      {
        CircuitState::Closed | CircuitState::HalfOpen => true,
        CircuitState::Open =>
        {
          // Check if timeout has elapsed
          if let Some( last_failure ) = state.last_failure_time
          {
            if last_failure.elapsed() >= self.config.timeout
            {
              // Transition to HalfOpen
              state.current_state = CircuitState::HalfOpen;
              state.success_count = 0;
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
        }
      }
    }

    /// Records a successful request.
    ///
    /// In `HalfOpen` state, enough successes will close the circuit.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::CircuitBreaker;
    ///
    /// let breaker = CircuitBreaker::default();
    ///
    /// // After successful request
    /// breaker.record_success();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn record_success( &self )
    {
      let mut state = self.state.lock().unwrap();

      match state.current_state
      {
        CircuitState::HalfOpen =>
        {
          state.success_count += 1;

          if state.success_count >= self.config.success_threshold
          {
            // Enough successes, close the circuit
            state.current_state = CircuitState::Closed;
            state.failure_count = 0;
            state.success_count = 0;
          }
        }
        CircuitState::Closed =>
        {
          // Reset failure count on success
          state.failure_count = 0;
        }
        CircuitState::Open => {}
      }
    }

    /// Records a failed request.
    ///
    /// Increments failure counter and may open the circuit if threshold is reached.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::CircuitBreaker;
    ///
    /// let breaker = CircuitBreaker::default();
    ///
    /// // After failed request
    /// breaker.record_failure();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn record_failure( &self )
    {
      let mut state = self.state.lock().unwrap();

      match state.current_state
      {
        CircuitState::Closed =>
        {
          state.failure_count += 1;

          if state.failure_count >= self.config.failure_threshold
          {
            // Too many failures, open the circuit
            state.current_state = CircuitState::Open;
            state.last_failure_time = Some( Instant::now() );
          }
        }
        CircuitState::HalfOpen =>
        {
          // Any failure in HalfOpen reopens the circuit
          state.current_state = CircuitState::Open;
          state.failure_count = 0;
          state.success_count = 0;
          state.last_failure_time = Some( Instant::now() );
        }
        CircuitState::Open =>
        {
          // Update last failure time
          state.last_failure_time = Some( Instant::now() );
        }
      }
    }

    /// Returns the current state of the circuit breaker.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::{ CircuitBreaker, CircuitState };
    ///
    /// let breaker = CircuitBreaker::default();
    /// assert_eq!( breaker.state(), CircuitState::Closed );
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn state( &self ) -> CircuitState
    {
      self.state.lock().unwrap().current_state
    }

    /// Executes a function with circuit breaker protection.
    ///
    /// Checks if request is allowed, executes function, and records result.
    ///
    /// # Errors
    ///
    /// Returns `XaiError::CircuitBreakerOpen` if circuit is open.
    /// Returns the function's error if execution fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use api_xai::CircuitBreaker;
    ///
    /// # async fn example() -> Result< (), Box< dyn std::error::Error > > {
    /// let breaker = CircuitBreaker::default();
    ///
    /// let result = breaker.call( || async {
    ///   // Your API call here
    ///   Ok( "response" )
    /// } ).await?;
    /// # Ok( () )
    /// # }
    /// ```
    pub async fn call< F, Fut, T >( &self, f : F ) -> Result< T >
    where
      F : FnOnce() -> Fut,
      Fut : std::future::Future< Output = Result< T > >,
    {
      if !self.is_request_allowed()
      {
        return Err( XaiError::CircuitBreakerOpen(
          "Circuit breaker is open, request rejected".to_string()
        ).into() );
      }

      match f().await
      {
        Ok( result ) =>
        {
          self.record_success();
          Ok( result )
        }
        Err( err ) =>
        {
          self.record_failure();
          Err( err )
        }
      }
    }

    /// Resets the circuit breaker to Closed state.
    ///
    /// Clears all counters and state. Use with caution.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::CircuitBreaker;
    ///
    /// let breaker = CircuitBreaker::default();
    /// breaker.reset();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn reset( &self )
    {
      let mut state = self.state.lock().unwrap();
      state.current_state = CircuitState::Closed;
      state.failure_count = 0;
      state.success_count = 0;
      state.last_failure_time = None;
    }
  }

  impl Default for CircuitBreaker
  {
    fn default() -> Self
    {
      Self::new( CircuitBreakerConfig::default() )
    }
  }
}

crate::mod_interface!
{
  exposed use
  {
    CircuitState,
    CircuitBreakerConfig,
    CircuitBreaker,
  };
}
