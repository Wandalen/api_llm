//! Code execution types for the Gemini API.

use serde::{ Deserialize, Serialize };

/// Configuration for code execution.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct CodeExecution
{
  // Empty for now, as per API spec
}

/// Configuration for code execution behavior.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct CodeExecutionConfig
{
  /// Timeout for code execution in seconds.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub timeout : Option< i32 >,

  /// Whether to enable network access during execution.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub enable_network : Option< bool >,
}

/// Result from code execution.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct CodeExecutionResult
{
  /// Outcome of the code execution.
  pub outcome : String,

  /// Output produced by the code execution.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub output : Option< String >,

  /// Error message if execution failed.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub error : Option< String >,

  /// Execution time in milliseconds.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub execution_time_ms : Option< i64 >,
}
