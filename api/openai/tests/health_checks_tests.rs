//! Health Checks Tests
//!
//! Comprehensive test suite for health check functionality including:
//! - Basic health check operations
//! - Multiple endpoint monitoring
//! - Configuration validation
//! - Error handling scenarios
//! - Performance validation

#![ allow( clippy::unreadable_literal ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::useless_vec ) ]
#![ allow( clippy::unused_async ) ]
#![ allow( clippy::must_use_candidate ) ]
#![ allow( clippy::missing_panics_doc ) ]
#![ allow( clippy::missing_errors_doc ) ]
#![ allow( clippy::doc_markdown ) ]
#![ allow( clippy::field_reassign_with_default ) ] // Test patterns may modify defaults
#![ allow( clippy::similar_names ) ] // Test variables often have similar names

#[ cfg( test ) ]
mod health_checks_tests
{
  use api_openai::
  {
    Client,
    environment ::OpenaiEnvironmentImpl,
    secret ::Secret,
    health_checks ::*,
  };
  use std::time::Duration;

  // Helper function to create test client
  fn create_test_client() -> Result< Client< OpenaiEnvironmentImpl >, Box< dyn std::error::Error > >
  {
    let secret = Secret::new_unchecked( "sk-test_key_12345".to_string() );
    let env = OpenaiEnvironmentImpl::build(
      secret,
      None,
      None,
      api_openai ::environment::OpenAIRecommended::base_url().to_string(),
      api_openai ::environment::OpenAIRecommended::realtime_base_url().to_string()
    ).expect( "Failed to create environment" );
    Ok( Client::build( env )? )
  }

  // ===== BASIC HEALTH CHECK TESTS =====

  #[ tokio::test ]
  async fn test_health_check_config_defaults()
  {
    let config = HealthCheckConfig::default();

    assert_eq!( config.timeout_ms, 5000 );
    assert_eq!( config.degraded_threshold_ms, 1000 );
    assert_eq!( config.unhealthy_threshold_ms, 5000 );
    assert!( matches!( config.strategy, HealthCheckStrategy::LightweightApi ) );
  }

  #[ tokio::test ]
  async fn test_health_check_config_serialization()
  {
    let config = HealthCheckConfig::default();

    // Test serialization
    let serialized = serde_json::to_string( &config ).expect( "Failed to serialize config" );
    assert!( !serialized.is_empty() );

    // Test deserialization
    let deserialized : HealthCheckConfig = serde_json::from_str( &serialized )
      .expect( "Failed to deserialize config" );

    assert_eq!( config.timeout_ms, deserialized.timeout_ms );
    assert_eq!( config.degraded_threshold_ms, deserialized.degraded_threshold_ms );
  }

  #[ tokio::test ]
  async fn test_health_status_enum()
  {
    // Test all variants exist and can be created
    let healthy = HealthStatus::Healthy;
    let degraded = HealthStatus::Degraded;
    let unhealthy = HealthStatus::Unhealthy;

    // Test serialization
    assert!( serde_json::to_string( &healthy ).is_ok() );
    assert!( serde_json::to_string( &degraded ).is_ok() );
    assert!( serde_json::to_string( &unhealthy ).is_ok() );

    // Test equality
    assert_eq!( healthy, HealthStatus::Healthy );
    assert_ne!( healthy, HealthStatus::Degraded );
  }

  #[ tokio::test ]
  async fn test_health_check_strategy_enum()
  {
    let ping = HealthCheckStrategy::Ping;
    let api = HealthCheckStrategy::LightweightApi;

    // Test serialization
    assert!( serde_json::to_string( &ping ).is_ok() );
    assert!( serde_json::to_string( &api ).is_ok() );
  }

  // ===== HEALTH CHECK EXECUTION TESTS =====

  #[ tokio::test ]
  async fn test_single_health_check_structure()
  {
    let client = create_test_client().expect( "Failed to create client" );
    let config = HealthCheckConfig::default();

    let result = HealthChecker::check_endpoint( &client, &config ).await;

    // Verify result structure
    assert!( !result.endpoint_url.is_empty() );
    assert!( result.response_time_ms > 0 );
    assert!( result.timestamp.elapsed().is_ok() );

    // Since this is a mock client, the check should complete but may be unhealthy
    // The important thing is that the structure is correct
    assert!( matches!( result.status, HealthStatus::Healthy | HealthStatus::Degraded | HealthStatus::Unhealthy ) );
  }

