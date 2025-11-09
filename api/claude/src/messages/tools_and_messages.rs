//! Tool calling and message types
//!
//! `ToolDefinition`, `ToolChoice`, `Message`, and `MessageBuilder`.

mod private
{
  use super::super::content::orphan::*;
  use serde::{ Serialize, Deserialize };
  
  #[ cfg( feature = "tools" ) ]
  use serde_json::Value;


  /// Tool definition for function calling
  #[ cfg( feature = "tools" ) ]
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct ToolDefinition
  {
    /// Name of the tool
    pub name : String,
    /// Description of what the tool does
    pub description : String,
    /// JSON schema for the tool's input parameters
    pub input_schema : Value,
  }

  #[ cfg( feature = "tools" ) ]
  impl ToolDefinition
  {
    /// Create a new tool definition
    #[ inline ]
    #[ must_use ]
    pub fn new< S1 : Into< String >, S2 : Into< String > >( name : S1, description : S2, input_schema : Value ) -> Self
    {
      Self
      {
        name : name.into(),
        description : description.into(),
        input_schema,
      }
    }

    /// Create a tool definition with no parameters
    #[ inline ]
    #[ must_use ]
    pub fn simple< S1 : Into< String >, S2 : Into< String > >( name : S1, description : S2 ) -> Self
    {
      let schema = serde_json::json!({
        "type": "object",
        "properties": {},
        "required": []
      });
      
      Self::new( name, description, schema )
    }
    
    /// Create a tool definition with typed parameters
    #[ inline ]
    #[ must_use ]
    pub fn with_properties< S1 : Into< String >, S2 : Into< String > >( 
      name : S1, 
      description : S2,
      properties : &Value,
      required : &[String],
    ) -> Self
    {
      let schema = serde_json::json!({
        "type": "object",
        "properties": properties.clone(),
        "required": required
      });
      
      Self::new( name, description, schema )
    }
    
    /// Validate this tool definition
    ///
    /// # Errors
    ///
    /// Returns an error if the tool name or description is empty or invalid
    #[ cfg( feature = "error-handling" ) ]
    pub fn validate( &self ) -> crate::error::AnthropicResult< () >
    {
      if self.name.trim().is_empty()
      {
        return Err( crate::error::AnthropicError::InvalidRequest( 
          "tool name cannot be empty".to_string() 
        ) );
      }
      
      if self.description.trim().is_empty()
      {
        return Err( crate::error::AnthropicError::InvalidRequest( 
          format!( "tool '{}' description cannot be empty", self.name )
        ) );
      }
      
      // Validate name format (alphanumeric, underscore, hyphen)
      if !self.name.chars().all( | c | c.is_alphanumeric() || c == '_' || c == '-' )
      {
        return Err( crate::error::AnthropicError::InvalidRequest( 
          format!( "tool name '{}' contains invalid characters - only alphanumeric, underscore, and hyphen allowed", self.name )
        ) );
      }
      
      // Check name length
      if self.name.len() > 64
      {
        return Err( crate::error::AnthropicError::InvalidRequest( 
          format!( "tool name '{}' too long - maximum 64 characters", self.name )
        ) );
      }
      
      // Check description length
      if self.description.len() > 1024
      {
        return Err( crate::error::AnthropicError::InvalidRequest( 
          format!( "tool '{}' description too long - maximum 1024 characters", self.name )
        ) );
      }
      
      Ok( () )
    }
    
    /// Get the tool's name
    #[ inline ]
    #[ must_use ]
    pub fn name( &self ) -> &str
    {
      &self.name
    }
    
    /// Get the tool's description
    #[ inline ]
    #[ must_use ]
    pub fn description( &self ) -> &str
    {
      &self.description
    }
    
    /// Get the tool's input schema
    #[ inline ]
    #[ must_use ]
    pub fn input_schema( &self ) -> &Value
    {
      &self.input_schema
    }
  }

  /// Tool use content in assistant messages
  #[ cfg( feature = "tools" ) ]
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ToolUseContent
  {
    /// Type - always "`tool_use`"
    pub r#type : String,
    /// Unique ID for this tool use
    pub id : String,
    /// Name of the tool being used
    pub name : String,
    /// Input parameters for the tool
    pub input : Value,
  }

  /// Tool result content in user messages
  #[ cfg( feature = "tools" ) ]
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ToolResultContent
  {
    /// Type - always "`tool_result`"
    pub r#type : String,
    /// ID of the tool use this result corresponds to
    pub tool_use_id : String,
    /// Result content from the tool
    pub content : String,
    /// Whether this result represents an error
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub is_error : Option< bool >,
  }

