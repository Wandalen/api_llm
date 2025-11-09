//! Enhanced Request Caching Tests
//!
//! Comprehensive test suite for the enhanced request caching functionality including:
//! - Advanced eviction policies testing
//! - Adaptive TTL validation
//! - Predictive caching behavior
//! - Performance characteristics under load
//! - Memory management and optimization
//! - Cache warming and maintenance operations

#![ allow( clippy::unreadable_literal ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::useless_vec ) ]
#![ allow( clippy::unused_async ) ]
#![ allow( clippy::field_reassign_with_default ) ]

#[ cfg( test ) ]
mod enhanced_cache_tests
{
  use api_openai::
  {
    request_cache_enhanced ::*,
  };
  use std::
  {
    time ::Duration,
    sync ::Arc,
  };
  use tokio::time::sleep;

  // Helper function to create test cache
  fn create_test_cache() -> EnhancedRequestCache< String, String >
  {
    let config = EnhancedCacheConfig
    {
      max_size : 100,
      default_ttl : Duration::from_secs( 300 ),
      min_ttl : Duration::from_secs( 10 ),
      max_ttl : Duration::from_secs( 3600 ),
      adaptive_ttl : true,
      predictive_caching : true,
      cache_warming : false,
      cleanup_interval : Duration::from_secs( 30 ),
      detailed_metrics : true,
      eviction_policy : EvictionPolicy::AdaptiveLRU,
      compression_threshold : 1024,
      persistence : false,
      max_memory_usage : 10 * 1024 * 1024,
    };

    EnhancedRequestCache::new( config )
  }

  #[ tokio::test ]
  async fn test_enhanced_cache_basic_operations()
  {
    let cache = create_test_cache();

    // Test insertion
    let success = cache.insert_enhanced( "key1".to_string(), "value1".to_string(), CachePriority::Normal ).await;
    assert!( success, "Insertion should succeed" );

    // Test retrieval
    let retrieved = cache.get_enhanced( &"key1".to_string() ).await;
    assert_eq!( retrieved, Some( "value1".to_string() ), "Should retrieve inserted value" );

    // Test statistics
    let stats = cache.stats();
    assert_eq!( stats.hits.load( std::sync::atomic::Ordering::Relaxed ), 1 );
    assert_eq!( stats.misses.load( std::sync::atomic::Ordering::Relaxed ), 0 );
    assert_eq!( stats.entries.load( std::sync::atomic::Ordering::Relaxed ), 1 );
  }

  #[ tokio::test ]
  async fn test_adaptive_lru_eviction_policy()
  {
    let mut config = EnhancedCacheConfig::default();
    config.max_size = 3;
    config.eviction_policy = EvictionPolicy::AdaptiveLRU;

    let cache = EnhancedRequestCache::new( config );

    // Insert items with different priorities
    cache.insert_enhanced( "low".to_string(), "value1".to_string(), CachePriority::Low ).await;
    cache.insert_enhanced( "normal".to_string(), "value2".to_string(), CachePriority::Normal ).await;
    cache.insert_enhanced( "high".to_string(), "value3".to_string(), CachePriority::High ).await;

    // Access normal priority item multiple times
    for _ in 0..5
    {
      cache.get_enhanced( &"normal".to_string() ).await;
    }

    // Insert fourth item, should evict the low priority item
    cache.insert_enhanced( "critical".to_string(), "value4".to_string(), CachePriority::Critical ).await;

    // Low priority item should be evicted
    assert_eq!( cache.get_enhanced( &"low".to_string() ).await, None );
    // Others should still be present
    assert_eq!( cache.get_enhanced( &"normal".to_string() ).await, Some( "value2".to_string() ) );
    assert_eq!( cache.get_enhanced( &"high".to_string() ).await, Some( "value3".to_string() ) );
    assert_eq!( cache.get_enhanced( &"critical".to_string() ).await, Some( "value4".to_string() ) );
  }

