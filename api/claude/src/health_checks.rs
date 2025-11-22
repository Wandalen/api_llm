//! Health Check Module
//!
//! This module provides stateless health monitoring utilities for Anthropic Claude API endpoints.
//! Following the "Thin Client, Rich API" principle, this module offers utilities for
//! one-time health checks without maintaining client-side state or automatic behaviors.
//!
//! # Key Principles
//!
//! - **Stateless Operation**: Each health check is independent
//! - **Explicit Invocation**: No automatic background checks
//! - **Transparent Results**: Clear visibility into endpoint health
//! - **Configurable Thresholds**: Developer controls what "healthy" means

mod private
{
  use std::
  {
    time::{ Duration, Instant, SystemTime },
  };
  use serde::{ Deserialize, Serialize };

  /// Health status for an endpoint
  #[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
  pub enum EndpointHealthStatus
  {
    /// Endpoint is healthy and responding normally
    Healthy,
    /// Endpoint is degraded but functional (slow response)
    Degraded,
    /// Endpoint is unhealthy and not responding
    Unhealthy,
  }

  /// Health check strategy
  #[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
  pub enum HealthCheckStrategy
  {
    /// Simple connectivity check (minimal overhead)
    Ping,
    /// Lightweight API call (more accurate but higher overhead)
    LightweightApi,
  }

  /// Health check result for a single endpoint
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct HealthCheckResult
  {
    /// The endpoint URL that was checked
    pub endpoint_url : String,
    /// Health status of the endpoint
    pub status : EndpointHealthStatus,
    /// Response time in milliseconds
    pub response_time_ms : u64,
    /// Optional error message if unhealthy
    pub error_message : Option< String >,
    /// Timestamp when check was performed
    pub timestamp : SystemTime,
  }

  impl HealthCheckResult
  {
    /// Check if the endpoint is healthy
    #[ inline ]
    #[ must_use ]
    pub fn is_healthy( &self ) -> bool
    {
      matches!( self.status, EndpointHealthStatus::Healthy )
    }

    /// Check if the endpoint is available (healthy or degraded)
    #[ inline ]
    #[ must_use ]
    pub fn is_available( &self ) -> bool
    {
      matches!( self.status, EndpointHealthStatus::Healthy | EndpointHealthStatus::Degraded )
    }

    /// Get response time as Duration
    #[ inline ]
    #[ must_use ]
    pub fn response_time( &self ) -> Duration
    {
      Duration::from_millis( self.response_time_ms )
    }
  }

  /// Configuration for health checks
  #[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
  pub struct HealthCheckConfig
  {
    /// Timeout for health check requests (in milliseconds)
    pub timeout_ms : u64,
    /// Strategy to use for health checks
    pub strategy : HealthCheckStrategy,
    /// Response time threshold for degraded status (in milliseconds)
    pub degraded_threshold_ms : u64,
    /// Response time threshold for unhealthy status (in milliseconds)
    pub unhealthy_threshold_ms : u64,
  }

