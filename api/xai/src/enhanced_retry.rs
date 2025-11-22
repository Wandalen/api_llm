mod private
{
  use std::time::Duration;
  use crate::error::{ XaiError, Result };
  use tokio::time::sleep;

  /// Enhanced retry configuration with exponential backoff.
  ///
  /// Provides configurable retry behavior with exponential backoff and jitter
  /// to handle transient failures gracefully.
  ///
  /// # Examples
  ///
  /// ```
  /// use api_xai::EnhancedRetryConfig;
  /// use std::time::Duration;
  ///
  /// let config = EnhancedRetryConfig::default()
  ///   .with_max_attempts( 5 )
  ///   .with_base_delay( Duration::from_millis( 100 ) )
  ///   .with_max_delay( Duration::from_secs( 10 ) );
  /// ```
  #[ derive( Debug, Clone ) ]
  pub struct EnhancedRetryConfig
  {
    /// Maximum number of retry attempts (including initial attempt).
    pub max_attempts : usize,

    /// Base delay for exponential backoff.
    pub base_delay : Duration,

    /// Maximum delay between retries.
    pub max_delay : Duration,

    /// Whether to add jitter to backoff delays.
    pub use_jitter : bool,
  }

  impl Default for EnhancedRetryConfig
  {
    fn default() -> Self
    {
      Self
      {
        max_attempts : 3,
        base_delay : Duration::from_millis( 100 ),
        max_delay : Duration::from_secs( 30 ),
        use_jitter : true,
      }
    }
  }

  impl EnhancedRetryConfig
  {
    /// Sets the maximum number of retry attempts.
    ///
    /// # Arguments
    ///
    /// * `attempts` - Total number of attempts (including initial attempt)
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::EnhancedRetryConfig;
    ///
    /// let config = EnhancedRetryConfig::default()
    ///   .with_max_attempts( 5 );
    /// ```
    #[ must_use ]
    pub fn with_max_attempts( mut self, attempts : usize ) -> Self
    {
      self.max_attempts = attempts;
      self
    }

    /// Sets the base delay for exponential backoff.
    ///
    /// The actual delay is calculated as : `base_delay * 2^attempt`
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::EnhancedRetryConfig;
    /// use std::time::Duration;
    ///
    /// let config = EnhancedRetryConfig::default()
    ///   .with_base_delay( Duration::from_millis( 200 ) );
    /// ```
    #[ must_use ]
    pub fn with_base_delay( mut self, delay : Duration ) -> Self
    {
      self.base_delay = delay;
      self
    }

    /// Sets the maximum delay between retries.
    ///
    /// Caps the exponential backoff to prevent excessively long delays.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::EnhancedRetryConfig;
    /// use std::time::Duration;
    ///
    /// let config = EnhancedRetryConfig::default()
    ///   .with_max_delay( Duration::from_secs( 60 ) );
    /// ```
    #[ must_use ]
    pub fn with_max_delay( mut self, delay : Duration ) -> Self
    {
      self.max_delay = delay;
      self
    }

    /// Enables or disables jitter in backoff delays.
    ///
    /// Jitter adds randomness to delays to prevent thundering herd problem.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::EnhancedRetryConfig;
    ///
    /// let config = EnhancedRetryConfig::default()
    ///   .with_jitter( false );
    /// ```
    #[ must_use ]
    pub fn with_jitter( mut self, enable : bool ) -> Self
    {
      self.use_jitter = enable;
      self
    }

    /// Calculates the delay for a given retry attempt.
    ///
    /// Uses exponential backoff with optional jitter.
    ///
    /// # Arguments
    ///
    /// * `attempt` - The current retry attempt number (0-indexed)
    ///
    /// # Returns
    ///
    /// Duration to wait before the next retry attempt.
    #[ allow( clippy::cast_possible_truncation) ]  // Bounded by min() operations
    pub fn delay_for_attempt( &self, attempt : usize ) -> Duration
    {
      // Calculate exponential delay : base * 2^attempt
      // Use saturating conversions to handle overflow gracefully
      let base_ms = self.base_delay.as_millis().min( u128::from(u64::MAX) ) as u64;
      let attempt_u32 = attempt.min( u32::MAX as usize ) as u32;
      let exponential_delay = base_ms.saturating_mul( 2u64.saturating_pow( attempt_u32 ) );

      // Cap at max_delay
      let max_ms = self.max_delay.as_millis().min( u128::from(u64::MAX) ) as u64;
      let delay_ms = exponential_delay.min( max_ms );

      // Add jitter if enabled (0-50% randomness)
      let final_delay_ms = if self.use_jitter
      {
        let jitter = ( rand::random::< f64 >() * 0.5 ) + 0.5; // 0.5 to 1.0
        #[ allow( clippy::cast_sign_loss) ]  // jitter is always positive (0.5-1.0)
        let result = ( delay_ms as f64 * jitter ).min( u64::MAX as f64 ) as u64;
        result
      }
      else
      {
        delay_ms
      };

      Duration::from_millis( final_delay_ms )
    }

    /// Checks if an error should be retried.
    ///
    /// Determines whether a given error is transient and worth retrying.
    ///
    /// # Retriable Errors
    ///
    /// - Network errors (connection failures)
    /// - Timeout errors
    /// - Rate limit errors (429 responses)
    /// - HTTP 5xx server errors
    ///
    /// # Non-retriable Errors
    ///
    /// - Invalid API key (authentication failures)
    /// - HTTP 4xx client errors (except 429)
    /// - Serialization errors
    ///
    /// # Arguments
    ///
    /// * `error` - The error to check
    ///
    /// # Returns
    ///
    /// `true` if the error should be retried, `false` otherwise.
    pub fn should_retry( &self, error : &XaiError ) -> bool
    {
      match error
      {
        // Always retry transient errors
        XaiError::Network( _ ) | XaiError::Timeout( _ ) | XaiError::RateLimit( _ ) => true,

        // Retry API errors that are server-side (5xx status codes)
        XaiError::Api { code, .. } =>
        {
          // Retry if code suggests server error
          if let Some( code_str ) = code
          {
            code_str.starts_with( '5' ) || code_str == "429"
          }
          else
          {
            false
          }
        }

        // Retry generic HTTP errors (may be transient)
        XaiError::Http( msg ) =>
        {
          // Check if message contains 5xx or 429 status
          msg.contains( "500" ) || msg.contains( "502" ) || msg.contains( "503" )
            || msg.contains( "504" ) || msg.contains( "429" )
        }

        // Don't retry circuit breaker errors (feature-gated, kept separate for conditional compilation)
        #[ cfg( feature = "circuit_breaker" ) ]
        #[ allow( clippy::match_same_arms ) ]  // Must be separate due to cfg
        XaiError::CircuitBreakerOpen( _ ) => false,

        // Don't retry client errors, permanent failures, validation errors, or streaming errors
        XaiError::InvalidApiKey( _ )
        | XaiError::Serialization( _ )
        | XaiError::Environment( _ )
        | XaiError::UrlParse( _ )
        | XaiError::Stream( _ )
        | XaiError::InvalidModel( _ )
        | XaiError::InvalidParameter( _ )
        | XaiError::ApiError( _ ) => false,
      }
    }

    /// Executes a function with retry logic.
    ///
    /// Automatically retries the function on retriable errors with exponential backoff.
    ///
    /// # Type Parameters
    ///
    /// * `F` - Function that returns a Future
    /// * `Fut` - Future type returned by the function
    /// * `T` - Success type of the Result
    ///
    /// # Arguments
    ///
    /// * `f` - Async function to execute with retries
    ///
    /// # Returns
    ///
    /// Result of the function execution after retries.
    ///
    /// # Errors
    ///
    /// Returns the last error if all retry attempts fail.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use api_xai::{ EnhancedRetryConfig, XaiError, Result };
    ///
    /// # async fn example() -> Result< () > {
    /// let retry_config = EnhancedRetryConfig::default()
    ///   .with_max_attempts( 3 );
    ///
    /// let result = retry_config.call( || async {
    ///   // Your API call here
    ///   Ok::< String, _ >( "response".to_string() ) as Result< String >
    /// } ).await?;
    /// # Ok( () )
    /// # }
    /// ```
    pub async fn call< F, Fut, T >( &self, mut f : F ) -> Result< T >
    where
      F : FnMut() -> Fut,
      Fut : std::future::Future< Output = Result< T > >,
    {
      let mut attempt = 0;

      loop
      {
        match f().await
        {
          Ok( result ) => return Ok( result ),
          Err( err ) =>
          {
            attempt += 1;

            // Check if we've exhausted retries
            if attempt >= self.max_attempts
            {
              return Err( err );
            }

            // Check if error is retriable
            let should_retry = if let Some( xai_err ) = err.downcast_ref::< XaiError >()
            {
              self.should_retry( xai_err )
            }
            else
            {
              // Unknown error type, don't retry
              false
            };

            if !should_retry
            {
              return Err( err );
            }

            // Calculate and wait for backoff delay
            let delay = self.delay_for_attempt( attempt - 1 );
            sleep( delay ).await;
          }
        }
      }
    }
  }

  /// Random number generation for jitter.
  mod rand
  {
    use std::cell::Cell;
    use std::time::{ SystemTime, UNIX_EPOCH };

    thread_local!
    {
      #[ allow( clippy::cast_possible_truncation) ]  // wrapping_rem ensures value fits in u64
      static RNG_STATE : Cell< u64 > = Cell::new(
        SystemTime::now()
          .duration_since( UNIX_EPOCH )
          .unwrap()
          .as_nanos()
          .wrapping_rem( u128::from(u64::MAX) ) as u64
      );
    }

    /// Generates a random f64 in range [0.0, 1.0) using xorshift.
    ///
    /// Simple, fast PRNG suitable for jitter. Not cryptographically secure.
    pub fn random< T : From< f64 > >() -> T
    {
      RNG_STATE.with( | state |
      {
        let mut x = state.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        state.set( x );

        // Convert to f64 in range [0.0, 1.0)
        T::from( ( x as f64 ) / ( u64::MAX as f64 ) )
      } )
    }
  }
}

crate::mod_interface!
{
  exposed use
  {
    EnhancedRetryConfig,
  };
}
