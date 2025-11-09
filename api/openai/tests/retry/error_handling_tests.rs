//! Retry Error Classification Tests
//!
//! Tests for error classification logic including:
//! - Retryable error detection (network, timeout, 5xx, 429)
//! - Non-retryable error detection (4xx, invalid arguments, etc.)

#[ cfg( feature = "retry" ) ]
mod retry_error_handling_tests
{
  use crate::enhanced_retry_helpers::*;
  use api_openai::error::OpenAIError;

  #[ tokio::test ]
  async fn test_error_classification_retryable()
  {
    let config = EnhancedRetryConfig::default();

    // Network errors should be retryable
    let network_error = OpenAIError::Network( "Connection failed".to_string() );
    assert!( config.is_retryable_error( &network_error ) );

    // Timeout errors should be retryable
    let timeout_error = OpenAIError::Timeout( "Request timeout".to_string() );
    assert!( config.is_retryable_error( &timeout_error ) );

    // 5xx HTTP errors should be retryable
    let server_error = OpenAIError::Http( "HTTP error with status 500: Internal Server Error".to_string() );
    assert!( config.is_retryable_error( &server_error ) );

    // 429 Rate limiting should be retryable
    let rate_limit_error = OpenAIError::Http( "HTTP error with status 429: Rate limit exceeded".to_string() );
    assert!( config.is_retryable_error( &rate_limit_error ) );

    // Rate limit errors should be retryable
    let rate_limit_direct = OpenAIError::RateLimit( "Rate limit exceeded".to_string() );
    assert!( config.is_retryable_error( &rate_limit_direct ) );
  }

  #[ tokio::test ]
  async fn test_error_classification_non_retryable()
  {
    let config = EnhancedRetryConfig::default();

    // API errors should not be retryable (using Internal as an alternative)
    let api_error = OpenAIError::Internal( "Invalid API key".to_string() );
    assert!( !config.is_retryable_error( &api_error ) );

    // Invalid argument errors should not be retryable
    let validation_error = OpenAIError::InvalidArgument( "Invalid request".to_string() );
    assert!( !config.is_retryable_error( &validation_error ) );

    // Internal errors should not be retryable
    let parse_error = OpenAIError::Internal( "JSON parse error".to_string() );
    assert!( !config.is_retryable_error( &parse_error ) );

    // Missing argument errors should not be retryable
    let config_error = OpenAIError::MissingArgument( "Missing required field".to_string() );
    assert!( !config.is_retryable_error( &config_error ) );

    // 4xx HTTP errors (except 429) should not be retryable
    let client_error = OpenAIError::Http( "HTTP error with status 400: Bad Request".to_string() );
    assert!( !config.is_retryable_error( &client_error ) );

    let not_found_error = OpenAIError::Http( "HTTP error with status 404: Not Found".to_string() );
    assert!( !config.is_retryable_error( &not_found_error ) );
  }
}
