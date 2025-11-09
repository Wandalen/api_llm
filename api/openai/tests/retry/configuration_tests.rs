//! Retry Configuration Tests
//!
//! Tests for `EnhancedRetryConfig` including:
//! - Default configuration values
//! - Builder pattern construction
//! - Configuration validation rules

#[ cfg( feature = "retry" ) ]
mod retry_configuration_tests
{
  use crate::enhanced_retry_helpers::*;

  #[ tokio::test ]
  async fn test_retry_config_default_values()
  {
    let config = EnhancedRetryConfig::default();

    assert_eq!( config.max_attempts, 3 );
    assert_eq!( config.base_delay_ms, 1000 );
    assert_eq!( config.max_delay_ms, 30000 );
    assert_eq!( config.max_elapsed_time_ms, 120_000 );
    assert_eq!( config.jitter_ms, 100 );
    assert!( ( config.backoff_multiplier - 2.0 ).abs() < f64::EPSILON );
  }

  #[ tokio::test ]
  async fn test_retry_config_builder_pattern()
  {
    let config = EnhancedRetryConfig::new()
      .with_max_attempts( 5 )
      .with_base_delay( 500 )
      .with_max_delay( 60000 )
      .with_max_elapsed_time( 180_000 )
      .with_jitter( 200 )
      .with_backoff_multiplier( 1.5 );

    assert_eq!( config.max_attempts, 5 );
    assert_eq!( config.base_delay_ms, 500 );
    assert_eq!( config.max_delay_ms, 60000 );
    assert_eq!( config.max_elapsed_time_ms, 180_000 );
    assert_eq!( config.jitter_ms, 200 );
    assert!( ( config.backoff_multiplier - 1.5 ).abs() < f64::EPSILON );
  }

  #[ tokio::test ]
  async fn test_retry_config_validation()
  {
    // Valid configuration
    let valid_config = EnhancedRetryConfig::default();
    assert!( valid_config.validate().is_ok() );

    // Invalid : max_attempts = 0
    let invalid_config = EnhancedRetryConfig::default().with_max_attempts( 0 );
    assert!( invalid_config.validate().is_err() );

    // Invalid : base_delay_ms = 0
    let invalid_config = EnhancedRetryConfig::default().with_base_delay( 0 );
    assert!( invalid_config.validate().is_err() );

    // Invalid : max_delay_ms < base_delay_ms
    let invalid_config = EnhancedRetryConfig::default()
      .with_base_delay( 5000 )
      .with_max_delay( 1000 );
    assert!( invalid_config.validate().is_err() );

    // Invalid : max_elapsed_time_ms = 0
    let invalid_config = EnhancedRetryConfig::default().with_max_elapsed_time( 0 );
    assert!( invalid_config.validate().is_err() );

    // Invalid : backoff_multiplier <= 0
    let invalid_config = EnhancedRetryConfig::default().with_backoff_multiplier( 0.0 );
    assert!( invalid_config.validate().is_err() );
  }
}
