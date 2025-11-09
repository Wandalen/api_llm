//! Streaming types for the Gemini API.

use serde::{ Deserialize, Serialize };
use super::content::{ Content, Part, Candidate };
use super::generation::{ GenerateContentRequest, GenerationConfig, UsageMetadata };

/// Response type for streaming content generation.
#[ cfg( feature = "streaming" ) ]
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct StreamingResponse
{
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Partial content candidates (present in incremental chunks).
  pub candidates : Option< Vec< Candidate > >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Usage metadata (typically present in final chunk).
  pub usage_metadata : Option< UsageMetadata >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Whether this is the final chunk in the stream.
  pub is_final : Option< bool >,

  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  /// Error information if stream encountered an issue.
  pub error : Option< String >,
}

/// Builder for creating streaming requests with fluent API.
#[ cfg( feature = "streaming" ) ]
#[ derive( Debug ) ]
pub struct StreamingRequestBuilder< 'a >
{
  model : &'a crate::models::api::ModelApi< 'a >,
  request : GenerateContentRequest,
}

#[ cfg( feature = "streaming" ) ]
impl< 'a > StreamingRequestBuilder< 'a >
{
  /// Create a new streaming request builder.
  #[ must_use ]
  #[ inline ]
  pub fn new( model : &'a crate::models::api::ModelApi< 'a > ) -> Self
  {
    Self
    {
      model,
      request : GenerateContentRequest::default(),
    }
  }

  /// Add content with the specified role.
  #[ must_use ]
  #[ inline ]
  pub fn add_content( mut self, role : &str, text : &str ) -> Self
  {
    let content = Content
    {
      parts : vec![ Part { text : Some( text.to_string() ), ..Default::default() } ],
      role : role.to_string(),
    };
    self.request.contents.push( content );
    self
  }

  /// Set the temperature for response generation.
  #[ must_use ]
  #[ inline ]
  pub fn temperature( mut self, temperature : f32 ) -> Self
  {
    if self.request.generation_config.is_none()
    {
      self.request.generation_config = Some( GenerationConfig::default() );
    }
    if let Some( ref mut config ) = self.request.generation_config
    {
      config.temperature = Some( temperature );
    }
    self
  }

  /// Set the maximum number of output tokens.
  #[ must_use ]
  #[ inline ]
  pub fn max_output_tokens( mut self, max_tokens : i32 ) -> Self
  {
    if self.request.generation_config.is_none()
    {
      self.request.generation_config = Some( GenerationConfig::default() );
    }
    if let Some( ref mut config ) = self.request.generation_config
    {
      config.max_output_tokens = Some( max_tokens );
    }
    self
  }

  /// Set the `top_p` value for nucleus sampling.
  #[ must_use ]
  #[ inline ]
  pub fn top_p( mut self, top_p : f32 ) -> Self
  {
    if self.request.generation_config.is_none()
    {
      self.request.generation_config = Some( GenerationConfig::default() );
    }
    if let Some( ref mut config ) = self.request.generation_config
    {
      config.top_p = Some( top_p );
    }
    self
  }

  /// Execute the streaming request.
  ///
  /// # Errors
  ///
  /// Returns an error if the request fails to execute or if the response cannot be parsed.
  #[ inline ]
  pub async fn execute( self ) -> Result< impl futures::Stream< Item = Result< StreamingResponse, crate::error::Error > >, crate::error::Error >
  {
    self.model.generate_content_stream( &self.request ).await
  }
}
