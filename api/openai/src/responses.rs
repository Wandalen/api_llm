// src/api/responses.rs
//! This module defines the `Responses` API client, which provides methods
//! for interacting with the `OpenAI` Responses API.
//!
//! For more details, refer to the [`OpenAI` Responses API documentation](https://platform.openai.com/docs/api-reference/responses).

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
  use crate::components::responses::
  {
    CreateResponseRequest,
    ResponseObject,
    ResponseItemList,
    ResponseStreamEvent,
  };
  use crate::components::common::ListQuery;

  // External crates

  use tokio::sync::mpsc;

  /// The client for the `OpenAI` Responses API.
  #[ derive( Debug, Clone ) ]
  pub struct Responses< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client Client< E >,
  }

  impl< 'client, E > Responses< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Creates a new `Responses` client.
    ///
    /// # Arguments
    /// - `client`: The core `OpenAI` `Client` to use for requests.
    #[ inline ]
    pub(crate) fn new( client : &'client Client< E > ) -> Self
    {
      Self { client }
    }

    /// Creates a model response.
    ///
    /// # Arguments
    /// - `request`: The request body for creating a response.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn create( &self, request : CreateResponseRequest ) -> Result< ResponseObject >
    {
      self.client.post( "responses", &request ).await
    }

    /// Retrieves a response.
    ///
    /// # Arguments
    /// - `response_id`: The ID of the response to retrieve.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn retrieve( &self, response_id : &str ) -> Result< ResponseObject >
    {
      let path = format!( "responses/{response_id}" );
      self.client.get( &path ).await
    }

    /// Lists input items for a response.
    ///
    /// # Arguments
    /// - `response_id`: The ID of the response.
    /// - `query`: Optional query parameters for listing input items.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn list_input_items( &self, response_id : &str, query : Option< ListQuery > ) -> Result< ResponseItemList >
    {
      let path = format!( "responses/{response_id}/input_items" );
      if let Some( q ) = query
      {
        self.client.get_with_query( &path, &q ).await
      }
      else
      {
        self.client.get( &path ).await
      }
    }

    /// Creates a model response and streams the response.
    ///
    /// # Arguments
    /// - `request`: The request body for creating a response.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn create_stream( &self, request : CreateResponseRequest ) -> Result< mpsc::Receiver< Result< ResponseStreamEvent > > >
    {
      self.client.post_stream( "responses", &request ).await
    }

    /// Deletes a response.
    ///
    /// # Arguments
    /// - `response_id`: The ID of the response to delete.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn delete( &self, response_id : &str ) -> Result< serde_json::Value >
    {
      let path = format!( "responses/{response_id}" );
      self.client.delete( &path ).await
    }

    /// Updates a response.
    ///
    /// **⚠️ DEPRECATED**: `OpenAI` API no longer supports PATCH operations on responses.
    /// This method will return HTTP 405 Method Not Allowed.
    ///
    /// # Arguments
    /// - `response_id`: The ID of the response to update.
    /// - `update`: The update request body.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ deprecated( since = "0.2.0", note = "OpenAI API no longer supports response updates" ) ]
    #[ inline ]
    pub async fn update( &self, response_id : &str, update : serde_json::Value ) -> Result< ResponseObject >
    {
      let path = format!( "responses/{response_id}" );
      self.client.patch( &path, &update ).await
    }

    /// Cancels an in-progress response.
    ///
    /// # Arguments
    /// - `response_id`: The ID of the response to cancel.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn cancel( &self, response_id : &str ) -> Result< ResponseObject >
    {
      let path = format!( "responses/{response_id}/cancel" );
      self.client.post_no_body( &path ).await
    }
  }
} // end mod private

crate ::mod_interface!
{
  // Expose all structs defined in this module
  exposed use
  {
    Responses,
  };
}