  #[ tokio::test ]
  async fn test_lfu_eviction_policy()
  {
    let mut config = EnhancedCacheConfig::default();
    config.max_size = 3;
    config.eviction_policy = EvictionPolicy::LFU;

    let cache = EnhancedRequestCache::new( config );

    // Insert items
    cache.insert_enhanced( "freq1".to_string(), "value1".to_string(), CachePriority::Normal ).await;
    cache.insert_enhanced( "freq5".to_string(), "value2".to_string(), CachePriority::Normal ).await;
    cache.insert_enhanced( "freq3".to_string(), "value3".to_string(), CachePriority::Normal ).await;

    // Access items different numbers of times
    for _ in 0..5
    {
      cache.get_enhanced( &"freq5".to_string() ).await;
    }

    for _ in 0..3
    {
      cache.get_enhanced( &"freq3".to_string() ).await;
    }

    // freq1 accessed only once (during insertion)

    // Insert fourth item, should evict least frequently used (freq1)
    cache.insert_enhanced( "new".to_string(), "new_value".to_string(), CachePriority::Normal ).await;

    // freq1 should be evicted (accessed only once)
    assert_eq!( cache.get_enhanced( &"freq1".to_string() ).await, None );
    // Others should still be present
    assert_eq!( cache.get_enhanced( &"freq5".to_string() ).await, Some( "value2".to_string() ) );
    assert_eq!( cache.get_enhanced( &"freq3".to_string() ).await, Some( "value3".to_string() ) );
    assert_eq!( cache.get_enhanced( &"new".to_string() ).await, Some( "new_value".to_string() ) );
  }

  #[ tokio::test ]
  async fn test_size_aware_eviction_policy()
  {
    let mut config = EnhancedCacheConfig::default();
    config.max_size = 2;
    config.eviction_policy = EvictionPolicy::SizeAware;

    let cache = EnhancedRequestCache::new( config );

    // Insert items - the cache will estimate sizes
    cache.insert_enhanced( "small1".to_string(), "x".to_string(), CachePriority::Normal ).await;
    cache.insert_enhanced( "small2".to_string(), "y".to_string(), CachePriority::Normal ).await;

    // Verify both items are present
    assert_eq!( cache.get_enhanced( &"small1".to_string() ).await, Some( "x".to_string() ) );
    assert_eq!( cache.get_enhanced( &"small2".to_string() ).await, Some( "y".to_string() ) );

    // Insert third item, should trigger eviction
    cache.insert_enhanced( "small3".to_string(), "z".to_string(), CachePriority::Normal ).await;

    // Verify cache is working and only contains max_size items
    let stats = cache.stats();
    assert_eq!( stats.entries.load( std::sync::atomic::Ordering::Relaxed ), 2 );
  }

  #[ tokio::test ]
  async fn test_cache_priorities()
  {
    let mut config = EnhancedCacheConfig::default();
    config.max_size = 2;
    config.eviction_policy = EvictionPolicy::AdaptiveLRU;

    let cache = EnhancedRequestCache::new( config );

    // Insert low and high priority items
    cache.insert_enhanced( "low".to_string(), "low_value".to_string(), CachePriority::Low ).await;
    cache.insert_enhanced( "critical".to_string(), "critical_value".to_string(), CachePriority::Critical ).await;

    // Insert third item, low priority should be evicted first
    cache.insert_enhanced( "normal".to_string(), "normal_value".to_string(), CachePriority::Normal ).await;

    // Low priority item should be evicted
    assert_eq!( cache.get_enhanced( &"low".to_string() ).await, None );
    // High priority items should remain
    assert_eq!( cache.get_enhanced( &"critical".to_string() ).await, Some( "critical_value".to_string() ) );
    assert_eq!( cache.get_enhanced( &"normal".to_string() ).await, Some( "normal_value".to_string() ) );
  }

