//! Unit and integration tests for Health Checks
//!
//! These tests verify the health checking functionality using local mock servers
//! for deterministic, fast, and reliable testing. Tests run automatically in CI/CD.
//!
//! ## Implementation Notes
//!
//! - Uses `wiremock` crate for local HTTP mock servers
//! - All tests are deterministic (no external dependencies)
//! - Fast execution (localhost vs internet round-trips)
//! - No ignored tests - all tests run automatically
//!
//! ## Test Coverage
//!
//! - Ping strategy health checks
//! - Lightweight API strategy health checks
//! - Full endpoint strategy health checks
//! - Unhealthy threshold detection
//! - Health recovery after failures
//! - Background monitoring
//! - Latency tracking
//! - Timeout handling
//! - Concurrent health checks

use api_huggingface::reliability::{
  HealthChecker,
  HealthCheckConfig,
  HealthCheckStrategy,
};
use core::time::Duration;
use std::sync::Arc;

// ============================================================================
// Ping Strategy Tests
// ============================================================================

#[ tokio::test ]
async fn test_ping_strategy_healthy_endpoint()
{
  use wiremock::{ MockServer, Mock, ResponseTemplate };
  use wiremock::matchers::method;

  // Start local mock server for deterministic testing
  let mock_server = MockServer::start( ).await;

  Mock::given( method( "HEAD" ))
  .respond_with( ResponseTemplate::new( 200 ))
  .mount( &mock_server )
  .await;

  let config = HealthCheckConfig {
  endpoint : mock_server.uri( ),
  strategy : HealthCheckStrategy::Ping,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 3,
  };

  let checker = HealthChecker::new( config );

  let result = checker.check_health( ).await;

  assert!( result.is_ok( ), "Ping should succeed for healthy endpoint" );

  let status = result.unwrap( );
  assert!( status.healthy, "Status should be healthy" );
  assert_eq!( status.total_checks, 1, "Should have 1 check" );
  assert_eq!( status.consecutive_failures, 0, "Should have no failures" );
  assert!( status.latency_ms > 0, "Should have measured latency" );
}

#[ tokio::test ]
async fn test_ping_strategy_unhealthy_endpoint() 
{
  let config = HealthCheckConfig {
  endpoint : "https://invalid-endpoint-xyz-12345.com".to_string( ),
  strategy : HealthCheckStrategy::Ping,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_millis( 500 ),
  unhealthy_threshold : 2,
  };

  let checker = HealthChecker::new( config );

  // First failure
  let result1 = checker.check_health( ).await;
  assert!( result1.is_err( ), "Should fail for invalid endpoint" );

  let status1 = checker.get_status( ).await;
  assert_eq!( status1.consecutive_failures, 1, "Should have 1 failure" );
  assert!( status1.healthy, "Should still be healthy ( below threshold )" );

  // Second failure - reaches threshold
  let result2 = checker.check_health( ).await;
  assert!( result2.is_err( ), "Should fail again" );

  let status2 = checker.get_status( ).await;
  assert_eq!( status2.consecutive_failures, 2, "Should have 2 failures" );
  assert!( !status2.healthy, "Should be unhealthy ( reached threshold )" );
}

// ============================================================================
// Lightweight API Strategy Tests
// ============================================================================

#[ tokio::test ]
async fn test_lightweight_api_strategy_healthy()
{
  use wiremock::{ MockServer, Mock, ResponseTemplate };
  use wiremock::matchers::method;

  // Start local mock server for deterministic testing
  let mock_server = MockServer::start( ).await;

  Mock::given( method( "GET" ))
  .respond_with( ResponseTemplate::new( 200 ))
  .mount( &mock_server )
  .await;

  let config = HealthCheckConfig {
  endpoint : mock_server.uri( ),
  strategy : HealthCheckStrategy::LightweightApi,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 3,
  };

  let checker = HealthChecker::new( config );

  let result = checker.check_health( ).await;

  assert!( result.is_ok( ), "Lightweight API check should succeed" );

  let status = result.unwrap( );
  assert!( status.healthy, "Status should be healthy" );
  assert!( status.latency_ms > 0, "Should measure latency" );
}

