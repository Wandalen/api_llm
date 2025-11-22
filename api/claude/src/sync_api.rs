//! Synchronous API functionality for Anthropic Claude API
//!
//! This module provides synchronous wrappers around the async API,
//! allowing users to use blocking calls instead of async/await.

// Allow missing inline attributes for sync API module
#[ allow( clippy::missing_inline_in_public_items ) ]
mod private
{
  use crate::{
    client::{ Client, ClientConfig },
    CreateMessageRequest, CreateMessageResponse,
    error::{ AnthropicError, AnthropicResult },
    secret::Secret,
    Message, messages::{ Content, Role },
  };
  #[ cfg( feature = "count-tokens" ) ]
  use crate::{ CountMessageTokensRequest, CountMessageTokensResponse };
  use std::{ sync::Arc, time::Duration };
  use tokio::runtime::Runtime;

  /// Synchronous client wrapper around the async Client
  #[ derive( Debug, Clone ) ]
  pub struct SyncClient
  {
    inner : Client,
    runtime : SyncRuntime,
  }

  /// Runtime manager for synchronous operations
  #[ derive( Debug ) ]
  pub struct SyncRuntime
  {
    handle : Arc< Runtime >,
  }

  impl Clone for SyncRuntime
  {
    fn clone( &self ) -> Self
    {
      Self
      {
        handle : Arc::clone( &self.handle ),
      }
    }
  }

  /// Builder for configuring synchronous clients
  #[ derive( Debug ) ]
  pub struct SyncClientBuilder
  {
    timeout : Option< Duration >,
    api_key : Option< String >,
    base_url : Option< String >,
  }

  impl SyncRuntime
  {
    /// Create a new synchronous runtime
    ///
    /// # Errors
    ///
    /// Returns an error if runtime creation fails
    pub fn new() -> AnthropicResult< Self >
    {
      let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err( |e| AnthropicError::InvalidRequest( format!( "Failed to create runtime : {e}" ) ) )?;

      Ok( Self
      {
        handle : Arc::new( rt ),
      } )
    }

    /// Execute a future synchronously using this runtime
    pub fn block_on< F >( &self, future : F ) -> F::Output
    where
      F : core::future::Future,
    {
      self.handle.block_on( future )
    }

    /// Get a handle to the runtime for async interop
    pub fn handle( &self ) -> &Runtime
    {
      &self.handle
    }
  }

  impl Default for SyncRuntime
  {
    fn default() -> Self
    {
      Self::new().expect( "Failed to create default runtime" )
    }
  }

  impl SyncClient
  {
    /// Create a new synchronous client with the given API key
    ///
    /// # Errors
    ///
    /// Returns an error if the API key is invalid or runtime creation fails
    pub fn new( api_key : &str ) -> AnthropicResult< Self >
    {
      let runtime = SyncRuntime::new()?;
      let secret = Secret::new( api_key.to_string() )
        .map_err( |e| AnthropicError::InvalidRequest( e.to_string() ) )?;
      let inner = Client::new( secret );

      Ok( Self
      {
        inner,
        runtime,
      } )
    }

    /// Create a synchronous client from environment variables
    ///
    /// # Errors
    ///
    /// Returns an error if environment variables are missing or invalid
    pub fn from_env() -> AnthropicResult< Self >
    {
      let runtime = SyncRuntime::new()?;
      let inner = Client::from_env()?;

      Ok( Self
      {
        inner,
        runtime,
      } )
    }

    /// Create a synchronous client from workspace configuration
    ///
    /// # Errors
    ///
    /// Returns an error if workspace loading fails or runtime creation fails
    pub fn from_workspace() -> AnthropicResult< Self >
    {
      let runtime = SyncRuntime::new()?;
      let inner = Client::from_workspace()?;

      Ok( Self
      {
        inner,
        runtime,
      } )
    }

    /// Create a synchronous client with a custom runtime
    ///
    /// # Errors
    ///
    /// Returns an error if the API key is invalid
    pub fn with_runtime( runtime : SyncRuntime, api_key : &str ) -> AnthropicResult< Self >
    {
      let secret = Secret::new( api_key.to_string() )
        .map_err( |e| AnthropicError::InvalidRequest( e.to_string() ) )?;
      let inner = Client::new( secret );

      Ok( Self
      {
        inner,
        runtime,
      } )
    }

