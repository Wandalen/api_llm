//! Integration tests for Dynamic Configuration
//!
//! These tests verify the dynamic configuration functionality.
//! Tests cover configuration updates, watchers, validation, history, and rollback.
//!
//! ## Test Coverage
//!
//! - Configuration updates
//! - Watcher notifications
//! - Concurrent updates
//! - Configuration validation
//! - History tracking
//! - Rollback functionality
//! - Integration with reliability components

#![ allow( clippy::field_reassign_with_default ) ]
#![ allow( clippy::map_unwrap_or ) ]
#![ allow( clippy::match_wild_err_arm ) ]
#![ allow( clippy::ignored_unit_patterns ) ]

use api_huggingface::config::{DynamicConfig, ReliabilityConfig};
use api_huggingface::reliability::{
  CircuitBreakerConfig,
  RateLimiterConfig,
  FailoverConfig,
  FailoverStrategy,
  HealthCheckConfig,
  HealthCheckStrategy,
};
use core::time::Duration;
use std::sync::Arc;
use tokio::sync::RwLock;

// ============================================================================
// Configuration Update Tests
// ============================================================================

#[ tokio::test ]
async fn test_update_circuit_breaker_config() 
{
  let initial_config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( initial_config );

  let mut new_config = ReliabilityConfig::default( );
  new_config.circuit_breaker = Some( CircuitBreakerConfig {
  failure_threshold : 5,
  success_threshold : 2,
  timeout : Duration::from_secs( 60 ),
  } );

  let result = dynamic_config.update( new_config ).await;
  assert!( result.is_ok( ), "Update should succeed" );

  let current = dynamic_config.get( ).await;
  assert!( current.circuit_breaker.is_some( ));
  assert_eq!( current.circuit_breaker.unwrap( ).failure_threshold, 5 );
}

#[ tokio::test ]
async fn test_update_rate_limiter_config() 
{
  let initial_config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( initial_config );

  let mut new_config = ReliabilityConfig::default( );
  new_config.rate_limiter = Some( RateLimiterConfig {
  requests_per_second : Some( 10 ),
  requests_per_minute : Some( 500 ),
  requests_per_hour : Some( 10000 ),
  } );

  let result = dynamic_config.update( new_config ).await;
  assert!( result.is_ok( ), "Update should succeed" );

  let current = dynamic_config.get( ).await;
  assert!( current.rate_limiter.is_some( ));
  assert_eq!( current.rate_limiter.unwrap( ).requests_per_second, Some( 10 ));
}

#[ tokio::test ]
async fn test_update_failover_config() 
{
  let initial_config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( initial_config );

  let mut new_config = ReliabilityConfig::default( );
  new_config.failover = Some( FailoverConfig {
  endpoints : vec!["endpoint1".to_string( ), "endpoint2".to_string( ) ],
  strategy : FailoverStrategy::Priority,
  max_retries : 3,
  failure_window : Duration::from_secs( 300 ),
  failure_threshold : 5,
  } );

  let result = dynamic_config.update( new_config ).await;
  assert!( result.is_ok( ), "Update should succeed" );

  let current = dynamic_config.get( ).await;
  assert!( current.failover.is_some( ));
  assert_eq!( current.failover.unwrap( ).endpoints.len( ), 2 );
}

#[ tokio::test ]
async fn test_update_health_check_config() 
{
  let initial_config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( initial_config );

  let mut new_config = ReliabilityConfig::default( );
  new_config.health_check = Some( HealthCheckConfig {
  endpoint : "https://api.example.com".to_string( ),
  strategy : HealthCheckStrategy::LightweightApi,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 3,
  } );

  let result = dynamic_config.update( new_config ).await;
  assert!( result.is_ok( ), "Update should succeed" );

  let current = dynamic_config.get( ).await;
  assert!( current.health_check.is_some( ));
  assert_eq!( current.health_check.unwrap( ).endpoint, "https://api.example.com" );
}

