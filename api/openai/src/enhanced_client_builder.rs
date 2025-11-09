//! Builder for Enhanced `OpenAI` Client Configuration
//!
//! This module provides a builder pattern for configuring `EnhancedClient` instances
//! with various features like connection pooling, caching, circuit breakers, and metrics.

use mod_interface::mod_interface;

mod private
{
  use crate::
  {
    enhanced_client ::EnhancedClient,
    environment ::{ OpenaiEnvironment, EnvironmentInterface },
    connection_manager ::ConnectionConfig,
    metrics_framework ::MetricsConfig,
    error ::Result,
  };

  // Feature-gated imports
  #[ cfg( feature = "circuit_breaker" ) ]
  use crate::enhanced_circuit_breaker::EnhancedCircuitBreakerConfig;

  #[ cfg( feature = "caching" ) ]
  use crate::response_cache::CacheConfig;

  use core::time::Duration;

  /// Builder for enhanced client configuration
  #[ derive( Debug, Clone ) ]
  #[ allow( dead_code ) ]
  pub struct EnhancedClientBuilder
  {
    connection : ConnectionConfig,
    #[ cfg( feature = "caching" ) ]
    cache : Option< CacheConfig >,
    #[ cfg( not( feature = "caching" ) ) ]
    cache : Option< () >,
    #[ cfg( feature = "circuit_breaker" ) ]
    circuit_breaker : Option< EnhancedCircuitBreakerConfig >,
    #[ cfg( not( feature = "circuit_breaker" ) ) ]
    circuit_breaker : Option< () >,
    metrics : Option< MetricsConfig >,
  }

  impl EnhancedClientBuilder
  {
    /// Create new builder with default configuration
    #[ must_use ]
    #[ inline ]
    pub fn new() -> Self
    {
      Self
      {
        connection : ConnectionConfig::default(),
        cache : None,
        circuit_breaker : None,
        metrics : None,
      }
    }

    /// Set maximum connections per host
    #[ must_use ]
    #[ inline ]
    pub fn max_connections_per_host( mut self, max : usize ) -> Self
    {
      self.connection.max_connections_per_host = max;
      self
    }

    /// Set minimum connections per host
    #[ must_use ]
    #[ inline ]
    pub fn min_connections_per_host( mut self, min : usize ) -> Self
    {
      self.connection.min_connections_per_host = min;
      self
    }

    /// Set connection idle timeout
    #[ must_use ]
    #[ inline ]
    pub fn idle_timeout( mut self, timeout : Duration ) -> Self
    {
      self.connection.idle_timeout = timeout;
      self
    }

    /// Enable or disable adaptive pooling
    #[ must_use ]
    #[ inline ]
    pub fn adaptive_pooling( mut self, enabled : bool ) -> Self
    {
      self.connection.adaptive_pooling = enabled;
      self
    }

    /// Enable or disable connection warming
    #[ must_use ]
    #[ inline ]
    pub fn connection_warming( mut self, enabled : bool ) -> Self
    {
      self.connection.enable_connection_warming = enabled;
      self
    }

    /// Set health check interval
    #[ must_use ]
    #[ inline ]
    pub fn health_check_interval( mut self, interval : Duration ) -> Self
    {
      self.connection.health_check_interval = interval;
      self
    }

    /// Enable response caching with configuration
    #[ cfg( feature = "caching" ) ]
    #[ must_use ]
    #[ inline ]
    pub fn with_cache( mut self, cache_config : CacheConfig ) -> Self
    {
      self.cache = Some( cache_config );
      self
    }

    /// Enable response caching with default configuration
    #[ cfg( feature = "caching" ) ]
    #[ must_use ]
    #[ inline ]
    pub fn with_default_cache( mut self ) -> Self
    {
      self.cache = Some( CacheConfig::default() );
      self
    }

    /// Enable circuit breaker with configuration
    #[ cfg( feature = "circuit_breaker" ) ]
    #[ must_use ]
    #[ inline ]
    pub fn with_circuit_breaker( mut self, circuit_breaker_config : EnhancedCircuitBreakerConfig ) -> Self
    {
      self.circuit_breaker = Some( circuit_breaker_config );
      self
    }

    /// Enable circuit breaker with default configuration
    #[ cfg( feature = "circuit_breaker" ) ]
    #[ must_use ]
    #[ inline ]
    pub fn with_default_circuit_breaker( mut self ) -> Self
    {
      self.circuit_breaker = Some( EnhancedCircuitBreakerConfig::default() );
      self
    }

    /// Enable metrics collection with configuration
    #[ must_use ]
    #[ inline ]
    pub fn with_metrics( mut self, metrics_config : MetricsConfig ) -> Self
    {
      self.metrics = Some( metrics_config );
      self
    }

    /// Enable metrics collection with default configuration
    #[ must_use ]
    #[ inline ]
    pub fn with_default_metrics( mut self ) -> Self
    {
      self.metrics = Some( MetricsConfig::default() );
      self
    }

    /// Build enhanced client with environment and full configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the enhanced client cannot be built with the specified configuration.
    #[ inline ]
    pub fn build< E >( self, environment : E ) -> Result< EnhancedClient< E > >
    where
      E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
    {
      // Build with available features - fallback to basic builder if advanced features unavailable
      #[ cfg( all( feature = "caching", feature = "circuit_breaker" ) ) ]
      {
        EnhancedClient::build_with_full_config(
          environment,
          self.connection,
          self.cache,
          self.circuit_breaker,
          self.metrics
        )
      }

      #[ cfg( not( all( feature = "caching", feature = "circuit_breaker" ) ) ) ]
      {
        // Use basic builder when advanced features are not available
        EnhancedClient::build_with_config( environment, self.connection )
      }
    }
  }

  impl Default for EnhancedClientBuilder
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }
}

mod_interface!
{
  exposed use
  {
    EnhancedClientBuilder,
  };
}
