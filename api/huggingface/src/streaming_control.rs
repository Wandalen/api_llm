//! Streaming control for pause, resume, and cancel operations.
//!
//! Provides a wrapper around streaming responses that allows runtime control
//! of the stream flow through a control channel.
//!
//! # Example
//!
//! ```rust,ignore
//! use api_huggingface::{ InferenceClient, streaming_control::{ ControlHandle, StreamControl } };
//!
//! let ( stream, control ) = client
//!   .create_controlled_stream( "Hello, world!", "gpt2", params )
//!   .await?;
//!
//! // Pause streaming
//! control.pause().await?;
//!
//! // Resume streaming
//! control.resume().await?;
//!
//! // Cancel streaming
//! control.cancel().await?;
//! ```

#[ cfg( feature = "streaming-control" ) ]
mod private
{
  use crate::error::{ Result, HuggingFaceError };
  use tokio::sync::mpsc;
  use futures_core::Stream;
  use core::{ pin::Pin, task::{ Context, Poll } };

  /// Control signal for stream operations
  #[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
  pub enum StreamControl
  {
    /// Pause the stream (buffer incoming events)
    Pause,
    /// Resume the stream (continue reading buffered events)
    Resume,
    /// Cancel the stream (stop immediately and drop connection)
    Cancel,
  }

  /// Stream state for tracking pause/resume/cancel
  #[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
  enum StreamState
  {
    /// Stream is actively reading and forwarding events
    Running,
    /// Stream is paused (events are buffered)
    Paused,
    /// Stream is cancelled (no more events will be read)
    Cancelled,
  }

  /// Handle for controlling a stream
  ///
  /// Provides async methods to pause, resume, and cancel streaming operations.
  #[ derive( Debug, Clone ) ]
  pub struct ControlHandle
  {
    control_tx : mpsc::Sender< StreamControl >,
  }

  impl ControlHandle
  {
    /// Create a new control handle
    #[ inline ]
    #[ must_use ]
    pub( crate ) fn new( control_tx : mpsc::Sender< StreamControl > ) -> Self
    {
      Self { control_tx }
    }

    /// Pause the stream
    ///
    /// Incoming events will be buffered until resume is called.
    ///
    /// # Errors
    ///
    /// Returns error if the control channel is closed
    #[ inline ]
    pub async fn pause( &self ) -> Result< () >
    {
      self.control_tx
        .send( StreamControl::Pause )
        .await
        .map_err( | _ | HuggingFaceError::Stream( "Control channel closed".to_string() ) )
    }

    /// Resume the stream
    ///
    /// Continues reading events from where it was paused.
    ///
    /// # Errors
    ///
    /// Returns error if the control channel is closed
    #[ inline ]
    pub async fn resume( &self ) -> Result< () >
    {
      self.control_tx
        .send( StreamControl::Resume )
        .await
        .map_err( | _ | HuggingFaceError::Stream( "Control channel closed".to_string() ) )
    }

    /// Cancel the stream
    ///
    /// Stops the stream immediately and closes the connection.
    ///
    /// # Errors
    ///
    /// Returns error if the control channel is closed
    #[ inline ]
    pub async fn cancel( &self ) -> Result< () >
    {
      self.control_tx
        .send( StreamControl::Cancel )
        .await
        .map_err( | _ | HuggingFaceError::Stream( "Control channel closed".to_string() ) )
    }

    /// Try to pause the stream without blocking
    ///
    /// # Errors
    ///
    /// Returns error if the control channel is full or closed
    #[ inline ]
    pub fn try_pause( &self ) -> Result< () >
    {
      self.control_tx
        .try_send( StreamControl::Pause )
        .map_err( | _ | HuggingFaceError::Stream( "Control channel full or closed".to_string() ) )
    }

    /// Try to resume the stream without blocking
    ///
    /// # Errors
    ///
    /// Returns error if the control channel is full or closed
    #[ inline ]
    pub fn try_resume( &self ) -> Result< () >
    {
      self.control_tx
        .try_send( StreamControl::Resume )
        .map_err( | _ | HuggingFaceError::Stream( "Control channel full or closed".to_string() ) )
    }

    /// Try to cancel the stream without blocking
    ///
    /// # Errors
    ///
    /// Returns error if the control channel is full or closed
    #[ inline ]
    pub fn try_cancel( &self ) -> Result< () >
    {
      self.control_tx
        .try_send( StreamControl::Cancel )
        .map_err( | _ | HuggingFaceError::Stream( "Control channel full or closed".to_string() ) )
    }
  }

