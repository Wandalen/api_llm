// src/secret.rs
//! This module defines the `Secret` type for handling sensitive information
//! like API keys. It wraps a string and ensures that the secret is not
//! accidentally exposed in debug output or logs.

/// Define a private namespace for all its items.
mod private
{
  // External crates
  use secrecy::{ SecretString, ExposeSecret }; // Added ExposeSecret
  use std::path::Path;
  use error_tools::untyped::Result; // Use untyped Result
  use workspace_tools::workspace;  // Add workspace_tools
  use core::sync::atomic::{ AtomicU64, Ordering };

  /// Global counter for tracking secret exposures
  static SECRET_EXPOSURE_COUNT : AtomicU64 = AtomicU64::new( 0 );

  /// Validates the format of an API key secret
  ///
  /// `OpenAI` API keys should follow the pattern : sk-[48 characters]
  /// Test keys should follow the pattern : sk-test-[24 characters]
  fn validate_api_key_format( secret : &str ) -> Result< () >
  {
    let trimmed = secret.trim();

    // Check minimum length
    if trimmed.len() < 10
    {
      return Err( error_tools::Error::from( crate::error::OpenAIError::InvalidArgument(
        "API key too short - minimum 10 characters required".to_string()
      ) ) );
    }

    // Check maximum reasonable length (prevent extremely long strings)
    if trimmed.len() > 200
    {
      return Err( error_tools::Error::from( crate::error::OpenAIError::InvalidArgument(
        "API key too long - maximum 200 characters allowed".to_string()
      ) ) );
    }

    // Check for OpenAI API key prefix
    if !trimmed.starts_with( "sk-" )
    {
      return Err( error_tools::Error::from( crate::error::OpenAIError::InvalidArgument(
        "API key must start with 'sk-' prefix".to_string()
      ) ) );
    }

    // Check for valid characters after prefix
    let key_part = &trimmed[ 3.. ];
    if key_part.is_empty()
    {
      return Err( error_tools::Error::from( crate::error::OpenAIError::InvalidArgument(
        "API key missing content after 'sk-' prefix".to_string()
      ) ) );
    }

    // Validate character set (alphanumeric and common special characters)
    if !key_part.chars().all( | c | c.is_ascii_alphanumeric() || "_-".contains( c ) )
    {
      return Err( error_tools::Error::from( crate::error::OpenAIError::InvalidArgument(
        "API key contains invalid characters - only alphanumeric, underscore, and hyphen allowed".to_string()
      ) ) );
    }

    Ok( () )
  }

  /// Represents a secret string, such as an API key.
  /// It wraps `secrecy::SecretString` to prevent accidental exposure.
  ///
  /// # Example
  ///
  /// ```no_run
  /// use api_openai::Secret;
  ///
  /// // Create from environment variable (recommended)
  /// let secret = Secret::load_from_env("OPENAI_API_KEY")?;
  ///
  /// // Create from string with validation
  /// let secret = Secret::new("sk-example123".to_string())?;
  ///
  /// // Create without validation (for testing)
  /// let secret = Secret::new_unchecked("test-key".to_string());
  /// # Ok::<(), Box< dyn core::error::Error > >(())
  /// ```
  #[ derive( Debug, Clone ) ] // Removed PartialEq, Eq, Hash, PartialOrd, Ord
  #[ non_exhaustive ]
  pub struct Secret( SecretString );

  impl Secret
  {
    /// Creates a new `Secret` from a string with validation.
    ///
    /// # Errors
    /// Returns `OpenAIError::InvalidArgument` if the secret format is invalid.
    #[ inline ]
    pub fn new( secret : String ) -> Result< Self >
    {
      validate_api_key_format( &secret )?;
      Ok( Self( SecretString::from( secret ) ) )
    }

    /// Creates a new `Secret` from a string without validation.
    /// This should only be used when the secret format is already known to be valid.
    ///
    /// # Safety
    /// This function bypasses validation and should only be used in controlled contexts.
    #[ inline ]
    #[ must_use ]
    pub fn new_unchecked( secret : String ) -> Self
    {
      Self( SecretString::from( secret ) )
    }

    /// Loads a secret from a file at the given path.
    ///
    /// # Arguments
    /// - `path`: The path to the file containing the secret.
    ///
    /// # Errors
    /// Returns `OpenAIError::File` if the file cannot be read.
    #[ allow( clippy::missing_panics_doc ) ] // This is a configuration error, panicking is acceptable
    #[ allow( clippy::panic ) ] // This is a configuration error, panicking is acceptable
    #[ must_use = "Loading a secret from a file should be handled or assigned." ]
    #[ inline ]
    pub fn load_with_path( path : &Path ) -> Result< Self > // Corrected path
    {
      let secret_string = std::fs::read_to_string( path )
      .map_err( | e | error_tools::Error::from( crate::error::OpenAIError::File( format!( "Failed to read secret file : {e}" ) ) ) )?; // Corrected path
      Self::new( secret_string.trim().to_string() )
        .map_err( | e | error_tools::Error::from( crate::error::OpenAIError::File( format!( "Invalid secret format in file : {e}" ) ) ) )
    }

