//! Buffered streaming for smoother UX.
//!
//! This module provides buffered streaming responses for smoother display,
//! batching small chunks together and controlling delivery timing.

use futures::Stream;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::Instant;

/// Configuration for buffered streaming.
#[ derive( Debug, Clone ) ]
pub struct BufferConfig
{
  /// Minimum buffer size before flushing (in characters)
  pub min_buffer_size : usize,
  /// Maximum time to wait before flushing buffer
  pub max_buffer_time : Duration,
  /// Whether to flush on newlines
  pub flush_on_newline : bool,
}

impl Default for BufferConfig
{
  fn default() -> Self
  {
    Self
    {
      min_buffer_size : 50,
      max_buffer_time : Duration::from_millis( 100 ),
      flush_on_newline : true,
    }
  }
}

impl BufferConfig
{
  /// Create a new buffer configuration.
  #[ must_use ]
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Set minimum buffer size.
  #[ must_use ]
  pub fn with_min_buffer_size( mut self, size : usize ) -> Self
  {
    self.min_buffer_size = size;
    self
  }

  /// Set maximum buffer time.
  #[ must_use ]
  pub fn with_max_buffer_time( mut self, duration : Duration ) -> Self
  {
    self.max_buffer_time = duration;
    self
  }

  /// Enable/disable flushing on newlines.
  #[ must_use ]
  pub fn with_flush_on_newline( mut self, enabled : bool ) -> Self
  {
    self.flush_on_newline = enabled;
    self
  }
}

/// Buffered stream wrapper.
#[ derive( Debug ) ]
pub struct BufferedStream< S >
where
  S : Stream< Item = String > + Unpin,
{
  inner : S,
  config : BufferConfig,
  buffer : String,
  last_flush : Instant,
}

impl< S > BufferedStream< S >
where
  S : Stream< Item = String > + Unpin,
{
  /// Create a new buffered stream.
  pub fn new( stream : S, config : BufferConfig ) -> Self
  {
    Self
    {
      inner : stream,
      config,
      buffer : String::new(),
      last_flush : Instant::now(),
    }
  }

  /// Check if buffer should be flushed.
  fn should_flush( &self ) -> bool
  {
    // Flush if buffer exceeds min size
    if self.buffer.len() >= self.config.min_buffer_size
    {
      return true;
    }

    // Flush if max time elapsed
    if self.last_flush.elapsed() >= self.config.max_buffer_time
    {
      return true;
    }

    // Flush if newline detected and enabled
    if self.config.flush_on_newline && self.buffer.contains( '\n' )
    {
      return true;
    }

    false
  }

  /// Flush the buffer and return contents.
  fn flush( &mut self ) -> Option< String >
  {
    if self.buffer.is_empty()
    {
      return None;
    }

    let content = self.buffer.clone();
    self.buffer.clear();
    self.last_flush = Instant::now();
    Some( content )
  }
}

impl< S > Stream for BufferedStream< S >
where
  S : Stream< Item = String > + Unpin,
{
  type Item = String;

  fn poll_next(
    mut self : Pin< &mut Self >,
    cx : &mut std::task::Context< '_ >,
  ) -> std::task::Poll< Option< Self::Item > >
  {
    use std::task::Poll;

    loop
    {
      // Try to get next chunk from inner stream
      match Pin::new( &mut self.inner ).poll_next( cx )
      {
        Poll::Ready( Some( chunk ) ) =>
        {
          self.buffer.push_str( &chunk );

          // Flush if conditions met
          if self.should_flush()
          {
            if let Some( content ) = self.flush()
            {
              return Poll::Ready( Some( content ) );
            }
          }
          // Continue buffering
          continue;
        }
        Poll::Ready( None ) =>
        {
          // Stream ended - flush remaining buffer
          return Poll::Ready( self.flush() );
        }
        Poll::Pending =>
        {
          // No new data available - check if should flush anyway
          if self.should_flush()
          {
            if let Some( content ) = self.flush()
            {
              return Poll::Ready( Some( content ) );
            }
          }
          return Poll::Pending;
        }
      }
    }
  }
}

/// Extension trait for adding buffering to streams.
pub trait BufferedStreamExt : Stream< Item = String > + Sized + Unpin
{
  /// Add buffering to this stream.
  fn buffered( self, config : BufferConfig ) -> BufferedStream< Self >
  {
    BufferedStream::new( self, config )
  }

  /// Add buffering with default configuration.
  fn buffered_default( self ) -> BufferedStream< Self >
  {
    BufferedStream::new( self, BufferConfig::default() )
  }
}

impl< S > BufferedStreamExt for S where S : Stream< Item = String > + Unpin {}

#[ cfg( test ) ]
mod tests
{
  use super::*;
  use tokio_stream;
  use tokio_stream::StreamExt;

  #[ tokio::test ]
  async fn test_buffer_config_creation()
  {
    let config = BufferConfig::new();
    assert_eq!( config.min_buffer_size, 50 );
    assert_eq!( config.max_buffer_time, Duration::from_millis( 100 ) );
    assert!( config.flush_on_newline );
  }

  #[ tokio::test ]
  async fn test_buffer_config_builder()
  {
    let config = BufferConfig::new()
      .with_min_buffer_size( 100 )
      .with_max_buffer_time( Duration::from_millis( 200 ) )
      .with_flush_on_newline( false );

    assert_eq!( config.min_buffer_size, 100 );
    assert_eq!( config.max_buffer_time, Duration::from_millis( 200 ) );
    assert!( !config.flush_on_newline );
  }

  #[ tokio::test ]
  async fn test_buffered_stream_basic()
  {
    let items = vec![ "a".to_string(), "b".to_string(), "c".to_string() ];
    let stream = tokio_stream::iter( items );

    let config = BufferConfig::new().with_min_buffer_size( 2 );
    let mut buffered = stream.buffered( config );

    let mut results = vec![];
    while let Some( chunk ) = buffered.next().await
    {
      results.push( chunk );
    }

    // Should combine small chunks
    assert!( !results.is_empty() );
  }

  #[ tokio::test ]
  async fn test_buffered_stream_flush_on_newline()
  {
    let items = vec![ "hello".to_string(), "\n".to_string(), "world".to_string() ];
    let stream = tokio_stream::iter( items );

    let config = BufferConfig::new()
      .with_min_buffer_size( 100 ) // Large buffer
      .with_flush_on_newline( true );

    let mut buffered = stream.buffered( config );

    let mut results = vec![];
    while let Some( chunk ) = buffered.next().await
    {
      results.push( chunk );
    }

    // Should flush on newline despite large buffer
    assert!( results.len() >= 2 );
  }

  #[ tokio::test ]
  async fn test_buffered_stream_size_threshold()
  {
    let items = vec![ "x".repeat( 60 ) ];
    let stream = tokio_stream::iter( items );

    let config = BufferConfig::new().with_min_buffer_size( 50 );
    let mut buffered = stream.buffered( config );

    let result = buffered.next().await;
    assert!( result.is_some() );
    assert_eq!( result.unwrap().len(), 60 );
  }
}
