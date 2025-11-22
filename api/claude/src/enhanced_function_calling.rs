//! Enhanced function calling with type-safe execution and validation
//!
//! Provides framework for defining, validating, and executing tools with
//! automatic JSON schema generation and type-safe parameter handling.
//!
//! # Architecture
//!
//! - `ToolExecutor` trait: Defines tool execution interface
//! - `ToolRegistry`: Manages tool registration and dispatch
//! - `ToolResult`: Type-safe result for tool execution
//! - Helper functions for creating tool definitions with type safety
//!
//! # Example
//!
//! ```rust,ignore
//! use api_claude::enhanced_function_calling::{ ToolExecutor, ToolRegistry };
//! use api_claude::ToolDefinition;
//!
//! struct WeatherTool;
//!
//! impl ToolExecutor for WeatherTool
//! {
//!   fn name( &self ) -> &str { "get_weather" }
//!   fn description( &self ) -> &str { "Get weather for a location" }
//!
//!   fn execute( &self, params : serde_json::Value ) -> ToolResult
//!   {
//!     let location = params[ "location" ].as_str()
//!       .ok_or( "Missing location parameter" )?;
//!     Ok( format!( "Weather in {}: Sunny, 72°F", location ) )
//!   }
//! }
//!
//! let mut registry = ToolRegistry::new();
//! registry.register( Box::new( WeatherTool ) );
//! let definitions = registry.definitions();
//! ```

mod private
{
  use std::collections::HashMap;
  use serde_json::Value;

  /// Result type for tool execution
  pub type ToolResult = Result< String, String >;

  /// Trait for executable tools with type-safe parameter handling
  pub trait ToolExecutor : Send + Sync
  {
    /// Get tool name
    fn name( &self ) -> &str;

    /// Get tool description
    fn description( &self ) -> &str;

    /// Get JSON schema for tool parameters
    ///
    /// Returns JSON schema object describing expected parameters.
    /// Default returns empty object schema.
    fn parameter_schema( &self ) -> Value
    {
      serde_json::json!(
      {
        "type" : "object",
        "properties" : {},
        "required" : []
      })
    }

    /// Execute tool with given parameters
    ///
    /// # Arguments
    ///
    /// * `params` - JSON object containing tool parameters
    ///
    /// # Returns
    ///
    /// Tool execution result as string, or error message
    ///
    /// # Errors
    ///
    /// Returns error if parameters are invalid or execution fails
    fn execute( &self, params : Value ) -> ToolResult;

    /// Get full tool definition for API requests
    ///
    /// Automatically generates `ToolDefinition` from tool metadata
    fn definition( &self ) -> crate::ToolDefinition
    {
      crate::ToolDefinition
      {
        name : self.name().to_string(),
        description : self.description().to_string(),
        input_schema : self.parameter_schema(),
      }
    }
  }

  /// Registry for managing and executing tools
  ///
  /// Allows registering multiple tools, retrieving definitions
  /// for API requests, and executing them by name.
  pub struct ToolRegistry
  {
    tools : HashMap< String, Box< dyn ToolExecutor > >,
  }

