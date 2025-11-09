// src/request_cache.rs
//! Request caching functionality for `OpenAI` API client.
//!
//! This module provides a comprehensive caching layer that stores API responses
//! with TTL management, LRU eviction, and thread-safe concurrent access.

#![ allow( clippy::missing_inline_in_public_items, clippy::unused_async ) ]

/// Define a private namespace for all its items.
mod private
{
  use std::
  {
    collections ::HashMap,
    sync ::{ Arc, Mutex },
    time ::Instant,
  };
  use core::
  {
    hash ::{ Hash, Hasher },
    time ::Duration,
  };
  use tokio::sync::RwLock;
  use core::sync::atomic::{ AtomicU32, AtomicU64, Ordering };
  use serde::{ Serialize, Deserialize };
  use std::collections::hash_map::DefaultHasher;

  /// Configuration for request caching behavior.
  #[ derive( Debug, Clone ) ]
  pub struct CacheConfig
  {
    /// Maximum number of entries to store in cache.
    pub max_size : usize,
    /// Default time-to-live for cache entries.
    pub default_ttl : Duration,
    /// Whether to enable automatic cleanup of expired entries.
    pub enable_cleanup : bool,
    /// Interval for automatic cleanup operations.
    pub cleanup_interval : Duration,
  }

  impl Default for CacheConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        max_size : 1000,
        default_ttl : Duration::from_secs( 300 ), // 5 minutes
        enable_cleanup : true,
        cleanup_interval : Duration::from_secs( 60 ), // 1 minute
      }
    }
  }

  /// Statistics for cache performance monitoring.
  #[ derive( Debug, Clone ) ]
  pub struct CacheStatistics
  {
    /// Total number of cache hits.
    pub hits : Arc< AtomicU64 >,
    /// Total number of cache misses.
    pub misses : Arc< AtomicU64 >,
    /// Total number of cache evictions.
    pub evictions : Arc< AtomicU64 >,
    /// Current number of entries in cache.
    pub entries : Arc< AtomicU32 >,
  }

  impl Default for CacheStatistics
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        hits : Arc::new( AtomicU64::new( 0 ) ),
        misses : Arc::new( AtomicU64::new( 0 ) ),
        evictions : Arc::new( AtomicU64::new( 0 ) ),
        entries : Arc::new( AtomicU32::new( 0 ) ),
      }
    }
  }

  impl CacheStatistics
  {
    /// Get hit rate as a percentage.
    #[ inline ]
    #[ must_use ]
    pub fn hit_rate( &self ) -> f64
    {
      let hits = self.hits.load( Ordering::Relaxed );
      let misses = self.misses.load( Ordering::Relaxed );
      let total = hits + misses;

      if total == 0
      {
        0.0
      }
      else
      {
        ( hits as f64 / total as f64 ) * 100.0
      }
    }
  }

  /// A cache entry containing the cached value with metadata.
  #[ derive( Debug, Clone ) ]
  pub struct CacheEntry< T >
  {
    /// The cached value.
    pub value : T,
    /// When this entry was created.
    pub timestamp : Instant,
    /// Time-to-live for this entry.
    pub ttl : Duration,
    /// Number of times this entry has been accessed.
    pub access_count : Arc< AtomicU32 >,
    /// When this entry was last accessed.
    pub last_accessed : Arc< Mutex< Instant > >,
  }

  impl< T > CacheEntry< T >
  {
    /// Create a new cache entry.
    #[ inline ]
    pub fn new( value : T, ttl : Duration ) -> Self
    {
      let now = Instant::now();
      Self
      {
        value,
        timestamp : now,
        ttl,
        access_count : Arc::new( AtomicU32::new( 0 ) ),
        last_accessed : Arc::new( Mutex::new( now ) ),
      }
    }

    /// Check if this entry has expired.
    #[ inline ]
    pub async fn is_expired( &self ) -> bool
    {
      self.timestamp.elapsed() > self.ttl
    }

    /// Update the access time and increment access count.
    #[ inline ]
    pub async fn touch( &self )
    {
      self.access_count.fetch_add( 1, Ordering::Relaxed );
      if let Ok( mut last_accessed ) = self.last_accessed.lock()
      {
        *last_accessed = Instant::now();
      }
    }

    /// Get the age of this entry.
    #[ inline ]
    pub fn age( &self ) -> Duration
    {
      self.timestamp.elapsed()
    }
  }

  /// Key used for caching requests based on endpoint, method, and content.
  #[ derive( Debug, Clone, PartialEq, Eq, Hash ) ]
  pub struct RequestCacheKey
  {
    /// API endpoint path.
    pub endpoint : String,
    /// HTTP method.
    pub method : String,
    /// Hash of request body.
    pub body_hash : u64,
    /// Hash of relevant headers.
    pub headers_hash : u64,
  }

  impl RequestCacheKey
  {
    /// Create a new cache key from request components.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization of the request body or headers fails.
    #[ inline ]
    pub fn new< T: Serialize >(
      endpoint : &str,
      method : &str,
      body : Option< &T >,
      headers : &HashMap<  String, String  >
    ) -> crate::error::Result< Self >
    {
      let body_hash = if let Some( body ) = body
      {
        let json = serde_json::to_string( body ).map_err( |e|
          crate ::error::OpenAIError::Internal( format!( "Failed to serialize body for cache key : {e}" ) )
        )?;
        Self::hash_string( &json )
      }
      else
      {
        0
      };

      // Only include relevant headers for caching
      let relevant_headers : HashMap<  String, String  > = headers
        .iter()
        .filter( |( key, _ )| Self::is_relevant_header( key ) )
        .map( |( k, v )| ( k.clone(), v.clone() ) )
        .collect();

      let headers_json = serde_json::to_string( &relevant_headers ).map_err( |e|
        crate ::error::OpenAIError::Internal( format!( "Failed to serialize headers for cache key : {e}" ) )
      )?;

      Ok( Self
      {
        endpoint : endpoint.to_string(),
        method : method.to_string(),
        body_hash,
        headers_hash : Self::hash_string( &headers_json ),
      })
    }

    /// Determine if a header is relevant for caching.
    fn is_relevant_header( key : &str ) -> bool
    {
      // Include headers that affect response content, exclude dynamic headers
      matches!( key.to_lowercase().as_str(),
        "content-type" | "accept" | "openai-organization" | "openai-project"
      )
    }

    /// Hash a string using the default hasher.
    fn hash_string( s : &str ) -> u64
    {
      let mut hasher = DefaultHasher::new();
      s.hash( &mut hasher );
      hasher.finish()
    }
  }

  /// Thread-safe request cache with TTL and LRU eviction.
  #[ derive( Debug ) ]
  pub struct RequestCache< K, V >
  where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
  {
    /// Storage for cache entries.
    entries : Arc< RwLock< HashMap< K, CacheEntry< V > > > >,
    /// Maximum cache size.
    max_size : usize,
    /// Default TTL for entries.
    default_ttl : Duration,
    /// Cache statistics.
    statistics : CacheStatistics,
    /// Configuration.
    #[ allow( dead_code ) ]
    config : CacheConfig,
  }

  impl< K, V > RequestCache< K, V >
  where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
  {
    /// Create a new request cache.
    #[ inline ]
    #[ must_use ]
    pub fn new( max_size : usize, default_ttl : Duration ) -> Self
    {
      Self
      {
        entries : Arc::new( RwLock::new( HashMap::new() ) ),
        max_size,
        default_ttl,
        statistics : CacheStatistics::default(),
        config : CacheConfig
        {
          max_size,
          default_ttl,
          ..Default::default()
        },
      }
    }

    /// Create a new request cache with custom configuration.
    #[ inline ]
    #[ must_use ]
    pub fn with_config( config : CacheConfig ) -> Self
    {
      Self
      {
        entries : Arc::new( RwLock::new( HashMap::new() ) ),
        max_size : config.max_size,
        default_ttl : config.default_ttl,
        statistics : CacheStatistics::default(),
        config,
      }
    }

    /// Get a value from the cache.
    #[ inline ]
    pub async fn get( &self, key : &K ) -> Option< V >
    {
      let entries = self.entries.read().await;

      if let Some( entry ) = entries.get( key )
      {
        if entry.is_expired().await
        {
          drop( entries );
          // Remove expired entry
          let mut entries = self.entries.write().await;
          entries.remove( key );
          self.statistics.entries.fetch_sub( 1, Ordering::Relaxed );
          self.statistics.misses.fetch_add( 1, Ordering::Relaxed );
          None
        }
        else
        {
          entry.touch().await;
          self.statistics.hits.fetch_add( 1, Ordering::Relaxed );
          Some( entry.value.clone() )
        }
      }
      else
      {
        self.statistics.misses.fetch_add( 1, Ordering::Relaxed );
        None
      }
    }

    /// Insert a value into the cache.
    #[ inline ]
    pub async fn insert( &self, key : K, value : V ) -> Option< V >
    {
      self.insert_with_ttl( key, value, self.default_ttl ).await
    }

    /// Insert a value with custom TTL.
    #[ inline ]
    pub async fn insert_with_ttl( &self, key : K, value : V, ttl : Duration ) -> Option< V >
    {
      let mut entries = self.entries.write().await;

      // Check if we need to evict entries
      if entries.len() >= self.max_size && !entries.contains_key( &key )
      {
        self.evict_lru( &mut entries ).await;
      }

      let entry = CacheEntry::new( value, ttl );
      let old_value = entries.insert( key, entry ).map( |e| e.value );

      if old_value.is_none()
      {
        self.statistics.entries.fetch_add( 1, Ordering::Relaxed );
      }

      old_value
    }

    /// Remove a value from the cache.
    #[ inline ]
    pub async fn remove( &self, key : &K ) -> Option< V >
    {
      let mut entries = self.entries.write().await;
      if let Some( entry ) = entries.remove( key )
      {
        self.statistics.entries.fetch_sub( 1, Ordering::Relaxed );
        Some( entry.value )
      }
      else
      {
        None
      }
    }

    /// Check if the cache contains a key.
    #[ inline ]
    pub async fn contains_key( &self, key : &K ) -> bool
    {
      let entries = self.entries.read().await;
      if let Some( entry ) = entries.get( key )
      {
        !entry.is_expired().await
      }
      else
      {
        false
      }
    }

    /// Get the number of entries in the cache.
    #[ inline ]
    pub async fn len( &self ) -> usize
    {
      let entries = self.entries.read().await;
      entries.len()
    }

    /// Check if the cache is empty.
    #[ inline ]
    pub async fn is_empty( &self ) -> bool
    {
      let entries = self.entries.read().await;
      entries.is_empty()
    }

    /// Clear all entries from the cache.
    #[ inline ]
    pub async fn clear( &self )
    {
      let mut entries = self.entries.write().await;
      let count = u32::try_from( entries.len() ).unwrap_or( u32::MAX );
      entries.clear();
      self.statistics.entries.store( 0, Ordering::Relaxed );
      self.statistics.evictions.fetch_add( u64::from( count ), Ordering::Relaxed );
    }

    /// Get cache statistics.
    #[ inline ]
    #[ must_use ]
    pub fn statistics( &self ) -> &CacheStatistics
    {
      &self.statistics
    }

    /// Cleanup expired entries.
    #[ inline ]
    pub async fn cleanup_expired( &self ) -> usize
    {
      let mut entries = self.entries.write().await;
      let mut keys_to_remove = Vec::new();

      for ( key, entry ) in entries.iter()
      {
        if entry.is_expired().await
        {
          keys_to_remove.push( key.clone() );
        }
      }

      let removed_count = keys_to_remove.len();
      for key in keys_to_remove
      {
        entries.remove( &key );
      }

      if removed_count > 0
      {
        self.statistics.entries.fetch_sub( u32::try_from( removed_count ).unwrap_or( u32::MAX ), Ordering::Relaxed );
        self.statistics.evictions.fetch_add( u64::try_from( removed_count ).unwrap_or( u64::MAX ), Ordering::Relaxed );
      }

      removed_count
    }

    /// Evict the least recently used entry.
    async fn evict_lru( &self, entries : &mut HashMap< K, CacheEntry< V > > )
    {
      if entries.is_empty()
      {
        return;
      }

      // Find the entry with the oldest last_accessed time
      let mut oldest_key = None;
      let mut oldest_time = Instant::now();

      for ( key, entry ) in entries.iter()
      {
        if let Ok( last_accessed ) = entry.last_accessed.lock()
        {
          if oldest_key.is_none() || *last_accessed < oldest_time
          {
            oldest_time = *last_accessed;
            oldest_key = Some( key.clone() );
          }
        }
      }

      if let Some( key ) = oldest_key
      {
        entries.remove( &key );
        self.statistics.entries.fetch_sub( 1, Ordering::Relaxed );
        self.statistics.evictions.fetch_add( 1, Ordering::Relaxed );
      }
    }
  }

  /// Cache implementation specifically for API requests and responses.
  pub type ApiRequestCache = RequestCache< RequestCacheKey, serde_json::Value >;

  impl ApiRequestCache
  {
    /// Create a cache optimized for API requests.
    #[ inline ]
    #[ must_use ]
    pub fn new_api_cache() -> Self
    {
      Self::new( 1000, Duration::from_secs( 300 ) )
    }

    /// Cache an API response with separate request and response types.
    ///
    /// # Errors
    ///
    /// Returns an error if cache key generation or value serialization fails.
    #[ inline ]
    pub async fn cache_response< I: Serialize, O: Serialize >(
      &self,
      endpoint : &str,
      method : &str,
      request_body : Option< &I >,
      headers : &HashMap<  String, String  >,
      response : &O,
    ) -> crate::error::Result< () >
    {
      let key = RequestCacheKey::new( endpoint, method, request_body, headers )?;
      let value = serde_json::to_value( response ).map_err( |e|
        crate ::error::OpenAIError::Internal( format!( "Failed to serialize response for caching : {e}" ) )
      )?;

      self.insert( key, value ).await;
      Ok( () )
    }

    /// Get a cached API response with separate request and response types.
    ///
    /// # Errors
    ///
    /// Returns an error if cache key generation or response deserialization fails.
    #[ inline ]
    pub async fn get_response< I: Serialize, O: for< 'de > Deserialize< 'de > >(
      &self,
      endpoint : &str,
      method : &str,
      request_body : Option< &I >,
      headers : &HashMap<  String, String  >,
    ) -> crate::error::Result< Option< O > >
    {
      let key = RequestCacheKey::new( endpoint, method, request_body, headers )?;

      if let Some( value ) = self.get( &key ).await
      {
        let response = serde_json::from_value( value ).map_err( |e|
          crate ::error::OpenAIError::Internal( format!( "Failed to deserialize cached response : {e}" ) )
        )?;
        Ok( Some( response ) )
      }
      else
      {
        Ok( None )
      }
    }
  }

} // end mod private

crate ::mod_interface!
{
  exposed use
  {
    CacheConfig,
    CacheStatistics,
    CacheEntry,
    RequestCacheKey,
    RequestCache,
    ApiRequestCache,
  };
}