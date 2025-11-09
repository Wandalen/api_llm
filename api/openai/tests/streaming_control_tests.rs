//! Streaming Control Tests
//!
//! Comprehensive test suite for streaming control functionality including:
//! - Cancellation token operations
//! - Stream control state management
//! - Control command channels
//! - Timeout and combination scenarios
//! - Performance validation

#![ allow( clippy::unreadable_literal ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::useless_vec ) ]
#![ allow( clippy::unused_async ) ]
#![ allow( clippy::must_use_candidate ) ]
#![ allow( clippy::missing_panics_doc ) ]
#![ allow( clippy::missing_errors_doc ) ]
#![ allow( clippy::doc_markdown ) ]

#[ cfg( test ) ]
mod streaming_control_tests
{
  use api_openai::streaming_control::*;
  use std::time::Duration;
  use tokio::time;

  // ===== CANCELLATION TOKEN TESTS =====

  #[ tokio::test ]
  async fn test_cancellation_token_creation()
  {
    let token = CancellationToken::new();
    assert!( !token.is_cancelled() );

    let token_default = CancellationToken::default();
    assert!( !token_default.is_cancelled() );
  }

  #[ tokio::test ]
  async fn test_cancellation_token_cancel()
  {
    let token = CancellationToken::new();
    assert!( !token.is_cancelled() );

    token.cancel();
    assert!( token.is_cancelled() );
  }

  #[ tokio::test ]
  async fn test_cancellation_token_clone()
  {
    let token = CancellationToken::new();
    let cloned = token.clone();

    assert!( !token.is_cancelled() );
    assert!( !cloned.is_cancelled() );

    token.cancel();
    assert!( token.is_cancelled() );
    assert!( cloned.is_cancelled() ); // Should share state
  }

  #[ tokio::test ]
  async fn test_cancellation_token_wait_immediate()
  {
    let token = CancellationToken::new();
    token.cancel();

    let result = token.wait_for_cancellation( Duration::from_millis( 100 ) ).await;
    assert!( result );
  }

  #[ tokio::test ]
  async fn test_cancellation_token_wait_timeout()
  {
    let token = CancellationToken::new();

    let result = token.wait_for_cancellation( Duration::from_millis( 50 ) ).await;
    assert!( !result ); // Should timeout without cancellation
  }

  #[ tokio::test ]
  async fn test_cancellation_token_wait_during()
  {
    let token = CancellationToken::new();
    let token_clone = token.clone();

    // Cancel after a short delay
    tokio ::spawn( async move
    {
      time ::sleep( Duration::from_millis( 25 ) ).await;
      token_clone.cancel();
    });

    let result = token.wait_for_cancellation( Duration::from_millis( 100 ) ).await;
    assert!( result );
  }

  // ===== STREAM CONTROL TESTS =====

  #[ tokio::test ]
  async fn test_stream_control_creation()
  {
    let control = StreamControl::new();
    assert!( matches!( control.state(), StreamState::Running ) );
    assert!( !control.cancellation_token().is_cancelled() );
    assert!( control.is_active() );

    let control_default = StreamControl::default();
    assert!( matches!( control_default.state(), StreamState::Running ) );
  }

  #[ tokio::test ]
  async fn test_stream_control_state_management()
  {
    let mut control = StreamControl::new();

    // Test state transitions
    control.set_state( StreamState::Paused );
    assert!( matches!( control.state(), StreamState::Paused ) );
    assert!( control.is_active() );

    control.set_state( StreamState::Completed );
    assert!( matches!( control.state(), StreamState::Completed ) );
    assert!( !control.is_active() );

    control.set_state( StreamState::Error( "Test error".to_string() ) );
    assert!( matches!( control.state(), StreamState::Error( _ ) ) );
    assert!( !control.is_active() );
  }

  #[ tokio::test ]
  async fn test_stream_control_cancel()
  {
    let mut control = StreamControl::new();
    assert!( control.is_active() );
    assert!( !control.cancellation_token().is_cancelled() );

    control.cancel();
    assert!( !control.is_active() );
    assert!( control.cancellation_token().is_cancelled() );
    assert!( matches!( control.state(), StreamState::Cancelled ) );
  }

  #[ tokio::test ]
  async fn test_stream_control_elapsed_time()
  {
    let control = StreamControl::new();
    time ::sleep( Duration::from_millis( 10 ) ).await;

    let elapsed = control.elapsed();
    assert!( elapsed >= Duration::from_millis( 10 ) );
    assert!( elapsed < Duration::from_millis( 100 ) ); // Reasonable upper bound
  }