  impl std::fmt::Debug for ToolRegistry
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      f.debug_struct( "ToolRegistry" )
        .field( "tool_count", &self.tools.len() )
        .finish()
    }
  }

  impl ToolRegistry
  {
    /// Create new empty tool registry
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
    /// If a tool with same name exists, it will be replaced.
    pub fn register( &mut self, tool : Box< dyn ToolExecutor > )
    {
      let name = tool.name().to_string();
      self.tools.insert( name, tool );
    }

    /// Get tool by name
    pub fn get( &self, name : &str ) -> Option< &dyn ToolExecutor >
    {
      self.tools.get( name ).map( | t | &**t )
    }

    /// Execute a tool by name with given parameters
    ///
    /// # Errors
    ///
    /// Returns error if tool not found or execution fails
    pub fn execute( &self, name : &str, params : Value ) -> ToolResult
    {
      let tool = self.get( name )
        .ok_or_else( || format!( "Tool '{name}' not found in registry" ) )?;

      tool.execute( params )
    }

    /// Get all tool definitions for use in API requests
    #[ must_use ]
    pub fn definitions( &self ) -> Vec< crate::ToolDefinition >
    {
      self.tools
        .values()
        .map( | tool | tool.definition() )
        .collect()
    }

    /// Get names of all registered tools
    #[ must_use ]
    pub fn tool_names( &self ) -> Vec< String >
    {
      self.tools.keys().cloned().collect()
    }

    /// Check if a tool is registered
    #[ must_use ]
    pub fn has_tool( &self, name : &str ) -> bool
    {
      self.tools.contains_key( name )
    }

    /// Get count of registered tools
    #[ must_use ]
    pub fn len( &self ) -> usize
    {
      self.tools.len()
    }

    /// Check if registry is empty
    #[ must_use ]
    pub fn is_empty( &self ) -> bool
    {
      self.tools.is_empty()
    }
  }

  impl Default for ToolRegistry
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Helper for creating a simple tool definition
  #[ must_use ]
  pub fn create_tool_definition(
    name : impl Into< String >,
    description : impl Into< String >,
    input_schema : Value,
  ) -> crate::ToolDefinition
  {
    crate::ToolDefinition
    {
      name : name.into(),
      description : description.into(),
      input_schema,
    }
  }

  /// Helper for creating a parameter schema
  #[ must_use ]
  pub fn create_parameter_schema(
    properties : &Value,
    required : &[ String ],
  ) -> Value
  {
    serde_json::json!(
    {
      "type" : "object",
      "properties" : properties,
      "required" : required
    })
  }

  #[ cfg( test ) ]
  mod tests
  {
    use super::*;

    struct TestTool
    {
      name : String,
      description : String,
    }

    impl ToolExecutor for TestTool
    {
      fn name( &self ) -> &str
      {
        &self.name
      }

      fn description( &self ) -> &str
      {
        &self.description
      }

      fn parameter_schema( &self ) -> Value
      {
        serde_json::json!(
        {
          "type" : "object",
          "properties" :
          {
            "input" :
            {
              "type" : "string",
              "description" : "Test input"
            }
          },
          "required" : [ "input" ]
        })
      }

      fn execute( &self, params : Value ) -> ToolResult
      {
        let input = params[ "input" ].as_str()
          .ok_or( "Missing input parameter" )?;
        Ok( format!( "Executed {} with: {}", self.name, input ) )
      }
    }

    #[ test ]
    fn test_tool_registry_new()
    {
      let registry = ToolRegistry::new();
      assert!( registry.is_empty() );
      assert_eq!( registry.len(), 0 );
    }

    #[ test ]
    fn test_tool_registry_register()
    {
      let mut registry = ToolRegistry::new();

      let tool = Box::new( TestTool
      {
        name : "test_tool".to_string(),
        description : "Test tool".to_string(),
      } );

      registry.register( tool );

      assert!( !registry.is_empty() );
      assert_eq!( registry.len(), 1 );
      assert!( registry.has_tool( "test_tool" ) );
    }

    #[ test ]
    fn test_tool_registry_execute()
    {
      let mut registry = ToolRegistry::new();

      let tool = Box::new( TestTool
      {
        name : "test_tool".to_string(),
        description : "Test tool".to_string(),
      } );

      registry.register( tool );

      let params = serde_json::json!( { "input" : "test value" } );
      let result = registry.execute( "test_tool", params );

      assert!( result.is_ok() );
      let output = result.unwrap();
      assert!( output.contains( "test_tool" ) );
      assert!( output.contains( "test value" ) );
    }

    #[ test ]
    fn test_tool_registry_execute_not_found()
    {
      let registry = ToolRegistry::new();
      let params = serde_json::json!( { "input" : "test" } );
      let result = registry.execute( "nonexistent", params );

      assert!( result.is_err() );
      let err = result.unwrap_err();
      assert!( err.contains( "not found" ) );
    }

    #[ test ]
    fn test_tool_registry_definitions()
    {
      let mut registry = ToolRegistry::new();

      let tool = Box::new( TestTool
      {
        name : "test_tool".to_string(),
        description : "Test tool description".to_string(),
      } );

      registry.register( tool );

      let definitions = registry.definitions();
      assert_eq!( definitions.len(), 1 );
      assert_eq!( definitions[ 0 ].name, "test_tool" );
      assert_eq!( definitions[ 0 ].description, "Test tool description" );
    }

    #[ test ]
    fn test_tool_registry_tool_names()
    {
      let mut registry = ToolRegistry::new();

      registry.register( Box::new( TestTool
      {
        name : "tool1".to_string(),
        description : "Tool 1".to_string(),
      } ) );

      registry.register( Box::new( TestTool
      {
        name : "tool2".to_string(),
        description : "Tool 2".to_string(),
      } ) );

      let names = registry.tool_names();
      assert_eq!( names.len(), 2 );
      assert!( names.contains( &"tool1".to_string() ) );
      assert!( names.contains( &"tool2".to_string() ) );
    }

    #[ test ]
    fn test_create_tool_definition()
    {
      let schema = serde_json::json!( { "type" : "object" } );
      let tool_def = create_tool_definition( "test", "Test tool", schema.clone() );

      assert_eq!( tool_def.name, "test" );
      assert_eq!( tool_def.description, "Test tool" );
      assert_eq!( tool_def.input_schema, schema );
    }

    #[ test ]
    fn test_create_parameter_schema()
    {
      let properties = serde_json::json!(
      {
        "location" :
        {
          "type" : "string",
          "description" : "City name"
        }
      });

      let schema = create_parameter_schema( &properties, &[ "location".to_string() ] );

      assert_eq!( schema[ "type" ], "object" );
      assert!( schema[ "properties" ][ "location" ].is_object() );
      assert_eq!( schema[ "required" ][ 0 ], "location" );
    }

    #[ test ]
    fn test_tool_executor_default_schema()
    {
      struct MinimalTool;

      impl ToolExecutor for MinimalTool
      {
        fn name( &self ) -> &'static str { "minimal" }
        fn description( &self ) -> &'static str { "Minimal tool" }
        fn execute( &self, _params : Value ) -> ToolResult
        {
          Ok( "done".to_string() )
        }
      }

      let tool = MinimalTool;
      let schema = tool.parameter_schema();

      assert_eq!( schema[ "type" ], "object" );
      assert!( schema[ "properties" ].is_object() );
    }
  }
}

crate::mod_interface!
{
  exposed use
  {
    ToolExecutor,
    ToolRegistry,
    ToolResult,
    create_tool_definition,
    create_parameter_schema,
  };
}
