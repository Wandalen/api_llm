//! Integration tests for performance metrics

#![ allow( clippy::float_cmp ) ]
#![ allow( clippy::manual_range_contains ) ]
#![ allow( clippy::std_instead_of_core ) ]

use api_huggingface::performance::{ PerformanceMetrics, MetricsConfig };
use core::time::Duration;
use tokio::time::sleep;

#[ tokio::test ]
async fn test_basic_request_recording()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  metrics.record_request( Duration::from_millis( 100 ), true, 1024 ).await;
  metrics.record_request( Duration::from_millis( 200 ), true, 2048 ).await;

  let snapshot = metrics.snapshot().await;
  assert_eq!( snapshot.total_requests, 2 );
  assert_eq!( snapshot.successful_requests, 2 );
  assert_eq!( snapshot.failed_requests, 0 );
  assert_eq!( snapshot.total_bytes, 3072 );
}

#[ tokio::test ]
async fn test_error_tracking()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  metrics.record_request( Duration::from_millis( 100 ), true, 100 ).await;
  metrics.record_request( Duration::from_millis( 100 ), false, 0 ).await;
  metrics.record_request( Duration::from_millis( 100 ), false, 0 ).await;

  let snapshot = metrics.snapshot().await;
  assert_eq!( snapshot.total_requests, 3 );
  assert_eq!( snapshot.successful_requests, 1 );
  assert_eq!( snapshot.failed_requests, 2 );
  assert!( ( snapshot.error_rate() - 0.666 ).abs() < 0.01 );
  assert!( ( snapshot.success_rate() - 0.333 ).abs() < 0.01 );
}

#[ tokio::test ]
async fn test_latency_min_max()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  metrics.record_request( Duration::from_millis( 50 ), true, 0 ).await;
  metrics.record_request( Duration::from_millis( 300 ), true, 0 ).await;
  metrics.record_request( Duration::from_millis( 150 ), true, 0 ).await;

  let snapshot = metrics.snapshot().await;
  assert_eq!( snapshot.min_latency(), Duration::from_millis( 50 ) );
  assert_eq!( snapshot.max_latency(), Duration::from_millis( 300 ) );
}

#[ tokio::test ]
async fn test_latency_mean()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  metrics.record_request( Duration::from_millis( 100 ), true, 0 ).await;
  metrics.record_request( Duration::from_millis( 200 ), true, 0 ).await;
  metrics.record_request( Duration::from_millis( 300 ), true, 0 ).await;

  let snapshot = metrics.snapshot().await;
  assert_eq!( snapshot.mean_latency(), Duration::from_millis( 200 ) );
}

#[ tokio::test ]
async fn test_latency_median()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  metrics.record_request( Duration::from_millis( 100 ), true, 0 ).await;
  metrics.record_request( Duration::from_millis( 200 ), true, 0 ).await;
  metrics.record_request( Duration::from_millis( 300 ), true, 0 ).await;
  metrics.record_request( Duration::from_millis( 400 ), true, 0 ).await;
  metrics.record_request( Duration::from_millis( 500 ), true, 0 ).await;

  let snapshot = metrics.snapshot().await;
  assert_eq!( snapshot.median_latency(), Duration::from_millis( 300 ) );
}

#[ tokio::test ]
async fn test_percentile_p95()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  // Add 100 measurements from 1ms to 100ms
  for i in 1..=100
  {
  metrics.record_request( Duration::from_millis( i ), true, 0 ).await;
  }

  let snapshot = metrics.snapshot().await;

  // p95 should be around 95ms
  let p95_ms = snapshot.p95_latency().as_millis();
  assert!( p95_ms >= 90 && p95_ms <= 100 );
}

#[ tokio::test ]
async fn test_percentile_p99()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  // Add 100 measurements from 1ms to 100ms
  for i in 1..=100
  {
  metrics.record_request( Duration::from_millis( i ), true, 0 ).await;
  }

  let snapshot = metrics.snapshot().await;

  // p99 should be around 99ms
  let p99_ms = snapshot.p99_latency().as_millis();
  assert!( p99_ms >= 95 && p99_ms <= 100 );
}

