//! Cache Implementation
//!
//! Provides in-memory caching with TTL, size limits, and statistics.
//!
//! ## Features
//!
//! - TTL-based expiration
//! - Size-limited with LRU eviction
//! - Thread-safe concurrent access
//! - Hit/miss statistics
//!
//! ## Usage
//!
//! ```no_run
//! # use api_huggingface::cache::{Cache, CacheConfig};
//! # use std::time::Duration;
//! # async fn example( ) -> Result< ( ), Box< dyn std::error::Error > > {
//! let cache = Cache::new( CacheConfig {
//!   max_entries : 100,
//!   default_ttl : Some( Duration::from_secs( 60 )),
//! } );
//!
//! cache.insert( "key".to_string( ), "value".to_string( ), None ).await;
//! let value = cache.get( &"key".to_string( )).await;
//! # Ok( ( ))
//! # }
//! ```

use core::time::Duration;
use core::hash::Hash;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

/// Cache entry with TTL
#[ derive( Debug, Clone ) ]
struct CacheEntry< V > 
{
  value : V,
  #[ allow( dead_code ) ] // Reserved for future entry age tracking
  inserted_at : Instant,
  expires_at : Option< Instant >,
  last_accessed : Instant,
}

impl< V > CacheEntry< V > 
{
  /// Create new cache entry
  #[ inline ]
  fn new( value : V, ttl : Option< Duration > ) -> Self 
  {
  let now = Instant::now( );
  Self {
      value,
      inserted_at : now,
      expires_at : ttl.map( |d| now + d ),
      last_accessed : now,
  }
  }

  /// Check if entry is expired
  #[ inline ]
  fn is_expired( &self ) -> bool 
  {
  self.expires_at.is_some_and( |exp| Instant::now( ) >= exp )
  }

  /// Update last accessed time
  #[ inline ]
  fn touch( &mut self ) 
  {
  self.last_accessed = Instant::now( );
  }
}

/// Cache configuration
#[ derive( Debug, Clone ) ]
pub struct CacheConfig 
{
  /// Maximum number of entries
  pub max_entries : usize,
  /// Default TTL for entries ( None = no expiration )
  pub default_ttl : Option< Duration >,
}

impl Default for CacheConfig 
{
  #[ inline ]
  fn default() -> Self 
  {
  Self {
      max_entries : 1000,
      default_ttl : Some( Duration::from_secs( 300 )), // 5 minutes
  }
  }
}

/// Cache statistics
#[ derive( Debug, Clone, Copy, Default ) ]
pub struct CacheStats 
{
  /// Number of cache hits
  pub hits : u64,
  /// Number of cache misses
  pub misses : u64,
  /// Number of entries evicted
  pub evictions : u64,
  /// Current number of entries
  pub entries : usize,
}

impl CacheStats 
{
  /// Calculate hit rate ( 0.0 - 1.0 )
  #[ inline ]
  #[ must_use ]
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

  /// Get total requests
  #[ inline ]
  #[ must_use ]
  pub fn total_requests( &self ) -> u64 
  {
  self.hits + self.misses
  }
}

/// Internal cache state
struct CacheState< K, V > 
{
  entries : HashMap< K, CacheEntry< V > >,
  config : CacheConfig,
  stats : CacheStats,
}

/// In-memory cache with TTL and size limits
#[ derive( Clone ) ]
pub struct Cache< K, V > 
{
  state : Arc< RwLock< CacheState< K, V > > >,
}

