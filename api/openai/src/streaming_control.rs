//! Streaming Control Module
//!
//! This module provides stateless streaming control utilities for `OpenAI` API responses.
//! Following the "Thin Client, Rich API" principle, this module offers control patterns
//! and cancellation tokens without maintaining persistent stream state.

mod private
{
  use std::
  {
    sync ::Arc,
    time ::Instant,
  };
  use core::
  {
    sync ::atomic::{ AtomicBool, Ordering },
    time ::Duration,
  };
  use serde::{ Deserialize, Serialize };
  use tokio::{ sync::mpsc, time };

  /// Stream control state for tracking operations
  #[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
  pub enum StreamState
  {
    /// Stream is actively running
    Running,
    /// Stream is paused (buffering)
    Paused,
    /// Stream is cancelled
    Cancelled,
    /// Stream completed normally
    Completed,
    /// Stream encountered an error
    Error( String ),
  }

  /// Cancellation token for controlling streaming operations
  #[ derive( Debug, Clone ) ]
  pub struct CancellationToken
  {
    /// Internal cancellation flag
    cancelled : Arc< AtomicBool >,
  }

  impl CancellationToken
  {
    /// Create a new cancellation token
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        cancelled : Arc::new( AtomicBool::new( false ) ),
      }
    }

    /// Cancel the operation
    #[ inline ]
    pub fn cancel( &self )
    {
      self.cancelled.store( true, Ordering::SeqCst );
    }

    /// Check if operation is cancelled
    #[ inline ]
    #[ must_use ]
    pub fn is_cancelled( &self ) -> bool
    {
      self.cancelled.load( Ordering::SeqCst )
    }

    /// Wait for cancellation or timeout
    #[ inline ]
    pub async fn wait_for_cancellation( &self, timeout : Duration ) -> bool
    {
      let start = Instant::now();
      while start.elapsed() < timeout
      {
        if self.is_cancelled()
        {
          return true;
        }
        time ::sleep( Duration::from_millis( 10 ) ).await;
      }
      false
    }
  }

  impl Default for CancellationToken
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Stream control handle for managing streaming operations
  #[ derive( Debug ) ]
  pub struct StreamControl
  {
    /// Current state of the stream
    state : StreamState,
    /// Cancellation token
    token : CancellationToken,
    /// Creation timestamp
    created_at : Instant,
  }

  impl StreamControl
  {
    /// Create a new stream control handle
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        state : StreamState::Running,
        token : CancellationToken::new(),
        created_at : Instant::now(),
      }
    }

    /// Get current stream state
    #[ inline ]
    #[ must_use ]
    pub fn state( &self ) -> &StreamState
    {
      &self.state
    }

    /// Get cancellation token
    #[ inline ]
    #[ must_use ]
    pub fn cancellation_token( &self ) -> &CancellationToken
    {
      &self.token
    }

    /// Get elapsed time since creation
    #[ inline ]
    #[ must_use ]
    pub fn elapsed( &self ) -> Duration
    {
      self.created_at.elapsed()
    }

    /// Update stream state
    #[ inline ]
    pub fn set_state( &mut self, state : StreamState )
    {
      self.state = state;
    }

    /// Cancel the stream
    #[ inline ]
    pub fn cancel( &mut self )
    {
      self.token.cancel();
      self.state = StreamState::Cancelled;
    }

    /// Check if stream is active (not cancelled, completed, or errored)
    #[ inline ]
    #[ must_use ]
    pub fn is_active( &self ) -> bool
    {
      matches!( self.state, StreamState::Running | StreamState::Paused )
    }
  }

  impl Default for StreamControl
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Configuration for streaming control behavior
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct StreamControlConfig
  {
    /// Maximum pause duration before automatic timeout
    pub max_pause_duration : Duration,
    /// Buffer size for paused streams
    pub pause_buffer_size : usize,
    /// Timeout for control operations
    pub control_timeout : Duration,
  }

  impl Default for StreamControlConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        max_pause_duration : Duration::from_secs( 300 ), // 5 minutes
        pause_buffer_size : 1024 * 1024, // 1MB
        control_timeout : Duration::from_secs( 5 ),
      }
    }
  }

  /// Stream control utilities
  #[ derive( Debug ) ]
  pub struct StreamControlManager;

  impl StreamControlManager
  {
    /// Create a controlled stream processing function
    /// This returns a function that can process stream items with control
    #[ inline ]
    pub fn create_controlled_processor< T >(
      control : StreamControl,
    ) -> impl Fn( T ) -> Option< T >
    where
      T : Send + 'static,
    {
      move | item : T | -> Option< T >
      {
        // Check cancellation before processing
        if control.token.is_cancelled()
        {
          return None;
        }

        // For stateless operation, we process immediately
        // In a real streaming scenario, this would integrate with the actual stream
        Some( item )
      }
    }

    /// Create a cancellable async operation
    ///
    /// # Errors
    ///
    /// Returns an error if the operation is cancelled.
    #[ inline ]
    pub async fn with_cancellation< T, F, Fut >(
      token : CancellationToken,
      operation : F,
    ) -> Result< T, &'static str >
    where
      F : FnOnce() -> Fut,
      Fut : core::future::Future< Output = T >,
    {
      let operation_future = operation();

      tokio ::select!
      {
        result = operation_future =>
        {
          if token.is_cancelled()
          {
            Err( "Operation was cancelled" )
          }
          else
          {
            Ok( result )
          }
        }
        () = Self::wait_for_cancellation( &token ) =>
        {
          Err( "Operation was cancelled" )
        }
      }
    }

    /// Wait for cancellation token to be triggered
    async fn wait_for_cancellation( token : &CancellationToken )
    {
      while !token.is_cancelled()
      {
        time ::sleep( Duration::from_millis( 10 ) ).await;
      }
    }

    /// Create a timeout-based cancellation token
    #[ inline ]
    #[ must_use ]
    pub fn create_timeout_token( timeout : Duration ) -> CancellationToken
    {
      let token = CancellationToken::new();
      let token_clone = token.clone();

      tokio ::spawn( async move
      {
        time ::sleep( timeout ).await;
        token_clone.cancel();
      });

      token
    }

    /// Combine multiple cancellation tokens (any cancellation triggers all)
    #[ inline ]
    #[ must_use ]
    pub fn combine_tokens( tokens : Vec< CancellationToken > ) -> CancellationToken
    {
      let combined = CancellationToken::new();
      let combined_clone = combined.clone();

      tokio ::spawn( async move
      {
        loop
        {
          if tokens.iter().any( CancellationToken::is_cancelled )
          {
            combined_clone.cancel();
            break;
          }
          time ::sleep( Duration::from_millis( 10 ) ).await;
        }
      });

      combined
    }

    /// Create a manual control channel for external control
    #[ inline ]
    #[ must_use ]
    pub fn create_control_channel() -> ( StreamControlSender, StreamControlReceiver )
    {
      let ( tx, rx ) = mpsc::unbounded_channel();
      ( StreamControlSender { sender : tx }, StreamControlReceiver { receiver : rx } )
    }
  }

  /// Sender for stream control commands
  #[ derive( Debug ) ]
  pub struct StreamControlSender
  {
    sender : mpsc::UnboundedSender< StreamControlCommand >,
  }

  impl StreamControlSender
  {
    /// Send a pause command
    ///
    /// # Errors
    ///
    /// Returns an error if the command cannot be sent.
    #[ inline ]
    pub fn pause( &self ) -> Result< (), &'static str >
    {
      self.sender.send( StreamControlCommand::Pause )
        .map_err( | _ | "Failed to send pause command" )
    }

    /// Send a resume command
    ///
    /// # Errors
    ///
    /// Returns an error if the command cannot be sent.
    #[ inline ]
    pub fn resume( &self ) -> Result< (), &'static str >
    {
      self.sender.send( StreamControlCommand::Resume )
        .map_err( | _ | "Failed to send resume command" )
    }

    /// Send a cancel command
    ///
    /// # Errors
    ///
    /// Returns an error if the command cannot be sent.
    #[ inline ]
    pub fn cancel( &self ) -> Result< (), &'static str >
    {
      self.sender.send( StreamControlCommand::Cancel )
        .map_err( | _ | "Failed to send cancel command" )
    }
  }

  /// Receiver for stream control commands
  #[ derive( Debug ) ]
  pub struct StreamControlReceiver
  {
    receiver : mpsc::UnboundedReceiver< StreamControlCommand >,
  }

  impl StreamControlReceiver
  {
    /// Try to receive a control command (non-blocking)
    #[ inline ]
    pub fn try_recv( &mut self ) -> Option< StreamControlCommand >
    {
      self.receiver.try_recv().ok()
    }

    /// Receive next control command (blocking)
    #[ inline ]
    pub async fn recv( &mut self ) -> Option< StreamControlCommand >
    {
      self.receiver.recv().await
    }
  }

  /// Commands for controlling stream operations
  #[ derive( Debug, Clone, PartialEq ) ]
  pub enum StreamControlCommand
  {
    /// Pause the stream
    Pause,
    /// Resume the stream
    Resume,
    /// Cancel the stream
    Cancel,
  }
}

crate ::mod_interface!
{
  exposed use private::StreamState;
  exposed use private::CancellationToken;
  exposed use private::StreamControl;
  exposed use private::StreamControlConfig;
  exposed use private::StreamControlManager;
  exposed use private::StreamControlSender;
  exposed use private::StreamControlReceiver;
  exposed use private::StreamControlCommand;
}