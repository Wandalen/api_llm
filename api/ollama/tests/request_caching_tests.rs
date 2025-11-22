//! Request caching functionality tests for `api_ollama` crate.
//!
//! These tests verify the request caching implementation that provides
//! performance optimization through intelligent caching with TTL management,
//! LRU eviction policies, and cache invalidation strategies.

#![ cfg( feature = "request_caching" ) ]

use api_ollama::{ OllamaClient, RequestCache, RequestCacheConfig };
use api_ollama::{ ChatRequest, ChatMessage, MessageRole, GenerateRequest };
use core::time::Duration;
use std::sync::Arc;
use tokio::time::sleep;

#[ tokio::test ]
async fn test_cache_config_creation_and_configuration()
{
  // Test creating a cache config with default settings
  let config = RequestCacheConfig::default();
  assert_eq!(config.max_entries(), 100);
  assert_eq!(config.default_ttl(), Duration::from_secs(300)); // 5 minutes
  assert_eq!(config.cleanup_interval(), Duration::from_secs(60));

  // Test creating a cache config with custom settings
  let custom_config = RequestCacheConfig::new()
    .with_max_entries(500)
    .with_default_ttl(Duration::from_secs(600))
    .with_cleanup_interval(Duration::from_secs(30));

  assert_eq!(custom_config.max_entries(), 500);
  assert_eq!(custom_config.default_ttl(), Duration::from_secs(600));
  assert_eq!(custom_config.cleanup_interval(), Duration::from_secs(30));
}

#[ tokio::test ]
async fn test_cache_creation_and_initial_state()
{
  let config = RequestCacheConfig::default();
  let cache = RequestCache::new(config);

  // Cache should start empty
  assert_eq!(cache.len(), 0);
  assert_eq!(cache.capacity(), 100);
  assert!(cache.is_empty());

  let stats = cache.stats();
  assert_eq!(stats.hits, 0);
  assert_eq!(stats.misses, 0);
  assert_eq!(stats.evictions, 0);
}

#[ tokio::test ]
async fn test_cache_entry_operations()
{
  let config = RequestCacheConfig::new().with_max_entries(3);
  let cache = RequestCache::new(config);

  let key1 = "test_key_1".to_string();
  let value1 = "test_value_1".to_string();
  let key2 = "test_key_2".to_string();
  let value2 = "test_value_2".to_string();

  // Test inserting entries
  cache.insert(key1.clone(), value1.clone(), None);
  cache.insert(key2.clone(), value2.clone(), None);

  assert_eq!(cache.len(), 2);
  assert!(!cache.is_empty());

  // Test retrieving entries
  assert_eq!(cache.get(&key1), Some(value1.clone()));
  assert_eq!(cache.get(&key2), Some(value2.clone()));
  assert_eq!(cache.get("nonexistent"), None);

  // Test contains
  assert!(cache.contains_key(&key1));
  assert!(cache.contains_key(&key2));
  assert!(!cache.contains_key("nonexistent"));
}

#[ tokio::test ]
async fn test_cache_ttl_expiration()
{
  let config = RequestCacheConfig::new().with_default_ttl(Duration::from_millis(100));
  let cache = RequestCache::new(config);

  let key = "test_key".to_string();
  let value = "test_value".to_string();

  // Insert entry with short TTL
  cache.insert(key.clone(), value.clone(), Some(Duration::from_millis(50)));

  // Entry should be accessible immediately
  assert_eq!(cache.get(&key), Some(value.clone()));

  // Wait for TTL to expire
  sleep(Duration::from_millis(60)).await;

  // Entry should be expired and return None
  assert_eq!(cache.get(&key), None);
  assert!(!cache.contains_key(&key));

  // Cache should automatically clean up expired entries
  assert_eq!(cache.len(), 0);
}

#[ tokio::test ]
async fn test_lru_eviction_policy()
{
  let config = RequestCacheConfig::new().with_max_entries(3);
  let cache = RequestCache::new(config);

  // Fill cache to capacity
  cache.insert("key1".to_string(), "value1".to_string(), None);
  cache.insert("key2".to_string(), "value2".to_string(), None);
  cache.insert("key3".to_string(), "value3".to_string(), None);

  assert_eq!(cache.len(), 3);

  // Access key1 to make it most recently used
  let _ = cache.get("key1");

  // Insert new entry - should evict least recently used (key2)
  cache.insert("key4".to_string(), "value4".to_string(), None);

  assert_eq!(cache.len(), 3);
  assert!(cache.contains_key("key1")); // Should still be present (recently accessed)
  assert!(!cache.contains_key("key2")); // Should be evicted (least recently used)
  assert!(cache.contains_key("key3")); // Should still be present
  assert!(cache.contains_key("key4")); // Should be present (newly inserted)

  let stats = cache.stats();
  assert_eq!(stats.evictions, 1);
}

