mod private
{
  use error_tools::dependency::thiserror;
  use serde::{ Deserialize, Serialize };

  /// Errors that can occur when using the Gemini API client.
  #[ derive( Debug, thiserror::Error, Clone, PartialEq ) ]
  pub enum Error
  {
      /// I/O error occurred.
      #[ error( "IO error : {0}" ) ]
      Io( String ),

      /// Error occurred while building the request.
      #[ error( "Request building error : {0}" ) ]
      RequestBuilding( String ),

      /// API returned an error response.
      #[ error( "API error : {0}" ) ]
      ApiError( String ),

      /// Authentication failed.
      #[ error( "Authentication error : {0}" ) ]
      AuthenticationError( String ),

      /// Rate limit has been exceeded.
      #[ error( "Rate limit exceeded : {0}" ) ]
      RateLimitError( String ),

      /// Invalid argument provided.
      #[ error( "Invalid argument : {0}" ) ]
      InvalidArgument( String ),

      /// Server returned an error.
      #[ error( "Server error : {0}" ) ]
      ServerError( String ),

      /// Failed to serialize request data.
      #[ error( "Serialization error : {0}" ) ]
      SerializationError( String ),

      /// Failed to deserialize response data.
      #[ error( "Deserialization error : {0}" ) ]
      DeserializationError( String ),

      /// Network-related error occurred.
      #[ error( "Network error : {0}" ) ]
      NetworkError( String ),

      /// Unknown error occurred.
      #[ error( "Unknown error : {0}" ) ]
      Unknown( String ),

      /// Feature not yet implemented.
      #[ error( "Not implemented : {0}" ) ]
      NotImplemented( String ),

      /// Configuration error for invalid settings.
      #[ error( "Configuration error : {0}" ) ]
      ConfigurationError( String ),

      /// Timeout error when operation takes too long.
      #[ error( "Timeout error : {0}" ) ]
      TimeoutError( String ),

      /// Resource not found error.
      #[ error( "Resource not found : {0}" ) ]
      NotFound( String ),

      /// Health check error.
      #[ error( "Health check error : {0}" ) ]
      Health( String ),

      /// Validation error for invalid input.
      #[ error( "Validation error : {message}" ) ]
      ValidationError 
      { 
        /// Validation error message
        message : String 
      },

      /// Batch processing error with partial results.
      #[ error( "Batch processing error : {successful} successful, {failed} failed - {message}" ) ]
      BatchProcessingError 
      { 
        /// Number of successful operations
        successful : usize, 
        /// Number of failed operations
        failed : usize, 
        /// Error message describing the issue
        message : String 
      },

      /// Circuit breaker is currently open, preventing requests.
      #[ cfg( feature = "circuit_breaker" ) ]
      #[ error( "Circuit breaker is open : {0}" ) ]
      CircuitBreakerOpen( String ),

      /// Cache operation failed.
      #[ cfg( feature = "caching" ) ]
      #[ error( "Cache error : {0}" ) ]
      CacheError( String ),

      /// Rate limiting is active, request was throttled.
      #[ cfg( feature = "rate_limiting" ) ]
      #[ error( "Rate limited : {0}" ) ]
      RateLimited( String ),

      /// Quota exceeded error for enterprise quota management.
      #[ cfg( feature = "enterprise_quota" ) ]
      #[ error( "Quota exceeded : {0}" ) ]
      QuotaExceeded( String ),
  }

  impl From< std::io::Error > for Error
  {
    #[ inline ]
    fn from( err : std::io::Error ) -> Self
    {
        Error::Io( err.to_string() )
    }
  }

  impl From< serde_json::Error > for Error
  {
    #[ inline ]
    fn from( err : serde_json::Error ) -> Self
    {
        Error::SerializationError( err.to_string() )
    }
  }

  impl From< reqwest::Error > for Error
  {
    #[ inline ]
    fn from( err : reqwest::Error ) -> Self
    {
        if err.is_timeout()
        {
          Error::TimeoutError( format!( "Request timeout : {err}" ) )
        }
        else if err.is_connect()
        {
          Error::NetworkError( format!( "Connection error : {err}" ) )
        }
        else if err.is_request()
        {
          Error::RequestBuilding( format!( "Request error : {err}" ) )
        }
        else if err.status() == Some( reqwest::StatusCode::NOT_FOUND )
        {
          Error::NotFound( format!( "Resource not found : {err}" ) )
        }
        else if err.status() == Some( reqwest::StatusCode::TOO_MANY_REQUESTS )
        {
          Error::RateLimitError( format!( "Rate limit exceeded : {err}" ) )
        }
        else
        {
          Error::NetworkError( err.to_string() )
        }
    }
  }

  /// API error response structure.
  #[ derive( Debug, Deserialize, Serialize ) ]
  #[ serde( rename_all = "camelCase" ) ]
  pub struct ApiErrorResponse
  {
      /// Error details.
      pub error : ApiErrorDetails,
  }

  /// Details about an API error.
  #[ derive( Debug, Deserialize, Serialize ) ]
  #[ serde( rename_all = "camelCase" ) ]
  pub struct ApiErrorDetails
  {
      /// Error code.
      pub code : i32,
      /// Error message.
      pub message : String,
      /// Optional status string.
      pub status : Option< String >,
  }
}

::mod_interface::mod_interface!
{
  exposed use private::Error;
  exposed use private::ApiErrorResponse;
  exposed use private::ApiErrorDetails;
}