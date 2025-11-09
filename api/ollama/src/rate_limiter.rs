//! Rate limiting implementation for controlling request rates.

#[ cfg( feature = "rate_limiting" ) ]
mod private
{
  use core::time::Duration;
  use std::collections::VecDeque;
  use std::sync::{ Arc, Mutex };
  use std::time::Instant;
  use error_tools::untyped::{ format_err, Result };

  /// Configuration for enhanced rate limiting behavior
  #[ derive( Debug, Clone ) ]
  pub struct RateLimitingConfig
  {
    /// Maximum number of requests per time window
    max_requests : u32,
    /// Time window duration in milliseconds
    window_duration_ms : u64,
    /// Maximum burst capacity (token bucket only)
    burst_capacity : u32,
    /// Token refill rate per second (token bucket only)
    refill_rate : f64,
    /// Rate limiting algorithm to use
    algorithm : RateLimitingAlgorithm,
    /// Request timeout when rate limit exceeded
    timeout_ms : u64,
    /// Whether to enable per-endpoint rate limiting
    per_endpoint : bool,
  }

  /// Rate limiting algorithm enumeration
  #[ derive( Debug, Clone, PartialEq ) ]
  pub enum RateLimitingAlgorithm
  {
    /// Token bucket algorithm with refill rate and burst capacity
    TokenBucket,
    /// Sliding window algorithm with request timestamps
    SlidingWindow,
  }

  /// Enhanced rate limiter implementation
  #[ derive( Debug, Clone ) ]
  pub struct RateLimiter
  {
    config : RateLimitingConfig,
    token_bucket_state : Option< Arc< Mutex< TokenBucketState > > >,
    sliding_window_state : Option< Arc< Mutex< SlidingWindowState > > >,
  }

  /// Token bucket rate limiter state
  #[ derive( Debug ) ]
  struct TokenBucketState
  {
    /// Current number of tokens available
    tokens : f64,
    /// Last time tokens were refilled
    last_refill : Instant,
    /// Total number of requests processed
    total_requests : u64,
    /// Total number of rate limited requests
    rate_limited_requests : u64,
  }

  /// Sliding window rate limiter state
  #[ derive( Debug ) ]
  struct SlidingWindowState
  {
    /// Request timestamps within the current window
    request_timestamps : VecDeque< Instant >,
    /// Total number of requests processed
    total_requests : u64,
    /// Total number of rate limited requests
    rate_limited_requests : u64,
  }

  impl TokenBucketState
  {
    #[ inline ]
    fn new( initial_tokens : f64 ) -> Self
    {
      Self
      {
        tokens : initial_tokens,
        last_refill : Instant::now(),
        total_requests : 0,
        rate_limited_requests : 0,
      }
    }
  }

  impl SlidingWindowState
  {
    #[ inline ]
    fn new() -> Self
    {
      Self
      {
        request_timestamps : VecDeque::new(),
        total_requests : 0,
        rate_limited_requests : 0,
      }
    }
  }

