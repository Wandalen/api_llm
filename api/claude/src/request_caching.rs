//! Request caching implementation for Anthropic Claude API
//!
//! This module provides caching functionality to store and retrieve API responses,
//! reducing redundant API calls and improving performance.

#[ cfg( feature = "request-caching" ) ]
mod private
{
  use crate::{
    CreateMessageRequest, CreateMessageResponse,
  };
  use std::{ collections::HashMap, time::{ Instant, Duration }, sync::{ Arc, Mutex } };
  use core::hash::{ Hash, Hasher };

  /// Configuration for request caching
  #[ derive( Debug, Clone ) ]
  pub struct CacheConfig
  {
    /// Time-to-live for cached entries in seconds
    ttl_seconds : u64,
    /// Maximum number of entries in cache
    max_entries : usize,
    /// Memory limit in megabytes
    memory_limit_mb : usize,
    /// Enable/disable caching
    enabled : bool,
  }

  impl Default for CacheConfig
  {
    fn default() -> Self
    {
      Self {
        ttl_seconds : 300, // 5 minutes
        max_entries : 1000,
        memory_limit_mb : 100, // 100 MB
        enabled : true,
      }
    }
  }

  impl CacheConfig
  {
    /// Create a new cache configuration
    pub fn new() -> Self
    {
      Self::default()
    }

    /// Set TTL in seconds
    #[ must_use ]
    pub fn with_ttl_seconds( mut self, ttl_seconds : u64 ) -> Self
    {
      self.ttl_seconds = ttl_seconds;
      self
    }

    /// Set maximum number of entries
    #[ must_use ]
    pub fn with_max_entries( mut self, max_entries : usize ) -> Self
    {
      self.max_entries = max_entries;
      self
    }

    /// Set memory limit in megabytes
    #[ must_use ]
    pub fn with_memory_limit_mb( mut self, memory_limit_mb : usize ) -> Self
    {
      self.memory_limit_mb = memory_limit_mb;
      self
    }

    /// Enable or disable caching
    #[ must_use ]
    pub fn with_enabled( mut self, enabled : bool ) -> Self
    {
      self.enabled = enabled;
      self
    }

    /// Validate configuration
    pub fn is_valid( &self ) -> bool
    {
      self.ttl_seconds > 0 &&
      self.max_entries > 0 &&
      self.memory_limit_mb > 0
    }

    /// Get TTL in seconds
    pub fn ttl_seconds( &self ) -> u64
    {
      self.ttl_seconds
    }

    /// Get maximum number of entries
    pub fn max_entries( &self ) -> usize
    {
      self.max_entries
    }

    /// Get memory limit in megabytes
    pub fn memory_limit_mb( &self ) -> usize
    {
      self.memory_limit_mb
    }

    /// Check if caching is enabled
    pub fn is_enabled( &self ) -> bool
    {
      self.enabled
    }
  }

  /// Cache entry with expiration
  #[ derive( Debug, Clone ) ]
  struct CacheEntry
  {
    response : CreateMessageResponse,
    created_at : Instant,
    last_accessed : Instant,
  }

  impl CacheEntry
  {
    fn new( response : CreateMessageResponse ) -> Self
    {
      let now = Instant::now();
      Self {
        response,
        created_at : now,
        last_accessed : now,
      }
    }

    fn is_expired( &self, ttl : Duration ) -> bool
    {
      self.created_at.elapsed() > ttl
    }

    fn touch( &mut self )
    {
      self.last_accessed = Instant::now();
    }
  }

  /// Cache metrics for monitoring
  #[ derive( Debug, Clone, Default ) ]
  pub struct CacheMetrics
  {
    hits : u64,
    misses : u64,
    stores : u64,
    evictions : u64,
  }

  impl CacheMetrics
  {
    /// Get number of cache hits
    pub fn hits( &self ) -> u64
    {
      self.hits
    }

    /// Get number of cache misses
    pub fn misses( &self ) -> u64
    {
      self.misses
    }

    /// Get number of stores
    pub fn stores( &self ) -> u64
    {
      self.stores
    }

    /// Get number of evictions
    pub fn evictions( &self ) -> u64
    {
      self.evictions
    }

    /// Calculate hit rate
    pub fn hit_rate( &self ) -> f64
    {
      let total = self.hits + self.misses;
      if total == 0
      {
        0.0
      } else {
        self.hits as f64 / total as f64
      }
    }
  }

  /// Request cache implementation
  #[ derive( Debug ) ]
  pub struct RequestCache
  {
    config : CacheConfig,
    storage : Arc< Mutex< HashMap< String, CacheEntry > > >,
    metrics : Arc< Mutex< CacheMetrics > >,
  }

  impl RequestCache
  {
    /// Create a new request cache
    pub fn new( config : CacheConfig ) -> Self
    {
      Self {
        config,
        storage : Arc::new( Mutex::new( HashMap::new() ) ),
        metrics : Arc::new( Mutex::new( CacheMetrics::default() ) ),
      }
    }