#[ tokio::test ]
async fn test_cache_invalidation()
{
  let config = RequestCacheConfig::default();
  let cache = RequestCache::new(config);

  // Insert multiple entries
  cache.insert("key1".to_string(), "value1".to_string(), None);
  cache.insert("key2".to_string(), "value2".to_string(), None);
  cache.insert("key3".to_string(), "value3".to_string(), None);

  assert_eq!(cache.len(), 3);

  // Test individual key invalidation
  cache.invalidate("key2");
  assert_eq!(cache.len(), 2);
  assert!(!cache.contains_key("key2"));

  // Test pattern-based invalidation
  cache.insert("user:1:profile".to_string(), "profile1".to_string(), None);
  cache.insert("user:1:settings".to_string(), "settings1".to_string(), None);
  cache.insert("user:2:profile".to_string(), "profile2".to_string(), None);

  assert_eq!(cache.len(), 5);

  // Invalidate all user:1 entries
  cache.invalidate_pattern("user:1:*");
  assert_eq!(cache.len(), 3);
  assert!(!cache.contains_key("user:1:profile"));
  assert!(!cache.contains_key("user:1:settings"));
  assert!(cache.contains_key("user:2:profile"));

  // Test cache clear
  cache.clear();
  assert_eq!(cache.len(), 0);
  assert!(cache.is_empty());
}

#[ tokio::test ]
async fn test_cache_statistics_tracking()
{
  let config = RequestCacheConfig::default();
  let cache = RequestCache::new(config);

  // Initial stats should be zero
  let stats = cache.stats();
  assert_eq!(stats.hits, 0);
  assert_eq!(stats.misses, 0);
  assert_eq!(stats.evictions, 0);

  // Insert and access entries to generate stats
  cache.insert("key1".to_string(), "value1".to_string(), None);

  // Cache hit
  let _ = cache.get("key1");
  let stats = cache.stats();
  assert_eq!(stats.hits, 1);
  assert_eq!(stats.misses, 0);

  // Cache miss
  let _ = cache.get("nonexistent");
  let stats = cache.stats();
  assert_eq!(stats.hits, 1);
  assert_eq!(stats.misses, 1);

  // Calculate hit ratio
  assert!((stats.hit_ratio() - 0.5).abs() < f64::EPSILON);
}

