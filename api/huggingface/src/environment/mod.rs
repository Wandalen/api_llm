//! Environment configuration and management for `HuggingFace` API.

mod private
{
use crate::{ error::{ Result, HuggingFaceError }, secret::Secret };
use reqwest::header::{ HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT };
use url::Url;

/// Trait defining the `HuggingFace` environment interface
pub trait HuggingFaceEnvironment
{
  /// Get the API key
  fn api_key( &self ) -> &Secret;
  
  /// Get the base URL for API requests
  fn base_url( &self ) -> &str;
  
  /// Get the user agent string
  fn user_agent( &self ) -> &str;
  
  /// Build the complete URL for a specific endpoint
  ///
  /// # Errors
  /// Returns error if URL construction fails
  fn endpoint_url( &self, path : &str ) -> Result< Url >;
}

/// Generic environment interface for header generation
pub trait EnvironmentInterface
{
  /// Generate HTTP headers for requests
  ///
  /// # Errors
  /// Returns error if header construction fails
  fn headers( &self ) -> Result< HeaderMap >;
}

/// Default implementation of `HuggingFace` environment
#[ derive( Debug, Clone ) ]
pub struct HuggingFaceEnvironmentImpl
{
  /// API key for authentication
  pub api_key : Secret,
  
  /// Base URL for API requests
  pub base_url : String,
  
  /// User agent for HTTP requests
  pub user_agent : String,
}

impl HuggingFaceEnvironmentImpl
{
  /// Create a new `HuggingFace` environment with recommended configuration.
  ///
  /// # Governing Principle Compliance
  ///
  /// This provides HuggingFace-recommended configuration without making it implicit.
  /// For explicit control, use `with_explicit_config()`.
  ///
  /// # Arguments
  /// - `api_key`: `HuggingFace` API key
  /// - `base_url`: Optional custom base URL (uses `HuggingFace` recommended if None)
  ///
  /// # Errors
  /// Returns error if the configuration is invalid
  #[ inline ]
  pub fn build(
  api_key : Secret,
  base_url : Option< String >
  ) -> Result< Self >
  {
  let base_url = base_url.unwrap_or_else( || Self::recommended_base_url().to_string() );
  let user_agent = Self::recommended_user_agent().to_string();

  Ok( Self
  {
      api_key,
      base_url,
      user_agent,
  })
  }

  /// Create environment with explicit configuration (no recommendations)
  ///
  /// # Governing Principle Compliance
  ///
  /// This requires explicit configuration for all values, providing full transparency
  /// and control over environment behavior.
  ///
  /// # Arguments
  /// - `api_key`: `HuggingFace` API key
  /// - `base_url`: Explicit base URL
  /// - `user_agent`: Explicit user agent string
  ///
  /// # Errors
  /// Returns error if the configuration is invalid
  #[ inline ]
  pub fn with_explicit_config(
  api_key : Secret,
  base_url : String,
  user_agent : String,
  ) -> Result< Self >
  {
  Ok( Self
  {
      api_key,
      base_url,
      user_agent,
  })
  }

  /// Get the HuggingFace-recommended base URL
  ///
  /// Updated to use the new Router API (OpenAI-compatible chat completions format)
  /// Note : Trailing slash is required for proper URL joining
  #[ inline ]
  #[ must_use ]
  pub fn recommended_base_url() -> &'static str
  {
  "https://router.huggingface.co/v1/"
  }

  /// Get the HuggingFace-recommended user agent string
  #[ inline ]
  #[ must_use ]
  pub fn recommended_user_agent() -> &'static str
  {
  "llm-tools-huggingface/0.2.0"
  }
  
  /// Create environment from environment variables
  ///
  /// # Errors
  /// Returns error if required environment variables are missing
  #[ inline ]
  pub fn from_env() -> Result< Self >
  {
  let api_key = Secret::load_from_env( "HUGGINGFACE_API_KEY" )
      .map_err( | e | HuggingFaceError::Authentication( e.to_string() ) )?;
  
  let base_url = std::env::var( "HUGGINGFACE_BASE_URL" ).ok();
  
  Self::build( api_key, base_url )
  }
}

impl HuggingFaceEnvironment for HuggingFaceEnvironmentImpl
{
  #[ inline ]
  fn api_key( &self ) -> &Secret
  {
  &self.api_key
  }
  
  #[ inline ]
  fn base_url( &self ) -> &str
  {
  &self.base_url
  }
  
  #[ inline ]
  fn user_agent( &self ) -> &str
  {
  &self.user_agent
  }
  
  #[ inline ]
  fn endpoint_url( &self, path : &str ) -> Result< Url >
  {
  let base = Url::parse( &self.base_url )
      .map_err( | e | HuggingFaceError::InvalidArgument( format!( "Invalid base URL: {e}" ) ) )?;
  
  base.join( path )
      .map_err( | e | HuggingFaceError::InvalidArgument( format!( "Invalid endpoint path : {e}" ) ) )
  }
}

impl EnvironmentInterface for HuggingFaceEnvironmentImpl
{
  #[ inline ]
  fn headers( &self ) -> Result< HeaderMap >
  {
  let mut headers = HeaderMap::new();
  
  // Add authorization header
  let auth_value = format!( "Bearer {}", self.api_key.expose_secret() );
  let auth_header = HeaderValue::from_str( &auth_value )
      .map_err( | e | HuggingFaceError::Authentication( format!( "Invalid API key format : {e}" ) ) )?;
  headers.insert( AUTHORIZATION, auth_header );
  
  // Add user agent
  let user_agent_header = HeaderValue::from_str( &self.user_agent )
      .map_err( | e | HuggingFaceError::InvalidArgument( format!( "Invalid user agent : {e}" ) ) )?;
  headers.insert( USER_AGENT, user_agent_header );
  
  Ok( headers )
  }
}

} // end mod private

crate::mod_interface!
{
  exposed use
  {
  private::HuggingFaceEnvironment,
  private::EnvironmentInterface,
  private::HuggingFaceEnvironmentImpl,
  };
}