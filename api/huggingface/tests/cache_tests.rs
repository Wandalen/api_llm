//! Integration tests for Content Caching
//!
//! These tests verify the caching functionality with various scenarios.
//!
//! ## Test Coverage
//!
//! - Basic insert/get operations
//! - TTL expiration
//! - Size limits and eviction
//! - LRU eviction behavior

#![ allow( clippy::float_cmp ) ]
#![ allow( clippy::uninlined_format_args ) ]
//! - Statistics tracking
//! - Concurrent access
//! - Cache cleanup
//! - Edge cases

use api_huggingface::cache::{Cache, CacheConfig};
use core::time::Duration;
use std::sync::Arc;

// ============================================================================
// Basic Operations
// ============================================================================

#[ tokio::test ]
async fn test_basic_insert_and_get() 
{
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", None ).await;
  cache.insert( "key2", "value2", None ).await;

  assert_eq!( cache.get( &"key1" ).await, Some( "value1" ));
  assert_eq!( cache.get( &"key2" ).await, Some( "value2" ));
}

#[ tokio::test ]
async fn test_cache_miss() 
{
  let cache : Cache< &str, &str > = Cache::new( CacheConfig::default( ));

  let value = cache.get( &"nonexistent" ).await;
  assert_eq!( value, None );
}

#[ tokio::test ]
async fn test_update_existing_key() 
{
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", None ).await;
  assert_eq!( cache.get( &"key1" ).await, Some( "value1" ));

  cache.insert( "key1", "value2", None ).await;
  assert_eq!( cache.get( &"key1" ).await, Some( "value2" ));
}

#[ tokio::test ]
async fn test_cache_with_different_types() 
{
  let cache : Cache< i32, String > = Cache::new( CacheConfig::default( ));

  cache.insert( 1, "one".to_string( ), None ).await;
  cache.insert( 2, "two".to_string( ), None ).await;

  assert_eq!( cache.get( &1 ).await, Some( "one".to_string( )) );
  assert_eq!( cache.get( &2 ).await, Some( "two".to_string( )) );
}

// ============================================================================
// TTL Tests
// ============================================================================

#[ tokio::test ]
async fn test_ttl_expiration() 
{
  let cache = Cache::new( CacheConfig::default( ));

  // Insert with short TTL
  cache.insert( "key1", "value1", Some( Duration::from_millis( 100 )) ).await;

  // Should exist immediately
  assert_eq!( cache.get( &"key1" ).await, Some( "value1" ));

  // Wait for expiration
  tokio::time::sleep( Duration::from_millis( 150 )).await;

  // Should be expired
  assert_eq!( cache.get( &"key1" ).await, None );
}

#[ tokio::test ]
async fn test_default_ttl() 
{
  let config = CacheConfig {
  max_entries : 100,
  default_ttl : Some( Duration::from_millis( 100 )),
  };
  let cache = Cache::new( config );

  // Insert without specifying TTL
  cache.insert( "key1", "value1", None ).await;

  // Should use default TTL
  tokio::time::sleep( Duration::from_millis( 150 )).await;

  assert_eq!( cache.get( &"key1" ).await, None );
}

#[ tokio::test ]
async fn test_no_ttl_never_expires() 
{
  let config = CacheConfig {
  max_entries : 100,
  default_ttl : None,
  };
  let cache = Cache::new( config );

  cache.insert( "key1", "value1", None ).await;

  // Wait some time
  tokio::time::sleep( Duration::from_millis( 100 )).await;

  // Should still exist
  assert_eq!( cache.get( &"key1" ).await, Some( "value1" ));
}

#[ tokio::test ]
async fn test_override_default_ttl() 
{
  let config = CacheConfig {
  max_entries : 100,
  default_ttl : Some( Duration::from_secs( 300 )),
  };
  let cache = Cache::new( config );

  // Override with shorter TTL
  cache.insert( "key1", "value1", Some( Duration::from_millis( 50 )) ).await;

  tokio::time::sleep( Duration::from_millis( 100 )).await;

  // Should be expired ( using override, not default )
  assert_eq!( cache.get( &"key1" ).await, None );
}

// ============================================================================
// Size Limit Tests
// ============================================================================

