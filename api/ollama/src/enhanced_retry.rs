//! Enhanced retry logic for Ollama API client with exponential backoff and jitter.
//!
//! This module provides configurable retry functionality that is feature-gated
//! behind the `retry` feature. When disabled, there is zero runtime overhead.
//!
//! # Key Features
//!
//! - **Feature-Gated**: Only available when `retry` feature is enabled
//! - **Exponential Backoff**: Configurable base delay and multiplier
//! - **Jitter**: Randomized delay to prevent thundering herd
//! - **Error Classification**: Distinguish retryable from non-retryable errors
//! - **Thread-Safe**: Safe for concurrent use
//! - **Metrics Integration**: Track retry attempts and success rates
//! - **Timeout Enforcement**: Respect max elapsed time and max attempts

#[ cfg( feature = "retry" ) ]
mod private
{
  // Note : error_tools types not needed for this implementation
  use std::time::{ Duration, Instant };
  use std::sync::{ Arc, Mutex };
  use std::pin::Pin;
  use std::future::Future;

  /// Configuration for retry behavior
  #[ derive( Debug, Clone ) ]
  pub struct RetryConfig
  {
    /// Maximum number of retry attempts
    pub max_attempts : u32,
    /// Base delay in milliseconds before first retry
    pub base_delay_ms : u64,
    /// Maximum total elapsed time for all retry attempts
    pub max_elapsed_time : Duration,
    /// Maximum jitter in milliseconds to add to each delay
    pub jitter_ms : u64,
    /// Multiplier for exponential backoff (e.g., 2.0 for doubling)
    pub backoff_multiplier : f64,
    /// Whether to log retry attempts
    pub log_attempts : bool,
  }

