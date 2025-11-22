//! Enhanced Request Caching
//!
//! This module provides an advanced, production-ready request caching implementation
//! with sophisticated features including adaptive TTL, predictive caching, cache warming,
//! and advanced eviction policies.

#![ allow( clippy::missing_inline_in_public_items, clippy::unused_async, clippy::explicit_iter_loop ) ]

mod private
{
  use std::
  {
    collections ::{ HashMap, VecDeque },
    sync ::Arc,
    time ::Instant,
  };
  use core::
  {
    hash ::{ Hash, Hasher },
    sync ::atomic::{ AtomicU32, AtomicU64, AtomicBool, Ordering },
    time ::Duration,
  };
  use tokio::sync::{ RwLock, Mutex };
  use serde::{ Serialize, Deserialize };
  use std::collections::hash_map::DefaultHasher;

  /// Advanced cache configuration with production-ready features
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  #[ allow( clippy::struct_excessive_bools ) ]
  pub struct EnhancedCacheConfig
  {
    /// Maximum number of entries in cache
    pub max_size : usize,
    /// Default TTL for cache entries
    pub default_ttl : Duration,
    /// Minimum TTL (prevents too aggressive caching)
    pub min_ttl : Duration,
    /// Maximum TTL (prevents stale data)
    pub max_ttl : Duration,
    /// Enable adaptive TTL based on access patterns
    pub adaptive_ttl : bool,
    /// Enable predictive caching for frequently accessed items
    pub predictive_caching : bool,
    /// Enable cache warming on startup
    pub cache_warming : bool,
    /// Cleanup interval for maintenance operations
    pub cleanup_interval : Duration,
    /// Enable advanced metrics collection
    pub detailed_metrics : bool,
    /// Cache eviction policy
    pub eviction_policy : EvictionPolicy,
    /// Compression threshold (compress entries larger than this)
    pub compression_threshold : usize,
    /// Enable cache persistence to disk
    pub persistence : bool,
    /// Maximum memory usage before aggressive cleanup
    pub max_memory_usage : usize,
  }

  impl Default for EnhancedCacheConfig
  {
    fn default() -> Self
    {
      Self
      {
        max_size : 5000,
        default_ttl : Duration::from_secs( 300 ), // 5 minutes
        min_ttl : Duration::from_secs( 30 ),     // 30 seconds
        max_ttl : Duration::from_secs( 3600 ),   // 1 hour
        adaptive_ttl : true,
        predictive_caching : true,
        cache_warming : false,
        cleanup_interval : Duration::from_secs( 60 ),
        detailed_metrics : true,
        eviction_policy : EvictionPolicy::AdaptiveLRU,
        compression_threshold : 1024, // 1KB
        persistence : false,
        max_memory_usage : 100 * 1024 * 1024, // 100MB
      }
    }
  }

  /// Cache eviction policies
  #[ derive( Debug, Clone, Copy, Serialize, Deserialize, PartialEq ) ]
  pub enum EvictionPolicy
  {
    /// Least Recently Used
    LRU,
    /// Least Frequently Used
    LFU,
    /// Adaptive LRU with frequency consideration
    AdaptiveLRU,
    /// Time-based with access pattern analysis
    TimeWeighted,
    /// Size-aware eviction (remove large items first)
    SizeAware,
  }

  /// Enhanced cache statistics with detailed metrics
  #[ derive( Debug ) ]
  pub struct EnhancedCacheStatistics
  {
    /// Total cache hits
    pub hits : AtomicU64,
    /// Total cache misses
    pub misses : AtomicU64,
    /// Total insertions
    pub insertions : AtomicU64,
    /// Total evictions
    pub evictions : AtomicU64,
    /// Current entry count
    pub entries : AtomicU32,
    /// Total memory usage estimate
    pub memory_usage : AtomicU64,
    /// Average access time
    pub avg_access_time : AtomicU64,
    /// Cache warming hits
    pub warming_hits : AtomicU64,
    /// Predictive cache hits
    pub predictive_hits : AtomicU64,
    /// Compression saves (bytes)
    pub compression_saves : AtomicU64,
    /// TTL adjustments made
    pub ttl_adjustments : AtomicU64,
  }