#[ tokio::test ]
async fn test_size_limit_enforcement() 
{
  let config = CacheConfig {
  max_entries : 3,
  default_ttl : None,
  };
  let cache = Cache::new( config );

  cache.insert( "key1", "value1", None ).await;
  cache.insert( "key2", "value2", None ).await;
  cache.insert( "key3", "value3", None ).await;

  assert_eq!( cache.len( ).await, 3 );

  // Fourth insert should trigger eviction
  cache.insert( "key4", "value4", None ).await;

  assert_eq!( cache.len( ).await, 3 );
}

#[ tokio::test ]
async fn test_lru_eviction() 
{
  let config = CacheConfig {
  max_entries : 3,
  default_ttl : None,
  };
  let cache = Cache::new( config );

  cache.insert( "key1", "value1", None ).await;
  tokio::time::sleep( Duration::from_millis( 10 )).await;

  cache.insert( "key2", "value2", None ).await;
  tokio::time::sleep( Duration::from_millis( 10 )).await;

  cache.insert( "key3", "value3", None ).await;
  tokio::time::sleep( Duration::from_millis( 10 )).await;

  // Access key1 to make it recently used
  cache.get( &"key1" ).await;
  tokio::time::sleep( Duration::from_millis( 10 )).await;

  // Insert key4 - should evict key2 ( least recently used )
  cache.insert( "key4", "value4", None ).await;

  assert!( cache.contains_key( &"key1" ).await );
  assert!( !cache.contains_key( &"key2" ).await );
  assert!( cache.contains_key( &"key3" ).await );
  assert!( cache.contains_key( &"key4" ).await );
}

#[ tokio::test ]
async fn test_update_doesnt_evict() 
{
  let config = CacheConfig {
  max_entries : 2,
  default_ttl : None,
  };
  let cache = Cache::new( config );

  cache.insert( "key1", "value1", None ).await;
  cache.insert( "key2", "value2", None ).await;

  // Update existing key - should not trigger eviction
  cache.insert( "key1", "value1_updated", None ).await;

  assert_eq!( cache.len( ).await, 2 );
  assert_eq!( cache.get( &"key1" ).await, Some( "value1_updated" ));
}

// ============================================================================
// Statistics Tests
// ============================================================================

#[ tokio::test ]
async fn test_hit_statistics() 
{
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", None ).await;

  // Hit
  cache.get( &"key1" ).await;
  cache.get( &"key1" ).await;

  let stats = cache.stats( ).await;
  assert_eq!( stats.hits, 2 );
  assert_eq!( stats.misses, 0 );
}

#[ tokio::test ]
async fn test_miss_statistics() 
{
  let cache : Cache< &str, &str > = Cache::new( CacheConfig::default( ));

  // Miss
  cache.get( &"nonexistent" ).await;
  cache.get( &"another" ).await;

  let stats = cache.stats( ).await;
  assert_eq!( stats.hits, 0 );
  assert_eq!( stats.misses, 2 );
}

#[ tokio::test ]
async fn test_hit_rate_calculation() 
{
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", None ).await;

  // 3 hits
  cache.get( &"key1" ).await;
  cache.get( &"key1" ).await;
  cache.get( &"key1" ).await;

  // 1 miss
  cache.get( &"nonexistent" ).await;

  let stats = cache.stats( ).await;
  assert_eq!( stats.hit_rate( ), 0.75 ); // 3/4 = 0.75
}

#[ tokio::test ]
async fn test_eviction_statistics() 
{
  let config = CacheConfig {
  max_entries : 2,
  default_ttl : None,
  };
  let cache = Cache::new( config );

  cache.insert( "key1", "value1", None ).await;
  cache.insert( "key2", "value2", None ).await;

  // This should evict one entry
  cache.insert( "key3", "value3", None ).await;

  let stats = cache.stats( ).await;
  assert_eq!( stats.evictions, 1 );
}

#[ tokio::test ]
async fn test_reset_statistics() 
{
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", None ).await;
  cache.get( &"key1" ).await;
  cache.get( &"nonexistent" ).await;

  let stats1 = cache.stats( ).await;
  assert_eq!( stats1.hits, 1 );
  assert_eq!( stats1.misses, 1 );

  cache.reset_stats( ).await;

  let stats2 = cache.stats( ).await;
  assert_eq!( stats2.hits, 0 );
  assert_eq!( stats2.misses, 0 );
  assert_eq!( stats2.entries, 1 ); // Entry count preserved
}

