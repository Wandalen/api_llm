//! Workspace secret management for Ollama client.
//!
//! Provides WorkspaceSecretStore for secure handling of API keys and secrets
//! from workspace configuration files.

#[ cfg( feature = "enabled" ) ]
mod private
{
  use std::collections::HashMap;
  use std::fmt;
  use error_tools::untyped::{ format_err, Result };
  use workspace_tools::{ workspace, WorkspaceError };

  /// Result type for Ollama API operations
  pub type OllamaResult< T > = Result< T >;

  /// Secret store that integrates with workspace_tools
  ///
  /// Provides fallback chain : workspace secrets → environment variables → error
  #[ derive( Clone ) ]
  pub struct WorkspaceSecretStore
  {
    secrets : HashMap<  String, String  >,
    workspace_path : Option< std::path::PathBuf >,
  }

  impl WorkspaceSecretStore
  {
    /// Create secret store from workspace_tools
    pub fn from_workspace() -> OllamaResult< Self >
    {
      let ws = workspace()
        .map_err( | e | format_err!( "Failed to resolve workspace : {}", Self::sanitize_error( &format!( "{}", e ) ) ) )?;

      let workspace_path = Some( ws.root().to_path_buf() );

      let secrets = match ws.load_secrets_from_file( "-secrets.sh" )
      {
        Ok( secrets ) => secrets,
        Err( WorkspaceError::IoError( _ ) ) =>
        {
          HashMap::new()
        },
        Err( e ) => return Err( format_err!( "Failed to load workspace configuration : {}", Self::sanitize_error( &format!( "{}", e ) ) ) ),
      };

      Ok( Self { secrets, workspace_path } )
    }

    /// Get a secret value with fallback chain : workspace secrets → environment → error
    pub fn get_secret( &self, key : &str ) -> OllamaResult< String >
    {
      if let Some( value ) = self.secrets.get( key )
      {
        return Ok( value.clone() );
      }

      std ::env::var( key )
        .map_err( | _ | format_err!( "{} not found in workspace secrets or environment variables", key ) )
    }

    /// Check if a secret exists in either workspace secrets or environment
    pub fn has_secret( &self, key : &str ) -> bool
    {
      self.secrets.contains_key( key ) || std::env::var( key ).is_ok()
    }

    /// Get all available secret keys (without values for security)
    pub fn available_keys( &self ) -> Vec< String >
    {
      let mut keys = self.secrets.keys().cloned().collect::< Vec< _ > >();

      for ( env_key, _ ) in std::env::vars()
      {
        if Self::looks_like_secret_key( &env_key ) && !keys.contains( &env_key )
        {
          keys.push( env_key );
        }
      }

      keys.sort();
      keys
    }

    /// Get workspace path if available
    pub fn workspace_path( &self ) -> Option< &std::path::Path >
    {
      self.workspace_path.as_deref()
    }

    /// Sanitize error messages to prevent secret exposure
    pub fn sanitize_error( error_msg : &str ) -> String
    {
      let mut sanitized = error_msg.to_string();

      sanitized = sanitized.replace( "Secrets file", "Configuration file" );
      sanitized = sanitized.replace( "secrets file", "configuration file" );
      sanitized = sanitized.replace( "secrets directory", "configuration directory" );
      sanitized = sanitized.replace( "Secrets directory", "Configuration directory" );
      sanitized = sanitized.replace( "-secrets.sh", "-config.sh" );

      let secret_indicators = [ "key=", "key:", "token=", "token:", "password=", "password:", "secret=", "secret:", "auth=", "auth:" ];

      for indicator in &secret_indicators
      {
        if let Some( start ) = sanitized.find( indicator )
        {
          let value_start = start + indicator.len();
          if let Some( end ) = sanitized[ value_start.. ].find( ' ' ).map( | i | value_start + i )
          {
            sanitized.replace_range( value_start..end, "***" );
          }
          else
          {
            sanitized.replace_range( value_start.., "***" );
          }
        }
      }

      sanitized
    }

    /// Check if environment variable key looks like a secret
    fn looks_like_secret_key( key : &str ) -> bool
    {
      let key_lower = key.to_lowercase();
      key_lower.contains( "key" ) ||
      key_lower.contains( "token" ) ||
      key_lower.contains( "secret" ) ||
      key_lower.contains( "password" ) ||
      key_lower.contains( "auth" )
    }
  }

  impl fmt::Debug for WorkspaceSecretStore
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      let masked_secrets : HashMap<  String, String  > = self.secrets
        .keys()
        .map( | k |
        {
          let masked_value = if self.secrets[ k ].len() <= 8
          {
            "***".to_string()
          }
          else
          {
            format!( "{}***", &self.secrets[ k ][ ..4 ] )
          };
          ( k.clone(), masked_value )
        })
        .collect();

      f.debug_struct( "WorkspaceSecretStore" )
        .field( "secrets", &masked_secrets )
        .field( "workspace_path", &self.workspace_path )
        .finish()
    }
  }

  impl fmt::Display for WorkspaceSecretStore
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "WorkspaceSecretStore with {} secrets from workspace", self.secrets.len() )?;
      if let Some( path ) = &self.workspace_path
      {
        write!( f, " at {}", path.display() )?;
      }
      Ok( () )
    }
  }
}

#[ cfg( feature = "enabled" ) ]
crate ::mod_interface!
{
  exposed use WorkspaceSecretStore;
}
