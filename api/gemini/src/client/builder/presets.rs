//! Builder preset configurations for common use cases.

use core::time::Duration;
use super::ClientBuilder;

impl ClientBuilder
{
  /// Create a development configuration preset with common development settings.
  ///
  /// This preset includes:
  /// - Extended timeout for debugging
  /// - Reduced rate limits for development
  /// - Disabled circuit breaker for easier testing
  /// - Enhanced logging when available
  #[ must_use ]
  #[ inline ]
  pub fn development_preset( mut self ) -> Self
  {
    self.timeout = Duration::from_secs( 120 ); // Extended timeout for debugging

    #[ cfg( feature = "retry" ) ]
    {
      self.max_retries = 1; // Faster failures in development
      self.base_delay = Duration::from_millis( 50 ); // Shorter delays
    }

    #[ cfg( feature = "circuit_breaker" ) ]
    {
      self.enable_circuit_breaker = false; // Disable circuit breaker for easier testing
    }

    #[ cfg( feature = "rate_limiting" ) ]
    {
      self.enable_rate_limiting = false; // Disable rate limiting in development
    }

    self
  }

  /// Create a production configuration preset with optimized production settings.
  ///
  /// This preset includes:
  /// - Conservative timeouts for stability
  /// - Aggressive retry logic
  /// - Circuit breaker enabled
  /// - Rate limiting enabled
  /// - Caching enabled for performance
  #[ must_use ]
  #[ inline ]
  pub fn production_preset( mut self ) -> Self
  {
    self.timeout = Duration::from_secs( 30 ); // Conservative timeout

    #[ cfg( feature = "retry" ) ]
    {
      self.max_retries = 5; // Aggressive retries
      self.base_delay = Duration::from_millis( 100 );
      self.max_delay = Duration::from_secs( 10 );
      self.enable_jitter = true;
      self.backoff_multiplier = 2.0;
      self.enable_retry_metrics = true;
    }

    #[ cfg( feature = "circuit_breaker" ) ]
    {
      self.enable_circuit_breaker = true;
      self.circuit_breaker_failure_threshold = 3;
      self.circuit_breaker_success_threshold = 2;
      self.circuit_breaker_timeout = Duration::from_secs( 30 );
      self.enable_circuit_breaker_metrics = true;
    }

    #[ cfg( feature = "caching" ) ]
    {
      self.enable_request_cache = true;
      self.cache_ttl = Duration::from_secs( 300 );
      self.cache_max_size = 1000;
      self.enable_cache_metrics = true;
    }

    #[ cfg( feature = "rate_limiting" ) ]
    {
      self.enable_rate_limiting = true;
      self.rate_limit_requests_per_second = 5.0; // Conservative rate limit
      self.rate_limit_algorithm = "token_bucket".to_string();
      self.enable_rate_limiting_metrics = true;
    }

    self
  }

  /// Create a high-performance configuration preset optimized for throughput.
  ///
  /// This preset includes:
  /// - Shorter timeouts for faster failures
  /// - Minimal retry logic
  /// - Aggressive rate limits
  /// - Optimized caching
  #[ must_use ]
  #[ inline ]
  pub fn high_performance_preset( mut self ) -> Self
  {
    self.timeout = Duration::from_secs( 10 ); // Short timeout for high throughput

    #[ cfg( feature = "retry" ) ]
    {
      self.max_retries = 1; // Minimal retries for speed
      self.base_delay = Duration::from_millis( 10 );
      self.max_delay = Duration::from_secs( 1 );
      self.enable_jitter = false; // Disable jitter for consistency
      self.backoff_multiplier = 1.5;
    }

    #[ cfg( feature = "circuit_breaker" ) ]
    {
      self.enable_circuit_breaker = true;
      self.circuit_breaker_failure_threshold = 10; // Higher threshold
      self.circuit_breaker_success_threshold = 1; // Quick recovery
      self.circuit_breaker_timeout = Duration::from_secs( 5 );
    }

    #[ cfg( feature = "caching" ) ]
    {
      self.enable_request_cache = true;
      self.cache_ttl = Duration::from_secs( 60 ); // Shorter TTL for freshness
      self.cache_max_size = 5000; // Larger cache
      self.enable_cache_metrics = true;
    }

    #[ cfg( feature = "rate_limiting" ) ]
    {
      self.enable_rate_limiting = true;
      self.rate_limit_requests_per_second = 50.0; // High rate limit
      self.rate_limit_algorithm = "adaptive".to_string();
      self.enable_rate_limiting_metrics = true;
    }

    self
  }

