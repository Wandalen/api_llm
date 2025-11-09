//! Streaming control tests for the Ollama API client

#![ allow( clippy::std_instead_of_core ) ] // std required for async operations and sync primitives

#[ cfg( feature = "streaming_control" ) ] // Feature gate - not implemented yet
use api_ollama::*;
#[ cfg( feature = "streaming_control" ) ]
use std::time::Duration;
#[ cfg( feature = "streaming_control" ) ]
use std::sync::Arc;
#[ cfg( feature = "streaming_control" ) ]
use tokio::sync::Mutex;

#[ cfg( feature = "streaming_control" ) ]
mod integration_tests
{
  use super::*;

  #[ tokio::test ]
  async fn test_basic_stream_control_creation() -> Result< (), Box< dyn std::error::Error > >
  {
    let control = StreamControl::new();

    assert_eq!( control.state().await, StreamState::Ready );
    assert!( !control.is_paused().await );
    assert!( !control.is_cancelled().await );

    Ok( () )
  }

  #[ tokio::test ]
  async fn test_stream_pause_and_resume() -> Result< (), Box< dyn std::error::Error > >
  {
    let control = StreamControl::new();

    // Start streaming state
    control.start().await?;
    assert_eq!( control.state().await, StreamState::Streaming );

    // Pause the stream
    control.pause().await?;
    assert_eq!( control.state().await, StreamState::Paused );
    assert!( control.is_paused().await );

    // Resume the stream
    control.resume().await?;
    assert_eq!( control.state().await, StreamState::Streaming );
    assert!( !control.is_paused().await );

    Ok( () )
  }

  #[ tokio::test ]
  async fn test_stream_cancellation() -> Result< (), Box< dyn std::error::Error > >
  {
    let control = StreamControl::new();

    // Start streaming
    control.start().await?;
    assert_eq!( control.state().await, StreamState::Streaming );

    // Cancel the stream
    control.cancel().await?;
    assert_eq!( control.state().await, StreamState::Cancelled );
    assert!( control.is_cancelled().await );

    // Once cancelled, should not be able to resume
    let result = control.resume().await;
    assert!( result.is_err() );

    Ok( () )
  }

  #[ tokio::test ]
  async fn test_controlled_stream_integration() -> Result< (), Box< dyn std::error::Error > >
  {
    // Create a test stream from real data structures
    let test_responses : Vec< Result< ChatResponse, String > > = vec![
      Ok( ChatResponse
      {
        message : ChatMessage
        {
          role : MessageRole::Assistant,
          content : "Test response 1".to_string(),
          images : None,
          #[ cfg( feature = "tool_calling" ) ]
          tool_calls : None,
        },
        done : false,
        done_reason : None,
        model : None,
        created_at : None,
        total_duration : None,
        load_duration : None,
        prompt_eval_count : None,
        prompt_eval_duration : None,
        eval_count : None,
        eval_duration : None,
      } ),
      Ok( ChatResponse
      {
        message : ChatMessage
        {
          role : MessageRole::Assistant,
          content : "Test response 2".to_string(),
          images : None,
          #[ cfg( feature = "tool_calling" ) ]
          tool_calls : None,
        },
        done : true,
        done_reason : None,
        model : None,
        created_at : None,
        total_duration : None,
        load_duration : None,
        prompt_eval_count : None,
        prompt_eval_duration : None,
        eval_count : None,
        eval_duration : None,
      } ),
    ];
    let test_stream = Box::pin( futures_util::stream::iter( test_responses ) );
    let control = StreamControl::new();
    control.start().await?;

    let controlled_stream = ControlledStream::new( test_stream, control );

    // Verify initial state
    assert_eq!( controlled_stream.control().state().await, StreamState::Streaming );

    // Pause the stream
    controlled_stream.control().pause().await?;
    assert!( controlled_stream.control().is_paused().await );

    // Resume the stream
    controlled_stream.control().resume().await?;
    assert!( !controlled_stream.control().is_paused().await );

    Ok( () )
  }

