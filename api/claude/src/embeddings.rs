//! Text embedding functionality for Anthropic API client
//!
//! Since Anthropic doesn't currently offer embeddings API, this module provides:
//! 1. Future-ready architecture for when Anthropic adds embeddings
//! 2. Framework for third-party embedding integration
//! 3. Proper error handling indicating feature not yet supported

#[ cfg( feature = "embeddings" ) ]
mod private
{
  use serde::{ Serialize, Deserialize };

  #[ cfg( feature = "error-handling" ) ]
  use crate::error::{ AnthropicError, AnthropicResult };

  #[ cfg( not( feature = "error-handling" ) ) ]
  type AnthropicError = crate::error_tools::Error;
  #[ cfg( not( feature = "error-handling" ) ) ]
  type AnthropicResult< T > = Result< T, crate::error_tools::Error >;

  /// Embedding request structure for future Anthropic embedding models
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct EmbeddingRequest
  {
    /// Model to use for embedding generation
    model : String,
    /// Input text or array of texts to embed
    input : EmbeddingInput,
    /// Format for returned embeddings
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    encoding_format : Option< String >,
    /// Maximum number of dimensions for the embedding
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    dimensions : Option< u32 >,
    /// User identifier for tracking
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    user : Option< String >,
  }

  /// Input for embedding requests - can be single text or batch
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  #[ serde( untagged ) ]
  pub enum EmbeddingInput
  {
    /// Single text input
    Single( String ),
    /// Multiple texts for batch processing
    Batch( Vec< String > ),
  }

  /// Response from embedding API
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct EmbeddingResponse
  {
    /// Type of object returned
    object : String,
    /// Array of embedding data
    data : Vec< EmbeddingData >,
    /// Model used for embedding
    model : String,
    /// Usage statistics
    usage : EmbeddingUsage,
  }

  /// Individual embedding data point
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct EmbeddingData
  {
    /// Type of object
    object : String,
    /// Index in the input array
    index : usize,
    /// The embedding vector
    embedding : Vec< f64 >,
  }

  /// Usage statistics for embedding requests
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct EmbeddingUsage
  {
    /// Number of tokens in the prompt
    prompt_tokens : u32,
    /// Total tokens used
    total_tokens : u32,
  }

