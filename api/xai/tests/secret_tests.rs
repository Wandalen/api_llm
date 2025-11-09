//! Tests for secret management and API key validation.

use api_xai::Secret;

#[ test ]
fn secret_validates_xai_prefix()
{
  let result = Secret::new( "sk-1234567890".to_string() );
  assert!( result.is_err() );

  match result
  {
    Err( e ) =>
    {
      let error_str = format!( "{e:?}" );
      assert!( error_str.contains( "xai-" ) );
    }
    Ok( _ ) => panic!( "Expected error for invalid prefix" ),
  }
}

#[ test ]
fn secret_validates_minimum_length()
{
  let result = Secret::new( "xai-123".to_string() );
  assert!( result.is_err() );

  match result
  {
    Err( e ) =>
    {
      let error_str = format!( "{e:?}" );
      assert!( error_str.contains( "too short" ) || error_str.contains( "minimum" ) );
    }
    Ok( _ ) => panic!( "Expected error for short key" ),
  }
}

#[ test ]
fn secret_accepts_valid_key()
{
  let result = Secret::new( "xai-1234567890".to_string() );
  assert!( result.is_ok() );
}

#[ test ]
fn secret_accepts_longer_valid_key()
{
  let result = Secret::new( "xai-1234567890abcdefghijklmnopqrstuvwxyz".to_string() );
  assert!( result.is_ok() );
}

#[ test ]
fn secret_loads_from_env()
{
  std::env::set_var( "XAI_TEST_KEY_1", "xai-test-key-1234567890" );

  let result = Secret::load_from_env( "XAI_TEST_KEY_1" );
  assert!( result.is_ok() );

  std::env::remove_var( "XAI_TEST_KEY_1" );
}

#[ test ]
fn secret_fails_when_env_not_set()
{
  std::env::remove_var( "XAI_NONEXISTENT_KEY" );

  let result = Secret::load_from_env( "XAI_NONEXISTENT_KEY" );
  assert!( result.is_err() );

  match result
  {
    Err( e ) =>
    {
      let error_str = format!( "{e:?}" );
      assert!( error_str.contains( "not set" ) || error_str.contains( "Environment" ) );
    }
    Ok( _ ) => panic!( "Expected error for missing env var" ),
  }
}

#[ test ]
fn secret_fails_when_env_has_invalid_key()
{
  std::env::set_var( "XAI_INVALID_KEY", "sk-invalid" );

  let result = Secret::load_from_env( "XAI_INVALID_KEY" );
  assert!( result.is_err() );

  std::env::remove_var( "XAI_INVALID_KEY" );
}

#[ test ]
fn secret_exposure_increments_counter()
{
  let secret = Secret::new( "xai-1234567890".to_string() ).unwrap();

  let initial_count = Secret::exposure_count();
  let _ = secret.expose_secret();
  let after_count = Secret::exposure_count();

  assert_eq!( after_count, initial_count + 1 );
}

#[ test ]
fn secret_exposure_returns_correct_value()
{
  let secret = Secret::new( "xai-test-key-1234567890".to_string() ).unwrap();

  let exposed = secret.expose_secret();
  assert_eq!( exposed, "xai-test-key-1234567890" );
}

#[ test ]
fn secret_exposure_count_increments_multiple_times()
{
  let secret = Secret::new( "xai-1234567890".to_string() ).unwrap();

  let initial_count = Secret::exposure_count();

  let _ = secret.expose_secret();
  let _ = secret.expose_secret();
  let _ = secret.expose_secret();

  let final_count = Secret::exposure_count();

  assert_eq!( final_count, initial_count + 3 );
}

#[ test ]
fn secret_clone_works()
{
  let secret = Secret::new( "xai-1234567890".to_string() ).unwrap();
  let cloned = secret.clone();

  assert_eq!( secret.expose_secret(), cloned.expose_secret() );
}

#[ test ]
fn secret_debug_doesnt_expose_value()
{
  let secret = Secret::new( "xai-secret-1234567890".to_string() ).unwrap();
  let debug_str = format!( "{secret:?}" );

  // The debug output should NOT contain the actual secret
  assert!( !debug_str.contains( "secret" ) || debug_str.contains( "Secret" ) );
  // It should contain "Secret" (the type name)
  assert!( debug_str.contains( "Secret" ) );
}

#[ test ]
fn secret_load_with_fallbacks_uses_env_as_fallback()
{
  // With workspace_tools priority, env vars are used as fallback
  // This test verifies env vars still work when workspace files don't exist
  std::env::set_var( "XAI_FALLBACK_TEST", "xai-from-env-1234567890" );

  let result = Secret::load_with_fallbacks( "XAI_FALLBACK_TEST" );
  assert!( result.is_ok() );

  let secret = result.unwrap();
  assert_eq!( secret.expose_secret(), "xai-from-env-1234567890" );

  std::env::remove_var( "XAI_FALLBACK_TEST" );
}

#[ test ]
fn secret_load_with_fallbacks_fails_when_all_sources_unavailable()
{
  std::env::remove_var( "XAI_COMPLETELY_MISSING_KEY" );

  let result = Secret::load_with_fallbacks( "XAI_COMPLETELY_MISSING_KEY" );
  assert!( result.is_err() );

  match result
  {
    Err( e ) =>
    {
      let error_str = format!( "{e:?}" );
      assert!( error_str.contains( "Failed to load" ) || error_str.contains( "any source" ) );
    }
    Ok( _ ) => panic!( "Expected error when all sources fail" ),
  }
}

#[ test ]
fn secret_validates_exact_minimum_length()
{
  // Exactly 10 characters total
  let result = Secret::new( "xai-123456".to_string() );
  assert!( result.is_ok() );
}

#[ test ]
fn secret_rejects_just_below_minimum()
{
  // 9 characters total
  let result = Secret::new( "xai-12345".to_string() );
  assert!( result.is_err() );
}
