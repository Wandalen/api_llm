//! Tests for enhanced function calling functionality
//!
//! Verifies the `ToolExecutor` trait, `ToolRegistry`, and helper functions
//! for type-safe tool execution.

#[ cfg( all( test, feature = "enhanced_function_calling" ) ) ]
mod tests
{
  use api_ollama::
  {
    enhanced_function_calling::{ ToolExecutor, ToolRegistry, ToolResult, helpers },
    ToolCall,
  };
  use serde_json::json;

  /// Example weather tool for testing
  struct WeatherTool;

  impl ToolExecutor for WeatherTool
  {
    fn name( &self ) -> &'static str
    {
      "get_weather"
    }

    fn description( &self ) -> &'static str
    {
      "Get current weather for a location"
    }

    fn parameter_schema( &self ) -> serde_json::Value
    {
      json!
      ({
        "type" : "object",
        "properties" :
        {
          "location" :
          {
            "type" : "string",
            "description" : "City name",
          },
          "unit" :
          {
            "type" : "string",
            "enum" : [ "celsius", "fahrenheit" ],
            "description" : "Temperature unit",
          },
        },
        "required" : [ "location" ],
      })
    }

    fn execute( &self, params : serde_json::Value ) -> ToolResult
    {
      let location = params[ "location" ].as_str()
        .ok_or( "Missing location parameter".to_string() )?;

      let unit = params[ "unit" ].as_str().unwrap_or( "fahrenheit" );

      Ok( format!( "Weather in {}: Sunny, 72°{}", location, if unit == "celsius" { "C" } else { "F" } ) )
    }
  }

  /// Example calculator tool for testing
  struct CalculatorTool;

  impl ToolExecutor for CalculatorTool
  {
    fn name( &self ) -> &'static str
    {
      "calculate"
    }

    fn description( &self ) -> &'static str
    {
      "Perform basic arithmetic"
    }

    fn parameter_schema( &self ) -> serde_json::Value
    {
      json!
      ({
        "type" : "object",
        "properties" :
        {
          "operation" :
          {
            "type" : "string",
            "enum" : [ "add", "subtract", "multiply", "divide" ],
          },
          "a" : { "type" : "number" },
          "b" : { "type" : "number" },
        },
        "required" : [ "operation", "a", "b" ],
      })
    }

    fn execute( &self, params : serde_json::Value ) -> ToolResult
    {
      let operation = params[ "operation" ].as_str()
        .ok_or( "Missing operation".to_string() )?;
      let a = params[ "a" ].as_f64()
        .ok_or( "Missing operand a".to_string() )?;
      let b = params[ "b" ].as_f64()
        .ok_or( "Missing operand b".to_string() )?;

      let result = match operation
      {
        "add" => a + b,
        "subtract" => a - b,
        "multiply" => a * b,
        "divide" =>
        {
          if b == 0.0
          {
            return Err( "Division by zero".to_string() );
          }
          a / b
        },
        _ => return Err( format!( "Unknown operation : {operation}" ) ),
      };

      Ok( format!( "{result}" ) )
    }
  }

  /// Test tool definition generation
  #[ test ]
  fn test_tool_definition()
  {
    let tool = WeatherTool;
    let definition = tool.definition();

    assert_eq!( definition.name, "get_weather" );
    assert_eq!( definition.description, "Get current weather for a location" );
    assert!( definition.parameters.is_object() );

    let params = definition.parameters.as_object().unwrap();
    assert!( params.contains_key( "properties" ) );
    assert!( params.contains_key( "required" ) );
  }

  /// Test tool execution with valid parameters
  #[ test ]
  fn test_tool_execution_valid()
  {
    let tool = WeatherTool;
    let params = json!
    ({
      "location" : "San Francisco",
      "unit" : "celsius",
    });

    let result = tool.execute( params );
    assert!( result.is_ok() );

    let output = result.unwrap();
    assert!( output.contains( "San Francisco" ) );
    assert!( output.contains( "72°C" ) );
  }

  /// Test tool execution with missing required parameter
  #[ test ]
  fn test_tool_execution_missing_param()
  {
    let tool = WeatherTool;
    let params = json!( {} );

    let result = tool.execute( params );
    assert!( result.is_err() );

    let error = result.unwrap_err();
    assert!( error.contains( "Missing location" ) );
  }

  /// Test tool registry creation
  #[ test ]
  fn test_registry_creation()
  {
    let registry = ToolRegistry::new();
    assert!( registry.is_empty() );
    assert_eq!( registry.len(), 0 );
  }

  /// Test tool registration
  #[ test ]
  fn test_registry_registration()
  {
    let mut registry = ToolRegistry::new();

    registry.register( Box::new( WeatherTool ) );
    assert_eq!( registry.len(), 1 );
    assert!( registry.contains( "get_weather" ) );

    registry.register( Box::new( CalculatorTool ) );
    assert_eq!( registry.len(), 2 );
    assert!( registry.contains( "calculate" ) );
  }