impl< K, V > Cache< K, V >
where
  K : Eq + Hash + Clone,
  V : Clone,
{
  /// Create new cache with given configuration
  #[ inline ]
  #[ must_use ]
  pub fn new( config : CacheConfig ) -> Self 
  {
  Self {
      state : Arc::new( RwLock::new( CacheState {
  entries : HashMap::new( ),
  config,
  stats : CacheStats::default( ),
      } )),
  }
  }

  /// Insert value into cache
  ///
  /// If `ttl` is None, uses default TTL from config.
  #[ inline ]
  pub async fn insert( &self, key : K, value : V, ttl : Option< Duration > ) 
  {
  let mut state = self.state.write( ).await;

  // Determine TTL to use
  let effective_ttl = ttl.or( state.config.default_ttl );

  // Check if we need to evict
  if state.entries.len( ) >= state.config.max_entries && !state.entries.contains_key( &key )
  {
      // Evict LRU entry
      let lru_key = state.entries.iter( )
  .min_by_key( |( _, entry )| entry.last_accessed )
  .map( |( k, _ )| k.clone( ));

      if let Some( lru_key ) = lru_key
      {
  state.entries.remove( &lru_key );
  state.stats.evictions += 1;
      }
  }

  // Insert new entry
  let entry = CacheEntry::new( value, effective_ttl );
  state.entries.insert( key, entry );
  state.stats.entries = state.entries.len( );
  }

  /// Get value from cache
  ///
  /// Returns None if key doesn't exist or entry is expired.
  #[ inline ]
  pub async fn get( &self, key : &K ) -> Option< V > 
  {
  let mut state = self.state.write( ).await;

  if let Some( entry ) = state.entries.get_mut( key )
  {
      if entry.is_expired( )
      {
  // Remove expired entry
  state.entries.remove( key );
  state.stats.entries = state.entries.len( );
  state.stats.misses += 1;
  None
      } else {
  // Update access time and clone value
  entry.touch( );
  let value = entry.value.clone( );
  state.stats.hits += 1;
  Some( value )
      }
  } else {
      state.stats.misses += 1;
      None
  }
  }

  /// Check if key exists in cache ( without updating access time )
  #[ inline ]
  pub async fn contains_key( &self, key : &K ) -> bool 
  {
  let state = self.state.read( ).await;
  state.entries.get( key )
      .is_some_and( |entry| !entry.is_expired( ))
  }

  /// Remove entry from cache
  #[ inline ]
  pub async fn remove( &self, key : &K ) -> Option< V > 
  {
  let mut state = self.state.write( ).await;
  let value = state.entries.remove( key ).map( |entry| entry.value );
  state.stats.entries = state.entries.len( );
  value
  }

  /// Clear all entries from cache
  #[ inline ]
  pub async fn clear( &self ) 
  {
  let mut state = self.state.write( ).await;
  state.entries.clear( );
  state.stats.entries = 0;
  }

  /// Remove expired entries
  #[ inline ]
  pub async fn cleanup_expired( &self ) -> usize 
  {
  let mut state = self.state.write( ).await;
  let before = state.entries.len( );

  state.entries.retain( |_, entry| !entry.is_expired( ));

  let removed = before - state.entries.len( );
  state.stats.entries = state.entries.len( );
  removed
  }

  /// Get cache statistics
  #[ inline ]
  pub async fn stats( &self ) -> CacheStats 
  {
  let state = self.state.read( ).await;
  state.stats
  }

  /// Reset statistics
  #[ inline ]
  pub async fn reset_stats( &self ) 
  {
  let mut state = self.state.write( ).await;
  state.stats = CacheStats {
      entries : state.entries.len( ),
      ..Default::default( )
  };
  }

  /// Get current cache size
  #[ inline ]
  pub async fn len( &self ) -> usize 
  {
  let state = self.state.read( ).await;
  state.entries.len( )
  }

  /// Check if cache is empty
  #[ inline ]
  pub async fn is_empty( &self ) -> bool 
  {
  let state = self.state.read( ).await;
  state.entries.is_empty( )
  }

  /// Get cache configuration
  #[ inline ]
  pub async fn config( &self ) -> CacheConfig 
  {
  let state = self.state.read( ).await;
  state.config.clone( )
  }
}

impl< K, V > core::fmt::Debug for Cache< K, V > 
{
  #[ inline ]
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result 
  {
  f.debug_struct( "Cache" )
      .field( "state", &"< CacheState >" )
      .finish( )
  }
}

/// Cache errors
#[ derive( Debug ) ]
pub enum CacheError 
{
  /// Cache is full and cannot accept new entries
  CacheFull,
  /// Entry not found in cache
  NotFound,
}

impl core::fmt::Display for CacheError 
{
  #[ inline ]
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result 
  {
  match self
  {
      Self::CacheFull => write!( f, "Cache is full" ),
      Self::NotFound => write!( f, "Entry not found in cache" ),
  }
  }
}

impl std::error::Error for CacheError {}

#[ cfg( test ) ]
mod tests {
  use super::*;

  #[ tokio::test ]
  async fn test_cache_insert_and_get() 
  {
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", None ).await;

  let value = cache.get( &"key1" ).await;
  assert_eq!( value, Some( "value1" ));
  }

  #[ tokio::test ]
  async fn test_cache_miss() 
  {
  let cache : Cache< &str, &str > = Cache::new( CacheConfig::default( ));

  let value = cache.get( &"nonexistent" ).await;
  assert_eq!( value, None );
  }

