//! Dynamic Configuration Implementation
//!
//! Provides runtime configuration management with watchers, history, and rollback.
//!
//! ## Features
//!
//! - **Runtime Updates**: Change configuration without restarting
//! - **Watchers**: Callbacks notified on configuration changes
//! - **History Tracking**: Maintains history of configuration changes
//! - **Rollback**: Revert to previous configurations
//! - **Validation**: Validate configs before applying
//! - **Thread-Safe**: Safe for concurrent access
//!
//! ## Usage
//!
//! ```no_run
//! # use api_huggingface::config::{DynamicConfig, ReliabilityConfig};
//! # use std::sync::Arc;
//! # async fn example( ) -> Result< ( ), Box< dyn std::error::Error > > {
//! let config = ReliabilityConfig::default( );
//! let dynamic_config = DynamicConfig::new( config );
//!
//! // Add a watcher
//! dynamic_config.add_watcher( |_old, _new| {
//!   println!( "Config updated" );
//! } ).await;
//!
//! // Update config
//! let mut new_config = dynamic_config.get( ).await;
//! new_config.circuit_breaker = None;
//! dynamic_config.update( new_config ).await?;
//! # Ok( ( ))
//! # }
//! ```

use crate::reliability::{
  CircuitBreakerConfig,
  RateLimiterConfig,
  FailoverConfig,
  HealthCheckConfig,
};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

/// Reliability configuration snapshot
#[ derive( Debug, Clone ) ]
pub struct ReliabilityConfig
{
  /// Circuit breaker configuration
  pub circuit_breaker : Option< CircuitBreakerConfig >,
  /// Rate limiter configuration
  pub rate_limiter : Option< RateLimiterConfig >,
  /// Failover configuration
  pub failover : Option< FailoverConfig >,
  /// Health check configuration
  pub health_check : Option< HealthCheckConfig >,
  /// Timestamp when this config was created/updated
  pub timestamp : Instant,
}

impl Default for ReliabilityConfig
{
  #[ inline ]
  fn default() -> Self
  {
  Self
  {
      circuit_breaker : None,
      rate_limiter : None,
      failover : None,
      health_check : None,
      timestamp : Instant::now( ),
  }
  }
}

impl ReliabilityConfig
{
  /// Create new config with timestamp
  #[ inline ]
  #[ must_use ]
  pub fn new() -> Self
  {
  Self::default( )
  }

  /// Update timestamp to current time
  #[ inline ]
  pub fn update_timestamp( &mut self )
  {
  self.timestamp = Instant::now( );
  }
}

/// Configuration watcher callback type
pub type ConfigWatcherCallback = Arc< dyn Fn( &ReliabilityConfig, &ReliabilityConfig ) + Send + Sync >;

/// Internal state for dynamic configuration
struct DynamicConfigState
{
  current : ReliabilityConfig,
  watchers : Vec< ConfigWatcherCallback >,
  history : VecDeque< ReliabilityConfig >,
  max_history : usize,
}

/// Dynamic configuration manager
#[ derive( Clone ) ]
pub struct DynamicConfig
{
  state : Arc< RwLock< DynamicConfigState > >,
}

impl core::fmt::Debug for DynamicConfig
{
  #[ inline ]
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
  f.debug_struct( "DynamicConfig" )
      .field( "state", &"< DynamicConfigState >" )
      .finish( )
  }
}

impl DynamicConfig
{
  /// Create new dynamic configuration with initial config
  #[ inline ]
  #[ must_use ]
  pub fn new( config : ReliabilityConfig ) -> Self
  {
  Self::with_history_size( config, 10 )
  }

  /// Create new dynamic configuration with custom history size
  #[ inline ]
  #[ must_use ]
  pub fn with_history_size( config : ReliabilityConfig, max_history : usize ) -> Self
  {
  Self
  {
      state : Arc::new( RwLock::new( DynamicConfigState
      {
  current : config,
  watchers : Vec::new( ),
  history : VecDeque::new( ),
  max_history,
      } )),
  }
  }

  /// Get current configuration
  #[ inline ]
  pub async fn get( &self ) -> ReliabilityConfig
  {
  let state = self.state.read( ).await;
  state.current.clone( )
  }

