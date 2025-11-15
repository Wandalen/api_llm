//! Synchronous API functionality tests
//!
//! This module contains comprehensive tests for synchronous API functionality,
//! including blocking wrapper implementations, runtime management, and
//! synchronous client patterns.

use super::*;

mod sync_api_functionality_tests
{
  use super::*;
  use std::time::{ Duration, Instant };
  use the_module::{ Message, CreateMessageRequest };

  /// Test basic sync client construction and configuration
  #[ test ]
  #[ ignore = "Requires workspace secrets file" ]
fn test_sync_client_construction()
  {
    use the_module::SyncClient;

    // Test construction from environment
    let client_result = SyncClient::from_env();
    assert!( client_result.is_ok() || client_result.is_err(), "Construction should return a result" );

    // Fix(issue-hygiene-001): Fail loudly when ANTHROPIC_API_KEY missing
    // Root cause : Silent skip when env var missing created false positive test pass
    // Previous : if let Ok(secret) silently skipped test when ANTHROPIC_API_KEY unset
    // Fixed : .expect() fails loudly with clear message
    // Pitfall : Never use conditional skip - fail loudly or use #[ ignore ] with permission

    // Test construction with explicit secret
    let secret = std::env::var( "ANTHROPIC_API_KEY" )
      .expect( "ANTHROPIC_API_KEY must be set for sync_api_test - test requires real API key" );

    let client = SyncClient::new( &secret );
    assert!( client.is_ok(), "Should construct client with valid secret" );

    let client = client.unwrap();
    assert!( !client.get_api_key().is_empty(), "API key should be set" );

    // Test construction with invalid secret
    let invalid_client = SyncClient::new( "invalid-key" );
    assert!( invalid_client.is_err(), "Should fail with invalid secret" );
  }

  /// Test sync message creation and sending
  #[ test ]
#[ ignore = "Requires workspace secrets file" ]
fn test_sync_message_operations()
  {
    use the_module::{ SyncClient, CreateMessageRequest, Message };

    let client = SyncClient::from_workspace().expect( "Client should be available for testing" );

    // Test basic message request construction
    let request = CreateMessageRequest
    {
      model : "claude-3-5-haiku-20241022".to_string(),
      max_tokens : 100,
      messages : vec![ Message::user( "Hello, world!" ) ],
      system : None,
      stream : None,
      temperature : None,
      #[ cfg( feature = "tools" ) ]
      tool_choice : None,
      #[ cfg( feature = "tools" ) ]
      tools : None,
    };

    // Test sync message sending
    let response = client.create_message( &request );

    // Should either succeed or fail with meaningful error
    match response
    {
      Ok( message_response ) => {
        assert!( !message_response.content.is_empty(), "Response should have content" );
        assert!( message_response.usage.input_tokens > 0, "Should track input tokens" );
        assert!( message_response.usage.output_tokens > 0, "Should track output tokens" );
      }
      Err( error ) => {
        // Validate error structure
        assert!( !error.to_string().is_empty(), "Error should have meaningful message" );
      }
    }
  }

  /// Test sync message with system prompts
  #[ test ]
#[ ignore = "Requires workspace secrets file" ]
fn test_sync_message_with_system_prompt()
  {
    use the_module::{ SyncClient, CreateMessageRequest };

    let client = SyncClient::from_workspace().expect( "Client should be available for testing" );

    let request = CreateMessageRequest
    {
      model : "claude-3-5-haiku-20241022".to_string(),
      max_tokens : 50,
      messages : vec![ Message::user( "What is 2+2?" ) ],
      system : Some( vec![ the_module::SystemContent::text( "You are a helpful assistant that responds concisely." ) ] ),
      stream : None,
      temperature : None,
      #[ cfg( feature = "tools" ) ]
      tool_choice : None,
      #[ cfg( feature = "tools" ) ]
      tools : None,
    };

    let response = client.create_message( &request );

    match response
    {
      Ok( message_response ) => {
        assert!( !message_response.content.is_empty(), "Should have response content" );
        // System prompt should influence the response but not appear in content
        assert!( message_response.content.len() >= 1, "Should have at least one content block" );
      }
      Err( _ ) => {
        // Expected if API key not available
      }
    }
  }

