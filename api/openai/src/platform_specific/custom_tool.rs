//! Custom Tool Integration
//!
//! Types and traits for integrating custom tools and functions.

use std::collections::HashMap;
use serde::{ Serialize, Deserialize };
use crate::error::Result;

/// Trait for custom tools that can be integrated.
#[ async_trait::async_trait ]
pub trait CustomTool : Send + Sync
{
  /// Tool name
  fn name( &self ) -> &str;

  /// Tool description
  fn description( &self ) -> &str;

  /// Tool parameter definitions
  fn parameters( &self ) -> ToolParameters;

  /// Execute the tool with given parameters
  async fn execute( &self, parameters : serde_json::Value ) -> Result< ToolResult >;
}

/// Tool parameter definitions.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ToolParameters
{
  /// Required parameter names
  pub required : Vec< String >,
  /// Parameter definitions
  pub properties : HashMap<  String, ParameterDefinition  >,
}

/// Definition of a single parameter.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ParameterDefinition
{
  /// Parameter type
  pub param_type : String,
  /// Parameter description
  pub description : String,
  /// Whether parameter is required
  pub required : bool,
  /// Default value if any
  pub default : Option< serde_json::Value >,
}

/// Result of tool execution.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ToolResult
{
  /// Tool output
  pub output : serde_json::Value,
  /// Whether execution was successful
  pub success : bool,
  /// Error message if execution failed
  pub error_message : Option< String >,
}
