//! Health check models and types
//!
//! This module provides explicit health check functionality following the
//! "Thin Client, Rich API" principle. All health checks are explicit,
//! on-demand operations with no automatic background monitoring.

use core::time::Duration;
use std::time::SystemTime;
use serde::{ Serialize, Deserialize };

/// Health status of an endpoint
#[ derive( Debug, Clone, PartialEq, Eq, Serialize, Deserialize ) ]
pub enum HealthStatus
{
  /// Endpoint is responding normally
  Healthy,
  /// Endpoint is responding but with degraded performance
  Degraded,
  /// Endpoint is not responding or returning errors
  Unhealthy,
}

/// Result of a health check operation
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct HealthCheckResult
{
  /// Current health status
  pub status : HealthStatus,
  /// Response time for the health check
  pub response_time : Option< Duration >,
  /// Timestamp when the check was performed
  pub checked_at : Option< SystemTime >,
  /// Endpoint that was checked
  pub endpoint : String,
  /// Error message if check failed
  pub error_message : Option< String >,
}

/// Configuration for health check operations
#[ derive( Debug, Clone ) ]
pub struct HealthCheckConfig
{
  /// Timeout for health check requests
  pub timeout : Duration,
  /// Strategy to use for health checking
  pub strategy : HealthCheckStrategy,
}

/// Available health check strategies
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum HealthCheckStrategy
{
  /// Simple HTTP ping to the base URL
  Ping,
  /// Lightweight API call (e.g., list models)
  LightweightApiCall,
}

impl Default for HealthCheckConfig
{
  #[ inline ]
  fn default() -> Self
  {
    Self {
      timeout : Duration::from_secs( 10 ),
      strategy : HealthCheckStrategy::Ping,
    }
  }
}

/// Builder for health check operations
#[ derive( Debug, Clone ) ]
pub struct HealthCheckBuilder
{
  config : HealthCheckConfig,
  client : crate::client::Client,
}

impl HealthCheckBuilder
{
  /// Create a new health check builder
  #[ must_use ]
  #[ inline ]
  pub fn new( client : crate::client::Client ) -> Self
  {
    Self {
      config : HealthCheckConfig::default(),
      client,
    }
  }

  /// Set the timeout for health checks
  #[ inline ]
  #[ must_use ]
  pub fn timeout( mut self, timeout : Duration ) -> Self
  {
    self.config.timeout = timeout;
    self
  }

  /// Set the health check strategy
  #[ inline ]
  #[ must_use ]
  pub fn strategy( mut self, strategy : HealthCheckStrategy ) -> Self
  {
    self.config.strategy = strategy;
    self
  }

  /// Perform an explicit health check on the configured endpoint
  ///
  /// # Errors
  ///
  /// Returns `Error` if the health check fails due to:
  /// - Network connectivity issues
  /// - HTTP client creation errors
  /// - Endpoint not responding or returning errors
  #[ inline ]
  pub async fn check_endpoint( self ) -> Result< HealthCheckResult, crate::error::Error >
  {
    let start_time = SystemTime::now();

    let result = match self.config.strategy
    {
      HealthCheckStrategy::Ping => self.perform_ping_check().await,
      HealthCheckStrategy::LightweightApiCall => self.perform_api_check().await,
    };

    let response_time = start_time.elapsed().ok();

    match result
    {
      Ok( () ) => Ok( HealthCheckResult {
        status : HealthStatus::Healthy,
        response_time,
        checked_at : Some( start_time ),
        endpoint : self.client.base_url().to_string(),
        error_message : None,
      } ),
      Err( e ) => Ok( HealthCheckResult {
        status : HealthStatus::Unhealthy,
        response_time,
        checked_at : Some( start_time ),
        endpoint : self.client.base_url().to_string(),
        error_message : Some( e.to_string() ),
      } ),
    }
  }

  /// Perform a simple ping check
  async fn perform_ping_check( &self ) -> Result< (), crate::error::Error >
  {
    let client = reqwest::Client::builder()
      .timeout( self.config.timeout )
      .build()
      .map_err( |e| crate::error::Error::Health( format!( "Failed to create HTTP client : {e}" ) ) )?;

    let response = client
      .head( self.client.base_url() )
      .send()
      .await
      .map_err( |e| crate::error::Error::Health( format!( "Health check request failed : {e}" ) ) )?;

    if response.status().is_success() || response.status().is_redirection()
    {
      Ok( () )
    } else {
      Err( crate::error::Error::Health( format!( "Health check failed with status : {}", response.status() ) ) )
    }
  }

  /// Perform a lightweight API call check
  async fn perform_api_check( &self ) -> Result< (), crate::error::Error >
  {
    // Use the list models endpoint as a lightweight check
    let _models = self.client.models().list().await?;
    Ok( () )
  }
}