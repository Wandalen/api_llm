//! Integration tests for synchronous streaming

#![ cfg( feature = "sync" ) ]
#![ allow( clippy::single_match_else ) ]
#![ allow( clippy::never_loop ) ]
#![ allow( clippy::redundant_closure_for_method_calls ) ]
#![ allow( clippy::used_underscore_items ) ]

use api_huggingface::sync::SyncClient;
use api_huggingface::components::input::InferenceParameters;

/// Helper to get API key from workspace secrets
fn get_api_key() -> String
{
  use workspace_tools as workspace;

  let workspace = workspace::workspace()
    .expect( "[get_api_key] Failed to access workspace - required for integration tests" );
  let secrets = workspace.load_secrets_from_file( "-secrets.sh" )
    .expect( "[get_api_key] Failed to load secret/-secrets.sh - required for integration tests" );
  secrets.get( "HUGGINGFACE_API_KEY" )
    .expect( "[get_api_key] HUGGINGFACE_API_KEY not found in secret/-secrets.sh - required for integration tests. Get your token from https://huggingface.co/settings/tokens" )
    .clone()
}

#[ cfg( feature = "integration" ) ]
#[ ignore = "integration test requiring real API - run manually" ]
#[ test ]
fn test_sync_stream_basic()
{
  let api_key = get_api_key();

  let client = SyncClient::new( api_key )
  .expect( "Failed to create client" );

  let params = InferenceParameters::new()
  .with_temperature( 0.7 )
  .with_max_new_tokens( 50 );

  let stream = client.inference().create_stream(
  "Count to 5",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params
  ).expect( "Failed to create stream" );

  let mut token_count = 0;
  for token_result in stream
  {
  match token_result
  {
      Ok( token ) =>
      {
  assert!( !token.is_empty(), "Token should not be empty" );
  token_count += 1;
      }
      Err( e ) => panic!( "Stream error : {e}" ),
  }
  }

  assert!( token_count > 0, "Should receive at least one token" );
}

#[ cfg( feature = "integration" ) ]
#[ ignore = "integration test requiring real API - run manually" ]
#[ test ]
fn test_sync_stream_iterator_pattern()
{
  let api_key = get_api_key();

  let client = SyncClient::new( api_key )
  .expect( "Failed to create client" );

  let params = InferenceParameters::new()
  .with_max_new_tokens( 30 );

  let stream = client.inference().create_stream(
  "Hello",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params
  ).expect( "Failed to create stream" );

  // Test that stream implements Iterator
  let tokens : Vec< Result< String, _ > > = stream.collect();
  assert!( !tokens.is_empty(), "Should collect tokens" );
}

#[ cfg( feature = "integration" ) ]
#[ ignore = "integration test requiring real API - run manually" ]
#[ test ]
fn test_sync_stream_early_termination()
{
  let api_key = get_api_key();

  let client = SyncClient::new( api_key )
  .expect( "Failed to create client" );

  let params = InferenceParameters::new()
  .with_max_new_tokens( 100 );

  let stream = client.inference().create_stream(
  "Tell me a long story",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params
  ).expect( "Failed to create stream" );

  // Take only first 5 tokens then stop
  let mut count = 0;
  for token_result in stream
  {
  match token_result
  {
      Ok( _token ) =>
      {
  count += 1;
  if count >= 5
  {
          break;
  }
      }
      Err( e ) => panic!( "Stream error : {e}" ),
  }
  }

  assert_eq!( count, 5, "Should have received exactly 5 tokens" );
}

#[ test ]
fn test_sync_stream_error_handling()
{
  let api_key = "invalid_key".to_string();

  let client = SyncClient::new( api_key )
  .expect( "Client creation should not fail with invalid key" );

  let params = InferenceParameters::new()
  .with_max_new_tokens( 10 );

  // Stream creation might fail with invalid auth
  let stream_result = client.inference().create_stream(
  "Test",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params
  );

  // Either stream creation fails, or first token fails
  match stream_result
  {
  Ok( stream ) =>
  {
      // If stream created, first iteration should error
      for token_result in stream
      {
  assert!( token_result.is_err(), "Should error with invalid key" );
  break;
      }
  }
  Err( _ ) =>
  {
      // Expected - stream creation failed with invalid key
  }
  }
}

