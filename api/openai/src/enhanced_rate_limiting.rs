//! Enhanced Rate Limiting Module
//!
//! This module provides enhanced rate limiting functionality for HTTP requests with
//! configurable algorithms (token bucket, sliding window), burst capacity, and state management.
//! All functionality is feature-gated to ensure zero overhead when disabled.

#![ allow( clippy::missing_inline_in_public_items ) ]

#[ cfg( feature = "rate_limiting" ) ]
mod private
{
  use crate::
  {
    error ::Result,
  };

  use core::time::Duration;
  use std::
  {
    sync ::{ Arc, Mutex },
    time ::Instant,
    collections ::VecDeque,
  };

  use serde::{ Serialize, Deserialize };

  /// Rate limiting algorithm enumeration
  #[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
  pub enum RateLimitingAlgorithm
  {
    /// Token bucket algorithm with refill rate and burst capacity
    TokenBucket,
    /// Sliding window algorithm with request timestamps
    SlidingWindow,
  }

  /// Enhanced rate limiting configuration
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct EnhancedRateLimitingConfig
  {
    /// Maximum number of requests per time window
    pub max_requests : u32,
    /// Time window duration in milliseconds
    pub window_duration_ms : u64,
    /// Maximum burst capacity (token bucket only)
    pub burst_capacity : u32,
    /// Token refill rate per second (token bucket only)
    pub refill_rate : f64,
    /// Rate limiting algorithm to use
    pub algorithm : RateLimitingAlgorithm,
    /// Request timeout when rate limit exceeded
    pub timeout_ms : u64,
    /// Whether to enable per-endpoint rate limiting
    pub per_endpoint : bool,
  }

  impl Default for EnhancedRateLimitingConfig
  {
    fn default() -> Self
    {
      Self
      {
        max_requests : 100,
        window_duration_ms : 60000, // 1 minute
        burst_capacity : 10,
        refill_rate : 1.66, // ~100 requests per minute
        algorithm : RateLimitingAlgorithm::TokenBucket,
        timeout_ms : 5000,
        per_endpoint : false,
      }
    }
  }

  impl EnhancedRateLimitingConfig
  {
    /// Create a new rate limiting configuration
    #[ must_use ]
    pub fn new() -> Self
    {
      Self::default()
    }

    /// Set maximum requests per window
    #[ must_use ]
    pub fn with_max_requests( mut self, max_requests : u32 ) -> Self
    {
      self.max_requests = max_requests;
      self
    }

    /// Set window duration
    #[ must_use ]
    pub fn with_window_duration( mut self, duration_ms : u64 ) -> Self
    {
      self.window_duration_ms = duration_ms;
      self
    }

    /// Set burst capacity for token bucket
    #[ must_use ]
    pub fn with_burst_capacity( mut self, capacity : u32 ) -> Self
    {
      self.burst_capacity = capacity;
      self
    }

    /// Set token refill rate
    #[ must_use ]
    pub fn with_refill_rate( mut self, rate : f64 ) -> Self
    {
      self.refill_rate = rate;
      self
    }

    /// Set rate limiting algorithm
    #[ must_use ]
    pub fn with_algorithm( mut self, algorithm : RateLimitingAlgorithm ) -> Self
    {
      self.algorithm = algorithm;
      self
    }

    /// Set timeout duration
    #[ must_use ]
    pub fn with_timeout( mut self, timeout_ms : u64 ) -> Self
    {
      self.timeout_ms = timeout_ms;
      self
    }

    /// Enable per-endpoint rate limiting
    #[ must_use ]
    pub fn with_per_endpoint( mut self, per_endpoint : bool ) -> Self
    {
      self.per_endpoint = per_endpoint;
      self
    }

    /// Validate configuration parameters
    ///
    /// # Errors
    ///
    /// Returns an error if any configuration parameter is invalid.
    pub fn validate( &self ) -> core::result::Result< (), String >
    {
      if self.max_requests == 0
      {
        return Err( "max_requests must be greater than 0".to_string() );
      }

      if self.window_duration_ms == 0
      {
        return Err( "window_duration_ms must be greater than 0".to_string() );
      }

      if self.burst_capacity == 0
      {
        return Err( "burst_capacity must be greater than 0".to_string() );
      }

      if self.refill_rate <= 0.0
      {
        return Err( "refill_rate must be greater than 0".to_string() );
      }

      if self.timeout_ms == 0
      {
        return Err( "timeout_ms must be greater than 0".to_string() );
      }

      Ok( () )
    }
  }

  /// Token bucket rate limiter state
  #[ derive( Debug ) ]
  pub struct TokenBucketState
  {
    /// Current number of tokens available
    pub tokens : f64,
    /// Last time tokens were refilled
    pub last_refill : Instant,
    /// Total number of requests processed
    pub total_requests : u64,
    /// Total number of rate limited requests
    pub rate_limited_requests : u64,
  }

