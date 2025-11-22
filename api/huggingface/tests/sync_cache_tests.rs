//! Integration tests for synchronous cache

#![ cfg( feature = "sync" ) ]
#![ allow( clippy::std_instead_of_core ) ]

use api_huggingface::sync::SyncClient;
use api_huggingface::cache::CacheConfig;
use core::time::Duration;

#[ test ]
fn test_sync_cache_basic()
{
  let client = SyncClient::new( "test_key".to_string() )
  .expect( "Failed to create client" );

  let cache = client.cache::< String, String >();

  cache.insert( "key1".to_string(), "value1".to_string(), None );

  let value = cache.get( &"key1".to_string() );
  assert_eq!( value, Some( "value1".to_string() ) );
}

#[ test ]
fn test_sync_cache_miss()
{
  let client = SyncClient::new( "test_key".to_string() )
  .expect( "Failed to create client" );

  let cache = client.cache::< String, String >();

  let value = cache.get( &"nonexistent".to_string() );
  assert_eq!( value, None );
}

#[ test ]
fn test_sync_cache_contains_key()
{
  let client = SyncClient::new( "test_key".to_string() )
  .expect( "Failed to create client" );

  let cache = client.cache::< String, String >();

  cache.insert( "key1".to_string(), "value1".to_string(), None );

  assert!( cache.contains_key( &"key1".to_string() ) );
  assert!( !cache.contains_key( &"key2".to_string() ) );
}

#[ test ]
fn test_sync_cache_remove()
{
  let client = SyncClient::new( "test_key".to_string() )
  .expect( "Failed to create client" );

  let cache = client.cache::< String, String >();

  cache.insert( "key1".to_string(), "value1".to_string(), None );
  assert!( cache.contains_key( &"key1".to_string() ) );

  let removed = cache.remove( &"key1".to_string() );
  assert_eq!( removed, Some( "value1".to_string() ) );
  assert!( !cache.contains_key( &"key1".to_string() ) );
}

#[ test ]
fn test_sync_cache_clear()
{
  let client = SyncClient::new( "test_key".to_string() )
  .expect( "Failed to create client" );

  let cache = client.cache::< String, String >();

  cache.insert( "key1".to_string(), "value1".to_string(), None );
  cache.insert( "key2".to_string(), "value2".to_string(), None );

  assert_eq!( cache.len(), 2 );

  cache.clear();

  assert_eq!( cache.len(), 0 );
  assert!( cache.is_empty() );
}

#[ test ]
fn test_sync_cache_len_and_empty()
{
  let client = SyncClient::new( "test_key".to_string() )
  .expect( "Failed to create client" );

  let cache = client.cache::< String, String >();

  assert_eq!( cache.len(), 0 );
  assert!( cache.is_empty() );

  cache.insert( "key1".to_string(), "value1".to_string(), None );

  assert_eq!( cache.len(), 1 );
  assert!( !cache.is_empty() );
}

#[ test ]
fn test_sync_cache_with_config()
{
  let client = SyncClient::new( "test_key".to_string() )
  .expect( "Failed to create client" );

  let config = CacheConfig
  {
  max_entries : 10,
  default_ttl : Some( Duration::from_secs( 60 ) ),
  };

  let cache = client.cache_with_config::< String, String >( config );

  cache.insert( "key1".to_string(), "value1".to_string(), None );

  let value = cache.get( &"key1".to_string() );
  assert_eq!( value, Some( "value1".to_string() ) );
}

#[ test ]
fn test_sync_cache_multiple_entries()
{
  let client = SyncClient::new( "test_key".to_string() )
  .expect( "Failed to create client" );

  let cache = client.cache::< String, String >();

  for i in 0..10
  {
  cache.insert( format!( "key{i}" ), format!( "value{i}" ), None );
  }

  assert_eq!( cache.len(), 10 );

  for i in 0..10
  {
  let value = cache.get( &format!( "key{i}" ) );
  assert_eq!( value, Some( format!( "value{i}" ) ) );
  }
}

#[ test ]
fn test_sync_cache_integer_keys()
{
  let client = SyncClient::new( "test_key".to_string() )
  .expect( "Failed to create client" );

  let cache = client.cache::< i32, String >();

  cache.insert( 1, "one".to_string(), None );
  cache.insert( 2, "two".to_string(), None );

  assert_eq!( cache.get( &1 ), Some( "one".to_string() ) );
  assert_eq!( cache.get( &2 ), Some( "two".to_string() ) );
}

#[ test ]
fn test_sync_cache_overwrite()
{
  let client = SyncClient::new( "test_key".to_string() )
  .expect( "Failed to create client" );

  let cache = client.cache::< String, String >();

  cache.insert( "key1".to_string(), "value1".to_string(), None );
  assert_eq!( cache.get( &"key1".to_string() ), Some( "value1".to_string() ) );

  cache.insert( "key1".to_string(), "value2".to_string(), None );
  assert_eq!( cache.get( &"key1".to_string() ), Some( "value2".to_string() ) );
}

#[ test ]
fn test_sync_cache_reusable()
{
  let client = SyncClient::new( "test_key".to_string() )
  .expect( "Failed to create client" );

  // Create multiple caches
  let cache1 = client.cache::< String, String >();
  let cache2 = client.cache::< i32, i32 >();

  cache1.insert( "key".to_string(), "value".to_string(), None );
  cache2.insert( 1, 100, None );

  assert_eq!( cache1.get( &"key".to_string() ), Some( "value".to_string() ) );
  assert_eq!( cache2.get( &1 ), Some( 100 ) );
}

#[ test ]
fn test_sync_cache_clone_values()
{
  let client = SyncClient::new( "test_key".to_string() )
  .expect( "Failed to create client" );

  let cache = client.cache::< String, Vec< i32 > >();

  let vec = vec![ 1, 2, 3 ];
  cache.insert( "key".to_string(), vec.clone(), None );

  let cached = cache.get( &"key".to_string() );
  assert_eq!( cached, Some( vec ) );
}

#[ test ]
fn test_sync_cache_empty_string()
{
  let client = SyncClient::new( "test_key".to_string() )
  .expect( "Failed to create client" );

  let cache = client.cache::< String, String >();

  cache.insert( "key".to_string(), String::new(), None );

  let value = cache.get( &"key".to_string() );
  assert_eq!( value, Some( String::new() ) );
}
