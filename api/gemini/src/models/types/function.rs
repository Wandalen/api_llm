//! Function calling and tool configuration types for the Gemini API.

use serde::{ Deserialize, Serialize };
use super::code_execution::{ CodeExecution, CodeExecutionConfig };
use super::search::GoogleSearchTool;

/// Tool that the model can use.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct Tool
{
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Function declarations the model can call.
  pub function_declarations : Option< Vec< FunctionDeclaration > >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Code execution configuration.
  pub code_execution : Option< CodeExecution >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Google Search tool for web search integration.
  pub google_search_retrieval : Option< GoogleSearchTool >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Enhanced code execution tool.
  pub code_execution_tool : Option< CodeExecutionTool >,
}

/// Declaration of a function the model can call.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct FunctionDeclaration
{
  /// The name of the function.
  pub name : String,
  /// Description of what the function does.
  pub description : String,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Parameters schema in JSON Schema format.
  pub parameters : Option< serde_json::Value >,
}

/// Enhanced function calling configuration with mode control.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct FunctionCallingConfig
{
  /// Mode for function calling behavior.
  pub mode : FunctionCallingMode,

  /// List of function names that are allowed to be called.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub allowed_function_names : Option< Vec< String > >,
}

/// Function calling mode enumeration.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "SCREAMING_SNAKE_CASE" ) ]
pub enum FunctionCallingMode
{
  /// Let the model decide when to call functions.
  #[ serde( rename = "AUTO" ) ]
  Auto,

  /// Force the model to call a function.
  #[ serde( rename = "ANY" ) ]
  Any,

  /// Disable all function calling.
  #[ serde( rename = "NONE" ) ]
  None,
}

/// Enhanced tool configuration with advanced options.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct ToolConfig
{
  /// Configuration for function calling behavior.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub function_calling_config : Option< FunctionCallingConfig >,

  /// Configuration for code execution.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub code_execution : Option< CodeExecutionConfig >,
}

/// Code execution tool for Python code generation and execution.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct CodeExecutionTool
{
  /// Configuration options for code execution.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub config : Option< CodeExecutionConfig >,
}
