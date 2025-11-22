mod private
{
  use crate::error::{ XaiError, Result };
  use secrecy::{ SecretString, ExposeSecret };
  use workspace_tools::workspace;
  use std::sync::atomic::{ AtomicUsize, Ordering };

  /// Global counter tracking the number of times secrets have been exposed.
  ///
  /// Used for security auditing and monitoring. Each call to `expose_secret()`
  /// increments this counter.
  static EXPOSURE_COUNTER : AtomicUsize = AtomicUsize::new( 0 );

  /// Secure wrapper for XAI API key.
  ///
  /// This type wraps a `SecretString` from the `secrecy` crate to prevent
  /// accidental exposure of the API key in logs or debug output.
  ///
  /// # Security Features
  ///
  /// - **Automatic Validation**: Enforces XAI API key format requirements
  /// - **Secure Storage**: Uses `SecretString` to prevent accidental leaks
  /// - **Exposure Auditing**: Tracks calls to `expose_secret()` via atomic counter
  /// - **Multi-tier Loading**: Fallback chain from env → workspace → config files
  ///
  /// # API Key Format
  ///
  /// XAI API keys must:
  /// - Start with `xai-` prefix
  /// - Be at least 10 characters long
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use api_xai::Secret;
  ///
  /// // Load from environment variable
  /// let secret = Secret::load_from_env( "XAI_API_KEY" )?;
  ///
  /// // Load with automatic fallback chain
  /// let secret = Secret::load_with_fallbacks( "XAI_API_KEY" )?;
  ///
  /// // Create from string
  /// let secret = Secret::new( "xai-your-key-here".to_string() )?;
  /// # Ok::<(), Box< dyn std::error::Error > >(())
  /// ```
  #[ derive( Debug, Clone ) ]
  pub struct Secret( SecretString );

  impl Secret
  {
    /// Creates a new `Secret` from a string, validating the format.
    ///
    /// # Validation
    ///
    /// - Must start with `xai-` prefix
    /// - Must be at least 10 characters long
    ///
    /// # Errors
    ///
    /// Returns `XaiError::InvalidApiKey` if validation fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use api_xai::Secret;
    ///
    /// let secret = Secret::new( "xai-1234567890".to_string() )?;
    /// # Ok::<(), Box< dyn std::error::Error > >(())
    /// ```
    pub fn new( key : String ) -> Result< Self >
    {
      Self::validate_format( &key )?;
      Ok( Self( SecretString::new( key.into_boxed_str() ) ) )
    }

    /// Loads the API key from an environment variable.
    ///
    /// # Arguments
    ///
    /// * `env_var` - Name of the environment variable (e.g., `"XAI_API_KEY"`)
    ///
    /// # Errors
    ///
    /// Returns `XaiError::Environment` if the environment variable is not set,
    /// or `XaiError::InvalidApiKey` if validation fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use api_xai::Secret;
    ///
    /// let secret = Secret::load_from_env( "XAI_API_KEY" )?;
    /// # Ok::<(), Box< dyn std::error::Error > >(())
    /// ```
    pub fn load_from_env( env_var : &str ) -> Result< Self >
    {
      let key = std::env::var( env_var )
        .map_err( |_| XaiError::Environment(
          format!( "Environment variable {env_var} not set" )
        ) )?;

      Self::new( key )
    }

    /// Loads the API key from workspace secrets directory.
    ///
    /// Uses the `workspace_tools` crate to locate and read secrets from
    /// the workspace secrets directory (typically `./-secrets/`).
    ///
    /// # Arguments
    ///
    /// * `key_name` - Logical name of the key (e.g., `XAI_API_KEY`)
    /// * `filename` - Filename in secrets directory (e.g., "-secrets.sh")
    ///
    /// # Errors
    ///
    /// Returns `XaiError::Environment` if the secret cannot be loaded,
    /// or `XaiError::InvalidApiKey` if validation fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use api_xai::Secret;
    ///
    /// let secret = Secret::load_from_workspace( "XAI_API_KEY", "-secrets.sh" )?;
    /// # Ok::<(), Box< dyn std::error::Error > >(())
    /// ```
    pub fn load_from_workspace( key_name : &str, filename : &str ) -> Result< Self >
    {
      let ws = workspace()
        .map_err( |e| XaiError::Environment(
          format!( "Failed to access workspace : {e}" )
        ) )?;

      let key = ws.load_secret_key( key_name, filename )
        .map_err( |e| XaiError::Environment(
          format!( "Failed to load from workspace secrets : {e}" )
        ) )?;

      Self::new( key )
    }

    /// Loads the API key with automatic fallback chain.
    ///
    /// Attempts to load the API key from multiple sources in priority order:
    ///
    /// 1. Workspace secrets file (`-secrets.sh`) - primary workspace pattern
    /// 2. Alternative workspace files (`secrets.sh`, `.env`)
    /// 3. Environment variable - fallback for CI/deployment
    ///
    /// This is the **recommended** loading method for most use cases.
    /// Follows wTools ecosystem conventions by prioritizing `workspace_tools`.
    ///
    /// # Arguments
    ///
    /// * `key_name` - Name of the key (e.g., `XAI_API_KEY`)
    ///
    /// # Errors
    ///
    /// Returns `XaiError::Environment` if the key cannot be loaded from any source,
    /// or `XaiError::InvalidApiKey` if validation fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use api_xai::Secret;
    ///
    /// // Recommended : tries all sources automatically (workspace-first)
    /// let secret = Secret::load_with_fallbacks( "XAI_API_KEY" )?;
    /// # Ok::<(), Box< dyn std::error::Error > >(())
    /// ```
    pub fn load_with_fallbacks( key_name : &str ) -> Result< Self >
    {
      // Priority 1: Workspace secrets (-secrets.sh) - primary workspace pattern
      if let Ok( secret ) = Self::load_from_workspace( key_name, "-secrets.sh" )
      {
        return Ok( secret );
      }

      // Priority 2: Alternative workspace file (secrets.sh)
      if let Ok( secret ) = Self::load_from_workspace( key_name, "secrets.sh" )
      {
        return Ok( secret );
      }

      // Priority 3: .env file
      if let Ok( secret ) = Self::load_from_workspace( key_name, ".env" )
      {
        return Ok( secret );
      }

      // Priority 4: Environment variable (fallback for CI/deployment)
      if let Ok( secret ) = Self::load_from_env( key_name )
      {
        return Ok( secret );
      }

      Err( XaiError::Environment(
        format!(
          "Failed to load {key_name} from any source (-secrets.sh, secrets.sh, .env, env)"
        )
      ).into() )
    }

    /// Validates the API key format.
    ///
    /// # Validation Rules
    ///
    /// - Must start with `xai-` prefix
    /// - Must be at least 10 characters long
    ///
    /// # Errors
    ///
    /// Returns `XaiError::InvalidApiKey` if validation fails.
    fn validate_format( key : &str ) -> Result< () >
    {
      if !key.starts_with( "xai-" )
      {
        return Err( XaiError::InvalidApiKey(
          "XAI API key must start with 'xai-' prefix".to_string()
        ).into() );
      }

      if key.len() < 10
      {
        return Err( XaiError::InvalidApiKey(
          "XAI API key too short (minimum 10 characters)".to_string()
        ).into() );
      }

      Ok( () )
    }

    /// Exposes the secret value for use in API requests.
    ///
    /// **WARNING**: This method exposes the secret in plain text. Use sparingly
    /// and only when necessary for API authentication.
    ///
    /// Each call increments the global exposure counter for security auditing.
    ///
    /// # Returns
    ///
    /// The plain text API key as a string slice.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use api_xai::Secret;
    ///
    /// let secret = Secret::load_with_fallbacks( "XAI_API_KEY" )?;
    ///
    /// // Use only when necessary (e.g., setting Authorization header)
    /// let api_key = secret.expose_secret();
    /// # Ok::<(), Box< dyn std::error::Error > >(())
    /// ```
    pub fn expose_secret( &self ) -> &str
    {
      EXPOSURE_COUNTER.fetch_add( 1, Ordering::Relaxed );
      self.0.expose_secret()
    }

    /// Returns the number of times secrets have been exposed.
    ///
    /// Used for security auditing and monitoring. Tracks all calls to
    /// `expose_secret()` across all `Secret` instances.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use api_xai::Secret;
    ///
    /// let secret = Secret::load_with_fallbacks( "XAI_API_KEY" )?;
    /// let initial_count = Secret::exposure_count();
    ///
    /// let _ = secret.expose_secret();
    /// assert_eq!( Secret::exposure_count(), initial_count + 1 );
    /// # Ok::<(), Box< dyn std::error::Error > >(())
    /// ```
    pub fn exposure_count() -> usize
    {
      EXPOSURE_COUNTER.load( Ordering::Relaxed )
    }
  }
}

crate::mod_interface!
{
  exposed use
  {
    Secret,
  };
}
