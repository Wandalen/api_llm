//! Enterprise Reliability Features
//!
//! This module provides production-ready reliability mechanisms:
//!
//! - **Circuit Breaker**: Automatic service degradation protection
//! - **Rate Limiting**: Token bucket rate limiting with multiple time windows
//! - **Failover**: Multi-endpoint failover strategies
//! - **Health Checks**: Automated endpoint health monitoring
//!
//! ## Circuit Breaker
//!
//! The circuit breaker prevents cascading failures by automatically
//! detecting failing services and rejecting requests until the service
//! recovers.
//!
//! ```no_run
//! # use api_huggingface::reliability::{CircuitBreaker, CircuitBreakerConfig};
//! # use std::time::Duration;
//! # async fn example() -> Result< (), Box< dyn std::error::Error > > {
//! let circuit_breaker = CircuitBreaker::new(
//!   CircuitBreakerConfig {
//!     failure_threshold : 5,
//!     success_threshold : 2,
//!     timeout : Duration::from_secs(60),
//!   }
//! );
//!
//! let _result = circuit_breaker.execute(async {
//!   Ok::< String, Box< dyn std::error::Error > >("response".to_string())
//! }).await;
//! # Ok(())
//! # }
//! ```
//!
//! ## Rate Limiting
//!
//! The rate limiter controls request rates using token bucket algorithm
//! with support for multiple time windows.
//!
//! ```no_run
//! # use api_huggingface::reliability::{RateLimiter, RateLimiterConfig};
//! # async fn example() -> Result< (), Box< dyn std::error::Error > > {
//! let rate_limiter = RateLimiter::new(
//!   RateLimiterConfig {
//!     requests_per_second : Some(10),
//!     requests_per_minute : Some(500),
//!     requests_per_hour : Some(10000),
//!   }
//! );
//!
//! // Acquire permission before request
//! rate_limiter.acquire().await?;
//! // ... make your request ...
//! # Ok(())
//! # }
//! ```
//!
//! ## Failover
//!
//! The failover manager provides automatic failover to backup endpoints
//! with multiple strategies and health tracking.
//!
//! ```no_run
//! # use api_huggingface::reliability::{FailoverManager, FailoverConfig, FailoverStrategy};
//! # use std::time::Duration;
//! # async fn example() -> Result< (), Box< dyn std::error::Error > > {
//! let failover = FailoverManager::new(
//!   FailoverConfig {
//!     endpoints : vec![
//!       "https://api-inference.huggingface.co".to_string(),
//!       "https://backup.huggingface.co".to_string(),
//!     ],
//!     strategy : FailoverStrategy::Priority,
//!     max_retries : 3,
//!     failure_window : Duration::from_secs(300),
//!     failure_threshold : 5,
//!   }
//! ).map_err(|e| format!("{:?}", e))?;
//!
//! // Execute with automatic failover
//! let _result = failover.execute_with_failover(|_endpoint| {
//!   Box::pin(async move {
//!     Ok::< String, Box< dyn std::error::Error > >("response".to_string())
//!   })
//! }).await;
//! # Ok(())
//! # }
//! ```
//!
//! ## Health Checks
//!
//! The health checker provides automated monitoring of endpoint health
//! with configurable strategies and background monitoring.
//!
//! ```no_run
//! # use api_huggingface::reliability::{HealthChecker, HealthCheckConfig, HealthCheckStrategy};
//! # use std::time::Duration;
//! # async fn example() -> Result< (), Box< dyn std::error::Error > > {
//! let health_checker = HealthChecker::new(
//!   HealthCheckConfig {
//!     endpoint : "https://api-inference.huggingface.co".to_string(),
//!     strategy : HealthCheckStrategy::LightweightApi,
//!     check_interval : Duration::from_secs(30),
//!     timeout : Duration::from_secs(5),
//!     unhealthy_threshold : 3,
//!   }
//! );
//!
//! // Start background monitoring
//! let _monitor = health_checker.start_monitoring().await;
//!
//! // Check current health
//! let status = health_checker.get_status().await;
//! println!("Healthy : {}, Latency : {}ms", status.healthy, status.latency_ms);
//! # Ok(())
//! # }
//! ```

pub mod circuit_breaker;
pub mod failover;
pub mod health_check;
pub mod rate_limiter;

pub use circuit_breaker::{
  CircuitBreaker,
  CircuitBreakerConfig,
  CircuitBreakerError,
  CircuitState,
};

pub use rate_limiter::{
  RateLimiter,
  RateLimiterConfig,
  RateLimitError,
  AvailableTokens,
};

pub use failover::{
  FailoverManager,
  FailoverConfig,
  FailoverStrategy,
  FailoverError,
  EndpointHealthStatus,
};

pub use health_check::{
  HealthChecker,
  HealthCheckConfig,
  HealthCheckStrategy,
  HealthCheckError,
  HealthStatus,
  MonitorHandle,
};
