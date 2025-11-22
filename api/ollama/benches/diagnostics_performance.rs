//! Diagnostics system performance benchmarks
//!
//! Measures overhead of diagnostics collection to ensure it remains
//! within acceptable bounds (< 2x without diagnostics).
//!
//! Originally in `tests/general_diagnostics_tests.rs` but moved here because
//! performance measurements should not be in functional test suite due to
//! timing variability per `test_organization.rulebook.md`.

#![ cfg( feature = "general_diagnostics" ) ]

use api_ollama::{ DiagnosticsConfig, DiagnosticsCollector, GenerateRequest };
use std::time::Instant;

fn main()
{
  benchmark_diagnostics_overhead();
}

fn benchmark_diagnostics_overhead()
{
  let config_without_diagnostics = DiagnosticsConfig::new()
  .with_performance_metrics( false )
  .with_error_analysis( false )
  .with_curl_generation( false );

  let config_with_diagnostics = DiagnosticsConfig::new()
  .with_performance_metrics( true )
  .with_error_analysis( true )
  .with_curl_generation( true );

  let collector_without = DiagnosticsCollector::new( config_without_diagnostics );
  let collector_with = DiagnosticsCollector::new( config_with_diagnostics );

  let iterations = 10000; // Increased for better statistical significance

  // Warmup
  for i in 0..100
  {
    let request_id = format!( "warmup-{i}" );
    let request = GenerateRequest
    {
      model : "test".to_string(),
      prompt : "test".to_string(),
      stream : Some( false ),
      options : None,
    };
    collector_without.track_request_start( &request_id, &request );
    collector_without.track_request_success( &request_id, 100 );
  }

  // Measure overhead without diagnostics
  let start_without = Instant::now();
  for i in 0..iterations
  {
    let request_id = format!( "overhead-test-without-{i}" );
    let request = GenerateRequest
    {
      model : "test".to_string(),
      prompt : "test".to_string(),
      stream : Some( false ),
      options : None,
    };
    collector_without.track_request_start( &request_id, &request );
    collector_without.track_request_success( &request_id, 100 );
  }
  let duration_without = start_without.elapsed();

  // Measure overhead with full diagnostics
  let start_with = Instant::now();
  for i in 0..iterations
  {
    let request_id = format!( "overhead-test-with-{i}" );
    let request = GenerateRequest
    {
      model : "test".to_string(),
      prompt : "test".to_string(),
      stream : Some( false ),
      options : None,
    };
    collector_with.track_request_start_with_curl( &request_id, &request, "http://localhost:11434" );
    collector_with.track_request_success( &request_id, 100 );
  }
  let duration_with = start_with.elapsed();

  let overhead_ratio = duration_with.as_nanos() as f64 / duration_without.as_nanos() as f64;

  println!( "\n=== Diagnostics Performance Benchmark ===" );
  println!( "Iterations : {iterations}" );
  println!( "Without diagnostics : {duration_without:?}" );
  println!( "With diagnostics : {duration_with:?}" );
  println!( "Overhead ratio : {overhead_ratio:.3}x" );

  if overhead_ratio > 3.0
  {
    println!( "⚠️  WARNING: Overhead exceeds 3x threshold!" );
  }
  else if overhead_ratio > 2.0
  {
    println!( "⚠️  CAUTION: Overhead exceeds 2x threshold" );
  }
  else
  {
    println!( "✅ Overhead within acceptable range (< 2x)" );
  }
}
