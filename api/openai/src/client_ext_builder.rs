// src/client_ext_builder.rs
//! Client builder methods extension.
//!
//! This module extends the `Client` with builder pattern methods for
//! configuring caching, retry logic, circuit breakers, and rate limiting.

mod private
{
  use crate::
  {
    client ::Client,
    environment ::{ OpenaiEnvironment, EnvironmentInterface },
    error ::Result,
    diagnostics ::DiagnosticsCollector,
    request_cache ::{ ApiRequestCache, CacheConfig },
  };

  #[ cfg( feature = "retry" ) ]
  use crate::enhanced_retry::{ EnhancedRetryConfig };

  #[ cfg( feature = "circuit_breaker" ) ]
  use crate::enhanced_circuit_breaker::{ EnhancedCircuitBreakerConfig, EnhancedCircuitBreaker };

  #[ cfg( feature = "rate_limiting" ) ]
  use crate::enhanced_rate_limiting::{ EnhancedRateLimitingConfig, EnhancedRateLimiter };

  use reqwest::Client as HttpClient;
  use std::sync::Arc;

  impl< E > Client< E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static, // Add Send + Sync + 'static
  {
    /// Creates a new `Client` instance.
    ///
    /// # Arguments
    /// - `environment`: The `OpenAI` environment configuration.
    ///
    /// # Errors
    /// Returns `OpenAIError::InvalidArgument` if the API key is invalid.
    #[ inline ]
    pub fn build( environment : E ) -> Result< Self >
    {
      let headers = environment.headers()?;
      let http_client = HttpClient::builder()
        .default_headers( headers )
        .timeout( core::time::Duration::from_secs( 300 ) ) // 5 minute default timeout
        .connect_timeout( core::time::Duration::from_secs( 30 ) ) // 30 second connect timeout
        .pool_max_idle_per_host( 10 ) // Connection pooling optimization
        .pool_idle_timeout( core::time::Duration::from_secs( 90 ) ) // Keep connections alive
        .tcp_keepalive( core::time::Duration::from_secs( 60 ) ) // TCP keepalive
        .build()?;

      // Initialize diagnostics collector if config is provided
      let diagnostics = environment.diagnostics_config()
        .map( |config| Arc::new( DiagnosticsCollector::new( config.clone() ) ) );

      Ok( Self
      {
        http_client,
        environment,
        diagnostics,
        cache : None,

        // Feature-gated fields initialization
        #[ cfg( feature = "retry" ) ]
        retry_config : None,

        #[ cfg( feature = "circuit_breaker" ) ]
        circuit_breaker_config : None,
        #[ cfg( feature = "circuit_breaker" ) ]
        circuit_breaker : None,

        #[ cfg( feature = "rate_limiting" ) ]
        rate_limiting_config : None,
        #[ cfg( feature = "rate_limiting" ) ]
        rate_limiter : None,
      })
    }

    /// Enable request caching with default configuration.
    #[ inline ]
    #[ must_use ]
    pub fn with_cache( mut self ) -> Self
    {
      self.cache = Some( Arc::new( ApiRequestCache::new_api_cache() ) );
      self
    }

    /// Enable request caching with custom configuration.
    #[ inline ]
    #[ must_use ]
    pub fn with_cache_config( mut self, config : CacheConfig ) -> Self
    {
      self.cache = Some( Arc::new( ApiRequestCache::with_config( config ) ) );
      self
    }

    /// Get cache statistics if caching is enabled.
    #[ inline ]
    pub fn cache_statistics( &self ) -> Option< &crate::request_cache::CacheStatistics >
    {
      self.cache.as_ref().map( |cache| cache.statistics() )
    }

    /// Clear the cache if caching is enabled.
    #[ inline ]
    pub async fn clear_cache( &self )
    {
      if let Some( cache ) = &self.cache
      {
        cache.clear().await;
      }
    }

    /// Enable enhanced retry logic with default configuration.
    /// Only available when the `retry` feature is enabled.
    #[ cfg( feature = "retry" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn with_retry( mut self ) -> Self
    {
      self.retry_config = Some( EnhancedRetryConfig::default() );
      self
    }

    /// Enable enhanced retry logic with custom configuration.
    /// Only available when the `retry` feature is enabled.
    #[ cfg( feature = "retry" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn with_retry_config( mut self, config : EnhancedRetryConfig ) -> Self
    {
      self.retry_config = Some( config );
      self
    }

    /// Get retry configuration if enabled.
    #[ cfg( feature = "retry" ) ]
    #[ inline ]
    pub fn retry_config( &self ) -> Option< &EnhancedRetryConfig >
    {
      self.retry_config.as_ref()
    }

    /// Enable circuit breaker logic with default configuration.
    /// Only available when the `circuit_breaker` feature is enabled.
    #[ cfg( feature = "circuit_breaker" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn with_circuit_breaker( mut self ) -> Self
    {
      let config = EnhancedCircuitBreakerConfig::default();
      self.circuit_breaker_config = Some( config.clone() );

      #[ cfg( feature = "circuit_breaker" ) ]
      {
        self.circuit_breaker = EnhancedCircuitBreaker::new( config ).ok();
      }

      self
    }

    /// Enable circuit breaker logic with custom configuration.
    /// Only available when the `circuit_breaker` feature is enabled.
    #[ cfg( feature = "circuit_breaker" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn with_circuit_breaker_config( mut self, config : EnhancedCircuitBreakerConfig ) -> Self
    {
      self.circuit_breaker_config = Some( config.clone() );
      self.circuit_breaker = EnhancedCircuitBreaker::new( config ).ok();
      self
    }

    /// Get circuit breaker configuration if enabled.
    #[ cfg( feature = "circuit_breaker" ) ]
    #[ inline ]
    pub fn circuit_breaker_config( &self ) -> Option< &EnhancedCircuitBreakerConfig >
    {
      self.circuit_breaker_config.as_ref()
    }

    /// Enable rate limiting logic with default configuration.
    /// Only available when the `rate_limiting` feature is enabled.
    #[ cfg( feature = "rate_limiting" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn with_rate_limiting( mut self ) -> Self
    {
      let config = EnhancedRateLimitingConfig::default();
      self.rate_limiting_config = Some( config.clone() );
      self.rate_limiter = EnhancedRateLimiter::new( config ).ok();
      self
    }

    /// Enable rate limiting logic with custom configuration.
    /// Only available when the `rate_limiting` feature is enabled.
    #[ cfg( feature = "rate_limiting" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn with_rate_limiting_config( mut self, config : EnhancedRateLimitingConfig ) -> Self
    {
      self.rate_limiting_config = Some( config.clone() );
      self.rate_limiter = EnhancedRateLimiter::new( config ).ok();
      self
    }

    /// Get rate limiting configuration if enabled.
    #[ cfg( feature = "rate_limiting" ) ]
    #[ inline ]
    pub fn rate_limiting_config( &self ) -> Option< &EnhancedRateLimitingConfig >
    {
      self.rate_limiting_config.as_ref()
    }
  }

} // end mod private