#[ tokio::test ]
async fn test_requests_per_second()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  // Record 10 requests
  for _ in 0..10
  {
  metrics.record_request( Duration::from_millis( 10 ), true, 100 ).await;
  }

  // Wait a bit to ensure duration > 0
  sleep( Duration::from_millis( 20 ) ).await;

  let snapshot = metrics.snapshot().await;
  let rps = snapshot.requests_per_second();

  // Should have some positive RPS
  assert!( rps > 0.0 );
  // Should be less than 1000 RPS (since we only did 10 requests)
  assert!( rps < 1000.0 );
}

#[ tokio::test ]
async fn test_bytes_per_second()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  // Record requests with different byte counts
  for i in 1..=10
  {
  metrics.record_request( Duration::from_millis( 10 ), true, i * 1024 ).await;
  }

  // Wait to ensure duration > 0
  sleep( Duration::from_millis( 20 ) ).await;

  let snapshot = metrics.snapshot().await;
  let bps = snapshot.bytes_per_second();

  // Should have some positive bytes/sec
  assert!( bps > 0.0 );
  // Total bytes = 1024 + 2048 + ... + 10240 = 56320
  assert_eq!( snapshot.total_bytes, 56320 );
}

#[ tokio::test ]
async fn test_window_snapshot()
{
  let config = MetricsConfig
  {
  max_samples : 10000,
  window_duration : Duration::from_secs( 60 ),
  };
  let metrics = PerformanceMetrics::new( config );

  // Add a request
  metrics.record_request( Duration::from_millis( 100 ), true, 1024 ).await;

  // Wait 100ms
  sleep( Duration::from_millis( 100 ) ).await;

  // Add another request
  metrics.record_request( Duration::from_millis( 200 ), true, 2048 ).await;

  // Get snapshot of last 150ms (should include both)
  let snapshot = metrics.snapshot_window( Duration::from_millis( 150 ) ).await;
  assert_eq!( snapshot.total_requests, 2 );

  // Get snapshot of last 50ms (should include only second)
  let snapshot = metrics.snapshot_window( Duration::from_millis( 50 ) ).await;
  assert_eq!( snapshot.total_requests, 1 );
}

#[ tokio::test ]
async fn test_cleanup_old_measurements()
{
  let config = MetricsConfig
  {
  max_samples : 10,
  window_duration : Duration::from_millis( 50 ),
  };
  let metrics = PerformanceMetrics::new( config );

  // Add some measurements
  for _ in 0..5
  {
  metrics.record_request( Duration::from_millis( 10 ), true, 100 ).await;
  }

  assert_eq!( metrics.measurement_count().await, 5 );

  // Wait for measurements to become old
  sleep( Duration::from_millis( 100 ) ).await;

  // Cleanup should remove old measurements
  let removed = metrics.cleanup().await;
  assert_eq!( removed, 5 );
  assert_eq!( metrics.measurement_count().await, 0 );
}

#[ tokio::test ]
async fn test_max_samples_limit()
{
  let config = MetricsConfig
  {
  max_samples : 5,
  window_duration : Duration::from_secs( 60 ),
  };
  let metrics = PerformanceMetrics::new( config );

  // Add more than max_samples
  for _ in 0..10
  {
  metrics.record_request( Duration::from_millis( 10 ), true, 100 ).await;
  }

  // Trigger cleanup
  let _ = metrics.cleanup().await;

  // Should be limited to max_samples
  assert!( metrics.measurement_count().await <= 5 );
}

#[ tokio::test ]
async fn test_reset_metrics()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  // Add some measurements
  metrics.record_request( Duration::from_millis( 100 ), true, 1024 ).await;
  metrics.record_request( Duration::from_millis( 200 ), false, 2048 ).await;

  assert_eq!( metrics.measurement_count().await, 2 );

  // Reset
  metrics.reset().await;

  assert_eq!( metrics.measurement_count().await, 0 );

  let snapshot = metrics.snapshot().await;
  assert_eq!( snapshot.total_requests, 0 );
  assert_eq!( snapshot.total_bytes, 0 );
}

#[ tokio::test ]
async fn test_empty_metrics()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  let snapshot = metrics.snapshot().await;
  assert_eq!( snapshot.total_requests, 0 );
  assert_eq!( snapshot.successful_requests, 0 );
  assert_eq!( snapshot.failed_requests, 0 );
  assert_eq!( snapshot.total_bytes, 0 );
  assert_eq!( snapshot.error_rate(), 0.0 );
  assert_eq!( snapshot.success_rate(), 0.0 );
  assert_eq!( snapshot.requests_per_second(), 0.0 );
  assert_eq!( snapshot.bytes_per_second(), 0.0 );
}