    /// Check if API key is configured
    pub fn has_api_key( &self ) -> bool
    {
      self.inner.api_key().is_some()
    }

    /// Get an indication of the API key (for testing only)
    pub fn get_api_key( &self ) -> String
    {
      if self.has_api_key()
      {
        "sk-ant-*****".to_string()
      }
      else
      {
        "< NO_API_KEY >".to_string()
      }
    }

    /// Get the configured timeout (placeholder)
    pub fn get_timeout( &self ) -> Duration
    {
      Duration::from_secs( 30 ) // Default timeout
    }

    /// Send a message synchronously and wait for response
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response is invalid
    pub fn create_message( &self, request : &CreateMessageRequest ) -> AnthropicResult< CreateMessageResponse >
    {
      self.runtime.block_on( self.inner.create_message( request.clone() ) )
    }

    /// Create a synchronous streaming message request
    ///
    /// Returns a blocking iterator over stream events
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or streaming cannot be initiated
    #[ cfg( feature = "streaming" ) ]
    pub fn create_message_stream( &self, request : &CreateMessageRequest ) -> AnthropicResult< SyncStreamIterator >
    {
      let stream = self.runtime.block_on( self.inner.create_message_stream( request.clone() ) )?;

      Ok( SyncStreamIterator
      {
        stream,
        runtime : self.runtime.clone(),
      } )
    }

    /// Count message tokens synchronously
    ///
    /// Returns token count for the given request without making an API call
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or token counting cannot be performed
    #[ cfg( feature = "count-tokens" ) ]
    pub fn count_message_tokens( &self, request : &CountMessageTokensRequest ) -> AnthropicResult< CountMessageTokensResponse >
    {
      self.runtime.block_on( self.inner.count_message_tokens( request.clone() ) )
    }
  }

  /// Synchronous iterator wrapper around async `EventStream`
  #[ cfg( feature = "streaming" ) ]
  pub struct SyncStreamIterator
  {
    stream : crate::streaming::EventStream,
    runtime : SyncRuntime,
  }

