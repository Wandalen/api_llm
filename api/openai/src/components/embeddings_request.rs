//! Request structures for embeddings API

/// Define a private namespace for all its items.
mod private
{
  use serde::{ Serialize, Deserialize };
  use former::Former;

  /// Input for embedding creation - can be a single string or array of strings
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  pub enum EmbeddingInput
  {
    /// Single text input
    Single( String ),
    /// Multiple text inputs for batch processing
    Multiple( Vec< String > ),
  }

  /// Request for creating embeddings
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Former ) ]
  pub struct CreateEmbeddingRequest
  {
    /// Input text to embed, encoded as a string or array of strings
    pub input : EmbeddingInput,

    /// ID of the model to use
    pub model : String,

    /// The number of dimensions the resulting output embeddings should have.
    /// Only supported in text-embedding-3 and later models.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub dimensions : Option< u32 >,

    /// The format to return the embeddings in. Can be either `float` or `base64`.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub encoding_format : Option< String >,

    /// A unique identifier representing your end-user, which can help `OpenAI` to monitor and detect abuse.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub user : Option< String >,
  }

  impl CreateEmbeddingRequest
  {
    /// Create a new embedding request with single text input
    #[ inline ]
    #[ must_use ]
    pub fn new_single( input : String, model : String ) -> Self
    {
      Self
      {
        input : EmbeddingInput::Single( input ),
        model,
        dimensions : None,
        encoding_format : None,
        user : None,
      }
    }

    /// Create a new embedding request with multiple text inputs
    #[ inline ]
    #[ must_use ]
    pub fn new_multiple( input : Vec< String >, model : String ) -> Self
    {
      Self
      {
        input : EmbeddingInput::Multiple( input ),
        model,
        dimensions : None,
        encoding_format : None,
        user : None,
      }
    }
  }

  impl Default for CreateEmbeddingRequest
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        input : EmbeddingInput::Single( String::new() ),
        model : "text-embedding-ada-002".to_string(),
        dimensions : None,
        encoding_format : None,
        user : None,
      }
    }
  }
}

crate ::mod_interface!
{
  exposed use
  {
    EmbeddingInput,
    CreateEmbeddingRequest,
  };
}