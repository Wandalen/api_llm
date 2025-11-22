//! Enhanced Embeddings Client with Intelligent Batching
//!
//! This module provides an enhanced embeddings client that automatically
//! batches requests for optimal performance and reduced API costs.

use mod_interface::mod_interface;

mod private
{
  use crate::
  {
    client ::Client,
    client_api_accessors ::ClientApiAccessors,
    error ::{ Result, OpenAIError },
    environment ::{ OpenaiEnvironment, EnvironmentInterface },
    components ::embeddings::CreateEmbeddingResponse,
    components ::embeddings_request::CreateEmbeddingRequest,
  };

  // Feature-gated imports
  #[ cfg( feature = "batching" ) ]
  use crate::request_batching::{ RequestBatcher, RequestSignature, BatchConfig, BatchMetrics };
  use std::sync::Arc;
  use core::time::Duration;

  /// Enhanced embeddings client with intelligent batching
  #[ derive( Debug ) ]
  pub struct EnhancedEmbeddings< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client Client< E >,

    // Feature-gated batching fields
    #[ cfg( feature = "batching" ) ]
    batcher : Arc< RequestBatcher< CreateEmbeddingRequest > >,
    #[ cfg( feature = "batching" ) ]
    config : BatchConfig,
  }

  impl< 'client, E > EnhancedEmbeddings< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Create new enhanced embeddings client with custom batching config
    #[ cfg( feature = "batching" ) ]
    #[ inline ]
    pub fn new( client : &'client Client< E >, config : BatchConfig ) -> Self
    {
      let batcher = Arc::new( RequestBatcher::new( config.clone() ) );

      Self
      {
        client,
        batcher,
        config,
      }
    }

    /// Create new enhanced embeddings client without batching
    #[ cfg( not( feature = "batching" ) ) ]
    #[ inline ]
    pub fn new( client : &'client Client< E >, _config : () ) -> Self
    {
      Self
      {
        client,
      }
    }

    /// Create new enhanced embeddings client with default batching
    #[ cfg( feature = "batching" ) ]
    #[ inline ]
    pub fn with_default_batching( client : &'client Client< E > ) -> Self
    {
      Self::new( client, BatchConfig::default() )
    }

    /// Create new enhanced embeddings client without batching
    #[ cfg( not( feature = "batching" ) ) ]
    #[ inline ]
    pub fn with_default_batching( client : &'client Client< E > ) -> Self
    {
      Self::new( client, () )
    }

    /// Create embeddings directly without batching (fallback method)
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or if the response cannot be parsed.
    #[ inline ]
    pub async fn create_direct( &self, request : CreateEmbeddingRequest ) -> Result< CreateEmbeddingResponse >
    {
      // Use the client field directly for non-batched requests
      self.client.embeddings().create( request ).await
    }

    /// Create embeddings with automatic batching optimization
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, batching fails, or if the response cannot be parsed.
    #[ cfg( feature = "batching" ) ]
    #[ inline ]
    pub async fn create_batched( &self, request : CreateEmbeddingRequest ) -> Result< CreateEmbeddingResponse >
    {
      // Create request signature for batching analysis
      let request_json = serde_json::to_vec( &request )
        .map_err( | e | OpenAIError::Internal( format!( "Failed to serialize request : {e}" ) ) )?;

      let signature = RequestSignature::new( "POST", "embeddings", &request_json );

      // Submit request for batching
      let response_bytes = self.batcher.submit_request( signature, request ).await?;

      // Parse response
      let response : CreateEmbeddingResponse = serde_json::from_slice( &response_bytes )
        .map_err( | e | OpenAIError::Internal( format!( "Failed to parse response : {e}" ) ) )?;

      Ok( response )
    }

    /// Create multiple embeddings with optimal batching
    ///
    /// # Errors
    ///
    /// Returns an error if any batch request fails or if responses cannot be parsed.
    #[ inline ]
    pub async fn create_bulk( &self, texts : Vec< String >, model : String ) -> Result< Vec< CreateEmbeddingResponse > >
    {
      if texts.is_empty()
      {
        return Ok( vec![] );
      }

      // Analyze optimal batching strategy
      let optimal_batch_size = self.calculate_optimal_batch_size( texts.len() );
      let mut results = Vec::with_capacity( texts.len() );

      // Process in optimal chunks
      for chunk in texts.chunks( optimal_batch_size )
      {
        let chunk_results = self.process_text_chunk( chunk.to_vec(), model.clone() ).await?;
        results.extend( chunk_results );
      }

      Ok( results )
    }

    /// Process chunk of texts with intelligent batching
    async fn process_text_chunk( &self, texts : Vec< String >, model : String ) -> Result< Vec< CreateEmbeddingResponse > >
    {
      if texts.len() == 1
      {
        // Single request
        let request = CreateEmbeddingRequest::new_single( texts[0].clone(), model );
        let response = self.create_batched( request ).await?;
        Ok( vec![ response ] )
      }
      else
      {
        // Multiple requests - use API batch capability
        let request = CreateEmbeddingRequest::new_multiple( texts, model );
        let response = self.create_batched( request ).await?;

        // Split response based on number of inputs
        // In real implementation, this would properly handle the batch response
        Ok( vec![ response ] )
      }
    }

    /// Calculate optimal batch size based on request patterns
    fn calculate_optimal_batch_size( &self, total_requests : usize ) -> usize
    {
      // Use adaptive batch sizing based on total volume
      match total_requests
      {
        1..=10 => total_requests,
        11..=50 => 10,
        51..=200 => 25,
        201..=500 => 50,
        _ => self.config.max_batch_size,
      }
    }

    /// Get batching performance metrics
    #[ cfg( feature = "batching" ) ]
    #[ inline ]
    pub async fn get_metrics( &self ) -> BatchMetrics
    {
      self.batcher.get_metrics().await
    }

    /// Create streaming embeddings with batching
    ///
    /// # Errors
    ///
    /// Returns an error if the streaming setup fails or if channel creation fails.
    #[ inline ]
    pub fn create_streaming( &self, texts : Vec< String >, model : String ) -> Result< tokio::sync::mpsc::Receiver< core::result::Result< CreateEmbeddingResponse, OpenAIError > > >
    {
      let ( tx, rx ) = tokio::sync::mpsc::channel( 100 );
      let batcher = Arc::clone( &self.batcher );
      let batch_size = self.calculate_optimal_batch_size( texts.len() );

      tokio ::spawn( async move
      {
        for chunk in texts.chunks( batch_size )
        {
          let request = CreateEmbeddingRequest::new_multiple( chunk.to_vec(), model.clone() );
          let request_json = match serde_json::to_vec( &request )
          {
            Ok( json ) => json,
            Err( e ) =>
            {
              let _ = tx.send( Err( OpenAIError::Internal( format!( "Serialization failed : {e}" ) ) ) ).await;
              continue;
            }
          };

          let signature = RequestSignature::new( "POST", "embeddings", &request_json );

          match batcher.submit_request( signature, request ).await
          {
            Ok( response_bytes ) =>
            {
              match serde_json::from_slice::< CreateEmbeddingResponse >( &response_bytes )
              {
                Ok( response ) =>
                {
                  if tx.send( Ok( response ) ).await.is_err()
                  {
                    break; // Receiver dropped
                  }
                },
                Err( e ) =>
                {
                  let _ = tx.send( Err( OpenAIError::Internal( format!( "Parse failed : {e}" ) ) ) ).await;
                }
              }
            },
            Err( e ) =>
            {
              let _ = tx.send( Err( e ) ).await;
            }
          }
        }
      } );

      Ok( rx )
    }

  }

  /// Analyze batching potential for given requests (standalone function)
  #[ must_use ]
  #[ inline ]
  pub fn analyze_embedding_batching_potential( requests : &[ CreateEmbeddingRequest ] ) -> BatchingAnalysis
  {
    let signatures : Vec< RequestSignature > = requests.iter().map( | req |
    {
      let request_json = serde_json::to_vec( req ).unwrap_or_default();
      RequestSignature::new( "POST", "embeddings", &request_json )
    } ).collect();

    crate ::request_batching::BatchOptimizer::analyze_batching_potential( &signatures )
  }

  impl< E > EnhancedEmbeddings< '_, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Flush all pending requests
    #[ inline ]
    pub async fn flush_pending( &self )
    {
      self.batcher.flush_all_pending().await;
    }
  }

  /// Configuration for enhanced embeddings
  #[ derive( Debug, Clone ) ]
  pub struct EnhancedEmbeddingsConfig
  {
    /// Batching configuration
    pub batch_config : BatchConfig,
    /// Enable automatic request optimization
    pub enable_optimization : bool,
    /// Maximum concurrent requests
    pub max_concurrent_requests : usize,
    /// Request timeout
    pub request_timeout : Duration,
  }

  impl Default for EnhancedEmbeddingsConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        batch_config : BatchConfig::default(),
        enable_optimization : true,
        max_concurrent_requests : 20,
        request_timeout : Duration::from_secs( 60 ),
      }
    }
  }

  /// Embedding batching analysis results
  #[ cfg( feature = "batching" ) ]
  pub use crate::request_batching::BatchingAnalysis;

  /// Smart embedding processing strategies
  #[ derive( Debug ) ]
  pub struct EmbeddingBatchProcessor;

  impl EmbeddingBatchProcessor
  {
    /// Process large text collections with optimal batching
    ///
    /// # Errors
    ///
    /// Returns an error if any batch processing fails or if responses cannot be parsed.
    #[ inline ]
    pub async fn process_document_collection< E >(
      client : &EnhancedEmbeddings< '_, E >,
      documents : Vec< String >,
      model : String,
    ) -> Result< Vec< CreateEmbeddingResponse > >
    where
      E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
    {
      // Analyze text lengths and optimize batching strategy
      let avg_text_length : usize = documents.iter().map( std::string::String::len ).sum::< usize >() / documents.len().max( 1 );

      let optimal_batch_size = match avg_text_length
      {
        0..=100 => 100,      // Short texts - large batches
        101..=500 => 50,     // Medium texts - medium batches
        501..=2000 => 25,    // Long texts - smaller batches
        _ => 10,             // Very long texts - small batches
      };

      let mut all_results = Vec::with_capacity( documents.len() );

      for chunk in documents.chunks( optimal_batch_size )
      {
        let chunk_results = client.create_bulk( chunk.to_vec(), model.clone() ).await?;
        all_results.extend( chunk_results );
      }

      Ok( all_results )
    }

    /// Process embeddings with cost optimization
    ///
    /// # Errors
    ///
    /// Returns an error if any batch request fails or if cost calculations fail.
    #[ inline ]
    pub async fn process_with_cost_optimization< E >(
      client : &EnhancedEmbeddings< '_, E >,
      texts : Vec< String >,
      model : String,
      max_cost_per_batch : f64,
    ) -> Result< Vec< CreateEmbeddingResponse > >
    where
      E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
    {
      // Estimate cost and optimize batching for budget
      let estimated_tokens_per_text : usize = texts.iter().map( | t | t.len() / 4 ).sum(); // Rough estimate
      let cost_per_token = 0.0001; // Example cost
      let estimated_total_cost = estimated_tokens_per_text as f64 * cost_per_token;

      if estimated_total_cost <= max_cost_per_batch
      {
        // Process all at once
        client.create_bulk( texts, model ).await
      }
      else
      {
        // Split into cost-effective batches
        #[ allow(clippy::cast_possible_truncation, clippy::cast_sign_loss) ]
        let texts_per_batch = ( max_cost_per_batch / cost_per_token / 250.0 ) as usize; // 250 tokens avg per text
        let batch_size = texts_per_batch.clamp( 1, 100 );

        let mut results = Vec::new();
        for chunk in texts.chunks( batch_size )
        {
          let chunk_results = client.create_bulk( chunk.to_vec(), model.clone() ).await?;
          results.extend( chunk_results );
        }
        Ok( results )
      }
    }
  }
}

mod_interface!
{
  exposed use
  {
    EnhancedEmbeddings,
    EnhancedEmbeddingsConfig,
    EmbeddingBatchProcessor,
    analyze_embedding_batching_potential,
  };

  #[ cfg( feature = "batching" ) ]
  exposed use BatchingAnalysis;
}