//! Builder for content generation requests.

use crate::error::Error;
use super::super::ModelApi;

/// Builder for fluent generation request configuration.
///
/// This builder allows step-by-step construction of complex generation
/// requests with method chaining for better ergonomics.
#[ derive( Debug ) ]
pub struct GenerationRequestBuilder< 'a >
{
  model : &'a ModelApi< 'a >,
  request : crate::models::GenerateContentRequest,
}

impl< 'a > GenerationRequestBuilder< 'a >
{
  /// Creates a new request builder.
  #[ inline ]
  #[ must_use ]
  pub fn new( model : &'a ModelApi< 'a > ) -> Self
  {
    Self {
      model,
      request : crate::models::GenerateContentRequest {
        contents : vec![],
        generation_config : None,
        safety_settings : None,
        tools : None,
        tool_config : None,
        system_instruction : None,
        cached_content : None,
      },
    }
  }

  /// Sets the prompt text for generation.
  ///
  /// This method configures the input prompt that the model will use to
  /// generate content. It automatically creates the proper content structure.
  ///
  /// # Arguments
  ///
  /// * `prompt` - The text prompt for content generation
  #[ inline ]
  #[ must_use ]
  pub fn with_prompt( mut self, prompt : &str ) -> Self
  {
    self.request.contents = vec![ crate::models::Content {
      parts : vec![ crate::models::Part {
        text : Some( prompt.to_string() ),
        ..Default::default()
      } ],
      role : "user".to_string(),
    } ];
    self
  }

  /// Sets the temperature for generation randomness.
  ///
  /// Temperature controls the randomness of the output:
  /// - 0.0: Most deterministic (always picks most likely token)
  /// - 1.0: Most random (samples according to probability distribution)
  ///
  /// # Arguments
  ///
  /// * `temperature` - Value between 0.0 and 1.0
  #[ inline ]
  #[ must_use ]
  pub fn with_temperature( mut self, temperature : f32 ) -> Self
  {
    self.ensure_generation_config();
    if let Some( ref mut config ) = self.request.generation_config
    {
      config.temperature = Some( temperature );
    }
    self
  }

  /// Sets the maximum number of output tokens.
  ///
  /// # Arguments
  ///
  /// * `max_tokens` - Maximum tokens to generate
  #[ inline ]
  #[ must_use ]
  pub fn with_max_tokens( mut self, max_tokens : i32 ) -> Self
  {
    self.ensure_generation_config();
    if let Some( ref mut config ) = self.request.generation_config
    {
      config.max_output_tokens = Some( max_tokens );
    }
    self
  }

  /// Sets stop sequences that will halt generation.
  ///
  /// # Arguments
  ///
  /// * `stop_sequences` - List of strings that stop generation when encountered
  #[ inline ]
  #[ must_use ]
  pub fn with_stop_sequences( mut self, stop_sequences : Vec< String > ) -> Self
  {
    self.ensure_generation_config();
    if let Some( ref mut config ) = self.request.generation_config
    {
      config.stop_sequences = Some( stop_sequences );
    }
    self
  }

  /// Executes the configured generation request.
  ///
  /// # Returns
  ///
  /// Returns the full [`crate::models::GenerateContentResponse`] from the model.
  ///
  /// # Errors
  ///
  /// Returns the same errors as [`ModelApi::generate_content`].
  #[ inline ]
  pub async fn execute( self ) -> Result< crate::models::GenerateContentResponse, Error >
  {
    self.model.generate_content( &self.request ).await
  }

  /// Executes the request and returns only the generated text.
  ///
  /// This is a convenience method that extracts the text from the first
  /// candidate in the response.
  ///
  /// # Returns
  ///
  /// Returns the generated text string.
  ///
  /// # Errors
  ///
  /// Returns generation errors plus text extraction errors.
  #[ inline ]
  pub async fn execute_text( self ) -> Result< String, Error >
  {
    let model_id = self.model.model_id.clone();
    let response = self.execute().await?;
    
    response.candidates
      .first()
      .and_then( |candidate| candidate.content.parts.first() )
      .and_then( |part| part.text.as_ref() )
      .cloned()
      .ok_or_else( || Error::ApiError( 
        format!( "No text content returned from model '{model_id}'." )
      ) )
  }

  /// Ensures `generation_config` exists in the request.
  #[ inline ]
  fn ensure_generation_config( &mut self )
  {
    if self.request.generation_config.is_none()
    {
      self.request.generation_config = Some( crate::models::GenerationConfig::default() );
    }
  }
}
