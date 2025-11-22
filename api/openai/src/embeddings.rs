// src/api/embeddings.rs
//! This module defines the `Embeddings` API client, which provides methods
//! for interacting with the `OpenAI` Embeddings API.
//!
//! For more details, refer to the [`OpenAI` Embeddings API documentation](https://platform.openai.com/docs/api-reference/embeddings).

/// Define a private namespace for all its items.
mod private
{
  // Use crate root for base access
  use crate::
  {
    client ::Client,
    error ::{ Result, OpenAIError },
    environment ::{ OpenaiEnvironment, EnvironmentInterface },
    curl_generation ::{ CurlGeneration, build_curl_request, serialize_request_to_json },
  };
  use crate::components::embeddings::
  {
    CreateEmbeddingResponse,
  };
  use crate::components::embeddings_request::
  {
    CreateEmbeddingRequest,
  };

  // External crates

  /// The client for the `OpenAI` Embeddings API.
  #[ derive( Debug, Clone ) ]
  pub struct Embeddings< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client Client< E >,
  }

  impl< 'client, E > Embeddings< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Creates a new `Embeddings` client.
    ///
    /// # Arguments
    /// - `client`: The core `OpenAI` `Client` to use for requests.
    #[ inline ]
    pub(crate) fn new( client : &'client Client< E > ) -> Self
    {
      Self { client }
    }

    /// Creates an embedding vector representing the input text.
    ///
    /// # Arguments
    /// - `request`: The request body for creating an embedding.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use api_openai::{ Client, environment::{ OpenaiEnvironmentImpl, OpenAIRecommended }, Secret, ClientApiAccessors };
    /// use api_openai::components::embeddings_request::CreateEmbeddingRequest;
    ///
    /// # async fn example() -> Result<(), Box< dyn std::error::Error > > {
    /// let secret = Secret::load_from_env("OPENAI_API_KEY")?;
    /// let env = OpenaiEnvironmentImpl::build(secret, None, None, OpenAIRecommended::base_url().to_string(), OpenAIRecommended::realtime_base_url().to_string())?;
    /// let client = Client::build(env)?;
    ///
    /// let request = CreateEmbeddingRequest::new_single(
    ///   "The quick brown fox jumps over the lazy dog".to_string(),
    ///   "text-embedding-ada-002".to_string()
    /// );
    ///
    /// let response = client.embeddings().create(request).await?;
    /// println!("Generated {} embeddings", response.data.len());
    /// # Ok(())
    /// # }
    /// ```
    #[ inline ]
    pub async fn create( &self, request : CreateEmbeddingRequest ) -> Result< CreateEmbeddingResponse >
    {
      // Validate request before processing
      #[ cfg( feature = "input_validation" ) ]
      {
        use crate::input_validation::Validate;
        if let Err( validation_errors ) = request.validate()
        {
          let error_messages : Vec< String > = validation_errors
            .iter()
            .map( | e | format!( "{e}" ) )
            .collect();
          return Err( error_tools::Error::from( crate::error::OpenAIError::InvalidArgument( format!( "Request validation failed : {}", error_messages.join( "; " ) ) ) ) );
        }
      }

      self.client.post( "embeddings", &request ).await
    }

  }

  impl< E > CurlGeneration for Embeddings< '_, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    type Request = CreateEmbeddingRequest;
    type Error = OpenAIError;

    /// Generate a cURL command for an embedding request
    #[ inline ]
    fn to_curl( &self, request : &Self::Request ) -> core::result::Result< String, Self::Error >
    {
      let json_body = serialize_request_to_json( request ).map_err( |e|
        OpenAIError::Internal( format!( "Failed to serialize request : {e}" ) )
      )?;

      let base_url = self.client.environment.base_url();
      let url = format!( "{base_url}embeddings" );

      let mut headers = vec![
        ( "Content-Type".to_string(), "application/json".to_string() ),
        ( "User-Agent".to_string(), "api-openai/1.0.0".to_string() ),
      ];

      // Add headers from environment (includes authorization, organization, project)
      let env_headers = self.client.environment.headers().map_err( |e|
        OpenAIError::Internal( format!( "Failed to get headers from environment : {e}" ) )
      )?;

      for ( key, value ) in &env_headers
      {
        if let Ok( value_str ) = value.to_str()
        {
          headers.push( ( key.as_str().to_string(), value_str.to_string() ) );
        }
      }

      let curl_request = build_curl_request( "POST", &url, &headers, Some( &json_body ) );
      Ok( curl_request.to_curl_command() )
    }

    /// Generate a safe cURL command with redacted sensitive information
    #[ inline ]
    fn to_curl_safe( &self, request : &Self::Request ) -> core::result::Result< String, Self::Error >
    {
      let json_body = serialize_request_to_json( request ).map_err( |e|
        OpenAIError::Internal( format!( "Failed to serialize request : {e}" ) )
      )?;

      let base_url = self.client.environment.base_url();
      let url = format!( "{base_url}embeddings" );

      let headers = vec![
        ( "Content-Type".to_string(), "application/json".to_string() ),
        ( "User-Agent".to_string(), "api-openai/1.0.0".to_string() ),
        ( "Authorization".to_string(), "Bearer [REDACTED]".to_string() ),
      ];

      let curl_request = build_curl_request( "POST", &url, &headers, Some( &json_body ) );
      Ok( curl_request.to_curl_command() )
    }

    /// Generate a cURL command with custom headers
    #[ inline ]
    fn to_curl_with_headers( &self, request : &Self::Request, custom_headers : &std::collections::HashMap<  String, String  > ) -> core::result::Result< String, Self::Error >
    {
      let json_body = serialize_request_to_json( request ).map_err( |e|
        OpenAIError::Internal( format!( "Failed to serialize request : {e}" ) )
      )?;

      let base_url = self.client.environment.base_url();
      let url = format!( "{base_url}embeddings" );

      let mut headers = vec![
        ( "Content-Type".to_string(), "application/json".to_string() ),
        ( "User-Agent".to_string(), "api-openai/1.0.0".to_string() ),
      ];

      // Add headers from environment (includes authorization, organization, project)
      let env_headers = self.client.environment.headers().map_err( |e|
        OpenAIError::Internal( format!( "Failed to get headers from environment : {e}" ) )
      )?;

      for ( key, value ) in &env_headers
      {
        if let Ok( value_str ) = value.to_str()
        {
          headers.push( ( key.as_str().to_string(), value_str.to_string() ) );
        }
      }

      // Add custom headers
      for ( key, value ) in custom_headers
      {
        headers.push( ( key.clone(), value.clone() ) );
      }

      let curl_request = build_curl_request( "POST", &url, &headers, Some( &json_body ) );
      Ok( curl_request.to_curl_command() )
    }
  }
} // end mod private

crate ::mod_interface!
{
  // Expose all structs defined in this module
  exposed use
  {
    Embeddings,
  };
}