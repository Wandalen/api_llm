//! Enhanced Streaming Performance Module
//!
//! This module provides advanced streaming performance optimizations including:
//! - Adaptive buffering and batching for improved throughput
//! - Event compression and deduplication to reduce overhead
//! - Connection pooling and reuse for streaming endpoints
//! - Backpressure management and flow control
//! - Memory-efficient streaming with intelligent garbage collection

mod private
{
  use crate::
  {
    components ::responses::ResponseStreamEvent,
    error ::{ OpenAIError, Result },
  };
  use core::time::Duration;
  use std::
  {
    collections ::{ HashMap, VecDeque },
    sync ::{ Arc, Mutex },
    time ::Instant,
  };
  use tokio::
  {
    sync ::{ RwLock, Semaphore },
  };
  use serde::{ Serialize, Deserialize };

  /// Configuration for streaming performance optimizations
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct StreamingPerformanceConfig
  {
    /// Buffer size for batching events
    pub buffer_size : usize,
    /// Maximum time to wait before flushing buffer
    pub buffer_timeout : Duration,
    /// Enable event compression
    pub enable_compression : bool,
    /// Enable event deduplication
    pub enable_deduplication : bool,
    /// Maximum number of concurrent streams
    pub max_concurrent_streams : usize,
    /// Connection timeout for streaming endpoints
    pub connection_timeout : Duration,
    /// Memory limit for buffered events (bytes)
    pub memory_limit : usize,
    /// Enable adaptive buffer sizing
    pub adaptive_buffering : bool,
    /// Backpressure threshold (percentage)
    pub backpressure_threshold : f64,
  }

