mod private
{
  use std::sync::{ Arc, Mutex };
  use std::time::{ Duration, Instant };
  use tokio::time::sleep;

  /// Rate limiter configuration using token bucket algorithm.
  ///
  /// Controls the rate of API requests to prevent exceeding rate limits.
  ///
  /// # Examples
  ///
  /// ```
  /// use api_xai::RateLimiterConfig;
  ///
  /// // 100 requests per minute
  /// let config = RateLimiterConfig::new( 100, 60 );
  /// ```
  #[ derive( Debug, Clone ) ]
  pub struct RateLimiterConfig
  {
    /// Maximum number of tokens in the bucket.
    pub capacity : usize,

    /// Number of tokens to refill per second.
    pub refill_rate : f64,
  }

  impl RateLimiterConfig
  {
    /// Creates a new rate limiter configuration.
    ///
    /// # Arguments
    ///
    /// * `requests` - Maximum number of requests
    /// * `period_seconds` - Time period in seconds
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::RateLimiterConfig;
    ///
    /// // 60 requests per minute
    /// let config = RateLimiterConfig::new( 60, 60 );
    /// ```
    pub fn new( requests : usize, period_seconds : u64 ) -> Self
    {
      Self
      {
        capacity : requests,
        refill_rate : requests as f64 / period_seconds as f64,
      }
    }

    /// Creates a limiter for requests per second.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::RateLimiterConfig;
    ///
    /// // 10 requests per second
    /// let config = RateLimiterConfig::per_second( 10 );
    /// ```
    pub fn per_second( requests : usize ) -> Self
    {
      Self::new( requests, 1 )
    }

    /// Creates a limiter for requests per minute.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::RateLimiterConfig;
    ///
    /// // 100 requests per minute
    /// let config = RateLimiterConfig::per_minute( 100 );
    /// ```
    pub fn per_minute( requests : usize ) -> Self
    {
      Self::new( requests, 60 )
    }

    /// Creates a limiter for requests per hour.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::RateLimiterConfig;
    ///
    /// // 1000 requests per hour
    /// let config = RateLimiterConfig::per_hour( 1000 );
    /// ```
    pub fn per_hour( requests : usize ) -> Self
    {
      Self::new( requests, 3600 )
    }
  }

  impl Default for RateLimiterConfig
  {
    fn default() -> Self
    {
      // Default : 100 requests per minute
      Self::per_minute( 100 )
    }
  }

  /// Token bucket rate limiter.
  ///
  /// Implements the token bucket algorithm for rate limiting API requests.
  /// Tokens are refilled at a constant rate, and requests consume tokens.
  ///
  /// # Examples
  ///
  /// ```
  /// use api_xai::{ RateLimiter, RateLimiterConfig };
  ///
  /// # async fn example() {
  /// let limiter = RateLimiter::new( RateLimiterConfig::per_second( 10 ) );
  ///
  /// // Acquire permission before making request
  /// limiter.acquire().await;
  /// // Make API request
  /// # }
  /// ```
  #[ derive( Debug, Clone ) ]
  pub struct RateLimiter
  {
    config : RateLimiterConfig,
    state : Arc< Mutex< RateLimiterState > >,
  }

  #[ derive( Debug ) ]
  struct RateLimiterState
  {
    tokens : f64,
    last_refill : Instant,
  }

  impl RateLimiter
  {
    /// Creates a new rate limiter with the given configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::{ RateLimiter, RateLimiterConfig };
    ///
    /// let limiter = RateLimiter::new( RateLimiterConfig::per_minute( 100 ) );
    /// ```
    pub fn new( config : RateLimiterConfig ) -> Self
    {
      Self
      {
        state : Arc::new( Mutex::new( RateLimiterState
        {
          tokens : config.capacity as f64,
          last_refill : Instant::now(),
        } ) ),
        config,
      }
    }

    /// Refills tokens based on elapsed time.
    fn refill( &self, state : &mut RateLimiterState )
    {
      let now = Instant::now();
      let elapsed = now.duration_since( state.last_refill ).as_secs_f64();

      // Add tokens based on refill rate and elapsed time
      state.tokens += elapsed * self.config.refill_rate;

      // Cap at capacity
      if state.tokens > self.config.capacity as f64
      {
        state.tokens = self.config.capacity as f64;
      }

      state.last_refill = now;
    }

    /// Acquires a token, waiting if necessary.
    ///
    /// Blocks until a token is available. Returns immediately if a token
    /// is available, otherwise sleeps until one is refilled.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::RateLimiter;
    ///
    /// # async fn example() {
    /// let limiter = RateLimiter::default();
    ///
    /// // Wait for permission to proceed
    /// limiter.acquire().await;
    ///
    /// // Make API request
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub async fn acquire( &self )
    {
      loop
      {
        let wait_time = {
          let mut state = self.state.lock().unwrap();
          self.refill( &mut state );

          if state.tokens >= 1.0
          {
            state.tokens -= 1.0;
            return;
          }

          // Calculate time to wait for next token
          let tokens_needed = 1.0 - state.tokens;
          Duration::from_secs_f64( tokens_needed / self.config.refill_rate )
          // MutexGuard dropped here when scope ends
        };

        // Sleep and retry (guard is already dropped)
        sleep( wait_time ).await;
      }
    }

    /// Tries to acquire a token without waiting.
    ///
    /// Returns `true` if a token was acquired, `false` if none available.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::RateLimiter;
    ///
    /// let limiter = RateLimiter::default();
    ///
    /// if limiter.try_acquire() {
    ///   // Token acquired, proceed with request
    /// } else {
    ///   // No tokens available, skip or queue request
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn try_acquire( &self ) -> bool
    {
      let mut state = self.state.lock().unwrap();
      self.refill( &mut state );

      if state.tokens >= 1.0
      {
        state.tokens -= 1.0;
        true
      }
      else
      {
        false
      }
    }

    /// Returns the current number of available tokens.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::RateLimiter;
    ///
    /// let limiter = RateLimiter::default();
    /// let available = limiter.available_tokens();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    #[ allow( clippy::cast_possible_truncation, clippy::cast_sign_loss) ]  // Tokens always non-negative, bounded by capacity
    pub fn available_tokens( &self ) -> usize
    {
      let mut state = self.state.lock().unwrap();
      self.refill( &mut state );
      state.tokens.max( 0.0 ).floor() as usize
    }

    /// Resets the rate limiter to full capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::RateLimiter;
    ///
    /// let limiter = RateLimiter::default();
    /// limiter.reset();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn reset( &self )
    {
      let mut state = self.state.lock().unwrap();
      state.tokens = self.config.capacity as f64;
      state.last_refill = Instant::now();
    }
  }

  impl Default for RateLimiter
  {
    fn default() -> Self
    {
      Self::new( RateLimiterConfig::default() )
    }
  }
}

crate::mod_interface!
{
  exposed use
  {
    RateLimiterConfig,
    RateLimiter,
  };
}
