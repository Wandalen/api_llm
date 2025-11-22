//! Tests for error types and error handling.

use api_xai::{ XaiError, Result };

#[ test ]
fn error_display_formats_api_error_correctly()
{
  let error = XaiError::Api
  {
    message : "Invalid model specified".to_string(),
    code : Some( "invalid_model".to_string() ),
    error_type : Some( "invalid_request_error".to_string() ),
  };

  let display = format!( "{error}" );
  assert!( display.contains( "Invalid model specified" ) );
  assert!( display.contains( "invalid_model" ) );
  assert!( display.contains( "invalid_request_error" ) );
}

#[ test ]
fn error_display_formats_http_error_correctly()
{
  let error = XaiError::Http( "404 Not Found".to_string() );
  let display = format!( "{error}" );
  assert_eq!( display, "HTTP Error : 404 Not Found" );
}

#[ test ]
fn error_display_formats_network_error_correctly()
{
  let error = XaiError::Network( "Connection refused".to_string() );
  let display = format!( "{error}" );
  assert_eq!( display, "Network Error : Connection refused" );
}

#[ test ]
fn error_display_formats_timeout_error_correctly()
{
  let error = XaiError::Timeout( "Request timed out after 30s".to_string() );
  let display = format!( "{error}" );
  assert_eq!( display, "Timeout Error : Request timed out after 30s" );
}

#[ test ]
fn error_display_formats_stream_error_correctly()
{
  let error = XaiError::Stream( "SSE parse error".to_string() );
  let display = format!( "{error}" );
  assert_eq!( display, "Stream Error : SSE parse error" );
}

#[ test ]
fn error_display_formats_rate_limit_error_correctly()
{
  let error = XaiError::RateLimit( "Too many requests".to_string() );
  let display = format!( "{error}" );
  assert_eq!( display, "Rate Limit Error : Too many requests" );
}

#[ test ]
fn error_display_formats_serialization_error_correctly()
{
  let error = XaiError::Serialization( "Invalid JSON".to_string() );
  let display = format!( "{error}" );
  assert_eq!( display, "Serialization Error : Invalid JSON" );
}

#[ test ]
fn error_display_formats_invalid_api_key_error_correctly()
{
  let error = XaiError::InvalidApiKey( "Key must start with xai-".to_string() );
  let display = format!( "{error}" );
  assert_eq!( display, "Invalid API Key : Key must start with xai-" );
}

#[ test ]
fn error_display_formats_environment_error_correctly()
{
  let error = XaiError::Environment( "XAI_API_KEY not set".to_string() );
  let display = format!( "{error}" );
  assert_eq!( display, "Environment Error : XAI_API_KEY not set" );
}

#[ test ]
fn error_display_formats_url_parse_error_correctly()
{
  let error = XaiError::UrlParse( "Invalid URL".to_string() );
  let display = format!( "{error}" );
  assert_eq!( display, "URL Parse Error : Invalid URL" );
}

#[ test ]
fn error_clone_works()
{
  let error = XaiError::Http( "Test error".to_string() );
  let cloned = error.clone();
  assert_eq!( error, cloned );
}

#[ test ]
fn error_debug_works()
{
  let error = XaiError::Network( "Connection error".to_string() );
  let debug_str = format!( "{error:?}" );
  assert!( debug_str.contains( "Network" ) );
  assert!( debug_str.contains( "Connection error" ) );
}

#[ test ]
fn error_partial_eq_works()
{
  let error1 = XaiError::Timeout( "timeout".to_string() );
  let error2 = XaiError::Timeout( "timeout".to_string() );
  let error3 = XaiError::Timeout( "different".to_string() );

  assert_eq!( error1, error2 );
  assert_ne!( error1, error3 );
}

#[ test ]
fn serde_json_error_converts_to_serialization_error()
{
  let json_error = serde_json::from_str::< serde_json::Value >( "{invalid json" )
    .expect_err( "Should fail to parse" );

  let xai_error : XaiError = json_error.into();

  match xai_error
  {
    XaiError::Serialization( msg ) =>
    {
      eprintln!( "Actual error message : {msg}" );
      assert!( msg.contains( "expected" ) || msg.contains( "EOF" ) || msg.contains( "key must be a string" ) );
    }
    _ => panic!( "Expected Serialization error" ),
  }
}

#[ test ]
fn url_parse_error_converts_correctly()
{
  let url_error = url::Url::parse( "not a url" )
    .expect_err( "Should fail to parse" );

  let xai_error : XaiError = url_error.into();

  match xai_error
  {
    XaiError::UrlParse( msg ) =>
    {
      assert!( !msg.is_empty() );
    }
    _ => panic!( "Expected UrlParse error" ),
  }
}

#[ test ]
fn result_type_alias_works()
{
  #[ allow( clippy::unnecessary_wraps ) ]  // Intentional for testing Result type alias
  fn returns_ok() -> Result< i32 >
  {
    Ok( 42 )
  }

  fn returns_err() -> Result< i32 >
  {
    Err( XaiError::Environment( "test error".to_string() ).into() )
  }

  assert_eq!( returns_ok().unwrap(), 42 );
  assert!( returns_err().is_err() );
}

#[ cfg( feature = "circuit_breaker" ) ]
#[ test ]
fn error_circuit_breaker_open_formats_correctly()
{
  let error = XaiError::CircuitBreakerOpen( "requests blocked".to_string() );
  let display = format!( "{error}" );
  assert!( display.contains( "Circuit Breaker Open" ) );
  assert!( display.contains( "requests blocked" ) );
}