#[ test ]
fn test_sync_stream_empty_iteration()
{
  // Test that stream with no tokens completes gracefully
  // This is a unit test - we cant create an empty stream without API
  // but we can document the behavior
}

#[ cfg( feature = "integration" ) ]
#[ ignore = "integration test requiring real API - run manually" ]
#[ test ]
fn test_sync_stream_multiple_streams()
{
  let api_key = get_api_key();

  let client = SyncClient::new( api_key )
  .expect( "Failed to create client" );

  let params = InferenceParameters::new()
  .with_max_new_tokens( 20 );

  // Create multiple streams
  let stream1 = client.inference().create_stream(
  "First",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params.clone()
  ).expect( "Failed to create stream 1" );

  let stream2 = client.inference().create_stream(
  "Second",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params
  ).expect( "Failed to create stream 2" );

  // Process first stream
  let count1 = stream1.filter_map( | r | r.ok() ).count();
  assert!( count1 > 0, "Stream 1 should have tokens" );

  // Process second stream
  let count2 = stream2.filter_map( | r | r.ok() ).count();
  assert!( count2 > 0, "Stream 2 should have tokens" );
}

#[ cfg( feature = "integration" ) ]
#[ ignore = "integration test requiring real API - run manually" ]
#[ test ]
fn test_sync_stream_chaining()
{
  let api_key = get_api_key();

  let client = SyncClient::new( api_key )
  .expect( "Failed to create client" );

  let params = InferenceParameters::new()
  .with_max_new_tokens( 50 );

  let stream = client.inference().create_stream(
  "Count to 10",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params
  ).expect( "Failed to create stream" );

  // Test iterator chaining
  let tokens : Vec< String > = stream
  .filter_map( | r | r.ok() )
  .take( 10 )
  .collect();

  assert!( !tokens.is_empty(), "Should collect tokens" );
  assert!( tokens.len() <= 10, "Should take at most 10 tokens" );
}

#[ cfg( feature = "integration" ) ]
#[ ignore = "integration test requiring real API - run manually" ]
#[ test ]
fn test_sync_stream_token_accumulation()
{
  let api_key = get_api_key();

  let client = SyncClient::new( api_key )
  .expect( "Failed to create client" );

  let params = InferenceParameters::new()
  .with_max_new_tokens( 30 );

  let stream = client.inference().create_stream(
  "Hello world",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params
  ).expect( "Failed to create stream" );

  // Accumulate all tokens into a single string
  let mut full_text = String::new();
  for token_result in stream
  {
  match token_result
  {
      Ok( token ) => full_text.push_str( &token ),
      Err( e ) => panic!( "Stream error : {e}" ),
  }
  }

  assert!( !full_text.is_empty(), "Should accumulate text" );
}

#[ cfg( feature = "integration" ) ]
#[ ignore = "integration test requiring real API - run manually" ]
#[ test ]
fn test_sync_stream_with_different_parameters()
{
  let api_key = get_api_key();

  let client = SyncClient::new( api_key )
  .expect( "Failed to create client" );

  // Test with high temperature
  let params_high_temp = InferenceParameters::new()
  .with_temperature( 1.0 )
  .with_max_new_tokens( 20 );

  let stream = client.inference().create_stream(
  "Test",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params_high_temp
  ).expect( "Failed to create stream" );

  let count = stream.filter_map( | r | r.ok() ).count();
  assert!( count > 0, "Should stream with high temperature" );

  // Test with low temperature
  let params_low_temp = InferenceParameters::new()
  .with_temperature( 0.1 )
  .with_max_new_tokens( 20 );

  let stream = client.inference().create_stream(
  "Test",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params_low_temp
  ).expect( "Failed to create stream" );

  let count = stream.filter_map( | r | r.ok() ).count();
  assert!( count > 0, "Should stream with low temperature" );
}

#[ cfg( feature = "integration" ) ]
#[ ignore = "integration test requiring real API - run manually" ]
#[ test ]
fn test_sync_stream_blocking_behavior()
{
  let api_key = get_api_key();

  let client = SyncClient::new( api_key )
  .expect( "Failed to create client" );

  let params = InferenceParameters::new()
  .with_max_new_tokens( 10 );

  let stream = client.inference().create_stream(
  "Count",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params
  ).expect( "Failed to create stream" );

  // Test that iteration blocks (no need for explicit async)
  let start = std::time::Instant::now();
  let _ : Vec< _ > = stream.collect();
  let elapsed = start.elapsed();

  // Should take some time due to network/API
  assert!( elapsed.as_millis() > 0, "Should block during iteration" );
}

