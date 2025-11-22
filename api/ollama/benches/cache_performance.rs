//! Request cache performance benchmarks
//!
//! Measures cache operation overhead to ensure reasonable performance
//! (target : < 100μs per operation in debug builds).
//!
//! Originally in `tests/request_caching_tests.rs` but moved here because
//! performance measurements should not be in functional test suite due to
//! timing variability per `test_organization.rulebook.md`.

#![ cfg( feature = "request_caching" ) ]

use api_ollama::{ RequestCache, RequestCacheConfig };
use std::time::Instant;

fn main()
{
  benchmark_cache_performance();
}

fn benchmark_cache_performance()
{
  let config = RequestCacheConfig::default();
  let cache = RequestCache::new( config );

  let iterations = 10000;

  // Warmup
  for i in 0..100
  {
    let key = format!( "warmup_{i}" );
    let value = format!( "value_{i}" );
    cache.insert( key.clone(), value, None );
    let _ = cache.get( &key );
  }

  // Measure cache operation overhead
  let start = Instant::now();
  for i in 0..iterations
  {
    let key = format!( "key_{i}" );
    let value = format!( "value_{i}" );
    cache.insert( key.clone(), value, None );
    let _ = cache.get( &key );
  }
  let duration = start.elapsed();
  let overhead_per_operation = duration / ( iterations * 2 ); // insert + get

  println!( "\n=== Cache Performance Benchmark ===" );
  println!( "Iterations : {iterations}" );
  println!( "Total duration : {duration:?}" );
  println!( "Per operation : {overhead_per_operation:?}" );

  let micros = overhead_per_operation.as_micros();
  if micros > 200
  {
    println!( "⚠️  WARNING: Cache operations exceed 200μs per operation!" );
  }
  else if micros > 100
  {
    println!( "⚠️  CAUTION: Cache operations exceed 100μs per operation" );
  }
  else
  {
    println!( "✅ Cache performance within acceptable range (< 100μs)" );
  }
}
