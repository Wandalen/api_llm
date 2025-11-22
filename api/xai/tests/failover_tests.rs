//! Unit tests for failover implementation.
//!
//! # Purpose
//!
//! Validates the failover manager's ability to track endpoint health
//! and rotate between multiple endpoints on failures.
//!
//! # Key Insights
//!
//! - **Health Tracking**: Endpoints transition through health states:
//!   Healthy -> Degraded (after first failure) -> Unhealthy (at threshold)
//!
//! - **Automatic Rotation**: When `auto_rotate` is enabled, the manager
//!   automatically switches to the next healthy endpoint when the current
//!   one becomes unhealthy (reaches `max_failures` threshold).
//!
//! - **Retry Cooldown**: Unhealthy endpoints cannot be used until the
//!   `retry_after` duration has elapsed. After cooldown, they transition
//!   to Degraded state and can be retried.
//!
//! - **Manual Rotation**: Developers can manually rotate to the next
//!   endpoint using `rotate()`. This respects health states and cooldowns.
//!
//! - **Success Reset**: A successful request resets the endpoint's failure
//!   counter to zero and marks it as Healthy.
//!
//! # Running Tests
//!
//! ```bash
//! cargo test --features failover --test failover_tests
//! ```

#![ cfg( feature = "failover" ) ]

use api_xai::{ FailoverManager, FailoverConfig, EndpointHealth };
use core::time::Duration;

#[ test ]
fn failover_manager_starts_with_first_endpoint()
{
  let manager = FailoverManager::new(
    vec![
      "https://api1.x.ai/v1/".to_string(),
      "https://api2.x.ai/v1/".to_string(),
    ],
    FailoverConfig::default()
  );

  assert_eq!( manager.current_endpoint(), "https://api1.x.ai/v1/" );
  assert_eq!( manager.current_index(), 0 );
  assert_eq!( manager.endpoint_count(), 2 );
}

#[ test ]
fn failover_tracks_endpoint_health()
{
  let config = FailoverConfig::default()
    .with_max_failures( 3 )
    .with_auto_rotate( false ); // Disable auto-rotation for this test

  let manager = FailoverManager::new(
    vec![
      "https://api1.x.ai/v1/".to_string(),
      "https://api2.x.ai/v1/".to_string(),
    ],
    config
  );

  // Check initial health
  let health = manager.endpoint_health();
  assert_eq!( health.len(), 2 );
  assert_eq!( health[ 0 ].1, EndpointHealth::Healthy );

  // First failure -> Degraded
  manager.record_failure();
  let health = manager.endpoint_health();
  assert_eq!( health[ 0 ].1, EndpointHealth::Degraded );

  // Second failure -> still Degraded
  manager.record_failure();
  let health = manager.endpoint_health();
  assert_eq!( health[ 0 ].1, EndpointHealth::Degraded );

  // Third failure -> Unhealthy
  manager.record_failure();
  let health = manager.endpoint_health();
  assert_eq!( health[ 0 ].1, EndpointHealth::Unhealthy );
}

#[ test ]
fn failover_rotates_automatically_on_unhealthy()
{
  let config = FailoverConfig::default()
    .with_max_failures( 2 )
    .with_auto_rotate( true );

  let manager = FailoverManager::new(
    vec![
      "https://api1.x.ai/v1/".to_string(),
      "https://api2.x.ai/v1/".to_string(),
    ],
    config
  );

  assert_eq!( manager.current_endpoint(), "https://api1.x.ai/v1/" );

  // First failure
  let rotated = manager.record_failure();
  assert!( !rotated, "Should not rotate on first failure" );

  // Second failure -> should trigger rotation
  let rotated = manager.record_failure();
  assert!( rotated, "Should rotate after reaching threshold" );
  assert_eq!( manager.current_endpoint(), "https://api2.x.ai/v1/" );
  assert_eq!( manager.current_index(), 1 );
}

#[ test ]
fn failover_success_resets_failure_count()
{
  let config = FailoverConfig::default()
    .with_max_failures( 3 )
    .with_auto_rotate( false );

  let manager = FailoverManager::new(
    vec![ "https://api1.x.ai/v1/".to_string() ],
    config
  );

  // Record some failures
  manager.record_failure();
  manager.record_failure();

  let health = manager.endpoint_health();
  assert_eq!( health[ 0 ].1, EndpointHealth::Degraded );

  // Record success -> should reset to Healthy
  manager.record_success();

  let health = manager.endpoint_health();
  assert_eq!( health[ 0 ].1, EndpointHealth::Healthy );

  // Now need 3 more failures to become unhealthy
  manager.record_failure();
  manager.record_failure();
  let health = manager.endpoint_health();
  assert_eq!( health[ 0 ].1, EndpointHealth::Degraded );

  manager.record_failure();
  let health = manager.endpoint_health();
  assert_eq!( health[ 0 ].1, EndpointHealth::Unhealthy );
}