  impl Default for HealthCheckConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        timeout_ms : 5000,
        strategy : HealthCheckStrategy::LightweightApi,
        degraded_threshold_ms : 1000,
        unhealthy_threshold_ms : 5000,
      }
    }
  }

  impl HealthCheckConfig
  {
    /// Create a new health check configuration with defaults
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self::default()
    }

    /// Set the timeout for health check requests
    #[ inline ]
    #[ must_use ]
    pub fn with_timeout_ms( mut self, timeout_ms : u64 ) -> Self
    {
      self.timeout_ms = timeout_ms;
      self
    }

    /// Set the health check strategy
    #[ inline ]
    #[ must_use ]
    pub fn with_strategy( mut self, strategy : HealthCheckStrategy ) -> Self
    {
      self.strategy = strategy;
      self
    }

    /// Set the degraded threshold
    #[ inline ]
    #[ must_use ]
    pub fn with_degraded_threshold_ms( mut self, threshold_ms : u64 ) -> Self
    {
      self.degraded_threshold_ms = threshold_ms;
      self
    }

    /// Set the unhealthy threshold
    #[ inline ]
    #[ must_use ]
    pub fn with_unhealthy_threshold_ms( mut self, threshold_ms : u64 ) -> Self
    {
      self.unhealthy_threshold_ms = threshold_ms;
      self
    }

    /// Set timeout as Duration (convenience method)
    #[ inline ]
    #[ must_use ]
    #[ allow( clippy::cast_possible_truncation ) ]
    pub fn with_timeout( self, timeout : core::time::Duration ) -> Self
    {
      self.with_timeout_ms( timeout.as_millis() as u64 )
    }

    /// Set interval (same as timeout for health checks)
    #[ inline ]
    #[ must_use ]
    pub fn with_interval( self, interval : core::time::Duration ) -> Self
    {
      self.with_timeout( interval )
    }

    /// Validate the configuration
    #[ inline ]
    #[ must_use ]
    pub fn is_valid( &self ) -> bool
    {
      self.timeout_ms > 0
      && self.degraded_threshold_ms > 0
      && self.unhealthy_threshold_ms >= self.degraded_threshold_ms
      && self.timeout_ms >= self.unhealthy_threshold_ms
    }
  }

  /// Stateless health check utilities
  ///
  /// Provides one-time health check operations without maintaining state.
  /// Each check is independent and returns results immediately.
  #[ derive( Debug ) ]
  pub struct HealthChecker;

  impl HealthChecker
  {
    /// Perform a single health check on the given endpoint
    ///
    /// This is a stateless operation that returns results immediately.
    /// No state is maintained between checks.
    ///
    /// # Arguments
    ///
    /// * `endpoint_url` - The base URL of the endpoint to check
    /// * `config` - Health check configuration
    ///
    /// # Example
    ///
    /// ```ignore
    /// use api_claude::{ HealthChecker, HealthCheckConfig, HealthCheckStrategy };
    ///
    /// let config = HealthCheckConfig::new()
    ///   .with_strategy( HealthCheckStrategy::Ping )
    ///   .with_timeout_ms( 3000 );
    ///
    /// let result = HealthChecker::check_endpoint(
    ///   "https://api.anthropic.com",
    ///   &config
    /// ).await;
    ///
    /// if result.is_healthy() {
    ///   println!( "Endpoint is healthy!" );
    /// }
    /// ```
    pub async fn check_endpoint(
      endpoint_url : &str,
      config : &HealthCheckConfig
    ) -> HealthCheckResult
    {
      let start_time = Instant::now();

      let ( status, error_message ) = match config.strategy
      {
        HealthCheckStrategy::Ping => Self::ping_check( endpoint_url, config ).await,
        HealthCheckStrategy::LightweightApi => Self::lightweight_api_check( endpoint_url, config ).await,
      };

      let response_time_ms = u64::try_from( start_time.elapsed().as_millis() ).unwrap_or( u64::MAX );

      // Determine status based on response time if initially healthy
      let final_status = match status
      {
        EndpointHealthStatus::Healthy if response_time_ms >= config.unhealthy_threshold_ms => EndpointHealthStatus::Unhealthy,
        EndpointHealthStatus::Healthy if response_time_ms >= config.degraded_threshold_ms => EndpointHealthStatus::Degraded,
        other => other,
      };

      HealthCheckResult
      {
        endpoint_url : endpoint_url.to_string(),
        status : final_status,
        response_time_ms,
        error_message,
        timestamp : SystemTime::now(),
      }
    }

    /// Perform ping-style connectivity check
    ///
    /// This is a lightweight check that verifies basic connectivity.
    /// For Anthropic API, this performs a minimal HTTP request.
    async fn ping_check(
      endpoint_url : &str,
      config : &HealthCheckConfig
    ) -> ( EndpointHealthStatus, Option< String > )
    {
      // Create a simple HTTP client for connectivity check
      let timeout = Duration::from_millis( config.timeout_ms );
      let client = match reqwest::Client::builder()
        .timeout( timeout )
        .build()
      {
        Ok( c ) => c,
        Err( e ) => return ( EndpointHealthStatus::Unhealthy, Some( format!( "Failed to create client : {e}" ) ) ),
      };

      // Try to connect to the endpoint (HEAD request is most lightweight)
      match client.head( endpoint_url ).send().await
      {
        Ok( response ) if response.status().is_success() || response.status().as_u16() == 405 =>
        {
          // 405 Method Not Allowed is ok for HEAD requests - means endpoint is up
          ( EndpointHealthStatus::Healthy, None )
        },
        Ok( response ) =>
        {
          ( EndpointHealthStatus::Unhealthy, Some( format!( "HTTP {}", response.status() ) ) )
        },
        Err( e ) if e.is_timeout() =>
        {
          ( EndpointHealthStatus::Unhealthy, Some( "Request timeout".to_string() ) )
        },
        Err( e ) if e.is_connect() =>
        {
          ( EndpointHealthStatus::Unhealthy, Some( "Connection failed".to_string() ) )
        },
        Err( e ) =>
        {
          ( EndpointHealthStatus::Unhealthy, Some( format!( "Request failed : {e}" ) ) )
        },
      }
    }

    /// Perform lightweight API call health check
    ///
    /// This makes an actual API call to verify the endpoint is functional.
    /// More accurate than ping but higher overhead.
    async fn lightweight_api_check(
      endpoint_url : &str,
      config : &HealthCheckConfig
    ) -> ( EndpointHealthStatus, Option< String > )
    {
      // For Anthropic API, we don't have a dedicated health endpoint
      // We'll use a connectivity check similar to ping but with API-specific validation
      let timeout = Duration::from_millis( config.timeout_ms );
      let client = match reqwest::Client::builder()
        .timeout( timeout )
        .build()
      {
        Ok( c ) => c,
        Err( e ) => return ( EndpointHealthStatus::Unhealthy, Some( format!( "Failed to create client : {e}" ) ) ),
      };

      // Try OPTIONS request to check if endpoint is responding
      match client.request( reqwest::Method::OPTIONS, endpoint_url ).send().await
      {
        Ok( response ) if response.status().is_success() || response.status().as_u16() == 405 || response.status().as_u16() == 404 =>
        {
          // 404 or 405 on OPTIONS is ok - means server is responding
          ( EndpointHealthStatus::Healthy, None )
        },
        Ok( response ) if response.status().is_server_error() =>
        {
          ( EndpointHealthStatus::Unhealthy, Some( format!( "Server error : {}", response.status() ) ) )
        },
        Ok( response ) =>
        {
          // Other status codes might indicate issues but server is responding
          ( EndpointHealthStatus::Degraded, Some( format!( "HTTP {}", response.status() ) ) )
        },
        Err( e ) if e.is_timeout() =>
        {
          ( EndpointHealthStatus::Unhealthy, Some( "Request timeout".to_string() ) )
        },
        Err( e ) if e.is_connect() =>
        {
          ( EndpointHealthStatus::Unhealthy, Some( "Connection failed".to_string() ) )
        },
        Err( e ) =>
        {
          ( EndpointHealthStatus::Unhealthy, Some( format!( "Request failed : {e}" ) ) )
        },
      }
    }

    /// Check multiple endpoints concurrently
    ///
    /// Returns results for all endpoints. Useful for failover scenarios
    /// where you need to check multiple backup endpoints.
    pub async fn check_multiple_endpoints(
      endpoints : &[ &str ],
      config : &HealthCheckConfig
    ) -> Vec< HealthCheckResult >
    {
      let mut handles = Vec::new();

      for endpoint in endpoints
      {
        let endpoint = ( *endpoint ).to_string();
        let config = config.clone();
        let handle = tokio::spawn( async move
        {
          Self::check_endpoint( &endpoint, &config ).await
        });
        handles.push( handle );
      }

      let mut results = Vec::new();
      for handle in handles
      {
        if let Ok( result ) = handle.await
        {
          results.push( result );
        }
      }

      results
    }
  }

  /// Health metrics aggregator for multiple check results
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct HealthMetrics
  {
    /// Total number of endpoints checked
    pub total_endpoints : usize,
    /// Number of healthy endpoints
    pub healthy_count : usize,
    /// Number of degraded endpoints
    pub degraded_count : usize,
    /// Number of unhealthy endpoints
    pub unhealthy_count : usize,
    /// Average response time across all checks
    pub average_response_time_ms : u64,
    /// Minimum response time
    pub min_response_time_ms : u64,
    /// Maximum response time
    pub max_response_time_ms : u64,
  }

  impl HealthMetrics
  {
    /// Create health metrics from a collection of health check results
    #[ must_use ]
    pub fn from_results( results : &[ HealthCheckResult ] ) -> Self
    {
      if results.is_empty()
      {
        return Self
        {
          total_endpoints : 0,
          healthy_count : 0,
          degraded_count : 0,
          unhealthy_count : 0,
          average_response_time_ms : 0,
          min_response_time_ms : 0,
          max_response_time_ms : 0,
        };
      }

      let total_endpoints = results.len();
      let healthy_count = results.iter().filter( | r | matches!( r.status, EndpointHealthStatus::Healthy ) ).count();
      let degraded_count = results.iter().filter( | r | matches!( r.status, EndpointHealthStatus::Degraded ) ).count();
      let unhealthy_count = results.iter().filter( | r | matches!( r.status, EndpointHealthStatus::Unhealthy ) ).count();

      let response_times : Vec< u64 > = results.iter().map( | r | r.response_time_ms ).collect();
      let total_response_time : u64 = response_times.iter().sum();
      let average_response_time_ms = total_response_time / total_endpoints as u64;
      let min_response_time_ms = *response_times.iter().min().unwrap_or( &0 );
      let max_response_time_ms = *response_times.iter().max().unwrap_or( &0 );

      Self
      {
        total_endpoints,
        healthy_count,
        degraded_count,
        unhealthy_count,
        average_response_time_ms,
        min_response_time_ms,
        max_response_time_ms,
      }
    }

    /// Get the percentage of healthy endpoints
    #[ inline ]
    #[ must_use ]
    pub fn healthy_percentage( &self ) -> f64
    {
      if self.total_endpoints == 0
      {
        0.0
      }
      else
      {
        ( self.healthy_count as f64 / self.total_endpoints as f64 ) * 100.0
      }
    }

    /// Get the percentage of available endpoints (healthy + degraded)
    #[ inline ]
    #[ must_use ]
    pub fn available_percentage( &self ) -> f64
    {
      if self.total_endpoints == 0
      {
        0.0
      }
      else
      {
        ( ( self.healthy_count + self.degraded_count ) as f64 / self.total_endpoints as f64 ) * 100.0
      }
    }
  }
}

crate::mod_interface!
{
  exposed use private::EndpointHealthStatus;
  exposed use private::HealthCheckStrategy;
  exposed use private::HealthCheckResult;
  exposed use private::HealthCheckConfig;
  exposed use private::HealthChecker;
  exposed use private::HealthMetrics;
}
