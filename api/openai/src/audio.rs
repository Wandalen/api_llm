// src/api/audio.rs
//! This module defines the `Audio` API client, which provides methods
//! for interacting with the `OpenAI` Audio API.
//!
//! For more details, refer to the [`OpenAI` Audio API documentation](https://platform.openai.com/docs/api-reference/audio).

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
  use crate::components::audio::
  {
    CreateSpeechRequest,
    CreateTranscriptionRequest,
    CreateTranscriptionResponseJson,
    CreateTranslationRequest,
    CreateTranslationResponseJson,
  };

  // External crates
  use reqwest::multipart::{ Form, Part };



  /// The client for the `OpenAI` Audio API.
  #[ derive( Debug, Clone ) ]
  pub struct Audio< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client Client< E >,
  }

  impl< 'client, E > Audio< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Creates a new `Audio` client.
    ///
    /// # Arguments
    /// - `client`: The core `OpenAI` `Client` to use for requests.
    #[ inline ]
    pub(crate) fn new( client : &'client Client< E > ) -> Self
    {
      Self { client }
    }

    /// Generates audio from the input text.
    ///
    /// # Arguments
    /// - `request`: The request body for generating audio.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn speech( &self, request : CreateSpeechRequest ) -> Result< Vec< u8 > >
    {
      let path = "/audio/speech";
      self.client.post_binary( path, &request ).await
    }

    /// Transcribes audio into the input language.
    ///
    /// # Arguments
    /// - `request`: The request body for transcribing audio.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn transcribe( &self, request : CreateTranscriptionRequest ) -> Result< CreateTranscriptionResponseJson >
    {
      // Create multipart form
      let file_part = Part::bytes( request.file )
        .file_name( request.filename )
        .mime_str( "audio/*" )
        .map_err( | e | crate::error::OpenAIError::Internal( format!( "Failed to create file part : {e}" ) ) )?;

      let mut form = Form::new()
        .part( "file", file_part )
        .text( "model", request.model );

      if let Some( language ) = request.language
      {
        form = form.text( "language", language );
      }

      if let Some( prompt ) = request.prompt
      {
        form = form.text( "prompt", prompt );
      }

      if let Some( response_format ) = request.response_format
      {
        form = form.text( "response_format", serde_json::to_string( &response_format )
          .map_err( | e | crate::error::OpenAIError::Internal( format!( "Failed to serialize response_format : {e}" ) ) )? );
      }

      if let Some( temperature ) = request.temperature
      {
        form = form.text( "temperature", temperature.to_string() );
      }

      if let Some( timestamp_granularities ) = request.timestamp_granularities
      {
        let granularities_json = serde_json::to_string( &timestamp_granularities )
          .map_err( | e | crate::error::OpenAIError::Internal( format!( "Failed to serialize timestamp_granularities : {e}" ) ) )?;
        form = form.text( "timestamp_granularities", granularities_json );
      }

      let path = "/audio/transcriptions";
      self.client.post_multipart( path, form ).await
    }

    /// Translates audio into English.
    ///
    /// # Arguments
    /// - `request`: The request body for translating audio.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn translate( &self, request : CreateTranslationRequest ) -> Result< CreateTranslationResponseJson >
    {
      // Create multipart form
      let file_part = Part::bytes( request.file )
        .file_name( request.filename )
        .mime_str( "audio/*" )
        .map_err( | e | crate::error::OpenAIError::Internal( format!( "Failed to create file part : {e}" ) ) )?;

      let mut form = Form::new()
        .part( "file", file_part )
        .text( "model", request.model );

      if let Some( prompt ) = request.prompt
      {
        form = form.text( "prompt", prompt );
      }

      if let Some( response_format ) = request.response_format
      {
        form = form.text( "response_format", serde_json::to_string( &response_format )
          .map_err( | e | crate::error::OpenAIError::Internal( format!( "Failed to serialize response_format : {e}" ) ) )? );
      }

      if let Some( temperature ) = request.temperature
      {
        form = form.text( "temperature", temperature.to_string() );
      }

      let path = "/audio/translations";
      self.client.post_multipart( path, form ).await
    }
  }
} // end mod private

crate ::mod_interface!
{
  // Expose all structs defined in this module
  exposed use
  {
    Audio,
  };
}