// ============================================================================
// Cleanup Tests
// ============================================================================

#[ tokio::test ]
async fn test_cleanup_expired_entries() 
{
  let cache = Cache::new( CacheConfig::default( ));

  // Insert with different TTLs
  cache.insert( "key1", "value1", Some( Duration::from_millis( 50 )) ).await;
  cache.insert( "key2", "value2", Some( Duration::from_millis( 200 )) ).await;
  cache.insert( "key3", "value3", None ).await;

  // Wait for key1 to expire
  tokio::time::sleep( Duration::from_millis( 100 )).await;

  let removed = cache.cleanup_expired( ).await;
  assert_eq!( removed, 1 );
  assert_eq!( cache.len( ).await, 2 );

  // key1 should be gone
  assert!( !cache.contains_key( &"key1" ).await );
  assert!( cache.contains_key( &"key2" ).await );
  assert!( cache.contains_key( &"key3" ).await );
}

#[ tokio::test ]
async fn test_clear_all_entries() 
{
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", None ).await;
  cache.insert( "key2", "value2", None ).await;
  cache.insert( "key3", "value3", None ).await;

  assert_eq!( cache.len( ).await, 3 );

  cache.clear( ).await;

  assert_eq!( cache.len( ).await, 0 );
  assert!( cache.is_empty( ).await );
}

// ============================================================================
// Remove Tests
// ============================================================================

#[ tokio::test ]
async fn test_remove_existing_key() 
{
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", None ).await;

  let removed = cache.remove( &"key1" ).await;
  assert_eq!( removed, Some( "value1" ));
  assert!( !cache.contains_key( &"key1" ).await );
}

#[ tokio::test ]
async fn test_remove_nonexistent_key() 
{
  let cache : Cache< &str, &str > = Cache::new( CacheConfig::default( ));

  let removed = cache.remove( &"nonexistent" ).await;
  assert_eq!( removed, None );
}

// ============================================================================
// Concurrent Access Tests
// ============================================================================

#[ tokio::test ]
async fn test_concurrent_inserts() 
{
  let cache = Arc::new( Cache::new( CacheConfig::default( )) );

  let mut handles = vec![ ];

  for i in 0..10
  {
  let cache_clone = cache.clone( );
  let handle = tokio::spawn( async move {
      cache_clone.insert( i, i * 10, None ).await;
  } );
  handles.push( handle );
  }

  for handle in handles
  {
  handle.await.unwrap( );
  }

  assert_eq!( cache.len( ).await, 10 );
}

#[ tokio::test ]
async fn test_concurrent_reads() 
{
  let cache = Arc::new( Cache::new( CacheConfig::default( )) );

  // Pre-populate
  for i in 0..10
  {
  cache.insert( i, i * 10, None ).await;
  }

  let mut handles = vec![ ];

  for i in 0..10
  {
  let cache_clone = cache.clone( );
  let handle = tokio::spawn( async move {
      cache_clone.get( &i ).await
  } );
  handles.push( handle );
  }

  for ( i, handle ) in handles.into_iter( ).enumerate( )
  {
  let value = handle.await.unwrap( );
  assert_eq!( value, Some( i * 10 ));
  }
}

#[ tokio::test ]
async fn test_concurrent_mixed_operations() 
{
  let cache = Arc::new( Cache::new( CacheConfig::default( )) );

  let mut handles = vec![ ];

  // Inserts
  for i in 0..5
  {
  let cache_clone = cache.clone( );
  let handle = tokio::spawn( async move {
      cache_clone.insert( i, i * 10, None ).await;
  } );
  handles.push( handle );
  }

  // Reads
  for i in 0..5
  {
  let cache_clone = cache.clone( );
  let handle = tokio::spawn( async move {
      cache_clone.get( &i ).await;
  } );
  handles.push( handle );
  }

  for handle in handles
  {
  handle.await.unwrap( );
  }

  // All operations should complete without deadlock
}

// ============================================================================
// Edge Cases
// ============================================================================

#[ tokio::test ]
async fn test_empty_cache() 
{
  let cache : Cache< &str, &str > = Cache::new( CacheConfig::default( ));

  assert!( cache.is_empty( ).await );
  assert_eq!( cache.len( ).await, 0 );

  let stats = cache.stats( ).await;
  assert_eq!( stats.entries, 0 );
}

