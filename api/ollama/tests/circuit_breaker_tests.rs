//! Circuit breaker functionality tests for `api_ollama` crate.
//!
//! These tests verify the circuit breaker pattern implementation that prevents
//! cascading failures by tracking request failures and automatically stopping
//! requests when failure thresholds are exceeded.

#![ cfg( feature = "circuit_breaker" ) ]

use api_ollama::{ OllamaClient, CircuitBreaker, CircuitBreakerConfig, CircuitBreakerState };
use api_ollama::{ ChatRequest, ChatMessage, MessageRole, GenerateRequest };
use core::time::Duration;
use std::sync::Arc;
use tokio::time::{ sleep, Instant };

#[ tokio::test ]
async fn test_circuit_breaker_creation_and_configuration()
{
  // Test creating a circuit breaker with default configuration
  let config = CircuitBreakerConfig::default();
  assert_eq!(config.failure_threshold(), 5);
  assert_eq!(config.recovery_timeout(), Duration::from_secs(60));
  assert_eq!(config.half_open_max_calls(), 3);

  // Test creating a circuit breaker with custom configuration
  let custom_config = CircuitBreakerConfig::new()
    .with_failure_threshold(10)
    .with_recovery_timeout(Duration::from_secs(30))
    .with_half_open_max_calls(5);

  assert_eq!(custom_config.failure_threshold(), 10);
  assert_eq!(custom_config.recovery_timeout(), Duration::from_secs(30));
  assert_eq!(custom_config.half_open_max_calls(), 5);
}

#[ tokio::test ]
async fn test_circuit_breaker_initial_state()
{
  let config = CircuitBreakerConfig::default();
  let circuit_breaker = CircuitBreaker::new(config);

  // Circuit breaker should start in Closed state
  assert_eq!(circuit_breaker.state(), CircuitBreakerState::Closed);
  assert_eq!(circuit_breaker.failure_count(), 0);
  assert!(circuit_breaker.can_execute());
}

#[ tokio::test ]
async fn test_circuit_breaker_state_transitions_closed_to_open()
{
  let config = CircuitBreakerConfig::new()
    .with_failure_threshold(3); // Low threshold for testing
  let circuit_breaker = CircuitBreaker::new(config);

  // Initially closed
  assert_eq!(circuit_breaker.state(), CircuitBreakerState::Closed);
  assert!(circuit_breaker.can_execute());

  // Record failures below threshold - should stay closed
  circuit_breaker.record_failure();
  assert_eq!(circuit_breaker.state(), CircuitBreakerState::Closed);
  assert_eq!(circuit_breaker.failure_count(), 1);

  circuit_breaker.record_failure();
  assert_eq!(circuit_breaker.state(), CircuitBreakerState::Closed);
  assert_eq!(circuit_breaker.failure_count(), 2);

  // Third failure should trigger open state
  circuit_breaker.record_failure();
  assert_eq!(circuit_breaker.state(), CircuitBreakerState::Open);
  assert_eq!(circuit_breaker.failure_count(), 3);
  assert!(!circuit_breaker.can_execute()); // Should reject calls
}

#[ tokio::test ]
async fn test_circuit_breaker_state_transitions_open_to_half_open()
{
  let config = CircuitBreakerConfig::new()
    .with_failure_threshold(2)
    .with_recovery_timeout(Duration::from_millis(100)); // Short timeout for testing
  let circuit_breaker = CircuitBreaker::new(config);

  // Force circuit breaker to open state
  circuit_breaker.record_failure();
  circuit_breaker.record_failure();
  assert_eq!(circuit_breaker.state(), CircuitBreakerState::Open);
  assert!(!circuit_breaker.can_execute());

  // Wait for recovery timeout
  sleep(Duration::from_millis(150)).await;

  // Circuit breaker should transition to half-open and allow limited calls
  assert_eq!(circuit_breaker.state(), CircuitBreakerState::HalfOpen);
  assert!(circuit_breaker.can_execute());
}

#[ tokio::test ]
async fn test_circuit_breaker_state_transitions_half_open_to_closed()
{
  let config = CircuitBreakerConfig::new()
    .with_failure_threshold(2)
    .with_recovery_timeout(Duration::from_millis(50))
    .with_half_open_max_calls(2);
  let circuit_breaker = CircuitBreaker::new(config);

  // Force to open state and wait for half-open transition
  circuit_breaker.record_failure();
  circuit_breaker.record_failure();
  sleep(Duration::from_millis(60)).await;
  assert_eq!(circuit_breaker.state(), CircuitBreakerState::HalfOpen);

  // Record successful calls in half-open state
  circuit_breaker.record_success();
  circuit_breaker.record_success();

  // Should transition back to closed after successful calls
  assert_eq!(circuit_breaker.state(), CircuitBreakerState::Closed);
  assert_eq!(circuit_breaker.failure_count(), 0);
}

#[ tokio::test ]
async fn test_circuit_breaker_state_transitions_half_open_to_open()
{
  let config = CircuitBreakerConfig::new()
    .with_failure_threshold(2)
    .with_recovery_timeout(Duration::from_millis(50))
    .with_half_open_max_calls(2);
  let circuit_breaker = CircuitBreaker::new(config);

  // Force to open state and wait for half-open transition
  circuit_breaker.record_failure();
  circuit_breaker.record_failure();
  sleep(Duration::from_millis(60)).await;
  assert_eq!(circuit_breaker.state(), CircuitBreakerState::HalfOpen);

  // Record a failure in half-open state
  circuit_breaker.record_failure();

  // Should transition back to open immediately
  assert_eq!(circuit_breaker.state(), CircuitBreakerState::Open);
  assert!(!circuit_breaker.can_execute());
}

