//! Request caching implementation with TTL and LRU eviction.

#[ cfg( feature = "request_caching" ) ]
mod private
{
  use core::time::Duration;
  use std::collections::HashMap;
  use std::sync::{ Arc, RwLock };
  use std::time::Instant;

  /// Configuration for request caching behavior
  #[ derive( Debug, Clone ) ]
  pub struct RequestCacheConfig
  {
    max_entries : usize,
    default_ttl : Duration,
    cleanup_interval : Duration,
  }

  /// Cache entry with metadata
  #[ derive( Debug, Clone ) ]
  pub struct CacheEntry
  {
    value : String,
    #[ allow( dead_code ) ]
    created_at : Instant,
    expires_at : Option< Instant >,
    access_count : u64,
    last_accessed : Instant,
  }

  /// Cache statistics for monitoring
  #[ derive( Debug, Clone, Default ) ]
  pub struct CacheStats
  {
    /// Number of cache hits
    pub hits : u64,
    /// Number of cache misses
    pub misses : u64,
    /// Number of cache evictions
    pub evictions : u64,
  }

  /// Request cache implementation with TTL and LRU eviction
  pub struct RequestCache
  {
    config : RequestCacheConfig,
    entries : Arc< RwLock< HashMap< String, CacheEntry > > >,
    access_order : Arc< RwLock< Vec< String > > >,
    stats : Arc< RwLock< CacheStats > >,
  }

