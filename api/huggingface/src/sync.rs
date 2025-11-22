//! Synchronous API wrappers for `HuggingFace` client
//!
//! This module provides blocking wrappers around the async `HuggingFace` API,
//! allowing usage in synchronous contexts without explicit async/await.
//!
//! # Features
//!
//! - **Blocking Operations**: All async operations wrapped as blocking calls
//! - **Thread Safety**: Client is Send + Sync and can be shared across threads
//! - **Runtime Management**: Tokio runtime automatically managed internally
//! - **Zero Magic**: Simple blocking wrappers with no automatic behavior
//!
//! # Usage Example
//!
//! ```no_run
//! use api_huggingface::sync::SyncClient;
//!
//! let client = SyncClient::new( "your_api_key".to_string() )
//!   .expect( "Failed to create client" );
//!
//! // Blocking call - no async/await needed
//! let result = client.inference().create(
//!   "What is 2+2?",
//!   "mistralai/Mistral-7B-Instruct-v0.3"
//! );
//!
//! match result
//! {
//!   Ok( response ) => println!( "Response : {}", response.extract_text_or_default( "" ) ),
//!   Err( e ) => eprintln!( "Error : {}", e ),
//! }
//! ```

use crate::
{
  Client,
  environment::HuggingFaceEnvironmentImpl,
  secret::Secret,
  error::{ Result, HuggingFaceError },
  components::
  {
  input::InferenceParameters,
  inference_shared::InferenceResponse,
  },
  token_counter::{ TokenCounter, CountingStrategy },
  cache::{ Cache, CacheConfig },
};

use std::sync::Arc;
use tokio::sync::mpsc;

/// Synchronous streaming iterator
///
/// Wraps an async stream and provides blocking iteration over stream tokens.
/// Each call to `next()` blocks until the next token is available or the stream ends.
///
/// # Examples
///
/// ```no_run
/// # use api_huggingface::{sync::SyncClient, components::input::InferenceParameters};
/// # fn example() -> Result< (), Box< dyn std::error::Error > > {
/// # let client = SyncClient::new("hf_test".to_string())?;
/// # let params = InferenceParameters::new();
/// let stream = client.inference().create_stream( "Hello", "model", params )?;
///
/// for token_result in stream
/// {
///   match token_result
///   {
///     Ok( token ) => print!( "{token}" ),
///     Err( e ) => eprintln!( "Error : {e}" ),
///   }
/// }
/// # Ok(())
/// # }
/// ```
pub struct SyncStream
{
  receiver : mpsc::Receiver< Result< String > >,
  runtime : Arc< tokio::runtime::Runtime >,
}

impl Iterator for SyncStream
{
  type Item = Result< String >;

  #[ inline ]
  fn next( &mut self ) -> Option< Self::Item >
  {
  self.runtime.block_on( async
  {
      self.receiver.recv().await
  } )
  }
}

impl core::fmt::Debug for SyncStream
{
  #[ inline ]
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
  f.debug_struct( "SyncStream" )
      .field( "receiver", &"< Receiver >" )
      .field( "runtime", &"< Runtime >" )
      .finish()
  }
}

/// Synchronous cache wrapper
///
/// Wraps an async `Cache` and provides blocking methods for cache operations.
///
/// # Examples
///
/// ```no_run
/// # use api_huggingface::sync::SyncClient;
/// # fn example() -> Result< (), Box< dyn std::error::Error > > {
/// let client = SyncClient::new( "api_key".to_string() )?;
/// let cache = client.cache();
///
/// cache.insert( "key", "value", None );
/// if let Some( value ) = cache.get( &"key" )
/// {
///   println!( "Cached : {value}" );
/// }
/// # Ok(())
/// # }
/// ```
pub struct SyncCache< K, V >
{
  cache : Cache< K, V >,
  runtime : Arc< tokio::runtime::Runtime >,
}

