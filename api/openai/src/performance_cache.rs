//! High-Performance Caching System
//!
//! This module implements an intelligent caching layer for `OpenAI` API responses
//! to significantly reduce latency and API costs for repeated requests.
//!
//! This module is feature-gated behind the `caching` and `compression` feature flags.

use mod_interface::mod_interface;

#[ cfg( all( feature = "caching", feature = "compression" ) ) ]
mod private
{
  use std::
  {
    collections ::HashMap,
    sync ::Arc,
  };
  use core::
  {
    hash ::Hash,
    time ::Duration,
  };
  use std::time::Instant;
  use tokio::sync::RwLock;
  use blake3::{ Hash as Blake3Hash, Hasher as Blake3Hasher };

  /// Cache configuration with performance optimizations
  #[ derive( Debug, Clone ) ]
  pub struct CacheConfig
  {
    /// Maximum number of entries in cache
    pub max_entries : usize,
    /// Default TTL for cached responses
    pub default_ttl : Duration,
    /// Maximum memory usage in bytes
    pub max_memory_bytes : usize,
    /// Enable compression for large responses
    pub enable_compression : bool,
    /// Cache hit ratio monitoring interval
    pub metrics_interval : Duration,
  }

  impl Default for CacheConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        max_entries : 10000,
        default_ttl : Duration::from_secs( 300 ), // 5 minutes
        max_memory_bytes : 100 * 1024 * 1024, // 100MB
        enable_compression : true,
        metrics_interval : Duration::from_secs( 60 ),
      }
    }
  }

  /// Cache key with smart hashing for optimal performance
  #[ derive( Debug, Clone, PartialEq, Eq, Hash ) ]
  pub struct CacheKey
  {
    /// HTTP method (GET, POST, etc.)
    pub method : String,
    /// API endpoint path
    pub path : String,
    /// Request body hash (Blake3 for speed)
    pub body_hash : Blake3Hash,
    /// Query parameters hash
    pub query_hash : Blake3Hash,
  }

  impl CacheKey
  {
    /// Create optimized cache key with fast hashing
    #[ inline ]
    #[ must_use ]
    pub fn new( method : &str, path : &str, body : &[u8], query : &str ) -> Self
    {
      let mut body_hasher = Blake3Hasher::new();
      body_hasher.update( body );
      let body_hash = body_hasher.finalize();

      let mut query_hasher = Blake3Hasher::new();
      query_hasher.update( query.as_bytes() );
      let query_hash = query_hasher.finalize();

      Self
      {
        method : method.to_string(),
        path : path.to_string(),
        body_hash,
        query_hash,
      }
    }

    /// Generate cache key for GET requests (no body)
    #[ inline ]
    #[ must_use ]
    pub fn for_get( path : &str, query : &str ) -> Self
    {
      Self::new( "GET", path, &[], query )
    }

    /// Generate cache key for POST requests with body
    #[ inline ]
    #[ must_use ]
    pub fn for_post( path : &str, body : &[u8] ) -> Self
    {
      Self::new( "POST", path, body, "" )
    }
  }

  /// Cached response with metadata
  #[ derive( Debug, Clone ) ]
  pub struct CachedResponse
  {
    /// Response data (potentially compressed)
    pub data : Vec< u8 >,
    /// When this entry was cached
    pub cached_at : Instant,
    /// When this entry expires
    pub expires_at : Instant,
    /// Size in bytes for memory management
    pub size_bytes : usize,
    /// Access count for LRU eviction
    pub access_count : u64,
    /// Last access time
    pub last_accessed : Instant,
    /// Whether data is compressed
    pub is_compressed : bool,
    /// Response content type
    pub content_type : String,
  }

  impl CachedResponse
  {
    /// Create new cached response with automatic compression
    #[ inline ]
    #[ must_use ]
    pub fn new( data : Vec< u8 >, ttl : Duration, content_type : String, enable_compression : bool ) -> Self
    {
      let now = Instant::now();
      let ( final_data, is_compressed ) = if enable_compression && data.len() > 1024
      {
        match Self::compress( &data )
        {
          Ok( compressed ) if compressed.len() < data.len() => ( compressed, true ),
          _ => ( data, false ), // Fallback to uncompressed
        }
      }
      else
      {
        ( data, false )
      };

      Self
      {
        size_bytes : final_data.len(),
        data : final_data,
        cached_at : now,
        expires_at : now + ttl,
        access_count : 1,
        last_accessed : now,
        is_compressed,
        content_type,
      }
    }

    /// Check if entry has expired
    #[ inline ]
    #[ must_use ]
    pub fn is_expired( &self ) -> bool
    {
      Instant::now() > self.expires_at
    }

    /// Mark as accessed for LRU tracking
    #[ inline ]
    pub fn mark_accessed( &mut self )
    {
      self.access_count += 1;
      self.last_accessed = Instant::now();
    }

    /// Get decompressed data
    ///
    /// # Errors
    /// Returns an error if decompression fails for compressed data.
    #[ inline ]
    pub fn get_data( &mut self ) -> Result< Vec< u8 >, String >
    {
      self.mark_accessed();

      if self.is_compressed
      {
        Self::decompress( &self.data )
      }
      else
      {
        Ok( self.data.clone() )
      }
    }

    /// Compress data using fast algorithm
    fn compress( data : &[u8] ) -> Result< Vec< u8 >, String >
    {
      use flate2::{ write::GzEncoder, Compression };
      use std::io::Write;

      let mut encoder = GzEncoder::new( Vec::new(), Compression::fast() );
      encoder.write_all( data ).map_err( | e | e.to_string() )?;
      encoder.finish().map_err( | e | e.to_string() )
    }

    /// Decompress data
    fn decompress( data : &[u8] ) -> Result< Vec< u8 >, String >
    {
      use flate2::read::GzDecoder;
      use std::io::Read;

      let mut decoder = GzDecoder::new( data );
      let mut decompressed = Vec::new();
      decoder.read_to_end( &mut decompressed ).map_err( | e | e.to_string() )?;
      Ok( decompressed )
    }
  }

  /// Cache performance metrics
  #[ derive( Debug, Clone, Default ) ]
  pub struct CacheMetrics
  {
    /// Total cache hits
    pub hits : u64,
    /// Total cache misses
    pub misses : u64,
    /// Cache hit ratio (0.0 to 1.0)
    pub hit_ratio : f64,
    /// Current memory usage in bytes
    pub memory_usage : usize,
    /// Number of evictions performed
    pub evictions : u64,
    /// Average response time for cache hits (microseconds)
    pub avg_hit_time_us : u64,
    /// Compression ratio (`compressed_size` / `original_size`)
    pub compression_ratio : f64,
  }

  impl CacheMetrics
  {
    /// Update hit ratio calculation
    #[ inline ]
    pub fn update_hit_ratio( &mut self )
    {
      let total = self.hits + self.misses;
      self.hit_ratio = if total > 0 { self.hits as f64 / total as f64 } else { 0.0 };
    }
  }

  /// High-performance cache implementation
  #[ derive( Debug ) ]
  pub struct PerformanceCache
  {
    /// Cache storage with fast concurrent access
    storage : Arc< RwLock< HashMap<  CacheKey, CachedResponse  > > >,
    /// Cache configuration
    config : CacheConfig,
    /// Performance metrics
    metrics : Arc< RwLock< CacheMetrics > >,
    /// Memory usage tracker
    current_memory : Arc< RwLock< usize > >,
  }

  impl PerformanceCache
  {
    /// Create new performance cache
    #[ inline ]
    #[ must_use ]
    pub fn new( config : CacheConfig ) -> Self
    {
      Self
      {
        storage : Arc::new( RwLock::new( HashMap::with_capacity( config.max_entries ) ) ),
        config,
        metrics : Arc::new( RwLock::new( CacheMetrics::default() ) ),
        current_memory : Arc::new( RwLock::new( 0 ) ),
      }
    }

    /// Get cached response if available and not expired
    #[ inline ]
    pub async fn get( &self, key : &CacheKey ) -> Option< Vec< u8 > >
    {
      let start_time = Instant::now();

      let mut storage = self.storage.write().await;

      // Check if entry exists and is expired
      let should_remove = if let Some( cached_response ) = storage.get( key )
      {
        cached_response.is_expired()
      }
      else
      {
        false
      };

      if should_remove
      {
        // Remove expired entry
        if let Some( expired_response ) = storage.remove( key )
        {
          let mut memory = self.current_memory.write().await;
          *memory = memory.saturating_sub( expired_response.size_bytes );
        }

        let mut metrics = self.metrics.write().await;
        metrics.misses += 1;
        metrics.update_hit_ratio();
        return None;
      }

      // Try to get and use the cached response
      if let Some( cached_response ) = storage.get_mut( key )
      {
        // Cache hit - return data
        if let Ok( data ) = cached_response.get_data()
        {
          let mut metrics = self.metrics.write().await;
          metrics.hits += 1;
          let elapsed_micros = u64::try_from( start_time.elapsed().as_micros() ).unwrap_or( u64::MAX );
          metrics.avg_hit_time_us = ( metrics.avg_hit_time_us + elapsed_micros ) / 2;
          metrics.update_hit_ratio();
          Some( data )
        }
        else
        {
          // Decompression failed - need to remove entry
          let size_bytes = cached_response.size_bytes;
          // Drop the mutable reference by ending the scope
          core ::mem::drop( storage.remove( key ) );
          let mut memory = self.current_memory.write().await;
          *memory = memory.saturating_sub( size_bytes );
          None
        }
      }
      else
      {
        // Cache miss
        let mut metrics = self.metrics.write().await;
        metrics.misses += 1;
        metrics.update_hit_ratio();
        None
      }
    }

    /// Store response in cache with automatic eviction
    #[ inline ]
    pub async fn put( &self, key : CacheKey, data : Vec< u8 >, content_type : String )
    {
      self.put_with_ttl( key, data, content_type, self.config.default_ttl ).await;
    }

    /// Store response with custom TTL
    #[ inline ]
    pub async fn put_with_ttl( &self, key : CacheKey, data : Vec< u8 >, content_type : String, ttl : Duration )
    {
      let cached_response = CachedResponse::new( data, ttl, content_type, self.config.enable_compression );

      // Check if we need to evict entries
      self.ensure_capacity( cached_response.size_bytes ).await;

      let mut storage = self.storage.write().await;
      let mut memory = self.current_memory.write().await;

      // Remove old entry if exists
      if let Some( old_response ) = storage.remove( &key )
      {
        *memory = memory.saturating_sub( old_response.size_bytes );
      }

      // Add new entry
      *memory += cached_response.size_bytes;
      storage.insert( key, cached_response );
    }

    /// Ensure cache has capacity for new entry
    async fn ensure_capacity( &self, needed_bytes : usize )
    {
      let current_memory = *self.current_memory.read().await;

      if current_memory + needed_bytes > self.config.max_memory_bytes
      {
        self.evict_lru( needed_bytes ).await;
      }
    }

    /// Evict least recently used entries
    async fn evict_lru( &self, needed_bytes : usize )
    {
      let mut storage = self.storage.write().await;
      let mut memory = self.current_memory.write().await;
      let mut freed_bytes = 0;

      // Collect entries sorted by access time (oldest first)
      let mut entries : Vec< _ > = storage.iter().map( | ( k, v ) | ( k.clone(), v.last_accessed ) ).collect();
      entries.sort_by_key( | ( _, last_accessed ) | *last_accessed );

      // Remove oldest entries until we have enough space
      for ( key, _ ) in entries
      {
        if freed_bytes >= needed_bytes
        {
          break;
        }

        if let Some( response ) = storage.remove( &key )
        {
          freed_bytes += response.size_bytes;
          *memory = memory.saturating_sub( response.size_bytes );

          let mut metrics = self.metrics.write().await;
          metrics.evictions += 1;
        }
      }
    }

    /// Get current cache metrics
    #[ inline ]
    pub async fn metrics( &self ) -> CacheMetrics
    {
      let metrics = self.metrics.read().await;
      let mut result = metrics.clone();
      result.memory_usage = *self.current_memory.read().await;
      result
    }

    /// Clear all cached entries
    #[ inline ]
    pub async fn clear( &self )
    {
      let mut storage = self.storage.write().await;
      let mut memory = self.current_memory.write().await;
      storage.clear();
      *memory = 0;
    }

    /// Remove expired entries
    #[ inline ]
    pub async fn cleanup_expired( &self )
    {
      let mut storage = self.storage.write().await;
      let mut memory = self.current_memory.write().await;
      let now = Instant::now();

      storage.retain( | _, response |
      {
        if response.expires_at <= now
        {
          *memory = memory.saturating_sub( response.size_bytes );
          false
        }
        else
        {
          true
        }
      } );
    }
  }
}

mod_interface!
{
  exposed use
  {
    CacheConfig,
    CacheKey,
    CachedResponse,
    CacheMetrics,
    PerformanceCache,
  };
}