  impl core::fmt::Debug for RequestCache
  {
    #[ inline ]
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      let len = self.len();
      f.debug_struct( "RequestCache" )
       .field( "entries", &len )
       .field( "capacity", &self.config.max_entries )
       .field( "config", &self.config )
       .finish()
    }
  }

  impl RequestCacheConfig
  {
    /// Create a new cache configuration
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        max_entries : 100,
        default_ttl : Duration::from_secs( 300 ), // 5 minutes
        cleanup_interval : Duration::from_secs( 60 ), // 1 minute
      }
    }

    /// Set the maximum number of cache entries
    #[ inline ]
    #[ must_use ]
    pub fn with_max_entries( mut self, max_entries : usize ) -> Self
    {
      self.max_entries = max_entries;
      self
    }

    /// Set the default TTL for cache entries
    #[ inline ]
    #[ must_use ]
    pub fn with_default_ttl( mut self, ttl : Duration ) -> Self
    {
      self.default_ttl = ttl;
      self
    }

    /// Set the cleanup interval for expired entries
    #[ inline ]
    #[ must_use ]
    pub fn with_cleanup_interval( mut self, interval : Duration ) -> Self
    {
      self.cleanup_interval = interval;
      self
    }

    /// Get the maximum number of entries
    #[ inline ]
    #[ must_use ]
    pub fn max_entries( &self ) -> usize
    {
      self.max_entries
    }

    /// Get the default TTL
    #[ inline ]
    #[ must_use ]
    pub fn default_ttl( &self ) -> Duration
    {
      self.default_ttl
    }

    /// Get the cleanup interval
    #[ inline ]
    #[ must_use ]
    pub fn cleanup_interval( &self ) -> Duration
    {
      self.cleanup_interval
    }
  }

  impl Default for RequestCacheConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  impl CacheStats
  {
    /// Calculate hit ratio
    #[ inline ]
    #[ must_use ]
    pub fn hit_ratio( &self ) -> f64
    {
      let total = self.hits + self.misses;
      if total == 0
      {
        0.0
      }
      else
      {
        self.hits as f64 / total as f64
      }
    }
  }

  impl RequestCache
  {
    /// Create a new request cache with the given configuration
    #[ inline ]
    #[ must_use ]
    pub fn new( config : RequestCacheConfig ) -> Self
    {
      Self
      {
        config,
        entries : Arc::new( RwLock::new( HashMap::new() ) ),
        access_order : Arc::new( RwLock::new( Vec::new() ) ),
        stats : Arc::new( RwLock::new( CacheStats::default() ) ),
      }
    }

    /// Generate a cache key from a hashable request
    #[ inline ]
    #[ must_use ]
    pub fn generate_key< T : core::hash::Hash >( &self, request : &T ) -> String
    {
      use std::collections::hash_map::DefaultHasher;
      use core::hash::Hasher;

      let mut hasher = DefaultHasher::new();
      request.hash( &mut hasher );
      format!( "cache_key_{:x}", hasher.finish() )
    }

    /// Get the number of entries in the cache
    #[ inline ]
    #[ must_use ]
    pub fn len( &self ) -> usize
    {
      self.entries.read().unwrap().len()
    }

    /// Check if the cache is empty
    #[ inline ]
    #[ must_use ]
    pub fn is_empty( &self ) -> bool
    {
      self.entries.read().unwrap().is_empty()
    }

    /// Get the capacity of the cache
    #[ inline ]
    #[ must_use ]
    pub fn capacity( &self ) -> usize
    {
      self.config.max_entries
    }

    /// Get cache statistics
    #[ inline ]
    #[ must_use ]
    pub fn stats( &self ) -> CacheStats
    {
      self.stats.read().unwrap().clone()
    }

    /// Clean up expired entries
    #[ inline ]
    fn cleanup_expired( &self )
    {
      let now = Instant::now();
      let mut entries = self.entries.write().unwrap();
      let mut access_order = self.access_order.write().unwrap();

      // Remove expired entries
      entries.retain( | key, entry |
      {
        let expired = if let Some( expires_at ) = entry.expires_at
        {
          now >= expires_at
        }
        else
        {
          false
        };

        if expired
        {
          // Remove from access order
          if let Some( pos ) = access_order.iter().position( | k | k == key )
          {
            access_order.remove( pos );
          }
        }

        !expired
      });
    }

    /// Evict least recently used entry (internal version with locks already held)
    #[ inline ]
    fn evict_lru_internal( &self, entries : &mut HashMap< String, CacheEntry >, access_order : &mut Vec< String > )
    {
      if let Some( lru_key ) = access_order.first().cloned()
      {
        entries.remove( &lru_key );
        access_order.remove( 0 );

        // Update eviction statistics
        let mut stats = self.stats.write().unwrap();
        stats.evictions += 1;
      }
    }

    /// Evict least recently used entry
    #[ inline ]
    fn evict_lru( &self )
    {
      let mut entries = self.entries.write().unwrap();
      let mut access_order = self.access_order.write().unwrap();

      self.evict_lru_internal( &mut entries, &mut access_order );
    }

    /// Update access order for a key
    #[ inline ]
    fn update_access_order( &self, key : String )
    {
      let mut access_order = self.access_order.write().unwrap();

      // Remove old position if exists
      if let Some( pos ) = access_order.iter().position( | k | k == &key )
      {
        access_order.remove( pos );
      }
      access_order.push( key );
    }

    /// Get a value from the cache
    #[ inline ]
    #[ must_use ]
    pub fn get( &self, key : &str ) -> Option< String >
    {
      // Clean up expired entries first
      self.cleanup_expired();

      let mut entries = self.entries.write().unwrap();
      let mut access_order = self.access_order.write().unwrap();
      let mut stats = self.stats.write().unwrap();

      if let Some( entry ) = entries.get_mut( key )
      {
        // Check if expired
        if let Some( expires_at ) = entry.expires_at
        {
          if Instant::now() >= expires_at
          {
            // Remove expired entry
            entries.remove( key );
            if let Some( pos ) = access_order.iter().position( | k | k == key )
            {
              access_order.remove( pos );
            }
            stats.misses += 1;
            return None;
          }
        }

        // Update access metadata
        entry.access_count += 1;
        entry.last_accessed = Instant::now();

        // Update access order
        if let Some( pos ) = access_order.iter().position( | k | k == key )
        {
          access_order.remove( pos );
        }
        access_order.push( key.to_string() );

        stats.hits += 1;
        Some( entry.value.clone() )
      }
      else
      {
        stats.misses += 1;
        None
      }
    }

    /// Insert a value into the cache with optional TTL
    #[ inline ]
    pub fn insert( &self, key : String, value : String, ttl : Option< Duration > )
    {
      let now = Instant::now();
      let expires_at = ttl.or( Some( self.config.default_ttl ) ).map( | t | now + t );

      let entry = CacheEntry
      {
        value,
        created_at : now,
        expires_at,
        access_count : 0,
        last_accessed : now,
      };

      // Clean up expired entries first
      self.cleanup_expired();

      let mut entries = self.entries.write().unwrap();
      let mut access_order = self.access_order.write().unwrap();

      // Check if we need to evict entries due to capacity
      if entries.len() >= self.config.max_entries && !entries.contains_key( &key )
      {
        self.evict_lru_internal( &mut entries, &mut access_order );
      }

      // Insert or update the entry
      entries.insert( key.clone(), entry );

      // Update access order
      if let Some( pos ) = access_order.iter().position( | k | k == &key )
      {
        access_order.remove( pos );
      }
      access_order.push( key );
    }

    /// Put a value into the cache
    #[ inline ]
    pub fn put( &self, key : String, value : String )
    {
      self.put_with_ttl( key, value, self.config.default_ttl );
    }

    /// Put a value into the cache with custom TTL
    #[ inline ]
    pub fn put_with_ttl( &self, key : String, value : String, ttl : Duration )
    {
      // Clean up expired entries first
      self.cleanup_expired();

      let mut entries = self.entries.write().unwrap();

      // Evict LRU if at capacity and key doesn't exist
      if entries.len() >= self.config.max_entries && !entries.contains_key( &key )
      {
        drop( entries ); // Release write lock before evicting
        self.evict_lru();
        entries = self.entries.write().unwrap();
      }

      let now = Instant::now();
      let entry = CacheEntry
      {
        value,
        created_at : now,
        expires_at : Some( now + ttl ),
        access_count : 0,
        last_accessed : now,
      };

      entries.insert( key.clone(), entry );
      drop( entries ); // Release write lock

      self.update_access_order( key );
    }

    /// Check if the cache contains a key
    #[ inline ]
    #[ must_use ]
    pub fn contains_key( &self, key : &str ) -> bool
    {
      self.get( key ).is_some()
    }

    /// Remove a specific key from the cache (alias for remove)
    #[ inline ]
    pub fn invalidate( &self, key : &str )
    {
      self.remove( key );
    }

    /// Remove a value from the cache
    #[ inline ]
    pub fn remove( &self, key : &str )
    {
      let mut entries = self.entries.write().unwrap();
      let mut access_order = self.access_order.write().unwrap();

      entries.remove( key );
      if let Some( pos ) = access_order.iter().position( | k | k == key )
      {
        access_order.remove( pos );
      }
    }

    /// Clear all entries from the cache
    #[ inline ]
    pub fn clear( &self )
    {
      let mut entries = self.entries.write().unwrap();
      let mut access_order = self.access_order.write().unwrap();

      entries.clear();
      access_order.clear();
    }

    /// Invalidate entries matching a pattern
    #[ inline ]
    pub fn invalidate_pattern( &self, pattern : &str )
    {
      let mut entries = self.entries.write().unwrap();
      let mut access_order = self.access_order.write().unwrap();

      let keys_to_remove : Vec< String > = entries
        .keys()
        .filter( | key | Self::matches_pattern( key, pattern ) )
        .cloned()
        .collect();

      for key in &keys_to_remove
      {
        entries.remove( key );
        if let Some( pos ) = access_order.iter().position( | k | k == key )
        {
          access_order.remove( pos );
        }
      }
    }

    /// Check if a key matches a pattern (supports * wildcards)
    #[ inline ]
    fn matches_pattern( key : &str, pattern : &str ) -> bool
    {
      if pattern == "*"
      {
        true
      }
      else if pattern.ends_with( '*' )
      {
        let prefix = &pattern[ ..pattern.len() - 1 ];
        key.starts_with( prefix )
      }
      else if pattern.starts_with( '*' )
      {
        let suffix = &pattern[ 1.. ];
        key.ends_with( suffix )
      }
      else
      {
        key == pattern
      }
    }
  }

  impl Clone for RequestCache
  {
    #[ inline ]
    fn clone( &self ) -> Self
    {
      Self
      {
        config : self.config.clone(),
        entries : Arc::clone( &self.entries ),
        access_order : Arc::clone( &self.access_order ),
        stats : Arc::clone( &self.stats ),
      }
    }
  }

  impl core::fmt::Display for RequestCache
  {
    #[ inline ]
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      let len = self.len();
      let capacity = self.capacity();
      write!( f, "Cache [{len}/{capacity}]" )
    }
  }
}

#[ cfg( feature = "request_caching" ) ]
crate ::mod_interface!
{
  exposed use private::RequestCache;
  exposed use private::RequestCacheConfig;
  exposed use private::CacheEntry;
  exposed use private::CacheStats;
}