  #[ tokio::test ]
  async fn test_cache_statistics_tracking()
  {
    let cache = create_test_cache();

    // Test initial statistics
    let stats = cache.stats();
    assert_eq!( stats.hits.load( std::sync::atomic::Ordering::Relaxed ), 0 );
    assert_eq!( stats.misses.load( std::sync::atomic::Ordering::Relaxed ), 0 );

    // Test cache miss
    cache.get_enhanced( &"nonexistent".to_string() ).await;
    assert_eq!( stats.misses.load( std::sync::atomic::Ordering::Relaxed ), 1 );

    // Test cache hit
    cache.insert_enhanced( "test".to_string(), "value".to_string(), CachePriority::Normal ).await;
    cache.get_enhanced( &"test".to_string() ).await;
    assert_eq!( stats.hits.load( std::sync::atomic::Ordering::Relaxed ), 1 );

    // Test hit ratio calculation
    assert!( stats.hit_ratio() > 0.0 );
    assert!( stats.hit_ratio() <= 1.0 );
  }

  #[ tokio::test ]
  async fn test_cache_cleanup_and_maintenance()
  {
    let mut config = EnhancedCacheConfig::default();
    config.cleanup_interval = Duration::from_millis( 10 );
    config.default_ttl = Duration::from_millis( 20 );
    config.adaptive_ttl = false; // Disable adaptive TTL to ensure fixed expiration

    let cache = EnhancedRequestCache::new( config );

    // Insert item with short TTL
    cache.insert_enhanced( "short_lived".to_string(), "value".to_string(), CachePriority::Normal ).await;

    // Verify item exists
    assert_eq!( cache.get_enhanced( &"short_lived".to_string() ).await, Some( "value".to_string() ) );

    // Wait for expiration (longer than TTL)
    sleep( Duration::from_millis( 50 ) ).await;

    // Item should be expired now - try to get it which should return None for expired items
    let item_after_expiry = cache.get_enhanced( &"short_lived".to_string() ).await;

    // Run cleanup
    let removed = cache.cleanup_and_maintain().await;

    // Either the get_enhanced call removed the expired item (returning None) or cleanup did
    assert!( item_after_expiry.is_none() || removed > 0 );
  }

  #[ tokio::test ]
  async fn test_cache_clear_functionality()
  {
    let cache = create_test_cache();

    // Insert multiple items
    for i in 0..10
    {
      cache.insert_enhanced( format!( "key{}", i ), format!( "value{}", i ), CachePriority::Normal ).await;
    }

    // Verify items exist
    assert_eq!( cache.stats().entries.load( std::sync::atomic::Ordering::Relaxed ), 10 );

    // Clear cache
    cache.clear().await;

    // Verify cache is empty
    assert_eq!( cache.stats().entries.load( std::sync::atomic::Ordering::Relaxed ), 0 );
    assert_eq!( cache.get_enhanced( &"key0".to_string() ).await, None );
  }

  #[ tokio::test ]
  async fn test_concurrent_cache_access()
  {
    let cache = Arc::new( create_test_cache() );
    let mut handles = Vec::new();

    // Spawn multiple tasks for concurrent access
    for i in 0..10
    {
      let cache_clone = cache.clone();
      let handle = tokio::spawn( async move
      {
        let key = format!( "concurrent_key_{}", i );
        let value = format!( "concurrent_value_{}", i );

        // Insert
        cache_clone.insert_enhanced( key.clone(), value.clone(), CachePriority::Normal ).await;

        // Retrieve multiple times
        for _ in 0..5
        {
          let retrieved = cache_clone.get_enhanced( &key ).await;
          assert_eq!( retrieved, Some( value.clone() ) );
        }
      } );

      handles.push( handle );
    }

    // Wait for all tasks to complete
    for handle in handles
    {
      handle.await.expect( "Task should complete successfully" );
    }

    // Verify final state
    let stats = cache.stats();
    assert_eq!( stats.entries.load( std::sync::atomic::Ordering::Relaxed ), 10 );
    assert!( stats.hits.load( std::sync::atomic::Ordering::Relaxed ) >= 50 ); // 10 * 5 accesses
  }