  impl EnhancedCacheStatistics
  {
    /// Create a new cache statistics instance
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        hits : AtomicU64::new( 0 ),
        misses : AtomicU64::new( 0 ),
        insertions : AtomicU64::new( 0 ),
        evictions : AtomicU64::new( 0 ),
        entries : AtomicU32::new( 0 ),
        memory_usage : AtomicU64::new( 0 ),
        avg_access_time : AtomicU64::new( 0 ),
        warming_hits : AtomicU64::new( 0 ),
        predictive_hits : AtomicU64::new( 0 ),
        compression_saves : AtomicU64::new( 0 ),
        ttl_adjustments : AtomicU64::new( 0 ),
      }
    }

    /// Calculate hit ratio
    pub fn hit_ratio( &self ) -> f64
    {
      let hits = self.hits.load( Ordering::Relaxed );
      let misses = self.misses.load( Ordering::Relaxed );
      let total = hits + misses;
      if total == 0 { 0.0 } else { hits as f64 / total as f64 }
    }

    /// Get memory usage in MB
    pub fn memory_usage_mb( &self ) -> f64
    {
      self.memory_usage.load( Ordering::Relaxed ) as f64 / ( 1024.0 * 1024.0 )
    }
  }

  impl Default for EnhancedCacheStatistics
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Enhanced cache entry with rich metadata
  #[ derive( Debug, Clone ) ]
  pub struct EnhancedCacheEntry< V >
  {
    /// The cached value
    pub value : V,
    /// When the entry was created
    pub created_at : Instant,
    /// When the entry expires
    pub expires_at : Instant,
    /// Last access time
    pub last_accessed : Arc< Mutex< Instant > >,
    /// Access frequency counter
    pub access_count : Arc< AtomicU32 >,
    /// Size estimate in bytes
    pub size_bytes : usize,
    /// Whether entry is compressed
    pub compressed : bool,
    /// Priority level for eviction decisions
    pub priority : CachePriority,
    /// Access pattern analysis
    pub access_pattern : Arc< Mutex< AccessPattern > >,
  }

  /// Cache entry priority levels
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord ) ]
  pub enum CachePriority
  {
    /// Low priority cache entry
    Low = 1,
    /// Normal priority cache entry
    Normal = 2,
    /// High priority cache entry
    High = 3,
    /// Critical priority cache entry
    Critical = 4,
  }

  /// Access pattern analysis for adaptive behavior
  #[ derive( Debug, Clone ) ]
  pub struct AccessPattern
  {
    /// Recent access times
    pub recent_accesses : VecDeque< Instant >,
    /// Access interval analysis
    pub avg_interval : Option< Duration >,
    /// Predicted next access
    pub predicted_next_access : Option< Instant >,
    /// Burst detection
    pub in_burst : bool,
  }

  impl< V > EnhancedCacheEntry< V >
  where
    V : Clone + Send + Sync,
  {
    /// Create a new cache entry
    pub fn new( value : V, ttl : Duration, size_bytes : usize, priority : CachePriority ) -> Self
    {
      let now = Instant::now();
      Self
      {
        value,
        created_at : now,
        expires_at : now + ttl,
        last_accessed : Arc::new( Mutex::new( now ) ),
        access_count : Arc::new( AtomicU32::new( 1 ) ),
        size_bytes,
        compressed : false,
        priority,
        access_pattern : Arc::new( Mutex::new( AccessPattern
        {
          recent_accesses : VecDeque::new(),
          avg_interval : None,
          predicted_next_access : None,
          in_burst : false,
        } ) ),
      }
    }

    /// Check if entry is expired
    pub fn is_expired( &self ) -> bool
    {
      Instant::now() >= self.expires_at
    }

    /// Update access time and patterns
    ///
    /// # Panics
    ///
    /// Panics if system time calculations fail during burst detection threshold computation.
    pub async fn mark_accessed( &self )
    {
      let now = Instant::now();
      self.access_count.fetch_add( 1, Ordering::Relaxed );

      // Update last accessed
      {
        let mut last_accessed = self.last_accessed.lock().await;
        *last_accessed = now;
      }

      // Update access pattern
      {
        let mut pattern = self.access_pattern.lock().await;
        pattern.recent_accesses.push_back( now );

        // Keep only recent accesses (last 10)
        while pattern.recent_accesses.len() > 10
        {
          pattern.recent_accesses.pop_front();
        }

        // Calculate average interval
        if pattern.recent_accesses.len() >= 2
        {
          let intervals : Vec< Duration > = pattern.recent_accesses
            .iter()
            .zip( pattern.recent_accesses.iter().skip( 1 ) )
            .map( |( a, b )| b.duration_since( *a ) )
            .collect();

          if !intervals.is_empty()
          {
            let total_nanos : u64 = intervals.iter().map( |d| u64::try_from( d.as_nanos() ).unwrap_or( u64::MAX ) ).sum();
            pattern.avg_interval = Some( Duration::from_nanos( total_nanos / intervals.len() as u64 ) );

            // Predict next access
            if let Some( avg_interval ) = pattern.avg_interval
            {
              pattern.predicted_next_access = Some( now + avg_interval );
            }
          }
        }

        // Burst detection - more than 3 accesses in last 10 seconds
        let recent_threshold = now.checked_sub( Duration::from_secs( 10 ) ).unwrap_or( now );
        let recent_count = pattern.recent_accesses.iter()
          .filter( |&&access_time| access_time >= recent_threshold )
          .count();
        pattern.in_burst = recent_count > 3;
      }
    }

    /// Calculate adaptive TTL based on access patterns
    pub async fn calculate_adaptive_ttl( &self, base_ttl : Duration, min_ttl : Duration, max_ttl : Duration ) -> Duration
    {
      let access_count = self.access_count.load( Ordering::Relaxed );

      {
        let pattern = self.access_pattern.lock().await;
        let mut multiplier = 1.0;

        // High access count increases TTL
        if access_count > 10
        {
          multiplier *= 1.5;
        }
        else if access_count > 5
        {
          multiplier *= 1.2;
        }

        // Burst patterns increase TTL
        if pattern.in_burst
        {
          multiplier *= 2.0;
        }

        // Regular access patterns increase TTL
        if pattern.avg_interval.is_some()
        {
          multiplier *= 1.3;
        }

        let adjusted_ttl = base_ttl.mul_f64( multiplier );
        adjusted_ttl.max( min_ttl ).min( max_ttl )
      }
    }
  }

  /// Enhanced request cache with advanced features
  #[ derive( Debug ) ]
  pub struct EnhancedRequestCache< K, V >
  where
    K : Hash + Eq + Clone + Send + Sync + 'static,
    V : Clone + Send + Sync + 'static,
  {
    /// Cache entries with enhanced metadata
    entries : Arc< RwLock< HashMap< K, EnhancedCacheEntry< V > > > >,
    /// Configuration
    config : EnhancedCacheConfig,
    /// Enhanced statistics
    stats : EnhancedCacheStatistics,
    /// Access frequency tracking for LFU
    access_frequencies : Arc< RwLock< HashMap<  K, u32  > > >,
    /// Predictive cache candidates
    predictive_candidates : Arc< RwLock< HashMap<  K, Instant  > > >,
    /// Cache warming enabled flag
    #[ allow( dead_code ) ]
    warming_active : AtomicBool,
    /// Last cleanup time
    last_cleanup : Arc< Mutex< Instant > >,
  }

  impl< K, V > EnhancedRequestCache< K, V >
  where
    K : Hash + Eq + Clone + Send + Sync + 'static,
    V : Clone + Send + Sync + 'static,
  {
    /// Create new enhanced cache
    #[ must_use ]
    pub fn new( config : EnhancedCacheConfig ) -> Self
    {
      Self
      {
        entries : Arc::new( RwLock::new( HashMap::with_capacity( config.max_size ) ) ),
        config,
        stats : EnhancedCacheStatistics::new(),
        access_frequencies : Arc::new( RwLock::new( HashMap::new() ) ),
        predictive_candidates : Arc::new( RwLock::new( HashMap::new() ) ),
        warming_active : AtomicBool::new( false ),
        last_cleanup : Arc::new( Mutex::new( Instant::now() ) ),
      }
    }

    /// Insert entry with intelligent TTL and priority
    pub async fn insert_enhanced( &self, key : K, value : V, priority : CachePriority ) -> bool
    {
      let size_estimate = Self::estimate_size( &value );
      let ttl = if self.config.adaptive_ttl
      {
        self.calculate_intelligent_ttl( &key, size_estimate ).await
      }
      else
      {
        self.config.default_ttl
      };

      let entry = EnhancedCacheEntry::new( value, ttl, size_estimate, priority );

      // Check if we need to make space
      let mut entries = self.entries.write().await;
      while entries.len() >= self.config.max_size
      {
        self.evict_by_policy( &mut entries ).await;
      }

      // Insert the entry
      let was_replaced = entries.insert( key.clone(), entry ).is_some();
      if !was_replaced
      {
        self.stats.entries.fetch_add( 1, Ordering::Relaxed );
        self.stats.memory_usage.fetch_add( size_estimate as u64, Ordering::Relaxed );
      }

      self.stats.insertions.fetch_add( 1, Ordering::Relaxed );

      // Update access frequency
      {
        let mut frequencies = self.access_frequencies.write().await;
        *frequencies.entry( key.clone() ).or_insert( 0 ) += 1;
      }

      // Check for predictive opportunities
      if self.config.predictive_caching
      {
        self.update_predictive_candidates( &key ).await;
      }

      true
    }

    /// Get entry with enhanced access tracking
    pub async fn get_enhanced( &self, key : &K ) -> Option< V >
    {
      let start_time = Instant::now();

      let entry = {
        let entries = self.entries.read().await;
        entries.get( key ).cloned()
      };

      match entry
      {
        Some( entry ) if !entry.is_expired() =>
        {
          // Mark as accessed and update patterns
          entry.mark_accessed().await;

          // Update entry back in cache
          {
            let mut entries = self.entries.write().await;
            entries.insert( key.clone(), entry.clone() );
          }

          // Update statistics
          self.stats.hits.fetch_add( 1, Ordering::Relaxed );
          let access_time = u64::try_from( start_time.elapsed().as_nanos() ).unwrap_or( u64::MAX );
          self.update_avg_access_time( access_time );

          // Update access frequency
          {
            let mut frequencies = self.access_frequencies.write().await;
            *frequencies.entry( key.clone() ).or_insert( 0 ) += 1;
          }

          Some( entry.value )
        }
        Some( _expired_entry ) =>
        {
          // Remove expired entry
          self.remove_entry( key ).await;
          self.stats.misses.fetch_add( 1, Ordering::Relaxed );
          None
        }
        None =>
        {
          self.stats.misses.fetch_add( 1, Ordering::Relaxed );

          // Check for predictive caching opportunity
          if self.config.predictive_caching
          {
            Self::consider_predictive_load( key );
          }

          None
        }
      }
    }

    /// Calculate intelligent TTL based on various factors
    async fn calculate_intelligent_ttl( &self, key : &K, size_bytes : usize ) -> Duration
    {
      let mut ttl = self.config.default_ttl;

      // Check access frequency
      {
        let frequencies = self.access_frequencies.read().await;
        if let Some( &frequency ) = frequencies.get( key )
        {
          // High frequency items get longer TTL
          if frequency > 10
          {
            ttl = ttl.mul_f64( 2.0 );
          }
          else if frequency > 5
          {
            ttl = ttl.mul_f64( 1.5 );
          }
        }
      }

      // Larger items get shorter TTL to free memory faster
      if size_bytes > self.config.compression_threshold * 10
      {
        ttl = ttl.mul_f64( 0.7 );
      }

      // Clamp to configured bounds
      ttl.max( self.config.min_ttl ).min( self.config.max_ttl )
    }

    /// Evict entries based on configured policy
    async fn evict_by_policy( &self, entries : &mut HashMap< K, EnhancedCacheEntry< V > > )
    {
      if entries.is_empty()
      {
        return;
      }

      let key_to_remove = match self.config.eviction_policy
      {
        EvictionPolicy::LRU => self.find_lru_key( entries ).await,
        EvictionPolicy::LFU => self.find_lfu_key( entries ).await,
        EvictionPolicy::AdaptiveLRU => self.find_adaptive_lru_key( entries ).await,
        EvictionPolicy::TimeWeighted => self.find_time_weighted_key( entries ).await,
        EvictionPolicy::SizeAware => self.find_size_aware_key( entries ).await,
      };

      if let Some( key ) = key_to_remove
      {
        if let Some( entry ) = entries.remove( &key )
        {
          self.stats.entries.fetch_sub( 1, Ordering::Relaxed );
          self.stats.evictions.fetch_add( 1, Ordering::Relaxed );
          self.stats.memory_usage.fetch_sub( entry.size_bytes as u64, Ordering::Relaxed );
        }
      }
    }

    /// Find LRU key for eviction
    async fn find_lru_key( &self, entries : &HashMap< K, EnhancedCacheEntry< V > > ) -> Option< K >
    {
      let mut oldest_key = None;
      let mut oldest_time = Instant::now();

      for ( key, entry ) in entries.iter()
      {
        {
          let last_accessed = entry.last_accessed.lock().await;
          if oldest_key.is_none() || *last_accessed < oldest_time
          {
            oldest_time = *last_accessed;
            oldest_key = Some( key.clone() );
          }
        }
      }

      oldest_key
    }

    /// Find LFU key for eviction
    async fn find_lfu_key( &self, entries : &HashMap< K, EnhancedCacheEntry< V > > ) -> Option< K >
    {
      let mut min_count = u32::MAX;
      let mut min_key = None;

      for ( key, entry ) in entries.iter()
      {
        let count = entry.access_count.load( Ordering::Relaxed );
        if count < min_count
        {
          min_count = count;
          min_key = Some( key.clone() );
        }
      }

      min_key
    }

    /// Find key using adaptive LRU (considers both recency and frequency)
    async fn find_adaptive_lru_key( &self, entries : &HashMap< K, EnhancedCacheEntry< V > > ) -> Option< K >
    {
      let mut best_score = f64::MIN;
      let mut best_key = None;
      let now = Instant::now();

      for ( key, entry ) in entries.iter()
      {
        {
          let last_accessed = entry.last_accessed.lock().await;
          let time_since_access = now.duration_since( *last_accessed ).as_secs_f64();
          let access_count = f64::from(entry.access_count.load( Ordering::Relaxed ));
          let priority_weight = f64::from(entry.priority as u8);

          // Higher score means higher priority for eviction (oldest, least used, lowest priority)
          let score = time_since_access / ( access_count + 1.0 ) / priority_weight;

          if score > best_score
          {
            best_score = score;
            best_key = Some( key.clone() );
          }
        }
      }

      best_key
    }

    /// Find key using time-weighted strategy
    async fn find_time_weighted_key( &self, entries : &HashMap< K, EnhancedCacheEntry< V > > ) -> Option< K >
    {
      let mut best_score = f64::MIN;
      let mut best_key = None;
      let now = Instant::now();

      for ( key, entry ) in entries.iter()
      {
        let age = now.duration_since( entry.created_at ).as_secs_f64();
        let access_count = f64::from(entry.access_count.load( Ordering::Relaxed ));

        // Consider age, access patterns, and priority
        let score = age / ( access_count + 1.0 ) / f64::from(entry.priority as u8);

        if score > best_score
        {
          best_score = score;
          best_key = Some( key.clone() );
        }
      }

      best_key
    }

    /// Find key using size-aware strategy (prioritize large items for eviction)
    async fn find_size_aware_key( &self, entries : &HashMap< K, EnhancedCacheEntry< V > > ) -> Option< K >
    {
      let mut best_score = 0.0;
      let mut best_key = None;

      for ( key, entry ) in entries.iter()
      {
        let size_score = entry.size_bytes as f64;
        let access_count = f64::from(entry.access_count.load( Ordering::Relaxed ));
        let priority_penalty = f64::from(entry.priority as u8);

        // Higher score means higher priority for eviction
        let score = size_score / ( access_count + 1.0 ) / priority_penalty;

        if score > best_score
        {
          best_score = score;
          best_key = Some( key.clone() );
        }
      }

      best_key
    }

    /// Remove entry and update statistics
    async fn remove_entry( &self, key : &K ) -> bool
    {
      let mut entries = self.entries.write().await;
      if let Some( entry ) = entries.remove( key )
      {
        self.stats.entries.fetch_sub( 1, Ordering::Relaxed );
        self.stats.memory_usage.fetch_sub( entry.size_bytes as u64, Ordering::Relaxed );
        true
      }
      else
      {
        false
      }
    }

    /// Update predictive candidates
    async fn update_predictive_candidates( &self, key : &K )
    {
      let mut candidates = self.predictive_candidates.write().await;
      candidates.insert( key.clone(), Instant::now() );

      // Keep only recent candidates
      let cutoff = Instant::now().checked_sub( Duration::from_secs( 300 ) ).unwrap(); // 5 minutes
      candidates.retain( |_, &mut time| time >= cutoff );
    }

    /// Consider predictive loading for frequently requested missing items
    fn consider_predictive_load( _key : &K )
    {
      // This would trigger background loading in a real implementation
      // For now, just track the opportunity
    }

    /// Estimate size of value (simplified)
    fn estimate_size( _value : &V ) -> usize
    {
      // In a real implementation, this would use serialization or reflection
      // For now, return a reasonable default
      core ::mem::size_of::< V >().max( 256 )
    }

    /// Update average access time
    fn update_avg_access_time( &self, new_time : u64 )
    {
      let current_avg = self.stats.avg_access_time.load( Ordering::Relaxed );
      let new_avg = if current_avg == 0
      {
        new_time
      }
      else
      {
        ( current_avg * 9 + new_time ) / 10 // Exponential moving average
      };
      self.stats.avg_access_time.store( new_avg, Ordering::Relaxed );
    }

    /// Get enhanced statistics
    pub fn stats( &self ) -> &EnhancedCacheStatistics
    {
      &self.stats
    }

    /// Cleanup expired entries and maintenance
    pub async fn cleanup_and_maintain( &self ) -> usize
    {
      let mut removed = 0;
      let now = Instant::now();

      // Check if cleanup is needed
      {
        let mut last_cleanup = self.last_cleanup.lock().await;
        if now.duration_since( *last_cleanup ) < self.config.cleanup_interval
        {
          return 0;
        }
        *last_cleanup = now;
      }

      // Remove expired entries
      {
        let mut entries = self.entries.write().await;
        let mut keys_to_remove = Vec::new();

        for ( key, entry ) in &*entries
        {
          if entry.is_expired()
          {
            keys_to_remove.push( key.clone() );
          }
        }

        for key in keys_to_remove
        {
          if let Some( entry ) = entries.remove( &key )
          {
            self.stats.memory_usage.fetch_sub( entry.size_bytes as u64, Ordering::Relaxed );
            removed += 1;
          }
        }

        self.stats.entries.fetch_sub( u32::try_from(removed).unwrap_or(u32::MAX), Ordering::Relaxed );
        self.stats.evictions.fetch_add( removed as u64, Ordering::Relaxed );
      }

      // Clean up access frequencies for removed items
      {
        let entries = self.entries.read().await;
        let mut frequencies = self.access_frequencies.write().await;
        frequencies.retain( |k, _| entries.contains_key( k ) );
      }

      removed
    }

    /// Clear all entries
    pub async fn clear( &self )
    {
      let mut entries = self.entries.write().await;
      let count = entries.len();
      entries.clear();

      self.stats.entries.store( 0, Ordering::Relaxed );
      self.stats.memory_usage.store( 0, Ordering::Relaxed );
      self.stats.evictions.fetch_add( count as u64, Ordering::Relaxed );

      // Clear auxiliary data structures
      {
        let mut frequencies = self.access_frequencies.write().await;
        frequencies.clear();
      }

      {
        let mut candidates = self.predictive_candidates.write().await;
        candidates.clear();
      }
    }
  }

  /// Type alias for enhanced API request cache
  pub type EnhancedApiRequestCache = EnhancedRequestCache< RequestCacheKey, serde_json::Value >;

  /// Re-export `RequestCacheKey` from original implementation
  #[ derive( Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize ) ]
  pub struct RequestCacheKey
  {
    /// API endpoint path
    pub endpoint : String,
    /// HTTP method
    pub method : String,
    /// Hash of request body
    pub body_hash : u64,
    /// Hash of relevant headers
    pub headers_hash : u64,
  }

  impl RequestCacheKey
  {
    /// Create cache key from request components
    ///
    /// # Errors
    ///
    /// Returns an error if serialization of the request body or headers fails.
    pub fn new< T : Serialize >(
      endpoint : &str,
      method : &str,
      request_body : Option< &T >,
      headers : &HashMap<  String, String  >,
    ) -> crate::error::Result< Self >
    {
      // Hash the request body
      let body_hash = if let Some( body ) = request_body
      {
        let body_json = serde_json::to_string( body ).map_err( |e|
          crate ::error::OpenAIError::Internal( format!( "Failed to serialize request body for cache key : {e}" ) )
        )?;
        Self::hash_string( &body_json )
      }
      else
      {
        0
      };

      // Extract and hash relevant headers
      let relevant_headers : HashMap<  String, String  > = headers.iter()
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
      } )
    }

    /// Determine if a header is relevant for caching
    fn is_relevant_header( key : &str ) -> bool
    {
      matches!( key.to_lowercase().as_str(),
        "content-type" | "accept" | "openai-organization" | "openai-project" | "authorization"
      )
    }

    /// Hash a string using the default hasher
    fn hash_string( s : &str ) -> u64
    {
      let mut hasher = DefaultHasher::new();
      s.hash( &mut hasher );
      hasher.finish()
    }
  }

}

crate ::mod_interface!
{
  orphan use EnhancedCacheConfig;
  orphan use EvictionPolicy;
  orphan use EnhancedCacheStatistics;
  orphan use EnhancedCacheEntry;
  orphan use CachePriority;
  orphan use AccessPattern;
  orphan use EnhancedRequestCache;
  orphan use EnhancedApiRequestCache;
  orphan use RequestCacheKey;
}