  /// Choice for how the model should use tools
  #[ cfg( feature = "tools" ) ]
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  #[ serde( tag = "type", rename_all = "lowercase" ) ]
  pub enum ToolChoice
  {
    /// Never use tools
    None,
    /// Use tools automatically when needed
    Auto,
    /// Force use of any available tool
    Any,
    /// Force use of a specific tool
    Tool
    {
      /// Name of the specific tool to use
      name : String,
    },
  }

  #[ cfg( feature = "tools" ) ]
  impl ToolChoice
  {
    /// Create a tool choice for a specific tool
    #[ inline ]
    #[ must_use ]
    pub fn specific< S : Into< String > >( name : S ) -> Self
    {
      Self::Tool { name : name.into() }
    }

    /// Check if this is the "auto" choice
    #[ inline ]
    #[ must_use ]
    pub fn is_auto( &self ) -> bool
    {
      matches!( self, ToolChoice::Auto )
    }

    /// Check if this is the "none" choice
    #[ inline ]
    #[ must_use ]
    pub fn is_none( &self ) -> bool
    {
      matches!( self, ToolChoice::None )
    }

    /// Check if this is the "any" choice
    #[ inline ]
    #[ must_use ]
    pub fn is_any( &self ) -> bool
    {
      matches!( self, ToolChoice::Any )
    }

    /// Check if this is a specific tool choice
    #[ inline ]
    #[ must_use ]
    pub fn is_specific( &self ) -> bool
    {
      matches!( self, ToolChoice::Tool { .. } )
    }

    /// Get the specific tool name if this is a specific tool choice
    #[ inline ]
    #[ must_use ]
    pub fn tool_name( &self ) -> Option< &str >
    {
      match self
      {
        ToolChoice::Tool { name } => Some( name ),
        _ => None,
      }
    }
  }


  /// Message in conversation
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct Message
  {
    /// Role of message sender
    pub role : Role,
    /// Content of message
    pub content : Vec< Content >,
    /// Optional cache control for this message
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub cache_control : Option< crate::CacheControl >,
  }

  /// Builder for constructing messages with multiple content types
  #[ derive( Debug, Default ) ]
  pub struct MessageBuilder
  {
    role : Option< Role >,
    content : Vec< Content >,
  }

  impl MessageBuilder
  {
    /// Create a new message builder
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self::default()
    }

    /// Set the role for this message
    #[ inline ]
    #[ must_use ]
    pub fn role( mut self, role : Role ) -> Self
    {
      self.role = Some( role );
      self
    }

    /// Set role to User
    #[ inline ]
    #[ must_use ]
    pub fn user( mut self ) -> Self
    {
      self.role = Some( Role::User );
      self
    }

    /// Set role to Assistant
    #[ inline ]
    #[ must_use ]
    pub fn assistant( mut self ) -> Self
    {
      self.role = Some( Role::Assistant );
      self
    }

    /// Add text content
    #[ inline ]
    #[ must_use ]
    pub fn text< S : Into< String > >( mut self, text : S ) -> Self
    {
      self.content.push( Content::new_text( text ) );
      self
    }

    /// Add image content (requires vision feature)
    #[ cfg( feature = "vision" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn image( mut self, source : ImageSource ) -> Self
    {
      self.content.push( Content::image( source ) );
      self
    }

    /// Add tool use content
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn tool_use< S1 : Into< String >, S2 : Into< String > >( mut self, id : S1, name : S2, input : Value ) -> Self
    {
      self.content.push( Content::tool_use( id, name, input ) );
      self
    }

    /// Add tool result content
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn tool_result< S1 : Into< String >, S2 : Into< String > >( mut self, tool_use_id : S1, content : S2 ) -> Self
    {
      self.content.push( Content::tool_result( tool_use_id, content ) );
      self
    }

    /// Add tool result content with error flag
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn tool_result_error< S1 : Into< String >, S2 : Into< String > >( mut self, tool_use_id : S1, content : S2, is_error : bool ) -> Self
    {
      self.content.push( Content::tool_result_error( tool_use_id, content, is_error ) );
      self
    }

    /// Add any content
    #[ inline ]
    #[ must_use ]
    pub fn content( mut self, content : Content ) -> Self
    {
      self.content.push( content );
      self
    }

    /// Add multiple content items
    #[ inline ]
    #[ must_use ]
    pub fn contents( mut self, contents : Vec< Content > ) -> Self
    {
      self.content.extend( contents );
      self
    }

