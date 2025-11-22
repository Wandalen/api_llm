//! Performance Monitoring Tests
//!
//! Comprehensive test suite for performance monitoring and request overhead limits
//! in the OpenAI API client, including:
//! - Request overhead measurement (<10ms target)
//! - Concurrent request performance validation
//! - Memory usage monitoring during operations
//! - Performance regression detection
//! - Throughput measurement under load

#![ allow( clippy::unreadable_literal ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::useless_vec ) ]
#![ allow( clippy::unused_async ) ]
#![ allow( clippy::must_use_candidate ) ]
#![ allow( clippy::missing_panics_doc ) ]
#![ allow( clippy::missing_errors_doc ) ]
#![ allow( clippy::doc_markdown ) ]

#[ cfg( test ) ]
mod performance_monitoring_tests
{
  use api_openai::
  {
    Client,
    environment ::OpenaiEnvironmentImpl,
    secret ::Secret,
    performance_monitoring ::*,
  };
  use std::time::Duration;

  // Helper function to create test client
  fn create_test_client() -> Result< Client< OpenaiEnvironmentImpl >, Box< dyn std::error::Error > >
  {
    let secret = Secret::new_unchecked("sk-test_key_12345".to_string());
    let env = OpenaiEnvironmentImpl::build(secret, None, None, api_openai::environment::OpenAIRecommended::base_url().to_string(), api_openai::environment::OpenAIRecommended::realtime_base_url().to_string())
      .expect("Failed to create environment");
    Ok(Client::build(env)?)
  }

  // ===== REQUEST OVERHEAD MEASUREMENT TESTS =====

  #[ tokio::test ]
  async fn test_request_overhead_measurement_succeeds()
  {
    // SUCCESS TEST: Request overhead measurement should succeed with implementation
    // Target : <10ms overhead per request

    let _client = create_test_client().expect("Failed to create client");

    // This should succeed as request overhead measurement is now implemented
    let result = measure_request_overhead().await;
    assert!(result.is_ok(), "Request overhead measurement should succeed with implementation");

    let overhead = result.unwrap();
    assert!(overhead.as_millis() < 10, "Request overhead should be less than 10ms");
  }
  #[ tokio::test ]
  async fn test_multiple_request_overhead_consistency_succeeds()
  {
    // SUCCESS TEST: Multiple request overhead measurements should be consistent

    let _client = create_test_client().expect("Failed to create client");

    // Configure performance monitoring with generous threshold for test environment
    configure_performance_monitoring( PerformanceConfig
    {
      max_request_overhead_ms : 50, // More generous threshold for test environment
      enable_memory_monitoring : true,
      enable_regression_detection : false, // Disable for cleaner test runs
      baseline_performance : None,
      regression_threshold_percent : 20.0,
    });

    // This should succeed as overhead measurement is now implemented
    let result = measure_overhead_consistency(10).await;
    assert!(result.is_ok(), "Overhead consistency measurement should succeed with implementation");

    let measurements = result.unwrap();
    assert_eq!(measurements.len(), 10);
    // All measurements should be under reasonable threshold (simulated overhead is 0.5ms)
    for measurement in measurements
    {
      assert!( measurement.as_millis() < 10, "Each overhead measurement should be less than 10ms" );
    }
  }

  // ===== CONCURRENT REQUEST PERFORMANCE TESTS =====

  #[ tokio::test ]
  async fn test_concurrent_request_performance_succeeds()
  {
    // SUCCESS TEST: Concurrent request performance validation should succeed with implementation

    let _client = create_test_client().expect("Failed to create client");

    // This should succeed as concurrent performance measurement is now implemented
    let result = measure_concurrent_performance(20).await;
    assert!(result.is_ok(), "Concurrent performance measurement should succeed with implementation");

    let results = result.unwrap();
    assert_eq!(results.len(), 20);
    // Each request should complete reasonably quickly
    for duration in results
    {
      assert!(duration.as_millis() < 100, "Each concurrent request should complete in under 100ms");
    }
  }

  // ===== MEMORY USAGE MONITORING TESTS =====

  #[ tokio::test ]
  async fn test_memory_usage_monitoring_succeeds()
  {
    // SUCCESS TEST: Memory usage monitoring during operations should succeed with implementation

    let _client = create_test_client().expect("Failed to create client");

    // This should succeed as memory monitoring is now implemented
    let result = monitor_memory_usage().await;
    assert!(result.is_ok(), "Memory usage monitoring should succeed with implementation");

    let report = result.unwrap();
    assert!(report.initial_usage > 0, "Initial memory usage should be positive");
    assert!(report.peak_usage >= report.initial_usage, "Peak usage should be at least initial usage");
    assert!(report.final_usage > 0, "Final memory usage should be positive");
  }

  // ===== PERFORMANCE REGRESSION DETECTION TESTS =====

  #[ tokio::test ]
  async fn test_performance_regression_detection_succeeds()
  {
    // SUCCESS TEST: Performance regression detection should succeed with implementation

    let _client = create_test_client().expect("Failed to create client");

    // Configure baseline performance for regression detection
    let config = PerformanceConfig {
      enable_regression_detection : true,
      baseline_performance : Some(Duration::from_millis(5)),
      regression_threshold_percent : 50.0,
      ..Default::default()
    };
    configure_performance_monitoring(config);

    // This should succeed as regression detection is now implemented
    let result = detect_performance_regression().await;
    assert!(result.is_ok(), "Performance regression detection should succeed with implementation");

    let report = result.unwrap();
    assert_eq!(report.baseline_performance, Duration::from_millis(5));
    assert!(report.current_performance.as_millis() > 0, "Current performance should be measured");
  }

  // ===== THROUGHPUT MEASUREMENT TESTS =====

  #[ tokio::test ]
  async fn test_throughput_measurement_under_load_succeeds()
  {
    // SUCCESS TEST: Throughput measurement under load should succeed with implementation

    let _client = create_test_client().expect("Failed to create client");

    // Use smaller values for testing to avoid long test times
    let result = measure_throughput_under_load(10, Duration::from_secs(1)).await;
    assert!(result.is_ok(), "Throughput measurement should succeed with implementation");

    let metrics = result.unwrap();
    assert!(metrics.requests_per_second > 0.0, "Should measure positive requests per second");
    assert!(metrics.successful_requests > 0, "Should have some successful requests");
    assert!(metrics.average_latency.as_millis() > 0, "Should measure positive average latency");
  }

}