  #[ tokio::test ]
  async fn test_cache_ttl_expiration() 
  {
  let cache = Cache::new( CacheConfig::default( ));

  // Insert with short TTL
  cache.insert( "key1", "value1", Some( Duration::from_millis( 50 )) ).await;

  // Should exist immediately
  assert!( cache.get( &"key1" ).await.is_some( ));

  // Wait for expiration
  tokio::time::sleep( Duration::from_millis( 100 )).await;

  // Should be expired
  assert!( cache.get( &"key1" ).await.is_none( ));
  }

  #[ tokio::test ]
  async fn test_cache_size_limit() 
  {
  let config = CacheConfig {
      max_entries : 3,
      default_ttl : None,
  };
  let cache = Cache::new( config );

  // Insert 3 entries
  cache.insert( "key1", "value1", None ).await;
  cache.insert( "key2", "value2", None ).await;
  cache.insert( "key3", "value3", None ).await;

  assert_eq!( cache.len( ).await, 3 );

  // Insert 4th entry - should evict LRU
  cache.insert( "key4", "value4", None ).await;

  assert_eq!( cache.len( ).await, 3 );

  let stats = cache.stats( ).await;
  assert_eq!( stats.evictions, 1 );
  }

  #[ tokio::test ]
  async fn test_cache_remove() 
  {
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", None ).await;
  assert!( cache.contains_key( &"key1" ).await );

  let removed = cache.remove( &"key1" ).await;
  assert_eq!( removed, Some( "value1" ));
  assert!( !cache.contains_key( &"key1" ).await );
  }

  #[ tokio::test ]
  async fn test_cache_clear() 
  {
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", None ).await;
  cache.insert( "key2", "value2", None ).await;

  assert_eq!( cache.len( ).await, 2 );

  cache.clear( ).await;

  assert_eq!( cache.len( ).await, 0 );
  assert!( cache.is_empty( ).await );
  }

  #[ tokio::test ]
  #[ allow( clippy::float_cmp ) ]
  async fn test_cache_stats() 
  {
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", None ).await;

  // Hit
  cache.get( &"key1" ).await;

  // Miss
  cache.get( &"key2" ).await;

  let stats = cache.stats( ).await;
  assert_eq!( stats.hits, 1 );
  assert_eq!( stats.misses, 1 );
  assert_eq!( stats.hit_rate( ), 0.5 );
  }

  #[ tokio::test ]
  async fn test_cleanup_expired() 
  {
  let cache = Cache::new( CacheConfig::default( ));

  // Insert with different TTLs
  cache.insert( "key1", "value1", Some( Duration::from_millis( 50 )) ).await;
  cache.insert( "key2", "value2", None ).await;

  // Wait for first to expire
  tokio::time::sleep( Duration::from_millis( 100 )).await;

  let removed = cache.cleanup_expired( ).await;
  assert_eq!( removed, 1 );
  assert_eq!( cache.len( ).await, 1 );
  }

  #[ tokio::test ]
  async fn test_contains_key() 
  {
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", None ).await;

  assert!( cache.contains_key( &"key1" ).await );
  assert!( !cache.contains_key( &"key2" ).await );
  }

  #[ tokio::test ]
  async fn test_reset_stats() 
  {
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", None ).await;
  cache.get( &"key1" ).await;

  let stats1 = cache.stats( ).await;
  assert_eq!( stats1.hits, 1 );

  cache.reset_stats( ).await;

  let stats2 = cache.stats( ).await;
  assert_eq!( stats2.hits, 0 );
  assert_eq!( stats2.misses, 0 );
  }

  #[ tokio::test ]
  async fn test_default_config() 
  {
  let config = CacheConfig::default( );
  assert_eq!( config.max_entries, 1000 );
  assert_eq!( config.default_ttl, Some( Duration::from_secs( 300 )) );
  }

  #[ tokio::test ]
  async fn test_lru_eviction() 
  {
  let config = CacheConfig {
      max_entries : 2,
      default_ttl : None,
  };
  let cache = Cache::new( config );

  cache.insert( "key1", "value1", None ).await;
  cache.insert( "key2", "value2", None ).await;

  // Access key1 to make it more recently used
  cache.get( &"key1" ).await;

  // Insert key3 - should evict key2 ( LRU )
  cache.insert( "key3", "value3", None ).await;

  assert!( cache.contains_key( &"key1" ).await );
  assert!( !cache.contains_key( &"key2" ).await );
  assert!( cache.contains_key( &"key3" ).await );
  }

  #[ tokio::test ]
  async fn test_cache_with_numbers() 
  {
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( 1, 100, None ).await;
  cache.insert( 2, 200, None ).await;

  assert_eq!( cache.get( &1 ).await, Some( 100 ));
  assert_eq!( cache.get( &2 ).await, Some( 200 ));
  }
}