  #[ tokio::test ]
  async fn test_stream_state_notifications() -> Result< (), Box< dyn std::error::Error > >
  {
    let control = StreamControl::new();
    let notifications = Arc::new( Mutex::new( Vec::new() ) );
    let notifications_clone = notifications.clone();

    // Subscribe to state changes
    control.on_state_change( move | old_state, new_state | {
      let notifications = notifications_clone.clone();
      tokio ::spawn( async move {
        notifications.lock().await.push( ( old_state, new_state ) );
      } );
    } ).await;

    // Trigger state changes
    control.start().await?;
    control.pause().await?;
    control.resume().await?;
    control.cancel().await?;

    // Wait for notifications to process
    tokio ::time::sleep( Duration::from_millis( 50 ) ).await;

    let received_notifications = notifications.lock().await;
    assert!( received_notifications.len() >= 3 );

    // Check that we got the expected state transitions
    assert!( received_notifications.iter().any( | ( old, new ) | *old == StreamState::Ready && *new == StreamState::Streaming ) );
    assert!( received_notifications.iter().any( | ( old, new ) | *old == StreamState::Streaming && *new == StreamState::Paused ) );

    Ok( () )
  }

  #[ tokio::test ]
  async fn test_buffer_management_during_pause() -> Result< (), Box< dyn std::error::Error > >
  {
    let control = StreamControl::new();
    let buffer = StreamBuffer::new( 1024 );

    // Start streaming and add data to buffer
    control.start().await?;

    let test_data = "Test streaming data chunk".as_bytes().to_vec();
    buffer.write( test_data.clone() ).await?;

    assert_eq!( buffer.len().await, test_data.len() );

    // Pause stream - buffer should retain data
    control.pause().await?;
    assert_eq!( buffer.len().await, test_data.len() );

    // Resume and read data
    control.resume().await?;
    let read_data = buffer.read( test_data.len() ).await?;
    assert_eq!( read_data, test_data );

    Ok( () )
  }

  #[ tokio::test ]
  async fn test_concurrent_stream_control() -> Result< (), Box< dyn std::error::Error > >
  {
    let control = Arc::new( StreamControl::new() );
    let mut handles = Vec::new();

    control.start().await?;

    // Spawn multiple concurrent control operations
    for i in 0..10
    {
      let control_clone = control.clone();
      let handle = tokio::spawn( async move {
        if i % 2 == 0
        {
          control_clone.pause().await
        } else {
          control_clone.resume().await
        }
      } );
      handles.push( handle );
    }

    // Wait for all operations to complete
    for handle in handles
    {
      handle.await??;
    }

    // Final state should be consistent
    let final_state = control.state().await;
    assert!( matches!( final_state, StreamState::Streaming | StreamState::Paused ) );

    Ok( () )
  }

  #[ tokio::test ]
  async fn test_timeout_handling_for_paused_streams() -> Result< (), Box< dyn std::error::Error > >
  {
    let control = StreamControl::with_timeout( Duration::from_millis( 100 ) );

    control.start().await?;
    control.pause().await?;

    // Wait for timeout to trigger
    tokio ::time::sleep( Duration::from_millis( 150 ) ).await;

    // Stream should be automatically cancelled due to timeout
    assert_eq!( control.state().await, StreamState::Cancelled );

    Ok( () )
  }

  #[ tokio::test ]
  async fn test_invalid_state_transitions() -> Result< (), Box< dyn std::error::Error > >
  {
    let control = StreamControl::new();

    // Cannot pause before starting
    let result = control.pause().await;
    assert!( result.is_err() );

    // Cannot resume before starting
    let result = control.resume().await;
    assert!( result.is_err() );

    // Start stream
    control.start().await?;

    // Cannot start again
    let result = control.start().await;
    assert!( result.is_err() );

    Ok( () )
  }