#[ tokio::test ]
async fn test_update_all_configs_together() 
{
  let initial_config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( initial_config );

  let mut new_config = ReliabilityConfig::default( );
  new_config.circuit_breaker = Some( CircuitBreakerConfig {
  failure_threshold : 3,
  success_threshold : 2,
  timeout : Duration::from_secs( 30 ),
  } );
  new_config.rate_limiter = Some( RateLimiterConfig {
  requests_per_second : Some( 5 ),
  requests_per_minute : None,
  requests_per_hour : None,
  } );

  let result = dynamic_config.update( new_config ).await;
  assert!( result.is_ok( ));

  let current = dynamic_config.get( ).await;
  assert!( current.circuit_breaker.is_some( ));
  assert!( current.rate_limiter.is_some( ));
  assert!( current.failover.is_none( ));
  assert!( current.health_check.is_none( ));
}

// ============================================================================
// Watcher Tests
// ============================================================================

#[ tokio::test ]
async fn test_watcher_receives_notification() 
{
  let initial_config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( initial_config );

  let notified = Arc::new( RwLock::new( false ));
  let notified_clone = notified.clone( );

  dynamic_config.add_watcher( move |_old, _new| {
  let notified = notified_clone.clone( );
  tokio::spawn( async move {
      let mut n = notified.write( ).await;
      *n = true;
  } );
  } ).await;

  dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );

  // Give watcher time to execute
  tokio::time::sleep( Duration::from_millis( 50 )).await;

  assert!( *notified.read( ).await, "Watcher should be notified" );
}

#[ tokio::test ]
async fn test_multiple_watchers_all_notified() 
{
  let initial_config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( initial_config );

  let count = Arc::new( RwLock::new( 0 ));

  let count_clone1 = count.clone( );
  dynamic_config.add_watcher( move |_, _| {
  let count = count_clone1.clone( );
  tokio::spawn( async move {
      let mut c = count.write( ).await;
      *c += 1;
  } );
  } ).await;

  let count_clone2 = count.clone( );
  dynamic_config.add_watcher( move |_, _| {
  let count = count_clone2.clone( );
  tokio::spawn( async move {
      let mut c = count.write( ).await;
      *c += 1;
  } );
  } ).await;

  dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );

  // Give watchers time to execute
  tokio::time::sleep( Duration::from_millis( 100 )).await;

  assert_eq!( *count.read( ).await, 2, "Both watchers should be notified" );
}

#[ tokio::test ]
async fn test_watcher_receives_old_and_new_config() 
{
  let mut initial_config = ReliabilityConfig::default( );
  initial_config.circuit_breaker = Some( CircuitBreakerConfig {
  failure_threshold : 3,
  success_threshold : 2,
  timeout : Duration::from_secs( 30 ),
  } );

  let dynamic_config = DynamicConfig::new( initial_config );

  let old_threshold = Arc::new( RwLock::new( 0 ));
  let new_threshold = Arc::new( RwLock::new( 0 ));

  let old_clone = old_threshold.clone( );
  let new_clone = new_threshold.clone( );

  dynamic_config.add_watcher( move |old, new| {
  let old_t = old_clone.clone( );
  let new_t = new_clone.clone( );
  let old_val = old.circuit_breaker.as_ref( ).map( |cb| cb.failure_threshold ).unwrap_or( 0 );
  let new_val = new.circuit_breaker.as_ref( ).map( |cb| cb.failure_threshold ).unwrap_or( 0 );

  tokio::spawn( async move {
      *old_t.write( ).await = old_val;
      *new_t.write( ).await = new_val;
  } );
  } ).await;

  let mut new_config = ReliabilityConfig::default( );
  new_config.circuit_breaker = Some( CircuitBreakerConfig {
  failure_threshold : 10,
  success_threshold : 2,
  timeout : Duration::from_secs( 30 ),
  } );

  dynamic_config.update( new_config ).await.unwrap( );

  // Give watcher time to execute
  tokio::time::sleep( Duration::from_millis( 50 )).await;

  assert_eq!( *old_threshold.read( ).await, 3 );
  assert_eq!( *new_threshold.read( ).await, 10 );
}

