//! Google Search and grounding types for the Gemini API.

use serde::{ Deserialize, Serialize };

/// Google Search tool for real-time web search integration.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct GoogleSearchTool
{
  /// Configuration options for Google Search (currently empty for enablement).
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub config : Option< serde_json::Value >,
}

/// Grounding metadata containing web search results and attribution.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct GroundingMetadata
{
  /// Queries that were sent to the web search service.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub web_search_queries : Option< Vec< String > >,

  /// List of supporting grounding chunks.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub grounding_chunks : Option< Vec< GroundingChunk > >,

  /// List of grounding support segments.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub grounding_supports : Option< Vec< GroundingSupport > >,

  /// Search entry point for the grounding.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub search_entry_point : Option< SearchEntryPoint >,
}

/// Individual grounding chunk from web search results.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct GroundingChunk
{
  /// URI of the source web page.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub uri : Option< String >,

  /// Title of the web page.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub title : Option< String >,

  /// Content excerpt from the web page.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub content : Option< String >,

  /// Publication date of the content.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub published_date : Option< String >,

  /// Domain of the source website.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub domain : Option< String >,
}

/// Grounding support indicating which parts of the response are grounded.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct GroundingSupport
{
  /// Start index of the grounded segment in the response.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub start_index : Option< i32 >,

  /// End index of the grounded segment in the response.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub end_index : Option< i32 >,

  /// Indices of grounding chunks that support this segment.
  pub grounding_chunk_indices : Vec< i32 >,

  /// Confidence score for this grounding support.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub confidence_score : Option< f64 >,
}

/// Search entry point providing access to search functionality.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct SearchEntryPoint
{
  /// The rendered content that can be used for search entry.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub rendered_content : Option< String >,

  /// SDK blob containing the search functionality.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub sdk_blob : Option< String >,
}
