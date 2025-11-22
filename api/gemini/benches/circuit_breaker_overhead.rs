//! Benchmarks for circuit breaker overhead measurement
#![allow(missing_docs)]

use criterion::{ criterion_group, criterion_main, Criterion };
use std::time::{ Duration, Instant };

fn benchmark_circuit_breaker_state_check( c: &mut Criterion )
{
  c.bench_function( "check_circuit_breaker_state", |b|
  {
    let breaker = CircuitBreakerState {
      state: State::Closed,
      failure_count: 0,
      last_failure_time: None,
      half_open_successes: 0,
    };

    b.iter( ||
    {
      // Simulate state check logic
      matches!( breaker.state, State::Closed )
    } );
  } );
}

fn benchmark_failure_recording( c: &mut Criterion )
{
  c.bench_function( "record_failure", |b|
  {
    b.iter( ||
    {
      let mut breaker = CircuitBreakerState {
        state: State::Closed,
        failure_count: 4,
        last_failure_time: None,
        half_open_successes: 0,
      };

      // Simulate failure recording
      breaker.failure_count += 1;
      breaker.last_failure_time = Some( Instant::now() );

      if breaker.failure_count >= 5
      {
        breaker.state = State::Open;
      }

      breaker
    } );
  } );
}

fn benchmark_success_recording( c: &mut Criterion )
{
  c.bench_function( "record_success", |b|
  {
    b.iter( ||
    {
      let mut breaker = CircuitBreakerState {
        state: State::HalfOpen,
        failure_count: 0,
        last_failure_time: None,
        half_open_successes: 1,
      };

      // Simulate success recording in half-open state
      breaker.half_open_successes += 1;

      if breaker.half_open_successes >= 2
      {
        breaker.state = State::Closed;
        breaker.failure_count = 0;
        breaker.half_open_successes = 0;
      }

      breaker
    } );
  } );
}

fn benchmark_timeout_check( c: &mut Criterion )
{
  c.bench_function( "check_timeout_for_half_open", |b|
  {
    let breaker = CircuitBreakerState {
      state: State::Open,
      failure_count: 5,
      last_failure_time: Some( Instant::now() - Duration::from_secs( 61 ) ),
      half_open_successes: 0,
    };

    let timeout = Duration::from_secs( 60 );

    b.iter( ||
    {
      // Simulate timeout check for transitioning to half-open
      if let Some( last_failure ) = breaker.last_failure_time
      {
        let elapsed = Instant::now() - last_failure;
        elapsed >= timeout
      } else {
        false
      }
    } );
  } );
}

// Circuit breaker state for benchmarking
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
enum State
{
  Closed,
  Open,
  HalfOpen,
}

#[ derive( Debug, Clone ) ]
struct CircuitBreakerState
{
  state: State,
  failure_count: u32,
  last_failure_time: Option< Instant >,
  half_open_successes: u32,
}

criterion_group!(
benches,
benchmark_circuit_breaker_state_check,
benchmark_failure_recording,
benchmark_success_recording,
benchmark_timeout_check
);
criterion_main!( benches );
