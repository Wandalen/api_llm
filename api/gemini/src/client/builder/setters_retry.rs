//! Retry feature configuration setters for ClientBuilder.

use core::time::Duration;
use super::ClientBuilder;

impl ClientBuilder
{
  /// Sets the maximum number of retry attempts for failed requests.
  #[ must_use ]
  #[ inline ]
  pub fn max_retries( mut self, max_retries : u32 ) -> Self
  {
    self.max_retries = max_retries;
    self
  }

  /// Sets the base delay for exponential backoff retry logic.
  #[ must_use ]
  #[ inline ]
  pub fn base_delay( mut self, base_delay : Duration ) -> Self
  {
    self.base_delay = base_delay;
    self
  }

  /// Sets the maximum delay for exponential backoff retry logic.
  #[ must_use ]
  #[ inline ]
  pub fn max_delay( mut self, max_delay : Duration ) -> Self
  {
    self.max_delay = max_delay;
    self
  }

  /// Enables or disables jitter in retry delays.
  #[ must_use ]
  #[ inline ]
  pub fn enable_jitter( mut self, enable_jitter : bool ) -> Self
  {
    self.enable_jitter = enable_jitter;
    self
  }

  /// Sets the timeout for individual requests (separate from retry logic).
  #[ must_use ]
  #[ inline ]
  pub fn request_timeout( mut self, request_timeout : Duration ) -> Self
  {
    self.request_timeout = Some( request_timeout );
    self
  }

  /// Sets the backoff multiplier for exponential backoff retry strategy.
  ///
  /// This multiplier determines how much the delay increases between retry attempts.
  /// A value of 2.0 doubles the delay each time, 1.5 increases by 50%, etc.
  ///
  /// # Arguments
  ///
  /// * `multiplier` - The multiplier for exponential backoff (must be > 1.0)
  ///
  /// # Panics
  ///
  /// Panics if multiplier is not greater than 1.0.
  #[ must_use ]
  #[ inline ]
  pub fn backoff_multiplier( mut self, multiplier : f64 ) -> Self
  {
    assert!( multiplier > 1.0, "Backoff multiplier must be greater than 1.0, got : {multiplier}" );
    self.backoff_multiplier = multiplier;
    self
  }

  /// Enables or disables retry metrics collection.
  ///
  /// When enabled, the client will collect metrics about retry attempts including:
  /// - Number of retry attempts per request
  /// - Total retry time
  /// - Success/failure rates
  /// - Backoff timing measurements
  #[ must_use ]
  #[ inline ]
  pub fn enable_retry_metrics( mut self, enable_metrics : bool ) -> Self
  {
    self.enable_retry_metrics = enable_metrics;
    self
  }

  /// Sets the maximum elapsed time for retry attempts.
  ///
  /// If specified, retry attempts will stop after this total time has elapsed,
  /// regardless of the number of attempts. If None, only `max_retries` will limit attempts.
  #[ must_use ]
  #[ inline ]
  pub fn max_elapsed_time( mut self, max_elapsed_time : Duration ) -> Self
  {
    self.max_elapsed_time = Some( max_elapsed_time );
    self
  }
}
