// src/error.rs
//! This module defines the error types for the `OpenAI` API client.
//! It includes a comprehensive `OpenAIError` enum that covers various
//! error scenarios, such as API errors, network issues, and serialization failures.

/// Define a private namespace for all its items.
mod private
{
  // Use crate root for base access
  // Removed : use crate::components::common::ResponseError;

  // External crates
  use serde_json;
  use core::fmt;
  use backoff; // Import backoff
  use error_tools::dependency::thiserror; // Add thiserror via error_tools

  /// Represents an error returned by the `OpenAI` API.
  /// Corresponds to the `Error` schema in the `OpenAPI` spec.
  ///
  /// # Used By
  /// - `ResponseObject`
  #[ derive( Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize ) ]
  #[ non_exhaustive ]
  pub struct ApiError
  {
    /// The error code.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub code : Option< String >,
    /// The error message.
    pub message : String,
    /// The error parameter.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub param : Option< String >,
    /// The error type.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub r#type : Option< String >,
  }

  impl fmt::Display for ApiError
  {
    #[ inline ]
    fn fmt( &self, formatter : &mut fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      write!( formatter, "API Error : {}", self.message )?;
      if let Some( code ) = &self.code
      {
        write!( formatter, " (Code : {code})" )?;
      }
      if let Some( param ) = &self.param
      {
        write!( formatter, " (Param : {param})" )?;
      }
      if let Some( r#type ) = &self.r#type
      {
        write!( formatter, " (Type : {type})" )?;
      }
      core ::result::Result::Ok( () )
    }
  }

  /// A wrapper for `ApiError` that includes the HTTP status code.
  #[ derive( Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize ) ]
  #[ non_exhaustive ]
  pub struct ApiErrorWrap
  {
    /// The HTTP status code associated with the error.
    pub status_code : u16,
    /// The API error details.
    pub error : ApiError,
  }

  impl fmt::Display for ApiErrorWrap
  {
    #[ inline ]
    fn fmt( &self, formatter : &mut fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      write!( formatter, "HTTP Status : {}, API Error : {}", self.status_code, self.error )
    }
  }

  /// Represents all possible errors that can occur when interacting with the `OpenAI` API.
  #[ derive( Debug, Clone, PartialEq, thiserror::Error ) ]
  #[ non_exhaustive ]
  pub enum OpenAIError
  {
    /// An error returned by the `OpenAI` API.
    #[ error( "API Error : {0}" ) ]
    Api( ApiError ),
    /// An error that occurred during HTTP communication.
    #[ error( "HTTP Error : {0}" ) ]
    Http( String ),
    /// An error that occurred during WebSocket communication.
    #[ error( "WebSocket Error : {0}" ) ]
    Ws( String ), // Changed from tokio_tungstenite::tungstenite::Error to String
    /// An error that occurred during WebSocket communication due to an invalid message.
    #[ error( "WebSocket Invalid Message Error : {0}" ) ]
    WsInvalidMessage( String ),
    /// An error that occurred during serialization or deserialization.
    #[ error( "Internal Error : {0}" ) ]
    Internal( String ), // Added Internal variant
    /// An error indicating an invalid argument was provided.
    #[ error( "Invalid Argument Error : {0}" ) ]
    InvalidArgument( String ),
    /// An error indicating a missing required argument.
    #[ error( "Missing Argument Error : {0}" ) ]
    MissingArgument( String ),
    /// An error indicating a missing environment variable.
    #[ error( "Missing Environment Error : {0}" ) ]
    MissingEnvironment( String ),
    /// An error indicating a missing header.
    #[ error( "Missing Header Error : {0}" ) ]
    MissingHeader( String ),
    /// An error indicating a missing file.
    #[ error( "Missing File Error : {0}" ) ]
    MissingFile( String ),
    /// An error indicating a file operation failed.
    #[ error( "File Error : {0}" ) ]
    File( String ),
    /// An error indicating a network issue.
    #[ error( "Network Error : {0}" ) ]
    Network( String ),
    /// An error indicating a timeout.
    #[ error( "Timeout Error : {0}" ) ]
    Timeout( String ),
    /// An error related to streaming data.
    #[ error( "Stream Error : {0}" ) ]
    Stream( String ), // Added Stream variant
    /// An unknown error.
    #[ error( "Unknown Error : {0}" ) ]
    Unknown( String ),
    /// A rate limiting error.
    #[ error( "Rate Limit Error : {0}" ) ]
    RateLimit( String ),
  }



  // Display implementation now provided by thiserror::Error derive macro

  impl From< reqwest::Error > for OpenAIError
  {
    #[ inline ]
    fn from( error : reqwest::Error ) -> Self
    {
      if error.is_timeout()
      {
        OpenAIError::Timeout( error.to_string() )
      }
      else if error.is_connect() || error.is_request()
      {
        OpenAIError::Network( error.to_string() )
      }
      else if error.is_builder()
      {
        OpenAIError::Internal( format!( "HTTP client build error : {error}" ) )
      }
      else if error.is_status()
      {
        let status = error.status().unwrap_or_default();
        OpenAIError::Http( format!( "HTTP error with status {status}: {error}" ) )
      }
      else
      {
        OpenAIError::Http( error.to_string() )
      }
    }
  }

  impl From< serde_json::Error > for OpenAIError
  {
    #[ inline ]
    fn from( error : serde_json::Error ) -> Self
    {
      OpenAIError::Internal( format!( "JSON error : {error}" ) )
    }
  }

  impl From< backoff::Error< OpenAIError > > for OpenAIError
  {
    #[ inline ]
    fn from( error : backoff::Error< OpenAIError > ) -> Self
    {
      match error
      {
        backoff ::Error::Transient { err, .. } => err,
        backoff ::Error::Permanent( e ) => e,
      }
    }
  }

  /// Helper function to map `serde_json::Error` to `OpenAIError::Internal`.
  #[ must_use ]
  #[ inline ]
  pub fn map_deserialization_error( error : &serde_json::Error ) -> OpenAIError
  {
    OpenAIError::Internal( format!( "Deserialization error : {error}" ) )
  }

  /// Type alias for Results using `error_tools` pattern
  pub type Result< T > = error_tools::untyped::Result< T >;

} // end mod private

crate ::mod_interface!
{
  // Expose all structs defined in this module
  exposed use
  {
    ApiError,
    ApiErrorWrap,
    OpenAIError,
    Result,
    map_deserialization_error,
  };
}