#[ tokio::test ]
async fn test_clear_watchers() 
{
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( config );

  dynamic_config.add_watcher( |_, _| {} ).await;
  dynamic_config.add_watcher( |_, _| {} ).await;

  assert_eq!( dynamic_config.watcher_count( ).await, 2 );

  dynamic_config.clear_watchers( ).await;

  assert_eq!( dynamic_config.watcher_count( ).await, 0 );
}

// ============================================================================
// Validation Tests
// ============================================================================

#[ tokio::test ]
async fn test_validation_rejects_zero_circuit_breaker_thresholds() 
{
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( config );

  let mut invalid_config = ReliabilityConfig::default( );
  invalid_config.circuit_breaker = Some( CircuitBreakerConfig {
  failure_threshold : 0,
  success_threshold : 2,
  timeout : Duration::from_secs( 60 ),
  } );

  let result = dynamic_config.update( invalid_config ).await;
  assert!( result.is_err( ), "Should reject zero failure threshold" );
}

#[ tokio::test ]
async fn test_validation_rejects_empty_rate_limiter() 
{
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( config );

  let mut invalid_config = ReliabilityConfig::default( );
  invalid_config.rate_limiter = Some( RateLimiterConfig {
  requests_per_second : None,
  requests_per_minute : None,
  requests_per_hour : None,
  } );

  let result = dynamic_config.update( invalid_config ).await;
  assert!( result.is_err( ), "Should reject rate limiter with no limits" );
}

#[ tokio::test ]
async fn test_validation_rejects_empty_failover_endpoints() 
{
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( config );

  let mut invalid_config = ReliabilityConfig::default( );
  invalid_config.failover = Some( FailoverConfig {
  endpoints : vec![ ],
  strategy : FailoverStrategy::Priority,
  max_retries : 3,
  failure_window : Duration::from_secs( 300 ),
  failure_threshold : 5,
  } );

  let result = dynamic_config.update( invalid_config ).await;
  assert!( result.is_err( ), "Should reject empty endpoints list" );
}

#[ tokio::test ]
async fn test_validation_rejects_empty_health_check_endpoint() 
{
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( config );

  let mut invalid_config = ReliabilityConfig::default( );
  invalid_config.health_check = Some( HealthCheckConfig {
  endpoint : String::new( ),
  strategy : HealthCheckStrategy::Ping,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 3,
  } );

  let result = dynamic_config.update( invalid_config ).await;
  assert!( result.is_err( ), "Should reject empty endpoint" );
}

#[ tokio::test ]
async fn test_is_valid_helper_function() 
{
  let valid_config = ReliabilityConfig::default( );
  assert!( DynamicConfig::is_valid( &valid_config ));

  let mut invalid_config = ReliabilityConfig::default( );
  invalid_config.circuit_breaker = Some( CircuitBreakerConfig {
  failure_threshold : 0,
  success_threshold : 0,
  timeout : Duration::from_secs( 60 ),
  } );

  assert!( !DynamicConfig::is_valid( &invalid_config ));
}

// ============================================================================
// History Tests
// ============================================================================

#[ tokio::test ]
async fn test_history_tracks_updates() 
{
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( config );

  assert_eq!( dynamic_config.history_size( ).await, 0 );

  dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );
  assert_eq!( dynamic_config.history_size( ).await, 1 );

  dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );
  assert_eq!( dynamic_config.history_size( ).await, 2 );

  dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );
  assert_eq!( dynamic_config.history_size( ).await, 3 );
}

#[ tokio::test ]
async fn test_history_respects_max_size() 
{
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::with_history_size( config, 3 );

  for _ in 0..10
  {
  dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );
  }

  assert_eq!( dynamic_config.history_size( ).await, 3, "Should keep only last 3" );
}

#[ tokio::test ]
async fn test_get_history_returns_configs() 
{
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( config );

  dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );
  dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );

  let history = dynamic_config.get_history( ).await;
  assert_eq!( history.len( ), 2 );
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