  // ===== STREAM STATE TESTS =====

  #[ tokio::test ]
  async fn test_stream_state_serialization()
  {
    let states = vec![
      StreamState::Running,
      StreamState::Paused,
      StreamState::Cancelled,
      StreamState::Completed,
      StreamState::Error( "Test error".to_string() ),
    ];

    for state in states
    {
      // Test serialization
      let serialized = serde_json::to_string( &state ).expect( "Failed to serialize state" );
      assert!( !serialized.is_empty() );

      // Test deserialization
      let deserialized : StreamState = serde_json::from_str( &serialized )
        .expect( "Failed to deserialize state" );

      assert_eq!( state, deserialized );
    }
  }

  #[ tokio::test ]
  async fn test_stream_state_equality()
  {
    assert_eq!( StreamState::Running, StreamState::Running );
    assert_eq!( StreamState::Paused, StreamState::Paused );
    assert_ne!( StreamState::Running, StreamState::Paused );

    let error1 = StreamState::Error( "Error 1".to_string() );
    let error2 = StreamState::Error( "Error 1".to_string() );
    let error3 = StreamState::Error( "Error 2".to_string() );

    assert_eq!( error1, error2 );
    assert_ne!( error1, error3 );
  }

  // ===== CONFIG TESTS =====

  #[ tokio::test ]
  async fn test_stream_control_config_defaults()
  {
    let config = StreamControlConfig::default();

    assert_eq!( config.max_pause_duration, Duration::from_secs( 300 ) );
    assert_eq!( config.pause_buffer_size, 1024 * 1024 );
    assert_eq!( config.control_timeout, Duration::from_secs( 5 ) );
  }

  #[ tokio::test ]
  async fn test_stream_control_config_serialization()
  {
    let config = StreamControlConfig::default();

    // Test serialization
    let serialized = serde_json::to_string( &config ).expect( "Failed to serialize config" );
    assert!( !serialized.is_empty() );

    // Test deserialization
    let deserialized : StreamControlConfig = serde_json::from_str( &serialized )
      .expect( "Failed to deserialize config" );

    assert_eq!( config.max_pause_duration, deserialized.max_pause_duration );
    assert_eq!( config.pause_buffer_size, deserialized.pause_buffer_size );
    assert_eq!( config.control_timeout, deserialized.control_timeout );
  }

  // ===== MANAGER UTILITY TESTS =====

  #[ tokio::test ]
  async fn test_with_cancellation_success()
  {
    let token = CancellationToken::new();

    let result = StreamControlManager::with_cancellation( token, || async { 42 } ).await;

    assert!( result.is_ok() );
    assert_eq!( result.unwrap(), 42 );
  }

  #[ tokio::test ]
  async fn test_with_cancellation_cancelled()
  {
    let token = CancellationToken::new();
    let token_clone = token.clone();

    // Cancel immediately
    token.cancel();

    let result = StreamControlManager::with_cancellation( token_clone, || async
    {
      time ::sleep( Duration::from_millis( 100 ) ).await;
      42
    }).await;

    assert!( result.is_err() );
    assert_eq!( result.unwrap_err(), "Operation was cancelled" );
  }

  #[ tokio::test ]
  async fn test_with_cancellation_during_operation()
  {
    let token = CancellationToken::new();
    let token_clone = token.clone();

    // Cancel after a short delay
    tokio ::spawn( async move
    {
      time ::sleep( Duration::from_millis( 25 ) ).await;
      token.cancel();
    });

    let result = StreamControlManager::with_cancellation( token_clone, || async
    {
      time ::sleep( Duration::from_millis( 100 ) ).await;
      42
    }).await;

    assert!( result.is_err() );
    assert_eq!( result.unwrap_err(), "Operation was cancelled" );
  }

  #[ tokio::test ]
  async fn test_timeout_token()
  {
    let token = StreamControlManager::create_timeout_token( Duration::from_millis( 50 ) );

    assert!( !token.is_cancelled() );

    // Wait for the timeout to trigger with some buffer for scheduling delays
    time ::sleep( Duration::from_millis( 100 ) ).await;

    // Retry mechanism to handle timing variations
    let mut attempts = 0;
    while !token.is_cancelled() && attempts < 5
    {
      time ::sleep( Duration::from_millis( 10 ) ).await;
      attempts += 1;
    }

    assert!( token.is_cancelled(), "Token should be cancelled after timeout" );
  }

