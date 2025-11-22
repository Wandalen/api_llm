//! Message content types
//!
//! `Role`, `Content` variants, `ImageContent`, and `ImageSource`.

mod private
{
  use serde::{ Serialize, Deserialize };

  use serde_json::Value;

  /// Message role in conversation
  ///
  /// # Examples
  ///
  /// ```
  /// use api_claude::Role;
  ///
  /// // Create different message roles
  /// let user_role = Role::User;
  /// let assistant_role = Role::Assistant;
  /// let system_role = Role::System;
  ///
  /// // Roles can be compared
  /// assert_eq!( user_role, Role::User );
  /// assert_ne!( user_role, Role::Assistant );
  /// ```
  #[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
  pub enum Role
  {
    /// User message
    #[ serde( rename = "user" ) ]
    User,
    /// Assistant message
    #[ serde( rename = "assistant" ) ]
    Assistant,
    /// System message
    #[ serde( rename = "system" ) ]
    System,
  }

  /// Content block in a message
  ///
  /// # Examples
  ///
  /// ```
  /// use api_claude::Content;
  ///
  /// // Create text content
  /// let text_content = Content::Text {
  ///   r#type : "text".to_string(),
  ///   text : "Hello, Claude!".to_string(),
  /// };
  ///
  /// // Content can be serialized/deserialized
  /// let json = serde_json::to_string( &text_content ).unwrap();
  /// assert!( json.contains( "Hello, Claude!" ) );
  /// ```
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  #[ serde( untagged ) ]
  pub enum Content
  {
    /// Text content
    Text
    {
      /// Type - always "text"
      r#type : String,
      /// Text content
      text : String,
    },
    /// Image content (vision feature)
    #[ cfg( feature = "vision" ) ]
    Image
    {
      /// Type - always "image"
      r#type : String,
      /// Image source information
      source : ImageSource,
    },
    /// Tool use content
    #[ cfg( feature = "tools" ) ]
    ToolUse
    {
      /// Type - always "`tool_use`"
      r#type : String,
      /// Unique ID for this tool use
      id : String,
      /// Name of the tool being used
      name : String,
      /// Input parameters for the tool
      input : Value,
    },
    /// Tool result content
    #[ cfg( feature = "tools" ) ]
    ToolResult
    {
      /// Type - always "`tool_result`"
      r#type : String,
      /// ID of the tool use this result corresponds to
      tool_use_id : String,
      /// Result content from the tool
      content : String,
      /// Whether this result represents an error
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      is_error : Option< bool >,
    },
  }

  impl Content
  {
    /// Create new text content
    #[ inline ]
    #[ must_use ]
    pub fn new_text< S : Into< String > >( text : S ) -> Self
    {
      Self::Text
      {
        r#type : "text".to_string(),
        text : text.into(),
      }
    }

    /// Create new image content (requires vision feature)
    #[ cfg( feature = "vision" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn image( source : ImageSource ) -> Self
    {
      Self::Image
      {
        r#type : "image".to_string(),
        source,
      }
    }

    /// Create new tool use content
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn tool_use< S1 : Into< String >, S2 : Into< String > >( id : S1, name : S2, input : Value ) -> Self
    {
      Self::ToolUse
      {
        r#type : "tool_use".to_string(),
        id : id.into(),
        name : name.into(),
        input,
      }
    }

    /// Create new tool result content
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn tool_result< S1 : Into< String >, S2 : Into< String > >( tool_use_id : S1, content : S2 ) -> Self
    {
      Self::ToolResult
      {
        r#type : "tool_result".to_string(),
        tool_use_id : tool_use_id.into(),
        content : content.into(),
        is_error : None,
      }
    }

    /// Create new tool result content with error flag
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn tool_result_error< S1 : Into< String >, S2 : Into< String > >( tool_use_id : S1, content : S2, is_error : bool ) -> Self
    {
      Self::ToolResult
      {
        r#type : "tool_result".to_string(),
        tool_use_id : tool_use_id.into(),
        content : content.into(),
        is_error : Some( is_error ),
      }
    }

