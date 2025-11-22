// src/api/assistants.rs
//! This module defines the `Assistants` API client, which provides methods
//! for interacting with the `OpenAI` Assistants API.
//!
//! For more details, refer to the [`OpenAI` Assistants API documentation](https://platform.openai.com/docs/api-reference/assistants).

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
  use crate::components::assistants_shared::
  {
    AssistantObject,
    ListAssistantsResponse,
    DeleteAssistantResponse,
  };
  use crate::components::common::ListQuery;

  // External crates



  /// The client for the `OpenAI` Assistants API.
  #[ derive( Debug, Clone ) ]
  pub struct Assistants< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client Client< E >,
  }

  impl< 'client, E > Assistants< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Creates a new `Assistants` client.
    ///
    /// # Arguments
    /// - `client`: The core `OpenAI` `Client` to use for requests.
    #[ inline ]
    pub(crate) fn new( client : &'client Client< E > ) -> Self
    {
      Self { client }
    }

    /// Creates an assistant.
    ///
    /// # Arguments
    /// - `request`: The request body for creating an assistant.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn create( &self, request : AssistantObject ) -> Result< AssistantObject >
    {
      self.client.post( "assistants", &request ).await
    }

    /// Retrieves an assistant.
    ///
    /// # Arguments
    /// - `assistant_id`: The ID of the assistant to retrieve.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn retrieve( &self, assistant_id : &str ) -> Result< AssistantObject >
    {
      let path = format!( "/assistants/{assistant_id}" );
      self.client.get( &path ).await
    }

    /// Modifies an assistant.
    ///
    /// # Arguments
    /// - `assistant_id`: The ID of the assistant to modify.
    /// - `request`: The request body for modifying the assistant.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn modify( &self, assistant_id : &str, request : AssistantObject ) -> Result< AssistantObject >
    {
      let path = format!( "/assistants/{assistant_id}" );
      self.client.post( &path, &request ).await
    }

    /// Deletes an assistant.
    ///
    /// # Arguments
    /// - `assistant_id`: The ID of the assistant to delete.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn delete( &self, assistant_id : &str ) -> Result< DeleteAssistantResponse >
    {
      let path = format!( "/assistants/{assistant_id}" );
      self.client.delete( &path ).await
    }

    /// Lists assistants.
    ///
    /// # Arguments
    /// - `query`: Optional query parameters for listing assistants.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn list( &self, query : Option< ListQuery > ) -> Result< ListAssistantsResponse >
    {
      let path = "/assistants";
      if let Some( q ) = query
      {
        self.client.get_with_query( path, &q ).await
      }
      else
      {
        self.client.get( path ).await
      }
    }

    /// Creates an assistant file.
    ///
    /// # Arguments
    /// - `assistant_id`: The ID of the assistant to create the file for.
    /// - `file_id`: The ID of the file to associate with the assistant.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn create_file( &self, assistant_id : &str, file_id : &str ) -> Result< serde_json::Value >
    {
      let path = format!( "/assistants/{assistant_id}/files" );
      let request = serde_json::json!({ "file_id": file_id });
      self.client.post( &path, &request ).await
    }

    /// Retrieves an assistant file.
    ///
    /// # Arguments
    /// - `assistant_id`: The ID of the assistant.
    /// - `file_id`: The ID of the file to retrieve.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn retrieve_file( &self, assistant_id : &str, file_id : &str ) -> Result< serde_json::Value >
    {
      let path = format!( "/assistants/{assistant_id}/files/{file_id}" );
      self.client.get( &path ).await
    }

    /// Deletes an assistant file.
    ///
    /// # Arguments
    /// - `assistant_id`: The ID of the assistant.
    /// - `file_id`: The ID of the file to delete.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn delete_file( &self, assistant_id : &str, file_id : &str ) -> Result< serde_json::Value >
    {
      let path = format!( "/assistants/{assistant_id}/files/{file_id}" );
      self.client.delete( &path ).await
    }

    /// Lists assistant files.
    ///
    /// # Arguments
    /// - `assistant_id`: The ID of the assistant to list files for.
    /// - `query`: Optional query parameters for listing files.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn list_files( &self, assistant_id : &str, query : Option< ListQuery > ) -> Result< serde_json::Value >
    {
      let path = format!( "/assistants/{assistant_id}/files" );
      if let Some( q ) = query
      {
        self.client.get_with_query( &path, &q ).await
      }
      else
      {
        self.client.get( &path ).await
      }
    }
  }
} // end mod private

crate ::mod_interface!
{
  // Expose all structs defined in this module
  exposed use
  {
    Assistants,
  };
}