  impl TokenBucketState
  {
    /// Create new token bucket state
    #[ must_use ]
    pub fn new( initial_tokens : f64 ) -> Self
    {
      Self
      {
        tokens : initial_tokens,
        last_refill : Instant::now(),
        total_requests : 0,
        rate_limited_requests : 0,
      }
    }

    /// Refill tokens based on elapsed time
    pub fn refill_tokens( &mut self, refill_rate : f64, burst_capacity : f64 )
    {
      let now = Instant::now();
      let elapsed = now.duration_since( self.last_refill ).as_secs_f64();
      let tokens_to_add = elapsed * refill_rate;

      self.tokens = ( self.tokens + tokens_to_add ).min( burst_capacity );
      self.last_refill = now;
    }

    /// Try to consume a token
    #[ must_use ]
    pub fn try_consume( &mut self ) -> bool
    {
      self.total_requests += 1;

      if self.tokens >= 1.0
      {
        self.tokens -= 1.0;
        true
      }
      else
      {
        self.rate_limited_requests += 1;
        false
      }
    }

    /// Reset state for testing
    pub fn reset( &mut self, initial_tokens : f64 )
    {
      self.tokens = initial_tokens;
      self.last_refill = Instant::now();
      self.total_requests = 0;
      self.rate_limited_requests = 0;
    }
  }

  /// Sliding window rate limiter state
  #[ derive( Debug, Default ) ]
  pub struct SlidingWindowState
  {
    /// Request timestamps within the current window
    pub request_timestamps : VecDeque< Instant >,
    /// Total number of requests processed
    pub total_requests : u64,
    /// Total number of rate limited requests
    pub rate_limited_requests : u64,
  }

  impl SlidingWindowState
  {
    /// Create new sliding window state
    #[ must_use ]
    pub fn new() -> Self
    {
      Self::default()
    }

    /// Clean up old timestamps outside the window
    ///
    /// # Panics
    ///
    /// Panics if the current time is before the window duration (time arithmetic overflow).
    pub fn cleanup_old_timestamps( &mut self, window_duration : Duration )
    {
      let cutoff_time = Instant::now().checked_sub( window_duration ).unwrap();

      while let Some( &front_time ) = self.request_timestamps.front()
      {
        if front_time < cutoff_time
        {
          self.request_timestamps.pop_front();
        }
        else
        {
          break;
        }
      }
    }

    /// Try to add a request to the window
    #[ must_use ]
    pub fn try_add_request( &mut self, max_requests : u32 ) -> bool
    {
      self.total_requests += 1;

      if self.request_timestamps.len() < max_requests as usize
      {
        self.request_timestamps.push_back( Instant::now() );
        true
      }
      else
      {
        self.rate_limited_requests += 1;
        false
      }
    }

    /// Reset state for testing
    pub fn reset( &mut self )
    {
      self.request_timestamps.clear();
      self.total_requests = 0;
      self.rate_limited_requests = 0;
    }
  }

  /// Enhanced rate limiter executor
  #[ derive( Debug, Clone ) ]
  pub struct EnhancedRateLimiter
  {
    config : EnhancedRateLimitingConfig,
    token_bucket_state : Option< Arc< Mutex< TokenBucketState > > >,
    sliding_window_state : Option< Arc< Mutex< SlidingWindowState > > >,
  }

  impl EnhancedRateLimiter
  {
    /// Create new rate limiter with configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration validation fails.
    pub fn new( config : EnhancedRateLimitingConfig ) -> core::result::Result< Self, String >
    {
      config.validate()?;

      let ( token_bucket_state, sliding_window_state ) = match config.algorithm
      {
        RateLimitingAlgorithm::TokenBucket =>
        {
          let state = TokenBucketState::new( f64::from( config.burst_capacity ) );
          ( Some( Arc::new( Mutex::new( state ) ) ), None )
        },
        RateLimitingAlgorithm::SlidingWindow =>
        {
          let state = SlidingWindowState::new();
          ( None, Some( Arc::new( Mutex::new( state ) ) ) )
        },
      };

      Ok( Self
      {
        config,
        token_bucket_state,
        sliding_window_state,
      } )
    }

    /// Execute operation with rate limiting protection
    ///
    /// # Errors
    ///
    /// Returns an error if rate limit is exceeded or the operation itself fails.
    pub async fn execute< F, Fut, T >( &self, operation : F ) -> Result< T >
    where
      F : Fn() -> Fut,
      Fut : core::future::Future< Output = Result< T > >,
    {
      // Check if request should be allowed
      if !self.should_allow_request()
      {
        return Err( error_tools::untyped::Error::msg( "Rate limit exceeded - request rejected" ) );
      }

      // Execute the operation
      operation().await
    }

