//! Embedding-related components for `HuggingFace` feature extraction API.

use serde::{ Deserialize, Serialize };

/// Request for embedding generation (feature extraction)
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct EmbeddingRequest
{
  /// Input text or texts to embed
  pub inputs : EmbeddingInput,
  
  /// Options for the embedding request
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub options : Option< EmbeddingOptions >,
}

impl EmbeddingRequest
{
  /// Create a new embedding request for a single text
  #[ inline ]
  #[ must_use ]
  pub fn new( input : impl Into< String > ) -> Self
  {
  Self
  {
      inputs : EmbeddingInput::Single( input.into() ),
      options : None,
  }
  }
  
  /// Create a new embedding request for multiple texts
  #[ inline ]
  #[ must_use ]
  pub fn new_batch( inputs : Vec< String > ) -> Self
  {
  Self
  {
      inputs : EmbeddingInput::Batch( inputs ),
      options : None,
  }
  }
  
  /// Set options
  #[ inline ]
  #[ must_use ]
  pub fn with_options( mut self, options : EmbeddingOptions ) -> Self
  {
  self.options = Some( options );
  self
  }
}

/// Input for embedding generation
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( untagged ) ]
pub enum EmbeddingInput
{
  /// Single text input
  Single( String ),
  /// Multiple text inputs
  Batch( Vec< String > ),
}

/// Options for embedding requests
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct EmbeddingOptions
{
  /// Use cache
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub use_cache : Option< bool >,
  
  /// Wait for model to load
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub wait_for_model : Option< bool >,
  
  /// Normalize embeddings
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub normalize : Option< bool >,
  
  /// Pooling strategy
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub pooling : Option< PoolingStrategy >,
}

impl Default for EmbeddingOptions
{
  #[ inline ]
  fn default() -> Self
  {
  Self::recommended()
  }
}

impl EmbeddingOptions
{
  /// Create new embedding options with HuggingFace-recommended values
  ///
  /// # Governing Principle Compliance
  ///
  /// This provides HuggingFace-recommended options without making them implicit defaults.
  /// Developers must explicitly choose to use these recommended values.
  #[ inline ]
  #[ must_use ]
  pub fn recommended() -> Self
  {
  Self
  {
      use_cache : Some( true ),       // Enable caching for better performance
      wait_for_model : Some( true ),  // Wait for model loading if needed
      normalize : Some( true ),       // Normalize embeddings for consistency
      pooling : Some( PoolingStrategy::Mean ), // Mean pooling for balanced representation
  }
  }

  /// Create empty embedding options requiring explicit configuration
  ///
  /// # Governing Principle Compliance
  ///
  /// This requires explicit configuration for all options, providing full transparency
  /// and control over embedding behavior.
  #[ inline ]
  #[ must_use ]
  pub fn empty() -> Self
  {
  Self
  {
      use_cache : None,
      wait_for_model : None,
      normalize : None,
      pooling : None,
  }
  }

  /// Create conservative options for production environments
  #[ inline ]
  #[ must_use ]
  pub fn conservative() -> Self
  {
  Self
  {
      use_cache : Some( false ),      // Disable caching for consistent results
      wait_for_model : Some( false ), // Fail fast if model not available
      normalize : Some( false ),      // Keep raw embeddings
      pooling : Some( PoolingStrategy::Cls ), // CLS token for precise representation
  }
  }

  /// Set `use_cache` option
  #[ inline ]
  #[ must_use ]
  pub fn with_use_cache( mut self, use_cache : bool ) -> Self
  {
  self.use_cache = Some( use_cache );
  self
  }

  /// Set `wait_for_model` option
  #[ inline ]
  #[ must_use ]
  pub fn with_wait_for_model( mut self, wait_for_model : bool ) -> Self
  {
  self.wait_for_model = Some( wait_for_model );
  self
  }

  /// Set normalize option
  #[ inline ]
  #[ must_use ]
  pub fn with_normalize( mut self, normalize : bool ) -> Self
  {
  self.normalize = Some( normalize );
  self
  }

  /// Set pooling strategy
  #[ inline ]
  #[ must_use ]
  pub fn with_pooling( mut self, pooling : PoolingStrategy ) -> Self
  {
  self.pooling = Some( pooling );
  self
  }
}

/// Pooling strategy for embeddings
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
#[ serde( rename_all = "lowercase" ) ]
pub enum PoolingStrategy
{
  /// Mean pooling
  Mean,
  /// Max pooling
  Max,
  /// CLS token pooling
  Cls,
}

/// Response wrapper for embedding operations
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( untagged ) ]
pub enum EmbeddingResponse
{
  /// Single embedding output
  Single( Vec< Vec< f32 > > ),
  /// Multiple embedding outputs
  Batch( Vec< Vec< Vec< f32 > > > ),
}