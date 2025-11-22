//! Cache management types for the Gemini API.

use serde::{ Deserialize, Serialize };
use super::content::Content;
use super::function::Tool;
use super::generation::UsageMetadata;

/// Request to create cached content.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct CreateCachedContentRequest
{
  /// The model to use for caching.
  pub model : String,

  /// Contents to cache.
  pub contents : Vec< Content >,

  /// Time-to-live for the cache in seconds format (e.g., "3600s").
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub ttl : Option< String >,

  /// Absolute expiration time in RFC3339 format.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub expire_time : Option< String >,

  /// Human-readable display name for the cache.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub display_name : Option< String >,

  /// System instruction for the cached content.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub system_instruction : Option< Content >,

  /// Tools available for the cached content.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub tools : Option< Vec< Tool > >,

  /// Tool configuration for the cached content.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub tool_config : Option< serde_json::Value >,
}

/// Response from creating cached content.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct CachedContentResponse
{
  /// Unique identifier for the cached content.
  pub name : String,

  /// The model used for caching.
  pub model : String,

  /// The cached contents.
  pub contents : Vec< Content >,

  /// Expiration time in RFC3339 format.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub expire_time : Option< String >,

  /// Creation time in RFC3339 format.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub create_time : Option< String >,

  /// Last update time in RFC3339 format.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub update_time : Option< String >,

  /// Human-readable display name for the cache.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub display_name : Option< String >,

  /// System instruction for the cached content.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub system_instruction : Option< Content >,

  /// Tools available for the cached content.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub tools : Option< Vec< Tool > >,

  /// Tool configuration for the cached content.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub tool_config : Option< serde_json::Value >,

  /// Usage metadata for the cached content.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub usage_metadata : Option< UsageMetadata >,
}

/// Response from listing cached contents.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct ListCachedContentsResponse
{
  /// List of cached contents.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub cached_contents : Option< Vec< CachedContentResponse > >,

  /// Token for retrieving the next page of results.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub next_page_token : Option< String >,
}

/// Request to update cached content.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct UpdateCachedContentRequest
{
  /// New time-to-live for the cache in seconds format (e.g., "3600s").
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub ttl : Option< String >,

  /// New absolute expiration time in RFC3339 format.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub expire_time : Option< String >,
}
