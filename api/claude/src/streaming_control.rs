//! Streaming control for pause/resume/cancel operations
//!
//! This module provides control mechanisms for streaming responses, allowing
//! pause, resume, and cancellation of active streams.

#[ cfg( feature = "streaming-control" ) ]
mod private
{
  use std::sync::{ Arc, Mutex };
  use std::collections::VecDeque;
  use std::pin::Pin;
  use std::task::{ Context, Poll };
  use futures::Stream;
  use crate::streaming::StreamEvent;

  /// State of a controlled stream
  #[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
  pub enum StreamState
  {
    /// Stream is actively consuming events
    Running,
    /// Stream is paused, buffering events
    Paused,
    /// Stream is cancelled, no more events
    Cancelled,
  }

  /// Internal state for stream control
  #[ derive( Debug ) ]
  struct ControlState
  {
    state : StreamState,
    buffer : VecDeque< StreamEvent >,
    buffer_limit : usize,
  }

  /// Handle for controlling stream operations
  ///
  /// Provides pause, resume, and cancel functionality for streaming responses.
  #[ derive( Debug, Clone ) ]
  pub struct StreamControl
  {
    state : Arc< Mutex< ControlState > >,
  }

  impl StreamControl
  {
    /// Create a new stream control handle
    pub fn new( buffer_limit : usize ) -> Self
    {
      Self
      {
        state : Arc::new( Mutex::new( ControlState
        {
          state : StreamState::Running,
          buffer : VecDeque::new(),
          buffer_limit,
        } ) ),
      }
    }

    /// Pause the stream
    ///
    /// When paused, events are buffered up to the buffer limit.
    /// If the buffer fills, oldest events are dropped.
    ///
    /// # Errors
    ///
    /// Returns an error if the stream is already cancelled.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn pause( &self ) -> Result< (), String >
    {
      let mut state = self.state.lock().unwrap();
      if state.state == StreamState::Cancelled
      {
        return Err( "Cannot pause cancelled stream".to_string() );
      }
      state.state = StreamState::Paused;
      Ok( () )
    }

    /// Resume the stream
    ///
    /// Buffered events will be delivered before new events.
    ///
    /// # Errors
    ///
    /// Returns an error if the stream is already cancelled.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn resume( &self ) -> Result< (), String >
    {
      let mut state = self.state.lock().unwrap();
      if state.state == StreamState::Cancelled
      {
        return Err( "Cannot resume cancelled stream".to_string() );
      }
      state.state = StreamState::Running;
      Ok( () )
    }

    /// Cancel the stream
    ///
    /// This is irreversible. The stream will stop producing events.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn cancel( &self )
    {
      let mut state = self.state.lock().unwrap();
      state.state = StreamState::Cancelled;
      state.buffer.clear();
    }

    /// Check if stream is paused
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn is_paused( &self ) -> bool
    {
      let state = self.state.lock().unwrap();
      state.state == StreamState::Paused
    }

    /// Check if stream is cancelled
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn is_cancelled( &self ) -> bool
    {
      let state = self.state.lock().unwrap();
      state.state == StreamState::Cancelled
    }

    /// Check if stream is running
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn is_running( &self ) -> bool
    {
      let state = self.state.lock().unwrap();
      state.state == StreamState::Running
    }

    /// Get current state
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn get_state( &self ) -> StreamState
    {
      let state = self.state.lock().unwrap();
      state.state
    }

    /// Get number of buffered events
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn buffer_size( &self ) -> usize
    {
      let state = self.state.lock().unwrap();
      state.buffer.len()
    }

    /// Buffer an event (called internally by `ControlledStream`)
    fn buffer_event( &self, event : StreamEvent )
    {
      let mut state = self.state.lock().unwrap();
      if state.buffer.len() >= state.buffer_limit
      {
        // Drop oldest event if buffer is full
        state.buffer.pop_front();
      }
      state.buffer.push_back( event );
    }

    /// Get next buffered event (called internally by `ControlledStream`)
    fn next_buffered( &self ) -> Option< StreamEvent >
    {
      let mut state = self.state.lock().unwrap();
      state.buffer.pop_front()
    }

    /// Check if there are buffered events
    fn has_buffered( &self ) -> bool
    {
      let state = self.state.lock().unwrap();
      !state.buffer.is_empty()
    }
  }

  /// Controlled stream wrapper
  ///
  /// Wraps a stream with pause/resume/cancel control functionality.
  #[ derive( Debug ) ]
  pub struct ControlledStream< S >
  where
    S: Stream< Item = Result< StreamEvent, crate::error::AnthropicError > > + Unpin,
  {
    inner : S,
    control : StreamControl,
  }

  impl< S > ControlledStream< S >
  where
    S: Stream< Item = Result< StreamEvent, crate::error::AnthropicError > > + Unpin,
  {
    /// Create a new controlled stream
    ///
    /// # Arguments
    ///
    /// * `inner` - The underlying stream to control
    /// * `buffer_limit` - Maximum number of events to buffer when paused
    pub fn new( inner : S, buffer_limit : usize ) -> ( Self, StreamControl )
    {
      let control = StreamControl::new( buffer_limit );
      let controlled = Self
      {
        inner,
        control : control.clone(),
      };
      ( controlled, control )
    }

    /// Get a clone of the control handle
    pub fn control( &self ) -> StreamControl
    {
      self.control.clone()
    }
  }

  impl< S > Stream for ControlledStream< S >
  where
    S: Stream< Item = Result< StreamEvent, crate::error::AnthropicError > > + Unpin,
  {
    type Item = Result< StreamEvent, crate::error::AnthropicError >;

    fn poll_next( mut self : Pin< &mut Self >, cx : &mut Context< '_ > ) -> Poll< Option< Self::Item > >
    {
      // Check if cancelled
      if self.control.is_cancelled()
      {
        return Poll::Ready( None );
      }

      // If paused, buffer events from inner stream
      if self.control.is_paused()
      {
        // Try to poll inner stream to buffer events
        match Pin::new( &mut self.inner ).poll_next( cx )
        {
          Poll::Ready( Some( Ok( event ) ) ) =>
          {
            self.control.buffer_event( event );
            Poll::Pending
          }
          Poll::Ready( Some( Err( e ) ) ) =>
          {
            // Don't buffer errors, return them immediately
            Poll::Ready( Some( Err( e ) ) )
          }
          Poll::Ready( None ) =>
          {
            // Stream ended while paused
            Poll::Ready( None )
          }
          Poll::Pending => Poll::Pending,
        }
      }
      else
      {
        // Running - first deliver any buffered events
        if self.control.has_buffered()
        {
          if let Some( event ) = self.control.next_buffered()
          {
            return Poll::Ready( Some( Ok( event ) ) );
          }
        }

        // Then poll inner stream
        Pin::new( &mut self.inner ).poll_next( cx )
      }
    }
  }
}

#[ cfg( feature = "streaming-control" ) ]
crate::mod_interface!
{
  exposed use
  {
    StreamControl,
    StreamState,
    ControlledStream,
  };
}

#[ cfg( not( feature = "streaming-control" ) ) ]
crate::mod_interface!
{
  // Empty when streaming-control feature is disabled
}