  #[ cfg( feature = "streaming" ) ]
  impl core::fmt::Debug for SyncStreamIterator
  {
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      f.debug_struct( "SyncStreamIterator" )
        .field( "runtime", &self.runtime )
        .finish_non_exhaustive()
    }
  }

  #[ cfg( feature = "streaming" ) ]
  impl Iterator for SyncStreamIterator
  {
    type Item = AnthropicResult< crate::streaming::StreamEvent >;

    fn next( &mut self ) -> Option< Self::Item >
    {
      use futures::StreamExt;

      self.runtime.block_on( self.stream.next() )
    }
  }

  impl SyncClientBuilder
  {
    /// Create a new builder
    pub fn new() -> Self
    {
      Self
      {
        timeout : None,
        api_key : None,
        base_url : None,
      }
    }

    /// Set the timeout for requests
    #[ must_use ]
    pub fn timeout( mut self, timeout : Duration ) -> Self
    {
      self.timeout = Some( timeout );
      self
    }

    /// Set the API key
    #[ must_use ]
    pub fn api_key< S : Into< String > >( mut self, api_key : S ) -> Self
    {
      self.api_key = Some( api_key.into() );
      self
    }

    /// Set the base URL
    #[ must_use ]
    pub fn base_url< S : Into< String > >( mut self, base_url : S ) -> Self
    {
      self.base_url = Some( base_url.into() );
      self
    }

    /// Build the client from environment variables
    ///
    /// # Errors
    ///
    /// Returns an error if environment variables are missing or invalid
    pub fn build_from_env( self ) -> AnthropicResult< SyncClient >
    {
      let runtime = SyncRuntime::new()?;

      let mut config = ClientConfig::recommended();

      if let Some( timeout ) = self.timeout
      {
        config.request_timeout = timeout;
      }

      if let Some( base_url ) = self.base_url
      {
        config.base_url = base_url;
      }

      let secret = Secret::load_from_env( "ANTHROPIC_API_KEY" )
        .map_err( |e| AnthropicError::InvalidRequest( e.to_string() ) )?;

      let inner = Client::with_config( secret, config );

      Ok( SyncClient
      {
        inner,
        runtime,
      } )
    }

    /// Build the client with explicit API key
    ///
    /// # Errors
    ///
    /// Returns an error if the API key is invalid or runtime creation fails
    pub fn build( self, api_key : &str ) -> AnthropicResult< SyncClient >
    {
      let runtime = SyncRuntime::new()?;

      let mut config = ClientConfig::recommended();

      if let Some( timeout ) = self.timeout
      {
        config.request_timeout = timeout;
      }

      if let Some( base_url ) = self.base_url
      {
        config.base_url = base_url;
      }

      let secret = Secret::new( api_key.to_string() )
        .map_err( |e| AnthropicError::InvalidRequest( e.to_string() ) )?;

      let inner = Client::with_config( secret, config );

      Ok( SyncClient
      {
        inner,
        runtime,
      } )
    }
  }

  impl Default for SyncClientBuilder
  {
    fn default() -> Self
    {
      Self::new()
    }
  }

  // Helper methods for CreateMessageRequest to match test expectations
  impl CreateMessageRequest
  {
    /// Create a new message request with model
    pub fn new( model : &str ) -> Self
    {
      Self
      {
        model : model.to_string(),
        max_tokens : 100, // Default
        messages : vec![],
        system : None,
        temperature : None,
        stream : None,
        #[ cfg( feature = "tools" ) ]
        tools : None,
        #[ cfg( feature = "tools" ) ]
        tool_choice : None,
      }
    }

    /// Add a user message
    pub fn add_user_message( &mut self, content : &str )
    {
      self.messages.push( Message::user( content ) );
    }

    /// Add a message
    pub fn add_message( &mut self, message : Message )
    {
      self.messages.push( message );
    }

    /// Set max tokens
    pub fn set_max_tokens( &mut self, max_tokens : u32 )
    {
      self.max_tokens = max_tokens;
    }

    /// Set system prompt
    pub fn set_system( &mut self, system : &str )
    {
      self.system = Some( vec![ crate::SystemContent::text( system ) ] );
    }

    /// Set temperature
    pub fn set_temperature( &mut self, temperature : f32 )
    {
      self.temperature = Some( temperature );
    }
  }

  // Helper methods for Message
  impl Message
  {
    /// Create assistant message from content blocks
    pub fn assistant_from_content( content : &[Content] ) -> Self
    {
      Self
      {
        role : Role::Assistant,
        content : content.to_owned(),
        cache_control : None,
      }
    }
  }

  // Implement async to sync conversion helpers
  impl SyncClient
  {
    /// Convert async client to sync (convenience method)
    ///
    /// # Errors
    ///
    /// Returns an error if runtime creation fails
    pub fn from_async( client : Client ) -> AnthropicResult< Self >
    {
      let runtime = SyncRuntime::new()?;

      Ok( Self
      {
        inner : client,
        runtime,
      } )
    }

    /// Get the underlying async client for advanced usage
    pub fn async_client( &self ) -> &Client
    {
      &self.inner
    }

    /// Get the runtime for advanced async interop
    pub fn runtime( &self ) -> &SyncRuntime
    {
      &self.runtime
    }
  }
}

#[ cfg( all( feature = "sync-api", feature = "streaming" ) ) ]
crate::mod_interface!
{
  exposed use
  {
    SyncClient,
    SyncRuntime,
    SyncClientBuilder,
    SyncStreamIterator,
  };
}

#[ cfg( all( feature = "sync-api", not( feature = "streaming" ) ) ) ]
crate::mod_interface!
{
  exposed use
  {
    SyncClient,
    SyncRuntime,
    SyncClientBuilder,
  };
}

#[ cfg( not( feature = "sync-api" ) ) ]
crate::mod_interface!
{
  // Empty when sync-api feature is disabled
}