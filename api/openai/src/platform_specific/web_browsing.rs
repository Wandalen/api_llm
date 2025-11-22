//! Web Browsing Capabilities
//!
//! Types and configurations for web browsing and content retrieval.

use serde::{ Serialize, Deserialize };
use core::time::Duration;

/// Configuration for web browsing operations.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct WebBrowsingConfig
{
  /// User agent string
  pub user_agent : String,
  /// Maximum page size in bytes
  pub max_page_size : usize,
  /// Whether to follow redirects
  pub follow_redirects : bool,
  /// Enable JavaScript execution
  pub javascript_enabled : bool,
  /// Enable screenshot capture
  pub screenshot_enabled : bool,
  /// Request timeout
  pub timeout : Duration,
}

impl Default for WebBrowsingConfig
{
  #[ inline ]
  fn default() -> Self
  {
    Self
    {
      user_agent : "Mozilla/5.0 (compatible; OpenAI-Client/1.0)".to_string(),
      max_page_size : 10 * 1024 * 1024, // 10MB
      follow_redirects : true,
      javascript_enabled : false, // Disabled by default for security
      screenshot_enabled : false,
      timeout : Duration::from_secs( 30 ),
    }
  }
}

/// Result of web browsing operation.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct BrowsingResult
{
  /// Final URL after redirects
  pub url : String,
  /// Page title
  pub title : String,
  /// Page content (text)
  pub content : String,
  /// Links found on page
  pub links : Vec< String >,
  /// Images found on page
  pub images : Vec< String >,
  /// Screenshot data (if enabled)
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub screenshot : Option< Vec< u8 > >,
  /// Browsing metadata
  pub metadata : BrowsingMetadata,
}

/// Metadata about browsing operation.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct BrowsingMetadata
{
  /// HTTP status code
  pub status_code : u16,
  /// Content type
  pub content_type : String,
  /// Content length in bytes
  pub content_length : usize,
  /// Load time in milliseconds
  pub load_time_ms : u64,
  /// Number of redirects followed
  pub redirect_count : usize,
}
