//! Builder types for embedding requests.
//!
//! Provides fluent builder APIs for constructing embedding requests.

use core::time::Duration;
use crate::error::Error;

use super::ModelApi;

/// Builder for fluent embedding request configuration.
///
/// This builder allows step-by-step construction of complex embedding
/// requests with method chaining for better ergonomics.
#[ derive( Debug ) ]
pub struct EmbeddingRequestBuilder< 'a >
{
  model : &'a ModelApi< 'a >,
  request : crate::models::EmbedContentRequest,
}

impl< 'a > EmbeddingRequestBuilder< 'a >
{
  /// Creates a new embedding request builder.
  #[ inline ]
  #[ must_use ]
  pub fn new( model : &'a ModelApi< 'a > ) -> Self
  {
    Self {
      model,
      request : crate::models::EmbedContentRequest {
        content : crate::models::Content {
          parts : vec![],
          role : "user".to_string(),
        },
        task_type : None,
        title : None,
        output_dimensionality : None,
      },
    }
  }

  /// Sets the text content for embedding.
  ///
  /// This method configures the input text that will be embedded.
  ///
  /// # Arguments
  ///
  /// * `text` - The text content to embed
  #[ inline ]
  #[ must_use ]
  pub fn with_text( mut self, text : &str ) -> Self
  {
    self.request.content.parts = vec![ crate::models::Part {
      text : Some( text.to_string() ),
      ..Default::default()
    } ];
    self
  }

  /// Sets the task type for embedding optimization.
  ///
  /// Task types help the model optimize embeddings for specific use cases:
  /// - "`RETRIEVAL_QUERY`": For search queries
  /// - "`RETRIEVAL_DOCUMENT`": For documents to be retrieved
  /// - "`SEMANTIC_SIMILARITY`": For similarity comparisons
  /// - "CLASSIFICATION": For text classification tasks
  ///
  /// # Arguments
  ///
  /// * `task_type` - The task type string
  #[ inline ]
  #[ must_use ]
  pub fn with_task_type( mut self, task_type : &str ) -> Self
  {
    self.request.task_type = Some( task_type.to_string() );
    self
  }

  /// Sets an optional title for the content.
  ///
  /// The title can provide additional context to improve embedding quality.
  ///
  /// # Arguments
  ///
  /// * `title` - The title string
  #[ inline ]
  #[ must_use ]
  pub fn with_title( mut self, title : &str ) -> Self
  {
    self.request.title = Some( title.to_string() );
    self
  }

  /// Sets the desired output dimensionality.
  ///
  /// This allows reducing the embedding dimensions for efficiency,
  /// though it may impact quality.
  ///
  /// # Arguments
  ///
  /// * `dimensions` - The desired number of dimensions
  #[ inline ]
  #[ must_use ]
  pub fn with_output_dimensionality( mut self, dimensions : i32 ) -> Self
  {
    self.request.output_dimensionality = Some( dimensions );
    self
  }

  /// Executes the configured embedding request.
  ///
  /// # Returns
  ///
  /// Returns the full [`crate::models::EmbedContentResponse`] from the model.
  ///
  /// # Errors
  ///
  /// Returns the same errors as [`ModelApi::embed_content`].
  #[ inline ]
  pub async fn execute( self ) -> Result< crate::models::EmbedContentResponse, Error >
  {
    self.model.embed_content( &self.request ).await
  }

  /// Executes the request and returns only the embedding vector.
  ///
  /// This is a convenience method that extracts the vector from the response.
  ///
  /// # Returns
  ///
  /// Returns the embedding vector.
  ///
  /// # Errors
  ///
  /// Returns embedding errors plus vector extraction errors.
  #[ inline ]
  pub async fn execute_vector( self ) -> Result< Vec< f32 >, Error >
  {
    let model_id = self.model.model_id.clone();
    let response = self.execute().await?;
    
    let values = response.embedding.values;
    if values.is_empty()
    {
      Err( Error::ApiError(
        format!( "No embedding values returned from model '{model_id}'." )
      ) )
    } else {
      Ok( values )
    }
  }
}
/// Builder for configuring batch embedding requests.
///
/// This builder provides fine-grained control over batch processing parameters
/// and allows optimization of batch operations for specific use cases.
#[ derive( Debug ) ]
pub struct BatchEmbeddingRequestBuilder< 'a >
{
  /// The `ModelApi` instance to use for batch operations
  model : &'a ModelApi< 'a >,
  /// Optional list of texts to embed
  texts : Option< Vec< &'a str > >,
  /// Optional batch size for processing chunks
  batch_size : Option< usize >,
  /// Optional timeout for batch operations  
  timeout : Option< Duration >,
}

impl< 'a > BatchEmbeddingRequestBuilder< 'a >
{
  /// Creates a new batch embedding request builder.
  ///
  /// # Arguments
  ///
  /// * `model` - The `ModelApi` instance to use for batch operations
  #[ inline ]
  #[ must_use ]
  pub fn new( model : &'a ModelApi< 'a > ) -> Self
  {
    Self {
      model,
      texts : None,
      batch_size : None,
      timeout : None,
    }
  }

  /// Sets the texts to embed in batch.
  ///
  /// # Arguments
  ///
  /// * `texts` - A slice of text strings to embed
  #[ inline ]
  #[ must_use ]
  pub fn with_texts( mut self, texts : &'a [ &'a str ] ) -> Self
  {
    self.texts = Some( texts.to_vec() );
    self
  }

  /// Sets the batch size for processing.
  ///
  /// This controls how many texts are processed in each API request.
  /// Larger batch sizes reduce API calls but may hit size limits.
  ///
  /// # Arguments
  ///
  /// * `size` - The maximum number of texts per batch
  #[ inline ]
  #[ must_use ]
  pub fn with_batch_size( mut self, size : usize ) -> Self
  {
    self.batch_size = Some( size );
    self
  }

  /// Sets the timeout for batch operations.
  ///
  /// # Arguments
  ///
  /// * `timeout` - The maximum duration to wait for batch completion
  #[ inline ]
  #[ must_use ]
  pub fn with_timeout( mut self, timeout : Duration ) -> Self
  {
    self.timeout = Some( timeout );
    self
  }

  /// Executes the batch embedding request.
  ///
  /// # Returns
  ///
  /// Returns a vector of embedding vectors for the configured texts.
  ///
  /// # Errors
  ///
  /// Returns errors if no texts are configured or if batch processing fails.
  #[ inline ]
  pub async fn execute( self ) -> Result< Vec< Vec< f32 > >, Error >
  {
    let texts = self.texts.ok_or_else( || Error::ValidationError {
      message : "No texts specified for batch embedding".to_string()
    } )?;

    // For now, delegate to the main batch method
    // In the future, this would use the configured parameters
    self.model.batch_embed_texts( &texts ).await
  }
}