  impl Default for RetryConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        max_attempts : 3,
        base_delay_ms : 1000, // 1 second base delay
        max_elapsed_time : Duration::from_secs( 30 ),
        jitter_ms : 500, // Up to 500ms jitter
        backoff_multiplier : 2.0, // Exponential doubling
        log_attempts : true,
      }
    }
  }

  impl RetryConfig
  {
    /// Create a new retry configuration with default values
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self::default()
    }

    /// Set maximum number of retry attempts
    #[ inline ]
    #[ must_use ]
    pub fn with_max_attempts( mut self, max_attempts : u32 ) -> Self
    {
      self.max_attempts = max_attempts;
      self
    }

    /// Set base delay in milliseconds
    #[ inline ]
    #[ must_use ]
    pub fn with_base_delay_ms( mut self, base_delay_ms : u64 ) -> Self
    {
      self.base_delay_ms = base_delay_ms;
      self
    }

    /// Set maximum elapsed time for all retry attempts
    #[ inline ]
    #[ must_use ]
    pub fn with_max_elapsed_time( mut self, max_elapsed_time : Duration ) -> Self
    {
      self.max_elapsed_time = max_elapsed_time;
      self
    }

    /// Set maximum jitter in milliseconds
    #[ inline ]
    #[ must_use ]
    pub fn with_jitter_ms( mut self, jitter_ms : u64 ) -> Self
    {
      self.jitter_ms = jitter_ms;
      self
    }

    /// Set backoff multiplier for exponential backoff
    #[ inline ]
    #[ must_use ]
    pub fn with_backoff_multiplier( mut self, backoff_multiplier : f64 ) -> Self
    {
      self.backoff_multiplier = backoff_multiplier;
      self
    }

    /// Enable or disable retry attempt logging
    #[ inline ]
    #[ must_use ]
    pub fn with_logging( mut self, log_attempts : bool ) -> Self
    {
      self.log_attempts = log_attempts;
      self
    }
  }

  /// Classification of errors for retry decisions
  #[ derive( Debug, Clone, PartialEq ) ]
  pub enum ErrorClassification
  {
    /// Error is retryable (network issues, timeouts, 5xx responses)
    Retryable,
    /// Error is not retryable (4xx client errors, authentication failures)
    NonRetryable,
    /// Operation timed out
    Timeout,
  }

  /// Metrics for retry operations
  #[ derive( Debug, Default, Clone ) ]
  pub struct RetryMetrics
  {
    /// Total number of retry attempts made
    pub total_attempts : Arc< Mutex< u64 > >,
    /// Number of operations that succeeded after retries
    pub successful_retries : Arc< Mutex< u64 > >,
    /// Number of operations that failed after all retries
    pub failed_operations : Arc< Mutex< u64 > >,
    /// Total delay time spent on retries
    pub total_delay_ms : Arc< Mutex< u64 > >,
  }

  impl RetryMetrics
  {
    /// Create new retry metrics instance
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self::default()
    }

    /// Record a retry attempt
    #[ inline ]
    pub fn record_attempt( &self )
    {
      let mut total = self.total_attempts.lock().unwrap();
      *total += 1;
    }

    /// Record a successful retry operation
    #[ inline ]
    pub fn record_success( &self )
    {
      let mut successful = self.successful_retries.lock().unwrap();
      *successful += 1;
    }

    /// Record a failed operation (after all retries exhausted)
    #[ inline ]
    pub fn record_failure( &self )
    {
      let mut failed = self.failed_operations.lock().unwrap();
      *failed += 1;
    }

    /// Record delay time spent on retry
    #[ inline ]
    pub fn record_delay( &self, delay : Duration )
    {
      let mut total_delay = self.total_delay_ms.lock().unwrap();
      *total_delay += delay.as_millis() as u64;
    }

    /// Get current retry statistics
    #[ inline ]
    #[ must_use ]
    pub fn get_stats( &self ) -> RetryStats
    {
      let total_attempts = *self.total_attempts.lock().unwrap();
      let successful_retries = *self.successful_retries.lock().unwrap();
      let failed_operations = *self.failed_operations.lock().unwrap();
      let total_delay_ms = *self.total_delay_ms.lock().unwrap();

      RetryStats
      {
        total_attempts,
        successful_retries,
        failed_operations,
        total_delay_ms,
        success_rate : if total_attempts > 0
        {
          successful_retries as f64 / total_attempts as f64
        }
        else
        {
          0.0
        },
      }
    }
  }

  /// Snapshot of retry statistics
  #[ derive( Debug, Clone ) ]
  pub struct RetryStats
  {
    /// Total retry attempts made
    pub total_attempts : u64,
    /// Operations that succeeded after retries
    pub successful_retries : u64,
    /// Operations that failed after all retries
    pub failed_operations : u64,
    /// Total delay time in milliseconds
    pub total_delay_ms : u64,
    /// Success rate (0.0 to 1.0)
    pub success_rate : f64,
  }

  /// Error classifier for determining retry eligibility
  #[ derive( Debug ) ]
  pub struct ErrorClassifier;

  impl ErrorClassifier
  {
    /// Classify an error to determine if it should be retried
    #[ inline ]
    #[ must_use ]
    pub fn classify( error_message : &str ) -> ErrorClassification
    {
      let error_lower = error_message.to_lowercase();

      // Non-retryable errors (client errors, authentication issues)
      if error_lower.contains( "400" ) ||  // Bad Request
         error_lower.contains( "401" ) ||  // Unauthorized
         error_lower.contains( "403" ) ||  // Forbidden
         error_lower.contains( "404" ) ||  // Not Found
         error_lower.contains( "405" ) ||  // Method Not Allowed
         error_lower.contains( "406" ) ||  // Not Acceptable
         error_lower.contains( "409" ) ||  // Conflict
         error_lower.contains( "410" ) ||  // Gone
         error_lower.contains( "422" ) ||  // Unprocessable Entity
         error_lower.contains( "unauthorized" ) ||
         error_lower.contains( "forbidden" ) ||
         error_lower.contains( "bad request" ) ||
         error_lower.contains( "invalid" ) && !error_lower.contains( "invalid response" )
      {
        return ErrorClassification::NonRetryable;
      }

      // Timeout errors
      if error_lower.contains( "timed out" ) ||
         error_lower.contains( "timeout" ) ||
         error_lower.contains( "deadline exceeded" )
      {
        return ErrorClassification::Timeout;
      }

      // Retryable errors (network issues, server errors)
      if error_lower.contains( "connection" ) ||
         error_lower.contains( "network" ) ||
         error_lower.contains( "dns" ) ||
         error_lower.contains( "502" ) ||  // Bad Gateway
         error_lower.contains( "503" ) ||  // Service Unavailable
         error_lower.contains( "504" ) ||  // Gateway Timeout
         error_lower.contains( "500" ) ||  // Internal Server Error
         error_lower.contains( "unreachable" ) ||
         error_lower.contains( "refused" ) ||
         error_lower.contains( "reset" ) ||
         error_lower.contains( "aborted" )
      {
        return ErrorClassification::Retryable;
      }

      // Default to retryable for unknown errors (conservative approach)
      ErrorClassification::Retryable
    }
  }

  /// Calculate delay for retry attempt with exponential backoff and jitter
  #[ inline ]
  pub fn calculate_retry_delay( attempt : u32, config : &RetryConfig ) -> Duration
  {
    // Calculate exponential backoff : base_delay * multiplier^attempt
    let base_delay_f64 = config.base_delay_ms as f64;
    let exponential_delay = base_delay_f64 * config.backoff_multiplier.powi( attempt as i32 );

    // Add jitter to prevent thundering herd effect
    let jitter = if config.jitter_ms > 0
    {
      fastrand ::u64( 0..=config.jitter_ms )
    }
    else
    {
      0
    };

    Duration::from_millis( exponential_delay as u64 + jitter )
  }

  /// Execute an operation with retry logic
  pub async fn execute_with_retries< F, T, E >(
    operation : F,
    config : RetryConfig,
    metrics : Option< &RetryMetrics >
  ) -> std::result::Result< T, E >
  where
    F: Fn() -> Pin< Box< dyn Future< Output = std::result::Result< T, E > > + Send > > + Send + Sync,
    E: std::fmt::Display + Send + Sync,
  {
    let start_time = Instant::now();
    let mut last_error = None;

    for attempt in 0..config.max_attempts
    {
      // Check if we've exceeded max elapsed time
      if start_time.elapsed() > config.max_elapsed_time
      {
        if config.log_attempts
        {
          println!( "⚠ Retry abandoned due to max elapsed time : {:?}", config.max_elapsed_time );
        }
        break;
      }

      // Record attempt in metrics
      if let Some( m ) = metrics
      {
        m.record_attempt();
      }

      match operation().await
      {
        Ok( result ) =>
        {
          if attempt > 0
          {
            // Success after retries
            if let Some( m ) = metrics
            {
              m.record_success();
            }
            if config.log_attempts
            {
              println!( "✓ Operation succeeded after {} retry attempts", attempt );
            }
          }
          return Ok( result );
        }
        Err( error ) =>
        {
          let error_str = error.to_string();
          let classification = ErrorClassifier::classify( &error_str );

          if config.log_attempts
          {
            println!( "⚠ Attempt {} failed : {} (classification : {:?})", attempt + 1, error_str, classification );
          }

          // Don't retry non-retryable errors
          if classification == ErrorClassification::NonRetryable
          {
            if config.log_attempts
            {
              println!( "⚠ Error classified as non-retryable, aborting retries" );
            }
            return Err( error );
          }

          last_error = Some( error );

          // Don't delay on the last attempt
          if attempt < config.max_attempts - 1
          {
            let delay = calculate_retry_delay( attempt, &config );

            if config.log_attempts
            {
              println!( "⏳ Waiting {:?} before retry attempt {}", delay, attempt + 2 );
            }

            // Record delay in metrics
            if let Some( m ) = metrics
            {
              m.record_delay( delay );
            }

            tokio ::time::sleep( delay ).await;
          }
        }
      }
    }

    // All retries exhausted
    if let Some( m ) = metrics
    {
      m.record_failure();
    }

    if config.log_attempts
    {
      println!( "⚠ All {} retry attempts exhausted", config.max_attempts );
    }

    // Return the last error
    Err( last_error.unwrap() )
  }

  /// Wrapper for HTTP operations with retry logic
  #[ derive( Debug, Clone ) ]
  pub struct RetryableHttpClient
  {
    /// Retry configuration
    pub config : Option< RetryConfig >,
    /// Retry metrics
    pub metrics : Arc< RetryMetrics >,
  }

  impl RetryableHttpClient
  {
    /// Create a new retryable HTTP client wrapper
    #[ inline ]
    #[ must_use ]
    pub fn new( config : Option< RetryConfig > ) -> Self
    {
      Self
      {
        config,
        metrics : Arc::new( RetryMetrics::new() ),
      }
    }

    /// Execute an HTTP operation with retries if configured
    pub async fn execute< F, T, E >( &self, operation : F ) -> std::result::Result< T, E >
    where
      F: Fn() -> Pin< Box< dyn Future< Output = std::result::Result< T, E > > + Send > > + Send + Sync,
      E: std::fmt::Display + Send + Sync,
    {
      match &self.config
      {
        Some( config ) =>
        {
          execute_with_retries( operation, config.clone(), Some( &self.metrics ) ).await
        }
        None =>
        {
          // No retry configuration, execute once
          operation().await
        }
      }
    }

    /// Get retry metrics
    #[ inline ]
    #[ must_use ]
    pub fn get_metrics( &self ) -> RetryStats
    {
      self.metrics.get_stats()
    }

    /// Reset retry metrics
    #[ inline ]
    pub fn reset_metrics( &self )
    {
      *self.metrics.total_attempts.lock().unwrap() = 0;
      *self.metrics.successful_retries.lock().unwrap() = 0;
      *self.metrics.failed_operations.lock().unwrap() = 0;
      *self.metrics.total_delay_ms.lock().unwrap() = 0;
    }
  }

  impl Default for RetryableHttpClient
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new( None )
    }
  }

  /// Convenience function to create a retry operation from a closure
  #[ inline ]
  pub fn retry_operation< F, T, E >( operation : F ) -> impl Fn() -> Pin< Box< dyn Future< Output = std::result::Result< T, E > > + Send > >
  where
    F: Fn() -> Pin< Box< dyn Future< Output = std::result::Result< T, E > > + Send > > + Send + Sync + Clone,
  {
    move ||
    {
      let op = operation.clone();
      op()
    }
  }

  /// Test utilities for retry logic
  #[ cfg( test ) ]
  pub mod test_utils
  {
    use super::*;

    /// Create a test operation that fails N times then succeeds
    pub fn test_operation_with_failures(
      failure_count : u32,
      success_message : String
    ) -> impl Fn() -> Pin< Box< dyn Future< Output = std::result::Result< String, String > > + Send > >
    {
      use std::sync::atomic::{ AtomicU32, Ordering };
      let attempt_counter = Arc::new( AtomicU32::new( 0 ) );

      move ||
      {
        let counter = Arc::clone( &attempt_counter );
        let msg = success_message.clone();
        Box::pin( async move
        {
          let current_attempt = counter.fetch_add( 1, Ordering::SeqCst ) + 1;

          if current_attempt <= failure_count
          {
            Err( format!( "Test failure on attempt {current_attempt}" ) )
          }
          else
          {
            Ok( format!( "{msg} (succeeded on attempt {current_attempt})" ) )
          }
        } )
      }
    }

    /// Create a test operation that always fails with a specific error
    pub fn test_operation_always_fails(
      error_message : String
    ) -> impl Fn() -> Pin< Box< dyn Future< Output = std::result::Result< String, String > > + Send > >
    {
      move ||
      {
        let error = error_message.clone();
        Box::pin( async move
        {
          Err( error )
        } )
      }
    }
  }
}

#[ cfg( feature = "retry" ) ]
crate ::mod_interface!
{
  exposed use private::RetryConfig;
  exposed use private::ErrorClassification;
  exposed use private::RetryMetrics;
  exposed use private::RetryStats;
  exposed use private::ErrorClassifier;
  exposed use private::RetryableHttpClient;
  exposed use private::execute_with_retries;
  exposed use private::calculate_retry_delay;
  exposed use private::retry_operation;

  #[ cfg( test ) ]
  exposed use private::test_utils;
}