impl< K, V > SyncCache< K, V >
where
  K : Eq + core::hash::Hash + Clone,
  V : Clone,
{
  /// Insert a value into the cache
  #[ inline ]
  pub fn insert( &self, key : K, value : V, ttl : Option< core::time::Duration > )
  {
  let cache = self.cache.clone();
  self.runtime.block_on( async move
  {
      cache.insert( key, value, ttl ).await;
  } );
  }

  /// Get a value from the cache
  #[ inline ]
  pub fn get( &self, key : &K ) -> Option< V >
  {
  let cache = self.cache.clone();
  let key = key.clone();
  self.runtime.block_on( async move
  {
      cache.get( &key ).await
  } )
  }

  /// Check if key exists in cache
  #[ inline ]
  pub fn contains_key( &self, key : &K ) -> bool
  {
  let cache = self.cache.clone();
  let key = key.clone();
  self.runtime.block_on( async move
  {
      cache.contains_key( &key ).await
  } )
  }

  /// Remove entry from cache
  #[ inline ]
  pub fn remove( &self, key : &K ) -> Option< V >
  {
  let cache = self.cache.clone();
  let key = key.clone();
  self.runtime.block_on( async move
  {
      cache.remove( &key ).await
  } )
  }

  /// Clear all entries from cache
  #[ inline ]
  pub fn clear( &self )
  {
  let cache = self.cache.clone();
  self.runtime.block_on( async move
  {
      cache.clear().await;
  } );
  }

  /// Get current cache size
  #[ inline ]
  #[ must_use ]
  pub fn len( &self ) -> usize
  {
  let cache = self.cache.clone();
  self.runtime.block_on( async move
  {
      cache.len().await
  } )
  }

  /// Check if cache is empty
  #[ inline ]
  #[ must_use ]
  pub fn is_empty( &self ) -> bool
  {
  let cache = self.cache.clone();
  self.runtime.block_on( async move
  {
      cache.is_empty().await
  } )
  }
}

impl< K, V > core::fmt::Debug for SyncCache< K, V >
{
  #[ inline ]
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
  f.debug_struct( "SyncCache" )
      .field( "cache", &"< Cache >" )
      .field( "runtime", &"< Runtime >" )
      .finish()
  }
}

/// Synchronous wrapper around the async `HuggingFace` client
///
/// This client uses an internal Tokio runtime to execute async operations
/// as blocking calls. The runtime is created once when the client is built
/// and reused for all operations.
///
/// # Thread Safety
///
/// This client is Send + Sync and can be safely shared across threads using Arc.
///
/// # Runtime Management
///
/// The internal Tokio runtime is automatically cleaned up when the client is dropped.
#[ derive( Debug ) ]
pub struct SyncClient
{
  client : Client< HuggingFaceEnvironmentImpl >,
  runtime : Arc< tokio::runtime::Runtime >,
}

impl SyncClient
{
  /// Create a new synchronous client from an API key
  ///
  /// # Arguments
  ///
  /// * `api_key` - `HuggingFace` API key
  ///
  /// # Errors
  ///
  /// Returns error if:
  /// - API key is invalid
  /// - Environment cannot be initialized
  /// - Tokio runtime cannot be created
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use api_huggingface::sync::SyncClient;
  ///
  /// let client = SyncClient::new( "hf_...".to_string() )
  ///   .expect( "Failed to create client" );
  /// ```
  #[ inline ]
  pub fn new( api_key : String ) -> Result< Self >
  {
  let secret = Secret::new( api_key );
  let env = HuggingFaceEnvironmentImpl::build( secret, None )?;
  let client = Client::build( env )?;

  let runtime = tokio::runtime::Runtime::new()
      .map_err( |e| HuggingFaceError::Generic( format!( "Failed to create runtime : {e}" ) ) )?;

  Ok( Self
  {
      client,
      runtime : Arc::new( runtime ),
  } )
  }

  /// Create a synchronous client with a custom base URL
  ///
  /// # Arguments
  ///
  /// * `api_key` - `HuggingFace` API key
  /// * `base_url` - Custom base URL for the API
  ///
  /// # Errors
  ///
  /// Returns error if:
  /// - API key is invalid
  /// - Base URL is invalid
  /// - Environment cannot be initialized
  /// - Tokio runtime cannot be created
  #[ inline ]
  pub fn with_base_url( api_key : String, base_url : String ) -> Result< Self >
  {
  let secret = Secret::new( api_key );
  let env = HuggingFaceEnvironmentImpl::build( secret, Some( base_url ) )?;
  let client = Client::build( env )?;

  let runtime = tokio::runtime::Runtime::new()
      .map_err( |e| HuggingFaceError::Generic( format!( "Failed to create runtime : {e}" ) ) )?;

  Ok( Self
  {
      client,
      runtime : Arc::new( runtime ),
  } )
  }

