//! Buffered Streaming for Smoother UX
//!
//! Buffers streaming responses for improved display.

/// Define a private namespace for all its items.
mod private
{
  use core::time::Duration;
  use std::time::Instant;
  use core::pin::Pin;
  use core::task::{ Context, Poll };
  use futures_core::Stream;
  use tokio::time::Sleep;

  /// Configuration for buffered streaming
  #[ derive( Debug, Clone ) ]
  pub struct BufferConfig
  {
    /// Maximum buffer size in characters
    pub max_buffer_size : usize,
    /// Maximum time to hold data before flushing
    pub max_buffer_time : Duration,
    /// Flush on newline characters
    pub flush_on_newline : bool,
  }

  impl Default for BufferConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        max_buffer_size : 100,
        max_buffer_time : Duration::from_millis( 50 ),
        flush_on_newline : true,
      }
    }
  }

  impl BufferConfig
  {
    /// Create new buffer configuration
    #[ must_use ]
    #[ inline ]
    pub fn new() -> Self
    {
      Self::default()
    }

    /// Set maximum buffer size
    #[ must_use ]
    #[ inline ]
    pub fn with_max_size( mut self, size : usize ) -> Self
    {
      self.max_buffer_size = size;
      self
    }

    /// Set maximum buffer time
    #[ must_use ]
    #[ inline ]
    pub fn with_max_time( mut self, time : Duration ) -> Self
    {
      self.max_buffer_time = time;
      self
    }

    /// Enable or disable flush on newline
    #[ must_use ]
    #[ inline ]
    pub fn with_flush_on_newline( mut self, enabled : bool ) -> Self
    {
      self.flush_on_newline = enabled;
      self
    }
  }

  /// Buffered stream wrapper
  #[ derive( Debug ) ]
  pub struct BufferedStream< S >
  {
    inner : S,
    buffer : String,
    config : BufferConfig,
    last_flush : Instant,
    _flush_timer : Option< Pin< Box< Sleep > > >,
  }

  impl< S > BufferedStream< S >
  where
    S : Stream< Item = String > + Unpin,
  {
    /// Create new buffered stream
    #[ inline ]
    pub fn new( stream : S, config : BufferConfig ) -> Self
    {
      Self
      {
        inner : stream,
        buffer : String::new(),
        config,
        last_flush : Instant::now(),
        _flush_timer : None,
      }
    }

    /// Check if buffer should be flushed
    #[ inline ]
    fn should_flush( &self ) -> bool
    {
      if self.buffer.is_empty()
      {
        return false;
      }

      // Flush if buffer is full
      if self.buffer.len() >= self.config.max_buffer_size
      {
        return true;
      }

      // Flush if time limit exceeded
      if self.last_flush.elapsed() >= self.config.max_buffer_time
      {
        return true;
      }

      // Flush if newline detected
      if self.config.flush_on_newline && self.buffer.contains( '\n' )
      {
        return true;
      }

      false
    }

    /// Flush the buffer
    #[ inline ]
    fn flush( &mut self ) -> Option< String >
    {
      if self.buffer.is_empty()
      {
        return None;
      }

      let result = self.buffer.clone();
      self.buffer.clear();
      self.last_flush = Instant::now();
      Some( result )
    }
  }

  impl< S > Stream for BufferedStream< S >
  where
    S : Stream< Item = String > + Unpin,
  {
    type Item = String;

    #[ inline ]
    fn poll_next( mut self : Pin< &mut Self >, cx : &mut Context< '_ > ) -> Poll< Option< Self::Item > >
    {
      loop
      {
        // Try to get next item from inner stream
        match Pin::new( &mut self.inner ).poll_next( cx )
        {
          Poll::Ready( Some( item ) ) =>
          {
            self.buffer.push_str( &item );

            if self.should_flush()
            {
              if let Some( flushed ) = self.flush()
              {
                return Poll::Ready( Some( flushed ) );
              }
            }
          },
          Poll::Ready( None ) =>
          {
            // Stream ended, flush remaining buffer
            if let Some( flushed ) = self.flush()
            {
              return Poll::Ready( Some( flushed ) );
            }
            return Poll::Ready( None );
          },
          Poll::Pending =>
          {
            // Check if we should flush due to time
            if self.should_flush()
            {
              if let Some( flushed ) = self.flush()
              {
                return Poll::Ready( Some( flushed ) );
              }
            }
            return Poll::Pending;
          },
        }
      }
    }
  }

  /// Extension trait for streams to add buffering
  pub trait StreamBufferExt : Stream< Item = String > + Sized + Unpin
  {
    /// Add buffering to the stream
    #[ inline ]
    fn with_buffer( self, config : BufferConfig ) -> BufferedStream< Self >
    {
      BufferedStream::new( self, config )
    }

    /// Add buffering with default configuration
    #[ inline ]
    fn with_buffer_default( self ) -> BufferedStream< Self >
    {
      BufferedStream::new( self, BufferConfig::default() )
    }
  }

  impl< S > StreamBufferExt for S where S : Stream< Item = String > + Unpin {}

  #[ cfg( test ) ]
  mod tests
  {
    use super::*;

    #[ test ]
    fn test_buffer_config_creation()
    {
      let config = BufferConfig::new();
      assert_eq!( config.max_buffer_size, 100 );
      assert_eq!( config.max_buffer_time, Duration::from_millis( 50 ) );
      assert!( config.flush_on_newline );
    }

    #[ test ]
    fn test_buffer_config_builder()
    {
      let config = BufferConfig::new()
        .with_max_size( 200 )
        .with_max_time( Duration::from_millis( 100 ) )
        .with_flush_on_newline( false );

      assert_eq!( config.max_buffer_size, 200 );
      assert_eq!( config.max_buffer_time, Duration::from_millis( 100 ) );
      assert!( !config.flush_on_newline );
    }
  }
}

crate::mod_interface!
{
  exposed use
  {
    BufferConfig,
    BufferedStream,
    StreamBufferExt,
  };
}
