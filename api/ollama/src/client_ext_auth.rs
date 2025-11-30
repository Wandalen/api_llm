//! OllamaClient authentication extension.
//!
//! Methods for managing API keys, auth tokens, and secret stores.
//! Provides secure authentication handling for Ollama API requests.

#[ cfg( feature = "secret_management" ) ]
mod private
{
  use crate::client::OllamaClient;
  use crate::{ OllamaResult, SecretStore };
  use crate::auth::AuthHelper;
  use error_tools::format_err;

  impl OllamaClient
  {
    /// Configure client with secret store
    #[ inline ]
    #[ must_use ]
    pub fn with_secret_store( mut self, secret_store : SecretStore ) -> Self
    {
      self.secret_store = Some( secret_store );
      self
    }

    /// Check if this client has secrets configured
    #[ cfg( feature = "secret_management" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn has_secrets( &self ) -> bool
    {
      self.secret_store.as_ref().is_some_and( | store | !store.is_empty() )
    }

    /// Get a secret from the client's secret store
    ///
    /// # Errors
    ///
    /// Returns an error if the secret store is not configured
    #[ cfg( feature = "secret_management" ) ]
    #[ inline ]
    pub fn get_secret( &mut self, key : &str ) -> OllamaResult< Option< String > >
    {
      match &mut self.secret_store
      {
        Some( store ) => store.get( key ),
        None => Err( format_err!( "No secret store configured" ) ),
      }
    }

    /// Add authentication to an HTTP request builder
    #[ cfg( feature = "secret_management" ) ]
    #[ inline ]
    pub( crate ) fn apply_authentication( &mut self, request_builder : reqwest::RequestBuilder ) -> reqwest::RequestBuilder
    {
      AuthHelper::apply_authentication( &mut self.secret_store, request_builder )
    }

    /// Set authentication credentials from secret store
    ///
    /// # Errors
    ///
    /// Returns an error if the API key cannot be stored in the secret store
    #[ cfg( feature = "secret_management" ) ]
    #[ inline ]
    pub fn with_api_key( mut self, api_key : &str ) -> OllamaResult< Self >
    {
      if self.secret_store.is_none()
      {
        self.secret_store = Some( SecretStore::new() );
      }
      
      if let Some( store ) = &mut self.secret_store
      {
        store.set( "api_key", api_key )?;
      }
      
      Ok( self )
    }

    /// Set authentication token from secret store  
    ///
    /// # Errors
    ///
    /// Returns an error if the auth token cannot be stored in the secret store
    #[ cfg( feature = "secret_management" ) ]
    #[ inline ]
    pub fn with_auth_token( mut self, auth_token : &str ) -> OllamaResult< Self >
    {
      if self.secret_store.is_none()
      {
        self.secret_store = Some( SecretStore::new() );
      }
      
      if let Some( store ) = &mut self.secret_store
      {
        store.set( "auth_token", auth_token )?;
      }
      
      Ok( self )
    }
  }
}