  /// Controlled stream that supports pause, resume, and cancel operations
  ///
  /// Wraps an `mpsc::Receiver` and adds stream control capabilities.
  pub struct ControlledStream
  {
    /// Inner receiver that provides the actual stream data
    inner : mpsc::Receiver< Result< String > >,
    /// Control channel receiver for pause/resume/cancel signals
    control_rx : mpsc::Receiver< StreamControl >,
    /// Current state of the stream
    state : StreamState,
  }

  impl ControlledStream
  {
    /// Create a new controlled stream
    ///
    /// # Arguments
    ///
    /// * `inner` - The underlying receiver providing stream data
    /// * `control_rx` - Receiver for control signals
    #[ inline ]
    #[ must_use ]
    pub( crate ) fn new(
      inner : mpsc::Receiver< Result< String > >,
      control_rx : mpsc::Receiver< StreamControl >,
    ) -> Self
    {
      Self
      {
        inner,
        control_rx,
        state : StreamState::Running,
      }
    }

    /// Get the current stream state
    #[ inline ]
    #[ must_use ]
    pub fn is_paused( &self ) -> bool
    {
      matches!( self.state, StreamState::Paused )
    }

    /// Check if the stream is cancelled
    #[ inline ]
    #[ must_use ]
    pub fn is_cancelled( &self ) -> bool
    {
      matches!( self.state, StreamState::Cancelled )
    }

    /// Check if the stream is running
    #[ inline ]
    #[ must_use ]
    pub fn is_running( &self ) -> bool
    {
      matches!( self.state, StreamState::Running )
    }
  }

  impl Stream for ControlledStream
  {
    type Item = Result< String >;

    #[ inline ]
    fn poll_next( mut self : Pin< &mut Self >, cx : &mut Context< '_ > ) -> Poll< Option< Self::Item > >
    {
      // Process all pending control signals
      loop
      {
        match self.control_rx.poll_recv( cx )
        {
          Poll::Ready( Some( control ) ) =>
          {
            match control
            {
              StreamControl::Pause =>
              {
                self.state = StreamState::Paused;
              },
              StreamControl::Resume =>
              {
                self.state = StreamState::Running;
              },
              StreamControl::Cancel =>
              {
                self.state = StreamState::Cancelled;
                return Poll::Ready( None );
              },
            }
          },
          Poll::Ready( None ) =>
          {
            // Control channel closed, treat as cancel
            self.state = StreamState::Cancelled;
            return Poll::Ready( None );
          },
          Poll::Pending => break,
        }
      }

      // If paused, return Pending to park the task
      if matches!( self.state, StreamState::Paused )
      {
        // Re-register the waker for control channel
        let _ = self.control_rx.poll_recv( cx );
        return Poll::Pending;
      }

      // If cancelled, return None
      if matches!( self.state, StreamState::Cancelled )
      {
        return Poll::Ready( None );
      }

      // Poll the inner receiver
      self.inner.poll_recv( cx )
    }
  }

  impl core::fmt::Debug for ControlledStream
  {
    #[ inline ]
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      f.debug_struct( "ControlledStream" )
        .field( "state", &self.state )
        .finish_non_exhaustive()
    }
  }

  /// Create a controlled stream from an existing receiver
  ///
  /// Returns a tuple of (`ControlledStream`, `ControlHandle`)
  ///
  /// # Arguments
  ///
  /// * `receiver` - The underlying stream receiver
  ///
  /// # Returns
  ///
  /// A tuple containing the controlled stream and its control handle
  #[ inline ]
  #[ must_use ]
  pub fn wrap_stream(
    receiver : mpsc::Receiver< Result< String > >,
  ) -> ( ControlledStream, ControlHandle )
  {
    let ( control_tx, control_rx ) = mpsc::channel( 10 );
    let stream = ControlledStream::new( receiver, control_rx );
    let handle = ControlHandle::new( control_tx );
    ( stream, handle )
  }
}

#[ cfg( feature = "streaming-control" ) ]
crate::mod_interface!
{
  exposed use private::StreamControl;
  exposed use private::ControlHandle;
  exposed use private::ControlledStream;
  exposed use private::wrap_stream;
}
