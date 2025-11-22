// src/api/images.rs
//! This module defines the `Images` API client, which provides methods
//! for interacting with the `OpenAI` Images API.
//!
//! For more details, refer to the [`OpenAI` Images API documentation](https://platform.openai.com/docs/api-reference/images).

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
  use crate::components::images::
  {
    ImagesResponse,
    CreateImageEditRequest,
    CreateImageVariationRequest,
  };

  // External crates
  use reqwest::multipart::{ Form, Part };


  /// The client for the `OpenAI` Images API.
  #[ derive( Debug, Clone ) ]
  pub struct Images< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client Client< E >,
  }

  impl< 'client, E > Images< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Creates a new `Images` client.
    ///
    /// # Arguments
    /// - `client`: The core `OpenAI` `Client` to use for requests.
    #[ inline ]
    pub(crate) fn new( client : &'client Client< E > ) -> Self
    {
      Self { client }
    }

    /// Creates an image given a prompt.
    ///
    /// # Arguments
    /// - `request`: The request body for image generation.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn generate( &self, request : serde_json::Value ) -> Result< ImagesResponse >
    {
      self.client.post( "images/generations", &request ).await
    }

    /// Creates an edited or extended image given an original image and a prompt.
    ///
    /// # Arguments
    /// - `request`: The request body for image editing.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn edit( &self, request : CreateImageEditRequest ) -> Result< ImagesResponse >
    {
      // Create multipart form
      let image_part = Part::bytes( request.image )
        .file_name( request.image_filename.clone() )
        .mime_str( "image/png" )
        .map_err( | e | crate::error::OpenAIError::Internal( format!( "Failed to create image part : {e}" ) ) )?;

      let mut form = Form::new()
        .part( "image", image_part )
        .text( "prompt", request.prompt );

      // Add optional mask
      if let Some( mask_data ) = request.mask
      {
        if let Some( mask_filename ) = request.mask_filename
        {
          let mask_part = Part::bytes( mask_data )
            .file_name( mask_filename )
            .mime_str( "image/png" )
            .map_err( | e | crate::error::OpenAIError::Internal( format!( "Failed to create mask part : {e}" ) ) )?;
          form = form.part( "mask", mask_part );
        }
      }

      // Add optional fields
      if let Some( model ) = request.model
      {
        form = form.text( "model", model );
      }

      if let Some( n ) = request.n
      {
        form = form.text( "n", n.to_string() );
      }

      if let Some( size ) = request.size
      {
        form = form.text( "size", size );
      }

      if let Some( response_format ) = request.response_format
      {
        form = form.text( "response_format", response_format );
      }

      if let Some( user ) = request.user
      {
        form = form.text( "user", user );
      }

      let path = "/images/edits";
      self.client.post_multipart( path, form ).await
    }

    /// Creates a variation of a given image.
    ///
    /// # Arguments
    /// - `request`: The request body for image variation.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn variation( &self, request : CreateImageVariationRequest ) -> Result< ImagesResponse >
    {
      // Create multipart form
      let image_part = Part::bytes( request.image )
        .file_name( request.image_filename.clone() )
        .mime_str( "image/png" )
        .map_err( | e | crate::error::OpenAIError::Internal( format!( "Failed to create image part : {e}" ) ) )?;

      let mut form = Form::new()
        .part( "image", image_part );

      // Add optional fields
      if let Some( model ) = request.model
      {
        form = form.text( "model", model );
      }

      if let Some( n ) = request.n
      {
        form = form.text( "n", n.to_string() );
      }

      if let Some( response_format ) = request.response_format
      {
        form = form.text( "response_format", response_format );
      }

      if let Some( size ) = request.size
      {
        form = form.text( "size", size );
      }

      if let Some( user ) = request.user
      {
        form = form.text( "user", user );
      }

      let path = "/images/variations";
      self.client.post_multipart( path, form ).await
    }
  }
} // end mod private

crate ::mod_interface!
{
  // Expose all structs defined in this module
  exposed use
  {
    Images,
  };
}