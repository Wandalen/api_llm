//! Caching feature configuration setters for ClientBuilder.

use core::time::Duration;
use super::ClientBuilder;

impl ClientBuilder
{
  /// Enables or disables request caching functionality.
  ///
  /// When enabled, the client will cache API responses based on request parameters
  /// to improve performance and reduce API calls.
  #[ must_use ]
  #[ inline ]
  pub fn enable_request_cache( mut self, enable : bool ) -> Self
  {
    self.enable_request_cache = enable;
    self
  }

  /// Sets the cache time-to-live (TTL) duration.
  ///
  /// Cached responses will expire after this duration and be removed
  /// from the cache. Must be greater than 0.
  ///
  /// # Arguments
  ///
  /// * `ttl` - Duration for cache entries to remain valid
  #[ must_use ]
  #[ inline ]
  pub fn cache_ttl( mut self, ttl : Duration ) -> Self
  {
    self.cache_ttl = ttl;
    self
  }

  /// Sets the maximum number of entries in the cache.
  ///
  /// When the cache reaches this size, least recently used (LRU) entries
  /// will be evicted to make room for new entries. Must be greater than 0.
  ///
  /// # Arguments
  ///
  /// * `max_size` - Maximum number of cache entries
  #[ must_use ]
  #[ inline ]
  pub fn cache_max_size( mut self, max_size : usize ) -> Self
  {
    self.cache_max_size = max_size;
    self
  }

  /// Enables or disables cache metrics collection.
  ///
  /// When enabled, the client will collect metrics about cache performance:
  /// - Hit/miss ratios
  /// - Cache size and memory usage
  /// - Eviction counts
  #[ must_use ]
  #[ inline ]
  pub fn enable_cache_metrics( mut self, enable_metrics : bool ) -> Self
  {
    self.enable_cache_metrics = enable_metrics;
    self
  }
}