#[ tokio::test ]
async fn test_ollama_client_cache_integration()
{
  let cache_config = RequestCacheConfig::new()
    .with_max_entries(10)
    .with_default_ttl(Duration::from_secs(60));

  let mut client = OllamaClient::new( "http://test.example:11434".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_request_cache(cache_config);

  assert!(client.has_cache());
  assert_eq!(client.cache_stats().hits, 0);
  assert_eq!(client.cache_stats().misses, 0);

  // Create a request that would normally fail due to unreachable server
  let request = ChatRequest {
    model : "test-model".to_string(),
    messages : vec![ChatMessage {
      role : MessageRole::User,
      content : "Hello, cache test".to_string(),
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    }],
    stream : Some(false),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  // First call should be a cache miss and network error
  let result1 = client.chat_cached(request.clone()).await;
  assert!(result1.is_err());

  let stats = client.cache_stats();
  assert_eq!(stats.misses, 1);
  assert_eq!(stats.hits, 0);

  // Manually insert cached response for testing
  client.cache_response(&request, "cached_response".to_string(), None);

  // Second call should be a cache hit
  let result2 = client.chat_cached(request.clone()).await;
  assert!(result2.is_ok());

  let stats = client.cache_stats();
  assert_eq!(stats.hits, 1);
  assert_eq!(stats.misses, 1);
}

#[ tokio::test ]
async fn test_cache_key_generation()
{
  let config = RequestCacheConfig::default();
  let cache = RequestCache::new(config);

  let request1 = ChatRequest {
    model : "llama2".to_string(),
    messages : vec![ChatMessage {
      role : MessageRole::User,
      content : "Hello".to_string(),
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    }],
    stream : Some(false),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  let request2 = ChatRequest {
    model : "llama2".to_string(),
    messages : vec![ChatMessage {
      role : MessageRole::User,
      content : "Hello".to_string(),
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    }],
    stream : Some(false),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  let request3 = ChatRequest {
    model : "llama2".to_string(),
    messages : vec![ChatMessage {
      role : MessageRole::User,
      content : "Different message".to_string(),
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    }],
    stream : Some(false),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  // Same requests should generate same cache keys
  let key1 = cache.generate_key(&request1);
  let key2 = cache.generate_key(&request2);
  assert_eq!(key1, key2);

  // Different requests should generate different cache keys
  let key3 = cache.generate_key(&request3);
  assert_ne!(key1, key3);
}

// Note : Performance overhead test moved to benches/cache_performance.rs
//
// Performance measurements were causing flaky test failures due to timing variability
// across different systems and load conditions. Per test_organization.rulebook.md,
// performance tests belong in benches/ directory, not in the functional test suite.
//
// Run with : cargo bench --bench cache_performance --all-features

#[ tokio::test ]
async fn test_cache_concurrent_access()
{
  let config = RequestCacheConfig::new().with_max_entries(100);
  let cache = Arc::new(RequestCache::new(config));

  let mut handles = vec![];

  // Spawn multiple tasks that access cache concurrently
  for i in 0..10
  {
    let cache = cache.clone();
    let handle = tokio::spawn(async move {
      for j in 0..10
      {
        let key = format!( "thread_{i}_key_{j}" );
        let value = format!( "thread_{i}_value_{j}" );

        // Insert and retrieve entries
        cache.insert(key.clone(), value.clone(), None);
        sleep(Duration::from_millis(1)).await;

        let retrieved = cache.get(&key);
        assert_eq!(retrieved, Some(value));
      }
    });
    handles.push(handle);
  }

  // Wait for all tasks to complete
  for handle in handles
  {
    handle.await.unwrap();
  }

  // Should have entries from all threads
  assert!(!cache.is_empty());
  assert!(cache.len() <= 100); // Shouldn't exceed max capacity
}

#[ tokio::test ]
async fn test_cache_with_different_request_types()
{
  let config = RequestCacheConfig::default();
  let cache = RequestCache::new(config);

  // Test with ChatRequest
  let chat_request = ChatRequest {
    model : "llama2".to_string(),
    messages : vec![ChatMessage {
      role : MessageRole::User,
      content : "Chat test".to_string(),
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    }],
    stream : Some(false),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  // Test with GenerateRequest
  let generate_request = GenerateRequest {
    model : "llama2".to_string(),
    prompt : "Generate test".to_string(),
    stream : Some(false),
    options : None,
  };

  // Generate keys for different request types
  let chat_key = cache.generate_key(&chat_request);
  let generate_key = cache.generate_key(&generate_request);

  // Keys should be different for different request types
  assert_ne!(chat_key, generate_key);

  // Cache should handle both request types
  cache.insert(chat_key.clone(), "chat_response".to_string(), None);
  cache.insert(generate_key.clone(), "generate_response".to_string(), None);

  assert_eq!(cache.get(&chat_key), Some("chat_response".to_string()));
  assert_eq!(cache.get(&generate_key), Some("generate_response".to_string()));
}

#[ tokio::test ]
async fn test_cache_memory_efficiency()
{
  let config = RequestCacheConfig::new().with_max_entries(5);
  let cache = RequestCache::new(config);

  // Fill cache beyond capacity to test memory bounds
  for i in 0..10
  {
    let key = format!( "key_{i}" );
    let value = format!( "value_{i}" );
    cache.insert(key, value, None);
  }

  // Cache should not exceed max capacity
  assert_eq!(cache.len(), 5);
  assert_eq!(cache.capacity(), 5);

  // Should have evicted oldest entries
  let stats = cache.stats();
  assert_eq!(stats.evictions, 5);
}

#[ tokio::test ]
async fn test_cache_debug_and_display()
{
  let config = RequestCacheConfig::default();
  let cache = RequestCache::new(config);

  // Test Debug implementation
  let debug_output = format!( "{cache:?}" );
  assert!(debug_output.contains("RequestCache"));
  assert!(debug_output.contains("entries: 0"));

  // Test Display implementation
  let display_output = format!( "{cache}" );
  assert!(display_output.contains("Cache"));
  assert!(display_output.contains("0/100"));
}
