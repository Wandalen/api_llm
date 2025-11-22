// src/api/files.rs
//! This module defines the `Files` API client, which provides methods
//! for interacting with the `OpenAI` Files API.
//!
//! For more details, refer to the [`OpenAI` Files API documentation](https://platform.openai.com/docs/api-reference/files).

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
  use crate::components::files::
  {
    FileObject,
    ListFilesResponse,
    CreateFileRequest,
  };
  use crate::components::common::ListQuery;

  // External crates
  use reqwest::multipart::{ Form, Part };
  use serde_json;

  /// The client for the `OpenAI` Files API.
  #[ derive( Debug, Clone ) ]
  pub struct Files< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client Client< E >,
  }

  impl< 'client, E > Files< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Creates a new `Files` client.
    ///
    /// # Arguments
    /// - `client`: The core `OpenAI` `Client` to use for requests.
    #[ inline ]
    pub(crate) fn new( client : &'client Client< E > ) -> Self
    {
      Self { client }
    }

    /// Uploads a file that can be used across various features.
    ///
    /// # Arguments
    /// - `request`: The request body for uploading a file.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn upload( &self, request : CreateFileRequest ) -> Result< FileObject >
    {
      // Create multipart form
      let file_part = Part::bytes( request.file )
        .file_name( request.filename.clone() )
        .mime_str( "application/octet-stream" )
        .map_err( | e | crate::error::OpenAIError::Internal( format!( "Failed to create file part : {e}" ) ) )?;

      let form = Form::new()
        .part( "file", file_part )
        .text( "purpose", request.purpose );

      let path = "/files";
      self.client.post_multipart( path, form ).await
    }

    /// Lists files that belong to the user's organization.
    ///
    /// # Arguments
    /// - `query`: Optional query parameters for listing files.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn list( &self, query : Option< ListQuery > ) -> Result< ListFilesResponse >
    {
      let path = "/files";
      if let Some( q ) = query
      {
        self.client.get_with_query( path, &q ).await
      }
      else
      {
        self.client.get( path ).await
      }
    }

    /// Retrieves a file.
    ///
    /// # Arguments
    /// - `file_id`: The ID of the file to retrieve.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn retrieve( &self, file_id : &str ) -> Result< FileObject >
    {
      let path = format!( "/files/{file_id}" );
      self.client.get( &path ).await
    }

    /// Deletes a file.
    ///
    /// # Arguments
    /// - `file_id`: The ID of the file to delete.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn delete( &self, file_id : &str ) -> Result< serde_json::Value >
    {
      let path = format!( "/files/{file_id}" );
      self.client.delete( &path ).await
    }

    /// Retrieves the content of the specified file.
    ///
    /// # Arguments
    /// - `file_id`: The ID of the file to retrieve content for.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn retrieve_content( &self, file_id : &str ) -> Result< Vec< u8 > >
    {
      let path = format!( "/files/{file_id}/content" );
      let response = self.client.get( &path ).await?;
      Ok( response )
    }
  }
} // end mod private

crate ::mod_interface!
{
  // Expose all structs defined in this module
  exposed use
  {
    Files,
  };
}