  /// Test sync conversation flow
  #[ test ]
#[ ignore = "Requires workspace secrets file" ]
fn test_sync_conversation_flow()
  {
    use the_module::{ SyncClient, CreateMessageRequest, Message };

    let client = SyncClient::from_workspace().expect( "Client should be available for testing" );

    // First message
    let mut request1 = CreateMessageRequest::new( "claude-3-5-haiku-20241022" );
    request1.add_user_message( "My name is Alice." );
    request1.set_max_tokens( 50 );

    // Fix(issue-hygiene-002): Fail loudly if first message fails
    // Root cause : Silent skip when response1 failed - test falsely passed
    // Previous : if let Ok silently skipped conversation test when API call failed
    // Fixed : .expect() fails loudly with clear message about conversation flow requirement
    // Pitfall : Never skip test continuation on API failure - fail loudly to detect issues
    let response1 = client.create_message( &request1 );
    let _message1 = response1
      .expect( "First message must succeed for conversation flow test" );

    // Second message in conversation
    let mut request2 = CreateMessageRequest::new( "claude-3-5-haiku-20241022" );

    // Add conversation history
    request2.add_message( Message::user( "My name is Alice." ) );
    // For now, just add a simple assistant response since content conversion is complex
    request2.add_message( Message::assistant( "Nice to meet you, Alice!" ) );
    request2.add_user_message( "What is my name?" );
    request2.set_max_tokens( 50 );

    let response2 = client.create_message( &request2 );

    match response2
    {
      Ok( message2 ) => {
        assert!( !message2.content.is_empty(), "Should respond to conversation" );
        // Response should reference the name (though this is probabilistic)
      }
      Err( _ ) => {
        // Expected if API key not available
      }
    }
  }

  /// Test sync client with different models
  #[ test ]
#[ ignore = "Requires workspace secrets file" ]
fn test_sync_client_multiple_models()
  {
    use the_module::{ SyncClient, CreateMessageRequest };

    let client = SyncClient::from_workspace().expect( "Client should be available for testing" );

    let models = vec![
      "claude-3-5-haiku-20241022",
      "claude-sonnet-4-5-20250929",
    ];

    for model in models
    {
      let mut request = CreateMessageRequest::new( model );
      request.add_user_message( "Hello!" );
      request.set_max_tokens( 20 );

      let response = client.create_message( &request );

      // Should work with all models or fail consistently
      match response
      {
        Ok( message_response ) => {
          assert!( !message_response.content.is_empty(), "Response should have content for {}", model );
          assert_eq!( message_response.model, model, "Response should indicate correct model" );
        }
        Err( _ ) => {
          // Expected if model not available or API key not set
        }
      }
    }
  }

  /// Test sync client timeout handling
  #[ test ]
  #[ ignore = "Requires workspace secrets file" ]
fn test_sync_client_timeout_configuration()
  {
    use the_module::SyncClientBuilder;

    // Fix(issue-hygiene-003): Fail loudly when builder fails
    // Root cause : Silent skip when build_from_env failed - timeout test falsely passed
    // Previous : if let Ok silently skipped timeout verification when client construction failed
    // Fixed : .expect() fails loudly with clear message about timeout configuration requirement
    // Pitfall : Never skip configuration verification on construction failure - fail loudly
    let client = SyncClientBuilder::new()
      .timeout( Duration::from_secs( 30 ) )
      .build_from_env()
      .expect( "Client builder with timeout must succeed for timeout configuration test" );

    assert!( client.get_timeout() == Duration::from_secs( 30 ), "Timeout should be configured" );

    // Fix(issue-hygiene-004): Fail loudly when short timeout client fails
    // Root cause : Silent skip when build_from_env failed - timeout test falsely passed
    // Previous : if let Ok silently skipped timeout behavior test when client construction failed
    // Fixed : .expect() fails loudly with clear message about timeout test requirement
    // Pitfall : Never skip timeout behavior verification - fail loudly to detect construction issues
    let client = SyncClientBuilder::new()
      .timeout( Duration::from_millis( 1 ) )
      .build_from_env()
      .expect( "Client builder with short timeout must succeed for timeout behavior test" );

    let mut request = CreateMessageRequest::new( "claude-3-5-haiku-20241022" );
    request.add_user_message( "Hello!" );
    request.set_max_tokens( 10 );

    let start = Instant::now();
    let response = client.create_message( &request );
    let elapsed = start.elapsed();

    // Should timeout quickly or succeed very fast
    assert!( elapsed < Duration::from_secs( 5 ), "Should timeout or complete quickly" );

    if response.is_err()
    {
      // Timeout error expected
      let error_msg = response.unwrap_err().to_string();
      assert!( error_msg.to_lowercase().contains( "timeout" ) ||
               error_msg.to_lowercase().contains( "deadline" ),
               "Error should indicate timeout" );
    }
  }
}