  /// Get a synchronous inference API interface
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use api_huggingface::sync::SyncClient;
  /// # fn example() -> Result< (), Box< dyn std::error::Error > > {
  /// let client = SyncClient::new( "api_key".to_string() )?;
  /// let _result = client.inference().create( "Hello", "model" );
  /// # Ok(())
  /// # }
  /// ```
  #[ inline ]
  #[ must_use ]
  pub fn inference( &self ) -> SyncInference
  {
  SyncInference
  {
      client : self.client.clone(),
      runtime : Arc::clone( &self.runtime ),
  }
  }

  /// Get a token counter instance
  ///
  /// Returns a new `TokenCounter` with the default estimation strategy.
  /// The token counter is already synchronous, so no runtime wrapping is needed.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use api_huggingface::sync::SyncClient;
  /// # fn example() -> Result< (), Box< dyn std::error::Error > > {
  /// let client = SyncClient::new( "api_key".to_string() )?;
  /// let counter = client.token_counter();
  /// let count = counter.count_tokens( "Hello world" );
  /// println!( "Tokens : {}", count.total );
  /// # Ok(())
  /// # }
  /// ```
  #[ inline ]
  #[ must_use ]
  pub fn token_counter( &self ) -> TokenCounter
  {
  TokenCounter::new( CountingStrategy::Estimation )
  }

  /// Get a token counter with a specific strategy
  ///
  /// # Arguments
  ///
  /// * `strategy` - The counting strategy to use
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use api_huggingface::{sync::SyncClient, token_counter::CountingStrategy};
  /// # fn example() -> Result< (), Box< dyn std::error::Error > > {
  /// let client = SyncClient::new( "api_key".to_string() )?;
  /// let counter = client.token_counter_with_strategy( CountingStrategy::CharacterBased );
  /// let count = counter.count_tokens( "Hello world" );
  /// # let _ = count;
  /// # Ok(())
  /// # }
  /// ```
  #[ inline ]
  #[ must_use ]
  pub fn token_counter_with_strategy( &self, strategy : CountingStrategy ) -> TokenCounter
  {
  TokenCounter::new( strategy )
  }

  /// Create a new cache with default configuration
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use api_huggingface::sync::SyncClient;
  /// # fn example() -> Result< (), Box< dyn std::error::Error > > {
  /// let client = SyncClient::new( "api_key".to_string() )?;
  /// let cache = client.cache::< String, String >();
  ///
  /// cache.insert( "key".to_string(), "value".to_string(), None );
  /// let _value = cache.get( &"key".to_string() );
  /// # Ok(())
  /// # }
  /// ```
  #[ inline ]
  #[ must_use ]
  pub fn cache< K, V >( &self ) -> SyncCache< K, V >
  where
  K : Eq + core::hash::Hash + Clone,
  V : Clone,
  {
  SyncCache
  {
      cache : Cache::new( CacheConfig::default() ),
      runtime : Arc::clone( &self.runtime ),
  }
  }

  /// Create a cache with custom configuration
  ///
  /// # Arguments
  ///
  /// * `config` - Cache configuration
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use api_huggingface::{sync::SyncClient, cache::CacheConfig};
  /// # use std::time::Duration;
  /// # fn example() -> Result< (), Box< dyn std::error::Error > > {
  /// let client = SyncClient::new( "api_key".to_string() )?;
  /// let config = CacheConfig
  /// {
  ///   max_entries : 100,
  ///   default_ttl : Some( Duration::from_secs( 60 ) ),
  /// };
  /// let _cache = client.cache_with_config::< String, String >( config );
  /// # Ok(())
  /// # }
  /// ```
  #[ inline ]
  #[ must_use ]
  pub fn cache_with_config< K, V >( &self, config : CacheConfig ) -> SyncCache< K, V >
  where
  K : Eq + core::hash::Hash + Clone,
  V : Clone,
  {
  SyncCache
  {
      cache : Cache::new( config ),
      runtime : Arc::clone( &self.runtime ),
  }
  }
}

/// Synchronous inference API interface
///
/// Provides blocking wrappers around inference operations.
#[ derive( Debug ) ]
pub struct SyncInference
{
  client : Client< HuggingFaceEnvironmentImpl >,
  runtime : Arc< tokio::runtime::Runtime >,
}

