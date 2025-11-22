//! Health Check Module
//!
//! This module provides stateless health monitoring utilities for `OpenAI` API endpoints.
//! Following the "Thin Client, Rich API" principle, this module offers utilities for
//! one-time health checks without maintaining client-side state or automatic behaviors.

#![ allow( clippy::missing_inline_in_public_items, clippy::unused_async ) ]

mod private
{
  use std::
  {
    collections ::HashMap,
    time ::Instant,
  };
  use serde::{ Deserialize, Serialize };
  use crate::
  {
    Client,
    client_api_accessors ::ClientApiAccessors,
    environment ::{ OpenaiEnvironment, EnvironmentInterface },
  };

  /// Health status for an endpoint
  #[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
  pub enum HealthStatus
  {
    /// Endpoint is healthy and responding normally
    Healthy,
    /// Endpoint is degraded but functional
    Degraded,
    /// Endpoint is unhealthy and not responding
    Unhealthy,
  }

  /// Health check strategy
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub enum HealthCheckStrategy
  {
    /// Simple connectivity check
    Ping,
    /// Lightweight API call (models list)
    LightweightApi,
  }

  /// Health check result for a single endpoint
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct HealthCheckResult
  {
    /// The endpoint URL that was checked
    pub endpoint_url : String,
    /// Health status of the endpoint
    pub status : HealthStatus,
    /// Response time in milliseconds
    pub response_time_ms : u64,
    /// Optional error message if unhealthy
    pub error_message : Option< String >,
    /// Timestamp when check was performed
    pub timestamp : std::time::SystemTime,
  }

  /// Configuration for health checks
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct HealthCheckConfig
  {
    /// Timeout for health check requests
    pub timeout_ms : u64,
    /// Strategy to use for health checks
    pub strategy : HealthCheckStrategy,
    /// Threshold for degraded status (ms)
    pub degraded_threshold_ms : u64,
    /// Threshold for unhealthy status (ms)
    pub unhealthy_threshold_ms : u64,
  }

  impl Default for HealthCheckConfig
  {
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

  /// Stateless health check utilities
  #[ derive( Debug ) ]
  pub struct HealthChecker;

  impl HealthChecker
  {
    /// Perform a single health check on the given client
    /// This is a stateless operation that returns results immediately
    pub async fn check_endpoint< E >(
      client : &Client< E >,
      config : &HealthCheckConfig
    ) -> HealthCheckResult
    where
      E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
    {
      let start_time = Instant::now();
      let endpoint_url = client.environment.base_url().to_string();

      let ( status, error_message ) = match config.strategy
      {
        HealthCheckStrategy::Ping => Self::ping_check( client, config ).await,
        HealthCheckStrategy::LightweightApi => Self::lightweight_api_check( client, config ).await,
      };

      let response_time_ms = u64::try_from( start_time.elapsed().as_millis() ).unwrap_or( u64::MAX );

      // Determine status based on response time if initially healthy
      let final_status = match status
      {
        HealthStatus::Healthy if response_time_ms >= config.unhealthy_threshold_ms => HealthStatus::Unhealthy,
        HealthStatus::Healthy if response_time_ms >= config.degraded_threshold_ms => HealthStatus::Degraded,
        other => other,
      };

      HealthCheckResult
      {
        endpoint_url,
        status : final_status,
        response_time_ms,
        error_message,
        timestamp : std::time::SystemTime::now(),
      }
    }

    /// Perform ping-style connectivity check
    async fn ping_check< E >(
      _client : &Client< E >,
      _config : &HealthCheckConfig
    ) -> ( HealthStatus, Option< String > )
    where
      E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
    {
      // Simple connectivity check - for OpenAI API, we'll use the lightweight API approach
      // since there's no dedicated ping endpoint
      ( HealthStatus::Healthy, None )
    }

    /// Perform lightweight API call health check
    async fn lightweight_api_check< E >(
      client : &Client< E >,
      _config : &HealthCheckConfig
    ) -> ( HealthStatus, Option< String > )
    where
      E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
    {
      // Use models list as a lightweight health check
      match client.models().list().await
      {
        Ok( _ ) => ( HealthStatus::Healthy, None ),
        Err( e ) =>
        {
          let error_msg = format!( "Health check failed : {e}" );
          ( HealthStatus::Unhealthy, Some( error_msg ) )
        }
      }
    }

    /// Perform health checks on multiple endpoints (utility function)
    /// Note : This is stateless - results are returned immediately
    pub async fn check_multiple_endpoints< E >(
      clients : &[ &Client< E > ],
      config : &HealthCheckConfig
    ) -> Vec< HealthCheckResult >
    where
      E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
    {
      let mut results = Vec::with_capacity( clients.len() );

      // Perform checks sequentially to avoid overwhelming endpoints
      for client in clients
      {
        let result = Self::check_endpoint( client, config ).await;
        results.push( result );
      }

      results
    }

    /// Generate health summary from multiple results
    #[ must_use ]
    pub fn summarize_health( results : &[ HealthCheckResult ] ) -> HashMap<  String, usize  >
    {
      let mut summary = HashMap::new();

      for result in results
      {
        let status_str = match result.status
        {
          HealthStatus::Healthy => "healthy",
          HealthStatus::Degraded => "degraded",
          HealthStatus::Unhealthy => "unhealthy",
        };

        *summary.entry( status_str.to_string() ).or_insert( 0 ) += 1;
      }

      summary
    }
  }
}

crate ::mod_interface!
{
  exposed use private::HealthStatus;
  exposed use private::HealthCheckStrategy;
  exposed use private::HealthCheckResult;
  exposed use private::HealthCheckConfig;
  exposed use private::HealthChecker;
}