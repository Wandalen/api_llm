//! Performance Metrics Implementation
//!
//! Provides comprehensive performance tracking for API requests.
//!
//! ## Features
//!
//! - **Latency Tracking**: Min, max, mean, percentiles (p50, p95, p99)
//! - **Throughput Metrics**: Requests per second, bytes transferred
//! - **Error Tracking**: Success/failure counts, error rate
//! - **Time Windows**: Last minute, last hour, all-time stats
//! - **Thread-Safe**: Safe for concurrent use
//!
//! ## Usage
//!
//! ```no_run
//! # use api_huggingface::performance::{PerformanceMetrics, MetricsConfig};
//! # use std::time::Instant;
//! # async fn example() -> Result< (), Box< dyn std::error::Error > > {
//! let metrics = PerformanceMetrics::new(MetricsConfig::default());
//!
//! // Record a request
//! let start = Instant::now();
//! // ... make request ...
//! metrics.record_request(start.elapsed(), true, 1024).await;
//!
//! // Get statistics
//! let snapshot = metrics.snapshot().await;
//! println!("Mean latency : {:?}", snapshot.latency.mean);
//! println!("P95 latency : {:?}", snapshot.latency.p95);
//! println!("Error rate : {:.2}%", snapshot.error_rate() * 100.0);
//! # Ok(())
//! # }
//! ```

pub mod metrics;

pub use metrics::{
  PerformanceMetrics,
  MetricsConfig,
  MetricsSnapshot,
  LatencyStats,
};