    /// Loads a secret from an environment variable.
    ///
    /// # Arguments
    /// - `env_var`: The name of the environment variable.
    ///
    /// # Errors
    /// Returns `OpenAIError::MissingEnvironment` if the environment variable is not found.
    #[ inline ]
    pub fn load_from_env( env_var : &str ) -> Result< Self > // Corrected path
    {
      let secret_string = std::env::var( env_var )
      .map_err( | e | error_tools::Error::from( crate::error::OpenAIError::MissingEnvironment( format!( "Missing environment variable {env_var}: {e}" ) ) ) )?; // Corrected path
      Self::new( secret_string.trim().to_string() )
        .map_err( | e | error_tools::Error::from( crate::error::OpenAIError::MissingEnvironment( format!( "Invalid secret format in {env_var}: {e}" ) ) ) )
    }

    /// Loads a secret from the centralized workspace secrets directory.
    ///
    /// # Arguments
    /// - `key_name`: The name of the secret key to load.
    /// - `filename`: The filename in .secrets directory (e.g., "-secrets.sh").
    ///
    /// # Errors
    /// Returns `OpenAIError::MissingEnvironment` if the secret cannot be found.
    #[ inline ]
    pub fn load_from_workspace( key_name : &str, filename : &str ) -> Result< Self >
    {
      let ws = workspace()
        .map_err( | e | error_tools::Error::from( crate::error::OpenAIError::MissingEnvironment( format!( "Failed to access workspace : {e}" ) ) ) )?;

      let secret_string = ws.load_secret_key( key_name, filename )
        .map_err( | e | error_tools::Error::from( crate::error::OpenAIError::MissingEnvironment( format!( "Failed to load secret {key_name} from {filename}: {e}" ) ) ) )?;

      Self::new( secret_string.trim().to_string() )
        .map_err( | e | error_tools::Error::from( crate::error::OpenAIError::MissingEnvironment( format!( "Invalid secret format for {key_name} in {filename}: {e}" ) ) ) )
    }

    /// Load secret with comprehensive fallback chain using `workspace_tools`
    ///
    /// Priority order:
    /// 1. Environment variable (fastest)
    /// 2. Workspace secrets file (main secrets file)
    /// 3. Alternative secrets files
    ///
    /// # Arguments
    /// - `key_name`: The name of the secret key to load (e.g., "`OPENAI_API_KEY`")
    ///
    /// # Errors
    /// Returns `OpenAIError::MissingEnvironment` if the key is not found in any location
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use api_openai::Secret;
    ///
    /// // Tries environment variable first, then workspace secrets
    /// let secret = Secret::load_with_fallbacks("OPENAI_API_KEY")?;
    /// # Ok::<(), Box< dyn std::error::Error > >(())
    /// ```
    #[ inline ]
    pub fn load_with_fallbacks( key_name : &str ) -> Result< Self >
    {
      // 1. Try environment variable first (fastest)
      if let Ok( secret ) = Self::load_from_env( key_name )
      {
        return Ok( secret );
      }

      // 2. Try workspace_tools with default secrets file
      if let Ok( secret ) = Self::load_from_workspace( key_name, "-secrets.sh" )
      {
        return Ok( secret );
      }

      // 3. Try alternative secrets files
      for filename in [ "secrets.sh", ".env" ]
      {
        if let Ok( secret ) = Self::load_from_workspace( key_name, filename )
        {
          return Ok( secret );
        }
      }

      Err( error_tools::Error::from( crate::error::OpenAIError::MissingEnvironment(
        format!( "{key_name} not found in environment or workspace secrets. Please add it to your environment variables or secret/-secrets.sh file" )
      ) ) )
    }

    /// Get the total number of secret exposures that have occurred
    /// This is useful for security auditing and monitoring
    #[ inline ]
    #[ must_use ]
    pub fn exposure_count() -> u64
    {
      SECRET_EXPOSURE_COUNT.load( Ordering::Relaxed )
    }

    /// Reset the secret exposure counter (for testing purposes)
    /// WARNING: This should only be used in test environments
    #[ cfg( test ) ]
    #[ inline ]
    pub fn reset_exposure_count()
    {
      SECRET_EXPOSURE_COUNT.store( 0, Ordering::Relaxed );
    }
  }

  impl ExposeSecret< str > for Secret
  {
    #[ inline ]
    fn expose_secret( &self ) -> &str
    {
      // Increment global exposure counter for audit trail
      let exposure_count = SECRET_EXPOSURE_COUNT.fetch_add( 1, Ordering::Relaxed ) + 1;

      // Log secret exposure for audit purposes (without exposing the secret value)
      let secret_hash = {
        use std::collections::hash_map::DefaultHasher;
        use core::hash::{ Hash, Hasher };
        let mut hasher = DefaultHasher::new();
        self.0.expose_secret().hash( &mut hasher );
        hasher.finish()
      };

      eprintln!(
        "[AUDIT] Secret exposure #{} - Hash : {:x} - Caller : {}:{}:{}",
        exposure_count,
        secret_hash,
        file!(),
        line!(),
        column!()
      );

      self.0.expose_secret()
    }
  }

  impl From< String > for Secret
  {
    #[ inline ]
    fn from( secret : String ) -> Self
    {
      Self::new_unchecked( secret )
    }
  }

  impl From< &str > for Secret
  {
    #[ inline ]
    fn from( secret : &str ) -> Self
    {
      Self::new_unchecked( secret.to_owned() )
    }
  }

} // end mod private

crate ::mod_interface!
{
  // Expose all structs defined in this module
  exposed use
  {
    Secret,
  };
}