    /// Check if request should be allowed based on rate limiting
    ///
    /// # Errors
    ///
    /// Returns an error if rate limiting state access fails.
    ///
    /// # Panics
    ///
    /// Panics if mutex lock acquisition fails during state access.
    fn should_allow_request( &self ) -> bool
    {
      match self.config.algorithm
      {
        RateLimitingAlgorithm::TokenBucket =>
        {
          if let Some( state ) = &self.token_bucket_state
          {
            let mut bucket = state.lock().unwrap();
            bucket.refill_tokens( self.config.refill_rate, f64::from( self.config.burst_capacity ) );
            bucket.try_consume()
          }
          else
          {
            true // No state, allow request
          }
        },
        RateLimitingAlgorithm::SlidingWindow =>
        {
          if let Some( state ) = &self.sliding_window_state
          {
            let mut window = state.lock().unwrap();
            window.cleanup_old_timestamps( Duration::from_millis( self.config.window_duration_ms ) );
            window.try_add_request( self.config.max_requests )
          }
          else
          {
            true // No state, allow request
          }
        }
      }
    }

    /// Reset rate limiter state for testing
    ///
    /// # Panics
    ///
    /// Panics if mutex lock acquisition fails during state access.
    pub fn reset( &self )
    {
      match self.config.algorithm
      {
        RateLimitingAlgorithm::TokenBucket =>
        {
          if let Some( state ) = &self.token_bucket_state
          {
            let mut bucket = state.lock().unwrap();
            bucket.reset( f64::from( self.config.burst_capacity ) );
          }
        },
        RateLimitingAlgorithm::SlidingWindow =>
        {
          if let Some( state ) = &self.sliding_window_state
          {
            let mut window = state.lock().unwrap();
            window.reset();
          }
        }
      }
    }

    /// Get current token bucket state for testing and metrics
    ///
    /// # Panics
    ///
    /// Panics if mutex lock acquisition fails during state access.
    #[ must_use ]
    pub fn get_token_bucket_state( &self ) -> Option< TokenBucketState >
    {
      if let Some( state ) = &self.token_bucket_state
      {
        let bucket = state.lock().unwrap();
        Some( TokenBucketState
        {
          tokens : bucket.tokens,
          last_refill : bucket.last_refill,
          total_requests : bucket.total_requests,
          rate_limited_requests : bucket.rate_limited_requests,
        } )
      }
      else
      {
        None
      }
    }

    /// Get current sliding window state for testing and metrics
    ///
    /// # Panics
    ///
    /// Panics if mutex lock acquisition fails during state access.
    #[ must_use ]
    pub fn get_sliding_window_state( &self ) -> Option< SlidingWindowState >
    {
      if let Some( state ) = &self.sliding_window_state
      {
        let window = state.lock().unwrap();
        Some( SlidingWindowState
        {
          request_timestamps : window.request_timestamps.clone(),
          total_requests : window.total_requests,
          rate_limited_requests : window.rate_limited_requests,
        } )
      }
      else
      {
        None
      }
    }

    /// Get rate limiter configuration
    #[ must_use ]
    pub fn config( &self ) -> &EnhancedRateLimitingConfig
    {
      &self.config
    }
  }
}

// Provide no-op implementations when rate limiting feature is disabled
#[ cfg( not( feature = "rate_limiting" ) ) ]
/// No-op rate limiting configuration module when rate limiting feature is disabled
pub mod private
{
  /// No-op rate limiting configuration when feature is disabled
  #[ derive( Debug, Clone ) ]
  pub struct EnhancedRateLimitingConfig;

  impl EnhancedRateLimitingConfig
  {
    /// Create a new no-op configuration
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
    }
  }

  impl Default for EnhancedRateLimitingConfig
  {
    fn default() -> Self
    {
      Self
    }
  }
}

// Re-export rate limiting functionality only when feature is enabled
#[ cfg( feature = "rate_limiting" ) ]
pub use private::
{
  EnhancedRateLimitingConfig,
  RateLimitingAlgorithm,
  TokenBucketState,
  SlidingWindowState,
  EnhancedRateLimiter,
};

#[ cfg( not( feature = "rate_limiting" ) ) ]
pub use private::EnhancedRateLimitingConfig;

// Export for mod_interface
crate ::mod_interface!
{
  #[ cfg( feature = "rate_limiting" ) ]
  exposed use
  {
    EnhancedRateLimitingConfig,
    RateLimitingAlgorithm,
    TokenBucketState,
    SlidingWindowState,
    EnhancedRateLimiter,
  };

  #[ cfg( not( feature = "rate_limiting" ) ) ]
  exposed use
  {
    EnhancedRateLimitingConfig,
  };
}