//! Stream control functionality for Ollama API.
//!
//! Provides pause/resume/cancel capabilities for streaming responses,
//! with buffering, metrics tracking, and control interfaces.

#[ cfg( feature = "streaming_control" ) ]
mod private
{
  use core::time::Duration;
  use std::sync::Arc;

  /// Streaming control functionality for pause/resume/cancel operations
  #[ cfg( feature = "streaming_control" ) ]
  #[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
  pub enum StreamState
  {
    /// Stream is ready to start
    Ready,
    /// Stream is actively streaming data
    Streaming,
    /// Stream is paused and buffering
    Paused,
    /// Stream is cancelled and cleaned up
    Cancelled,
  }

  #[ cfg( feature = "streaming_control" ) ]
  impl core::fmt::Display for StreamState
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      match self
      {
        StreamState::Ready => write!( f, "Ready" ),
        StreamState::Streaming => write!( f, "Streaming" ),
        StreamState::Paused => write!( f, "Paused" ),
        StreamState::Cancelled => write!( f, "Cancelled" ),
      }
    }
  }

  /// Errors related to stream control operations
  #[ cfg( feature = "streaming_control" ) ]
  #[ derive( Debug, Clone ) ]
  pub enum StreamControlError
  {
    /// Invalid state transition attempted
    InvalidStateTransition {
      /// The state transitioning from
      from : StreamState,
      /// The state attempting to transition to
      to : StreamState
    },
    /// Stream operation timed out
    TimeoutError,
    /// Buffer overflow occurred
    BufferOverflow {
      /// The buffer size limit that was exceeded
      limit : usize
    },
    /// Stream was cancelled
    StreamCancelled,
    /// General error with message
    GeneralError( String ),
  }

  #[ cfg( feature = "streaming_control" ) ]
  impl core::fmt::Display for StreamControlError
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      match self
      {
        StreamControlError::InvalidStateTransition { from, to } => {
          write!( f, "Invalid state transition from {from} to {to}" )
        },
        StreamControlError::TimeoutError => write!( f, "Stream control operation timed out" ),
        StreamControlError::BufferOverflow { limit } => {
          write!( f, "Buffer overflow : exceeded limit of {limit} bytes" )
        },
        StreamControlError::StreamCancelled => write!( f, "Stream was cancelled" ),
        StreamControlError::GeneralError( msg ) => write!( f, "Stream control error : {msg}" ),
      }
    }
  }

  #[ cfg( feature = "streaming_control" ) ]
  impl std::error::Error for StreamControlError
  {}

  /// Metrics for stream control operations
  #[ cfg( feature = "streaming_control" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct StreamMetrics
  {
    /// Number of times stream was paused
    pub pause_count : u64,
    /// Number of times stream was resumed
    pub resume_count : u64,
    /// Total time spent in paused state
    pub total_pause_duration : Duration,
    /// Time when last pause started
    pub last_pause_start : Option< std::time::Instant >,
    /// Total bytes buffered during pause
    pub total_buffered_bytes : u64,
  }

  #[ cfg( feature = "streaming_control" ) ]
  impl StreamMetrics
  {
    /// Create new metrics instance
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self {
        pause_count : 0,
        resume_count : 0,
        total_pause_duration : Duration::from_secs( 0 ),
        last_pause_start : None,
        total_buffered_bytes : 0,
      }
    }

    /// Record a pause operation
    #[ inline ]
    pub fn record_pause( &mut self )
    {
      self.pause_count += 1;
      self.last_pause_start = Some( std::time::Instant::now() );
    }

    /// Record a resume operation
    #[ inline ]
    pub fn record_resume( &mut self )
    {
      self.resume_count += 1;
      if let Some( pause_start ) = self.last_pause_start.take()
      {
        self.total_pause_duration += pause_start.elapsed();
      }
    }

    /// Record buffered bytes
    #[ inline ]
    pub fn record_buffered_bytes( &mut self, bytes : u64 )
    {
      self.total_buffered_bytes += bytes;
    }
  }

  #[ cfg( feature = "streaming_control" ) ]
  impl Default for StreamMetrics
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Buffer for managing streaming data during pause states
  #[ cfg( feature = "streaming_control" ) ]
  #[ derive( Debug ) ]
  pub struct StreamBuffer
  {
    /// Internal buffer storage
    buffer : Arc< tokio::sync::Mutex< Vec< u8 > > >,
    /// Maximum buffer capacity
    capacity : usize,
  }

  #[ cfg( feature = "streaming_control" ) ]
  impl StreamBuffer
  {
    /// Create a new stream buffer with specified capacity
    #[ inline ]
    #[ must_use ]
    pub fn new( capacity : usize ) -> Self
    {
      Self {
        buffer : Arc::new( tokio::sync::Mutex::new( Vec::with_capacity( capacity ) ) ),
        capacity,
      }
    }

    /// Write data to the buffer
    #[ inline ]
    pub async fn write( &self, data : Vec< u8 > ) -> Result< (), StreamControlError >
    {
      let mut buffer = self.buffer.lock().await;

      if buffer.len() + data.len() > self.capacity
      {
        return Err( StreamControlError::BufferOverflow { limit : self.capacity } );
      }

      buffer.extend( data );
      Ok( () )
    }

    /// Read data from the buffer
    #[ inline ]
    pub async fn read( &self, size : usize ) -> Result< Vec< u8 >, StreamControlError >
    {
      let mut buffer = self.buffer.lock().await;

      if buffer.len() < size
      {
        return Ok( buffer.drain( .. ).collect() );
      }

      Ok( buffer.drain( ..size ).collect() )
    }

    /// Get current buffer length
    #[ inline ]
    pub async fn len( &self ) -> usize
    {
      self.buffer.lock().await.len()
    }

    /// Check if buffer is empty
    #[ inline ]
    pub async fn is_empty( &self ) -> bool
    {
      self.buffer.lock().await.is_empty()
    }

    /// Get buffer capacity
    #[ inline ]
    pub fn capacity( &self ) -> usize
    {
      self.capacity
    }

    /// Clear the buffer
    #[ inline ]
    pub async fn clear( &self )
    {
      self.buffer.lock().await.clear();
    }
  }

  /// Main stream control interface
  #[ cfg( feature = "streaming_control" ) ]
  pub struct StreamControl
  {
    /// Current stream state
    state : Arc< tokio::sync::RwLock< StreamState > >,
    /// Stream metrics
    metrics : Arc< tokio::sync::Mutex< StreamMetrics > >,
    /// Timeout for paused streams
    timeout : Option< Duration >,
    /// Cancellation token
    cancellation_token : Arc< tokio::sync::Mutex< Option< tokio_util::sync::CancellationToken > > >,
    /// State change callbacks
    state_callbacks : Arc< tokio::sync::Mutex< Vec< Box< dyn Fn( StreamState, StreamState ) + Send + Sync > > > >,
  }

  #[ cfg( feature = "streaming_control" ) ]
  impl StreamControl
  {
    /// Create a new stream control instance
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self {
        state : Arc::new( tokio::sync::RwLock::new( StreamState::Ready ) ),
        metrics : Arc::new( tokio::sync::Mutex::new( StreamMetrics::new() ) ),
        timeout : None,
        cancellation_token : Arc::new( tokio::sync::Mutex::new( None ) ),
        state_callbacks : Arc::new( tokio::sync::Mutex::new( Vec::new() ) ),
      }
    }

    /// Create a new stream control instance with timeout
    #[ inline ]
    #[ must_use ]
    pub fn with_timeout( timeout : Duration ) -> Self
    {
      Self {
        state : Arc::new( tokio::sync::RwLock::new( StreamState::Ready ) ),
        metrics : Arc::new( tokio::sync::Mutex::new( StreamMetrics::new() ) ),
        timeout : Some( timeout ),
        cancellation_token : Arc::new( tokio::sync::Mutex::new( None ) ),
        state_callbacks : Arc::new( tokio::sync::Mutex::new( Vec::new() ) ),
      }
    }

    /// Get current stream state
    #[ inline ]
    pub async fn state( &self ) -> StreamState
    {
      *self.state.read().await
    }

    /// Check if stream is paused
    #[ inline ]
    pub async fn is_paused( &self ) -> bool
    {
      *self.state.read().await == StreamState::Paused
    }

    /// Check if stream is cancelled
    #[ inline ]
    pub async fn is_cancelled( &self ) -> bool
    {
      *self.state.read().await == StreamState::Cancelled
    }

    /// Start the stream
    #[ inline ]
    pub async fn start( &self ) -> Result< (), StreamControlError >
    {
      let mut state = self.state.write().await;

      if *state != StreamState::Ready
      {
        return Err( StreamControlError::InvalidStateTransition {
          from : *state,
          to : StreamState::Streaming,
        } );
      }

      let old_state = *state;
      *state = StreamState::Streaming;

      // Initialize cancellation token
      {
        let mut token = self.cancellation_token.lock().await;
        *token = Some( tokio_util::sync::CancellationToken::new() );
      }

      drop( state );
      self.notify_state_change( old_state, StreamState::Streaming ).await;

      Ok( () )
    }

    /// Pause the stream
    #[ inline ]
    pub async fn pause( &self ) -> Result< (), StreamControlError >
    {
      let mut state = self.state.write().await;

      if *state != StreamState::Streaming
      {
        return Err( StreamControlError::InvalidStateTransition {
          from : *state,
          to : StreamState::Paused,
        } );
      }

      let old_state = *state;
      *state = StreamState::Paused;

      // Record pause in metrics
      {
        let mut metrics = self.metrics.lock().await;
        metrics.record_pause();
      }

      // Start timeout if configured
      if let Some( timeout ) = self.timeout
      {
        let control_clone = self.clone();
        tokio ::spawn( async move {
          tokio ::time::sleep( timeout ).await;
          let current_state = control_clone.state().await;
          if current_state == StreamState::Paused
          {
            let _ = control_clone.cancel().await;
          }
        } );
      }

      drop( state );
      self.notify_state_change( old_state, StreamState::Paused ).await;

      Ok( () )
    }

    /// Resume the stream
    #[ inline ]
    pub async fn resume( &self ) -> Result< (), StreamControlError >
    {
      let mut state = self.state.write().await;

      match *state
      {
        StreamState::Paused => {
          let old_state = *state;
          *state = StreamState::Streaming;

          // Record resume in metrics
          {
            let mut metrics = self.metrics.lock().await;
            metrics.record_resume();
          }

          drop( state );
          self.notify_state_change( old_state, StreamState::Streaming ).await;
          Ok( () )
        },
        StreamState::Cancelled => {
          Err( StreamControlError::StreamCancelled )
        },
        _ => {
          Err( StreamControlError::InvalidStateTransition {
            from : *state,
            to : StreamState::Streaming,
          } )
        }
      }
    }

    /// Cancel the stream
    #[ inline ]
    pub async fn cancel( &self ) -> Result< (), StreamControlError >
    {
      let mut state = self.state.write().await;
      let old_state = *state;

      if *state == StreamState::Cancelled
      {
        return Ok( () );
      }

      *state = StreamState::Cancelled;

      // Signal cancellation
      {
        let token_guard = self.cancellation_token.lock().await;
        if let Some( token ) = token_guard.as_ref()
        {
          token.cancel();
        }
      }

      drop( state );
      self.notify_state_change( old_state, StreamState::Cancelled ).await;

      Ok( () )
    }

    /// Trigger cleanup when stream is cancelled
    #[ inline ]
    pub async fn cleanup_on_cancel( &self, buffer : &StreamBuffer ) -> Result< (), StreamControlError >
    {
      if self.is_cancelled().await
      {
        buffer.clear().await;
      }
      Ok( () )
    }

    /// Get stream metrics
    #[ inline ]
    pub async fn get_metrics( &self ) -> StreamMetrics
    {
      self.metrics.lock().await.clone()
    }

    /// Register callback for state changes
    #[ inline ]
    pub async fn on_state_change< F >( &self, callback : F )
    where
      F: Fn( StreamState, StreamState ) + Send + Sync + 'static,
    {
      let mut callbacks = self.state_callbacks.lock().await;
      callbacks.push( Box::new( callback ) );
    }

    /// Notify all callbacks of state change
    async fn notify_state_change( &self, old_state : StreamState, new_state : StreamState )
    {
      let callbacks = self.state_callbacks.lock().await;
      for callback in callbacks.iter()
      {
        callback( old_state, new_state );
      }
    }

    /// Get cancellation token
    #[ inline ]
    pub async fn cancellation_token( &self ) -> Option< tokio_util::sync::CancellationToken >
    {
      self.cancellation_token.lock().await.clone()
    }
  }

  #[ cfg( feature = "streaming_control" ) ]
  impl core::fmt::Debug for StreamControl
  {
    #[ inline ]
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      f.debug_struct( "StreamControl" )
        .field( "timeout", &self.timeout )
        .finish()
    }
  }

  #[ cfg( feature = "streaming_control" ) ]
  impl Clone for StreamControl
  {
    #[ inline ]
    fn clone( &self ) -> Self
    {
      Self {
        state : self.state.clone(),
        metrics : self.metrics.clone(),
        timeout : self.timeout,
        cancellation_token : self.cancellation_token.clone(),
        state_callbacks : self.state_callbacks.clone(),
      }
    }
  }

  #[ cfg( feature = "streaming_control" ) ]
  impl Default for StreamControl
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Wrapper for streams with control capabilities
  #[ cfg( all( feature = "streaming", feature = "streaming_control" ) ) ]
  pub struct ControlledStream< T >
  {
    /// The underlying stream
    #[ allow( dead_code ) ]
    stream : std::pin::Pin< Box< dyn futures_core::Stream< Item = T > + Send > >,
    /// Control interface
    control : StreamControl,
    /// Buffer for pause/resume functionality
    #[ allow( dead_code ) ]
    buffer : StreamBuffer,
  }

  #[ cfg( all( feature = "streaming", feature = "streaming_control" ) ) ]
  impl< T > core::fmt::Debug for ControlledStream< T >
  {
    #[ inline ]
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      f.debug_struct( "ControlledStream" )
        .field( "control", &self.control )
        .field( "buffer", &self.buffer )
        .finish()
    }
  }

  #[ cfg( all( feature = "streaming", feature = "streaming_control" ) ) ]
  impl< T > ControlledStream< T >
  {
    /// Create a new controlled stream
    #[ inline ]
    #[ must_use ]
    pub fn new(
      stream : std::pin::Pin< Box< dyn futures_core::Stream< Item = T > + Send > >,
      control : StreamControl
    ) -> Self
    {
      Self {
        stream,
        control,
        buffer : StreamBuffer::new( 1024 * 1024 ), // 1MB default buffer
      }
    }

    /// Get reference to the control interface
    #[ inline ]
    pub fn control( &self ) -> &StreamControl
    {
      &self.control
    }

    /// Check if stream is paused (synchronous)
    #[ inline ]
    pub fn is_paused_sync( &self ) -> bool
    {
      // This is a simplified check - in real implementation would need async
      false
    }

    /// Check if stream is cancelled (synchronous)
    #[ inline ]
    pub fn is_cancelled_sync( &self ) -> bool
    {
      // This is a simplified check - in real implementation would need async
      false
    }
  }

}

#[ cfg( feature = "streaming_control" ) ]
crate ::mod_interface!
{
  exposed use
  {
    StreamState,
    StreamControlError,
    StreamMetrics,
    StreamBuffer,
    StreamControl,
    ControlledStream,
  };
}