  #[ tokio::test ]
  async fn test_combine_tokens()
  {
    let token1 = CancellationToken::new();
    let token2 = CancellationToken::new();
    let token3 = CancellationToken::new();

    let combined = StreamControlManager::combine_tokens( vec![ token1.clone(), token2.clone(), token3.clone() ] );

    assert!( !combined.is_cancelled() );

    // Cancel one token
    token2.cancel();

    // Wait for combination to propagate
    time ::sleep( Duration::from_millis( 20 ) ).await;

    assert!( combined.is_cancelled() );
  }

  // ===== CONTROL CHANNEL TESTS =====

  #[ tokio::test ]
  async fn test_control_channel_creation()
  {
    let ( sender, mut receiver ) = StreamControlManager::create_control_channel();

    // Test that we can send commands
    assert!( sender.pause().is_ok() );
    assert!( sender.resume().is_ok() );
    assert!( sender.cancel().is_ok() );

    // Test that we can receive commands
    assert!( matches!( receiver.try_recv(), Some( StreamControlCommand::Pause ) ) );
    assert!( matches!( receiver.try_recv(), Some( StreamControlCommand::Resume ) ) );
    assert!( matches!( receiver.try_recv(), Some( StreamControlCommand::Cancel ) ) );
    assert!( receiver.try_recv().is_none() ); // Should be empty now
  }

  #[ tokio::test ]
  async fn test_control_channel_async_recv()
  {
    let ( sender, mut receiver ) = StreamControlManager::create_control_channel();

    // Send command after a delay
    tokio ::spawn( async move
    {
      time ::sleep( Duration::from_millis( 25 ) ).await;
      let _ = sender.pause();
    });

    // Receive asynchronously
    let command = receiver.recv().await;
    assert!( matches!( command, Some( StreamControlCommand::Pause ) ) );
  }

  #[ tokio::test ]
  async fn test_control_command_equality()
  {
    assert_eq!( StreamControlCommand::Pause, StreamControlCommand::Pause );
    assert_eq!( StreamControlCommand::Resume, StreamControlCommand::Resume );
    assert_eq!( StreamControlCommand::Cancel, StreamControlCommand::Cancel );

    assert_ne!( StreamControlCommand::Pause, StreamControlCommand::Resume );
    assert_ne!( StreamControlCommand::Resume, StreamControlCommand::Cancel );
    assert_ne!( StreamControlCommand::Cancel, StreamControlCommand::Pause );
  }

  // ===== INTEGRATION TESTS =====

  #[ tokio::test ]
  async fn test_streaming_control_integration()
  {
    let mut control = StreamControl::new();
    let ( sender, mut receiver ) = StreamControlManager::create_control_channel();

    // Simulate receiving pause command
    sender.pause().expect( "Failed to send pause command" );
    let command = receiver.try_recv();
    assert!( matches!( command, Some( StreamControlCommand::Pause ) ) );

    // Update control state based on command
    control.set_state( StreamState::Paused );
    assert!( matches!( control.state(), StreamState::Paused ) );
    assert!( control.is_active() );

    // Cancel the control
    control.cancel();
    assert!( !control.is_active() );
    assert!( control.cancellation_token().is_cancelled() );
  }

  #[ tokio::test ]
  async fn test_performance_many_tokens()
  {
    let start = std::time::Instant::now();

    // Create many tokens quickly
    let mut tokens = Vec::new();
    for _ in 0..1000
    {
      tokens.push( CancellationToken::new() );
    }

    // Cancel all tokens
    for token in &tokens
    {
      token.cancel();
    }

    // Verify all are cancelled
    for token in &tokens
    {
      assert!( token.is_cancelled() );
    }

    let duration = start.elapsed();
    assert!( duration < Duration::from_millis( 100 ) ); // Should be very fast
  }

  #[ tokio::test ]
  async fn test_memory_cleanup()
  {
    // Create and drop many controls to test memory cleanup
    for _ in 0..100
    {
      let control = StreamControl::new();
      let token = control.cancellation_token().clone();
      drop( control );

      // Token should still work
      assert!( !token.is_cancelled() );
      token.cancel();
      assert!( token.is_cancelled() );
    }
  }
}