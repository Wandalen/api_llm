//! Configuration Management
//!
//! This module provides configuration management for the `HuggingFace` API client,
//! with support for dynamic runtime configuration updates.
//!
//! ## Features
//!
//! - **Dynamic Updates**: Change configuration at runtime without restarting
//! - **Config Watchers**: Get notified when configuration changes
//! - **Config History**: Track configuration changes over time
//! - **Rollback Support**: Revert to previous configurations
//! - **Thread-Safe**: Safe for concurrent use
//!
//! ## Usage
//!
//! ```no_run
//! # use api_huggingface::config::{DynamicConfig, ReliabilityConfig};
//! # use api_huggingface::reliability::CircuitBreakerConfig;
//! # use std::{time::Duration, sync::Arc};
//! # async fn example() -> Result< (), Box< dyn std::error::Error > > {
//! let config = ReliabilityConfig {
//!   circuit_breaker : Some(CircuitBreakerConfig {
//!     failure_threshold : 5,
//!     success_threshold : 2,
//!     timeout : Duration::from_secs(60),
//!   }),
//!   rate_limiter : None,
//!   failover : None,
//!   health_check : None,
//!   timestamp : std::time::Instant::now(),
//! };
//!
//! let dynamic_config = DynamicConfig::new(config);
//!
//! // Register a watcher
//! dynamic_config.add_watcher(|_old, _new| {
//!   println!("Config changed!");
//! }).await;
//!
//! // Update configuration
//! let new_config = ReliabilityConfig::default();
//! dynamic_config.update(new_config).await?;
//!
//! // Rollback if needed
//! dynamic_config.rollback().await?;
//! # Ok(())
//! # }
//! ```

pub mod dynamic;

pub use dynamic::{
  DynamicConfig,
  ReliabilityConfig,
  ConfigWatcherCallback,
  ConfigError,
};
