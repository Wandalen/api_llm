//! Core model types for the Gemini API.

use serde::{ Deserialize, Serialize };

/// Represents a Gemini model with its capabilities and limits.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct Model
{
  /// Unique identifier for the model.
  pub name : String,

  /// Human-readable name for the model.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub display_name : Option< String >,

  /// Description of the model's capabilities.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub description : Option< String >,

  /// Version of the model.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub version : Option< String >,

  /// Maximum number of input tokens supported.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub input_token_limit : Option< i32 >,

  /// Maximum number of output tokens supported.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub output_token_limit : Option< i32 >,

  /// List of supported generation methods.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub supported_generation_methods : Option< Vec< String > >,

  /// Default temperature setting for the model.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub temperature : Option< f32 >,

  /// Default top-p setting for the model.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub top_p : Option< f32 >,

  /// Default top-k setting for the model.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub top_k : Option< i32 >,
}

/// Response from listing available models.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct ListModelsResponse
{
  /// List of available models.
  pub models : Vec< Model >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Token for retrieving the next page of results.
  pub next_page_token : Option< String >,
}
