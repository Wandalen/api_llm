//! Synchronous API wrapper for the `OpenAI` client.
//!
//! This module provides blocking wrappers around the async `OpenAI` API,
//! allowing users to use the client in synchronous contexts without
//! dealing with async/await directly.

/// Define a private namespace for all its items.
mod private
{
  use crate::
  {
    Client,
    client ::ClientApiAccessors,
    error ::{ Result, OpenAIError },
    environment ::{ OpenaiEnvironment, EnvironmentInterface },
  };
  use crate::components::
  {
    chat_shared ::{ ChatCompletionRequest, CreateChatCompletionResponse, ChatCompletionStreamResponse },
    embeddings_request ::CreateEmbeddingRequest,
    embeddings ::CreateEmbeddingResponse,
    models ::{ ListModelsResponse, Model },
  };
  use std::sync::
  {
    Arc,
    mpsc ::{ self, Receiver },
  };
  use core::sync::atomic::{ AtomicBool, Ordering };
  use std::thread::{ self, JoinHandle };
  use core::time::Duration;
  use tokio::runtime::Runtime;

  /// Synchronous wrapper around the async `OpenAI` client.
  ///
  /// This client provides blocking methods that internally use a Tokio runtime
  /// to execute async operations synchronously.
  #[ derive( Debug ) ]
  pub struct SyncClient< E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    async_client : Client< E >,
    runtime : Arc< Runtime >,
  }

  impl< E > SyncClient< E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Creates a new synchronous client with an embedded runtime.
    ///
    /// # Arguments
    /// - `environment`: The `OpenAI` environment configuration
    ///
    /// # Errors
    /// Returns an error if the client or runtime cannot be created.
    #[ inline ]
    pub fn new( environment : E ) -> Result< Self >
    {
      let runtime = Runtime::new().map_err( |e| OpenAIError::Internal( format!( "Failed to create runtime : {e}" ) ) )?;
      let async_client = Client::build( environment ).map_err( |e| OpenAIError::Internal( format!( "Failed to build async client : {e}" ) ) )?;

      Ok( Self
      {
        async_client,
        runtime : Arc::new( runtime ),
      })
    }

    /// Creates a new synchronous client with an external runtime.
    ///
    /// # Arguments
    /// - `environment`: The `OpenAI` environment configuration
    /// - `runtime`: External Tokio runtime to use
    ///
    /// # Errors
    /// Returns an error if the client cannot be created.
    #[ inline ]
    pub fn with_runtime( environment : E, runtime : Arc< Runtime > ) -> Result< Self >
    {
      let async_client = Client::build( environment ).map_err( |e| OpenAIError::Internal( format!( "Failed to build async client : {e}" ) ) )?;

      Ok( Self
      {
        async_client,
        runtime,
      })
    }

    /// Returns a synchronous embeddings API client.
    #[ inline ]
    pub fn embeddings( &self ) -> SyncEmbeddings< '_, E >
    {
      SyncEmbeddings::new( self )
    }

    /// Returns a synchronous chat API client.
    #[ inline ]
    pub fn chat( &self ) -> SyncChat< '_, E >
    {
      SyncChat::new( self )
    }

    /// Returns a synchronous models API client.
    #[ inline ]
    pub fn models( &self ) -> SyncModels< '_, E >
    {
      SyncModels::new( self )
    }
  }

  /// Synchronous wrapper for the embeddings API.
  #[ derive( Debug ) ]
  pub struct SyncEmbeddings< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client SyncClient< E >,
  }

  impl< 'client, E > SyncEmbeddings< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Creates a new synchronous embeddings client.
    #[ inline ]
    pub fn new( client : &'client SyncClient< E > ) -> Self
    {
      Self { client }
    }

    /// Creates an embedding synchronously.
    ///
    /// # Arguments
    /// - `request`: The embedding request
    ///
    /// # Errors
    /// Returns an error if the request fails.
    #[ inline ]
    pub fn create( &self, request : CreateEmbeddingRequest ) -> Result< CreateEmbeddingResponse >
    {
      self.client.runtime.block_on( async {
        self.client.async_client.embeddings().create( request ).await
      })
    }

  }

  /// Synchronous wrapper for the chat API.
  #[ derive( Debug ) ]
  pub struct SyncChat< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client SyncClient< E >,
  }

  impl< 'client, E > SyncChat< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Creates a new synchronous chat client.
    #[ inline ]
    pub fn new( client : &'client SyncClient< E > ) -> Self
    {
      Self { client }
    }

    /// Creates a chat completion synchronously.
    ///
    /// # Arguments
    /// - `request`: The chat completion request
    ///
    /// # Errors
    /// Returns an error if the request fails.
    #[ inline ]
    pub fn create( &self, request : ChatCompletionRequest ) -> Result< CreateChatCompletionResponse >
    {
      self.client.runtime.block_on( async {
        self.client.async_client.chat().create( request ).await
      })
    }

    /// Creates a streaming chat completion synchronously.
    ///
    /// Returns an iterator that yields chat completion chunks as they arrive.
    /// This method bridges async streaming to synchronous iteration.
    ///
    /// # Arguments
    /// - `request`: The chat completion request with stream enabled
    ///
    /// # Errors
    /// Returns an error if the stream cannot be created.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use api_openai::{ SyncClient, environment::OpenaiEnvironmentImpl };
    /// # use api_openai::components::chat_shared::ChatCompletionRequest;
    /// # fn main() -> Result< (), Box< dyn std::error::Error > > {
    /// # let environment = OpenaiEnvironmentImpl::build(
    /// #   api_openai::secret::Secret::new_unchecked("test".to_string()),
    /// #   None, None, "http://test".to_string(), "ws://test".to_string()
    /// # )?;
    /// # let client = SyncClient::new( environment )?;
    /// # let request = ChatCompletionRequest::former()
    /// #   .model( "gpt-4".to_string() )
    /// #   .messages( vec![] )
    /// #   .form();
    /// let stream = client.chat().create_stream( request )?;
    /// for chunk in stream {
    ///   match chunk {
    ///     Ok(response) =>
    ///     {
    ///       if let Some(choice) = response.choices.first() {
    ///         if let Some(content) = &choice.delta.content {
    ///           print!("{}", content);
    ///         }
    ///       }
    ///     }
    ///     Err(e) =>
    ///     {
    ///       eprintln!("Stream error : {}", e);
    ///       break;
    ///     }
    ///   }
    /// }
    /// # Ok( () )
    /// # }
    /// ```
    #[ inline ]
    pub fn create_stream( &self, request : ChatCompletionRequest ) -> Result< SyncStreamIterator< ChatCompletionStreamResponse > >
    {
      // Start the async stream in the runtime and get the receiver
      let runtime = self.client.runtime.clone();
      let async_receiver = runtime.block_on( async {
        self.client.async_client.chat().create_stream( request ).await
      })?;

      SyncStreamIterator::from_tokio_receiver( async_receiver, StreamConfig::default() )
    }

    /// Creates a streaming chat completion with custom configuration.
    ///
    /// # Arguments
    /// - `request`: The chat completion request with stream enabled
    /// - `config`: Stream configuration including timeout and cancellation
    ///
    /// # Errors
    /// Returns an error if the stream cannot be created.
    #[ inline ]
    pub fn create_stream_with_config(
      &self,
      request : ChatCompletionRequest,
      config : StreamConfig
    ) -> Result< SyncStreamIterator< ChatCompletionStreamResponse > >
    {
      // Start the async stream in the runtime and get the receiver
      let runtime = self.client.runtime.clone();
      let async_receiver = runtime.block_on( async {
        self.client.async_client.chat().create_stream( request ).await
      })?;

      SyncStreamIterator::from_tokio_receiver( async_receiver, config )
    }

    /// Creates a streaming chat completion and collects the full response.
    ///
    /// This is a convenience method that consumes the entire stream and
    /// concatenates all content chunks into a single string.
    ///
    /// # Arguments
    /// - `request`: The chat completion request with stream enabled
    ///
    /// # Errors
    /// Returns an error if the stream fails or cannot be fully consumed.
    #[ inline ]
    pub fn create_stream_collect( &self, request : ChatCompletionRequest ) -> Result< String >
    {
      let stream = self.create_stream( request )?;
      let mut result = String::new();

      for chunk in stream
      {
        match chunk
        {
          Ok( response ) =>
          {
            if let Some( choice ) = response.choices.first()
            {
              if let Some( content ) = &choice.delta.content
              {
                result.push_str( content );
              }
            }
          }
          Err( e ) => return Err( e ),
        }
      }

      Ok( result )
    }
  }

  /// Synchronous wrapper for the models API.
  #[ derive( Debug ) ]
  pub struct SyncModels< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client SyncClient< E >,
  }

  impl< 'client, E > SyncModels< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Creates a new synchronous models client.
    #[ inline ]
    pub fn new( client : &'client SyncClient< E > ) -> Self
    {
      Self { client }
    }

    /// Lists available models synchronously.
    ///
    /// # Errors
    /// Returns an error if the request fails.
    #[ inline ]
    pub fn list( &self ) -> Result< ListModelsResponse >
    {
      self.client.runtime.block_on( async {
        self.client.async_client.models().list().await
      })
    }

    /// Retrieves a specific model synchronously.
    ///
    /// # Arguments
    /// - `model`: The model ID to retrieve
    ///
    /// # Errors
    /// Returns an error if the request fails.
    #[ inline ]
    pub fn retrieve( &self, model : &str ) -> Result< Model >
    {
      self.client.runtime.block_on( async {
        self.client.async_client.models().retrieve( model ).await
      })
    }
  }

  /// Configuration for synchronous streaming operations.
  ///
  /// Controls behavior of streaming operations including timeouts,
  /// buffering, and cancellation.
  #[ derive( Debug, Clone ) ]
  pub struct StreamConfig
  {
    /// Timeout for the entire stream operation
    pub timeout : Option< Duration >,
    /// Buffer size for the internal channel
    pub buffer_size : usize,
    /// Cancellation token to stop streaming early
    pub cancellation_token : Option< Arc< AtomicBool > >,
  }

  impl Default for StreamConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        timeout : Some( Duration::from_secs( 300 ) ), // 5 minutes default
        buffer_size : 100,
        cancellation_token : None,
      }
    }
  }

  /// Synchronous iterator that bridges async receivers to sync iteration.
  ///
  /// This iterator consumes an async receiver in a background thread and
  /// provides synchronous access to the items through the Iterator trait.
  pub struct SyncStreamIterator< T >
  where
    T: Send + 'static,
  {
    /// Receiver for stream items
    receiver : Receiver< Result< T > >,
    /// Handle to the background thread
    handle : Option< JoinHandle< () > >,
    /// Cancellation token for early termination
    cancellation_token : Arc< AtomicBool >,
    /// Whether the stream has ended
    ended : bool,
  }

  impl< T > core::fmt::Debug for SyncStreamIterator< T >
  where
    T: Send + 'static,
  {
    #[ inline ]
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      f.debug_struct( "SyncStreamIterator" )
        .field( "ended", &self.ended )
        .field( "cancelled", &self.is_cancelled() )
        .field( "has_handle", &self.handle.is_some() )
        .field( "receiver", &"< receiver >" )
        .field( "cancellation_token", &"< token >" )
        .finish()
    }
  }

  impl< T > SyncStreamIterator< T >
  where
    T: Send + 'static,
  {
    /// Creates a new sync stream iterator from a Tokio mpsc receiver.
    ///
    /// # Arguments
    /// - `tokio_receiver`: The Tokio mpsc receiver to bridge to sync iteration
    /// - `config`: Stream configuration
    ///
    /// # Errors
    /// Returns an error if the background thread cannot be started.
    #[ inline ]
    pub fn from_tokio_receiver(
      mut tokio_receiver : tokio::sync::mpsc::Receiver< Result< T > >,
      config : StreamConfig,
    ) -> Result< Self >
    {
      let ( sender, receiver ) = mpsc::channel();
      let cancellation_token = config.cancellation_token.unwrap_or_else( || Arc::new( AtomicBool::new( false ) ) );
      let cancel_clone = cancellation_token.clone();

      // Create a runtime for the background thread
      let runtime = Arc::new( Runtime::new().map_err( | e | OpenAIError::Internal( format!( "Failed to create runtime : {e}" ) ) )? );

      let handle = thread::spawn( move || {
        runtime.block_on( async move {
          loop
          {
            // Check for cancellation
            if cancel_clone.load( Ordering::Relaxed )
            {
              break;
            }

            match tokio_receiver.recv().await
            {
              Some( result ) =>
              {
                if sender.send( result ).is_err()
                {
                  // Receiver has been dropped, stop streaming
                  break;
                }
              }
              None =>
              {
                // Channel closed, end of stream
                break;
              }
            }
          }
        });
      });

      Ok( SyncStreamIterator
      {
        receiver,
        handle : Some( handle ),
        cancellation_token,
        ended : false,
      })
    }

    /// Cancels the stream operation.
    ///
    /// This will signal the background thread to stop processing
    /// and cause the iterator to end on the next call to `next()`.
    #[ inline ]
    pub fn cancel( &self )
    {
      self.cancellation_token.store( true, Ordering::Relaxed );
    }

    /// Checks if the stream has been cancelled.
    #[ inline ]
    #[ must_use ]
    pub fn is_cancelled( &self ) -> bool
    {
      self.cancellation_token.load( Ordering::Relaxed )
    }
  }

  impl< T > Iterator for SyncStreamIterator< T >
  where
    T: Send + 'static,
  {
    type Item = Result< T >;

    #[ inline ]
    fn next( &mut self ) -> Option< Self::Item >
    {
      if self.ended
      {
        return None;
      }

      // Check for cancellation
      if self.is_cancelled()
      {
        self.ended = true;
        return None;
      }

      if let Ok( item ) = self.receiver.recv()
      {
        Some( item )
      }
      else
      {
        // Channel is closed, stream has ended
        self.ended = true;
        None
      }
    }
  }

  impl< T > Drop for SyncStreamIterator< T >
  where
    T: Send + 'static,
  {
    #[ inline ]
    fn drop( &mut self )
    {
      // Cancel the operation
      self.cancel();

      // Wait for the background thread to finish
      if let Some( handle ) = self.handle.take()
      {
        let _ = handle.join();
      }
    }
  }

  #[ cfg( test ) ]
  mod tests
  {
    use super::*;

    #[ test ]
    fn test_stream_config_default()
    {
      let config = StreamConfig::default();
      assert!( config.timeout.is_some() );
      assert_eq!( config.buffer_size, 100 );
      assert!( config.cancellation_token.is_none() );
    }

    #[ test ]
    fn test_stream_config_custom()
    {
      let cancel_token = Arc::new( AtomicBool::new( false ) );
      let config = StreamConfig
      {
        timeout : Some( Duration::from_secs( 60 ) ),
        buffer_size : 50,
        cancellation_token : Some( cancel_token.clone() ),
      };

      assert_eq!( config.timeout, Some( Duration::from_secs( 60 ) ) );
      assert_eq!( config.buffer_size, 50 );
      assert!( config.cancellation_token.is_some() );
    }

    #[ test ]
    fn test_cancellation_token_behavior()
    {
      let token = Arc::new( AtomicBool::new( false ) );
      assert!( !token.load( Ordering::Relaxed ) );

      token.store( true, Ordering::Relaxed );
      assert!( token.load( Ordering::Relaxed ) );
    }
  }

} // end mod private

crate ::mod_interface!
{
  exposed use
  {
    SyncClient,
    SyncEmbeddings,
    SyncChat,
    SyncModels,
    SyncStreamIterator,
    StreamConfig,
  };
}