#[ tokio::test ]
async fn test_single_request()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  metrics.record_request( Duration::from_millis( 100 ), true, 1024 ).await;

  let snapshot = metrics.snapshot().await;
  assert_eq!( snapshot.total_requests, 1 );
  assert_eq!( snapshot.successful_requests, 1 );
  assert_eq!( snapshot.error_rate(), 0.0 );
  assert_eq!( snapshot.success_rate(), 1.0 );
  assert_eq!( snapshot.min_latency(), Duration::from_millis( 100 ) );
  assert_eq!( snapshot.max_latency(), Duration::from_millis( 100 ) );
  assert_eq!( snapshot.mean_latency(), Duration::from_millis( 100 ) );
}

#[ tokio::test ]
async fn test_concurrent_recording()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  // Spawn multiple tasks recording concurrently
  let mut handles = vec![];

  for i in 0..10
  {
  let m = metrics.clone();
  let handle = tokio::spawn( async move
  {
      for _ in 0..10
      {
  m.record_request( Duration::from_millis( i * 10 ), true, 100 ).await;
      }
  } );
  handles.push( handle );
  }

  // Wait for all tasks
  for handle in handles
  {
  handle.await.expect( "[test_concurrent_request_tracking] Tokio task should complete successfully - check tokio::spawn() and concurrent access patterns" );
  }

  let snapshot = metrics.snapshot().await;
  assert_eq!( snapshot.total_requests, 100 );
  assert_eq!( snapshot.successful_requests, 100 );
}

#[ tokio::test ]
async fn test_large_dataset()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  // Record 1000 requests
  for i in 1..=1000
  {
  let latency = Duration::from_millis( i % 500 );
  let success = i % 10 != 0; // 10% failure rate
  metrics.record_request( latency, success, 100 ).await;
  }

  let snapshot = metrics.snapshot().await;
  assert_eq!( snapshot.total_requests, 1000 );
  assert_eq!( snapshot.successful_requests, 900 );
  assert_eq!( snapshot.failed_requests, 100 );
  assert!( ( snapshot.error_rate() - 0.1 ).abs() < 0.01 );
}

#[ tokio::test ]
async fn test_extreme_latencies()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  // Very fast request
  metrics.record_request( Duration::from_micros( 1 ), true, 10 ).await;

  // Very slow request
  metrics.record_request( Duration::from_secs( 10 ), true, 10000 ).await;

  let snapshot = metrics.snapshot().await;
  assert_eq!( snapshot.min_latency(), Duration::from_micros( 1 ) );
  assert_eq!( snapshot.max_latency(), Duration::from_secs( 10 ) );
}

#[ tokio::test ]
async fn test_zero_duration_edge_case()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  metrics.record_request( Duration::ZERO, true, 100 ).await;

  let snapshot = metrics.snapshot().await;
  assert_eq!( snapshot.min_latency(), Duration::ZERO );
  assert_eq!( snapshot.mean_latency(), Duration::ZERO );
}

#[ tokio::test ]
async fn test_all_failed_requests()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  for _ in 0..10
  {
  metrics.record_request( Duration::from_millis( 100 ), false, 0 ).await;
  }

  let snapshot = metrics.snapshot().await;
  assert_eq!( snapshot.successful_requests, 0 );
  assert_eq!( snapshot.failed_requests, 10 );
  assert_eq!( snapshot.error_rate(), 1.0 );
  assert_eq!( snapshot.success_rate(), 0.0 );
}

#[ tokio::test ]
async fn test_config_retrieval()
{
  let config = MetricsConfig
  {
  max_samples : 1234,
  window_duration : Duration::from_secs( 42 ),
  };
  let metrics = PerformanceMetrics::new( config );

  let retrieved = metrics.config().await;
  assert_eq!( retrieved.max_samples, 1234 );
  assert_eq!( retrieved.window_duration, Duration::from_secs( 42 ) );
}

#[ tokio::test ]
async fn test_snapshot_timing()
{
  let metrics = PerformanceMetrics::new( MetricsConfig::default() );

  metrics.record_request( Duration::from_millis( 100 ), true, 100 ).await;

  let snapshot1 = metrics.snapshot().await;
  sleep( Duration::from_millis( 50 ) ).await;
  let snapshot2 = metrics.snapshot().await;

  // Duration should increase between snapshots
  assert!( snapshot2.duration >= snapshot1.duration );
}