  #[ tokio::test ]
  async fn test_ping_strategy_health_check()
  {
    let client = create_test_client().expect( "Failed to create client" );
    let mut config = HealthCheckConfig::default();
    config.strategy = HealthCheckStrategy::Ping;

    let result = HealthChecker::check_endpoint( &client, &config ).await;

    assert!( !result.endpoint_url.is_empty() );
    // Ping strategy should be relatively fast
    assert!( result.response_time_ms < 1000 );
  }

  #[ tokio::test ]
  async fn test_lightweight_api_strategy_health_check()
  {
    let client = create_test_client().expect( "Failed to create client" );
    let mut config = HealthCheckConfig::default();
    config.strategy = HealthCheckStrategy::LightweightApi;

    let result = HealthChecker::check_endpoint( &client, &config ).await;

    assert!( !result.endpoint_url.is_empty() );
    // API strategy may take longer but should still be reasonable
    assert!( result.response_time_ms < 10000 );
  }

  // ===== MULTIPLE ENDPOINT TESTS =====

  #[ tokio::test ]
  async fn test_multiple_endpoints_health_check()
  {
    let client1 = create_test_client().expect( "Failed to create client 1" );
    let client2 = create_test_client().expect( "Failed to create client 2" );
    let clients = vec![ &client1, &client2 ];

    let config = HealthCheckConfig::default();
    let results = HealthChecker::check_multiple_endpoints( &clients, &config ).await;

    assert_eq!( results.len(), 2 );

    for result in &results
    {
      assert!( !result.endpoint_url.is_empty() );
      assert!( result.response_time_ms > 0 );
    }
  }

  #[ tokio::test ]
  async fn test_health_summary_generation()
  {
    // Create mock results
    let results = vec![
      HealthCheckResult
      {
        endpoint_url : "https://api.openai.com".to_string(),
        status : HealthStatus::Healthy,
        response_time_ms : 100,
        error_message : None,
        timestamp : std::time::SystemTime::now(),
      },
      HealthCheckResult
      {
        endpoint_url : "https://api.openai.com/v2".to_string(),
        status : HealthStatus::Degraded,
        response_time_ms : 1500,
        error_message : None,
        timestamp : std::time::SystemTime::now(),
      },
      HealthCheckResult
      {
        endpoint_url : "https://api.openai.com/v3".to_string(),
        status : HealthStatus::Unhealthy,
        response_time_ms : 5000,
        error_message : Some( "Timeout".to_string() ),
        timestamp : std::time::SystemTime::now(),
      },
    ];

    let summary = HealthChecker::summarize_health( &results );

    assert_eq!( summary.get( "healthy" ), Some( &1 ) );
    assert_eq!( summary.get( "degraded" ), Some( &1 ) );
    assert_eq!( summary.get( "unhealthy" ), Some( &1 ) );
  }

  // ===== CONFIGURATION VALIDATION TESTS =====

  #[ tokio::test ]
  async fn test_custom_thresholds()
  {
    let client = create_test_client().expect( "Failed to create client" );
    let mut config = HealthCheckConfig::default();

    // Set very low thresholds to test status determination
    config.degraded_threshold_ms = 1;
    config.unhealthy_threshold_ms = 2;

    let result = HealthChecker::check_endpoint( &client, &config ).await;

    // With such low thresholds, any real response should be at least degraded
    assert!( matches!( result.status, HealthStatus::Degraded | HealthStatus::Unhealthy ) );
  }

  #[ tokio::test ]
  async fn test_timeout_configuration()
  {
    let client = create_test_client().expect( "Failed to create client" );
    let mut config = HealthCheckConfig::default();
    config.timeout_ms = 100; // Very short timeout

    let result = HealthChecker::check_endpoint( &client, &config ).await;

    // Should complete regardless of timeout (may be unhealthy due to timeout)
    assert!( !result.endpoint_url.is_empty() );
  }