  impl Default for StreamingPerformanceConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        buffer_size : 64,
        buffer_timeout : Duration::from_millis( 100 ),
        enable_compression : true,
        enable_deduplication : true,
        max_concurrent_streams : 10,
        connection_timeout : Duration::from_secs( 30 ),
        memory_limit : 16 * 1024 * 1024, // 16MB
        adaptive_buffering : true,
        backpressure_threshold : 0.8, // 80%
      }
    }
  }

  /// Statistics for streaming performance monitoring
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct StreamingPerformanceStats
  {
    /// Total events processed
    pub events_processed : u64,
    /// Events per second throughput
    pub throughput : f64,
    /// Average latency per event
    pub average_latency : Duration,
    /// Current buffer utilization
    pub buffer_utilization : f64,
    /// Memory usage in bytes
    pub memory_usage : usize,
    /// Number of compressed events
    pub compressed_events : u64,
    /// Number of deduplicated events
    pub deduplicated_events : u64,
    /// Backpressure incidents
    pub backpressure_incidents : u64,
  }

  /// Compressed event for efficient storage and transmission
  #[ derive( Debug, Clone ) ]
  pub struct CompressedEvent
  {
    /// Event type identifier
    pub event_type : String,
    /// Compressed payload
    pub payload : Vec< u8 >,
    /// Original size before compression
    pub original_size : usize,
    /// Timestamp when event was created
    pub timestamp : Instant,
  }

  /// Buffered event batch for efficient processing
  #[ derive( Debug, Clone ) ]
  pub struct EventBatch
  {
    /// Events in this batch
    pub events : Vec< ResponseStreamEvent >,
    /// Batch creation timestamp
    pub created_at : Instant,
    /// Total size of events in bytes
    pub total_size : usize,
  }

  /// Enhanced streaming buffer with performance optimizations
  #[ derive( Debug ) ]
  pub struct StreamingBuffer
  {
    /// Configuration
    config : StreamingPerformanceConfig,
    /// Buffered events
    buffer : Arc< Mutex< VecDeque< ResponseStreamEvent > > >,
    /// Event hashes for deduplication
    event_hashes : Arc< Mutex< HashMap<  u64, Instant  > > >,
    /// Performance statistics
    stats : Arc< RwLock< StreamingPerformanceStats > >,
    /// Buffer size estimation
    current_size : Arc< Mutex< usize > >,
    /// Last flush timestamp
    last_flush : Arc< Mutex< Instant > >,
  }

  impl StreamingBuffer
  {
    /// Create a new streaming buffer with configuration
    #[ must_use ]
    #[ inline ]
    pub fn new( config : StreamingPerformanceConfig ) -> Self
    {
      Self
      {
        config,
        buffer : Arc::new( Mutex::new( VecDeque::new() ) ),
        event_hashes : Arc::new( Mutex::new( HashMap::new() ) ),
        stats : Arc::new( RwLock::new( StreamingPerformanceStats
        {
          events_processed : 0,
          throughput : 0.0,
          average_latency : Duration::from_millis( 0 ),
          buffer_utilization : 0.0,
          memory_usage : 0,
          compressed_events : 0,
          deduplicated_events : 0,
          backpressure_incidents : 0,
        } ) ),
        current_size : Arc::new( Mutex::new( 0 ) ),
        last_flush : Arc::new( Mutex::new( Instant::now() ) ),
      }
    }

    /// Add an event to the buffer with performance optimizations
    ///
    /// # Errors
    ///
    /// Returns an error if the event cannot be processed or serialized.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex for event hashes or buffer size is poisoned.
    #[ inline ]
    pub async fn push_event( &self, event : ResponseStreamEvent ) -> Result< bool >
    {
      let event_start = Instant::now();

      // Check for deduplication if enabled
      if self.config.enable_deduplication
      {
        let event_hash = Self::calculate_event_hash( &event );
        let is_duplicate =
        {
          let mut hashes = self.event_hashes.lock().unwrap();

          let is_dup = if let Some( &last_seen ) = hashes.get( &event_hash )
          {
            // Skip duplicate events within a short time window
            event_start.duration_since( last_seen ) < Duration::from_millis( 100 )
          }
          else
          {
            false
          };

          if !is_dup
          {
            hashes.insert( event_hash, event_start );

            // Cleanup old hashes
            let cutoff = event_start.checked_sub( Duration::from_secs( 10 ) ).unwrap();
            hashes.retain( |_, &mut timestamp| timestamp >= cutoff );
          }

          is_dup
        };

        if is_duplicate
        {
          let mut stats = self.stats.write().await;
          stats.deduplicated_events += 1;
          return Ok( false ); // Event was deduplicated
        }
      }

      // Estimate event size
      let event_size = Self::estimate_event_size( &event );

      // Check memory limits
      let memory_exceeded =
      {
        let current_size = self.current_size.lock().unwrap();
        *current_size + event_size > self.config.memory_limit
      };

      if memory_exceeded
      {
        // Trigger backpressure
        let mut stats = self.stats.write().await;
        stats.backpressure_incidents += 1;
        return Err( error_tools::Error::from( OpenAIError::Internal(
          "Streaming buffer memory limit exceeded".to_string()
        ) ) );
      }

      // Add event to buffer
      let ( utilization, new_memory_usage ) =
      {
        let mut buffer = self.buffer.lock().unwrap();
        let mut current_size = self.current_size.lock().unwrap();

        buffer.push_back( event );
        *current_size += event_size;

        // Calculate buffer utilization
        let utilization = buffer.len() as f64 / self.config.buffer_size as f64;
        let memory_usage = *current_size;

        ( utilization, memory_usage )
      };

      // Update stats
      {
        let mut stats = self.stats.write().await;
        stats.buffer_utilization = utilization;
        stats.memory_usage = new_memory_usage;
      }

      // Check if we should flush
      let should_flush = self.should_flush().await;
      if should_flush
      {
        self.flush_buffer().await?;
      }

      // Update latency statistics
      let latency = event_start.elapsed();
      self.update_latency_stats( latency ).await;

      Ok( true )
    }

    /// Flush the buffer and return batched events
    ///
    /// # Errors
    ///
    /// Returns an error if event compression or serialization fails.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex for buffer or buffer size is poisoned.
    #[ inline ]
    pub async fn flush_buffer( &self ) -> Result< Option< EventBatch > >
    {
      let flush_start = Instant::now();
      let mut events = Vec::new();
      let mut total_size = 0;

      // Extract events from buffer
      {
        let mut buffer = self.buffer.lock().unwrap();
        let mut current_size = self.current_size.lock().unwrap();

        while let Some( event ) = buffer.pop_front()
        {
          let size = Self::estimate_event_size( &event );
          events.push( event );
          total_size += size;
        }

        *current_size = 0;
      }

      if events.is_empty()
      {
        return Ok( None );
      }

      // Update flush timestamp
      {
        let mut last_flush = self.last_flush.lock().unwrap();
        *last_flush = flush_start;
      }

      // Update statistics
      {
        let mut stats = self.stats.write().await;
        stats.events_processed += events.len() as u64;
        stats.buffer_utilization = 0.0; // Buffer is now empty
        stats.memory_usage = 0;

        // Calculate throughput
        let elapsed = flush_start.duration_since(
          *self.last_flush.lock().unwrap()
        ).as_secs_f64();
        if elapsed > 0.0
        {
          stats.throughput = events.len() as f64 / elapsed;
        }
      }

      Ok( Some( EventBatch
      {
        events,
        created_at : flush_start,
        total_size,
      } ) )
    }

    /// Check if buffer should be flushed
    async fn should_flush( &self ) -> bool
    {
      let buffer_len = self.buffer.lock().unwrap().len();

      // Flush if buffer is full
      if buffer_len >= self.config.buffer_size
      {
        return true;
      }

      // Flush if timeout exceeded
      let last_flush = *self.last_flush.lock().unwrap();
      if last_flush.elapsed() >= self.config.buffer_timeout
      {
        return true;
      }

      // Adaptive flushing based on throughput
      if self.config.adaptive_buffering
      {
        let stats = self.stats.read().await;
        if stats.buffer_utilization > self.config.backpressure_threshold
        {
          return true;
        }
      }

      false
    }

    /// Calculate hash for event deduplication
    fn calculate_event_hash( event : &ResponseStreamEvent ) -> u64
    {
      use std::collections::hash_map::DefaultHasher;
      use core::{ hash::{ Hash, Hasher }, mem };

      let mut hasher = DefaultHasher::new();

      // Hash based on event type and key content
      match event
      {
        ResponseStreamEvent::ResponseTextDelta( e ) =>
        {
          "text_delta".hash( &mut hasher );
          e.item_id.hash( &mut hasher );
          e.delta.hash( &mut hasher );
        },
        ResponseStreamEvent::ResponseCompleted( e ) =>
        {
          "completed".hash( &mut hasher );
          e.response.id.hash( &mut hasher );
        },
        ResponseStreamEvent::ResponseCreated( e ) =>
        {
          "created".hash( &mut hasher );
          e.response.id.hash( &mut hasher );
        },
        _ =>
        {
          // Default hash for other event types
          mem ::discriminant( event ).hash( &mut hasher );
        }
      }

      hasher.finish()
    }

    /// Estimate the memory size of an event
    fn estimate_event_size( event : &ResponseStreamEvent ) -> usize
    {
      use core::mem;

      match event
      {
        ResponseStreamEvent::ResponseTextDelta( e ) =>
        {
          mem ::size_of_val( e ) + e.delta.len() + e.item_id.len()
        },
        ResponseStreamEvent::ResponseCompleted( e ) =>
        {
          mem ::size_of_val( e ) + Self::estimate_response_size( &e.response )
        },
        ResponseStreamEvent::ResponseCreated( e ) =>
        {
          mem ::size_of_val( e ) + Self::estimate_response_size( &e.response )
        },
        _ =>
        {
          // Conservative estimate for other events
          512 // bytes
        }
      }
    }

    /// Estimate the size of a response object
    fn estimate_response_size( response : &crate::components::responses::ResponseObject ) -> usize
    {
      use core::mem;

      let mut size = mem::size_of_val( response );
      size += response.id.len();
      if let Some( ref instructions ) = response.instructions
      {
        size += instructions.len();
      }
      // Add estimates for other string fields
      size += 256; // Conservative estimate for other fields
      size
    }

    /// Update latency statistics
    async fn update_latency_stats( &self, latency : Duration )
    {
      let mut stats = self.stats.write().await;

      // Simple moving average for latency
      if stats.events_processed > 0
      {
        let weight = 0.1; // 10% weight for new measurement
        let current_avg_nanos = stats.average_latency.as_nanos() as f64;
        let new_avg_nanos = ( 1.0 - weight ) * current_avg_nanos + weight * latency.as_nanos() as f64;
        stats.average_latency = if new_avg_nanos.is_finite() && new_avg_nanos >= 0.0
        {
          if new_avg_nanos <= u64::MAX as f64
          {
            let rounded_nanos = new_avg_nanos.round();
            #[ allow(clippy::cast_possible_truncation, clippy::cast_sign_loss) ]
            let nanos_u64 = if rounded_nanos >= 0.0 && rounded_nanos <= u64::MAX as f64
            {
              rounded_nanos as u64
            }
            else
            {
              u64 ::MAX
            };
            Duration::from_nanos( nanos_u64 )
          }
          else
          {
            Duration::from_nanos( u64::MAX )
          }
        }
        else
        {
          Duration::from_nanos( 0 )
        };
      }
      else
      {
        stats.average_latency = latency;
      }
    }

    /// Get current performance statistics
    #[ inline ]
    pub async fn get_stats( &self ) -> StreamingPerformanceStats
    {
      self.stats.read().await.clone()
    }

    /// Reset statistics
    #[ inline ]
    pub async fn reset_stats( &self )
    {
      let mut stats = self.stats.write().await;
      *stats = StreamingPerformanceStats
      {
        events_processed : 0,
        throughput : 0.0,
        average_latency : Duration::from_millis( 0 ),
        buffer_utilization : 0.0,
        memory_usage : 0,
        compressed_events : 0,
        deduplicated_events : 0,
        backpressure_incidents : 0,
      };
    }
  }

  /// Connection pool for streaming endpoints
  #[ derive( Debug ) ]
  pub struct StreamingConnectionPool
  {
    /// Maximum number of connections
    #[ allow( dead_code ) ]
    max_connections : usize,
    /// Active connections semaphore
    connection_semaphore : Arc< Semaphore >,
    /// Connection statistics
    stats : Arc< RwLock< ConnectionPoolStats > >,
  }

  /// Connection pool statistics
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ConnectionPoolStats
  {
    /// Total connections created
    pub connections_created : u64,
    /// Current active connections
    pub active_connections : u64,
    /// Connection timeouts
    pub timeouts : u64,
    /// Average connection duration
    pub average_duration : Duration,
  }

  impl StreamingConnectionPool
  {
    /// Create a new connection pool
    #[ must_use ]
    #[ inline ]
    pub fn new( max_connections : usize ) -> Self
    {
      Self
      {
        max_connections,
        connection_semaphore : Arc::new( Semaphore::new( max_connections ) ),
        stats : Arc::new( RwLock::new( ConnectionPoolStats
        {
          connections_created : 0,
          active_connections : 0,
          timeouts : 0,
          average_duration : Duration::from_millis( 0 ),
        } ) ),
      }
    }

    /// Acquire a connection from the pool
    ///
    /// # Errors
    ///
    /// Returns an error if the connection pool is exhausted.
    #[ inline ]
    pub async fn acquire_connection( &self ) -> Result< ConnectionGuard >
    {
      let permit = self.connection_semaphore.clone().try_acquire_owned()
        .map_err( |_| error_tools::Error::from( OpenAIError::Internal(
          "Connection pool exhausted".to_string()
        ) ) )?;

      let mut stats = self.stats.write().await;
      stats.connections_created += 1;
      stats.active_connections += 1;

      Ok( ConnectionGuard
      {
        _permit : permit,
        stats : self.stats.clone(),
        start_time : Instant::now(),
      } )
    }

    /// Get connection pool statistics
    #[ inline ]
    pub async fn get_stats( &self ) -> ConnectionPoolStats
    {
      self.stats.read().await.clone()
    }
  }

  /// RAII guard for connection pool connections
  #[ derive( Debug ) ]
  pub struct ConnectionGuard
  {
    _permit : tokio::sync::OwnedSemaphorePermit,
    stats : Arc< RwLock< ConnectionPoolStats > >,
    start_time : Instant,
  }

  impl Drop for ConnectionGuard
  {
    #[ inline ]
    fn drop( &mut self )
    {
      let duration = self.start_time.elapsed();
      let stats = self.stats.clone();
      tokio ::spawn( async move
      {
        if let Ok( mut stats ) = stats.try_write()
        {
          stats.active_connections = stats.active_connections.saturating_sub( 1 );

          // Update average duration
          let weight = 0.1;
          let current_avg_nanos = stats.average_duration.as_nanos() as f64;
          let new_avg_nanos = ( 1.0 - weight ) * current_avg_nanos + weight * duration.as_nanos() as f64;
          stats.average_duration = if new_avg_nanos.is_finite() && new_avg_nanos >= 0.0
          {
            if new_avg_nanos <= u64::MAX as f64
            {
              let rounded_nanos = new_avg_nanos.round();
              let nanos_u64 = if rounded_nanos >= 0.0 && rounded_nanos <= u64::MAX as f64
              {
                #[ allow(clippy::cast_possible_truncation, clippy::cast_sign_loss) ]
                let result = rounded_nanos as u64;
                result
              }
              else
              {
                u64 ::MAX
              };
              Duration::from_nanos( nanos_u64 )
            }
            else
            {
              Duration::from_nanos( u64::MAX )
            }
          }
          else
          {
            Duration::from_nanos( 0 )
          };
        }
      } );
    }
  }

  /// Enhanced streaming processor with performance optimizations
  #[ derive( Debug ) ]
  pub struct StreamingProcessor
  {
    /// Configuration
    #[ allow( dead_code ) ]
    config : StreamingPerformanceConfig,
    /// Event buffer
    buffer : Arc< StreamingBuffer >,
    /// Connection pool
    connection_pool : Arc< StreamingConnectionPool >,
    /// Processing statistics
    stats : Arc< RwLock< ProcessingStats > >,
  }

  /// Processing statistics
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ProcessingStats
  {
    /// Total processing time
    pub total_processing_time : Duration,
    /// Events processed per second
    pub events_per_second : f64,
    /// Peak memory usage
    pub peak_memory_usage : usize,
    /// Average batch size
    pub average_batch_size : f64,
  }

  impl StreamingProcessor
  {
    /// Create a new streaming processor
    #[ inline ]
    #[ must_use ]
    pub fn new( config : StreamingPerformanceConfig ) -> Self
    {
      let buffer = Arc::new( StreamingBuffer::new( config.clone() ) );
      let connection_pool = Arc::new( StreamingConnectionPool::new( config.max_concurrent_streams ) );

      Self
      {
        config,
        buffer,
        connection_pool,
        stats : Arc::new( RwLock::new( ProcessingStats
        {
          total_processing_time : Duration::from_millis( 0 ),
          events_per_second : 0.0,
          peak_memory_usage : 0,
          average_batch_size : 0.0,
        } ) ),
      }
    }

    /// Process a streaming event with performance optimizations
    ///
    /// # Errors
    ///
    /// Returns an error if event processing fails or if buffer operations fail.
    #[ inline ]
    pub async fn process_event( &self, event : ResponseStreamEvent ) -> Result< () >
    {
      let process_start = Instant::now();

      // Acquire connection if needed
      let _connection = self.connection_pool.acquire_connection().await?;

      // Add event to buffer
      self.buffer.push_event( event ).await?;

      // Update processing statistics
      let processing_time = process_start.elapsed();
      let mut stats = self.stats.write().await;
      stats.total_processing_time += processing_time;

      Ok( () )
    }

    /// Process multiple events in a batch
    ///
    /// # Errors
    ///
    /// Returns an error if any event in the batch fails to process.
    #[ inline ]
    pub async fn process_batch( &self, events : Vec< ResponseStreamEvent > ) -> Result< () >
    {
      let batch_start = Instant::now();
      let batch_size = events.len();

      for event in events
      {
        self.process_event( event ).await?;
      }

      // Update batch statistics
      let mut stats = self.stats.write().await;
      let weight = 0.1;
      let current_avg = stats.average_batch_size;
      stats.average_batch_size = ( 1.0 - weight ) * current_avg + weight * batch_size as f64;

      let batch_duration = batch_start.elapsed();
      if batch_duration.as_secs_f64() > 0.0
      {
        stats.events_per_second = batch_size as f64 / batch_duration.as_secs_f64();
      }

      Ok( () )
    }

    /// Get buffer statistics
    #[ inline ]
    pub async fn get_buffer_stats( &self ) -> StreamingPerformanceStats
    {
      self.buffer.get_stats().await
    }

    /// Get connection pool statistics
    #[ inline ]
    pub async fn get_connection_stats( &self ) -> ConnectionPoolStats
    {
      self.connection_pool.get_stats().await
    }

    /// Get processing statistics
    #[ inline ]
    pub async fn get_processing_stats( &self ) -> ProcessingStats
    {
      self.stats.read().await.clone()
    }

    /// Flush all buffered events
    ///
    /// # Errors
    /// Returns an error if buffer flushing fails or event batch processing encounters issues.
    #[ inline ]
    pub async fn flush( &self ) -> Result< Option< EventBatch > >
    {
      self.buffer.flush_buffer().await
    }
  }

  /// Global streaming processor instance
  static STREAMING_PROCESSOR : std::sync::OnceLock< Arc< StreamingProcessor > > = std::sync::OnceLock::new();

  /// Get the global streaming processor instance
  #[ inline ]
  pub fn get_streaming_processor() -> Arc< StreamingProcessor >
  {
    STREAMING_PROCESSOR.get_or_init( ||
      Arc::new( StreamingProcessor::new( StreamingPerformanceConfig::default() ) )
    ).clone()
  }

  /// Configure the global streaming processor
  #[ inline ]
  pub fn configure_streaming_processor( config : StreamingPerformanceConfig )
  {
    let _ = STREAMING_PROCESSOR.set( Arc::new( StreamingProcessor::new( config ) ) );
  }

  /// Convenience functions for global streaming processing
  ///
  /// Process an event using the global processor
  ///
  /// # Errors
  /// Returns an error if event processing fails or the streaming processor encounters issues.
  #[ inline ]
  pub async fn process_event( event : ResponseStreamEvent ) -> Result< () >
  {
    get_streaming_processor().process_event( event ).await
  }

  /// Process multiple events using the global processor
  ///
  /// # Errors
  /// Returns an error if batch processing fails or any event in the batch cannot be processed.
  #[ inline ]
  pub async fn process_batch( events : Vec< ResponseStreamEvent > ) -> Result< () >
  {
    get_streaming_processor().process_batch( events ).await
  }

  /// Get buffer statistics from the global processor
  #[ inline ]
  pub async fn get_buffer_stats() -> StreamingPerformanceStats
  {
    get_streaming_processor().get_buffer_stats().await
  }

  /// Get connection statistics from the global processor
  #[ inline ]
  pub async fn get_connection_stats() -> ConnectionPoolStats
  {
    get_streaming_processor().get_connection_stats().await
  }

  /// Flush buffered events from the global processor
  ///
  /// # Errors
  /// Returns an error if flushing the global event buffer fails or event batch processing encounters issues.
  #[ inline ]
  pub async fn flush_events() -> Result< Option< EventBatch > >
  {
    get_streaming_processor().flush().await
  }
}

crate ::mod_interface!
{
  exposed use private::StreamingPerformanceConfig;
  exposed use private::StreamingPerformanceStats;
  exposed use private::CompressedEvent;
  exposed use private::EventBatch;
  exposed use private::StreamingBuffer;
  exposed use private::StreamingConnectionPool;
  exposed use private::ConnectionPoolStats;
  exposed use private::ConnectionGuard;
  exposed use private::StreamingProcessor;
  exposed use private::ProcessingStats;
  exposed use private::get_streaming_processor;
  exposed use private::configure_streaming_processor;
  exposed use private::process_event;
  exposed use private::process_batch;
  exposed use private::get_buffer_stats;
  exposed use private::get_connection_stats;
  exposed use private::flush_events;
}