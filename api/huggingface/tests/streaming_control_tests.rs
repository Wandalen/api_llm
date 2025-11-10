//! Tests for streaming control functionality
//!
//! Verifies pause, resume, and cancel operations for controlled streams.

#[ cfg( all( test, feature = "streaming-control" ) ) ]
mod tests
{
  use api_huggingface::
  {
    streaming_control::wrap_stream,
    error::{ Result, HuggingFaceError },
  };
  use tokio::sync::mpsc;
  use futures_util::StreamExt;

  /// Helper to create a mock stream for testing
  fn create_mock_stream() -> mpsc::Receiver< Result< String > >
  {
    let ( tx, rx ) = mpsc::channel( 10 );

    tokio::spawn( async move
    {
      for i in 0..10
      {
        if tx.send( Ok( format!( "chunk_{i}" ) ) ).await.is_err()
        {
          break;
        }
        tokio::time::sleep( tokio::time::Duration::from_millis( 50 ) ).await;
      }
    });

    rx
  }

  /// Test basic streaming without control
  #[ tokio::test ]
  async fn test_streaming_no_control()
  {
    let receiver = create_mock_stream();
    let ( mut stream, _control ) = wrap_stream( receiver );

    let mut count = 0;
    while let Some( result ) = stream.next().await
    {
      assert!( result.is_ok() );
      count += 1;
    }

    assert_eq!( count, 10 );
  }

  /// Test pause and resume operations
  #[ tokio::test ]
  async fn test_pause_resume()
  {
    let receiver = create_mock_stream();
    let ( mut stream, control ) = wrap_stream( receiver );

    // Read first 2 chunks
    for _ in 0..2
    {
      assert!( stream.next().await.is_some() );
    }

    // Pause the stream
    control.pause().await.unwrap();

    // Wait a bit (during this time, chunks are buffered)
    tokio::time::sleep( tokio::time::Duration::from_millis( 200 ) ).await;

    // Resume the stream
    control.resume().await.unwrap();

    // Read remaining chunks
    let mut count = 2;
    while let Some( result ) = stream.next().await
    {
      assert!( result.is_ok() );
      count += 1;
    }

    assert_eq!( count, 10 );
  }

  /// Test cancel operation
  #[ tokio::test ]
  async fn test_cancel()
  {
    let receiver = create_mock_stream();
    let ( mut stream, control ) = wrap_stream( receiver );

    // Read first 3 chunks
    for _ in 0..3
    {
      assert!( stream.next().await.is_some() );
    }

    // Cancel the stream
    control.cancel().await.unwrap();

    // Stream should end immediately
    assert!( stream.next().await.is_none() );
  }

  /// Test pause state query
  #[ tokio::test ]
  async fn test_is_paused()
  {
    let receiver = create_mock_stream();
    let ( stream, control ) = wrap_stream( receiver );

    assert!( !stream.is_paused() );
    assert!( stream.is_running() );

    control.pause().await.unwrap();

    // Note: We can't directly check is_paused() after pause since
    // the stream processes control signals during polling.
    // This test verifies the API is available.

    control.cancel().await.unwrap();
  }

  /// Test `is_cancelled` state query
  #[ tokio::test ]
  async fn test_is_cancelled()
  {
    let receiver = create_mock_stream();
    let ( stream, control ) = wrap_stream( receiver );

    assert!( !stream.is_cancelled() );
    assert!( stream.is_running() );

    control.cancel().await.unwrap();

    // Stream state changes during polling, so we just verify the API
  }

  /// Test `try_pause` (non-blocking)
  #[ tokio::test ]
  async fn test_try_pause()
  {
    let receiver = create_mock_stream();
    let ( mut stream, control ) = wrap_stream( receiver );

    // try_pause should succeed immediately
    assert!( control.try_pause().is_ok() );

    // Clean up
    control.cancel().await.unwrap();
    while stream.next().await.is_some() {}
  }

  /// Test `try_resume` (non-blocking)
  #[ tokio::test ]
  async fn test_try_resume()
  {
    let receiver = create_mock_stream();
    let ( mut stream, control ) = wrap_stream( receiver );

    control.pause().await.unwrap();

    // try_resume should succeed immediately
    assert!( control.try_resume().is_ok() );

    // Clean up
    control.cancel().await.unwrap();
    while stream.next().await.is_some() {}
  }

  /// Test `try_cancel` (non-blocking)
  #[ tokio::test ]
  async fn test_try_cancel()
  {
    let receiver = create_mock_stream();
    let ( mut stream, control ) = wrap_stream( receiver );

    // try_cancel should succeed immediately
    assert!( control.try_cancel().is_ok() );

    // Stream should end
    assert!( stream.next().await.is_none() );
  }

