//! Benchmarks for retry logic overhead measurement
#![allow(missing_docs)]

use criterion::{ criterion_group, criterion_main, Criterion };
use std::time::Duration;

fn benchmark_retry_config_creation( c: &mut Criterion )
{
  c.bench_function( "create_retry_config", |b|
  {
    b.iter( ||
    {
      // Simulate retry configuration creation
      RetryConfig {
        max_attempts: 3,
        initial_delay: Duration::from_millis( 100 ),
        max_delay: Duration::from_secs( 10 ),
        backoff_multiplier: 2.0,
        enable_jitter: true,
      }
    } );
  } );
}

fn benchmark_retry_decision( c: &mut Criterion )
{
  let config = RetryConfig {
    max_attempts: 3,
    initial_delay: Duration::from_millis( 100 ),
    max_delay: Duration::from_secs( 10 ),
    backoff_multiplier: 2.0,
    enable_jitter: true,
  };

  c.bench_function( "retry_decision_logic", |b|
  {
    b.iter( ||
    {
      // Simulate retry decision logic
      let attempt = 1;
      attempt < config.max_attempts
    } );
  } );
}

fn benchmark_backoff_calculation( c: &mut Criterion )
{
  let config = RetryConfig {
    max_attempts: 3,
    initial_delay: Duration::from_millis( 100 ),
    max_delay: Duration::from_secs( 10 ),
    backoff_multiplier: 2.0,
    enable_jitter: true,
  };

  c.bench_function( "calculate_backoff_delay", |b|
  {
    b.iter( ||
    {
      // Simulate exponential backoff calculation
      let attempt = 2_i32;
      let base_delay = config.initial_delay.as_millis() as f64;
      let multiplier = config.backoff_multiplier.powi( attempt );
      let delay_ms = ( base_delay * multiplier ).min( config.max_delay.as_millis() as f64 );

      if config.enable_jitter
      {
        let jitter = ( delay_ms * 0.1 ) * rand::random::< f64 >();
        Duration::from_millis( ( delay_ms + jitter ) as u64 )
      } else {
        Duration::from_millis( delay_ms as u64 )
      }
    } );
  } );
}

fn benchmark_error_classification( c: &mut Criterion )
{
  c.bench_function( "classify_retryable_error", |b|
  {
    b.iter( ||
    {
      // Simulate error classification logic
      let status_code = 503;
      let is_retryable = matches!( status_code, 429 | 500 | 502 | 503 | 504 );
      is_retryable
    } );
  } );
}

// Simple retry configuration struct for benchmarking
#[ derive( Debug, Clone ) ]
struct RetryConfig
{
  max_attempts: u32,
  initial_delay: Duration,
  max_delay: Duration,
  backoff_multiplier: f64,
  enable_jitter: bool,
}

criterion_group!(
benches,
benchmark_retry_config_creation,
benchmark_retry_decision,
benchmark_backoff_calculation,
benchmark_error_classification
);
criterion_main!( benches );
