//! Synchronous client wrappers for blocking API operations.
//!
//! This module provides synchronous versions of the async Gemini API client,
//! using a managed Tokio runtime to block on async operations.

use core::time::Duration;
use crate::error::Error;
use crate::models::{
  GenerateContentRequest, GenerateContentResponse,
  EmbedContentRequest, EmbedContentResponse,
  ListModelsResponse, CreateCachedContentRequest, CachedContentResponse,
  ListCachedContentsResponse, UpdateCachedContentRequest,
};
use super::Client;

/// Synchronous client builder for blocking API operations
#[ derive( Debug, Clone ) ]
pub struct SyncClientBuilder
{
  api_key : Option< String >,
  timeout : Option< Duration >,
}

impl SyncClientBuilder
{
  /// Create a new sync client builder
  #[ must_use ]
  #[ inline ]
  pub fn new() -> Self
  {
    Self {
      api_key : None,
      timeout : None,
    }
  }

  /// Set the API key for authentication
  #[ must_use ]
  #[ inline ]
  pub fn api_key< S : Into< String > >( mut self, api_key : S ) -> Self
  {
    self.api_key = Some( api_key.into() );
    self
  }

  /// Set the request timeout
  #[ must_use ]
  #[ inline ]
  pub fn timeout( mut self, timeout : Duration ) -> Self
  {
    self.timeout = Some( timeout );
    self
  }


  /// Build the synchronous client
  ///
  /// # Errors
  ///
  /// Returns an error if the client cannot be built or if required parameters are missing
  #[ inline ]
  pub fn build( self ) -> Result< SyncClient, Error >
  {
    let api_key = self.api_key.ok_or_else( || Error::AuthenticationError(
      "API key is required for sync client".to_string()
    ) )?;

    let rt = tokio::runtime::Runtime::new()
      .map_err( |e| Error::NetworkError( format!( "Failed to create tokio runtime : {e}" ) ) )?;

    let client = rt.block_on( async {
      let mut builder = Client::builder().api_key( api_key );

      if let Some( timeout ) = self.timeout
      {
        builder = builder.timeout( timeout );
      }

      // Note : max_retries might not be available on ClientBuilder API
      // This would be a future enhancement when the builder API supports it

      builder.build()
    } )?;

    Ok( SyncClient {
      client,
      runtime : std::sync::Arc::new( rt ),
    } )
  }
}

impl Default for SyncClientBuilder
{
  #[ inline ]
  fn default() -> Self
  {
    Self::new()
  }
}

/// Synchronous client providing blocking API operations
#[ derive( Debug ) ]
pub struct SyncClient
{
  client : Client,
  runtime : std::sync::Arc< tokio::runtime::Runtime >,
}

impl SyncClient
{
  /// Get access to the models API
  #[ must_use ]
  #[ inline ]
  pub fn models( &self ) -> SyncModelsApi< '_ >
  {
    SyncModelsApi {
      client : &self.client,
      runtime : &self.runtime,
    }
  }

  /// Get access to the cached content API
  #[ must_use ]
  #[ inline ]
  pub fn cached_content( &self ) -> SyncCachedContentApi< '_ >
  {
    SyncCachedContentApi {
      client : &self.client,
      runtime : &self.runtime,
    }
  }
}

/// Synchronous models API providing blocking operations
#[ derive( Debug ) ]
pub struct SyncModelsApi< 'a >
{
  client : &'a Client,
  runtime : &'a tokio::runtime::Runtime,
}

impl SyncModelsApi< '_ >
{
  /// List all available models synchronously
  ///
  /// # Errors
  ///
  /// Returns an error if the API request fails
  #[ inline ]
  pub fn list( &self ) -> Result< ListModelsResponse, Error >
  {
    self.runtime.block_on( async {
      self.client.models().list().await
    } )
  }

  /// Get a specific model by name synchronously
  ///
  /// # Errors
  ///
  /// Returns an error if the model is not found or the API request fails
  #[ inline ]
  pub fn by_name< S : AsRef< str > >( &self, name : S ) -> Result< SyncModelApi< '_ >, Error >
  {
    let model_name = name.as_ref().to_string();

    Ok( SyncModelApi {
      client : self.client,
      runtime : self.runtime,
      model_name,
    } )
  }
}

/// Synchronous model API for specific model operations
#[ derive( Debug ) ]
pub struct SyncModelApi< 'a >
{
  client : &'a Client,
  runtime : &'a tokio::runtime::Runtime,
  model_name : String,
}

impl SyncModelApi< '_ >
{
  /// Generate content synchronously
  ///
  /// # Errors
  ///
  /// Returns an error if the content generation fails
  #[ inline ]
  pub fn generate_content( &self, request : &GenerateContentRequest ) -> Result< GenerateContentResponse, Error >
  {
    self.runtime.block_on( async {
      self.client.models().by_name( &self.model_name )
        .generate_content( request ).await
    } )
  }

