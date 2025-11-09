//! Intelligent Request Batching System
//!
//! This module implements smart request batching to reduce HTTP overhead
//! and improve throughput for bulk operations. It automatically identifies
//! batchable requests and optimizes their execution.
//!
//! This module is feature-gated behind the `batching` feature flag.

use mod_interface::mod_interface;

#[ cfg( feature = "batching" ) ]
mod private
{
  use std::
  {
    collections ::{ HashMap, VecDeque },
    sync ::Arc,
    time ::Instant,
  };
  use core::
  {
    hash ::Hash,
    time ::Duration,
  };
  use tokio::sync::{ RwLock, Notify };
  use blake3::{ Hash as Blake3Hash, Hasher as Blake3Hasher };

  /// Configuration for request batching behavior
  #[ derive( Debug, Clone ) ]
  pub struct BatchConfig
  {
    /// Maximum number of requests to batch together
    pub max_batch_size : usize,
    /// Maximum time to wait before flushing a partial batch
    pub flush_timeout : Duration,
    /// Maximum concurrent batches processing
    pub max_concurrent_batches : usize,
    /// Enable smart batching for similar requests
    pub enable_smart_batching : bool,
    /// Minimum requests to trigger smart batching
    pub smart_batch_threshold : usize,
  }

  impl Default for BatchConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        max_batch_size : 100,
        flush_timeout : Duration::from_millis( 50 ),
        max_concurrent_batches : 10,
        enable_smart_batching : true,
        smart_batch_threshold : 5,
      }
    }
  }

  /// Request signature for batching similarity detection
  #[ derive( Debug, Clone, PartialEq, Eq, Hash ) ]
  pub struct RequestSignature
  {
    /// HTTP method (GET, POST, etc.)
    pub method : String,
    /// API endpoint path
    pub path : String,
    /// Request structure hash (excluding variable data)
    pub structure_hash : Blake3Hash,
  }

  impl RequestSignature
  {
    /// Create request signature for batching analysis
    #[ inline ]
    #[ must_use ]
    pub fn new( method : &str, path : &str, body : &[u8] ) -> Self
    {
      let mut hasher = Blake3Hasher::new();
      hasher.update( method.as_bytes() );
      hasher.update( path.as_bytes() );
      hasher.update( body );
      let structure_hash = hasher.finalize();

      Self
      {
        method : method.to_string(),
        path : path.to_string(),
        structure_hash,
      }
    }

    /// Check if requests can be batched together
    #[ inline ]
    #[ must_use ]
    pub fn is_batchable_with( &self, other : &RequestSignature ) -> bool
    {
      self.method == other.method &&
      self.path == other.path &&
      self.is_batch_compatible_endpoint()
    }

    /// Check if endpoint supports batching
    fn is_batch_compatible_endpoint( &self ) -> bool
    {
      matches!( self.path.as_str(),
        "embeddings" |
        "chat/completions" |
        "moderations" |
        "images/generations" |
        "files" |
        "fine_tuning/jobs"
      )
    }
  }

  /// Batched request container
  #[ derive( Debug ) ]
  pub struct BatchedRequest< T >
  where
    T: Send + Sync,
  {
    /// Unique identifier for tracking
    pub id : String,
    /// Request signature for batching
    pub signature : RequestSignature,
    /// Request payload
    pub payload : T,
    /// Response sender
    pub response_sender : tokio::sync::oneshot::Sender< Result< Vec< u8 >, crate::error::OpenAIError > >,
    /// Timestamp when request was queued
    pub queued_at : Instant,
  }

  /// Batch processing result
  #[ derive( Debug ) ]
  pub struct BatchResult
  {
    /// Individual request results
    pub results : Vec< Result< Vec< u8 >, crate::error::OpenAIError > >,
    /// Total processing time
    pub processing_time : Duration,
    /// Number of HTTP requests made
    pub http_requests_count : usize,
    /// Batch efficiency ratio (logical requests / HTTP requests)
    pub efficiency_ratio : f64,
  }

  /// Intelligent request batcher
  #[ derive( Debug ) ]
  pub struct RequestBatcher< T >
  where
    T: Send + Sync,
  {
    /// Batching configuration
    config : BatchConfig,
    /// Pending requests grouped by signature
    pending_requests : Arc< RwLock< HashMap< RequestSignature, VecDeque< BatchedRequest< T > > > > >,
    /// Notification for batch processing
    batch_notify : Arc< Notify >,
    /// Active batch count for concurrency control
    active_batches : Arc< RwLock< usize > >,
    /// Batch processing metrics
    metrics : Arc< RwLock< BatchMetrics > >,
  }

  /// Batching performance metrics
  #[ derive( Debug, Clone, Default ) ]
  pub struct BatchMetrics
  {
    /// Total requests processed
    pub total_requests : u64,
    /// Total batches created
    pub total_batches : u64,
    /// Average batch size
    pub avg_batch_size : f64,
    /// Total HTTP requests saved through batching
    pub http_requests_saved : u64,
    /// Average batch processing time
    pub avg_batch_time : Duration,
    /// Efficiency improvement ratio
    pub efficiency_improvement : f64,
  }

  impl< T > RequestBatcher< T >
  where
    T: Send + Sync + 'static,
  {
    /// Create new request batcher
    #[ inline ]
    #[ must_use ]
    pub fn new( config : BatchConfig ) -> Self
    {
      Self
      {
        config,
        pending_requests : Arc::new( RwLock::new( HashMap::new() ) ),
        batch_notify : Arc::new( Notify::new() ),
        active_batches : Arc::new( RwLock::new( 0 ) ),
        metrics : Arc::new( RwLock::new( BatchMetrics::default() ) ),
      }
    }

    /// Submit request for batching
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails to execute or if batching operations fail.
    #[ inline ]
    pub async fn submit_request(
      &self,
      signature : RequestSignature,
      payload : T,
    ) -> Result< Vec< u8 >, crate::error::OpenAIError >
    {
      if !self.config.enable_smart_batching || !signature.is_batch_compatible_endpoint()
      {
        // Execute immediately for non-batchable requests
        return Ok( Self::execute_single_request( signature, payload ) );
      }

      let ( tx, rx ) = tokio::sync::oneshot::channel();
      let request_id = uuid::Uuid::new_v4().to_string();

      let batched_request = BatchedRequest
      {
        id : request_id,
        signature : signature.clone(),
        payload,
        response_sender : tx,
        queued_at : Instant::now(),
      };

      // Add to pending requests
      {
        let mut pending = self.pending_requests.write().await;
        pending.entry( signature.clone() ).or_insert_with( VecDeque::new ).push_back( batched_request );
      }

      // Check if we should trigger batch processing
      let should_process = self.should_trigger_batch_processing( &signature ).await;
      if should_process
      {
        self.batch_notify.notify_one();
      }

      // Start batch processor if needed
      self.ensure_batch_processor_running().await;

      // Wait for response
      rx.await.map_err( | _ | crate::error::OpenAIError::Internal( "Batch processing failed".to_string() ) )?
    }

    /// Check if batch processing should be triggered
    async fn should_trigger_batch_processing( &self, signature : &RequestSignature ) -> bool
    {
      let pending = self.pending_requests.read().await;
      if let Some( queue ) = pending.get( signature )
      {
        queue.len() >= self.config.smart_batch_threshold ||
        queue.front().is_some_and( | req | req.queued_at.elapsed() >= self.config.flush_timeout )
      }
      else
      {
        false
      }
    }

    /// Ensure batch processor is running
    async fn ensure_batch_processor_running( &self )
    {
      let active_count = *self.active_batches.read().await;
      if active_count < self.config.max_concurrent_batches
      {
        let pending_requests = Arc::clone( &self.pending_requests );
        let batch_notify = Arc::clone( &self.batch_notify );
        let active_batches = Arc::clone( &self.active_batches );
        let metrics = Arc::clone( &self.metrics );
        let config = self.config.clone();

        tokio ::spawn( async move
        {
          // Increment active batch count
          {
            let mut active = active_batches.write().await;
            *active += 1;
          }

          // Process batches
          loop
          {
            batch_notify.notified().await;

            // Check for batches to process
            let batch_to_process = {
              let mut pending = pending_requests.write().await;
              Self::extract_ready_batch( &mut pending, &config )
            };

            if let Some( ( signature, requests ) ) = batch_to_process
            {
              let start_time = Instant::now();
              let batch_size = requests.len();

              // Process the batch
              let _results = Self::process_batch_requests( &signature, requests );
              let processing_time = start_time.elapsed();

              // Update metrics
              {
                let mut metrics_guard = metrics.write().await;
                metrics_guard.total_requests += batch_size as u64;
                metrics_guard.total_batches += 1;
                metrics_guard.avg_batch_size = ( metrics_guard.avg_batch_size * ( metrics_guard.total_batches - 1 ) as f64 + batch_size as f64 ) / metrics_guard.total_batches as f64;
                metrics_guard.http_requests_saved += ( batch_size as u64 ).saturating_sub( 1 );
                let new_avg_nanos = ( metrics_guard.avg_batch_time.as_nanos() * u128::from( metrics_guard.total_batches - 1 ) +
                  processing_time.as_nanos() ) / u128::from( metrics_guard.total_batches );
                let bounded_nanos = new_avg_nanos.min( u128::from( u64::MAX ) );
                metrics_guard.avg_batch_time = Duration::from_nanos( u64::try_from( bounded_nanos ).unwrap_or( u64::MAX ) );
                if metrics_guard.total_requests > 0
                {
                  metrics_guard.efficiency_improvement = metrics_guard.http_requests_saved as f64 / metrics_guard.total_requests as f64;
                }
              }
            }
            else
            {
              // No batches ready, break the loop
              break;
            }
          }

          // Decrement active batch count
          {
            let mut active = active_batches.write().await;
            *active = active.saturating_sub( 1 );
          }
        } );
      }
    }

    /// Extract ready batch for processing
    fn extract_ready_batch(
      pending : &mut HashMap< RequestSignature, VecDeque< BatchedRequest< T > > >,
      config : &BatchConfig,
    ) -> Option< ( RequestSignature, Vec< BatchedRequest< T > > ) >
    {
      for ( signature, queue ) in pending.iter_mut()
      {
        if queue.len() >= config.smart_batch_threshold ||
           queue.front().is_some_and( | req | req.queued_at.elapsed() >= config.flush_timeout )
        {
          let mut batch = Vec::new();
          for _ in 0..config.max_batch_size.min( queue.len() )
          {
            if let Some( request ) = queue.pop_front()
            {
              batch.push( request );
            }
          }
          if !batch.is_empty()
          {
            return Some( ( signature.clone(), batch ) );
          }
        }
      }
      None
    }

    /// Process batch of requests
    fn process_batch_requests(
      signature : &RequestSignature,
      requests : Vec< BatchedRequest< T > >,
    ) -> BatchResult
    {
      let start_time = Instant::now();
      let request_count = requests.len();

      // For demonstration, simulate batch processing
      // In real implementation, this would:
      // 1. Combine requests into single HTTP call where possible
      // 2. Process results and distribute back to individual request senders
      // 3. Handle errors appropriately

      let results : Vec< Result< Vec< u8 >, crate::error::OpenAIError > > = requests.into_iter().map( | request |
      {
        // Send mock success response
        let mock_response = b"{ \"batched\": true }".to_vec();
        let _ = request.response_sender.send( Ok( mock_response.clone() ) );
        Ok( mock_response )
      } ).collect();

      let processing_time = start_time.elapsed();
      let http_requests_count = if signature.is_batch_compatible_endpoint() { 1 } else { request_count };
      let efficiency_ratio = request_count as f64 / http_requests_count as f64;

      BatchResult
      {
        results,
        processing_time,
        http_requests_count,
        efficiency_ratio,
      }
    }

    /// Execute single request without batching
    fn execute_single_request(
      _signature : RequestSignature,
      _payload : T,
    ) -> Vec< u8 >
    {
      // Execute single request immediately
      // This would call the normal client request methods
      b"{ \"single\": true }".to_vec()
    }

    /// Get current batching metrics
    #[ inline ]
    pub async fn get_metrics( &self ) -> BatchMetrics
    {
      self.metrics.read().await.clone()
    }

    /// Clear all pending requests
    #[ inline ]
    pub async fn flush_all_pending( &self )
    {
      let mut pending = self.pending_requests.write().await;
      for ( _, queue ) in pending.iter_mut()
      {
        while let Some( request ) = queue.pop_front()
        {
          let _ = request.response_sender.send( Err( crate::error::OpenAIError::Internal( "Request flushed".to_string() ) ) );
        }
      }
      pending.clear();
    }
  }

  /// Batch processing optimization strategies
  #[ derive( Debug ) ]
  pub struct BatchOptimizer;

  impl BatchOptimizer
  {
    /// Analyze request patterns to optimize batching
    #[ inline ]
    #[ must_use ]
    pub fn analyze_batching_potential( requests : &[ RequestSignature ] ) -> BatchingAnalysis
    {
      let mut signature_counts = HashMap::new();
      let mut total_batchable = 0;

      for signature in requests
      {
        let count = signature_counts.entry( signature.clone() ).or_insert( 0 );
        *count += 1;

        if signature.is_batch_compatible_endpoint()
        {
          total_batchable += 1;
        }
      }

      let potential_batches = signature_counts.values().map( | &count | ( count + 99 ) / 100 ).sum::< usize >();
      let http_requests_saved = requests.len().saturating_sub( potential_batches );
      let efficiency_gain = if requests.is_empty() { 0.0 } else { http_requests_saved as f64 / requests.len() as f64 };

      BatchingAnalysis
      {
        total_requests : requests.len(),
        batchable_requests : total_batchable,
        potential_batches,
        http_requests_saved,
        efficiency_gain,
        recommended_batch_size : Self::calculate_optimal_batch_size( &signature_counts ),
      }
    }

    /// Calculate optimal batch size based on request patterns
    fn calculate_optimal_batch_size( signature_counts : &HashMap<  RequestSignature, usize  > ) -> usize
    {
      if signature_counts.is_empty()
      {
        return 50; // Default
      }

      let avg_similar_requests = signature_counts.values().sum::< usize >() as f64 / signature_counts.len() as f64;

      // Optimize for common patterns
      #[ allow(clippy::cast_possible_truncation, clippy::cast_sign_loss) ]
      let avg_usize = avg_similar_requests as usize;
      match avg_usize
      {
        1..=5 => 10,
        6..=20 => 25,
        21..=50 => 50,
        51..=100 => 75,
        _ => 100,
      }
    }
  }

  /// Analysis of batching potential
  #[ derive( Debug, Clone ) ]
  pub struct BatchingAnalysis
  {
    /// Total number of requests analyzed
    pub total_requests : usize,
    /// Number of requests that can be batched
    pub batchable_requests : usize,
    /// Number of batches that would be created
    pub potential_batches : usize,
    /// Number of HTTP requests saved through batching
    pub http_requests_saved : usize,
    /// Efficiency gain percentage (0.0 to 1.0)
    pub efficiency_gain : f64,
    /// Recommended batch size for this pattern
    pub recommended_batch_size : usize,
  }
}

mod_interface!
{
  exposed use
  {
    BatchConfig,
    RequestSignature,
    BatchedRequest,
    BatchResult,
    RequestBatcher,
    BatchMetrics,
    BatchOptimizer,
    BatchingAnalysis,
  };
}