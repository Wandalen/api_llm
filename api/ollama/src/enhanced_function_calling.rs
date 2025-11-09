//! Enhanced function calling with type-safe execution and validation.
//!
//! Provides a framework for defining, validating, and executing tools/functions
//! with automatic JSON schema generation and type-safe parameter handling.
//!
//! # Architecture
//!
//! The enhanced function calling system consists of:
//! - `ToolExecutor` trait: Defines tool execution interface
//! - `ToolRegistry`: Manages tool registration and dispatch
//! - `ToolResult`: Type-safe result type for tool execution
//! - Helper functions for creating `ToolDefinition` with type safety
//!
//! # Example
//!
//! ```rust,ignore
//! use api_ollama::enhanced_function_calling::{ ToolExecutor, ToolRegistry, ToolResult };
//! use api_ollama::ToolDefinition;
//!
//! // Define a tool executor
//! struct WeatherTool;
//!
//! impl ToolExecutor for WeatherTool
//! {
//!   fn name( &self ) -> &str { "get_weather" }
//!
//!   fn description( &self ) -> &str { "Get weather for a location" }
//!
//!   fn execute( &self, params : serde_json::Value ) -> ToolResult
//!   {
//!     // Extract and validate parameters
//!     let location = params[ "location" ].as_str()
//!       .ok_or( "Missing location parameter" )?;
//!
//!     // Execute tool logic
//!     let result = format!( "Weather in {}: Sunny, 72°F", location );
//!     Ok( result )
//!   }
//! }
//!
//! // Register and use
//! let mut registry = ToolRegistry::new();
//! registry.register( Box::new( WeatherTool ) );
//!
//! let definitions = registry.definitions();
//! // Use definitions in ChatRequest...
//! ```
//!
//! # Future: Procedural Macros
//!
//! The full implementation will include a `#[tool]` proc-macro for automatic
//! ToolDefinition generation from function signatures. This requires a separate
//! `ollama_macros` crate and will be implemented in a future phase.

#[ cfg( feature = "enhanced_function_calling" ) ]
mod private
{
  use std::collections::HashMap;
  use std::fmt;

  /// Result type for tool execution
  pub type ToolResult = Result< String, String >;

  /// Trait for executable tools with type-safe parameter handling
  pub trait ToolExecutor : Send + Sync
  {
    /// Get the tool name
    fn name( &self ) -> &str;

    /// Get the tool description
    fn description( &self ) -> &str;

    /// Get the JSON schema for tool parameters
    ///
    /// Returns a JSON schema object describing the expected parameters.
    /// Default implementation returns an empty object schema.
    fn parameter_schema( &self ) -> serde_json::Value
    {
      serde_json::json!
      ({
        "type" : "object",
        "properties" : {},
        "required" : []
      })
    }

    /// Execute the tool with given parameters
    ///
    /// # Arguments
    ///
    /// * `params` - JSON object containing tool parameters
    ///
    /// # Returns
    ///
    /// Returns the tool execution result as a string, or an error message
    ///
    /// # Errors
    ///
    /// Returns an error if parameters are invalid or execution fails
    fn execute( &self, params : serde_json::Value ) -> ToolResult;

    /// Get the full tool definition for use in API requests
    ///
    /// Automatically generates a `ToolDefinition` from the tool metadata.
    fn definition( &self ) -> crate::ToolDefinition
    {
      crate::ToolDefinition
      {
        name : self.name().to_string(),
        description : self.description().to_string(),
        parameters : self.parameter_schema(),
      }
    }
  }

  /// Registry for managing and executing tools
  ///
  /// The registry allows registering multiple tools, retrieving their definitions
  /// for API requests, and executing them by name.
  pub struct ToolRegistry
  {
    tools : HashMap< String, Box< dyn ToolExecutor > >,
  }