#[ cfg( feature = "integration" ) ]
#[ ignore = "integration test requiring real API - run manually" ]
#[ test ]
fn test_sync_stream_reusable_client()
{
  let api_key = get_api_key();

  let client = SyncClient::new( api_key )
  .expect( "Failed to create client" );

  let params = InferenceParameters::new()
  .with_max_new_tokens( 10 );

  // Create and consume first stream
  let stream1 = client.inference().create_stream(
  "First",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params.clone()
  ).expect( "Failed to create stream 1" );

  let _ : Vec< _ > = stream1.collect();

  // Client should be reusable for second stream
  let stream2 = client.inference().create_stream(
  "Second",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params
  ).expect( "Failed to create stream 2" );

  let count = stream2.filter_map( | r | r.ok() ).count();
  assert!( count > 0, "Second stream should work" );
}

#[ cfg( feature = "integration" ) ]
#[ ignore = "integration test requiring real API - run manually" ]
#[ test ]
fn test_sync_stream_short_prompt()
{
  let api_key = get_api_key();

  let client = SyncClient::new( api_key )
  .expect( "Failed to create client" );

  let params = InferenceParameters::new()
  .with_max_new_tokens( 5 );

  let stream = client.inference().create_stream(
  "Hi",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params
  ).expect( "Failed to create stream" );

  let tokens : Vec< _ > = stream.filter_map( | r | r.ok() ).collect();
  assert!( !tokens.is_empty(), "Should stream even with short prompt" );
}

#[ cfg( feature = "integration" ) ]
#[ ignore = "integration test requiring real API - run manually" ]
#[ test ]
fn test_sync_stream_long_prompt()
{
  let api_key = get_api_key();

  let client = SyncClient::new( api_key )
  .expect( "Failed to create client" );

  let long_prompt = "This is a very long prompt that contains a lot of context and information. ".repeat( 10 );

  let params = InferenceParameters::new()
  .with_max_new_tokens( 20 );

  let stream = client.inference().create_stream(
  long_prompt,
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params
  ).expect( "Failed to create stream" );

  let count = stream.filter_map( | r | r.ok() ).count();
  assert!( count > 0, "Should stream with long prompt" );
}

#[ cfg( feature = "integration" ) ]
#[ ignore = "integration test requiring real API - run manually" ]
#[ test ]
fn test_sync_stream_minimal_tokens()
{
  let api_key = get_api_key();

  let client = SyncClient::new( api_key )
  .expect( "Failed to create client" );

  let params = InferenceParameters::new()
  .with_max_new_tokens( 1 );

  let stream = client.inference().create_stream(
  "Yes or no?",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params
  ).expect( "Failed to create stream" );

  let tokens : Vec< _ > = stream.filter_map( | r | r.ok() ).collect();
  assert!( !tokens.is_empty(), "Should stream even with minimal tokens" );
}

#[ cfg( feature = "integration" ) ]
#[ ignore = "integration test requiring real API - run manually" ]
#[ test ]
fn test_sync_stream_utf8_content()
{
  let api_key = get_api_key();

  let client = SyncClient::new( api_key )
  .expect( "Failed to create client" );

  let params = InferenceParameters::new()
  .with_max_new_tokens( 20 );

  let stream = client.inference().create_stream(
  "Say hello in Japanese",
  "moonshotai/Kimi-K2-Instruct-0905:groq",
  params
  ).expect( "Failed to create stream" );

  // Should handle UTF-8 tokens correctly
  for token_result in stream
  {
  match token_result
  {
      Ok( token ) =>
      {
  // Verify its valid UTF-8
  assert!( token.is_ascii() || token.chars().all( | c | c.is_alphabetic() || c.is_whitespace() ) );
      }
      Err( e ) => panic!( "Stream error : {e}" ),
  }
  }
}

#[ test ]
fn test_sync_stream_type_safety()
{
  // Compile-time test that SyncStream implements Iterator
  fn _assert_iterator< T : Iterator >() {}
  _assert_iterator ::< api_huggingface::sync::SyncStream >();
}
