//! OllamaClient extension for workspace integration.
//!
//! Provides methods for creating clients from workspace configurations,
//! auto-discovering workspace settings, and loading workspace secrets.

#[ cfg( any( feature = "workspace", all( feature = "workspace", feature = "secret_management" ) ) ) ]
mod private
{
  use core::time::Duration;
  use std::path::Path;
  use std::env;
  use crate::client::OllamaClient;
  use crate::OllamaResult;
  use crate::auth::WorkspaceConfig;
  use error_tools::format_err;

  /// Extension to OllamaClient for workspace integration
  impl OllamaClient
  {
    /// Create client from workspace configuration file
    ///
    /// # Errors
    ///
    /// Returns an error if the workspace configuration file cannot be read or parsed
    #[ cfg( feature = "workspace" ) ]
    #[ inline ]
    pub fn from_workspace< P : AsRef< Path > >( config_path : P ) -> OllamaResult< Self >
    {
      let workspace_config = WorkspaceConfig::from_file( config_path )?;
      let timeout = workspace_config.timeout_secs()
        .map( Duration::from_secs )
        .unwrap_or_else( Self::recommended_timeout_default );
      let client = Self::new( workspace_config.server_url().to_string(), timeout );

      Ok( client )
    }

    /// Create client using auto-discovery of workspace configuration
    ///
    /// # Errors
    ///
    /// Returns an error if no workspace configuration can be discovered or parsed
    #[ cfg( feature = "workspace" ) ]
    #[ inline ]
    pub fn from_auto_workspace() -> OllamaResult< Self >
    {
      match WorkspaceConfig::auto_discover()
      {
        Ok( workspace_config ) =>
        {
          let timeout = workspace_config.timeout_secs()
            .map( Duration::from_secs )
            .unwrap_or_else( Self::recommended_timeout_default );
          let client = Self::new( workspace_config.server_url().to_string(), timeout );

          Ok( client )
        },
        Err( e ) => Err( format_err!( "No workspace configuration found : {}", e ) ),
      }
    }

    /// Create client from workspace secrets
    ///
    /// Loads API credentials from workspace secret/-secrets.sh and creates a configured client.
    ///
    /// # Errors
    ///
    /// Returns an error if workspace secrets cannot be loaded or required credentials are missing
    #[ cfg( all( feature = "workspace", feature = "secret_management" ) ) ]
    #[ inline ]
    pub fn from_workspace_secrets() -> OllamaResult< Self >
    {
      #[ cfg( all( feature = "workspace", feature = "secret_management" ) ) ]
      {
        use workspace_tools::{ workspace, WorkspaceError };

        let ws = workspace()
          .map_err( | e | format_err!( "Failed to resolve workspace : {}", e ) )?;

        // Load secrets from workspace secret/-secrets.sh
        let workspace_secrets = match ws.load_secrets_from_file( "-secrets.sh" )
        {
          Ok( secrets ) => secrets,
          Err( WorkspaceError::IoError( _ ) ) =>
          {
            // Secret file doesnt exist - try environment fallback
            std ::collections::HashMap::new()
          },
          Err( e ) =>
          {
            // Sanitize error message to avoid exposing secret-related terms
            let sanitized = crate::workspace::WorkspaceSecretStore::sanitize_error( &format!( "{e}" ) );
            return Err( format_err!( "Failed to load workspace configuration : {}", sanitized ) );
          },
        };

        // Get server URL with fallback chain
        let server_url = workspace_secrets.get( "OLLAMA_URL" )
          .or_else( || workspace_secrets.get( "OLLAMA_SERVER_URL" ) )
          .cloned()
          .or_else( || env::var( "OLLAMA_URL" ).ok() )
          .or_else( || env::var( "OLLAMA_SERVER_URL" ).ok() )
          .unwrap_or_else( || "http://localhost:11434".to_string() );

        // Configure timeout if specified
        let timeout_str = workspace_secrets.get( "OLLAMA_TIMEOUT_SECS" )
          .cloned()
          .or_else( || env::var( "OLLAMA_TIMEOUT_SECS" ).ok() );

        let timeout = timeout_str
          .and_then( | s | s.parse::< u64 >().ok() )
          .map( Duration::from_secs )
          .unwrap_or_else( Self::recommended_timeout_default );

        let mut client = Self::new( server_url, timeout );

        // Configure API key if available
        let api_key = workspace_secrets.get( "OLLAMA_API_KEY" )
          .cloned()
          .or_else( || env::var( "OLLAMA_API_KEY" ).ok() );

        if let Some( api_key ) = api_key
        {
          #[ cfg( feature = "authentication" ) ]
          {
            client = client.with_api_key( &api_key )?;
          }
          #[ cfg( not( feature = "authentication" ) ) ]
          {
            let _ = api_key; // Silence unused variable warning
          }
        }

        // Configure secret store with workspace secrets
        #[ cfg( feature = "secret_management" ) ]
        {
          use crate::SecretStore;
          let secret_store = SecretStore::from_workspace()?;
          client.secret_store = Some( secret_store );
        }

        Ok( client )
      }

      #[ cfg( not( all( feature = "workspace", feature = "secret_management" ) ) ) ]
      {
        Err( format_err!( "Workspace secrets require both 'workspace' and 'secret_management' features" ) )
      }
    }

