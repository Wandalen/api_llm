//! Model information types for Ollama API.
//!
//! Provides structures for model metadata, details, and listings.

#[ cfg( feature = "enabled" ) ]
mod private
{
  use serde::Deserialize;

  /// Model information
  #[ derive( Debug, Deserialize ) ]
  pub struct ModelInfo
  {
    /// Modelfile content
    #[ serde( default ) ]
    pub modelfile : String,
    /// Model parameters
    #[ serde( default ) ]
    pub parameters : String,
    /// Prompt template
    #[ serde( default ) ]
    pub template : String,
    /// System message
    #[ serde( default ) ]
    pub system : String,
    /// Model details
    pub details : Option< ModelDetails >,
    /// Model information metadata
    pub model_info : Option< serde_json::Value >,
    /// Model tensors
    #[ serde( default ) ]
    pub tensors : Vec< serde_json::Value >,
    /// Model capabilities
    #[ serde( default ) ]
    pub capabilities : Vec< String >,
    /// Timestamp when the model was last modified
    pub modified_at : String,
  }

  /// Model details
  #[ derive( Debug, Deserialize ) ]
  pub struct ModelDetails
  {
    /// Parent model
    #[ serde( default ) ]
    pub parent_model : String,
    /// Model format
    #[ serde( default ) ]
    pub format : String,
    /// Model family
    #[ serde( default ) ]
    pub family : String,
    /// Model families
    #[ serde( default ) ]
    pub families : Vec< String >,
    /// Parameter size
    #[ serde( default ) ]
    pub parameter_size : String,
    /// Quantization level
    #[ serde( default ) ]
    pub quantization_level : String,
  }

  /// Model entry from tags/list endpoint
  #[ derive( Debug, Deserialize ) ]
  pub struct ModelEntry
  {
    /// Name of the model
    pub name : String,
    /// Model identifier
    pub model : String,
    /// Timestamp when the model was last modified
    pub modified_at : String,
    /// Size of the model in bytes
    pub size : u64,
    /// SHA256 digest of the model
    pub digest : String,
    /// Model details
    pub details : Option< ModelDetails >,
  }

  /// Response from tags endpoint listing available models
  #[ derive( Debug, Deserialize ) ]
  pub struct TagsResponse
  {
    /// List of available models
    pub models : Vec< ModelEntry >,
  }
}

#[ cfg( feature = "enabled" ) ]
crate ::mod_interface!
{
  exposed use
  {
    ModelInfo,
    ModelDetails,
    ModelEntry,
    TagsResponse,
  };
}
