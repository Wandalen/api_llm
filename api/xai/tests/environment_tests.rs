//! Tests for environment configuration and HTTP client setup.

use api_xai::{ XaiEnvironmentImpl, XaiEnvironment, Secret };
use core::time::Duration;
use url::Url;

#[ test ]
fn environment_uses_default_base_url()
{
  let secret = Secret::new( "xai-test-key-1234567890".to_string() ).unwrap();
  let env = XaiEnvironmentImpl::new( secret ).unwrap();

  assert_eq!( env.base_url().as_str(), "https://api.x.ai/v1/" );
}

#[ test ]
fn environment_uses_default_timeout()
{
  let secret = Secret::new( "xai-test-key-1234567890".to_string() ).unwrap();
  let env = XaiEnvironmentImpl::new( secret ).unwrap();

  assert_eq!( env.timeout(), Duration::from_secs( 30 ) );
}

#[ test ]
fn environment_accepts_custom_base_url()
{
  let secret = Secret::new( "xai-test-key-1234567890".to_string() ).unwrap();
  let custom_url = Url::parse( "https://custom.api.endpoint/v1" ).unwrap();

  let env = XaiEnvironmentImpl::new( secret ).unwrap()
    .with_base_url( custom_url.clone() );

  assert_eq!( env.base_url(), &custom_url );
}

#[ test ]
fn environment_accepts_custom_timeout()
{
  let secret = Secret::new( "xai-test-key-1234567890".to_string() ).unwrap();

  let env = XaiEnvironmentImpl::new( secret ).unwrap()
    .with_timeout( Duration::from_secs( 60 ) );

  assert_eq!( env.timeout(), Duration::from_secs( 60 ) );
}

#[ test ]
fn environment_generates_correct_headers()
{
  let secret = Secret::new( "xai-test-key-1234567890".to_string() ).unwrap();
  let env = XaiEnvironmentImpl::new( secret ).unwrap();

  let headers = env.headers().unwrap();

  // Should have Authorization header
  assert!( headers.contains_key( "authorization" ) );

  // Should have Content-Type header
  assert!( headers.contains_key( "content-type" ) );
}

#[ test ]
fn environment_includes_bearer_token()
{
  let secret = Secret::new( "xai-test-key-1234567890".to_string() ).unwrap();
  let env = XaiEnvironmentImpl::new( secret ).unwrap();

  let headers = env.headers().unwrap();
  let auth_header = headers.get( "authorization" ).unwrap();
  let auth_str = auth_header.to_str().unwrap();

  assert!( auth_str.starts_with( "Bearer xai-" ) );
  assert!( auth_str.contains( "test-key-1234567890" ) );
}

#[ test ]
fn environment_includes_json_content_type()
{
  let secret = Secret::new( "xai-test-key-1234567890".to_string() ).unwrap();
  let env = XaiEnvironmentImpl::new( secret ).unwrap();

  let headers = env.headers().unwrap();
  let content_type = headers.get( "content-type" ).unwrap();

  assert_eq!( content_type, "application/json" );
}

#[ test ]
fn environment_builder_chains_correctly()
{
  let secret = Secret::new( "xai-test-key-1234567890".to_string() ).unwrap();
  let custom_url = Url::parse( "https://test.api/v1" ).unwrap();

  let env = XaiEnvironmentImpl::new( secret ).unwrap()
    .with_base_url( custom_url.clone() )
    .with_timeout( Duration::from_secs( 45 ) );

  assert_eq!( env.base_url(), &custom_url );
  assert_eq!( env.timeout(), Duration::from_secs( 45 ) );
}

#[ test ]
fn environment_clone_works()
{
  let secret = Secret::new( "xai-test-key-1234567890".to_string() ).unwrap();
  let env = XaiEnvironmentImpl::new( secret ).unwrap();

  let cloned = env.clone();

  assert_eq!( env.base_url(), cloned.base_url() );
  assert_eq!( env.timeout(), cloned.timeout() );
}

#[ test ]
fn environment_debug_works()
{
  let secret = Secret::new( "xai-test-key-1234567890".to_string() ).unwrap();
  let env = XaiEnvironmentImpl::new( secret ).unwrap();

  let debug_str = format!( "{env:?}" );

  // Should contain type name
  assert!( debug_str.contains( "XaiEnvironmentImpl" ) );
}

#[ test ]
fn environment_api_key_returns_reference()
{
  let secret = Secret::new( "xai-test-key-1234567890".to_string() ).unwrap();
  let env = XaiEnvironmentImpl::new( secret ).unwrap();

  let key = env.api_key();
  assert_eq!( key.expose_secret(), "xai-test-key-1234567890" );
}

#[ test ]
fn environment_with_very_short_timeout()
{
  let secret = Secret::new( "xai-test-key-1234567890".to_string() ).unwrap();

  let env = XaiEnvironmentImpl::new( secret ).unwrap()
    .with_timeout( Duration::from_millis( 100 ) );

  assert_eq!( env.timeout(), Duration::from_millis( 100 ) );
}

#[ test ]
fn environment_with_very_long_timeout()
{
  let secret = Secret::new( "xai-test-key-1234567890".to_string() ).unwrap();

  let env = XaiEnvironmentImpl::new( secret ).unwrap()
    .with_timeout( Duration::from_secs( 300 ) );

  assert_eq!( env.timeout(), Duration::from_secs( 300 ) );
}
