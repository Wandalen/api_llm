//! Tool definitions and function calling support for `HuggingFace` API.

use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

/// Tool definition for function calling
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct Tool
{
  /// Tool name
  pub name : String,
  
  /// Tool description
  pub description : String,
  
  /// Function parameters schema
  pub parameters : ToolParameters,
  
  /// Whether the tool is required
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub required : Option< bool >,
}

impl Tool
{
  /// Create a new tool definition
  #[ inline ]
  #[ must_use ]
  pub fn new( 
  name : impl Into< String >, 
  description : impl Into< String >, 
  parameters : ToolParameters 
  ) -> Self
  {
  Self
  {
      name : name.into(),
      description : description.into(),
      parameters,
      required : None,
  }
  }
  
  /// Set if the tool is required
  #[ inline ]
  #[ must_use ]
  pub fn with_required( mut self, required : bool ) -> Self
  {
  self.required = Some( required );
  self
  }
}

/// Tool parameters schema
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ToolParameters
{
  /// Parameter type (usually "object")
  #[ serde( rename = "type" ) ]
  pub parameter_type : String,
  
  /// Parameter properties
  pub properties : HashMap< String, ParameterProperty >,
  
  /// Required parameter names
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub required : Option< Vec< String > >,
}

impl Default for ToolParameters
{
  #[ inline ]
  fn default() -> Self
  {
  Self::new()
  }
}

impl ToolParameters
{
  /// Create new tool parameters
  #[ inline ]
  #[ must_use ]
  pub fn new() -> Self
  {
  Self
  {
      parameter_type : "object".to_string(),
      properties : HashMap::new(),
      required : None,
  }
  }
  
  /// Add a parameter property
  #[ inline ]
  #[ must_use ]
  pub fn with_property( mut self, name : impl Into< String >, property : ParameterProperty ) -> Self
  {
  self.properties.insert( name.into(), property );
  self
  }
  
  /// Set required parameters
  #[ inline ]
  #[ must_use ]
  pub fn with_required( mut self, required : Vec< String > ) -> Self
  {
  self.required = Some( required );
  self
  }
}

// No Default implementation - use new() for explicit configuration

/// Parameter property definition
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ParameterProperty
{
  /// Property type
  #[ serde( rename = "type" ) ]
  pub property_type : String,
  
  /// Property description
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub description : Option< String >,
  
  /// Enum values for string properties
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub enum_values : Option< Vec< serde_json::Value > >,
  
  /// Default value
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub default : Option< serde_json::Value >,
}

impl ParameterProperty
{
  /// Create a string parameter property
  #[ inline ]
  #[ must_use ]
  pub fn string( description : impl Into< String > ) -> Self
  {
  Self
  {
      property_type : "string".to_string(),
      description : Some( description.into() ),
      enum_values : None,
      default : None,
  }
  }
  
  /// Create a number parameter property
  #[ inline ]
  #[ must_use ]
  pub fn number( description : impl Into< String > ) -> Self
  {
  Self
  {
      property_type : "number".to_string(),
      description : Some( description.into() ),
      enum_values : None,
      default : None,
  }
  }
  
  /// Create a boolean parameter property
  #[ inline ]
  #[ must_use ]
  pub fn boolean( description : impl Into< String > ) -> Self
  {
  Self
  {
      property_type : "boolean".to_string(),
      description : Some( description.into() ),
      enum_values : None,
      default : None,
  }
  }
}