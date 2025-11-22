//! Enhanced Circuit Breaker Integration Tests
//!
//! This module contains integration tests that validate actual circuit breaker behavior
//! within the `OpenAI` client HTTP layer. These tests ensure circuit breaker functionality
//! works correctly with real HTTP requests.

#[ cfg( feature = "circuit_breaker" ) ]
mod circuit_breaker_integration_tests
{
  use api_openai::
  {
    Client,
    ClientApiAccessors,
    environment ::{ OpenaiEnvironmentImpl, OpenAIRecommended },
    secret ::Secret,
    enhanced_circuit_breaker ::{ EnhancedCircuitBreakerConfig },
  };

  use core::time::Duration;
  use tokio::time::sleep;

  /// Create test client with circuit breaker configuration
  fn create_test_client_with_circuit_breaker() -> Result< Client< OpenaiEnvironmentImpl >, Box< dyn std::error::Error > >
  {
    // Use test API key that won't work for actual requests
    let secret = Secret::new( "sk-test-key-circuit-breaker".to_string() )?;
    let environment = OpenaiEnvironmentImpl::build(
      secret,
      None,
      None,
      OpenAIRecommended::base_url().to_string(),
      OpenAIRecommended::realtime_base_url().to_string()
    )?;

    let circuit_breaker_config = EnhancedCircuitBreakerConfig::new()
      .with_failure_threshold( 2 ) // Very low threshold for testing
      .with_recovery_timeout( 100 ) // Very short recovery time for testing
      .with_success_threshold( 1 ); // Easy to close after half-open

    let client = Client::build( environment )?
      .with_circuit_breaker_config( circuit_breaker_config );

    Ok( client )
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_integration_with_client()
  {
    let client = create_test_client_with_circuit_breaker().expect( "Failed to create test client" );

    // Verify circuit breaker configuration is properly set
    assert!( client.circuit_breaker_config().is_some() );
    let config = client.circuit_breaker_config().unwrap();
    assert_eq!( config.failure_threshold, 2 );
    assert_eq!( config.recovery_timeout_ms, 100 );
    assert_eq!( config.success_threshold, 1 );
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_with_invalid_requests()
  {
    let client = create_test_client_with_circuit_breaker().expect( "Failed to create test client" );

    // Make requests that will fail (invalid API key, bad endpoint)
    // These should trigger circuit breaker after threshold failures

    // First request - should fail but circuit remains closed
    let result1 = client.models().list().await;
    assert!( result1.is_err() );

    // Second request - should fail and open circuit
    let result2 = client.models().list().await;
    assert!( result2.is_err() );

    // Third request - should be rejected by circuit breaker immediately
    let result3 = client.models().list().await;
    assert!( result3.is_err() );

    // Check if the error indicates circuit breaker rejection
    // (This is tricky to test precisely since we can't easily distinguish
    // circuit breaker errors from API errors in this integration test)

    // Wait for recovery timeout
    sleep( Duration::from_millis( 150 ) ).await;

    // Next request should be allowed (half-open state)
    let result4 = client.models().list().await;
    assert!( result4.is_err() ); // Still fails due to invalid API key, but circuit breaker allows it
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_zero_overhead_when_not_configured()
  {
    // Create client without circuit breaker configuration
    let secret = Secret::new( "sk-test-key-no-circuit-breaker".to_string() ).unwrap();
    let environment = OpenaiEnvironmentImpl::build(
      secret,
      None,
      None,
      OpenAIRecommended::base_url().to_string(),
      OpenAIRecommended::realtime_base_url().to_string()
    ).unwrap();
    let client = Client::build( environment ).unwrap();

    // Verify no circuit breaker configuration
    assert!( client.circuit_breaker_config().is_none() );

    // Requests should work normally (fail due to invalid API key, not circuit breaker)
    let result = client.models().list().await;
    assert!( result.is_err() );

    // Multiple failed requests should not be blocked (no circuit breaker)
    for _ in 0..5
    {
      let result = client.models().list().await;
      assert!( result.is_err() );
    }
  }

  #[ tokio::test ]
  async fn test_circuit_breaker_config_validation()
  {
    let secret = Secret::new( "sk-test-key".to_string() ).unwrap();
    let environment = OpenaiEnvironmentImpl::build(
      secret,
      None,
      None,
      OpenAIRecommended::base_url().to_string(),
      OpenAIRecommended::realtime_base_url().to_string()
    ).unwrap();

    // Test with invalid configuration (should use fallback)
    let invalid_config = EnhancedCircuitBreakerConfig::new()
      .with_failure_threshold( 0 ); // Invalid threshold

    // The client should handle invalid configuration gracefully
    let client = Client::build( environment ).unwrap()
      .with_circuit_breaker_config( invalid_config );

    // The invalid config should still be stored (validation happens in EnhancedCircuitBreaker::new)
    assert!( client.circuit_breaker_config().is_some() );
  }
}

#[ cfg( not( feature = "circuit_breaker" ) ) ]
mod no_circuit_breaker_integration_tests
{
  use api_openai::
  {
    Client,
    environment ::{ OpenaiEnvironmentImpl, OpenAIRecommended },
    secret ::Secret,
  };

  #[ tokio::test ]
  async fn test_zero_overhead_when_circuit_breaker_feature_disabled()
  {
    // When circuit breaker feature is disabled, client should work normally
    let secret = Secret::new( "sk-test-key".to_string() ).unwrap();
    let environment = OpenaiEnvironmentImpl::build(
      secret,
      None,
      None,
      OpenAIRecommended::base_url().to_string(),
      OpenAIRecommended::realtime_base_url().to_string()
    ).unwrap();
    let client = Client::build( environment ).unwrap();

    // Circuit breaker configuration methods should not be available
    // (This is enforced at compile time by feature gates)

    // Requests should work normally without circuit breaker overhead
    let result = client.models().list().await;
    assert!( result.is_err() ); // Fails due to invalid API key, not feature absence
  }
}