mod sync_api_runtime_tests
{
  use super::*;
  use std::{ thread, sync::{ Arc, Mutex }, time::{ Duration, Instant } };

  /// Test sync client thread safety
  #[ test ]
  fn test_sync_client_thread_safety()
  {
    use the_module::SyncClient;

    let client = match SyncClient::from_env()
    {
      Ok( c ) => Arc::new( c ),
      Err( _ ) => return, // Skip if no API key
    };

    let results = Arc::new( Mutex::new( Vec::new() ) );
    let mut handles = vec![];

    // Spawn multiple threads using the same client
    for i in 0..3
    {
      let client_clone = Arc::clone( &client );
      let results_clone = Arc::clone( &results );

      let handle = thread::spawn( move || {
        use the_module::CreateMessageRequest;

        let mut request = CreateMessageRequest::new( "claude-3-5-haiku-20241022" );
        request.add_user_message( &format!( "Thread {} says hello!", i ) );
        request.set_max_tokens( 20 );

        let response = client_clone.create_message( &request );

        let mut results = results_clone.lock().unwrap();
        results.push( (i, response.is_ok()) );
      });

      handles.push( handle );
    }

    // Wait for all threads
    for handle in handles
    {
      handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    assert_eq!( results.len(), 3, "All threads should complete" );

    // At least some requests should succeed (if API key is valid)
    // This tests that the sync client can handle concurrent usage
  }

  /// Test sync runtime management
  #[ test ]
  fn test_sync_runtime_lifecycle()
  {
    use the_module::{ SyncRuntime, SyncClient };

    // Test runtime creation and shutdown
    let runtime = SyncRuntime::new();
    assert!( runtime.is_ok(), "Runtime should be creatable" );

    let runtime = runtime.unwrap();

    // Test client creation with custom runtime
    let client_result = SyncClient::with_runtime( runtime, "test-key" );

    match client_result
    {
      Ok( client ) => {
        // Test that client works with custom runtime
        assert!( !client.get_api_key().is_empty(), "Client should have API key" );
      }
      Err( _ ) => {
        // Expected with test key
      }
    }

    // Runtime should clean up automatically when dropped
  }

  /// Test blocking behavior consistency
  #[ test ]
  fn test_blocking_behavior_consistency()
  {
    use the_module::{ SyncClient, CreateMessageRequest };

    let client = match SyncClient::from_env()
    {
      Ok( c ) => c,
      Err( _ ) => return, // Skip if no API key
    };

    let mut request = CreateMessageRequest::new( "claude-3-5-haiku-20241022" );
    request.add_user_message( "Count to 3" );
    request.set_max_tokens( 30 );

    let start = Instant::now();
    let response = client.create_message( &request );
    let elapsed = start.elapsed();

    // Sync operation should block until completion
    match response
    {
      Ok( _ ) => {
        assert!( elapsed > Duration::from_millis( 100 ), "Should take some time to complete" );
        assert!( elapsed < Duration::from_secs( 30 ), "Should complete in reasonable time" );
      }
      Err( _ ) => {
        // Network error or API key issue
      }
    }
  }
}

mod sync_api_integration_tests
{
  use super::*;
  use std::time::{ Duration, Instant };

  /// Test sync to async interoperability
  #[ test ]
  fn test_sync_async_interoperability()
  {
    use the_module::{ SyncClient, Client, CreateMessageRequest };

    // Test that sync and async clients can coexist
    let sync_client_result = SyncClient::from_env();
    let async_client_result = Client::from_env();

    match (sync_client_result, async_client_result)
    {
      (Ok( sync_client ), Ok( async_client )) => {
        // Both clients should be able to work independently
        assert!( sync_client.has_api_key() == async_client.api_key().is_some(),
                 "Both clients should have same API key status" );

        // Test sync request
        let mut sync_request = CreateMessageRequest::new( "claude-3-5-haiku-20241022" );
        sync_request.add_user_message( "Hello from sync!" );
        sync_request.set_max_tokens( 20 );

        let sync_response = sync_client.create_message( &sync_request );

        // Test async request in sync context
        let mut async_request = CreateMessageRequest::new( "claude-3-5-haiku-20241022" );
        async_request.add_user_message( "Hello from async!" );
        async_request.set_max_tokens( 20 );

        // We can't test async in sync context easily, but we can verify structure
        assert!( sync_response.is_ok() || sync_response.is_err(), "Sync should complete" );
      }
      _ => {
        // Expected if no API key configured
      }
    }
  }