  impl EmbeddingRequest
  {
    /// Create new embedding request builder
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        model : String::new(),
        input : EmbeddingInput::Single( String::new() ),
        encoding_format : None,
        dimensions : None,
        user : None,
      }
    }

    /// Set the model for embedding generation
    #[ inline ]
    #[ must_use ]
    pub fn model< S : Into< String > >( mut self, model : S ) -> Self
    {
      self.model = model.into();
      self
    }

    /// Set single text input
    #[ inline ]
    #[ must_use ]
    pub fn input< S : Into< String > >( mut self, input : S ) -> Self
    {
      self.input = EmbeddingInput::Single( input.into() );
      self
    }

    /// Set batch text inputs
    #[ inline ]
    #[ must_use ]
    pub fn input_batch( mut self, inputs : Vec< String > ) -> Self
    {
      self.input = EmbeddingInput::Batch( inputs );
      self
    }

    /// Set encoding format (e.g., "float", "base64")
    #[ inline ]
    #[ must_use ]
    pub fn encoding_format< S : Into< String > >( mut self, format : S ) -> Self
    {
      self.encoding_format = Some( format.into() );
      self
    }

    /// Set maximum dimensions for embedding
    #[ inline ]
    #[ must_use ]
    pub fn dimensions( mut self, dims : u32 ) -> Self
    {
      self.dimensions = Some( dims );
      self
    }

    /// Set user identifier
    #[ inline ]
    #[ must_use ]
    pub fn user< S : Into< String > >( mut self, user : S ) -> Self
    {
      self.user = Some( user.into() );
      self
    }

    /// Get the model
    #[ inline ]
    #[ must_use ]
    pub fn get_model( &self ) -> &str
    {
      &self.model
    }

    /// Get the input as string (for single input)
    #[ inline ]
    #[ must_use ]
    pub fn get_input( &self ) -> &str
    {
      match &self.input
      {
        EmbeddingInput::Single( text ) => text,
        EmbeddingInput::Batch( texts ) => texts.first().map_or( "", String::as_str ),
      }
    }

    /// Get input batch (for batch input)
    #[ inline ]
    #[ must_use ]
    pub fn get_input_batch( &self ) -> Vec< &str >
    {
      match &self.input
      {
        EmbeddingInput::Single( text ) => vec![ text ],
        EmbeddingInput::Batch( texts ) => texts.iter().map( String::as_str ).collect(),
      }
    }

    /// Get encoding format
    #[ inline ]
    #[ must_use ]
    pub fn get_encoding_format( &self ) -> &str
    {
      self.encoding_format.as_deref().unwrap_or( "float" )
    }

    /// Get dimensions
    #[ inline ]
    #[ must_use ]
    pub fn get_dimensions( &self ) -> Option< u32 >
    {
      self.dimensions
    }

    /// Validate the embedding request
    ///
    /// # Errors
    ///
    /// Returns an error if the model is empty, input is empty, or constraints are violated
    pub fn validate( &self ) -> AnthropicResult< () >
    {
      if self.model.trim().is_empty()
      {
        #[ cfg( feature = "error-handling" ) ]
        return Err( AnthropicError::InvalidArgument( "Model cannot be empty".to_string() ) );
        #[ cfg( not( feature = "error-handling" ) ) ]
        return Err( crate::error_tools::Error::msg( "Model cannot be empty" ) );
      }

      // Validate input
      match &self.input
      {
        EmbeddingInput::Single( text ) =>
        {
          if text.trim().is_empty()
          {
            #[ cfg( feature = "error-handling" ) ]
            return Err( AnthropicError::InvalidArgument( "Input text cannot be empty".to_string() ) );
            #[ cfg( not( feature = "error-handling" ) ) ]
            return Err( crate::error_tools::Error::msg( "Input text cannot be empty" ) );
          }

          // Check maximum input length (reasonable constraint)
          if text.len() > 50000
          {
            #[ cfg( feature = "error-handling" ) ]
            return Err( AnthropicError::InvalidArgument( "Input text too long (max 50,000 characters)".to_string() ) );
            #[ cfg( not( feature = "error-handling" ) ) ]
            return Err( crate::error_tools::Error::msg( "Input text too long (max 50,000 characters)" ) );
          }
        }
        EmbeddingInput::Batch( texts ) =>
        {
          if texts.is_empty()
          {
            #[ cfg( feature = "error-handling" ) ]
            return Err( AnthropicError::InvalidArgument( "Batch input cannot be empty".to_string() ) );
            #[ cfg( not( feature = "error-handling" ) ) ]
            return Err( crate::error_tools::Error::msg( "Batch input cannot be empty" ) );
          }

          // Check batch size limit
          if texts.len() > 100
          {
            #[ cfg( feature = "error-handling" ) ]
            return Err( AnthropicError::InvalidArgument( "Batch size too large (max 100 texts)".to_string() ) );
            #[ cfg( not( feature = "error-handling" ) ) ]
            return Err( crate::error_tools::Error::msg( "Batch size too large (max 100 texts)" ) );
          }

          // Validate each text in batch
          for ( index, text ) in texts.iter().enumerate()
          {
            if text.trim().is_empty()
            {
              #[ cfg( feature = "error-handling" ) ]
              return Err( AnthropicError::InvalidArgument( format!( "Text at index {index} is empty" ) ) );
              #[ cfg( not( feature = "error-handling" ) ) ]
              return Err( crate::error_tools::Error::msg( format!( "Text at index {index} is empty" ) ) );
            }

            if text.len() > 50000
            {
              #[ cfg( feature = "error-handling" ) ]
              return Err( AnthropicError::InvalidArgument( format!( "Text at index {index} too long (max 50,000 characters)" ) ) );
              #[ cfg( not( feature = "error-handling" ) ) ]
              return Err( crate::error_tools::Error::msg( format!( "Text at index {index} too long (max 50,000 characters)" ) ) );
            }
          }
        }
      }

      // Validate dimensions if specified
      if let Some( dims ) = self.dimensions
      {
        if dims == 0 || dims > 4096
        {
          #[ cfg( feature = "error-handling" ) ]
          return Err( AnthropicError::InvalidArgument( "Dimensions must be between 1 and 4096".to_string() ) );
          #[ cfg( not( feature = "error-handling" ) ) ]
          return Err( crate::error_tools::Error::msg( "Dimensions must be between 1 and 4096" ) );
        }
      }

      Ok( () )
    }
  }

  impl Default for EmbeddingRequest
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  impl EmbeddingResponse
  {
    /// Get the data array
    #[ inline ]
    #[ must_use ]
    pub fn data( &self ) -> &Vec< EmbeddingData >
    {
      &self.data
    }

    /// Get the model used
    #[ inline ]
    #[ must_use ]
    pub fn model( &self ) -> &str
    {
      &self.model
    }

    /// Get usage statistics
    #[ inline ]
    #[ must_use ]
    pub fn usage( &self ) -> &EmbeddingUsage
    {
      &self.usage
    }

    /// Get total number of embeddings
    #[ inline ]
    #[ must_use ]
    pub fn embedding_count( &self ) -> usize
    {
      self.data.len()
    }
  }

  impl EmbeddingData
  {
    /// Get the embedding vector
    #[ inline ]
    #[ must_use ]
    pub fn embedding( &self ) -> &Vec< f64 >
    {
      &self.embedding
    }

    /// Get the index
    #[ inline ]
    #[ must_use ]
    pub fn index( &self ) -> usize
    {
      self.index
    }

    /// Get embedding dimensions
    #[ inline ]
    #[ must_use ]
    pub fn dimensions( &self ) -> usize
    {
      self.embedding.len()
    }
  }

  impl EmbeddingUsage
  {
    /// Get prompt tokens
    #[ inline ]
    #[ must_use ]
    pub fn prompt_tokens( &self ) -> u32
    {
      self.prompt_tokens
    }

    /// Get total tokens
    #[ inline ]
    #[ must_use ]
    pub fn total_tokens( &self ) -> u32
    {
      self.total_tokens
    }
  }
}

#[ cfg( feature = "embeddings" ) ]
crate::mod_interface!
{
  exposed use
  {
    EmbeddingRequest,
    EmbeddingResponse,
    EmbeddingData,
    EmbeddingUsage,
    EmbeddingInput,
  };
}