#[ tokio::test ]
async fn test_circuit_breaker_integration_with_ollama_client()
{
  let circuit_config = CircuitBreakerConfig::new()
    .with_failure_threshold(3)
    .with_recovery_timeout(Duration::from_millis(100));

  let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_circuit_breaker(circuit_config);

  assert!(client.has_circuit_breaker());
  assert_eq!(client.circuit_breaker_state(), CircuitBreakerState::Closed);

  // Create a request that will fail
  let request = ChatRequest {
    model : "test-model".to_string(),
    messages : vec![ChatMessage {
      role : MessageRole::User,
      content : "Hello".to_string(),
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    }],
    stream : Some(false),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  // Make failed requests - should eventually open circuit breaker
  for i in 1..=3
  {
    let result = client.chat(request.clone()).await;
    assert!(result.is_err());

    if i < 3
    {
      assert_eq!(client.circuit_breaker_state(), CircuitBreakerState::Closed);
    } else {
      assert_eq!(client.circuit_breaker_state(), CircuitBreakerState::Open);
    }
  }

  // Circuit breaker should now reject calls immediately
  let start = Instant::now();
  let result = client.chat(request.clone()).await;
  let duration = start.elapsed();

  assert!(result.is_err());
  assert!(duration < Duration::from_millis(10)); // Should fail fast
  let error_str = format!( "{}", result.unwrap_err() );
  assert!(error_str.contains("Circuit breaker is open"));
}

#[ tokio::test ]
async fn test_circuit_breaker_recovery_mechanism()
{
  let circuit_config = CircuitBreakerConfig::new()
    .with_failure_threshold(2)
    .with_recovery_timeout(Duration::from_millis(50))
    .with_half_open_max_calls(2);

  let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_circuit_breaker(circuit_config);

  // Force circuit breaker to open
  let request = GenerateRequest {
    model : "test-model".to_string(),
    prompt : "test".to_string(),
    stream : Some(false),
    options : None,
  };

  for _ in 0..2
  {
    let _ = client.generate(request.clone()).await;
  }
  assert_eq!(client.circuit_breaker_state(), CircuitBreakerState::Open);

  // Wait for recovery timeout
  sleep(Duration::from_millis(60)).await;

  // Circuit breaker should transition to half-open
  assert_eq!(client.circuit_breaker_state(), CircuitBreakerState::HalfOpen);

  // Should allow limited calls in half-open state
  let result = client.generate(request.clone()).await;
  assert!(result.is_err()); // Still fails due to unreachable server
}

#[ tokio::test ]
async fn test_circuit_breaker_performance_overhead()
{
  let circuit_config = CircuitBreakerConfig::default();
  let circuit_breaker = CircuitBreaker::new(circuit_config);

  // Measure overhead of circuit breaker state checking
  let iterations = 10000;
  let start = Instant::now();

  for _ in 0..iterations
  {
    let _ = circuit_breaker.can_execute();
  }

  let duration = start.elapsed();
  let overhead_per_check = duration / iterations;

  // Circuit breaker overhead should be minimal (< 1 microsecond per check)
  assert!(overhead_per_check < Duration::from_micros(1));

  println!( "Circuit breaker overhead : {overhead_per_check:?} per check" );
}

#[ tokio::test ]
async fn test_circuit_breaker_concurrent_access()
{
  let circuit_config = CircuitBreakerConfig::new().with_failure_threshold(10);
  let circuit_breaker = Arc::new(CircuitBreaker::new(circuit_config));

  let mut handles = vec![];

  // Spawn multiple tasks that record failures concurrently
  for _ in 0..5
  {
    let cb = circuit_breaker.clone();
    let handle = tokio::spawn(async move {
      for _ in 0..2
      {
        cb.record_failure();
        sleep(Duration::from_millis(1)).await;
      }
    });
    handles.push(handle);
  }

  // Wait for all tasks to complete
  for handle in handles
  {
    handle.await.unwrap();
  }

  // Should have recorded all failures correctly
  assert_eq!(circuit_breaker.failure_count(), 10);
  assert_eq!(circuit_breaker.state(), CircuitBreakerState::Open);
}

#[ tokio::test ]
async fn test_circuit_breaker_with_mixed_success_and_failure()
{
  let config = CircuitBreakerConfig::new().with_failure_threshold(5);
  let circuit_breaker = CircuitBreaker::new(config);

  // Mixed pattern of success and failure
  circuit_breaker.record_failure(); // 1
  circuit_breaker.record_success(); // Reset
  circuit_breaker.record_failure(); // 1
  circuit_breaker.record_failure(); // 2
  circuit_breaker.record_success(); // Reset
  circuit_breaker.record_failure(); // 1

  assert_eq!(circuit_breaker.state(), CircuitBreakerState::Closed);
  assert_eq!(circuit_breaker.failure_count(), 1);

  // Now record enough consecutive failures to trigger open state
  for _ in 0..4
  {
    circuit_breaker.record_failure();
  }

  assert_eq!(circuit_breaker.state(), CircuitBreakerState::Open);
}

#[ tokio::test ]
async fn test_circuit_breaker_debug_and_display()
{
  let config = CircuitBreakerConfig::default();
  let circuit_breaker = CircuitBreaker::new(config);

  // Test Debug implementation
  let debug_output = format!( "{circuit_breaker:?}" );
  assert!(debug_output.contains("CircuitBreaker"));
  assert!(debug_output.contains("Closed"));

  // Test Display implementation
  let display_output = format!( "{circuit_breaker}" );
  assert!(display_output.contains("Circuit breaker"));
  assert!(display_output.contains("state : Closed"));
}