  /// Test registry definitions
  #[ test ]
  fn test_registry_definitions()
  {
    let mut registry = ToolRegistry::new();
    registry.register( Box::new( WeatherTool ) );
    registry.register( Box::new( CalculatorTool ) );

    let definitions = registry.definitions();
    assert_eq!( definitions.len(), 2 );

    let names : Vec< _ > = definitions.iter().map( | d | d.name.as_str() ).collect();
    assert!( names.contains( &"get_weather" ) );
    assert!( names.contains( &"calculate" ) );
  }

  /// Test registry execution
  #[ test ]
  fn test_registry_execution()
  {
    let mut registry = ToolRegistry::new();
    registry.register( Box::new( WeatherTool ) );

    let tool_call = ToolCall
    {
      id : "call_123".to_string(),
      function : json!
      ({
        "name" : "get_weather",
        "arguments" :
        {
          "location" : "New York",
        },
      }),
    };

    let result = registry.execute( &tool_call );
    assert!( result.is_ok() );

    let output = result.unwrap();
    assert!( output.contains( "New York" ) );
  }

  /// Test registry execution with unknown tool
  #[ test ]
  fn test_registry_execution_unknown_tool()
  {
    let registry = ToolRegistry::new();

    let tool_call = ToolCall
    {
      id : "call_456".to_string(),
      function : json!
      ({
        "name" : "unknown_tool",
        "arguments" : {},
      }),
    };

    let result = registry.execute( &tool_call );
    assert!( result.is_err() );

    let error = result.unwrap_err();
    assert!( error.contains( "not found" ) );
  }

  /// Test calculator tool execution
  #[ test ]
  fn test_calculator_tool()
  {
    let tool = CalculatorTool;

    let params = json!
    ({
      "operation" : "add",
      "a" : 10,
      "b" : 5,
    });

    let result = tool.execute( params ).unwrap();
    assert_eq!( result, "15" );

    let params = json!
    ({
      "operation" : "multiply",
      "a" : 7,
      "b" : 6,
    });

    let result = tool.execute( params ).unwrap();
    assert_eq!( result, "42" );
  }

  /// Test calculator division by zero
  #[ test ]
  fn test_calculator_division_by_zero()
  {
    let tool = CalculatorTool;

    let params = json!
    ({
      "operation" : "divide",
      "a" : 10,
      "b" : 0,
    });

    let result = tool.execute( params );
    assert!( result.is_err() );
    assert!( result.unwrap_err().contains( "Division by zero" ) );
  }

  /// Test helper : `create_simple_tool`
  #[ test ]
  fn test_helper_create_simple_tool()
  {
    let tool = helpers::create_simple_tool(
      "test_tool",
      "A test tool",
      &[
        ( "param1", "string", "First parameter" ),
        ( "param2", "number", "Second parameter" ),
      ],
      &[ "param1" ],
    );

    assert_eq!( tool.name, "test_tool" );
    assert_eq!( tool.description, "A test tool" );

    let params = tool.parameters.as_object().unwrap();
    let properties = params[ "properties" ].as_object().unwrap();
    assert_eq!( properties.len(), 2 );
    assert!( properties.contains_key( "param1" ) );
    assert!( properties.contains_key( "param2" ) );

    let required = params[ "required" ].as_array().unwrap();
    assert_eq!( required.len(), 1 );
    assert_eq!( required[ 0 ], "param1" );
  }

  /// Test helper : `create_enum_tool`
  #[ test ]
  fn test_helper_create_enum_tool()
  {
    let tool = helpers::create_enum_tool(
      "mode_tool",
      "Set mode",
      &[
        ( "mode", &[ "fast", "slow", "medium" ], "Operation mode" ),
      ],
      &[ "mode" ],
    );

    assert_eq!( tool.name, "mode_tool" );

    let params = tool.parameters.as_object().unwrap();
    let properties = params[ "properties" ].as_object().unwrap();
    let mode_param = properties[ "mode" ].as_object().unwrap();

    assert_eq!( mode_param[ "type" ], "string" );
    assert!( mode_param.contains_key( "enum" ) );

    let enum_values = mode_param[ "enum" ].as_array().unwrap();
    assert_eq!( enum_values.len(), 3 );
  }

  /// Test registry debug output
  #[ test ]
  fn test_registry_debug()
  {
    let mut registry = ToolRegistry::new();
    registry.register( Box::new( WeatherTool ) );

    let debug_output = format!( "{registry:?}" );
    assert!( debug_output.contains( "ToolRegistry" ) );
    assert!( debug_output.contains( "get_weather" ) );
  }

  /// Test registry default
  #[ test ]
  fn test_registry_default()
  {
    let registry = ToolRegistry::default();
    assert!( registry.is_empty() );
  }
}