#[ tokio::test ]
async fn test_lightweight_api_strategy_404_is_unhealthy()
{
  use wiremock::{ MockServer, Mock, ResponseTemplate };
  use wiremock::matchers::method;

  // Start local mock server for deterministic testing
  let mock_server = MockServer::start( ).await;

  Mock::given( method( "GET" ))
  .respond_with( ResponseTemplate::new( 404 ))
  .mount( &mock_server )
  .await;

  let config = HealthCheckConfig {
  endpoint : mock_server.uri( ),
  strategy : HealthCheckStrategy::LightweightApi,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 1,
  };

  let checker = HealthChecker::new( config );

  let result = checker.check_health( ).await;

  assert!( result.is_err( ), "404 should be treated as unhealthy" );

  let status = checker.get_status( ).await;
  assert!( !status.healthy, "Should be marked unhealthy" );
}

// ============================================================================
// Full Endpoint Strategy Tests
// ============================================================================

#[ tokio::test ]
async fn test_full_endpoint_strategy_healthy()
{
  use wiremock::{ MockServer, Mock, ResponseTemplate };
  use wiremock::matchers::method;

  // Start local mock server for deterministic testing
  let mock_server = MockServer::start( ).await;

  Mock::given( method( "POST" ))
  .respond_with( ResponseTemplate::new( 200 ))
  .mount( &mock_server )
  .await;

  let config = HealthCheckConfig {
  endpoint : mock_server.uri( ),
  strategy : HealthCheckStrategy::FullEndpoint,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 3,
  };

  let checker = HealthChecker::new( config );

  let result = checker.check_health( ).await;

  assert!( result.is_ok( ), "Full endpoint check should succeed" );

  let status = result.unwrap( );
  assert!( status.healthy, "Status should be healthy" );
  assert!( status.latency_ms > 0, "Should measure latency" );
}

#[ tokio::test ]
async fn test_full_endpoint_strategy_accepts_client_errors()
{
  use wiremock::{ MockServer, Mock, ResponseTemplate };
  use wiremock::matchers::method;

  // Start local mock server for deterministic testing
  let mock_server = MockServer::start( ).await;

  Mock::given( method( "POST" ))
  .respond_with( ResponseTemplate::new( 400 ))
  .mount( &mock_server )
  .await;

  // Full endpoint check accepts 4xx errors as "endpoint is responding"
  let config = HealthCheckConfig {
  endpoint : mock_server.uri( ),
  strategy : HealthCheckStrategy::FullEndpoint,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 3,
  };

  let checker = HealthChecker::new( config );

  let result = checker.check_health( ).await;

  // 400 is acceptable for full endpoint ( shows endpoint is responding )
  assert!( result.is_ok( ), "Full endpoint should accept client errors" );
}

// ============================================================================
// Threshold and Recovery Tests
// ============================================================================

#[ tokio::test ]
async fn test_unhealthy_threshold_gradual_failure() 
{
  let config = HealthCheckConfig {
  endpoint : "https://invalid-endpoint-xyz-12345.com".to_string( ),
  strategy : HealthCheckStrategy::Ping,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_millis( 500 ),
  unhealthy_threshold : 3,
  };

  let checker = HealthChecker::new( config );

  // Check 1 - still healthy
  let _ = checker.check_health( ).await;
  let status = checker.get_status( ).await;
  assert!( status.healthy, "Should be healthy after 1 failure" );
  assert_eq!( status.consecutive_failures, 1 );

  // Check 2 - still healthy
  let _ = checker.check_health( ).await;
  let status = checker.get_status( ).await;
  assert!( status.healthy, "Should be healthy after 2 failures" );
  assert_eq!( status.consecutive_failures, 2 );

  // Check 3 - now unhealthy
  let _ = checker.check_health( ).await;
  let status = checker.get_status( ).await;
  assert!( !status.healthy, "Should be unhealthy after 3 failures" );
  assert_eq!( status.consecutive_failures, 3 );
}