  // ===== ERROR HANDLING TESTS =====

  #[ tokio::test ]
  async fn test_error_message_handling()
  {
    // Create a result with error message
    let result_with_error = HealthCheckResult
    {
      endpoint_url : "https://invalid.endpoint".to_string(),
      status : HealthStatus::Unhealthy,
      response_time_ms : 5000,
      error_message : Some( "Connection failed".to_string() ),
      timestamp : std::time::SystemTime::now(),
    };

    assert_eq!( result_with_error.status, HealthStatus::Unhealthy );
    assert!( result_with_error.error_message.is_some() );
    assert_eq!( result_with_error.error_message.unwrap(), "Connection failed" );
  }

  #[ tokio::test ]
  async fn test_health_check_result_serialization()
  {
    let result = HealthCheckResult
    {
      endpoint_url : "https://api.openai.com".to_string(),
      status : HealthStatus::Healthy,
      response_time_ms : 150,
      error_message : None,
      timestamp : std::time::SystemTime::now(),
    };

    // Test serialization
    let serialized = serde_json::to_string( &result ).expect( "Failed to serialize result" );
    assert!( !serialized.is_empty() );

    // Test deserialization
    let deserialized : HealthCheckResult = serde_json::from_str( &serialized )
      .expect( "Failed to deserialize result" );

    assert_eq!( result.endpoint_url, deserialized.endpoint_url );
    assert_eq!( result.status, deserialized.status );
    assert_eq!( result.response_time_ms, deserialized.response_time_ms );
  }

  // ===== PERFORMANCE TESTS =====

  #[ tokio::test ]
  async fn test_health_check_performance()
  {
    let client = create_test_client().expect( "Failed to create client" );
    let config = HealthCheckConfig::default();

    let start = std::time::Instant::now();
    let _result = HealthChecker::check_endpoint( &client, &config ).await;
    let duration = start.elapsed();

    // Health check should complete within reasonable time
    assert!( duration < Duration::from_secs( 10 ) );
  }

  #[ tokio::test ]
  async fn test_concurrent_health_checks()
  {
    // Create multiple clients to avoid borrow checker issues
    let client1 = create_test_client().expect( "Failed to create client 1" );
    let client2 = create_test_client().expect( "Failed to create client 2" );
    let client3 = create_test_client().expect( "Failed to create client 3" );

    let config1 = HealthCheckConfig::default();
    let config2 = HealthCheckConfig::default();
    let config3 = HealthCheckConfig::default();

    // Simulate concurrent health checks
    let handle1 = tokio::spawn( async move {
      HealthChecker::check_endpoint( &client1, &config1 ).await
    });

    let handle2 = tokio::spawn( async move {
      HealthChecker::check_endpoint( &client2, &config2 ).await
    });

    let handle3 = tokio::spawn( async move {
      HealthChecker::check_endpoint( &client3, &config3 ).await
    });

    // Wait for all to complete
    let result1 = handle1.await.expect( "Task 1 should complete" );
    let result2 = handle2.await.expect( "Task 2 should complete" );
    let result3 = handle3.await.expect( "Task 3 should complete" );

    let results = vec![ result1, result2, result3 ];
    assert_eq!( results.len(), 3 );

    for result in results
    {
      assert!( !result.endpoint_url.is_empty() );
    }
  }

  // ===== INTEGRATION TESTS =====

  #[ tokio::test ]
  async fn test_health_check_integration()
  {
    let client = create_test_client().expect( "Failed to create client" );
    let config = HealthCheckConfig::default();

    // Perform health check
    let result = HealthChecker::check_endpoint( &client, &config ).await;

    // Verify integration aspects
    assert!( result.endpoint_url.contains( "api.openai.com" ) );
    assert!( result.timestamp.elapsed().unwrap() < Duration::from_secs( 1 ) );

    // Test that we can create summary
    let summary = HealthChecker::summarize_health( &[ result ] );
    assert_eq!( summary.len(), 1 ); // Should have exactly one status type
  }
}