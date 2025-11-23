//! Synchronous API wrapper for Ollama client.

#[ cfg( feature = "sync_api" ) ]
mod private
{
  use core::time::Duration;
  use std::sync::Arc;
  use error_tools::untyped::{ format_err, Result as OllamaResult };
  use crate::client::OllamaClient;

  /// Synchronous Ollama client that wraps async operations
  #[ derive( Clone ) ]
  pub struct SyncOllamaClient
  {
    async_client : OllamaClient,
    runtime : Arc< tokio::runtime::Runtime >,
    config : SyncApiConfig,
  }

  /// Configuration for synchronous API operations
  #[ derive( Debug, Clone ) ]
  pub struct SyncApiConfig
  {
    /// Base URL for the Ollama server
    pub base_url : String,
    /// Request timeout duration
    pub timeout : Duration,
    /// Number of threads in the runtime pool
    pub thread_pool_size : usize,
    /// Whether to enable HTTP keepalive
    pub enable_keepalive : bool,
  }

  /// Runtime manager for sync operations
  #[ derive( Debug ) ]
  pub struct SyncRuntimeManager
  {
    thread_count : usize,
    #[ allow( dead_code ) ]
    runtime : Arc< tokio::runtime::Runtime >,
  }

  /// Builder for SyncApiConfig
  #[ derive( Debug ) ]
  pub struct SyncApiConfigBuilder
  {
    base_url : Option< String >,
    timeout : Option< Duration >,
    thread_pool_size : Option< usize >,
    enable_keepalive : Option< bool >,
  }

  /// Synchronous wrapper around async chat stream
  #[ cfg( feature = "streaming" ) ]
  pub struct SyncChatStream
  {
    runtime : Arc< tokio::runtime::Runtime >,
    inner : std::pin::Pin< Box< dyn futures_core::Stream< Item = OllamaResult< crate::ChatResponse > > + Send > >,
  }