#[ test ]
fn failover_manual_rotation_works()
{
  let manager = FailoverManager::new(
    vec![
      "https://api1.x.ai/v1/".to_string(),
      "https://api2.x.ai/v1/".to_string(),
      "https://api3.x.ai/v1/".to_string(),
    ],
    FailoverConfig::default()
  );

  assert_eq!( manager.current_index(), 0 );

  manager.rotate();
  assert_eq!( manager.current_index(), 1 );

  manager.rotate();
  assert_eq!( manager.current_index(), 2 );

  manager.rotate();
  assert_eq!( manager.current_index(), 0 ); // Wraps around
}

#[ test ]
fn failover_skips_unhealthy_endpoints_on_rotation()
{
  let config = FailoverConfig::default()
    .with_max_failures( 1 );

  let manager = FailoverManager::new(
    vec![
      "https://api1.x.ai/v1/".to_string(),
      "https://api2.x.ai/v1/".to_string(),
      "https://api3.x.ai/v1/".to_string(),
    ],
    config
  );

  // Mark endpoint 1 as unhealthy
  manager.record_failure();

  // Rotate should skip unhealthy endpoint 1 and go to endpoint 2
  manager.rotate();
  assert_eq!( manager.current_index(), 2, "Should skip index 1 (unhealthy)" );
}

#[ test ]
fn failover_respects_retry_cooldown()
{
  let config = FailoverConfig::default()
    .with_max_failures( 1 )
    .with_retry_after( Duration::from_millis( 100 ) );

  let manager = FailoverManager::new(
    vec![
      "https://api1.x.ai/v1/".to_string(),
      "https://api2.x.ai/v1/".to_string(),
    ],
    config
  );

  // Mark endpoint 0 as unhealthy
  manager.record_failure();
  let health = manager.endpoint_health();
  assert_eq!( health[ 0 ].1, EndpointHealth::Unhealthy );

  // Rotate should skip endpoint 0 (still in cooldown)
  manager.rotate();
  assert_eq!( manager.current_index(), 1 );

  // Wait for cooldown to expire
  std::thread::sleep( Duration::from_millis( 150 ) );

  // Now rotation should allow endpoint 0 (degraded state after cooldown)
  manager.rotate();
  assert_eq!( manager.current_index(), 0 );

  // Endpoint should be degraded now (cooldown elapsed)
  let health = manager.endpoint_health();
  assert_eq!( health[ 0 ].1, EndpointHealth::Degraded );
}

#[ test ]
fn failover_reset_restores_all_endpoints()
{
  let config = FailoverConfig::default()
    .with_max_failures( 1 );

  let manager = FailoverManager::new(
    vec![
      "https://api1.x.ai/v1/".to_string(),
      "https://api2.x.ai/v1/".to_string(),
    ],
    config
  );

  // Mark both endpoints as unhealthy
  manager.record_failure();
  manager.rotate();
  manager.record_failure();

  let health = manager.endpoint_health();
  assert_eq!( health[ 0 ].1, EndpointHealth::Unhealthy );
  assert_eq!( health[ 1 ].1, EndpointHealth::Unhealthy );

  // Reset
  manager.reset();

  // All should be healthy and index should be 0
  let health = manager.endpoint_health();
  assert_eq!( health[ 0 ].1, EndpointHealth::Healthy );
  assert_eq!( health[ 1 ].1, EndpointHealth::Healthy );
  assert_eq!( manager.current_index(), 0 );
}

#[ test ]
fn failover_config_builder_works()
{
  let config = FailoverConfig::default()
    .with_max_failures( 10 )
    .with_retry_after( Duration::from_secs( 120 ) )
    .with_auto_rotate( false );

  assert_eq!( config.max_failures, 10 );
  assert_eq!( config.retry_after, Duration::from_secs( 120 ) );
  assert!( !config.auto_rotate );
}

#[ test ]
#[ should_panic( expected = "Must provide at least one endpoint" ) ]
fn failover_panics_on_empty_endpoints()
{
  FailoverManager::new( vec![], FailoverConfig::default() );
}

#[ test ]
fn failover_handles_single_endpoint()
{
  let manager = FailoverManager::new(
    vec![ "https://api1.x.ai/v1/".to_string() ],
    FailoverConfig::default()
  );

  manager.record_failure();
  manager.record_failure();
  manager.record_failure();

  // Even with single unhealthy endpoint, rotation should still work
  // (it will use the same endpoint)
  manager.rotate();
  assert_eq!( manager.current_index(), 0 );
}

#[ test ]
fn failover_all_unhealthy_uses_next_anyway()
{
  let config = FailoverConfig::default()
    .with_max_failures( 1 )
    .with_auto_rotate( false ); // Disable auto-rotation for predictable test

  let manager = FailoverManager::new(
    vec![
      "https://api1.x.ai/v1/".to_string(),
      "https://api2.x.ai/v1/".to_string(),
    ],
    config
  );

  // Mark all endpoints as unhealthy
  manager.record_failure();
  manager.rotate();
  manager.record_failure();

  // Both are unhealthy, but rotation should still work (circular)
  manager.rotate();
  assert_eq!( manager.current_index(), 0 );
}