  #[ tokio::test ]
  async fn test_stream_metrics_collection() -> Result< (), Box< dyn std::error::Error > >
  {
    let control = StreamControl::new();

    control.start().await?;

    // Pause and resume to generate metrics
    control.pause().await?;
    tokio ::time::sleep( Duration::from_millis( 50 ) ).await;
    control.resume().await?;
    tokio ::time::sleep( Duration::from_millis( 50 ) ).await;
    control.pause().await?;

    let metrics = control.get_metrics().await;
    assert!( metrics.pause_count > 0 );
    assert!( metrics.resume_count > 0 );
    assert!( metrics.total_pause_duration > Duration::from_millis( 0 ) );

    Ok( () )
  }

  #[ tokio::test ]
  async fn test_memory_cleanup_on_cancel() -> Result< (), Box< dyn std::error::Error > >
  {
    let control = StreamControl::new();
    let buffer = StreamBuffer::new( 1024 );

    // Add data to buffer
    let test_data = vec![ 1u8; 512 ];
    buffer.write( test_data ).await?;
    assert_eq!( buffer.len().await, 512 );

    // Cancel stream and trigger cleanup
    control.start().await?;
    control.cancel().await?;
    control.cleanup_on_cancel( &buffer ).await?;

    // After cancellation, buffer should be cleared
    assert_eq!( buffer.len().await, 0 );

    Ok( () )
  }
}

#[ cfg( feature = "streaming_control" ) ]
mod unit_tests
{
  use super::*;

  #[ test ]
  fn test_stream_state_enum()
  {
    let ready = StreamState::Ready;
    let streaming = StreamState::Streaming;
    let paused = StreamState::Paused;
    let cancelled = StreamState::Cancelled;

    assert_ne!( ready, streaming );
    assert_ne!( streaming, paused );
    assert_ne!( paused, cancelled );

    // Test Display implementation
    assert_eq!( format!( "{ready}" ), "Ready" );
    assert_eq!( format!( "{streaming}" ), "Streaming" );
    assert_eq!( format!( "{paused}" ), "Paused" );
    assert_eq!( format!( "{cancelled}" ), "Cancelled" );
  }

  #[ test ]
  fn test_stream_control_error_types()
  {
    let invalid_transition = StreamControlError::InvalidStateTransition {
      from : StreamState::Ready,
      to : StreamState::Paused,
    };

    let timeout_error = StreamControlError::TimeoutError;
    let buffer_overflow = StreamControlError::BufferOverflow { limit : 1024 };

    assert!( format!( "{invalid_transition}" ).contains( "Ready" ) );
    assert!( format!( "{timeout_error}" ).contains( "timed out" ) );
    assert!( format!( "{buffer_overflow}" ).contains( "1024" ) );
  }

  #[ test ]
  fn test_stream_metrics_calculation()
  {
    let mut metrics = StreamMetrics::new();

    metrics.record_pause();
    metrics.record_resume();
    metrics.record_pause();

    assert_eq!( metrics.pause_count, 2 );
    assert_eq!( metrics.resume_count, 1 );
  }

  #[ test ]
  fn test_buffer_capacity_limits()
  {
    let buffer = StreamBuffer::new( 10 );
    assert_eq!( buffer.capacity(), 10 );

    let large_buffer = StreamBuffer::new( 1024 * 1024 );
    assert_eq!( large_buffer.capacity(), 1024 * 1024 );
  }

  #[ test ]
  fn test_controlled_stream_wrapper()
  {
    let control = StreamControl::new();
    let stream = core::pin::Pin::from( Box::new( futures_util::stream::empty::< Result< ChatResponse, String > >() ) );
    let controlled = ControlledStream::new( stream, control );

    assert!( !controlled.is_paused_sync() );
    assert!( !controlled.is_cancelled_sync() );
  }
}