  /// Test multiple pause/resume cycles
  #[ tokio::test ]
  async fn test_multiple_pause_resume()
  {
    let receiver = create_mock_stream();
    let ( mut stream, control ) = wrap_stream( receiver );

    // Read, pause, resume multiple times
    for cycle in 0..3
    {
      // Read one chunk
      assert!( stream.next().await.is_some(), "Failed at cycle {cycle}" );

      // Pause
      control.pause().await.unwrap();

      // Short delay
      tokio::time::sleep( tokio::time::Duration::from_millis( 50 ) ).await;

      // Resume
      control.resume().await.unwrap();
    }

    // Cancel and finish
    control.cancel().await.unwrap();
    assert!( stream.next().await.is_none() );
  }

  /// Test control after stream ends naturally
  #[ tokio::test ]
  async fn test_control_after_natural_end()
  {
    let ( tx, rx ) = mpsc::channel( 10 );

    // Create a short stream (2 items)
    tokio::spawn( async move
    {
      tx.send( Ok( "chunk_0".to_string() ) ).await.ok();
      tx.send( Ok( "chunk_1".to_string() ) ).await.ok();
      // Stream ends naturally
    });

    let ( mut stream, _control ) = wrap_stream( rx );

    // Consume all items
    while stream.next().await.is_some() {}

    // Control operations after stream ends should fail gracefully
    // (control channel is closed when stream task ends)
    assert!( stream.next().await.is_none() );
  }

  /// Test control handle cloning
  #[ tokio::test ]
  async fn test_control_handle_clone()
  {
    let receiver = create_mock_stream();
    let ( mut stream, control ) = wrap_stream( receiver );

    // Clone the control handle
    let control_clone = control.clone();

    // Both handles should work
    control.pause().await.unwrap();
    control_clone.resume().await.unwrap();

    // Read a chunk to verify stream works
    assert!( stream.next().await.is_some() );

    // Clean up
    control.cancel().await.unwrap();
    while stream.next().await.is_some() {}
  }

  /// Test stream with errors
  #[ tokio::test ]
  async fn test_streaming_with_errors()
  {
    let ( tx, rx ) = mpsc::channel( 10 );

    tokio::spawn( async move
    {
      tx.send( Ok( "chunk_0".to_string() ) ).await.ok();
      tx.send( Err( HuggingFaceError::Stream( "test error".to_string() ) ) ).await.ok();
      tx.send( Ok( "chunk_2".to_string() ) ).await.ok();
    });

    let ( mut stream, control ) = wrap_stream( rx );

    // First chunk OK
    assert!( stream.next().await.unwrap().is_ok() );

    // Second chunk is error
    assert!( stream.next().await.unwrap().is_err() );

    // Third chunk OK
    assert!( stream.next().await.unwrap().is_ok() );

    control.cancel().await.unwrap();
  }

  /// Test pause during error handling
  #[ tokio::test ]
  async fn test_pause_during_errors()
  {
    let ( tx, rx ) = mpsc::channel( 10 );

    tokio::spawn( async move
    {
      for i in 0..5
      {
        if i == 2
        {
          tx.send( Err( HuggingFaceError::Stream( "error".to_string() ) ) ).await.ok();
        }
        else
        {
          tx.send( Ok( format!( "chunk_{i}" ) ) ).await.ok();
        }
        tokio::time::sleep( tokio::time::Duration::from_millis( 50 ) ).await;
      }
    });

    let ( mut stream, control ) = wrap_stream( rx );

    // Read first chunk (chunk_0)
    assert!( stream.next().await.unwrap().is_ok() );

    // Pause before error
    control.pause().await.unwrap();
    tokio::time::sleep( tokio::time::Duration::from_millis( 100 ) ).await;

    // Resume and get buffered chunks (chunk_1, then error at i==2)
    control.resume().await.unwrap();

    // Get chunk_1 (buffered during pause)
    assert!( stream.next().await.unwrap().is_ok() );

    // Get error at i==2
    assert!( stream.next().await.unwrap().is_err() );

    // Continue with remaining chunks
    control.cancel().await.unwrap();
  }

  /// Test Debug implementations
  #[ test ]
  fn test_debug_implementations()
  {
    let ( _tx, rx ) = mpsc::channel::< Result< String > >( 10 );

    let ( stream, handle ) = wrap_stream( rx );

    // Verify Debug trait works
    let _ = format!( "{stream:?}" );
    let _ = format!( "{handle:?}" );
  }
}
