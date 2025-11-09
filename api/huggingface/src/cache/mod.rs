//! Content Caching Implementation
//!
//! Provides in-memory caching for API responses with TTL and size limits.
//!
//! ## Features
//!
//! - **TTL Support**: Automatic expiration of cached entries
//! - **Size Limits**: Maximum cache size with LRU eviction
//! - **Thread-Safe**: Safe for concurrent use
//! - **Statistics**: Track hits, misses, and cache size
//! - **Generic**: Works with any serializable types
//!
//! ## Usage
//!
//! ```no_run
//! # use api_huggingface::cache::{Cache, CacheConfig};
//! # use std::time::Duration;
//! # async fn example() -> Result< (), Box< dyn std::error::Error > > {
//! let cache = Cache::new(CacheConfig {
//!   max_entries : 1000,
//!   default_ttl : Some(Duration::from_secs(300)),
//! });
//!
//! // Cache a value
//! cache.insert("key".to_string(), "value".to_string(), None).await;
//!
//! // Retrieve from cache
//! if let Some(value) = cache.get(&"key".to_string()).await {
//!   println!("Cache hit : {}", value);
//! }
//! # Ok(())
//! # }
//! ```

pub mod implementation;

pub use implementation::{
  Cache,
  CacheConfig,
  CacheStats,
  CacheError,
};