#[ tokio::test ]
async fn test_single_entry_cache() 
{
  let config = CacheConfig {
  max_entries : 1,
  default_ttl : None,
  };
  let cache = Cache::new( config );

  cache.insert( "key1", "value1", None ).await;
  assert_eq!( cache.len( ).await, 1 );

  // Insert second entry - should evict first
  cache.insert( "key2", "value2", None ).await;
  assert_eq!( cache.len( ).await, 1 );
  assert!( !cache.contains_key( &"key1" ).await );
  assert!( cache.contains_key( &"key2" ).await );
}

#[ tokio::test ]
async fn test_zero_ttl() 
{
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", Some( Duration::from_millis( 0 )) ).await;

  // Should expire immediately
  assert_eq!( cache.get( &"key1" ).await, None );
}

#[ tokio::test ]
async fn test_contains_key_doesnt_update_stats() 
{
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", None ).await;

  // contains_key shouldn't affect stats
  cache.contains_key( &"key1" ).await;
  cache.contains_key( &"key2" ).await;

  let stats = cache.stats( ).await;
  assert_eq!( stats.hits, 0 );
  assert_eq!( stats.misses, 0 );
}

#[ tokio::test ]
async fn test_expired_entry_counts_as_miss() 
{
  let cache = Cache::new( CacheConfig::default( ));

  cache.insert( "key1", "value1", Some( Duration::from_millis( 50 )) ).await;

  tokio::time::sleep( Duration::from_millis( 100 )).await;

  // Accessing expired entry should count as miss
  cache.get( &"key1" ).await;

  let stats = cache.stats( ).await;
  assert_eq!( stats.hits, 0 );
  assert_eq!( stats.misses, 1 );
}

// ============================================================================
// Configuration Tests
// ============================================================================

#[ tokio::test ]
async fn test_default_configuration() 
{
  let config = CacheConfig::default( );

  assert_eq!( config.max_entries, 1000 );
  assert_eq!( config.default_ttl, Some( Duration::from_secs( 300 )) );
}

#[ tokio::test ]
async fn test_get_cache_config() 
{
  let config = CacheConfig {
  max_entries : 500,
  default_ttl : Some( Duration::from_secs( 600 )),
  };

  let cache : Cache< String, String > = Cache::new( config.clone( ));

  let retrieved_config = cache.config( ).await;
  assert_eq!( retrieved_config.max_entries, 500 );
  assert_eq!( retrieved_config.default_ttl, Some( Duration::from_secs( 600 )) );
}

// ============================================================================
// Real-World Scenarios
// ============================================================================

#[ tokio::test ]
async fn test_response_caching_scenario() 
{
  let cache : Cache< String, Vec< u8 > > = Cache::new( CacheConfig {
  max_entries : 100,
  default_ttl : Some( Duration::from_secs( 60 )),
  } );

  // Simulate caching API responses
  let request_key = "model:text-generation,input:hello".to_string( );
  let response_data = vec![1, 2, 3, 4, 5 ];

  cache.insert( request_key.clone( ), response_data.clone( ), None ).await;

  // Retrieve from cache
  let cached_response = cache.get( &request_key ).await;
  assert_eq!( cached_response, Some( response_data ));
}

#[ tokio::test ]
async fn test_high_throughput_scenario() 
{
  let cache = Arc::new( Cache::new( CacheConfig {
  max_entries : 1000,
  default_ttl : Some( Duration::from_secs( 300 )),
  } ));

  let mut handles = vec![ ];

  // Simulate 100 concurrent requests
  for i in 0..100
  {
  let cache_clone = cache.clone( );
  let handle = tokio::spawn( async move {
      let key = format!( "request-{}", i );
      let value = format!( "response-{}", i );

      // Insert
      cache_clone.insert( key.clone( ), value.clone( ), None ).await;

      // Immediate read
      cache_clone.get( &key ).await
  } );
  handles.push( handle );
  }

  let mut successes = 0;
  for handle in handles
  {
  if handle.await.unwrap( ).is_some( )
  {
      successes += 1;
  }
  }

  assert_eq!( successes, 100 );
}