  /// Generate content with streaming synchronously
  ///
  /// Collects all streaming responses into a vector of content strings.
  /// This method blocks until all streaming data is received.
  ///
  /// # Arguments
  ///
  /// * `request` - The content generation request
  ///
  /// # Returns
  ///
  /// Returns a vector of content strings from the streaming response
  ///
  /// # Errors
  ///
  /// Returns an error if the streaming request fails or if content parsing fails
  #[ cfg( feature = "streaming" ) ]
  #[ inline ]
  pub fn generate_content_stream( &self, request : &GenerateContentRequest ) -> Result< Vec< String >, Error >
  {
    use futures::StreamExt;

    self.runtime.block_on( async {
      let stream = self.client.models().by_name( &self.model_name )
        .generate_content_stream( request ).await?;

      let mut results = Vec::new();

      // Pin the stream to make it Unpin
      tokio ::pin!( stream );

      while let Some( response_result ) = stream.next().await
      {
        match response_result
        {
          Ok( response ) => {
            // Extract text content from streaming response
            if let Some( candidates ) = response.candidates
            {
              for candidate in candidates
              {
                for part in candidate.content.parts
                {
                  if let Some( text ) = part.text
                  {
                    results.push( text );
                  }
                }
              }
            }
          },
          Err( e ) => return Err( e ),
        }
      }

      Ok( results )
    } )
  }

  /// Generate embeddings synchronously
  ///
  /// # Errors
  ///
  /// Returns an error if the embedding generation fails
  #[ inline ]
  pub fn embed_content( &self, request : &EmbedContentRequest ) -> Result< EmbedContentResponse, Error >
  {
    self.runtime.block_on( async {
      self.client.models().by_name( &self.model_name )
        .embed_content( request ).await
    } )
  }

  /// Count tokens in the provided content synchronously
  ///
  /// # Arguments
  ///
  /// * `request` - The count tokens request containing content to analyze
  ///
  /// # Returns
  ///
  /// Returns the count tokens response with total token count
  ///
  /// # Errors
  ///
  /// Returns an error if the token counting fails or if the request is invalid
  #[ inline ]
  pub fn count_tokens( &self, request : &crate::models::CountTokensRequest ) -> Result< crate::models::CountTokensResponse, Error >
  {
    self.runtime.block_on( async {
      self.client.models().by_name( &self.model_name )
        .count_tokens( request ).await
    } )
  }
}

/// Synchronous cached content API providing blocking operations
#[ derive( Debug ) ]
pub struct SyncCachedContentApi< 'a >
{
  client : &'a Client,
  runtime : &'a tokio::runtime::Runtime,
}

impl SyncCachedContentApi< '_ >
{
  /// Create new cached content synchronously
  ///
  /// # Arguments
  ///
  /// * `request` - The create cached content request
  ///
  /// # Returns
  ///
  /// Returns the cached content response with details of the created cache
  ///
  /// # Errors
  ///
  /// Returns an error if the cached content creation fails
  #[ inline ]
  pub fn create( &self, request : &CreateCachedContentRequest ) -> Result< CachedContentResponse, Error >
  {
    self.runtime.block_on( async {
      self.client.cached_content().create( request ).await
    } )
  }

  /// List all cached contents synchronously
  ///
  /// # Arguments
  ///
  /// * `page_size` - Optional maximum number of cached contents to return per page
  /// * `page_token` - Optional token for retrieving subsequent pages
  ///
  /// # Returns
  ///
  /// Returns the list cached contents response with the available cache entries
  ///
  /// # Errors
  ///
  /// Returns an error if the listing operation fails
  #[ inline ]
  pub fn list( &self, page_size : Option< i32 >, page_token : Option< &str > ) -> Result< ListCachedContentsResponse, Error >
  {
    self.runtime.block_on( async {
      self.client.cached_content().list( page_size, page_token ).await
    } )
  }

  /// Get a specific cached content by ID synchronously
  ///
  /// # Arguments
  ///
  /// * `cache_id` - The unique identifier of the cached content to retrieve
  ///
  /// # Returns
  ///
  /// Returns the cached content response with the requested cache details
  ///
  /// # Errors
  ///
  /// Returns an error if the cached content is not found or the request fails
  #[ inline ]
  pub fn get( &self, cache_id : &str ) -> Result< CachedContentResponse, Error >
  {
    self.runtime.block_on( async {
      self.client.cached_content().get( cache_id ).await
    } )
  }

  /// Update cached content properties synchronously
  ///
  /// # Arguments
  ///
  /// * `cache_id` - The unique identifier of the cached content to update
  /// * `request` - The update cached content request with the changes
  ///
  /// # Returns
  ///
  /// Returns the updated cached content response
  ///
  /// # Errors
  ///
  /// Returns an error if the update operation fails or the cache is not found
  #[ inline ]
  pub fn update( &self, cache_id : &str, request : &UpdateCachedContentRequest ) -> Result< CachedContentResponse, Error >
  {
    self.runtime.block_on( async {
      self.client.cached_content().update( cache_id, request ).await
    } )
  }

  /// Delete cached content synchronously
  ///
  /// # Arguments
  ///
  /// * `cache_id` - The unique identifier of the cached content to delete
  ///
  /// # Returns
  ///
  /// Returns `Ok(())` if the cached content was successfully deleted
  ///
  /// # Errors
  ///
  /// Returns an error if the deletion fails or the cache is not found
  #[ inline ]
  pub fn delete( &self, cache_id : &str ) -> Result< (), Error >
  {
    self.runtime.block_on( async {
      self.client.cached_content().delete( cache_id ).await
    } )
  }
}