// ============================================================================
// Rollback Tests
// ============================================================================

#[ tokio::test ]
async fn test_rollback_restores_previous_config() 
{
  let mut initial_config = ReliabilityConfig::default( );
  initial_config.circuit_breaker = Some( CircuitBreakerConfig {
  failure_threshold : 3,
  success_threshold : 2,
  timeout : Duration::from_secs( 30 ),
  } );

  let dynamic_config = DynamicConfig::new( initial_config );

  let mut new_config = ReliabilityConfig::default( );
  new_config.circuit_breaker = Some( CircuitBreakerConfig {
  failure_threshold : 10,
  success_threshold : 5,
  timeout : Duration::from_secs( 120 ),
  } );

  dynamic_config.update( new_config ).await.unwrap( );

  let current = dynamic_config.get( ).await;
  assert_eq!( current.circuit_breaker.as_ref( ).unwrap( ).failure_threshold, 10 );

  dynamic_config.rollback( ).await.unwrap( );

  let rolled_back = dynamic_config.get( ).await;
  assert_eq!( rolled_back.circuit_breaker.as_ref( ).unwrap( ).failure_threshold, 3 );
}

#[ tokio::test ]
async fn test_rollback_multiple_times() 
{
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( config );

  // Update 3 times with different thresholds
  for threshold in [5, 10, 15 ]
  {
  let mut new_config = ReliabilityConfig::default( );
  new_config.circuit_breaker = Some( CircuitBreakerConfig {
      failure_threshold : threshold,
      success_threshold : 2,
      timeout : Duration::from_secs( 60 ),
  } );
  dynamic_config.update( new_config ).await.unwrap( );
  }

  assert_eq!( dynamic_config.get( ).await.circuit_breaker.unwrap( ).failure_threshold, 15 );

  dynamic_config.rollback( ).await.unwrap( );
  assert_eq!( dynamic_config.get( ).await.circuit_breaker.unwrap( ).failure_threshold, 10 );

  dynamic_config.rollback( ).await.unwrap( );
  assert_eq!( dynamic_config.get( ).await.circuit_breaker.unwrap( ).failure_threshold, 5 );
}

#[ tokio::test ]
async fn test_rollback_fails_with_no_history() 
{
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( config );

  let result = dynamic_config.rollback( ).await;
  assert!( result.is_err( ), "Should fail with no history" );
}

#[ tokio::test ]
async fn test_rollback_notifies_watchers() 
{
  let mut initial_config = ReliabilityConfig::default( );
  initial_config.circuit_breaker = Some( CircuitBreakerConfig {
  failure_threshold : 3,
  success_threshold : 2,
  timeout : Duration::from_secs( 30 ),
  } );

  let dynamic_config = DynamicConfig::new( initial_config );

  let notified = Arc::new( RwLock::new( false ));
  let notified_clone = notified.clone( );

  dynamic_config.add_watcher( move |_, _| {
  let n = notified_clone.clone( );
  tokio::spawn( async move {
      *n.write( ).await = true;
  } );
  } ).await;

  let mut new_config = ReliabilityConfig::default( );
  new_config.circuit_breaker = Some( CircuitBreakerConfig {
  failure_threshold : 10,
  success_threshold : 5,
  timeout : Duration::from_secs( 120 ),
  } );

  dynamic_config.update( new_config ).await.unwrap( );

  // Reset notification flag
  *notified.write( ).await = false;

  dynamic_config.rollback( ).await.unwrap( );

  // Give watcher time to execute
  tokio::time::sleep( Duration::from_millis( 50 )).await;

  assert!( *notified.read( ).await, "Rollback should notify watchers" );
}

// ============================================================================
// Concurrent Access Tests
// ============================================================================

