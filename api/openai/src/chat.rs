// src/api/chat.rs
//! This module defines the `Chat` API client, which provides methods
//! for interacting with the `OpenAI` Chat API.
//!
//! For more details, refer to the [`OpenAI` Chat API documentation](https://platform.openai.com/docs/api-reference/chat).

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
  use crate::components::chat_shared::
  {
    ChatCompletionRequest,
    CreateChatCompletionResponse,
    ChatCompletionStreamResponse,
  };

  // External crates

  use tokio::sync::mpsc;

  /// The client for the `OpenAI` Chat API.
  #[ derive( Debug, Clone ) ]
  pub struct Chat< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client Client< E >,
  }

  impl< 'client, E > Chat< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Creates a new `Chat` client.
    ///
    /// # Arguments
    /// - `client`: The core `OpenAI` `Client` to use for requests.
    #[ inline ]
    pub(crate) fn new( client : &'client Client< E > ) -> Self
    {
      Self { client }
    }

    /// Creates a chat completion.
    ///
    /// # Arguments
    /// - `request`: The request body for creating a chat completion.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use api_openai::{ Client, environment::{ OpenaiEnvironment, EnvironmentInterface }, components::chat_shared::ChatCompletionRequest, ClientApiAccessors };
    ///
    /// # async fn example(client : Client< impl OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static >) -> Result<(), Box< dyn core::error::Error > >
    /// # {
    /// let request = ChatCompletionRequest::former()
    ///   .model("gpt-4".to_string())
    ///   .form();
    ///
    /// let response = client.chat().create(request).await?;
    /// println!("Response : {:?}", response);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn create( &self, request : ChatCompletionRequest ) -> Result< CreateChatCompletionResponse >
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

      self.client.post( "chat/completions", &request ).await
    }

    /// Creates a chat completion and streams the response.
    ///
    /// # Arguments
    /// - `request`: The request body for creating a chat completion.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn create_stream( &self, request : ChatCompletionRequest ) -> Result< mpsc::Receiver< Result< ChatCompletionStreamResponse > > >
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

      self.client.post_stream( "chat/completions", &request ).await
    }
  }
} // end mod private

crate ::mod_interface!
{
  // Expose all structs defined in this module
  exposed use
  {
    Chat,
  };
}