  /// Configure the client for testing with sensible test defaults.
  ///
  /// This preset includes:
  /// - Very short timeouts for fast test execution
  /// - No retries for predictable behavior
  /// - All features disabled by default
  /// - Deterministic configuration
  #[ must_use ]
  #[ inline ]
  pub fn testing_preset( mut self ) -> Self
  {
    self.timeout = Duration::from_millis( 100 ); // Very short timeout for tests

    #[ cfg( feature = "retry" ) ]
    {
      self.max_retries = 0; // No retries in tests
      self.base_delay = Duration::from_millis( 1 );
      self.max_delay = Duration::from_millis( 2 ); // Must be greater than base_delay
      self.enable_jitter = false; // Deterministic behavior
      self.backoff_multiplier = 1.1; // Minimal but valid backoff multiplier
      self.enable_retry_metrics = false;
    }

    #[ cfg( feature = "circuit_breaker" ) ]
    {
      self.enable_circuit_breaker = false; // Disable for predictable tests
    }

    #[ cfg( feature = "caching" ) ]
    {
      self.enable_request_cache = false; // Disable caching in tests
    }

    #[ cfg( feature = "rate_limiting" ) ]
    {
      self.enable_rate_limiting = false; // Disable rate limiting in tests
    }

    self
  }

  /// Apply conditional configuration based on environment or runtime conditions.
  ///
  /// This method allows for dynamic configuration based on external factors
  /// while maintaining the builder pattern's fluent interface.
  ///
  /// # Arguments
  ///
  /// * `condition` - Boolean condition to evaluate
  /// * `configure_fn` - Function to apply configuration when condition is true
  #[ must_use ]
  #[ inline ]
  pub fn when< F >( self, condition : bool, configure_fn : F ) -> Self
  where
    F : FnOnce( ClientBuilder ) -> ClientBuilder,
  {
    if condition
    {
      configure_fn( self )
    }
    else
    {
      self
    }
  }

  /// Apply configuration from environment variables with fallbacks.
  ///
  /// This method reads common configuration from environment variables
  /// and applies them to the builder, maintaining defaults when variables
  /// are not present.
  #[ must_use ]
  #[ inline ]
  pub fn from_environment( mut self ) -> Self
  {
    // Read timeout from environment
    if let Ok( timeout_str ) = std::env::var( "GEMINI_TIMEOUT_SECONDS" )
    {
      if let Ok( timeout_secs ) = timeout_str.parse::< u64 >()
      {
        self.timeout = Duration::from_secs( timeout_secs );
      }
    }

    // Read base URL from environment
    if let Ok( base_url ) = std::env::var( "GEMINI_BASE_URL" )
    {
      if !base_url.is_empty()
      {
        self.base_url = base_url;
      }
    }

    // Read feature configurations from environment
    #[ cfg( feature = "retry" ) ]
    {
      if let Ok( max_retries_str ) = std::env::var( "GEMINI_MAX_RETRIES" )
      {
        if let Ok( max_retries ) = max_retries_str.parse::< u32 >()
        {
          self.max_retries = max_retries;
        }
      }
    }

    #[ cfg( feature = "rate_limiting" ) ]
    {
      if let Ok( rps_str ) = std::env::var( "GEMINI_RATE_LIMIT_RPS" )
      {
        if let Ok( rps ) = rps_str.parse::< f64 >()
        {
          if rps > 0.0
          {
            self.enable_rate_limiting = true;
            self.rate_limit_requests_per_second = rps;
          }
        }
      }
    }

    self
  }
}