  impl RateLimitingConfig
  {
    /// Create a new rate limiting configuration with default values
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        max_requests : 100,
        window_duration_ms : 60000, // 1 minute
        burst_capacity : 10,
        refill_rate : 1.67, // ~100 requests per minute
        algorithm : RateLimitingAlgorithm::TokenBucket,
        timeout_ms : 5000,
        per_endpoint : false,
      }
    }

    /// Set maximum requests per window
    #[ inline ]
    #[ must_use ]
    pub fn with_max_requests( mut self, max_requests : u32 ) -> Self
    {
      self.max_requests = max_requests;
      self
    }

    /// Set window duration in milliseconds
    #[ inline ]
    #[ must_use ]
    pub fn with_window_duration( mut self, duration_ms : u64 ) -> Self
    {
      self.window_duration_ms = duration_ms;
      self
    }

    /// Set burst capacity for token bucket
    #[ inline ]
    #[ must_use ]
    pub fn with_burst_capacity( mut self, capacity : u32 ) -> Self
    {
      self.burst_capacity = capacity;
      self
    }

    /// Set token refill rate per second
    #[ inline ]
    #[ must_use ]
    pub fn with_refill_rate( mut self, rate : f64 ) -> Self
    {
      self.refill_rate = rate;
      self
    }

    /// Set rate limiting algorithm
    #[ inline ]
    #[ must_use ]
    pub fn with_algorithm( mut self, algorithm : RateLimitingAlgorithm ) -> Self
    {
      self.algorithm = algorithm;
      self
    }

    /// Set request timeout when rate limit exceeded
    #[ inline ]
    #[ must_use ]
    pub fn with_timeout( mut self, timeout_ms : u64 ) -> Self
    {
      self.timeout_ms = timeout_ms;
      self
    }

    /// Enable per-endpoint rate limiting
    #[ inline ]
    #[ must_use ]
    pub fn with_per_endpoint( mut self, per_endpoint : bool ) -> Self
    {
      self.per_endpoint = per_endpoint;
      self
    }

    /// Validate configuration parameters
    #[ inline ]
    pub fn validate( &self ) -> std::result::Result< (), String >
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

      Ok( () )
    }

    /// Get maximum requests per window
    #[ inline ]
    #[ must_use ]
    pub fn max_requests( &self ) -> u32
    {
      self.max_requests
    }

    /// Get window duration in milliseconds
    #[ inline ]
    #[ must_use ]
    pub fn window_duration_ms( &self ) -> u64
    {
      self.window_duration_ms
    }

    /// Get burst capacity
    #[ inline ]
    #[ must_use ]
    pub fn burst_capacity( &self ) -> u32
    {
      self.burst_capacity
    }

    /// Get token refill rate
    #[ inline ]
    #[ must_use ]
    pub fn refill_rate( &self ) -> f64
    {
      self.refill_rate
    }

    /// Get the rate limiting algorithm
    #[ inline ]
    #[ must_use ]
    pub fn algorithm( &self ) -> &RateLimitingAlgorithm
    {
      &self.algorithm
    }

    /// Get request timeout when rate limit exceeded
    #[ inline ]
    #[ must_use ]
    pub fn timeout_ms( &self ) -> u64
    {
      self.timeout_ms
    }

    /// Check if per-endpoint rate limiting is enabled
    #[ inline ]
    #[ must_use ]
    pub fn is_per_endpoint( &self ) -> bool
    {
      self.per_endpoint
    }
  }

  impl Default for RateLimitingConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  impl RateLimiter
  {
    /// Create new rate limiter with configuration
    #[ inline ]
    pub fn new( config : RateLimitingConfig ) -> Result< Self >
    {
      config.validate().map_err( |e| format_err!( "Rate limiting configuration validation failed : {}", e ) )?;

      let ( token_bucket_state, sliding_window_state ) = match config.algorithm
      {
        RateLimitingAlgorithm::TokenBucket =>
        {
          let state = Arc::new( Mutex::new( TokenBucketState::new( config.burst_capacity as f64 ) ) );
          ( Some( state ), None )
        },
        RateLimitingAlgorithm::SlidingWindow =>
        {
          let state = Arc::new( Mutex::new( SlidingWindowState::new() ) );
          ( None, Some( state ) )
        },
      };

      Ok( Self
      {
        config,
        token_bucket_state,
        sliding_window_state,
      } )
    }

    /// Get the rate limiting configuration
    #[ inline ]
    #[ must_use ]
    pub fn config( &self ) -> &RateLimitingConfig
    {
      &self.config
    }

    /// Check if a request should be allowed
    #[ inline ]
    #[ must_use ]
    pub fn should_allow_request( &self ) -> bool
    {
      match self.config.algorithm
      {
        RateLimitingAlgorithm::TokenBucket =>
        {
          if let Some( ref state ) = self.token_bucket_state
          {
            let mut bucket = state.lock().unwrap();

            // Refill tokens based on time elapsed
            let now = Instant::now();
            let elapsed = now.duration_since( bucket.last_refill ).as_secs_f64();
            let tokens_to_add = elapsed * self.config.refill_rate;
            bucket.tokens = ( bucket.tokens + tokens_to_add ).min( self.config.burst_capacity as f64 );
            bucket.last_refill = now;

            // Check if we have tokens available
            bucket.total_requests += 1;
            if bucket.tokens >= 1.0
            {
              bucket.tokens -= 1.0;
              true
            }
            else
            {
              bucket.rate_limited_requests += 1;
              false
            }
          }
          else
          {
            // No state means allow by default
            true
          }
        },
        RateLimitingAlgorithm::SlidingWindow =>
        {
          if let Some( ref state ) = self.sliding_window_state
          {
            let mut window = state.lock().unwrap();
            let now = Instant::now();
            let window_duration = Duration::from_millis( self.config.window_duration_ms );

            // Remove expired timestamps
            while let Some( &timestamp ) = window.request_timestamps.front()
            {
              if now.duration_since( timestamp ) > window_duration
              {
                window.request_timestamps.pop_front();
              }
              else
              {
                break;
              }
            }

            // Check if we're within the limit
            window.total_requests += 1;
            if window.request_timestamps.len() < self.config.max_requests as usize
            {
              window.request_timestamps.push_back( now );
              true
            }
            else
            {
              window.rate_limited_requests += 1;
              false
            }
          }
          else
          {
            // No state means allow by default
            true
          }
        },
      }
    }

    /// Reset the rate limiter state
    #[ inline ]
    pub fn reset( &self )
    {
      match self.config.algorithm
      {
        RateLimitingAlgorithm::TokenBucket =>
        {
          if let Some( ref state ) = self.token_bucket_state
          {
            let mut bucket = state.lock().unwrap();
            *bucket = TokenBucketState::new( self.config.burst_capacity as f64 );
          }
        },
        RateLimitingAlgorithm::SlidingWindow =>
        {
          if let Some( ref state ) = self.sliding_window_state
          {
            let mut window = state.lock().unwrap();
            *window = SlidingWindowState::new();
          }
        }
      }
    }
  }
}

#[ cfg( feature = "rate_limiting" ) ]
crate ::mod_interface!
{
  exposed use private::RateLimiter;
  exposed use private::RateLimitingConfig;
  exposed use private::RateLimitingAlgorithm;
}