    /// Create client from workspace secrets at specific path
    ///
    /// Loads API credentials from workspace secret/-secrets.sh at specified path and creates a configured client.
    /// This method is useful for testing and when workspace path is known explicitly.
    ///
    /// # Errors
    ///
    /// Returns an error if workspace path is invalid or required credentials are missing
    #[ cfg( all( feature = "workspace", feature = "secret_management" ) ) ]
    #[ inline ]
    pub fn from_workspace_secrets_at< P : AsRef< Path > >( workspace_path : P ) -> OllamaResult< Self >
    {
      #[ cfg( all( feature = "workspace", feature = "secret_management" ) ) ]
      {
        use workspace_tools::{ Workspace, WorkspaceError };

        let ws = Workspace::new( workspace_path.as_ref() );

        // Load secrets from workspace secret/-secrets.sh
        let workspace_secrets = match ws.load_secrets_from_file( "-secrets.sh" )
        {
          Ok( secrets ) => secrets,
          Err( WorkspaceError::IoError( _ ) ) =>
          {
            // Secret file doesnt exist - use empty map, will fallback to environment
            std ::collections::HashMap::new()
          },
          Err( e ) =>
          {
            // Sanitize error message to avoid exposing secret-related terms
            let sanitized = crate::workspace::WorkspaceSecretStore::sanitize_error( &format!( "{e}" ) );
            return Err( format_err!( "Failed to load workspace configuration : {}", sanitized ) );
          },
        };

        // Clean up keys by removing "export " prefix
        let mut cleaned_secrets = std::collections::HashMap::new();
        for ( key, value ) in workspace_secrets
        {
          let clean_key = if let Some( stripped ) = key.strip_prefix( "export " )
          {
            stripped.to_string()
          }
          else
          {
            key
          };
          cleaned_secrets.insert( clean_key, value );
        }

        // Configure server URL with fallback chain
        let server_url = cleaned_secrets.get( "OLLAMA_URL" )
          .or_else( || cleaned_secrets.get( "OLLAMA_SERVER_URL" ) )
          .cloned()
          .or_else( || env::var( "OLLAMA_URL" ).ok() )
          .unwrap_or_else( || "http://localhost:11434".to_string() );

        // Configure timeout if specified
        let timeout_str = cleaned_secrets.get( "OLLAMA_TIMEOUT_SECS" )
          .cloned()
          .or_else( || env::var( "OLLAMA_TIMEOUT_SECS" ).ok() );

        let timeout = timeout_str
          .and_then( | s | s.parse::< u64 >().ok() )
          .map( Duration::from_secs )
          .unwrap_or_else( Self::recommended_timeout_default );

        let mut client = Self::new( server_url, timeout );

        // Configure API key if available
        let api_key = cleaned_secrets.get( "OLLAMA_API_KEY" )
          .cloned()
          .or_else( || env::var( "OLLAMA_API_KEY" ).ok() );

        if let Some( api_key ) = api_key
        {
          #[ cfg( feature = "authentication" ) ]
          {
            client = client.with_api_key( &api_key )?;
          }
          #[ cfg( not( feature = "authentication" ) ) ]
          {
            let _ = api_key; // Silence unused variable warning
          }
        }

        // Configure secret store with workspace secrets
        #[ cfg( feature = "secret_management" ) ]
        {
          use crate::SecretStore;
          let secret_store = SecretStore::from_path( ws.root() )?;
          client.secret_store = Some( secret_store );
        }

        Ok( client )
      }

      #[ cfg( not( all( feature = "workspace", feature = "secret_management" ) ) ) ]
      {
        Err( format_err!( "Workspace secrets require both 'workspace' and 'secret_management' features" ) )
      }
    }
  }
}

#[ cfg( any( feature = "workspace", all( feature = "workspace", feature = "secret_management" ) ) ) ]
crate ::mod_interface!
{
  exposed use {};
}
