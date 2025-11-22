// src/environment/mod.rs
//! This module defines the environment configuration for the `OpenAI` API client.
//! It includes traits and concrete implementations for managing API keys,
//! organization IDs, project IDs, and base URLs.

/// Define a private namespace for all its items.
mod private
{
  // Use crate root for base access
  use crate::error::OpenAIError;
  use crate::secret::Secret;
  use crate::diagnostics::DiagnosticsConfig;

  // External crates
  use reqwest::header;
  use url::Url;
  use secrecy::ExposeSecret;
  use error_tools::untyped::Result;

  /// Official `OpenAI` API base URL.
  const OPENAI_BASE_URL : &str = "https://api.openai.com/v1/";
  /// Official `OpenAI` Realtime API base URL.
  const OPENAI_REALTIME_BASE_URL : &str = "https://api.openai.com/v1/realtime/";
  /// `OpenAI` Beta header for API requests.
  pub const OPENAI_BETA_HEADER : &str = "OpenAI-Beta";

  /// Recommended configuration values for `OpenAI` API client following "Thin Client, Rich API" principles.
  ///
  /// This structure provides `OpenAI`-specific recommended values without making them automatic defaults.
  /// Developers must explicitly choose to use these values, maintaining transparency and control.
  #[ derive( Debug ) ]
  pub struct OpenAIRecommended;

  impl OpenAIRecommended
  {
    /// Returns the official `OpenAI` API base URL.
    ///
    /// Following the governing principle : this provides information for explicit developer choice
    /// rather than being an automatic default.
    #[ inline ]
    #[ must_use ]
    pub fn base_url() -> &'static str
    {
      OPENAI_BASE_URL
    }

    /// Returns the official `OpenAI` Realtime API base URL.
    ///
    /// Following the governing principle : this provides information for explicit developer choice
    /// rather than being an automatic default.
    #[ inline ]
    #[ must_use ]
    pub fn realtime_base_url() -> &'static str
    {
      OPENAI_REALTIME_BASE_URL
    }
  }

  /// A trait defining the interface for environment-related information.
  pub trait EnvironmentInterface : Send + Sync + 'static
  {
    /// Returns the API key.
    fn api_key( &self ) -> &crate::secret::Secret;
    /// Returns the organization ID, if available.
    fn organization_id( &self ) -> Option< &str >;
    /// Returns the project ID, if available.
    fn project_id( &self ) -> Option< &str >;
  }

  /// A trait defining the interface for `OpenAI` environment configuration.
  pub trait OpenaiEnvironment : Send + Sync + 'static
  {
    /// Returns the API key.
    fn api_key( &self ) -> &Secret;
    /// Returns the organization ID, if available.
    fn organization_id( &self ) -> Option< &str >;
    /// Returns the project ID, if available.
    fn project_id( &self ) -> Option< &str >;
    /// Returns the base URL for the `OpenAI` API.
    fn base_url( &self ) -> &Url;
    /// Returns the base URL for the `OpenAI` Realtime API.
    fn realtime_base_url( &self ) -> &Url;
    /// Returns the diagnostics configuration, if available.
    fn diagnostics_config( &self ) -> Option< &DiagnosticsConfig >;
    /// Returns the HTTP headers for the `OpenAI` API.
    ///
    /// # Errors
    /// Returns `OpenAIError::InvalidArgument` if headers contain invalid values.
    fn headers( &self ) -> Result< header::HeaderMap >;
    /// Joins a path to the base URL.
    ///
    /// # Errors
    /// Returns `OpenAIError::Internal` if URL joining fails.
    fn join_base_url( &self, path : &str ) -> Result< Url >;
    /// Joins a path to the realtime base URL.
    ///
    /// # Errors
    /// Returns `OpenAIError::Internal` if URL joining fails.
    fn join_realtime_base_url( &self, path : &str ) -> Result< Url >;
  }

  /// Concrete implementation of `OpenaiEnvironment`.
  #[ derive( Debug, Clone ) ]
  #[ non_exhaustive ]
  pub struct OpenaiEnvironmentImpl
  {
    /// The API key for authentication.
    pub api_key : Secret,
    /// The base URL for the `OpenAI` API.
    pub base_url : Url,
    /// The organization ID, if applicable.
    pub organization_id : Option< String >,
    /// The project ID, if applicable.
    pub project_id : Option< String >,
    /// The base URL for the `OpenAI` Realtime API.
    pub realtime_base_url : Url,
    /// Optional diagnostics configuration.
    pub diagnostics_config : Option< DiagnosticsConfig >,
  }

  impl OpenaiEnvironmentImpl
  {
    /// Creates a new `OpenaiEnvironmentImpl` instance.
    ///
    /// # Arguments
    /// - `api_key`: The API key for authentication.
    /// - `organization_id`: Optional organization ID.
    /// - `project_id`: Optional project ID.
    /// - `base_url`: Base URL for the `OpenAI` API. Use `OpenAIRecommended::base_url()` for official `OpenAI` API.
    /// - `realtime_base_url`: Base URL for the `OpenAI` Realtime API. Use `OpenAIRecommended::realtime_base_url()` for official `OpenAI` Realtime API.
    ///
    /// # Errors
    /// Returns `OpenAIError::InvalidArgument` if any provided URL is invalid.
    #[ inline ]
    #[ allow( clippy::needless_pass_by_value ) ]
    pub fn build
    (
      api_key : Secret,
      organization_id : Option< String >,
      project_id : Option< String >,
      base_url : String,
      realtime_base_url : String,
    ) -> Result< Self >
    {
      let base_url = Url::parse( &base_url ).map_err( | e | error_tools::Error::from( OpenAIError::InvalidArgument( format!( "Invalid base URL: {e}" ) ) ) )?;

      let realtime_base_url = Url::parse( &realtime_base_url ).map_err( | e | error_tools::Error::from( OpenAIError::InvalidArgument( format!( "Invalid realtime base URL: {e}" ) ) ) )?;

      Ok( Self
      {
        api_key,
        base_url,
        organization_id,
        project_id,
        realtime_base_url,
        diagnostics_config : None,
      })
    }