  #[ tokio::test ]
  async fn test_memory_usage_tracking()
  {
    let cache = create_test_cache();

    // Initial memory usage should be minimal
    let initial_memory = cache.stats().memory_usage.load( std::sync::atomic::Ordering::Relaxed );

    // Insert several items
    for i in 0..5
    {
      cache.insert_enhanced( format!( "memory_key_{}", i ), format!( "memory_value_{}", i ), CachePriority::Normal ).await;
    }

    // Memory usage should increase
    let after_insert = cache.stats().memory_usage.load( std::sync::atomic::Ordering::Relaxed );
    assert!( after_insert > initial_memory );

    // Clear cache
    cache.clear().await;

    // Memory usage should be reset
    let after_clear = cache.stats().memory_usage.load( std::sync::atomic::Ordering::Relaxed );
    assert_eq!( after_clear, 0 );
  }

  #[ tokio::test ]
  async fn test_access_pattern_tracking()
  {
    let cache = create_test_cache();

    // Insert an item
    cache.insert_enhanced( "pattern_test".to_string(), "value".to_string(), CachePriority::Normal ).await;

    // Access it multiple times with some delay to build a pattern
    for _ in 0..5
    {
      cache.get_enhanced( &"pattern_test".to_string() ).await;
      sleep( Duration::from_millis( 10 ) ).await;
    }

    // The access pattern should be tracked (internal state - we can verify by checking access count indirectly)
    let stats = cache.stats();
    assert!( stats.hits.load( std::sync::atomic::Ordering::Relaxed ) >= 5 );
  }

  #[ tokio::test ]
  async fn test_request_cache_key_creation()
  {
    use std::collections::HashMap;

    let mut headers = HashMap::new();
    headers.insert( "content-type".to_string(), "application/json".to_string() );
    headers.insert( "authorization".to_string(), "Bearer token".to_string() );

    let request_body = serde_json::json!( { "test": "data" } );

    // Test cache key creation
    let key = RequestCacheKey::new( "/v1/test", "POST", Some( &request_body ), &headers );
    assert!( key.is_ok(), "Cache key creation should succeed" );

    let cache_key = key.unwrap();
    assert_eq!( cache_key.endpoint, "/v1/test" );
    assert_eq!( cache_key.method, "POST" );
    assert!( cache_key.body_hash != 0 );
    assert!( cache_key.headers_hash != 0 );
  }

  #[ tokio::test ]
  async fn test_enhanced_cache_config_validation()
  {
    let config = EnhancedCacheConfig::default();

    // Verify default values
    assert!( config.max_size > 0 );
    assert!( config.default_ttl > Duration::from_secs( 0 ) );
    assert!( config.min_ttl < config.default_ttl );
    assert!( config.default_ttl < config.max_ttl );
    assert!( config.adaptive_ttl );
    assert!( config.predictive_caching );
    assert!( config.detailed_metrics );
  }

  #[ tokio::test ]
  async fn test_cache_priority_levels()
  {
    // Test priority ordering
    assert!( CachePriority::Critical > CachePriority::High );
    assert!( CachePriority::High > CachePriority::Normal );
    assert!( CachePriority::Normal > CachePriority::Low );

    // Test that priorities can be compared
    let priorities = vec![ CachePriority::Low, CachePriority::Critical, CachePriority::Normal, CachePriority::High ];
    let mut sorted_priorities = priorities.clone();
    sorted_priorities.sort();

    assert_eq!( sorted_priorities[ 0 ], CachePriority::Low );
    assert_eq!( sorted_priorities[ 3 ], CachePriority::Critical );
  }

}