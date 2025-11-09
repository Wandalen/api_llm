//! Embeddings generation types for Ollama API.
//!
//! Provides request and response structures for generating text embeddings.

#[ cfg( feature = "embeddings" ) ]
mod private
{
  use serde::{ Serialize, Deserialize };

  /// Embeddings generation request
  #[ derive( Debug, Clone, Serialize ) ]
  pub struct EmbeddingsRequest
  {
    /// Model name to use for embeddings generation
    pub model : String,
    /// Input text to generate embeddings for
    pub prompt : String,
    /// Optional model parameters
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub options : Option< std::collections::HashMap<  String, serde_json::Value  > >,
  }

  /// Embeddings generation response
  #[ derive( Debug, Deserialize ) ]
  pub struct EmbeddingsResponse
  {
    /// Generated embedding vector
    pub embedding : Vec< f64 >,
  }
}

#[ cfg( feature = "embeddings" ) ]
crate ::mod_interface!
{
  exposed use
  {
    EmbeddingsRequest,
    EmbeddingsResponse,
  };
}