    /// Build the message
    ///
    /// # Panics
    ///
    /// Panics if role is not set or content is empty
    #[ inline ]
    #[ must_use ]
    pub fn build( self ) -> Message
    {
      Message
      {
        role : self.role.expect( "Role must be set" ),
        content : if self.content.is_empty() { panic!( "Content cannot be empty" ) } else { self.content },
        cache_control : None,
      }
    }
  }

  impl Message
  {
    /// Create a new message builder
    #[ inline ]
    #[ must_use ]
    pub fn builder() -> MessageBuilder
    {
      MessageBuilder::new()
    }

    /// Create new user message
    #[ inline ]
    #[ must_use ]
    pub fn user< S : Into< String > >( text : S ) -> Self
    {
      Self
      {
        role : Role::User,
        content : vec![ Content::new_text( text ) ],
        cache_control : None,
      }
    }

    /// Create new assistant message
    #[ inline ]
    #[ must_use ]
    pub fn assistant< S : Into< String > >( text : S ) -> Self
    {
      Self
      {
        role : Role::Assistant,
        content : vec![ Content::new_text( text ) ],
        cache_control : None,
      }
    }

    /// Create assistant message with tool use
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn assistant_with_tool_use( tool_uses : Vec< ToolUseContent > ) -> Self
    {
      let content = tool_uses.into_iter().map( | tool_use |
        Content::ToolUse
        {
          r#type : tool_use.r#type,
          id : tool_use.id,
          name : tool_use.name,
          input : tool_use.input,
        }
      ).collect();

      Self
      {
        role : Role::Assistant,
        content,
        cache_control : None,
      }
    }

    /// Create user message with tool result
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn user_with_tool_result( tool_results : Vec< ToolResultContent > ) -> Self
    {
      let content = tool_results.into_iter().map( | tool_result |
        Content::ToolResult
        {
          r#type : tool_result.r#type,
          tool_use_id : tool_result.tool_use_id,
          content : tool_result.content,
          is_error : tool_result.is_error,
        }
      ).collect();

      Self
      {
        role : Role::User,
        content,
        cache_control : None,
      }
    }

    /// Create user message with image content only (requires vision feature)
    #[ cfg( feature = "vision" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn user_image( image : ImageContent ) -> Self
    {
      let content = vec![ Content::Image
      {
        r#type : image.r#type,
        source : image.source,
      } ];

      Self
      {
        role : Role::User,
        content,
        cache_control : None,
      }
    }

    /// Create user message with text and image content (requires vision feature)
    #[ cfg( feature = "vision" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn user_with_image< S : Into< String > >( text : S, image : ImageContent ) -> Self
    {
      let content = vec![
        Content::new_text( text ),
        Content::Image { r#type : image.r#type, source : image.source }
      ];

      Self
      {
        role : Role::User,
        content,
        cache_control : None,
      }
    }

    /// Create user message with text and multiple images (requires vision feature)
    #[ cfg( feature = "vision" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn user_with_images< S : Into< String > >( text : S, images : Vec< ImageContent > ) -> Self
    {
      let mut content = vec![ Content::new_text( text ) ];

      for image in images
      {
        content.push( Content::Image
        {
          r#type : image.r#type,
          source : image.source,
        } );
      }

      Self
      {
        role : Role::User,
        content,
        cache_control : None,
      }
    }

    /// Get the first text content from this message
    #[ inline ]
    #[ must_use ]
    pub fn first_text( &self ) -> Option< &str >
    {
      self.content.iter()
        .find_map( | c | c.text() )
    }

    /// Get all text content from this message
    #[ inline ]
    #[ must_use ]
    pub fn all_text( &self ) -> Vec< &str >
    {
      self.content.iter()
        .filter_map( | c | c.text() )
        .collect()
    }

    /// Count content items by type
    #[ inline ]
    #[ must_use ]
    pub fn count_by_type( &self, content_type : &str ) -> usize
    {
      self.content.iter()
        .filter( | c | c.r#type() == content_type )
        .count()
    }

    /// Check if message has any text content
    #[ inline ]
    #[ must_use ]
    pub fn has_text( &self ) -> bool
    {
      self.content.iter().any( Content::is_text )
    }

    /// Check if message has any image content (requires vision feature)
    #[ cfg( feature = "vision" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn has_images( &self ) -> bool
    {
      self.content.iter().any( Content::is_image )
    }

    /// Check if message has any tool use content
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn has_tool_use( &self ) -> bool
    {
      self.content.iter().any( Content::is_tool_use )
    }

    /// Check if message has any tool result content
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn has_tool_results( &self ) -> bool
    {
      self.content.iter().any( Content::is_tool_result )
    }

    /// Get all tool use IDs from this message
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn tool_use_ids( &self ) -> Vec< &str >
    {
      self.content.iter()
        .filter_map( | c | c.tool_use_id() )
        .collect()
    }
  }
}

crate::mod_interface!
{
  exposed use Message;
  exposed use MessageBuilder;
  
  #[ cfg( feature = "tools" ) ]
  exposed use ToolDefinition;
  #[ cfg( feature = "tools" ) ]
  exposed use ToolUseContent;
  #[ cfg( feature = "tools" ) ]
  exposed use ToolResultContent;
  #[ cfg( feature = "tools" ) ]
  exposed use ToolChoice;
}
