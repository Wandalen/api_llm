//! cURL generation functionality for `OpenAI` API requests.
//!
//! This module provides utilities to convert API requests into equivalent cURL commands
//! for debugging, documentation, and external integration purposes.

/// Define a private namespace for all its items.
mod private
{
  use std::collections::HashMap;
  use serde::Serialize;
  use crate::error::{ OpenAIError, Result };

  /// Main cURL generation utility
  #[ derive( Debug, Clone ) ]
  pub struct CurlGenerator
  {
    /// List of supported HTTP methods
    supported_methods : Vec< String >,
  }

  impl CurlGenerator
  {
    /// Create a new cURL generator
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        supported_methods : vec![
          "GET".to_string(),
          "POST".to_string(),
          "PUT".to_string(),
          "DELETE".to_string(),
        ],
      }
    }

    /// Check if cURL generation is supported
    #[ inline ]
    #[ must_use ]
    pub fn can_generate_curl( &self ) -> bool
    {
      true
    }

    /// Get list of supported HTTP methods
    #[ inline ]
    #[ must_use ]
    pub fn get_supported_methods( &self ) -> &Vec< String >
    {
      &self.supported_methods
    }
  }

  /// Builder for constructing cURL requests
  #[ derive( Debug, Clone ) ]
  pub struct CurlRequestBuilder
  {
    method : String,
    url : String,
    headers : Vec< (String, String) >,
    body : Option< String >,
  }

  impl CurlRequestBuilder
  {
    /// Create a new cURL request builder
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        method : "GET".to_string(),
        url : String::new(),
        headers : Vec::new(),
        body : None,
      }
    }

    /// Set the HTTP method
    #[ inline ]
    #[ must_use ]
    pub fn method( mut self, method : &str ) -> Self
    {
      self.method = method.to_string();
      self
    }

    /// Set the URL
    #[ inline ]
    #[ must_use ]
    pub fn url( mut self, url : &str ) -> Self
    {
      self.url = url.to_string();
      self
    }

    /// Add a header
    #[ inline ]
    #[ must_use ]
    pub fn header( mut self, key : &str, value : &str ) -> Self
    {
      self.headers.push( (key.to_string(), value.to_string()) );
      self
    }

    /// Set the request body
    #[ inline ]
    #[ must_use ]
    pub fn body( mut self, body : &str ) -> Self
    {
      self.body = Some( body.to_string() );
      self
    }

    /// Build the cURL request
    #[ inline ]
    #[ must_use ]
    pub fn build( self ) -> CurlRequest
    {
      CurlRequest
      {
        method : self.method,
        url : self.url,
        headers : self.headers,
        body : self.body,
      }
    }
  }

  /// Represents a cURL request with all necessary information
  #[ derive( Debug, Clone ) ]
  pub struct CurlRequest
  {
    /// HTTP method
    pub method : String,
    /// Request URL
    pub url : String,
    /// Request headers
    pub headers : Vec< (String, String) >,
    /// Request body (optional)
    pub body : Option< String >,
  }

  impl CurlRequest
  {
    /// Convert to a cURL command string
    #[ inline ]
    #[ must_use ]
    pub fn to_curl_command( &self ) -> String
    {
      let mut command = format!( "curl -X {} '{}'", self.method, self.url );

      for (key, value) in &self.headers
      {
        command.push_str( " -H '" );
        command.push_str( key );
        command.push_str( ": " );
        command.push_str( value );
        command.push( '\'' );
      }

      if let Some( body ) = &self.body
      {
        let escaped_body = body.replace( '\"', "\\\"" );
        command.push_str( " -d '" );
        command.push_str( &escaped_body );
        command.push( '\'' );
      }

      command
    }

    /// Convert to a cURL command with formatting options
    #[ inline ]
    #[ must_use ]
    pub fn to_curl_command_with_options( &self, options : &CurlFormatOptions ) -> String
    {
      let mut command = self.to_curl_command();

      if options.formatting.include_verbose
      {
        command.push_str( " --verbose" );
      }

      if options.formatting.include_silent
      {
        command.push_str( " --silent" );
      }

      if options.connection.include_insecure
      {
        command.push_str( " --insecure" );
      }

      if let Some( timeout ) = options.timeout
      {
        command.push_str( " --max-time " );
        command.push_str( &timeout.to_string() );
      }

      if options.formatting.pretty_print
      {
        // Format with line breaks for readability
        command = command.replace( " -H", " \\\n  -H" );
        command = command.replace( " -d", " \\\n  -d" );
      }

      command
    }

    /// Create a safe version with redacted sensitive headers
    #[ inline ]
    #[ must_use ]
    pub fn to_curl_command_safe( &self ) -> String
    {
      let mut safe_request = self.clone();

      // Redact sensitive headers
      for (key, value) in &mut safe_request.headers
      {
        if key.to_lowercase().contains( "authorization" ) ||
           key.to_lowercase().contains( "api-key" ) ||
           key.to_lowercase().contains( "token" )
        {
          *value = "[REDACTED]".to_string();
        }
      }

      safe_request.to_curl_command()
    }
  }

  /// Options for formatting cURL commands
  ///
  /// Groups related formatting settings to avoid excessive boolean parameters.
  /// Uses structured configuration pattern following best practices.
  #[ derive( Debug, Clone ) ]
  pub struct CurlFormatOptions
  {
    /// Output formatting options
    pub formatting : CurlFormattingOptions,
    /// Security and connection options
    pub connection : CurlConnectionOptions,
    /// Optional timeout in seconds
    pub timeout : Option< u32 >,
  }

  /// Formatting-related options for cURL commands
  #[ derive( Debug, Clone ) ]
  pub struct CurlFormattingOptions
  {
    /// Whether to format with line breaks for readability
    pub pretty_print : bool,
    /// Whether to include --verbose flag
    pub include_verbose : bool,
    /// Whether to include --silent flag
    pub include_silent : bool,
  }

  /// Connection and security options for cURL commands
  #[ derive( Debug, Clone ) ]
  pub struct CurlConnectionOptions
  {
    /// Whether to include --insecure flag
    pub include_insecure : bool,
  }

  impl Default for CurlFormatOptions
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        formatting : CurlFormattingOptions::default(),
        connection : CurlConnectionOptions::default(),
        timeout : None,
      }
    }
  }

  impl Default for CurlFormattingOptions
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        pretty_print : false,
        include_verbose : false,
        include_silent : false,
      }
    }
  }

  impl Default for CurlConnectionOptions
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        include_insecure : false,
      }
    }
  }

  /// Trait for API clients to support cURL generation
  pub trait CurlGeneration
  {
    /// The request type that can be serialized
    type Request : Serialize;
    /// The error type returned by cURL generation methods
    type Error;

    /// Generate a cURL command for a request
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be serialized to a cURL command format.
    fn to_curl( &self, request : &Self::Request ) -> core::result::Result< String, Self::Error >;

    /// Generate a safe cURL command with redacted sensitive information
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying `to_curl` method fails.
    #[ inline ]
    fn to_curl_safe( &self, request : &Self::Request ) -> core::result::Result< String, Self::Error >
    {
      // Default implementation delegates to regular to_curl and then redacts
      self.to_curl( request )
        .map( |cmd| redact_sensitive_info( &cmd ) )
    }

    /// Generate a cURL command with custom headers
    ///
    /// # Errors
    ///
    /// Returns an error if the request cannot be serialized or if header processing fails.
    fn to_curl_with_headers( &self, request : &Self::Request, headers : &HashMap<  String, String  > ) -> core::result::Result< String, Self::Error >;
  }

  /// Helper function to redact sensitive information from cURL commands
  fn redact_sensitive_info( curl_command : &str ) -> String
  {
    let mut redacted = curl_command.to_string();

    // Redact Bearer tokens
    if let Some( start ) = redacted.find( "Bearer " )
    {
      if let Some( end ) = redacted[start + 7..].find( '\'' )
      {
        let token_end = start + 7 + end;
        redacted.replace_range( start + 7..token_end, "[REDACTED]" );
      }
    }

    // Redact API keys
    let api_key_patterns = [ "sk-", "xoxb-", "xoxp-" ];
    for pattern in &api_key_patterns
    {
      if let Some( start ) = redacted.find( pattern )
      {
        if let Some( end ) = redacted[start..].find( '\'' )
        {
          redacted.replace_range( start..start + end, "[REDACTED]" );
        }
      }
    }

    redacted
  }

  /// Helper function to build a cURL request from HTTP request components
  #[ inline ]
  #[ must_use ]
  pub fn build_curl_request(
    method : &str,
    url : &str,
    headers : &[(String, String)],
    body : Option< &str >
  ) -> CurlRequest
  {
    CurlRequest
    {
      method : method.to_string(),
      url : url.to_string(),
      headers : headers.to_vec(),
      body : body.map( str::to_string ),
    }
  }

  /// Helper function to serialize a request to JSON
  ///
  /// # Errors
  ///
  /// Returns an error if the request cannot be serialized to JSON format.
  #[ inline ]
  pub fn serialize_request_to_json< T : Serialize >( request : &T ) -> Result< String >
  {
    serde_json ::to_string( request )
      .map_err( |e| error_tools::Error::from( OpenAIError::Internal( format!( "Failed to serialize request : {e}" ) ) ) )
  }

  impl Default for CurlGenerator
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  impl Default for CurlRequestBuilder
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }
}

crate ::mod_interface!
{
  exposed use
  {
    CurlGenerator,
    CurlRequestBuilder,
    CurlRequest,
    CurlFormatOptions,
    CurlFormattingOptions,
    CurlConnectionOptions,
    CurlGeneration,
    build_curl_request,
    serialize_request_to_json,
  };
}