  /// Update configuration and notify watchers
  ///
  /// # Errors
  ///
  /// Returns `ConfigError::ValidationFailed` if the new configuration is invalid.
  #[ inline ]
  pub async fn update( &self, mut new_config : ReliabilityConfig ) -> Result< ( ), ConfigError >
  {
  // Validate new config
  Self::validate( &new_config )?;

  // Update timestamp
  new_config.update_timestamp( );

  let mut state = self.state.write( ).await;

  // Save old config to history
  let old_config = state.current.clone( );
  state.history.push_back( old_config.clone( ));

  // Trim history if needed
  while state.history.len( ) > state.max_history
  {
      state.history.pop_front( );
  }

  // Update current config
  state.current = new_config.clone( );

  // Notify watchers
  for watcher in &state.watchers
  {
      watcher( &old_config, &new_config );
  }

  Ok( ( ))
  }

  /// Add a configuration watcher
  ///
  /// Watchers are called whenever the configuration is updated.
  #[ inline ]
  pub async fn add_watcher< F >( &self, watcher : F )
  where
  F : Fn( &ReliabilityConfig, &ReliabilityConfig ) + Send + Sync + 'static,
  {
  let mut state = self.state.write( ).await;
  state.watchers.push( Arc::new( watcher ));
  }

  /// Remove all watchers
  #[ inline ]
  pub async fn clear_watchers( &self )
  {
  let mut state = self.state.write( ).await;
  state.watchers.clear( );
  }

  /// Get number of registered watchers
  #[ inline ]
  pub async fn watcher_count( &self ) -> usize
  {
  let state = self.state.read( ).await;
  state.watchers.len( )
  }

  /// Rollback to previous configuration
  ///
  /// # Errors
  ///
  /// Returns `ConfigError::NoHistory` if there is no history to rollback to.
  #[ inline ]
  pub async fn rollback( &self ) -> Result< ( ), ConfigError >
  {
  let mut state = self.state.write( ).await;

  let old_config = state.history.pop_back( )
      .ok_or( ConfigError::NoHistory )?;

  let current_config = state.current.clone( );

  // Restore old config
  state.current = old_config.clone( );

  // Notify watchers
  for watcher in &state.watchers
  {
      watcher( &current_config, &old_config );
  }

  Ok( ( ))
  }

  /// Get configuration history
  #[ inline ]
  pub async fn get_history( &self ) -> Vec< ReliabilityConfig >
  {
  let state = self.state.read( ).await;
  state.history.iter( ).cloned( ).collect( )
  }

  /// Get number of configs in history
  #[ inline ]
  pub async fn history_size( &self ) -> usize
  {
  let state = self.state.read( ).await;
  state.history.len( )
  }

  /// Clear configuration history
  #[ inline ]
  pub async fn clear_history( &self )
  {
  let mut state = self.state.write( ).await;
  state.history.clear( );
  }

  /// Validate configuration
  ///
  /// # Errors
  ///
  /// Returns `ConfigError::ValidationFailed` if validation fails.
  #[ inline ]
  fn validate( config : &ReliabilityConfig ) -> Result< ( ), ConfigError >
  {
  // Validate circuit breaker config
  if let Some( ref cb_config ) = config.circuit_breaker
  {
      if cb_config.failure_threshold == 0
      {
  return Err( ConfigError::ValidationFailed {
          field : "circuit_breaker.failure_threshold".to_string( ),
          reason : "must be greater than 0".to_string( ),
  } );
      }
      if cb_config.success_threshold == 0
      {
  return Err( ConfigError::ValidationFailed {
          field : "circuit_breaker.success_threshold".to_string( ),
          reason : "must be greater than 0".to_string( ),
  } );
      }
  }

  // Validate rate limiter config
  if let Some( ref rl_config ) = config.rate_limiter
  {
      if rl_config.requests_per_second.is_none( )
  && rl_config.requests_per_minute.is_none( )
  && rl_config.requests_per_hour.is_none( )
      {
  return Err( ConfigError::ValidationFailed {
          field : "rate_limiter".to_string( ),
          reason : "at least one rate limit must be set".to_string( ),
  } );
      }
  }

  // Validate failover config
  if let Some( ref fo_config ) = config.failover
  {
      if fo_config.endpoints.is_empty( )
      {
  return Err( ConfigError::ValidationFailed {
          field : "failover.endpoints".to_string( ),
          reason : "must have at least one endpoint".to_string( ),
  } );
      }
      if fo_config.failure_threshold == 0
      {
  return Err( ConfigError::ValidationFailed {
          field : "failover.failure_threshold".to_string( ),
          reason : "must be greater than 0".to_string( ),
  } );
      }
  }

  // Validate health check config
  if let Some( ref hc_config ) = config.health_check
  {
      if hc_config.endpoint.is_empty( )
      {
  return Err( ConfigError::ValidationFailed {
          field : "health_check.endpoint".to_string( ),
          reason : "endpoint must not be empty".to_string( ),
  } );
      }
      if hc_config.unhealthy_threshold == 0
      {
  return Err( ConfigError::ValidationFailed {
          field : "health_check.unhealthy_threshold".to_string( ),
          reason : "must be greater than 0".to_string( ),
  } );
      }
  }

  Ok( ( ))
  }

