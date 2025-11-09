//! Embedding types for the Gemini API.

use serde::{ Deserialize, Serialize };
use super::content::Content;

/// Request for generating embeddings.
#[ derive( Debug, Clone, Serialize, Deserialize, Default ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct EmbedContentRequest
{
  /// Content to embed.
  pub content : Content,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Optional task type for embedding.
  pub task_type : Option< String >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Optional title for embedding.
  pub title : Option< String >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Desired embedding dimensions.
  pub output_dimensionality : Option< i32 >,
}

/// Response containing embeddings.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct EmbedContentResponse
{
  /// The generated embedding.
  pub embedding : ContentEmbedding,
}

/// Vector embedding of content.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct ContentEmbedding
{
  /// The embedding values.
  pub values : Vec< f32 >,
}

/// Request for batch embedding.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct BatchEmbedContentsRequest
{
  /// List of embedding requests.
  pub requests : Vec< EmbedContentRequest >,
}

/// Response for batch embedding.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct BatchEmbedContentsResponse
{
  /// List of embeddings.
  pub embeddings : Vec< ContentEmbedding >,
}
