//! Rate limiting feature configuration setters for ClientBuilder.

use core::time::Duration;
use super::ClientBuilder;

impl ClientBuilder
{
  /// Enables or disables rate limiting functionality.
  ///
  /// When enabled, the client will limit request rates according to
  /// configured parameters to prevent overwhelming the API.
  #[ must_use ]
  #[ inline ]
  pub fn enable_rate_limiting( mut self, enable : bool ) -> Self
  {
    self.enable_rate_limiting = enable;
    self
  }

  /// Sets the rate limit in requests per second.
  ///
  /// Controls how many requests are allowed per second.
  /// Must be greater than 0.0.
  ///
  /// # Arguments
  ///
  /// * `rate` - Number of requests allowed per second
  #[ must_use ]
  #[ inline ]
  pub fn rate_limit_requests_per_second( mut self, rate : f64 ) -> Self
  {
    self.rate_limit_requests_per_second = rate;
    self
  }

  /// Sets the rate limiting algorithm.
  ///
  /// Available algorithms:
  /// - "`token_bucket"`: Token bucket with burst capacity
  /// - "`sliding_window"`: Sliding window algorithm
  /// - "adaptive": Adaptive rate limiting based on response times
  ///
  /// # Arguments
  ///
  /// * `algorithm` - Rate limiting algorithm name
  #[ must_use ]
  #[ inline ]
  pub fn rate_limit_algorithm( mut self, algorithm : &str ) -> Self
  {
    self.rate_limit_algorithm = algorithm.to_string();
    self
  }

  /// Sets the token bucket size for burst capacity.
  ///
  /// Only applies when using "`token_bucket`" algorithm.
  /// Must be greater than 0.
  ///
  /// # Arguments
  ///
  /// * `size` - Maximum number of tokens in the bucket
  #[ must_use ]
  #[ inline ]
  pub fn rate_limit_bucket_size( mut self, size : usize ) -> Self
  {
    self.rate_limit_bucket_size = size;
    self
  }

  /// Enables or disables rate limiting metrics collection.
  ///
  /// When enabled, collects metrics about:
  /// - Request rates and patterns
  /// - Rate limiting violations
  /// - Token bucket levels
  /// - Average response times
  #[ must_use ]
  #[ inline ]
  pub fn enable_rate_limiting_metrics( mut self, enable_metrics : bool ) -> Self
  {
    self.enable_rate_limiting_metrics = enable_metrics;
    self
  }

  // Placeholder methods for test compilation - these would be implemented with full feature
  /// Set the token refill rate for token bucket algorithm (placeholder for future implementation).
  #[ must_use ]
  #[ inline ]
  pub fn rate_limit_tokens_per_second( self, _tokens : f64 ) -> Self
  {
    self
  }

  /// Set the request limit per sliding window (placeholder for future implementation).
  #[ must_use ]
  #[ inline ]
  pub fn rate_limit_requests_per_window( self, _requests : usize ) -> Self
  {
    self
  }

  /// Set the sliding window size duration (placeholder for future implementation).
  #[ must_use ]
  #[ inline ]
  pub fn rate_limit_window_size( self, _window : Duration ) -> Self
  {
    self
  }

  /// Enable per-endpoint rate limiting (placeholder for future implementation).
  #[ must_use ]
  #[ inline ]
  pub fn rate_limit_per_endpoint( self, _per_endpoint : bool ) -> Self
  {
    self
  }

  /// Set backoff strategy for rate limiting violations (placeholder for future implementation).
  #[ must_use ]
  #[ inline ]
  pub fn rate_limit_backoff_strategy( self, _strategy : &str ) -> Self
  {
    self
  }

  /// Set maximum backoff duration for rate limiting (placeholder for future implementation).
  #[ must_use ]
  #[ inline ]
  pub fn rate_limit_max_backoff( self, _max_backoff : Duration ) -> Self
  {
    self
  }

  /// Enable priority queue for request handling (placeholder for future implementation).
  #[ must_use ]
  #[ inline ]
  pub fn enable_priority_queues( self, _enable : bool ) -> Self
  {
    self
  }

  /// Set target latency for adaptive rate limiting (placeholder for future implementation).
  #[ must_use ]
  #[ inline ]
  pub fn rate_limit_adaptive_target_latency( self, _target : Duration ) -> Self
  {
    self
  }

  /// Set adjustment factor for adaptive rate limiting (placeholder for future implementation).
  #[ must_use ]
  #[ inline ]
  pub fn rate_limit_adaptive_adjustment_factor( self, _factor : f64 ) -> Self
  {
    self
  }
}
