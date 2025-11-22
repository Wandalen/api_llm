mod private
{
  use crate::error::{ XaiError, Result };
  use crate::secret::Secret;
  use url::Url;
  use reqwest::header;
  use std::time::Duration;

  /// Default base URL for XAI API.
  ///
  /// All API requests will be made to endpoints under this base URL.
  /// Trailing slash is required for proper URL joining with paths starting with `/`.
  pub const DEFAULT_BASE_URL : &str = "https://api.x.ai/v1/";

  /// Default request timeout in seconds.
  ///
  /// Requests exceeding this duration will be cancelled with a timeout error.
  pub const DEFAULT_TIMEOUT_SECS : u64 = 30;

  /// Environment configuration trait for XAI API client.
  ///
  /// This trait abstracts the environment configuration, allowing for different
  /// implementations (production, testing, custom configurations).
  ///
  /// # Required Methods
  ///
  /// - `api_key()` - Returns the API authentication key
  /// - `base_url()` - Returns the base URL for API requests
  /// - `timeout()` - Returns the request timeout duration
  /// - `headers()` - Constructs the HTTP headers for requests
  ///
  /// # Trait Bounds
  ///
  /// Implementations must be `Send + Sync + 'static` to support async operations
  /// and thread safety.
  pub trait XaiEnvironment : Send + Sync + 'static
  {
    /// Returns the API authentication key.
    fn api_key( &self ) -> &Secret;

    /// Returns the base URL for API requests.
    fn base_url( &self ) -> &Url;

    /// Returns the request timeout duration.
    fn timeout( &self ) -> Duration;

    /// Constructs HTTP headers for API requests.
    ///
    /// Default implementation includes:
    /// - Authorization header with Bearer token
    /// - Content-Type : application/json
    ///
    /// # Errors
    ///
    /// Returns `XaiError::Http` if header construction fails.
    fn headers( &self ) -> Result< header::HeaderMap >
    {
      let mut headers = header::HeaderMap::new();

      // Authorization : Bearer xai-...
      let auth_value = format!( "Bearer {secret}", secret = self.api_key().expose_secret() );
      headers.insert(
        header::AUTHORIZATION,
        auth_value.parse()
          .map_err( |e| XaiError::Http( format!( "Invalid authorization header : {e}" ) ) )?
      );

      // Content-Type : application/json
      headers.insert(
        header::CONTENT_TYPE,
        "application/json".parse()
          .map_err( |e| XaiError::Http( format!( "Invalid content-type header : {e}" ) ) )?
      );

      Ok( headers )
    }
  }

  /// Default implementation of XAI environment configuration.
  ///
  /// Provides a standard environment setup with configurable base URL and timeout.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use api_xai::{ XaiEnvironmentImpl, Secret };
  ///
  /// // Basic setup with defaults
  /// let secret = Secret::load_with_fallbacks( "XAI_API_KEY" )?;
  /// let env = XaiEnvironmentImpl::new( secret )?;
  /// # Ok::<(), Box< dyn std::error::Error > >(())
  /// ```
  ///
  /// ```no_run
  /// use api_xai::{ XaiEnvironmentImpl, Secret };
  ///
  /// // Custom configuration
  /// let secret = Secret::load_with_fallbacks( "XAI_API_KEY" )?;
  /// let env = XaiEnvironmentImpl::new( secret )?
  ///   .with_timeout( std::time::Duration::from_secs( 60 ) );
  /// # Ok::<(), Box< dyn std::error::Error > >(())
  /// ```
  #[ derive( Debug, Clone ) ]
  pub struct XaiEnvironmentImpl
  {
    api_key : Secret,
    base_url : Url,
    timeout : Duration,
  }

  impl XaiEnvironmentImpl
  {
    /// Creates a new environment with default configuration.
    ///
    /// # Arguments
    ///
    /// * `api_key` - XAI API authentication key
    ///
    /// # Default Values
    ///
    /// - Base URL: `https://api.x.ai/v1`
    /// - Timeout : 30 seconds
    ///
    /// # Errors
    ///
    /// Returns `XaiError::UrlParse` if the default base URL is invalid
    /// (this should never happen in practice).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use api_xai::{ XaiEnvironmentImpl, Secret };
    ///
    /// let secret = Secret::load_with_fallbacks( "XAI_API_KEY" )?;
    /// let env = XaiEnvironmentImpl::new( secret )?;
    /// # Ok::<(), Box< dyn std::error::Error > >(())
    /// ```
    pub fn new( api_key : Secret ) -> Result< Self >
    {
      Ok( Self
      {
        api_key,
        base_url : Url::parse( DEFAULT_BASE_URL )?,
        timeout : Duration::from_secs( DEFAULT_TIMEOUT_SECS ),
      } )
    }

    /// Sets a custom base URL.
    ///
    /// Use this to configure a different API endpoint (e.g., for testing
    /// or proxy configurations).
    ///
    /// # Arguments
    ///
    /// * `base_url` - Custom base URL for API requests
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use api_xai::{ XaiEnvironmentImpl, Secret };
    /// use url::Url;
    ///
    /// let secret = Secret::load_with_fallbacks( "XAI_API_KEY" )?;
    /// let custom_url = Url::parse( "https://custom.api.endpoint/v1" )?;
    ///
    /// let env = XaiEnvironmentImpl::new( secret )?
    ///   .with_base_url( custom_url );
    /// # Ok::<(), Box< dyn std::error::Error > >(())
    /// ```
    #[ must_use ]
    pub fn with_base_url( mut self, base_url : Url ) -> Self
    {
      self.base_url = base_url;
      self
    }

    /// Sets a custom timeout duration.
    ///
    /// Use this to configure longer timeouts for slow connections or
    /// shorter timeouts for fast-fail behavior.
    ///
    /// # Arguments
    ///
    /// * `timeout` - Custom timeout duration
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use api_xai::{ XaiEnvironmentImpl, Secret };
    /// use std::time::Duration;
    ///
    /// let secret = Secret::load_with_fallbacks( "XAI_API_KEY" )?;
    ///
    /// // 60 second timeout
    /// let env = XaiEnvironmentImpl::new( secret )?
    ///   .with_timeout( Duration::from_secs( 60 ) );
    /// # Ok::<(), Box< dyn std::error::Error > >(())
    /// ```
    #[ must_use ]
    pub fn with_timeout( mut self, timeout : Duration ) -> Self
    {
      self.timeout = timeout;
      self
    }
  }

  impl XaiEnvironment for XaiEnvironmentImpl
  {
    fn api_key( &self ) -> &Secret
    {
      &self.api_key
    }

    fn base_url( &self ) -> &Url
    {
      &self.base_url
    }

    fn timeout( &self ) -> Duration
    {
      self.timeout
    }
  }
}

crate::mod_interface!
{
  exposed use
  {
    DEFAULT_BASE_URL,
    DEFAULT_TIMEOUT_SECS,
    XaiEnvironment,
    XaiEnvironmentImpl,
  };
}