    /// Creates a new `OpenaiEnvironmentImpl` instance with diagnostics configuration.
    ///
    /// # Arguments
    /// - `api_key`: The API key for authentication.
    /// - `organization_id`: Optional organization ID.
    /// - `project_id`: Optional project ID.
    /// - `base_url`: Base URL for the `OpenAI` API. Use `OpenAIRecommended::base_url()` for official `OpenAI` API.
    /// - `realtime_base_url`: Base URL for the `OpenAI` Realtime API. Use `OpenAIRecommended::realtime_base_url()` for official `OpenAI` Realtime API.
    /// - `diagnostics_config`: Optional diagnostics configuration.
    ///
    /// # Errors
    /// Returns `OpenAIError::InvalidArgument` if any provided URL is invalid.
    #[ inline ]
    #[ allow( clippy::needless_pass_by_value ) ]
    pub fn build_with_diagnostics
    (
      api_key : Secret,
      organization_id : Option< String >,
      project_id : Option< String >,
      base_url : String,
      realtime_base_url : String,
      diagnostics_config : Option< DiagnosticsConfig >,
    ) -> Result< Self >
    {
      let base_url = Url::parse( &base_url ).map_err( | e | error_tools::Error::from( OpenAIError::InvalidArgument( format!( "Invalid base URL: {e}" ) ) ) )?;

      let realtime_base_url = Url::parse( &realtime_base_url ).map_err( | e | error_tools::Error::from( OpenAIError::InvalidArgument( format!( "Invalid realtime base URL: {e}" ) ) ) )?;

      Ok( Self
      {
        api_key,
        base_url,
        organization_id,
        project_id,
        realtime_base_url,
        diagnostics_config,
      })
    }
  }

  impl OpenaiEnvironment for OpenaiEnvironmentImpl
  {
    #[ inline ]
    fn api_key( &self ) -> &Secret
    {
      &self.api_key
    }

    #[ inline ]
    fn organization_id( &self ) -> Option< &str >
    {
      self.organization_id.as_deref()
    }

    #[ inline ]
    fn project_id( &self ) -> Option< &str >
    {
      self.project_id.as_deref()
    }

    #[ inline ]
    fn base_url( &self ) -> &Url
    {
      &self.base_url
    }

    #[ inline ]
    fn realtime_base_url( &self ) -> &Url
    {
      &self.realtime_base_url
    }

    #[ inline ]
    fn diagnostics_config( &self ) -> Option< &DiagnosticsConfig >
    {
      self.diagnostics_config.as_ref()
    }

    #[ inline ]
    fn headers( &self ) -> Result< header::HeaderMap >
    {
      let mut headers = header::HeaderMap::new();
      let api_key = self.api_key.expose_secret();
      let auth_value = header::HeaderValue::from_str( &format!( "Bearer {api_key}" ) )
      .map_err( | error | error_tools::Error::from( OpenAIError::InvalidArgument( format!( "Invalid API key : {error}" ) ) ) )?;
      headers.insert( header::AUTHORIZATION, auth_value );

      if let Some( org_id ) = OpenaiEnvironment::organization_id( self )
      {
        let org_value = header::HeaderValue::from_str( org_id )
        .map_err( | error | error_tools::Error::from( OpenAIError::InvalidArgument( format!( "Invalid Organization ID: {error}" ) ) ) )?;
        headers.insert( header::HeaderName::from_static( "openai-organization" ), org_value );
      }

      if let Some( project_id ) = OpenaiEnvironment::project_id( self )
      {
        let project_value = header::HeaderValue::from_str( project_id )
        .map_err( | error | error_tools::Error::from( OpenAIError::InvalidArgument( format!( "Invalid Project ID: {error}" ) ) ) )?;
        headers.insert( header::HeaderName::from_static( "openai-project" ), project_value );
      }
      Ok( headers )
    }

    #[ inline ]
    fn join_base_url( &self, path : &str ) -> Result< Url >
    {
      self.base_url.join( path ).map_err( | e | error_tools::Error::from( OpenAIError::Internal( format!( "Failed to join base URL: {e}" ) ) ) )
    }

    #[ inline ]
    fn join_realtime_base_url( &self, path : &str ) -> Result< Url >
    {
      self.realtime_base_url.join( path ).map_err( | e | error_tools::Error::from( OpenAIError::Internal( format!( "Failed to join realtime URL: {e}" ) ) ) )
    }
  }

  impl EnvironmentInterface for OpenaiEnvironmentImpl
  {
    #[ inline ]
    fn api_key( &self ) -> &Secret
    {
      &self.api_key
    }

    #[ inline ]
    fn organization_id( &self ) -> Option< &str >
    {
      self.organization_id.as_deref()
    }

    #[ inline ]
    fn project_id( &self ) -> Option< &str >
    {
      self.project_id.as_deref()
    }
  }
}

crate ::mod_interface!
{
  // Expose the trait and its concrete implementation
  exposed use
  {
    EnvironmentInterface,
    OpenaiEnvironment,
    OpenaiEnvironmentImpl,
    OpenAIRecommended,
    OPENAI_BETA_HEADER,
  };
}