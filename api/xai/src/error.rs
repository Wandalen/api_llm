mod private
{
  use error_tools::dependency::thiserror;

  /// Error types for XAI API operations.
  ///
  /// This enum covers all possible error conditions when interacting with the XAI API,
  /// including network errors, API errors, serialization issues, and configuration problems.
  ///
  /// # Error Categories
  ///
  /// - **API Errors**: Errors returned by the XAI API itself
  /// - **HTTP Errors**: Generic HTTP-level failures
  /// - **Network Errors**: Connection and transport issues
  /// - **Timeout Errors**: Request timeout conditions
  /// - **Stream Errors**: SSE streaming failures
  /// - **Rate Limit Errors**: API rate limiting
  /// - **Serialization Errors**: JSON parsing failures
  /// - **Authentication Errors**: Invalid API key or credentials
  /// - **Environment Errors**: Configuration and setup issues
  #[ derive( Debug, Clone, PartialEq, thiserror::Error ) ]
  #[ non_exhaustive ]
  pub enum XaiError
  {
    /// API error returned by XAI service.
    ///
    /// Contains structured error information including message, error code,
    /// and error type from the API response.
    #[ error( "API Error : {message} (code : {code:?}, type : {error_type:?})" ) ]
    Api
    {
      /// Human-readable error message
      message : String,
      /// Error code (e.g., `"invalid_request_error"`)
      code : Option< String >,
      /// Error type classification
      error_type : Option< String >,
    },

    /// HTTP-level error.
    ///
    /// Generic HTTP failures that dont fit into more specific categories.
    #[ error( "HTTP Error : {0}" ) ]
    Http( String ),

    /// Network connectivity error.
    ///
    /// Connection failures, DNS resolution errors, or other transport-level issues.
    #[ error( "Network Error : {0}" ) ]
    Network( String ),

    /// Request timeout error.
    ///
    /// The request exceeded the configured timeout duration.
    #[ error( "Timeout Error : {0}" ) ]
    Timeout( String ),

    /// SSE streaming error.
    ///
    /// Failures during Server-Sent Events streaming, including parse errors
    /// and connection interruptions.
    #[ error( "Stream Error : {0}" ) ]
    Stream( String ),

    /// Rate limit exceeded error.
    ///
    /// The API returned a 429 status indicating rate limiting is active.
    /// Client should implement exponential backoff.
    #[ error( "Rate Limit Error : {0}" ) ]
    RateLimit( String ),

    /// Serialization or deserialization error.
    ///
    /// JSON parsing failures or serialization issues with request/response data.
    #[ error( "Serialization Error : {0}" ) ]
    Serialization( String ),

    /// Invalid API key error.
    ///
    /// The provided API key is malformed or doesn't meet validation requirements.
    #[ error( "Invalid API Key : {0}" ) ]
    InvalidApiKey( String ),

    /// Environment configuration error.
    ///
    /// Issues with environment setup, missing variables, or invalid configuration.
    #[ error( "Environment Error : {0}" ) ]
    Environment( String ),

    /// URL parsing error.
    ///
    /// Invalid URL format in configuration or endpoint construction.
    #[ error( "URL Parse Error : {0}" ) ]
    UrlParse( String ),

    /// Circuit breaker open error.
    ///
    /// The circuit breaker is in open state, preventing requests to a failing endpoint.
    #[ cfg( feature = "circuit_breaker" ) ]
    #[ error( "Circuit Breaker Open : {0}" ) ]
    CircuitBreakerOpen( String ),

    /// Invalid model error.
    ///
    /// The specified model name is not recognized or supported.
    #[ error( "Invalid Model : {0}" ) ]
    InvalidModel( String ),

    /// Invalid parameter error.
    ///
    /// Request parameter validation failed.
    #[ error( "Invalid Parameter : {0}" ) ]
    InvalidParameter( String ),

    /// Generic API error.
    ///
    /// Catch-all for API-related errors that dont fit other categories.
    #[ error( "API Error : {0}" ) ]
    ApiError( String ),
  }

  /// Result type alias using `error_tools`.
  ///
  /// This is the standard Result type used throughout the `api_xai` crate.
  /// All fallible operations return this type.
  pub type Result< T > = error_tools::untyped::Result< T >;

  // Conversion from reqwest errors
  impl From< reqwest::Error > for XaiError
  {
    fn from( error : reqwest::Error ) -> Self
    {
      if error.is_timeout()
      {
        XaiError::Timeout( error.to_string() )
      }
      else if error.is_connect() || error.is_request()
      {
        XaiError::Network( error.to_string() )
      }
      else if error.is_status()
      {
        if let Some( status ) = error.status()
        {
          if status.as_u16() == 429
          {
            return XaiError::RateLimit( format!( "Rate limit exceeded : {status}" ) );
          }
        }
        XaiError::Http( error.to_string() )
      }
      else
      {
        XaiError::Http( error.to_string() )
      }
    }
  }

  // Conversion from serde_json errors
  impl From< serde_json::Error > for XaiError
  {
    fn from( error : serde_json::Error ) -> Self
    {
      XaiError::Serialization( error.to_string() )
    }
  }

  // Conversion from URL parse errors
  impl From< url::ParseError > for XaiError
  {
    fn from( error : url::ParseError ) -> Self
    {
      XaiError::UrlParse( error.to_string() )
    }
  }

  // Conversion from reqwest header value errors
  impl From< reqwest::header::InvalidHeaderValue > for XaiError
  {
    fn from( error : reqwest::header::InvalidHeaderValue ) -> Self
    {
      XaiError::Http( format!( "Invalid header value : {error}" ) )
    }
  }
}

crate::mod_interface!
{
  exposed use
  {
    XaiError,
    Result,
  };
}