  /// Check if configuration is valid
  #[ inline ]
  #[ must_use ]
  pub fn is_valid( config : &ReliabilityConfig ) -> bool
  {
  Self::validate( config ).is_ok( )
  }
}

/// Configuration errors
#[ derive( Debug ) ]
pub enum ConfigError
{
  /// Configuration validation failed
  ValidationFailed
  {
  /// Field that failed validation
  field : String,
  /// Reason for failure
  reason : String,
  },
  /// No history available for rollback
  NoHistory,
}

impl core::fmt::Display for ConfigError
{
  #[ inline ]
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
  match self
  {
      Self::ValidationFailed { field, reason } => {
  write!( f, "Validation failed for {field}: {reason}" )
      }
      Self::NoHistory => write!( f, "No configuration history available for rollback" ),
  }
  }
}

impl std::error::Error for ConfigError {}

#[ cfg( test ) ]
mod tests {
  use super::*;
  use core::time::Duration;

  #[ tokio::test ]
  async fn test_dynamic_config_creation()
  {
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( config );

  let current = dynamic_config.get( ).await;
  assert!( current.circuit_breaker.is_none( ));
  assert!( current.rate_limiter.is_none( ));
  assert!( current.failover.is_none( ));
  assert!( current.health_check.is_none( ));
  }

  #[ tokio::test ]
  async fn test_config_update()
  {
  let initial_config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( initial_config );

  let new_config = ReliabilityConfig {
      circuit_breaker : Some( CircuitBreakerConfig {
  failure_threshold : 5,
  success_threshold : 2,
  timeout : Duration::from_secs( 60 ),
      } ),
      ..ReliabilityConfig::default( )
  };

  let result = dynamic_config.update( new_config ).await;
  assert!( result.is_ok( ));

  let current = dynamic_config.get( ).await;
  assert!( current.circuit_breaker.is_some( ));
  }

  #[ tokio::test ]
  async fn test_watcher_notification()
  {
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( config );

  let notified = Arc::new( RwLock::new( false ));
  let notified_clone = notified.clone( );

  dynamic_config.add_watcher( move |_old, _new| {
      let notified = notified_clone.clone( );
      tokio::spawn( async move {
  let mut n = notified.write( ).await;
  *n = true;
      } );
  } ).await;

  let new_config = ReliabilityConfig::default( );
  dynamic_config.update( new_config ).await.unwrap( );

  // Give watcher time to execute
  tokio::time::sleep( Duration::from_millis( 50 )).await;

  let was_notified = *notified.read( ).await;
  assert!( was_notified, "Watcher should have been notified" );
  }

  #[ tokio::test ]
  async fn test_history_tracking()
  {
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( config );

  assert_eq!( dynamic_config.history_size( ).await, 0 );

  // First update
  dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );
  assert_eq!( dynamic_config.history_size( ).await, 1 );

