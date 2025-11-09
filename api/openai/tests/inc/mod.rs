use super::*;

// --- Test Environment ---

/// Initializes tracing subscriber specifically for tests.
#[ allow( dead_code ) ]
fn tracing_init_test()
{
  static TRACING_INIT : std::sync::Once = std::sync::Once::new();
  TRACING_INIT.call_once( ||
  {
    // try_init() avoids panic if already initialized
    let _ = tracing_subscriber::fmt()
    .with_test_writer() // Crucial for working with test output capture
    .try_init()
    ;
  } );
}
// --- End Test Environment ---

mod basic_test;
mod components_test;
mod experiment;
mod test_data_factories;
pub mod circuit_breaker_test_support;
pub mod enhanced_retry_helpers;
