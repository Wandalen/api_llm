//! Content generation types for the Gemini API.

use serde::{ Deserialize, Serialize };
use super::content::{ Content, SystemInstruction };
use super::function::{ Tool, ToolConfig };

/// Request for generating content using a model.
#[ derive( Debug, Clone, Serialize, Deserialize, Default ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct GenerateContentRequest
{
  /// The content of the conversation with the model.
  pub contents : Vec< Content >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Configuration options for model generation.
  pub generation_config : Option< GenerationConfig >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Safety settings for blocking unsafe content.
  pub safety_settings : Option< Vec< SafetySetting > >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Tools the model can use for this request.
  pub tools : Option< Vec< Tool > >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Configuration for tool usage.
  pub tool_config : Option< ToolConfig >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// System instruction for the model.
  pub system_instruction : Option< SystemInstruction >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Cached content that can be referenced.
  pub cached_content : Option< String >,
}

/// Response from content generation.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct GenerateContentResponse
{
  /// Generated content candidates.
  pub candidates : Vec< super::content::Candidate >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Feedback about the prompt.
  pub prompt_feedback : Option< PromptFeedback >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Token usage information.
  pub usage_metadata : Option< UsageMetadata >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Grounding metadata for web search results.
  pub grounding_metadata : Option< super::search::GroundingMetadata >,
}

/// Configuration for how the model generates responses.
#[ derive( Debug, Clone, Serialize, Deserialize, Default ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct GenerationConfig
{
  /// Controls randomness in generation (0.0 to 1.0).
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub temperature : Option< f32 >,

  /// Nucleus sampling parameter.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub top_p : Option< f32 >,

  /// Top-k sampling parameter.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub top_k : Option< i32 >,

  /// Number of response candidates to generate.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub candidate_count : Option< i32 >,

  /// Maximum number of tokens to generate.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub max_output_tokens : Option< i32 >,

  /// Sequences that will stop generation.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub stop_sequences : Option< Vec< String > >,
}

/// Safety setting for blocking content.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct SafetySetting
{
  /// The safety category.
  pub category : String,
  /// The threshold for blocking.
  pub threshold : String,
}

/// Feedback about the prompt.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct PromptFeedback
{
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Reason why the prompt was blocked.
  pub block_reason : Option< String >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Safety ratings for the prompt.
  pub safety_ratings : Option< Vec< super::content::SafetyRating > >,
}

/// Token usage statistics.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct UsageMetadata
{
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Number of tokens in the prompt.
  pub prompt_token_count : Option< i32 >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Number of tokens in the candidates.
  pub candidates_token_count : Option< i32 >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Total number of tokens.
  pub total_token_count : Option< i32 >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Number of cached content tokens.
  pub cached_content_token_count : Option< i32 >,
}

/// Request for batch content generation.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct BatchGenerateContentRequest
{
  /// List of content generation requests.
  pub requests : Vec< GenerateContentRequest >,
}

/// Response for batch content generation.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct BatchGenerateContentResponse
{
  /// List of generated content responses.
  pub responses : Vec< GenerateContentResponse >,
}