impl SyncInference
{
  /// Create a blocking text generation inference request
  ///
  /// This is a simple synchronous wrapper around the async inference API.
  ///
  /// # Arguments
  ///
  /// * `inputs` - Input text or prompt
  /// * `model` - Model identifier
  ///
  /// # Errors
  ///
  /// Returns error if the API call fails
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use api_huggingface::sync::SyncClient;
  /// # fn example() -> Result< (), Box< dyn std::error::Error > > {
  /// # let client = SyncClient::new("hf_test".to_string())?;
  /// let _result = client.inference().create(
  ///   "What is 2+2?",
  ///   "mistralai/Mistral-7B-Instruct-v0.3"
  /// )?;
  /// # Ok(())
  /// # }
  /// ```
  #[ inline ]
  pub fn create< I, M >( &self, inputs : I, model : M ) -> Result< InferenceResponse >
  where
  I : Into< String >,
  M : AsRef< str >,
  {
  let inference = self.client.inference();
  self.runtime.block_on( async move
  {
      inference.create( inputs, model ).await
  })
  }

  /// Create a blocking text generation inference request with parameters
  ///
  /// # Arguments
  ///
  /// * `inputs` - Input text or prompt
  /// * `model` - Model identifier
  /// * `parameters` - Inference parameters (temperature, `max_tokens`, etc.)
  ///
  /// # Errors
  ///
  /// Returns error if the API call fails
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use api_huggingface::{sync::SyncClient, components::input::InferenceParameters};
  /// # fn example() -> Result< (), Box< dyn std::error::Error > > {
  /// # let client = SyncClient::new("hf_test".to_string())?;
  /// let params = InferenceParameters::new()
  ///   .with_temperature( 0.7 )
  ///   .with_max_new_tokens( 150 );
  ///
  /// let _result = client.inference().create_with_parameters(
  ///   "Explain quantum computing",
  ///   "mistralai/Mistral-7B-Instruct-v0.3",
  ///   params
  /// )?;
  /// # Ok(())
  /// # }
  /// ```
  #[ inline ]
  pub fn create_with_parameters< I, M >(
  &self,
  inputs : I,
  model : M,
  parameters : InferenceParameters
  ) -> Result< InferenceResponse >
  where
  I : Into< String >,
  M : AsRef< str >,
  {
  let inference = self.client.inference();
  self.runtime.block_on( async move
  {
      inference.create_with_parameters( inputs, model, parameters ).await
  })
  }

  /// Create a blocking streaming inference request
  ///
  /// Returns an iterator that yields tokens as they become available.
  /// Each iteration blocks until the next token arrives or the stream ends.
  ///
  /// # Arguments
  ///
  /// * `inputs` - Input text or prompt
  /// * `model` - Model identifier
  /// * `parameters` - Inference parameters (will be modified to enable streaming)
  ///
  /// # Errors
  ///
  /// Returns error if the API call fails to initialize
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use api_huggingface::{sync::SyncClient, components::input::InferenceParameters};
  /// # fn example() -> Result< (), Box< dyn std::error::Error > > {
  /// # let client = SyncClient::new("hf_test".to_string())?;
  /// let params = InferenceParameters::new()
  ///   .with_temperature( 0.7 )
  ///   .with_max_new_tokens( 150 );
  ///
  /// let stream = client.inference().create_stream(
  ///   "Tell me a story",
  ///   "mistralai/Mistral-7B-Instruct-v0.3",
  ///   params
  /// )?;
  ///
  /// for token_result in stream
  /// {
  ///   match token_result
  ///   {
  ///     Ok( token ) => print!( "{token}" ),
  ///     Err( e ) => eprintln!( "Error : {e}" ),
  ///   }
  /// }
  /// # Ok(())
  /// # }
  /// ```
  #[ inline ]
  pub fn create_stream< I, M >(
  &self,
  inputs : I,
  model : M,
  parameters : InferenceParameters
  ) -> Result< SyncStream >
  where
  I : Into< String >,
  M : AsRef< str >,
  {
  let inference = self.client.inference();
  let runtime = Arc::clone( &self.runtime );

  let receiver = self.runtime.block_on( async move
  {
      inference.create_stream( inputs, model, parameters ).await
  } )?;

  Ok( SyncStream
  {
      receiver,
      runtime,
  } )
  }
}

// SyncClient is automatically Send + Sync because:
// - Client is Send + Sync
// - Arc< Runtime > is Send + Sync
// No unsafe impl needed