    /// Generate cache key from request
    pub fn generate_cache_key( &self, request : &CreateMessageRequest ) -> String
    {
      use std::collections::hash_map::DefaultHasher;

      let mut hasher = DefaultHasher::new();

      // Hash key components for deterministic cache keys
      request.model.hash( &mut hasher );
      request.max_tokens.hash( &mut hasher );

      // Hash messages content
      for message in &request.messages
      {
        format!( "{message:?}" ).hash( &mut hasher );
      }

      // Hash optional fields if present
      if let Some( ref system ) = request.system
      {
        system.hash( &mut hasher );
      }

      if let Some( temp ) = request.temperature
      {
        temp.to_bits().hash( &mut hasher );
      }

      #[ cfg( feature = "tools" ) ]
      {
        if let Some( ref tools ) = request.tools
        {
          format!( "{tools:?}" ).hash( &mut hasher );
        }
        if let Some( ref tool_choice ) = request.tool_choice
        {
          format!( "{tool_choice:?}" ).hash( &mut hasher );
        }
      }

      format!( "{:x}", hasher.finish() )
    }

    /// Store response in cache
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned
    pub fn store( &self, request : &CreateMessageRequest, response : CreateMessageResponse )
    {
      if !self.config.is_enabled()
      {
        return;
      }

      let key = self.generate_cache_key( request );
      let entry = CacheEntry::new( response );

      {
        let mut storage = self.storage.lock().unwrap();

        // Evict oldest entries if at capacity
        if storage.len() >= self.config.max_entries
        {
          self.evict_lru( &mut storage );
        }

        storage.insert( key, entry );
      }

      // Update metrics
      {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.stores += 1;
      }
    }

    /// Get response from cache
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned
    pub fn get( &self, request : &CreateMessageRequest ) -> Option< CreateMessageResponse >
    {
      if !self.config.is_enabled()
      {
        return None;
      }

      let key = self.generate_cache_key( request );
      let ttl = Duration::from_secs( self.config.ttl_seconds );

      let result = {
        let mut storage = self.storage.lock().unwrap();

        if let Some( entry ) = storage.get_mut( &key )
        {
          if entry.is_expired( ttl )
          {
            storage.remove( &key );
            None
          } else {
            entry.touch();
            Some( entry.response.clone() )
          }
        } else {
          None
        }
      };

      // Update metrics
      {
        let mut metrics = self.metrics.lock().unwrap();
        if result.is_some()
        {
          metrics.hits += 1;
        } else {
          metrics.misses += 1;
        }
      }

      result
    }

    /// Invalidate specific cache entry
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned
    pub fn invalidate( &self, request : &CreateMessageRequest )
    {
      let key = self.generate_cache_key( request );
      let mut storage = self.storage.lock().unwrap();
      storage.remove( &key );
    }

    /// Clear all cache entries
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned
    pub fn clear( &self )
    {
      let mut storage = self.storage.lock().unwrap();
      storage.clear();
    }

    /// Get current cache size
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned
    pub fn size( &self ) -> usize
    {
      let storage = self.storage.lock().unwrap();
      storage.len()
    }

    /// Get cache configuration
    pub fn config( &self ) -> &CacheConfig
    {
      &self.config
    }

    /// Get cache metrics
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned
    pub fn metrics( &self ) -> CacheMetrics
    {
      let metrics = self.metrics.lock().unwrap();
      metrics.clone()
    }

    /// Evict least recently used entry
    fn evict_lru( &self, storage : &mut HashMap< String, CacheEntry > )
    {
      if storage.is_empty()
      {
        return;
      }

      // Find LRU entry
      let mut oldest_key = None;
      let mut oldest_time = Instant::now();

      for ( key, entry ) in storage.iter()
      {
        if entry.last_accessed < oldest_time
        {
          oldest_time = entry.last_accessed;
          oldest_key = Some( key.clone() );
        }
      }

      // Remove LRU entry
      if let Some( key ) = oldest_key
      {
        storage.remove( &key );

        // Update eviction metrics
        let mut metrics = self.metrics.lock().unwrap();
        metrics.evictions += 1;
      }
    }
  }

  impl Clone for RequestCache
  {
    fn clone( &self ) -> Self
    {
      Self {
        config : self.config.clone(),
        storage : Arc::clone( &self.storage ),
        metrics : Arc::clone( &self.metrics ),
      }
    }
  }
}

#[ cfg( feature = "request-caching" ) ]
crate::mod_interface!
{
  exposed use
  {
    CacheConfig,
    RequestCache,
    CacheMetrics,
  };
}

#[ cfg( not( feature = "request-caching" ) ) ]
crate::mod_interface!
{
  // Empty when request-caching feature is disabled
}