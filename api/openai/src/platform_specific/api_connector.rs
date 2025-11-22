//! Third-Party API Connectors
//!
//! Types and configurations for connecting to external APIs.

use std::collections::HashMap;
use serde::{ Serialize, Deserialize };
use core::time::Duration;
use crate::error::Result;

/// Configuration for third-party API connectors.
#[ derive( Debug, Clone ) ]
pub struct ApiConnectorConfig
{
  /// Base URL for the API
  pub base_url : String,
  /// Authentication configuration
  pub authentication : ApiAuthentication,
  /// Rate limiting configuration
  pub rate_limits : Option< RateLimitConfig >,
  /// Retry configuration
  pub retry_config : Option< RetryConfig >,
}

/// Authentication methods for third-party APIs.
#[ derive( Debug, Clone ) ]
pub enum ApiAuthentication
{
  /// No authentication
  None,
  /// API key in header
  ApiKey
  {
    /// Header name
    header : String,
    /// API key value
    key : String,
  },
  /// Bearer token authentication
  Bearer
  {
    /// Bearer token
    token : String,
  },
  /// `OAuth2` authentication
  OAuth2
  {
    /// Client ID
    client_id : String,
    /// Client secret
    client_secret : String,
  },
  /// Custom headers
  Custom
  {
    /// Custom headers
    headers : HashMap<  String, String  >,
  },
}

/// Rate limiting configuration.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct RateLimitConfig
{
  /// Requests per minute
  pub requests_per_minute : u32,
  /// Requests per hour
  pub requests_per_hour : u32,
  /// Burst limit
  pub burst_limit : u32,
}

/// Retry configuration.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct RetryConfig
{
  /// Maximum number of retries
  pub max_retries : u32,
  /// Base delay between retries
  pub base_delay : Duration,
  /// Maximum delay between retries
  pub max_delay : Duration,
  /// Exponential backoff factor
  pub backoff_factor : f64,
}

/// Trait for third-party API connectors.
#[ async_trait::async_trait ]
pub trait ApiConnector : Send + Sync
{
  /// Connector name
  fn name( &self ) -> &str;

  /// Connector configuration
  fn config( &self ) -> &ApiConnectorConfig;

  /// Make a request to the API
  async fn make_request(
    &self,
    method : &str,
    endpoint : &str,
    body : Option< serde_json::Value >
  ) -> Result< serde_json::Value >;
}