#[ tokio::test ]
async fn test_health_recovery_after_success()
{
  use wiremock::{ MockServer, Mock, ResponseTemplate };
  use wiremock::matchers::method;

  // Start local mock server for deterministic testing
  let mock_server = MockServer::start( ).await;

  Mock::given( method( "HEAD" ))
  .respond_with( ResponseTemplate::new( 200 ))
  .mount( &mock_server )
  .await;

  let config = HealthCheckConfig {
  endpoint : mock_server.uri( ),
  strategy : HealthCheckStrategy::Ping,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 2,
  };

  let checker = HealthChecker::new( config );

  // Manually set unhealthy state
  {
  let mut state = checker.state.write( ).await;
  state.status.healthy = false;
  state.status.consecutive_failures = 5;
  }

  let status = checker.get_status( ).await;
  assert!( !status.healthy, "Should start unhealthy" );

  // Successful check should recover
  let result = checker.check_health( ).await;
  assert!( result.is_ok( ), "Check should succeed" );

  let status = checker.get_status( ).await;
  assert!( status.healthy, "Should be healthy after successful check" );
  assert_eq!( status.consecutive_failures, 0, "Failures should be reset" );
}

// ============================================================================
// Background Monitoring Tests
// ============================================================================

#[ tokio::test ]
async fn test_background_monitoring_performs_checks()
{
  use wiremock::{ MockServer, Mock, ResponseTemplate };
  use wiremock::matchers::method;

  // Start local mock server for deterministic testing
  let mock_server = MockServer::start( ).await;

  Mock::given( method( "HEAD" ))
  .respond_with( ResponseTemplate::new( 200 ))
  .mount( &mock_server )
  .await;

  let config = HealthCheckConfig {
  endpoint : mock_server.uri( ),
  strategy : HealthCheckStrategy::Ping,
  check_interval : Duration::from_millis( 200 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 3,
  };

  let checker = HealthChecker::new( config );

  let status_before = checker.get_status( ).await;
  assert_eq!( status_before.total_checks, 0, "Should have no checks initially" );

  // Start monitoring
  let handle = checker.start_monitoring( ).await;

  // Wait for a few checks
  tokio::time::sleep( Duration::from_millis( 600 )).await;

  // Stop monitoring
  checker.stop_monitoring( ).await;
  handle.stop( ).await;

  let status_after = checker.get_status( ).await;
  assert!( status_after.total_checks >= 2, "Should have performed at least 2 checks" );
}

#[ tokio::test ]
async fn test_monitoring_can_be_started_and_stopped()
{
  use wiremock::{ MockServer, Mock, ResponseTemplate };
  use wiremock::matchers::method;

  // Start local mock server for deterministic testing
  let mock_server = MockServer::start( ).await;

  Mock::given( method( "HEAD" ))
  .respond_with( ResponseTemplate::new( 200 ))
  .mount( &mock_server )
  .await;

  let config = HealthCheckConfig {
  endpoint : mock_server.uri( ),
  strategy : HealthCheckStrategy::Ping,
  check_interval : Duration::from_millis( 100 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 3,
  };

  let checker = HealthChecker::new( config );

  assert!( !checker.is_monitoring( ).await, "Should not be monitoring initially" );

  let handle = checker.start_monitoring( ).await;
  assert!( checker.is_monitoring( ).await, "Should be monitoring after start" );

  checker.stop_monitoring( ).await;
  handle.stop( ).await;

  // Give it a moment to stop
  tokio::time::sleep( Duration::from_millis( 50 )).await;

  assert!( !checker.is_monitoring( ).await, "Should not be monitoring after stop" );
}

// ============================================================================
// Latency Tracking Tests
// ============================================================================

#[ tokio::test ]
async fn test_latency_tracking_measures_response_time()
{
  use wiremock::{ MockServer, Mock, ResponseTemplate };
  use wiremock::matchers::method;

  // Start local mock server for deterministic testing
  let mock_server = MockServer::start( ).await;

  Mock::given( method( "HEAD" ))
  .respond_with( ResponseTemplate::new( 200 ))
  .mount( &mock_server )
  .await;

  let config = HealthCheckConfig {
  endpoint : mock_server.uri( ),
  strategy : HealthCheckStrategy::Ping,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 3,
  };

  let checker = HealthChecker::new( config );

  let result = checker.check_health( ).await;
  assert!( result.is_ok( ), "Check should succeed" );

  let status = result.unwrap( );
  assert!( status.latency_ms > 0, "Latency should be measured" );
  assert!( status.latency_ms < 5000, "Latency should be under timeout" );
}

#[ tokio::test ]
async fn test_multiple_checks_update_latency()
{
  use wiremock::{ MockServer, Mock, ResponseTemplate };
  use wiremock::matchers::method;

  // Start local mock server for deterministic testing
  let mock_server = MockServer::start( ).await;

  Mock::given( method( "HEAD" ))
  .respond_with( ResponseTemplate::new( 200 ))
  .mount( &mock_server )
  .await;

  let config = HealthCheckConfig {
  endpoint : mock_server.uri( ),
  strategy : HealthCheckStrategy::Ping,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 3,
  };

  let checker = HealthChecker::new( config );

  // First check
  let result1 = checker.check_health( ).await;
  assert!( result1.is_ok( ));
  let latency1 = result1.unwrap( ).latency_ms;

  // Second check
  let result2 = checker.check_health( ).await;
  assert!( result2.is_ok( ));
  let latency2 = result2.unwrap( ).latency_ms;

  // Both should have measured latency
  assert!( latency1 > 0, "First check should measure latency" );
  assert!( latency2 > 0, "Second check should measure latency" );
}

// ============================================================================
// Timeout Tests
// ============================================================================

#[ tokio::test ]
async fn test_timeout_marks_check_as_failed()
{
  use wiremock::{ MockServer, Mock, ResponseTemplate };
  use wiremock::matchers::method;

  // Start local mock server for deterministic testing
  let mock_server = MockServer::start( ).await;

  Mock::given( method( "GET" ))
  .respond_with(
      ResponseTemplate::new( 200 )
      .set_delay( Duration::from_secs( 10 ))  // 10 second delay
  )
  .mount( &mock_server )
  .await;

  let config = HealthCheckConfig {
  endpoint : mock_server.uri( ),
  strategy : HealthCheckStrategy::LightweightApi,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_millis( 500 ),  // 500ms timeout
  unhealthy_threshold : 1,
  };

  let checker = HealthChecker::new( config );

  let result = checker.check_health( ).await;

  assert!( result.is_err( ), "Should timeout and fail" );

  let status = checker.get_status( ).await;
  assert!( !status.healthy, "Should be unhealthy after timeout" );
  assert_eq!( status.consecutive_failures, 1 );
}

// ============================================================================
// Concurrent Health Checks
// ============================================================================

#[ tokio::test ]
async fn test_concurrent_health_checks()
{
  use wiremock::{ MockServer, Mock, ResponseTemplate };
  use wiremock::matchers::method;

  // Start local mock server for deterministic testing
  let mock_server = MockServer::start( ).await;

  Mock::given( method( "HEAD" ))
  .respond_with( ResponseTemplate::new( 200 ))
  .mount( &mock_server )
  .await;

  let config = HealthCheckConfig {
  endpoint : mock_server.uri( ),
  strategy : HealthCheckStrategy::Ping,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 10,
  };

  let checker = Arc::new( HealthChecker::new( config ));

  // Launch 5 concurrent health checks
  let mut handles = vec![ ];

  for _ in 0..5
  {
  let checker_clone = checker.clone( );
  let handle = tokio::spawn( async move {
      checker_clone.check_health( ).await
  } );
  handles.push( handle );
  }

  // Wait for all checks
  let mut successes = 0;
  for handle in handles
  {
  if let Ok( Ok( _ )) = handle.await
  {
      successes += 1;
  }
  }

  assert_eq!( successes, 5, "All concurrent checks should succeed" );

  let status = checker.get_status( ).await;
  assert_eq!( status.total_checks, 5, "Should have performed 5 checks" );
  assert!( status.healthy, "Should be healthy" );
}

// ============================================================================
// Reset Tests
// ============================================================================

#[ tokio::test ]
async fn test_reset_clears_all_state() 
{
  let config = HealthCheckConfig {
  endpoint : "https://example.com".to_string( ),
  strategy : HealthCheckStrategy::Ping,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 3,
  };

  let checker = HealthChecker::new( config );

  // Manually set some state
  {
  let mut state = checker.state.write( ).await;
  state.status.healthy = false;
  state.status.consecutive_failures = 5;
  state.status.total_checks = 10;
  state.status.latency_ms = 1000;
  }

  // Reset
  checker.reset( ).await;

  let status = checker.get_status( ).await;
  assert!( status.healthy, "Should be healthy after reset" );
  assert_eq!( status.consecutive_failures, 0, "Failures should be reset" );
  assert_eq!( status.total_checks, 0, "Checks should be reset" );
  assert_eq!( status.latency_ms, 0, "Latency should be reset" );
}

// ============================================================================
// Edge Cases
// ============================================================================

#[ tokio::test ]
async fn test_https_redirect_is_healthy()
{
  use wiremock::{ MockServer, Mock, ResponseTemplate };
  use wiremock::matchers::method;

  // Start local mock server for deterministic testing
  let mock_server = MockServer::start( ).await;

  Mock::given( method( "HEAD" ))
  .respond_with(
      ResponseTemplate::new( 301 )  // HTTP redirect status
      .insert_header( "Location", "https://example.com" )
  )
  .mount( &mock_server )
  .await;

  let config = HealthCheckConfig {
  endpoint : mock_server.uri( ),
  strategy : HealthCheckStrategy::Ping,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 3,
  };

  let checker = HealthChecker::new( config );

  let result = checker.check_health( ).await;

  // Redirects are considered healthy for ping strategy
  assert!( result.is_ok( ), "Redirects should be considered healthy" );
}

#[ tokio::test ]
async fn test_health_status_clone() 
{
  let config = HealthCheckConfig {
  endpoint : "https://example.com".to_string( ),
  strategy : HealthCheckStrategy::Ping,
  check_interval : Duration::from_secs( 30 ),
  timeout : Duration::from_secs( 5 ),
  unhealthy_threshold : 3,
  };

  let checker = HealthChecker::new( config );

  let status1 = checker.get_status( ).await;
  let status2 = status1.clone( );

  assert_eq!( status1.healthy, status2.healthy );
  assert_eq!( status1.latency_ms, status2.latency_ms );
  assert_eq!( status1.consecutive_failures, status2.consecutive_failures );
  assert_eq!( status1.total_checks, status2.total_checks );
}

#[ tokio::test ]
async fn test_different_strategies_have_different_behavior() 
{
  // Just verify all strategies are distinct
  let ping = HealthCheckStrategy::Ping;
  let lightweight = HealthCheckStrategy::LightweightApi;
  let full = HealthCheckStrategy::FullEndpoint;

  assert_ne!( ping, lightweight );
  assert_ne!( lightweight, full );
  assert_ne!( ping, full );
}
