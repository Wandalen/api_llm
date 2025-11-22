//! Structures related to the legacy Completions API.

/// Define a private namespace for all its items.
mod private
{
  // Use full paths from crate root for components
  use crate::components::common::CompletionUsage;
  // Serde imports
  use serde::{ Serialize, Deserialize };
  // Std imports
  use std::collections::HashMap;

  /// Log probability information for the choice.
  ///
  /// # Used By
  /// - `CompletionChoice`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct CompletionLogProbs
  {
    /// The character offset from the start of the prompt for each token.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub text_offset : Option< Vec< i32 > >,
    /// The log probability of each token chosen.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub token_logprobs : Option< Vec< f64 > >,
    /// The tokens chosen by the model.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tokens : Option< Vec< String > >,
    /// A map of the most likely tokens and their log probability, at each token position.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub top_logprobs : Option< Vec< HashMap<  String, f64  > > >,
  }

  /// Represents one possible completion choice.
  ///
  /// # Used By
  /// - `CreateCompletionResponse`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct CompletionChoice
  {
    /// The reason the model stopped generating tokens.
    pub finish_reason : String, // Enum : stop, length, content_filter
    /// The index of the choice in the list of choices.
    pub index : i32,
    /// Log probability information for the choice.
    pub logprobs : Option< CompletionLogProbs >,
    /// The generated completion text.
    pub text : String,
  }

  /// Represents a completion response from the legacy API.
  /// Note : both the streamed and non-streamed response objects share the same shape.
  ///
  /// # Used By
  /// - `/completions` (POST)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct CreateCompletionResponse
  {
    /// A unique identifier for the completion.
    pub id : String,
    /// The list of completion choices the model generated for the input prompt.
    pub choices : Vec< CompletionChoice >,
    /// The Unix timestamp (in seconds) of when the completion was created.
    pub created : i64,
    /// The model used for completion.
    pub model : String,
    /// This fingerprint represents the backend configuration that the model runs with.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub system_fingerprint : Option< String >,
    /// The object type, which is always "`text_completion`".
    pub object : String,
    /// Usage statistics for the completion request.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub usage : Option< CompletionUsage >,
  }
} // end mod private

crate ::mod_interface!
{
  exposed use
  {
    CompletionLogProbs,
    CompletionChoice,
    CreateCompletionResponse
  };
}
