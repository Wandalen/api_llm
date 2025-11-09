//! Search Grounding Integration
//!
//! Types and configurations for search-grounded AI responses.

use serde::{ Serialize, Deserialize };

/// Configuration for search grounding operations.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct SearchGroundingConfig
{
  /// Search engine to use
  pub search_engine : SearchEngine,
  /// Maximum number of results to retrieve
  pub max_results : usize,
  /// Length of content snippets
  pub snippet_length : usize,
  /// Enable safe search filtering
  pub enable_safe_search : bool,
  /// Language preference for results
  pub language_preference : Option< String >,
}

impl Default for SearchGroundingConfig
{
  #[ inline ]
  fn default() -> Self
  {
    Self
    {
      search_engine : SearchEngine::Google,
      max_results : 10,
      snippet_length : 200,
      enable_safe_search : true,
      language_preference : Some( "en".to_string() ),
    }
  }
}

/// Available search engines for grounding.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub enum SearchEngine
{
  /// Google Search API
  Google,
  /// Bing Search API
  Bing,
  /// Custom search endpoint
  Custom
  {
    /// API endpoint URL
    endpoint : String,
    /// API key for authentication
    api_key : String,
  },
}

/// Response from search grounding operation.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct GroundedResponse
{
  /// The grounded response text
  pub response : String,
  /// Sources used for grounding
  pub sources : Vec< SearchSource >,
  /// Confidence score (0.0 to 1.0)
  pub confidence_score : f64,
  /// Metadata about the search operation
  pub search_metadata : SearchMetadata,
}

/// Individual search result source.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct SearchSource
{
  /// Source URL
  pub url : String,
  /// Page title
  pub title : String,
  /// Content snippet
  pub snippet : String,
  /// Relevance score
  pub relevance_score : f64,
}

/// Metadata about search operation.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct SearchMetadata
{
  /// Query that was executed
  pub query : String,
  /// Number of results found
  pub total_results : usize,
  /// Time taken for search (ms)
  pub search_time_ms : u64,
  /// Search engine used
  pub engine_used : String,
}