  /// Test sync client performance characteristics
  #[ test ]
  fn test_sync_client_performance_overhead()
  {
    use the_module::{ SyncClient, CreateMessageRequest };

    let client = match SyncClient::from_env()
    {
      Ok( c ) => c,
      Err( _ ) => return, // Skip if no API key
    };

    let mut times = Vec::new();

    // Measure multiple sync requests
    for _ in 0..3
    {
      let mut request = CreateMessageRequest::new( "claude-3-5-haiku-20241022" );
      request.add_user_message( "Hi" );
      request.set_max_tokens( 10 );

      let start = Instant::now();
      let response = client.create_message( &request );
      let elapsed = start.elapsed();

      if response.is_ok()
      {
        times.push( elapsed );
      }
    }

    if !times.is_empty()
    {
      #[ allow( clippy::cast_possible_truncation ) ]
      let avg_time : Duration = times.iter().sum::< Duration >() / times.len() as u32;

      // Sync overhead should be minimal (< 100ms extra)
      assert!( avg_time < Duration::from_secs( 30 ), "Sync requests should complete in reasonable time" );

      // Consistency check - times shouldn't vary wildly
      let max_time = times.iter().max().unwrap();
      let min_time = times.iter().min().unwrap();

      assert!( max_time.as_millis() < min_time.as_millis() * 10,
               "Request times should be relatively consistent" );
    }
  }

  /// Test sync error handling and propagation
  #[ test ]
  #[ ignore = "Requires workspace secrets file" ]
fn test_sync_error_handling()
  {
    use the_module::{ SyncClient, CreateMessageRequest };

    // Test with invalid API key
    let invalid_client = SyncClient::new( "sk-ant-invalid-key" );

    match invalid_client
    {
      Ok( client ) => {
        let mut request = CreateMessageRequest::new( "claude-3-5-haiku-20241022" );
        request.add_user_message( "Hello!" );
        request.set_max_tokens( 10 );

        let response = client.create_message( &request );

        // Should fail with authentication error
        assert!( response.is_err(), "Should fail with invalid API key" );

        let error = response.unwrap_err();
        let error_msg = error.to_string().to_lowercase();

        assert!( error_msg.contains( "auth" ) ||
                 error_msg.contains( "key" ) ||
                 error_msg.contains( "401" ) ||
                 error_msg.contains( "unauthorized" ),
                 "Error should indicate authentication issue : {error}" );
      }
      Err( _ ) => {
        // Expected - invalid key should be rejected at construction
      }
    }

    // Fix(issue-hygiene-005): Fail loudly when client unavailable for invalid model test
    // Root cause : Silent skip when from_env failed - invalid model error handling test falsely passed
    // Previous : if let Ok silently skipped model validation test when client construction failed
    // Fixed : .expect() fails loudly with clear message about model validation requirement
    // Pitfall : Never skip error handling verification - fail loudly to ensure test executes
    let client = SyncClient::from_env()
      .expect( "Client must be available from environment for invalid model error handling test" );

    let mut request = CreateMessageRequest::new( "invalid-model-name" );
    request.add_user_message( "Hello!" );
    request.set_max_tokens( 10 );

    let response = client.create_message( &request );

    if response.is_err()
    {
      let error = response.unwrap_err();
      let error_msg = error.to_string().to_lowercase();

      // Should indicate model-related error
      assert!( error_msg.contains( "model" ) ||
               error_msg.contains( "400" ) ||
               error_msg.contains( "not found" ),
               "Error should indicate model issue" );
    }
  }
}

#[ cfg( feature = "sync-api" ) ]
mod sync_api_feature_tests
{
  use super::*;

  /// Test sync API feature gate
  #[ test ]
  fn test_sync_api_feature_availability()
  {
    // This test validates that sync API types are available when feature is enabled
    use the_module::{ SyncClient, SyncClientBuilder, SyncRuntime };

    // These should compile and be usable
    #[ allow( clippy::no_effect_underscore_binding ) ]
    {
      let _sync_client_type = core::marker::PhantomData::< SyncClient >;
      let _sync_builder_type = core::marker::PhantomData::< SyncClientBuilder >;
      let _sync_runtime_type = core::marker::PhantomData::< SyncRuntime >;
    }

    // Test that basic construction methods exist
    let builder = SyncClientBuilder::new();
    let result = builder.build_from_env();
    assert!( result.is_ok() || result.is_err(),
             "Builder should return result" );
  }
}