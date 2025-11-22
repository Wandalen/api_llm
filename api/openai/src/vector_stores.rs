// src/api/vector_stores.rs
//! This module defines the `serde_json::Values` API client, which provides methods
//! for interacting with the `OpenAI` Vector Stores API.
//!
//! For more details, refer to the [OpenAI Vector Stores API documentation](https://platform.openai.com/docs/api-reference/vector-stores).

/// Define a private namespace for all its items.
mod private
{
  // Use crate root for base access
  use crate::
  {
    client ::Client,
    error ::Result,
    environment ::{ OpenaiEnvironment, EnvironmentInterface },
  };
  // Vector stores components are not fully implemented
  // Using basic types for now
  use crate::components::common::ListQuery;

  // External crates

  use serde_json;

  /// The client for the `OpenAI` Vector Stores API.
  #[ derive( Debug, Clone ) ]
  pub struct VectorStores< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client Client< E >,
  }

  impl< 'client, E > VectorStores< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Creates a new `VectorStores` client.
    ///
    /// # Arguments
    /// - `client`: The core `OpenAI` `Client` to use for requests.
    #[ inline ]
    pub(crate) fn new( client : &'client Client< E > ) -> Self
    {
      Self { client }
    }

    /// Creates a vector store.
    ///
    /// # Arguments
    /// - `request`: The request body for creating a vector store.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn create( &self, request : serde_json::Value ) -> Result< serde_json::Value >
    {
      self.client.post( "vector_stores", &request ).await
    }

    /// Lists vector stores.
    ///
    /// # Arguments
    /// - `query`: Optional query parameters for listing vector stores.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn list( &self, query : Option< ListQuery > ) -> Result< serde_json::Value >
    {
      let path = "/vector_stores";
      if let Some( q ) = query
      {
        self.client.get_with_query( path, &q ).await
      }
      else
      {
        self.client.get( path ).await
      }
    }

    /// Retrieves a vector store.
    ///
    /// # Arguments
    /// - `vector_store_id`: The ID of the vector store to retrieve.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn retrieve( &self, vector_store_id : &str ) -> Result< serde_json::Value >
    {
      let path = format!( "/vector_stores/{vector_store_id}" );
      self.client.get( &path ).await
    }

    /// Modifies a vector store.
    ///
    /// # Arguments
    /// - `vector_store_id`: The ID of the vector store to modify.
    /// - `request`: The request body for modifying the vector store.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn modify( &self, vector_store_id : &str, request : serde_json::Value ) -> Result< serde_json::Value >
    {
      let path = format!( "/vector_stores/{vector_store_id}" );
      self.client.post( &path, &request ).await
    }

    /// Deletes a vector store.
    ///
    /// # Arguments
    /// - `vector_store_id`: The ID of the vector store to delete.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn delete( &self, vector_store_id : &str ) -> Result< serde_json::Value >
    {
      let path = format!( "/vector_stores/{vector_store_id}" );
      self.client.delete( &path ).await
    }

    /// Creates a vector store file.
    ///
    /// # Arguments
    /// - `vector_store_id`: The ID of the vector store to create the file for.
    /// - `request`: The request body for creating a vector store file.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn create_file( &self, vector_store_id : &str, request : serde_json::Value ) -> Result< serde_json::Value >
    {
      let path = format!( "/vector_stores/{vector_store_id}/files" );
      self.client.post( &path, &request ).await
    }

    /// Lists vector store files.
    ///
    /// # Arguments
    /// - `vector_store_id`: The ID of the vector store.
    /// - `query`: Optional query parameters for listing files.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn list_files( &self, vector_store_id : &str, query : Option< ListQuery > ) -> Result< serde_json::Value >
    {
      let path = format!( "/vector_stores/{vector_store_id}/files" );
      if let Some( q ) = query
      {
        self.client.get_with_query( &path, &q ).await
      }
      else
      {
        self.client.get( &path ).await
      }
    }

    /// Retrieves a vector store file.
    ///
    /// # Arguments
    /// - `vector_store_id`: The ID of the vector store.
    /// - `file_id`: The ID of the file to retrieve.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn retrieve_file( &self, vector_store_id : &str, file_id : &str ) -> Result< serde_json::Value >
    {
      let path = format!( "/vector_stores/{vector_store_id}/files/{file_id}" );
      self.client.get( &path ).await
    }

    /// Deletes a vector store file.
    ///
    /// # Arguments
    /// - `vector_store_id`: The ID of the vector store.
    /// - `file_id`: The ID of the file to delete.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn delete_file( &self, vector_store_id : &str, file_id : &str ) -> Result< serde_json::Value >
    {
      let path = format!( "/vector_stores/{vector_store_id}/files/{file_id}" );
      self.client.delete( &path ).await
    }

    /// Modifies a vector store file.
    ///
    /// # Arguments
    /// - `vector_store_id`: The ID of the vector store.
    /// - `file_id`: The ID of the file to modify.
    /// - `request`: The request body for modifying the file.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn modify_file( &self, vector_store_id : &str, file_id : &str, request : serde_json::Value ) -> Result< serde_json::Value >
    {
      let path = format!( "/vector_stores/{vector_store_id}/files/{file_id}" );
      self.client.post( &path, &request ).await
    }

    /// Creates a vector store file batch.
    ///
    /// # Arguments
    /// - `vector_store_id`: The ID of the vector store.
    /// - `request`: The request body for creating a file batch.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn create_file_batch( &self, vector_store_id : &str, request : serde_json::Value ) -> Result< serde_json::Value >
    {
      let path = format!( "/vector_stores/{vector_store_id}/file_batches" );
      self.client.post( &path, &request ).await
    }

    /// Retrieves a vector store file batch.
    ///
    /// # Arguments
    /// - `vector_store_id`: The ID of the vector store.
    /// - `batch_id`: The ID of the file batch to retrieve.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn retrieve_file_batch( &self, vector_store_id : &str, batch_id : &str ) -> Result< serde_json::Value >
    {
      let path = format!( "/vector_stores/{vector_store_id}/file_batches/{batch_id}" );
      self.client.get( &path ).await
    }

    /// Lists vector store file batches.
    ///
    /// # Arguments
    /// - `vector_store_id`: The ID of the vector store.
    /// - `query`: Optional query parameters for listing file batches.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn list_file_batches( &self, vector_store_id : &str, query : Option< ListQuery > ) -> Result< serde_json::Value >
    {
      let path = format!( "/vector_stores/{vector_store_id}/file_batches" );
      if let Some( q ) = query
      {
        self.client.get_with_query( &path, &q ).await
      }
      else
      {
        self.client.get( &path ).await
      }
    }

    /// Cancels a vector store file batch.
    ///
    /// # Arguments
    /// - `vector_store_id`: The ID of the vector store.
    /// - `batch_id`: The ID of the file batch to cancel.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn cancel_file_batch( &self, vector_store_id : &str, batch_id : &str ) -> Result< serde_json::Value >
    {
      let path = format!( "/vector_stores/{vector_store_id}/file_batches/{batch_id}/cancel" );
      self.client.post( &path, &serde_json::json!({}) ).await
    }

    /// Updates a vector store file batch.
    ///
    /// # Arguments
    /// - `vector_store_id`: The ID of the vector store.
    /// - `batch_id`: The ID of the file batch to update.
    /// - `request`: The request body for updating the file batch.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn update_file_batch( &self, vector_store_id : &str, batch_id : &str, request : serde_json::Value ) -> Result< serde_json::Value >
    {
      let path = format!( "/vector_stores/{vector_store_id}/file_batches/{batch_id}" );
      self.client.post( &path, &request ).await
    }
  }
} // end mod private

crate ::mod_interface!
{
  // Expose all structs defined in this module
  exposed use
  {
    VectorStores,
  };
}