#[ tokio::test ]
async fn test_concurrent_updates() 
{
  let config = ReliabilityConfig::default( );
  let dynamic_config = Arc::new( DynamicConfig::new( config ));

  let mut handles = vec![ ];

  for threshold in 1..=10
  {
  let dc = dynamic_config.clone( );
  let handle = tokio::spawn( async move {
      let mut config = ReliabilityConfig::default( );
      config.circuit_breaker = Some( CircuitBreakerConfig {
  failure_threshold : threshold,
  success_threshold : 2,
  timeout : Duration::from_secs( 60 ),
      } );
      dc.update( config ).await
  } );
  handles.push( handle );
  }

  let mut successes = 0;
  for handle in handles
  {
  if let Ok( Ok( _ )) = handle.await
  {
      successes += 1;
  }
  }

  assert_eq!( successes, 10, "All concurrent updates should succeed" );
  assert_eq!( dynamic_config.history_size( ).await, 10 );
}

#[ tokio::test ]
async fn test_concurrent_reads_during_update() 
{
  let config = ReliabilityConfig::default( );
  let dynamic_config = Arc::new( DynamicConfig::new( config ));

  let mut handles = vec![ ];

  // Spawn update task
  let dc_update = dynamic_config.clone( );
  handles.push( tokio::spawn( async move {
  for _ in 0..5
  {
      dc_update.update( ReliabilityConfig::default( )).await.unwrap( );
      tokio::time::sleep( Duration::from_millis( 10 )).await;
  }
  } ));

  // Spawn multiple read tasks
  for _ in 0..10
  {
  let dc_read = dynamic_config.clone( );
  handles.push( tokio::spawn( async move {
      for _ in 0..20
      {
  let _ = dc_read.get( ).await;
  tokio::time::sleep( Duration::from_millis( 5 )).await;
      }
  } ));
  }

  for handle in handles
  {
  handle.await.unwrap( );
  }

  // If we got here without deadlock, test passed
}

// ============================================================================
// Configuration Clone and Timestamp Tests
// ============================================================================

#[ tokio::test ]
async fn test_config_timestamp_updates_on_change() 
{
  let initial_config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::new( initial_config );

  let timestamp1 = dynamic_config.get( ).await.timestamp;

  tokio::time::sleep( Duration::from_millis( 10 )).await;

  dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );

  let timestamp2 = dynamic_config.get( ).await.timestamp;

  assert!( timestamp2 > timestamp1, "Timestamp should update on config change" );
}

#[ tokio::test ]
async fn test_config_clone() 
{
  let mut config1 = ReliabilityConfig::default( );
  config1.circuit_breaker = Some( CircuitBreakerConfig {
  failure_threshold : 5,
  success_threshold : 2,
  timeout : Duration::from_secs( 60 ),
  } );

  let config2 = config1.clone( );

  assert!( config2.circuit_breaker.is_some( ));
  assert_eq!( config2.circuit_breaker.unwrap( ).failure_threshold, 5 );
}

// ============================================================================
// Edge Cases
// ============================================================================

#[ tokio::test ]
async fn test_update_with_none_values() 
{
  let mut initial_config = ReliabilityConfig::default( );
  initial_config.circuit_breaker = Some( CircuitBreakerConfig {
  failure_threshold : 5,
  success_threshold : 2,
  timeout : Duration::from_secs( 60 ),
  } );

  let dynamic_config = DynamicConfig::new( initial_config );

  // Update to config with all None
  let empty_config = ReliabilityConfig::default( );
  dynamic_config.update( empty_config ).await.unwrap( );

  let current = dynamic_config.get( ).await;
  assert!( current.circuit_breaker.is_none( ));
}

#[ tokio::test ]
async fn test_zero_history_size() 
{
  let config = ReliabilityConfig::default( );
  let dynamic_config = DynamicConfig::with_history_size( config, 0 );

  dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );
  dynamic_config.update( ReliabilityConfig::default( )).await.unwrap( );

  assert_eq!( dynamic_config.history_size( ).await, 0, "Should not keep history" );

  let result = dynamic_config.rollback( ).await;
  assert!( result.is_err( ), "Cannot rollback with zero history" );
}
