//! Benchmarks for streaming buffer overhead measurement
#![allow(missing_docs)]

use criterion::{ criterion_group, criterion_main, Criterion };
use std::collections::VecDeque;

fn benchmark_buffer_allocation( c: &mut Criterion )
{
  c.bench_function( "allocate_stream_buffer", |b|
  {
    b.iter( ||
    {
      // Simulate buffer allocation
      VecDeque::< String >::with_capacity( 1024 )
    } );
  } );
}

fn benchmark_buffer_push( c: &mut Criterion )
{
  c.bench_function( "push_to_stream_buffer", |b|
  {
    b.iter( ||
    {
      let mut buffer = VecDeque::< String >::with_capacity( 1024 );
    let chunk = "data : {\"text\":\"Hello\"}\n\n".to_string();

      // Simulate pushing to buffer
      buffer.push_back( chunk );
      buffer
    } );
  } );
}

fn benchmark_buffer_pop( c: &mut Criterion )
{
  c.bench_function( "pop_from_stream_buffer", |b|
  {
    b.iter( ||
    {
      let mut buffer = VecDeque::< String >::with_capacity( 1024 );
    buffer.push_back( "data : {\"text\":\"Hello\"}\n\n".to_string() );

      // Simulate popping from buffer
      buffer.pop_front()
    } );
  } );
}

fn benchmark_sse_parsing( c: &mut Criterion )
{
let sse_line = "data : {\"candidates\":[{\"content\":{\"parts\":[{\"text\":\"Hello\"}]}}]}\n\n";

  c.bench_function( "parse_sse_line", |b|
  {
    b.iter( ||
    {
      // Simulate SSE line parsing
      if let Some( data ) = sse_line.strip_prefix( "data: " )
      {
        let json_str = data.trim();
        !json_str.is_empty()
      } else {
        false
      }
    } );
  } );
}

fn benchmark_json_chunk_parsing( c: &mut Criterion )
{
let json_chunk = r#"{"candidates":[{"content":{"parts":[{"text":"Hello"}]}}]}"#;

  c.bench_function( "parse_json_chunk", |b|
  {
    b.iter( ||
    {
      // Simulate JSON parsing of streaming chunk
      serde_json::from_str::< serde_json::Value >( json_chunk ).ok()
    } );
  } );
}

fn benchmark_chunk_accumulation( c: &mut Criterion )
{
  c.bench_function( "accumulate_partial_chunks", |b|
  {
    b.iter( ||
    {
      let mut accumulated = String::new();
      let chunk1 = "{\"text\":\"Hel";
      let chunk2 = "lo world\"}";

      // Simulate chunk accumulation
      accumulated.push_str( chunk1 );
      accumulated.push_str( chunk2 );
      accumulated
    } );
  } );
}

fn benchmark_buffer_size_check( c: &mut Criterion )
{
  c.bench_function( "check_buffer_size_limit", |b|
  {
    let buffer = VecDeque::< String >::with_capacity( 1024 );
    let max_buffer_size = 10 * 1024 * 1024; // 10MB

    b.iter( ||
    {
      // Simulate buffer size checking
      let total_size: usize = buffer.iter().map( | s | s.len() ).sum();
      total_size < max_buffer_size
    } );
  } );
}

fn benchmark_event_type_classification( c: &mut Criterion )
{
  c.bench_function( "classify_stream_event_type", |b|
  {
  let line = "data : {\"text\":\"content\"}";

    b.iter( ||
    {
      // Simulate event type classification
      if line.starts_with( "data: " )
      {
        EventType::Data
      } else if line.starts_with( "event : " )
      {
        EventType::Event
      } else if line.starts_with( ": " )
      {
        EventType::Comment
      } else {
        EventType::Unknown
      }
    } );
  } );
}

// Event types for benchmarking
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
enum EventType
{
  Data,
  Event,
  Comment,
  Unknown,
}

criterion_group!(
benches,
benchmark_buffer_allocation,
benchmark_buffer_push,
benchmark_buffer_pop,
benchmark_sse_parsing,
benchmark_json_chunk_parsing,
benchmark_chunk_accumulation,
benchmark_buffer_size_check,
benchmark_event_type_classification
);
criterion_main!( benches );