  impl ToolRegistry
  {
    /// Create a new empty tool registry
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        tools : HashMap::new(),
      }
    }

    /// Register a tool in the registry
    ///
    /// # Arguments
    ///
    /// * `tool` - The tool executor to register
    ///
    /// # Panics
    ///
    /// Panics if a tool with the same name is already registered
    #[ inline ]
    pub fn register( &mut self, tool : Box< dyn ToolExecutor > )
    {
      let name = tool.name().to_string();
      if self.tools.contains_key( &name )
      {
        panic!( "Tool '{}' is already registered", name );
      }
      self.tools.insert( name, tool );
    }

    /// Get all tool definitions for use in API requests
    ///
    /// Returns a vector of `ToolDefinition` objects that can be passed
    /// to `ChatRequest::tools`.
    #[ inline ]
    #[ must_use ]
    pub fn definitions( &self ) -> Vec< crate::ToolDefinition >
    {
      self.tools.values()
        .map( | tool | tool.definition() )
        .collect()
    }

    /// Execute a tool by name with given parameters
    ///
    /// # Arguments
    ///
    /// * `tool_call` - The tool call containing name and parameters
    ///
    /// # Returns
    ///
    /// Returns the tool execution result or an error
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Tool is not found in registry
    /// - Tool execution fails
    #[ inline ]
    pub fn execute( &self, tool_call : &crate::ToolCall ) -> ToolResult
    {
      // Extract function name from the tool call
      let function_name = tool_call.function
        .get( "name" )
        .and_then( | v | v.as_str() )
        .ok_or_else( || "Missing function name in tool call".to_string() )?;

      // Get the tool executor
      let tool = self.tools.get( function_name )
        .ok_or_else( || format!( "Tool '{}' not found in registry", function_name ) )?;

      // Extract parameters
      let params = tool_call.function
        .get( "arguments" )
        .cloned()
        .unwrap_or( serde_json::json!( {} ) );

      // Execute the tool
      tool.execute( params )
    }

    /// Get the number of registered tools
    #[ inline ]
    #[ must_use ]
    pub fn len( &self ) -> usize
    {
      self.tools.len()
    }

    /// Check if registry is empty
    #[ inline ]
    #[ must_use ]
    pub fn is_empty( &self ) -> bool
    {
      self.tools.is_empty()
    }

    /// Check if a tool is registered
    #[ inline ]
    #[ must_use ]
    pub fn contains( &self, name : &str ) -> bool
    {
      self.tools.contains_key( name )
    }
  }

  impl Default for ToolRegistry
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  impl fmt::Debug for ToolRegistry
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      f.debug_struct( "ToolRegistry" )
        .field( "tool_count", &self.tools.len() )
        .field( "tool_names", &self.tools.keys().collect::< Vec< _ > >() )
        .finish()
    }
  }

  /// Helper functions for creating tool definitions with type safety
  pub mod helpers
  {
    use serde_json::json;

    /// Create a simple tool definition with basic parameters
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let tool = create_simple_tool(
    ///   "get_weather",
    ///   "Get weather for a location",
    ///   &[ ( "location", "string", "The city name" ) ],
    ///   &[ "location" ]
    /// );
    /// ```
    #[ inline ]
    #[ must_use ]
    pub fn create_simple_tool(
      name : &str,
      description : &str,
      parameters : &[ ( &str, &str, &str ) ], // (name, type, description)
      required : &[ &str ],
    ) -> crate::ToolDefinition
    {
      let mut properties = serde_json::Map::new();

      for ( param_name, param_type, param_desc ) in parameters
      {
        properties.insert(
          param_name.to_string(),
          json!
          ({
            "type" : param_type,
            "description" : param_desc,
          })
        );
      }

      crate::ToolDefinition
      {
        name : name.to_string(),
        description : description.to_string(),
        parameters : json!
        ({
          "type" : "object",
          "properties" : properties,
          "required" : required,
        }),
      }
    }

    /// Create a tool definition with enum parameters
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let tool = create_enum_tool(
    ///   "set_mode",
    ///   "Set operation mode",
    ///   &[ ( "mode", &[ "fast", "slow", "medium" ], "The operation mode" ) ],
    ///   &[ "mode" ]
    /// );
    /// ```
    #[ inline ]
    #[ must_use ]
    pub fn create_enum_tool(
      name : &str,
      description : &str,
      parameters : &[ ( &str, &[ &str ], &str ) ], // (name, enum_values, description)
      required : &[ &str ],
    ) -> crate::ToolDefinition
    {
      let mut properties = serde_json::Map::new();

      for ( param_name, enum_values, param_desc ) in parameters
      {
        properties.insert(
          param_name.to_string(),
          json!
          ({
            "type" : "string",
            "enum" : enum_values,
            "description" : param_desc,
          })
        );
      }

      crate::ToolDefinition
      {
        name : name.to_string(),
        description : description.to_string(),
        parameters : json!
        ({
          "type" : "object",
          "properties" : properties,
          "required" : required,
        }),
      }
    }
  }
}

#[ cfg( feature = "enhanced_function_calling" ) ]
crate::mod_interface!
{
  exposed use private::ToolExecutor;
  exposed use private::ToolRegistry;
  exposed use private::ToolResult;
  exposed use private::helpers;
}
