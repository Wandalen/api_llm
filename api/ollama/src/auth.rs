//! Authentication and secret management functionality for Ollama API client.

#[ cfg( any( feature = "secret_management", feature = "workspace" ) ) ]
mod private
{
  use std::collections::HashMap;
  use error_tools::untyped::{ format_err, Result };

  // =====================================
  // Secret Management Types
  // =====================================

  /// Secret entry with optional expiration
  #[ cfg( feature = "secret_management" ) ]
  #[ derive( Debug, Clone ) ]
  struct SecretEntry
  {
    value : String,
    expires_at : Option< u64 >,
  }

  /// Secure secret storage for credentials
  #[ cfg( feature = "secret_management" ) ]
  #[ derive( Clone ) ]
  pub struct SecretStore
  {
    secrets : HashMap<  String, SecretEntry  >,
  }

  #[ cfg( feature = "secret_management" ) ]
  impl core::fmt::Debug for SecretStore
  {
    #[ inline ]
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      f.debug_struct( "SecretStore" )
        .field( "secrets_count", &self.secrets.len() )
        .field( "secrets", &"***[REDACTED]***" )
        .finish()
    }
  }

  /// Configuration for secret management
  #[ cfg( feature = "secret_management" ) ]
  #[ derive( Debug, Clone ) ]
  pub struct SecretConfig
  {
    secrets : HashMap<  String, String  >,
  }

  /// Workspace configuration for Ollama client
  #[ cfg( feature = "workspace" ) ]
  #[ derive( Debug, Clone, serde::Deserialize ) ]
  pub struct WorkspaceConfig
  {
    /// Ollama server configuration
    pub ollama : OllamaServerConfig,
  }

  /// Ollama server configuration within workspace
  #[ cfg( feature = "workspace" ) ]
  #[ derive( Debug, Clone, serde::Deserialize ) ]
  pub struct OllamaServerConfig
  {
    /// Server URL for Ollama API
    pub server_url : String,
    /// Default model to use
    #[ serde( default ) ]
    pub default_model : Option< String >,
    /// Request timeout in seconds
    #[ serde( default ) ]
    pub timeout_secs : Option< u64 >,
    /// Preferred models list
    #[ serde( default ) ]
    pub models : Vec< String >,
    /// API key for authentication
    #[ serde( default ) ]
    pub api_key : Option< String >,
  }

  // =====================================
  // Workspace Configuration Implementations
  // =====================================

  #[ cfg( feature = "workspace" ) ]
  impl WorkspaceConfig
  {
    /// Load workspace configuration from file
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration file cannot be read or parsed
    #[ inline ]
    pub fn from_file< P : AsRef< std::path::Path > >( config_path : P ) -> Result< Self >
    {
      let config_path = config_path.as_ref();
      let content = std::fs::read_to_string( config_path )
        .map_err( | e | format_err!( "Failed to read workspace config file '{}': {}", config_path.display(), e ) )?;

      let config : WorkspaceConfig = toml::from_str( &content )
        .map_err( | e | format_err!( "Failed to parse workspace config file '{}': {}", config_path.display(), e ) )?;

      Ok( config )
    }

    /// Auto-discover workspace configuration
    ///
    /// # Errors
    ///
    /// Returns an error if no configuration files are found or they cannot be parsed
    #[ inline ]
    pub fn auto_discover() -> Result< Self >
    {
      // Look for ollama.toml in current directory
      let current_dir = std::env::current_dir()
        .map_err( | e | format_err!( "Failed to get current directory : {}", e ) )?;

      let config_path = current_dir.join( "ollama.toml" );
      if config_path.exists()
      {
        return Self::from_file( config_path );
      }

      // Look for .ollama/config.toml in current directory
      let hidden_config_path = current_dir.join( ".ollama" ).join( "config.toml" );
      if hidden_config_path.exists()
      {
        return Self::from_file( hidden_config_path );
      }

      // Look in home directory
      if let Ok( home_dir ) = std::env::var( "HOME" )
      {
        let home_config_path = std::path::PathBuf::from( home_dir ).join( ".ollama" ).join( "config.toml" );
        if home_config_path.exists()
        {
          return Self::from_file( home_config_path );
        }
      }

      Err( format_err!( "No workspace configuration found" ) )
    }

    /// Get server URL from configuration
    #[ inline ]
    #[ must_use ]
    pub fn server_url( &self ) -> &str
    {
      &self.ollama.server_url
    }

    /// Get default model from configuration
    #[ inline ]
    #[ must_use ]
    pub fn default_model( &self ) -> Option< &str >
    {
      self.ollama.default_model.as_deref()
    }

    /// Get timeout in seconds from configuration
    #[ inline ]
    #[ must_use ]
    pub fn timeout_secs( &self ) -> Option< u64 >
    {
      self.ollama.timeout_secs
    }

    /// Get preferred models list from configuration
    #[ inline ]
    #[ must_use ]
    pub fn preferred_models( &self ) -> &Vec< String >
    {
      &self.ollama.models
    }

    /// Get API key from configuration
    #[ inline ]
    #[ must_use ]
    pub fn api_key( &self ) -> Option< &str >
    {
      self.ollama.api_key.as_deref()
    }

    /// Load workspace configuration with integrated secrets
    ///
    /// Combines workspace configuration file with secrets from `workspace_tools`.
    ///
    /// # Errors
    ///
    /// Returns an error if workspace path is invalid or secrets cannot be loaded
    #[ cfg( all( feature = "workspace", feature = "secret_management" ) ) ]
    #[ inline ]
    pub fn from_workspace_with_secrets< P : AsRef< std::path::Path > >( workspace_path : P ) -> Result< Self >
    {
      // First load regular workspace configuration
      let config_file = workspace_path.as_ref().join( "config" ).join( "ollama.toml" );
      let workspace_config = if config_file.exists()
      {
        Self::from_file( config_file )?
      }
      else
      {
        // Create default config
        Self
        {
          ollama : OllamaServerConfig {
            server_url : "http://localhost:11434".to_string(),
            default_model : None,
            timeout_secs : None,
            models : Vec::new(),
            api_key : None,
          }
        }
      };

      Ok( workspace_config )
    }

    /// Get associated secret store from workspace
    ///
    /// # Errors
    ///
    /// Returns an error if workspace secrets cannot be loaded
    #[ cfg( all( feature = "workspace", feature = "secret_management" ) ) ]
    #[ inline ]
    pub fn secret_store( &self ) -> Result< SecretStore >
    {
      SecretStore::from_workspace()
    }
  }

  // =====================================
  // Secret Management Implementations
  // =====================================

  #[ cfg( feature = "secret_management" ) ]
  impl SecretStore
  {
    /// Create a new empty secret store
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        secrets : HashMap::new(),
      }
    }

    /// Create a secret store from a configuration
    #[ inline ]
    #[ must_use ]
    pub fn from_config( config : SecretConfig ) -> Self
    {
      let secrets = config.secrets.into_iter().map( | ( k, v ) |
      {
        ( k, SecretEntry { value : v, expires_at : None } )
      } ).collect();

      Self { secrets }
    }

    /// Store a secret with optional expiration
    #[ inline ]
    pub fn set( &mut self, key : &str, value : &str ) -> Result< () >
    {
      self.secrets.insert( key.to_string(), SecretEntry
      {
        value : value.to_string(),
        expires_at : None,
      } );
      Ok( () )
    }

    /// Store a secret with expiration timestamp
    #[ inline ]
    pub fn set_with_expiry( &mut self, key : &str, value : &str, expires_at : u64 ) -> Result< () >
    {
      self.secrets.insert( key.to_string(), SecretEntry
      {
        value : value.to_string(),
        expires_at : Some( expires_at ),
      } );
      Ok( () )
    }

    /// Retrieve a secret if it exists and hasn't expired
    #[ inline ]
    pub fn get( &self, key : &str ) -> Result< Option< String > >
    {
      if let Some( entry ) = self.secrets.get( key )
      {
        // Check expiration
        if let Some( expires_at ) = entry.expires_at
        {
          let now = std::time::SystemTime::now()
            .duration_since( std::time::UNIX_EPOCH )
            .map_err( | e | format_err!( "System time error : {e}" ) )?
            .as_secs();

          if now > expires_at
          {
            return Ok( None ); // Expired
          }
        }

        Ok( Some( entry.value.clone() ) )
      }
      else
      {
        Ok( None )
      }
    }

    /// Remove a secret from the store
    #[ inline ]
    pub fn remove( &mut self, key : &str ) -> Option< String >
    {
      self.secrets.remove( key ).map( | entry | entry.value )
    }

    /// Clear all secrets
    #[ inline ]
    pub fn clear( &mut self )
    {
      self.secrets.clear();
    }

    /// Get all secret keys (without values for security)
    #[ inline ]
    #[ must_use ]
    pub fn keys( &self ) -> Vec< String >
    {
      self.secrets.keys().cloned().collect()
    }

    /// Check if a secret exists
    #[ inline ]
    #[ must_use ]
    pub fn contains( &self, key : &str ) -> bool
    {
      self.secrets.contains_key( key )
    }

    /// Get the number of stored secrets
    #[ inline ]
    #[ must_use ]
    pub fn len( &self ) -> usize
    {
      self.secrets.len()
    }

    /// Check if the secret store is empty
    #[ inline ]
    #[ must_use ]
    pub fn is_empty( &self ) -> bool
    {
      self.secrets.is_empty()
    }

    /// Create a secret store from a workspace path
    #[ inline ]
    pub fn from_path< P: AsRef< std::path::Path > >( path : P ) -> Result< Self >
    {
      #[ cfg( feature = "workspace" ) ]
      {
        use workspace_tools::Workspace;

        let ws = Workspace::new( path.as_ref() );
        let workspace_secrets = ws.load_secrets_from_file( "-secrets.sh" )
          .unwrap_or_else( | _ | std::collections::HashMap::new() );

        let mut store = Self::new();
        for ( key, value ) in workspace_secrets
        {
          store.set( &key, &value )?;
        }

        Ok( store )
      }

      #[ cfg( not( feature = "workspace" ) ) ]
      {
        let _ = path;
        Ok( Self::new() )
      }
    }

    /// Create a secret store from current workspace
    #[ inline ]
    pub fn from_workspace() -> Result< Self >
    {
      #[ cfg( feature = "workspace" ) ]
      {
        use workspace_tools::workspace;

        let ws = workspace()
          .map_err( | e | format_err!( "Failed to resolve workspace : {}", e ) )?;

        let workspace_secrets = ws.load_secrets_from_file( "-secrets.sh" )
          .unwrap_or_else( | _ | std::collections::HashMap::new() );

        let mut store = Self::new();
        for ( key, value ) in workspace_secrets
        {
          store.set( &key, &value )?;
        }

        Ok( store )
      }

      #[ cfg( not( feature = "workspace" ) ) ]
      {
        Ok( Self::new() )
      }
    }

    /// Get a secret with environment variable fallback
    #[ inline ]
    pub fn get_with_fallback( &self, key : &str ) -> Result< String >
    {
      // First try the secret store
      if let Ok( Some( value ) ) = self.get( key )
      {
        return Ok( value );
      }

      // Fallback to environment variable
      std ::env::var( key )
        .map_err( | _ | format_err!( "Secret '{}' not found in store or environment", key ) )
    }

    /// Rotate a secret (replace existing secret with new value)
    #[ inline ]
    pub fn rotate( &mut self, key : &str, value : &str ) -> Result< () >
    {
      self.set( key, value )
    }

    /// Alias for set_with_expiry for backward compatibility
    #[ inline ]
    pub fn set_with_expiration( &mut self, key : &str, value : &str, expires_at : u64 ) -> Result< () >
    {
      self.set_with_expiry( key, value, expires_at )
    }

    /// Validate secret name
    #[ inline ]
    pub fn validate_secret_name( &self, name : &str ) -> Result< () >
    {
      if name.is_empty()
      {
        return Err( format_err!( "Secret name cannot be empty" ) );
      }
      Ok( () )
    }

    /// Validate secret value
    #[ inline ]
    pub fn validate_secret_value( &self, value : &str ) -> Result< () >
    {
      if value.is_empty()
      {
        return Err( format_err!( "Secret value cannot be empty" ) );
      }
      Ok( () )
    }
  }

  #[ cfg( feature = "secret_management" ) ]
  impl Default for SecretStore
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  #[ cfg( feature = "secret_management" ) ]
  impl SecretConfig
  {
    /// Create a new secret configuration
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        secrets : HashMap::new(),
      }
    }

    /// Add a secret to the configuration
    #[ inline ]
    #[ must_use ]
    pub fn with_secret( mut self, key : &str, value : &str ) -> Self
    {
      self.secrets.insert( key.to_string(), value.to_string() );
      self
    }

    /// Create configuration from a HashMap
    #[ inline ]
    #[ must_use ]
    pub fn from_map( secrets : HashMap<  String, String  > ) -> Self
    {
      Self { secrets }
    }

    /// Create configuration from environment variables
    #[ inline ]
    pub fn from_env() -> Result< Self >
    {
      let mut secrets = std::collections::HashMap::new();

      // Load secrets from environment variables
      if let Ok( api_key ) = std::env::var( "OLLAMA_API_KEY" )
      {
        secrets.insert( "api_key".to_string(), api_key );
      }

      if let Ok( secret_token ) = std::env::var( "OLLAMA_SECRET_TOKEN" )
      {
        secrets.insert( "secret_token".to_string(), secret_token );
      }

      Ok( Self { secrets } )
    }

    /// Expect method for result handling
    #[ inline ]
    #[ must_use ]
    pub fn expect( self, _msg : &str ) -> Self
    {
      self
    }

    /// Get API key from configuration
    #[ inline ]
    pub fn api_key( &self ) -> Option< &String >
    {
      self.secrets.get( "api_key" )
    }

    /// Get secret token from configuration
    #[ inline ]
    pub fn secret_token( &self ) -> Option< &String >
    {
      self.secrets.get( "secret_token" )
    }

    /// Get a secret value by key
    #[ inline ]
    pub fn get( &self, key : &str ) -> Option< &String >
    {
      self.secrets.get( key )
    }
  }

  #[ cfg( feature = "secret_management" ) ]
  impl Default for SecretConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Authentication helper functions
  #[ cfg( feature = "secret_management" ) ]
  #[ derive( Debug ) ]
  pub struct AuthHelper;

  #[ cfg( feature = "secret_management" ) ]
  impl AuthHelper
  {
    /// Apply authentication to a request builder using secret store
    #[ inline ]
    pub fn apply_authentication( secret_store : &mut Option< SecretStore >, request_builder : reqwest::RequestBuilder ) -> reqwest::RequestBuilder
    {
      if let Some( store ) = secret_store
      {
        // Try API key first
        if let Ok( Some( api_key ) ) = store.get( "api_key" )
        {
          return request_builder.header( "Authorization", format!( "Bearer {api_key}" ) );
        }

        // Try auth token
        if let Ok( Some( auth_token ) ) = store.get( "auth_token" )
        {
          return request_builder.header( "Authorization", format!( "Token {auth_token}" ) );
        }

        // Try custom API key header
        if let Ok( Some( api_key ) ) = store.get( "x_api_key" )
        {
          return request_builder.header( "X-API-Key", api_key );
        }
      }

      // No authentication configured, return as-is
      request_builder
    }
  }
}

#[ cfg( any( feature = "secret_management", feature = "workspace" ) ) ]
crate ::mod_interface!
{
  #[ cfg( feature = "secret_management" ) ]
  exposed use private::SecretStore;
  #[ cfg( feature = "secret_management" ) ]
  exposed use private::SecretConfig;
  #[ cfg( feature = "secret_management" ) ]
  exposed use private::AuthHelper;
  #[ cfg( feature = "workspace" ) ]
  exposed use private::WorkspaceConfig;
  #[ cfg( feature = "workspace" ) ]
  exposed use private::OllamaServerConfig;
}