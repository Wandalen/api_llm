//! Text generation types for Ollama API.
//!
//! Provides request and response structures for the text generation endpoint.

#[ cfg( feature = "enabled" ) ]
mod private
{
  use serde::{ Serialize, Deserialize };
  use core::hash::{ Hash, Hasher };

  /// Text generation request
  #[ derive( Debug, Clone, Serialize ) ]
  pub struct GenerateRequest
  {
    /// Model name to use for generation
    pub model : String,
    /// Text prompt for generation
    pub prompt : String,
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    /// Whether to stream the response
    pub stream : Option< bool >,
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    /// Additional model parameters
    pub options : Option< serde_json::Value >,
  }

  #[ cfg( feature = "request_caching" ) ]
  impl Hash for GenerateRequest
  {
    #[ inline ]
    fn hash< H : Hasher >( &self, state : &mut H )
    {
      self.model.hash( state );
      self.prompt.hash( state );
      self.stream.hash( state );
      if let Some( ref options ) = self.options
      {
        options.to_string().hash( state );
      }
    }
  }

  /// Text generation response
  #[ derive( Debug, Serialize, Deserialize ) ]
  pub struct GenerateResponse
  {
    #[ serde( default ) ]
    /// Generated text response
    pub response : String,
    #[ serde( default ) ]
    /// Whether generation is complete
    pub done : bool,
    #[ serde( default ) ]
    /// Reason for completion (e.g., "stop")
    pub done_reason : Option< String >,
    #[ serde( default ) ]
    /// Model name used for generation
    pub model : Option< String >,
    #[ serde( default ) ]
    /// Timestamp of response creation
    pub created_at : Option< String >,
    #[ serde( default ) ]
    /// Context information
    pub context : Option< Vec< u32 > >,
    #[ serde( default ) ]
    /// Total time taken for generation in nanoseconds
    pub total_duration : Option< u64 >,
    #[ serde( default ) ]
    /// Time taken to load the model in nanoseconds
    pub load_duration : Option< u64 >,
    #[ serde( default ) ]
    /// Number of tokens in the prompt
    pub prompt_eval_count : Option< u32 >,
    #[ serde( default ) ]
    /// Time taken for prompt evaluation in nanoseconds
    pub prompt_eval_duration : Option< u64 >,
    #[ serde( default ) ]
    /// Number of tokens generated
    pub eval_count : Option< u32 >,
    #[ serde( default ) ]
    /// Time taken for evaluation in nanoseconds
    pub eval_duration : Option< u64 >,
  }
}

#[ cfg( feature = "enabled" ) ]
crate ::mod_interface!
{
  exposed use
  {
    GenerateRequest,
    GenerateResponse,
  };
}