  // Second update
  dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );
  assert_eq!( dynamic_config.history_size( ).await, 2 );
  }

  #[ tokio::test ]
  async fn test_rollback()
  {
  let initial_config = ReliabilityConfig {
      circuit_breaker : Some( CircuitBreakerConfig {
  failure_threshold : 3,
  success_threshold : 2,
  timeout : Duration::from_secs( 30 ),
      } ),
      ..ReliabilityConfig::default( )
  };

  let dynamic_config = DynamicConfig::new( initial_config.clone( ));

  // Update config
  let new_config = ReliabilityConfig {
      circuit_breaker : Some( CircuitBreakerConfig {
  failure_threshold : 10,
  success_threshold : 5,
  timeout : Duration::from_secs( 120 ),
      } ),
      ..ReliabilityConfig::default( )
  };

  dynamic_config.update( new_config ).await.unwrap( );

  let current = dynamic_config.get( ).await;
  assert_eq!( current.circuit_breaker.as_ref( ).unwrap( ).failure_threshold, 10 );

  // Rollback
  dynamic_config.rollback( ).await.unwrap( );

  let rolled_back = dynamic_config.get( ).await;
  assert_eq!( rolled_back.circuit_breaker.as_ref( ).unwrap( ).failure_threshold, 3 );
  }

  #[ tokio::test ]
  async fn test_rollback_no_history()
  {
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( config );

  let result = dynamic_config.rollback( ).await;
  assert!( result.is_err( ));

  match result
  {
      Err( ConfigError::NoHistory ) => {}
      _ => panic!( "Expected NoHistory error" ),
  }
  }

  #[ tokio::test ]
  async fn test_validation_circuit_breaker_failure_threshold()
  {
  let config = ReliabilityConfig {
      circuit_breaker : Some( CircuitBreakerConfig {
  failure_threshold : 0,  // Invalid
  success_threshold : 2,
  timeout : Duration::from_secs( 60 ),
      } ),
      ..ReliabilityConfig::default( )
  };

  let dynamic_config = DynamicConfig::new( ReliabilityConfig::default( ));
  let result = dynamic_config.update( config ).await;

  assert!( result.is_err( ));
  match result
  {
      Err( ConfigError::ValidationFailed { field, .. } ) => {
  assert_eq!( field, "circuit_breaker.failure_threshold" );
      }
      _ => panic!( "Expected ValidationFailed error" ),
  }
  }

  #[ tokio::test ]
  async fn test_validation_rate_limiter_no_limits()
  {
  let config = ReliabilityConfig {
      rate_limiter : Some( RateLimiterConfig {
  requests_per_second : None,
  requests_per_minute : None,
  requests_per_hour : None,
      } ),
      ..ReliabilityConfig::default( )
  };

  let dynamic_config = DynamicConfig::new( ReliabilityConfig::default( ));
  let result = dynamic_config.update( config ).await;

  assert!( result.is_err( ));
  }

  #[ tokio::test ]
  async fn test_max_history_size()
  {
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::with_history_size( config, 3 );

  // Add 5 configs
  for _ in 0..5
  {
      dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );
  }

  // Should only keep last 3
  assert_eq!( dynamic_config.history_size( ).await, 3 );
  }

  #[ tokio::test ]
  async fn test_clear_history()
  {
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( config );

  dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );
  dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );

  assert_eq!( dynamic_config.history_size( ).await, 2 );

  dynamic_config.clear_history( ).await;

  assert_eq!( dynamic_config.history_size( ).await, 0 );
  }

  #[ tokio::test ]
  async fn test_watcher_count()
  {
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( config );

  assert_eq!( dynamic_config.watcher_count( ).await, 0 );

  dynamic_config.add_watcher( |_, _| {} ).await;
  assert_eq!( dynamic_config.watcher_count( ).await, 1 );

  dynamic_config.add_watcher( |_, _| {} ).await;
  assert_eq!( dynamic_config.watcher_count( ).await, 2 );

  dynamic_config.clear_watchers( ).await;
  assert_eq!( dynamic_config.watcher_count( ).await, 0 );
  }

  #[ tokio::test ]
  async fn test_config_default()
  {
  let config = ReliabilityConfig::default( );
  assert!( config.circuit_breaker.is_none( ));
  assert!( config.rate_limiter.is_none( ));
  assert!( config.failover.is_none( ));
  assert!( config.health_check.is_none( ));
  }

  #[ tokio::test ]
  async fn test_is_valid_helper()
  {
  let mut valid_config = ReliabilityConfig::default( );
  assert!( DynamicConfig::is_valid( &valid_config ));

  valid_config.circuit_breaker = Some( CircuitBreakerConfig {
      failure_threshold : 0,  // Invalid
      success_threshold : 2,
      timeout : Duration::from_secs( 60 ),
  } );

  assert!( !DynamicConfig::is_valid( &valid_config ));
  }
}