    /// Get the content type
    #[ inline ]
    #[ must_use ]
    #[ allow( clippy::match_same_arms ) ] // Different enum variants with conditional compilation
    pub fn r#type( &self ) -> &str
    {
      match self
      {
        Content::Text { r#type, .. } => r#type,
        #[ cfg( feature = "vision" ) ]
        Content::Image { r#type, .. } => r#type,
        #[ cfg( feature = "tools" ) ]
        Content::ToolUse { r#type, .. } => r#type,
        #[ cfg( feature = "tools" ) ]
        Content::ToolResult { r#type, .. } => r#type,
      }
    }

    /// Get text content if this is a text content type
    #[ inline ]
    #[ must_use ]
    pub fn text( &self ) -> Option< &str >
    {
      match self
      {
        Content::Text { text, .. } => Some( text ),
        _ => None,
      }
    }

    /// Check if this content is text type
    #[ inline ]
    #[ must_use ]
    pub fn is_text( &self ) -> bool
    {
      matches!( self, Content::Text { .. } )
    }

    /// Check if this content is image type (requires vision feature)
    #[ cfg( feature = "vision" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn is_image( &self ) -> bool
    {
      matches!( self, Content::Image { .. } )
    }

    /// Check if this content is tool use type
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn is_tool_use( &self ) -> bool
    {
      matches!( self, Content::ToolUse { .. } )
    }

    /// Check if this content is tool result type
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn is_tool_result( &self ) -> bool
    {
      matches!( self, Content::ToolResult { .. } )
    }

    /// Get tool use ID if this is a tool use content
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn tool_use_id( &self ) -> Option< &str >
    {
      match self
      {
        Content::ToolUse { id, .. } => Some( id ),
        _ => None,
      }
    }

    /// Get tool use name if this is a tool use content
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn tool_name( &self ) -> Option< &str >
    {
      match self
      {
        Content::ToolUse { name, .. } => Some( name ),
        _ => None,
      }
    }

    /// Get tool use input if this is a tool use content
    #[ cfg( feature = "tools" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn tool_input( &self ) -> Option< &Value >
    {
      match self
      {
        Content::ToolUse { input, .. } => Some( input ),
        _ => None,
      }
    }
  }

  /// Image content for vision support (requires vision feature)
  #[ cfg( feature = "vision" ) ]
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ImageContent
  {
    /// Type - always "`image`"
    pub r#type : String,
    /// Image source information
    pub source : ImageSource,
  }

  #[ cfg( feature = "vision" ) ]
  impl ImageContent
  {
    /// Create a new image content with the given source
    #[ inline ]
    #[ must_use ]
    pub fn new( source : ImageSource ) -> Self
    {
      Self
      {
        r#type : "image".to_string(),
        source,
      }
    }

    /// Create image content from JPEG base64 data
    #[ inline ]
    #[ must_use ]
    pub fn jpeg< S : Into< String > >( data : S ) -> Self
    {
      Self::new( ImageSource::jpeg( data ) )
    }

    /// Create image content from PNG base64 data
    #[ inline ]
    #[ must_use ]
    pub fn png< S : Into< String > >( data : S ) -> Self
    {
      Self::new( ImageSource::png( data ) )
    }

    /// Create image content from GIF base64 data
    #[ inline ]
    #[ must_use ]
    pub fn gif< S : Into< String > >( data : S ) -> Self
    {
      Self::new( ImageSource::gif( data ) )
    }

    /// Create image content from WebP base64 data
    #[ inline ]
    #[ must_use ]
    pub fn webp< S : Into< String > >( data : S ) -> Self
    {
      Self::new( ImageSource::webp( data ) )
    }

    /// Validate image content
    ///
    /// # Errors
    ///
    /// Returns an error if the image content type is invalid or image source validation fails
    #[ inline ]
    pub fn validate( &self ) -> Result< (), crate::error_tools::Error >
    {
      if self.r#type != "image"
      {
        return Err( crate::error_tools::Error::msg( format!( "Invalid image content type : '{}'. Expected 'image'.", self.r#type ) ) );
      }

      self.source.validate()
    }

    /// Check if the image content is valid
    #[ inline ]
    #[ must_use ]
    pub fn is_valid( &self ) -> bool
    {
      self.r#type == "image" && self.source.is_valid_base64()
    }

    /// Get the media type of the image
    #[ inline ]
    #[ must_use ]
    pub fn media_type( &self ) -> &str
    {
      &self.source.media_type
    }

    /// Get estimated size in bytes
    #[ inline ]
    #[ must_use ]
    pub fn estimated_size_bytes( &self ) -> usize
    {
      self.source.estimated_size_bytes()
    }
  }

  /// Image source specification (requires vision feature)
  #[ cfg( feature = "vision" ) ]
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct ImageSource
  {
    /// Type of image source - currently only "`base64`" 
    pub r#type : String,
    /// MIME type of the image (e.g., "image/jpeg", "image/png", "image/gif", "image/webp")
    pub media_type : String,
    /// Base64-encoded image data
    pub data : String,
  }

  #[ cfg( feature = "vision" ) ]
  impl ImageSource
  {
    /// Create a new base64 image source
    #[ inline ]
    #[ must_use ]
    pub fn base64< S1 : Into< String >, S2 : Into< String > >( media_type : S1, data : S2 ) -> Self
    {
      Self
      {
        r#type : "base64".to_string(),
        media_type : media_type.into(),
        data : data.into(),
      }
    }

    /// Create a JPEG image source from base64 data
    #[ inline ]
    #[ must_use ]
    pub fn jpeg< S : Into< String > >( data : S ) -> Self
    {
      Self::base64( "image/jpeg", data )
    }

    /// Create a PNG image source from base64 data
    #[ inline ]
    #[ must_use ]
    pub fn png< S : Into< String > >( data : S ) -> Self
    {
      Self::base64( "image/png", data )
    }

    /// Create a GIF image source from base64 data
    #[ inline ]
    #[ must_use ]
    pub fn gif< S : Into< String > >( data : S ) -> Self
    {
      Self::base64( "image/gif", data )
    }

    /// Create a WebP image source from base64 data
    #[ inline ]
    #[ must_use ]
    pub fn webp< S : Into< String > >( data : S ) -> Self
    {
      Self::base64( "image/webp", data )
    }

    /// Validate image source format and content
    ///
    /// # Errors
    ///
    /// Returns an error if the source type, media type, or data format is invalid
    #[ inline ]
    pub fn validate( &self ) -> Result< (), crate::error_tools::Error >
    {
      // Validate source type
      if self.r#type != "base64"
      {
        return Err( crate::error_tools::Error::msg( format!( "Invalid image source type : '{}'. Only 'base64' is supported.", self.r#type ) ) );
      }

      // Validate media type
      const VALID_MEDIA_TYPES : &[ &str ] = &[ "image/jpeg", "image/png", "image/gif", "image/webp" ];
      if !VALID_MEDIA_TYPES.contains( &self.media_type.as_str() )
      {
        return Err( crate::error_tools::Error::msg( format!( "Invalid image media type : '{}'. Supported types : {:?}", self.media_type, VALID_MEDIA_TYPES ) ) );
      }

      // Validate data is not empty
      if self.data.is_empty()
      {
        return Err( crate::error_tools::Error::msg( "Image data cannot be empty" ) );
      }

      // Validate base64 format (basic check)
      if !self.data.chars().all( | c | c.is_alphanumeric() || c == '+' || c == '/' || c == '=' )
      {
        return Err( crate::error_tools::Error::msg( "Invalid base64 image data format" ) );
      }

      Ok( () )
    }

    /// Check if image data appears to be valid base64
    #[ inline ]
    #[ must_use ]
    pub fn is_valid_base64( &self ) -> bool
    {
      !self.data.is_empty() && self.data.chars().all( | c | c.is_alphanumeric() || c == '+' || c == '/' || c == '=' )
    }

    /// Get estimated size in bytes (rough approximation from base64 length)
    #[ inline ]
    #[ must_use ]
    pub fn estimated_size_bytes( &self ) -> usize
    {
      // Base64 encoding increases size by ~33%, so reverse estimate
      ( self.data.len() * 3 ) / 4
    }
  }
}

crate::mod_interface!
{
  exposed use Role;
  exposed use Content;

  #[ cfg( feature = "vision" ) ]
  exposed use ImageContent;
  #[ cfg( feature = "vision" ) ]
  exposed use ImageSource;
}