  #[ cfg( feature = "streaming" ) ]
  impl std::fmt::Debug for SyncChatStream
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      f.debug_struct( "SyncChatStream" ).finish_non_exhaustive()
    }
  }

  /// Synchronous wrapper around async generate stream
  #[ cfg( feature = "streaming" ) ]
  pub struct SyncGenerateStream
  {
    runtime : Arc< tokio::runtime::Runtime >,
    inner : std::pin::Pin< Box< dyn futures_core::Stream< Item = OllamaResult< crate::GenerateResponse > > + Send > >,
  }

  #[ cfg( feature = "streaming" ) ]
  impl std::fmt::Debug for SyncGenerateStream
  {
    fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
    {
      f.debug_struct( "SyncGenerateStream" ).finish_non_exhaustive()
    }
  }

  #[ cfg( feature = "streaming" ) ]
  impl SyncChatStream
  {
    /// Create new sync chat stream from async stream
    #[ inline ]
    fn new( runtime : Arc< tokio::runtime::Runtime >, stream : std::pin::Pin< Box< dyn futures_core::Stream< Item = OllamaResult< crate::ChatResponse > > + Send > > ) -> Self
    {
      Self { runtime, inner : stream }
    }
  }

  #[ cfg( feature = "streaming" ) ]
  impl SyncGenerateStream
  {
    /// Create new sync generate stream from async stream
    #[ inline ]
    fn new( runtime : Arc< tokio::runtime::Runtime >, stream : std::pin::Pin< Box< dyn futures_core::Stream< Item = OllamaResult< crate::GenerateResponse > > + Send > > ) -> Self
    {
      Self { runtime, inner : stream }
    }
  }

  #[ cfg( feature = "streaming" ) ]
  impl Iterator for SyncChatStream
  {
    type Item = OllamaResult< crate::ChatResponse >;

    #[ inline ]
    fn next( &mut self ) -> Option< Self::Item >
    {
      use futures_util::StreamExt;
      self.runtime.block_on( self.inner.next() )
    }
  }

  #[ cfg( feature = "streaming" ) ]
  impl Iterator for SyncGenerateStream
  {
    type Item = OllamaResult< crate::GenerateResponse >;

    #[ inline ]
    fn next( &mut self ) -> Option< Self::Item >
    {
      use futures_util::StreamExt;
      self.runtime.block_on( self.inner.next() )
    }
  }

  impl SyncOllamaClient
  {
    /// Create a new synchronous Ollama client
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid or the runtime cannot be created
    #[ inline ]
    #[ must_use ]
    pub fn new( base_url : &str, timeout : Duration ) -> OllamaResult< Self >
    {
      let async_client = OllamaClient::new( base_url.to_string(), timeout );
      let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()
        .map_err( | e | format_err!( "Failed to create tokio runtime : {}", e ) )?;
      let config = SyncApiConfig
      {
        base_url : base_url.to_string(),
        timeout,
        thread_pool_size : 4,
        enable_keepalive : true,
      };

      Ok( SyncOllamaClient
      {
        async_client,
        runtime : Arc::new( runtime ),
        config,
      })
    }

    /// Create a new synchronous client with custom configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the runtime cannot be created
    #[ inline ]
    #[ must_use ]
    pub fn with_config( config : SyncApiConfig ) -> OllamaResult< Self >
    {
      let async_client = OllamaClient::new( config.base_url.clone(), config.timeout );

      let mut builder = tokio::runtime::Builder::new_current_thread();
      builder.enable_all();

      let runtime = builder.build()
        .map_err( | e | format_err!( "Failed to create tokio runtime : {}", e ) )?;

      Ok( SyncOllamaClient {
        async_client,
        runtime : Arc::new( runtime ),
        config,
      })
    }

    /// Create sync client from existing async client
    ///
    /// # Errors
    ///
    /// Returns an error if the runtime cannot be created
    #[ inline ]
    pub fn from_async( async_client : OllamaClient ) -> OllamaResult< Self >
    {
      let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()
        .map_err( | e | format_err!( "Failed to create tokio runtime : {}", e ) )?;
      let config = SyncApiConfig::default();

      Ok( SyncOllamaClient {
        async_client,
        runtime : Arc::new( runtime ),
        config,
      })
    }

    /// Get the base URL
    #[ inline ]
    #[ must_use ]
    pub fn base_url( &self ) -> &str
    {
      &self.config.base_url
    }

    /// Get the timeout
    #[ inline ]
    #[ must_use ]
    pub fn timeout( &self ) -> Duration
    {
      self.config.timeout
    }

    /// Get the thread pool size
    #[ inline ]
    #[ must_use ]
    pub fn thread_pool_size( &self ) -> usize
    {
      self.config.thread_pool_size
    }

    /// Check if keepalive is enabled
    #[ inline ]
    #[ must_use ]
    pub fn keepalive_enabled( &self ) -> bool
    {
      self.config.enable_keepalive
    }

    /// Send a synchronous chat completion request
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails
    #[ inline ]
    pub fn chat( &mut self, request : crate::ChatRequest ) -> OllamaResult< crate::ChatResponse >
    {
      let runtime = Arc::clone( &self.runtime );
      runtime.block_on( self.async_client.chat( request ) )
    }

    /// Send a synchronous generation request
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails
    #[ inline ]
    pub fn generate( &mut self, request : crate::GenerateRequest ) -> OllamaResult< crate::GenerateResponse >
    {
      let runtime = Arc::clone( &self.runtime );
      runtime.block_on( self.async_client.generate( request ) )
    }

    /// Get available models synchronously
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails
    #[ inline ]
    pub fn list_models( &mut self ) -> OllamaResult< crate::TagsResponse >
    {
      let runtime = Arc::clone( &self.runtime );
      runtime.block_on( self.async_client.list_models() )
    }

    /// Delete a model synchronously
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails
    #[ cfg( feature = "model_details" ) ]
    #[ inline ]
    pub fn delete_model( &mut self, request : crate::DeleteModelRequest ) -> OllamaResult< () >
    {
      let runtime = Arc::clone( &self.runtime );
      runtime.block_on( self.async_client.delete_model( request ) )
    }

    /// Get embeddings synchronously
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails
    #[ cfg( feature = "embeddings" ) ]
    #[ inline ]
    pub fn embeddings( &mut self, request : crate::EmbeddingsRequest ) -> OllamaResult< crate::EmbeddingsResponse >
    {
      let runtime = Arc::clone( &self.runtime );
      runtime.block_on( self.async_client.embeddings( request ) )
    }

    /// Count tokens in a request
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails
    #[ cfg( feature = "count_tokens" ) ]
    #[ inline ]
    pub fn count_tokens( &mut self, request : crate::TokenCountRequest ) -> OllamaResult< crate::TokenCountResponse >
    {
      let runtime = Arc::clone( &self.runtime );
      runtime.block_on( self.async_client.count_tokens( request ) )
    }

    /// Cache content for reuse
    ///
    /// # Errors
    ///
    /// Returns an error if caching fails
    #[ cfg( feature = "cached_content" ) ]
    #[ inline ]
    pub fn cache_content( &mut self, request : crate::CachedContentRequest ) -> OllamaResult< crate::CachedContentResponse >
    {
      let runtime = Arc::clone( &self.runtime );
      runtime.block_on( self.async_client.cache_content( request ) )
    }

    /// Invalidate cached content
    ///
    /// # Errors
    ///
    /// Returns an error if invalidation fails
    #[ cfg( feature = "cached_content" ) ]
    #[ inline ]
    pub fn invalidate_cache( &mut self, request : crate::CacheInvalidationRequest ) -> OllamaResult< crate::CacheInvalidationResponse >
    {
      let runtime = Arc::clone( &self.runtime );
      runtime.block_on( self.async_client.invalidate_cache( request ) )
    }

    /// Get cache performance metrics
    ///
    /// # Errors
    ///
    /// Returns an error if metrics retrieval fails
    #[ cfg( feature = "cached_content" ) ]
    #[ inline ]
    pub fn cache_metrics( &mut self ) -> OllamaResult< crate::CachePerformanceMetrics >
    {
      let runtime = Arc::clone( &self.runtime );
      runtime.block_on( self.async_client.cache_metrics() )
    }

    /// Send chat request with streaming response
    ///
    /// Returns a blocking iterator that yields chat response chunks
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails
    #[ cfg( feature = "streaming" ) ]
    #[ inline ]
    pub fn chat_stream( &mut self, request : crate::ChatRequest ) -> OllamaResult< SyncChatStream >
    {
      let runtime = Arc::clone( &self.runtime );
      let async_stream = runtime.block_on( self.async_client.chat_stream( request ) )?;
      Ok( SyncChatStream::new( runtime, async_stream ) )
    }

    /// Send generate request with streaming response
    ///
    /// Returns a blocking iterator that yields generate response chunks
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails
    #[ cfg( feature = "streaming" ) ]
    #[ inline ]
    pub fn generate_stream( &mut self, request : crate::GenerateRequest ) -> OllamaResult< SyncGenerateStream >
    {
      let runtime = Arc::clone( &self.runtime );
      let async_stream = runtime.block_on( self.async_client.generate_stream( request ) )?;
      Ok( SyncGenerateStream::new( runtime, async_stream ) )
    }

    /// Get configuration
    #[ inline ]
    #[ must_use ]
    pub fn config( &self ) -> &SyncApiConfig
    {
      &self.config
    }
  }

  impl core::fmt::Debug for SyncOllamaClient
  {
    #[ inline ]
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      f.debug_struct( "SyncOllamaClient" )
        .field( "config", &self.config )
        .finish()
    }
  }

  impl SyncApiConfig
  {
    /// Create a new configuration builder
    #[ inline ]
    #[ must_use ]
    pub fn builder() -> SyncApiConfigBuilder
    {
      SyncApiConfigBuilder::default()
    }
  }

  impl Default for SyncApiConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        base_url : "http://localhost:11434".to_string(),
        timeout : OllamaClient::recommended_timeout_fast(),
        thread_pool_size : 4,
        enable_keepalive : true,
      }
    }
  }

  impl SyncRuntimeManager
  {
    /// Create a new sync runtime manager
    #[ inline ]
    #[ must_use ]
    pub fn new( thread_count : usize ) -> Self
    {
      let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect( "Failed to create runtime" );

      Self
      {
        thread_count,
        runtime : Arc::new( runtime ),
      }
    }

    /// Get the thread count
    #[ inline ]
    #[ must_use ]
    pub fn thread_count( &self ) -> usize
    {
      self.thread_count
    }

    /// Check if the runtime is healthy
    #[ inline ]
    #[ must_use ]
    pub fn is_healthy( &self ) -> bool
    {
      true // Simple implementation for now
    }

    /// Spawn a blocking task
    ///
    /// # Errors
    ///
    /// Returns an error if the task cannot be spawned
    #[ inline ]
    pub fn spawn_blocking< F, R >( &self, f : F ) -> OllamaResult< std::thread::JoinHandle< R > >
    where
      F : FnOnce() -> R + Send + 'static,
      R : Send + 'static,
    {
      let handle = std::thread::spawn( f );
      Ok( handle )
    }

    /// Execute a future synchronously on this runtime
    #[ inline ]
    pub fn block_on< F : std::future::Future >( &self, future : F ) -> F::Output
    {
      self.runtime.block_on( future )
    }

    /// Spawn a task on the runtime and get a handle
    #[ inline ]
    pub fn spawn< F >( &self, future : F ) -> tokio::task::JoinHandle< F::Output >
    where
      F : std::future::Future + Send + 'static,
      F::Output : Send + 'static,
    {
      self.runtime.spawn( future )
    }
  }

  impl SyncApiConfigBuilder
  {
    /// Create a new config builder
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        base_url : None,
        timeout : None,
        thread_pool_size : None,
        enable_keepalive : None,
      }
    }

    /// Set the base URL
    #[ inline ]
    #[ must_use ]
    pub fn base_url( mut self, url : &str ) -> Self
    {
      self.base_url = Some( url.to_string() );
      self
    }

    /// Set the timeout
    #[ inline ]
    #[ must_use ]
    pub fn timeout( mut self, timeout : Duration ) -> Self
    {
      self.timeout = Some( timeout );
      self
    }

    /// Set the thread pool size
    #[ inline ]
    #[ must_use ]
    pub fn thread_pool_size( mut self, size : usize ) -> Self
    {
      self.thread_pool_size = Some( size );
      self
    }

    /// Enable or disable keepalive
    #[ inline ]
    #[ must_use ]
    pub fn enable_keepalive( mut self, enable : bool ) -> Self
    {
      self.enable_keepalive = Some( enable );
      self
    }

    /// Build the configuration
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing
    #[ inline ]
    pub fn build( self ) -> OllamaResult< SyncApiConfig >
    {
      let base_url = self.base_url
        .ok_or_else( || format_err!( "base_url is required" ) )?;
      let timeout = self.timeout.unwrap_or( Duration::from_secs( 30 ) );
      let thread_pool_size = self.thread_pool_size.unwrap_or( 4 );
      let enable_keepalive = self.enable_keepalive.unwrap_or( true );

      Ok( SyncApiConfig {
        base_url,
        timeout,
        thread_pool_size,
        enable_keepalive,
      })
    }
  }

  impl Default for SyncApiConfigBuilder
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }
}

#[ cfg( feature = "sync_api" ) ]
crate ::mod_interface!
{
  exposed use private::SyncOllamaClient;
  exposed use private::SyncApiConfig;
  exposed use private::SyncRuntimeManager;
  exposed use private::SyncApiConfigBuilder;
  #[ cfg( feature = "streaming" ) ]
  exposed use private::SyncChatStream;
  #[ cfg( feature = "streaming" ) ]
  exposed use private::SyncGenerateStream;
}
