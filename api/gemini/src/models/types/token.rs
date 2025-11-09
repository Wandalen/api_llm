//! Token counting and analysis types for the Gemini API.

use serde::{ Deserialize, Serialize };
use super::content::Content;
use super::generation::GenerateContentRequest;

/// Request for counting tokens in content.
#[ derive( Debug, Clone, Serialize, Deserialize, Default ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct CountTokensRequest
{
  /// Contents to count tokens for.
  pub contents : Vec< Content >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Optional generation request to include in token counting.
  pub generate_content_request : Option< GenerateContentRequest >,
}

/// Response from counting tokens.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct CountTokensResponse
{
  /// Total number of tokens in the input.
  pub total_tokens : i32,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Number of tokens in cached content.
  pub cached_content_token_count : Option< i32 >,
}

/// Request for batch token counting.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct BatchCountTokensRequest
{
  /// List of token counting requests.
  pub requests : Vec< CountTokensRequest >,
}

/// Response from batch token counting.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct BatchCountTokensResponse
{
  /// List of token count responses.
  pub responses : Vec< CountTokensResponse >,
}

/// Enhanced token analysis request.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct AnalyzeTokensRequest
{
  /// Contents to analyze for token usage.
  pub contents : Vec< Content >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Optional generation request to include in analysis.
  pub generate_content_request : Option< GenerateContentRequest >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Whether to include detailed token breakdown.
  pub include_breakdown : Option< bool >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Whether to estimate generation token usage.
  pub estimate_generation_tokens : Option< bool >,
}

/// Enhanced token analysis response.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct AnalyzeTokensResponse
{
  /// Total number of input tokens.
  pub total_tokens : i32,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Number of tokens in cached content.
  pub cached_content_token_count : Option< i32 >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Estimated number of output tokens.
  pub estimated_output_tokens : Option< i32 >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Detailed breakdown of token usage by content type.
  pub token_breakdown : Option< TokenBreakdown >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Cost estimation based on token usage.
  pub cost_estimate : Option< CostEstimate >,
}

/// Detailed breakdown of token usage.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct TokenBreakdown
{
  /// Tokens used by text content.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub text_tokens : Option< i32 >,

  /// Tokens used by image content.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub image_tokens : Option< i32 >,

  /// Tokens used by video content.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub video_tokens : Option< i32 >,

  /// Tokens used by audio content.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub audio_tokens : Option< i32 >,

  /// Tokens used by function calls and responses.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub function_tokens : Option< i32 >,

  /// Tokens used by system instructions.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub system_tokens : Option< i32 >,
}

/// Cost estimation for token usage.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct CostEstimate
{
  /// Estimated cost for input tokens.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub input_cost : Option< f64 >,

  /// Estimated cost for output tokens.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub output_cost : Option< f64 >,

  /// Total estimated cost.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub total_cost : Option< f64 >,

  /// Currency for cost estimates.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub currency : Option< String >,
}
