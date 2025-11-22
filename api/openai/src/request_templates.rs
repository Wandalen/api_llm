//! Request Templates for Common Use Cases
//!
//! Pre-configured templates for typical AI tasks.

/// Define a private namespace for all its items.
mod private
{
  use crate::components::chat_shared::{ ChatCompletionRequest, ChatCompletionRequestMessage };

  /// Request template for common use cases
  #[ derive( Debug, Clone ) ]
  pub struct RequestTemplate
  {
    model : String,
    max_tokens : Option< i32 >,
    temperature : Option< f32 >,
    system_prompt : Option< String >,
  }

  impl RequestTemplate
  {
    /// Create a chat template with balanced parameters
    #[ must_use ]
    #[ inline ]
    pub fn chat( model : impl Into< String > ) -> Self
    {
      Self
      {
        model : model.into(),
        max_tokens : Some( 1024 ),
        temperature : Some( 0.7 ),
        system_prompt : Some( String::from( "You are a helpful assistant." ) ),
      }
    }

    /// Create a code generation template
    #[ must_use ]
    #[ inline ]
    pub fn code_generation( model : impl Into< String > ) -> Self
    {
      Self
      {
        model : model.into(),
        max_tokens : Some( 2048 ),
        temperature : Some( 0.3 ),
        system_prompt : Some( String::from( "You are an expert programmer. Generate clean, efficient code." ) ),
      }
    }

    /// Create a creative writing template
    #[ must_use ]
    #[ inline ]
    pub fn creative_writing( model : impl Into< String > ) -> Self
    {
      Self
      {
        model : model.into(),
        max_tokens : Some( 2048 ),
        temperature : Some( 0.9 ),
        system_prompt : Some( String::from( "You are a creative writer with a vivid imagination." ) ),
      }
    }

    /// Create a factual Q&A template
    #[ must_use ]
    #[ inline ]
    pub fn factual_qa( model : impl Into< String > ) -> Self
    {
      Self
      {
        model : model.into(),
        max_tokens : Some( 512 ),
        temperature : Some( 0.2 ),
        system_prompt : Some( String::from( "You provide accurate, factual answers based on knowledge." ) ),
      }
    }

    /// Create a summarization template
    #[ must_use ]
    #[ inline ]
    pub fn summarization( model : impl Into< String > ) -> Self
    {
      Self
      {
        model : model.into(),
        max_tokens : Some( 500 ),
        temperature : Some( 0.3 ),
        system_prompt : Some( String::from( "You summarize content concisely while preserving key information." ) ),
      }
    }

    /// Set custom system prompt
    #[ must_use ]
    #[ inline ]
    pub fn with_prompt( mut self, prompt : impl Into< String > ) -> Self
    {
      self.system_prompt = Some( prompt.into() );
      self
    }

    /// Set temperature
    #[ must_use ]
    #[ inline ]
    pub fn with_temperature( mut self, temperature : f32 ) -> Self
    {
      self.temperature = Some( temperature );
      self
    }

    /// Set max tokens
    #[ must_use ]
    #[ inline ]
    pub fn with_max_tokens( mut self, max_tokens : i32 ) -> Self
    {
      self.max_tokens = Some( max_tokens );
      self
    }

    /// Build the final chat completion request
    #[ must_use ]
    #[ inline ]
    pub fn build( self, user_message : impl Into< String > ) -> ChatCompletionRequest
    {
      use crate::components::chat_shared::ChatCompletionRequestMessageContent;

      let mut messages = Vec::new();

      if let Some( system_prompt ) = self.system_prompt
      {
        messages.push( ChatCompletionRequestMessage
        {
          role : String::from( "system" ),
          content : Some( ChatCompletionRequestMessageContent::Text( system_prompt ) ),
          name : None,
          tool_calls : None,
          tool_call_id : None,
        } );
      }

      messages.push( ChatCompletionRequestMessage
      {
        role : String::from( "user" ),
        content : Some( ChatCompletionRequestMessageContent::Text( user_message.into() ) ),
        name : None,
        tool_calls : None,
        tool_call_id : None,
      } );

      ChatCompletionRequest
      {
        model : self.model,
        messages,
        temperature : self.temperature,
        top_p : None,
        max_tokens : self.max_tokens,
        n : None,
        stop : None,
        stream : None,
        system_prompt : None,
        user : None,
        tools : None,
        tool_choice : None,
        response_format : None,
        seed : None,
        logit_bias : None,
        logprobs : None,
        top_logprobs : None,
      }
    }
  }

  #[ cfg( test ) ]
  mod tests
  {
    use super::*;

    #[ test ]
    fn test_chat_template()
    {
      let template = RequestTemplate::chat( "gpt-4" );
      let request = template.build( "Hello" );

      assert_eq!( request.model, "gpt-4" );
      assert_eq!( request.messages.len(), 2 );
      assert_eq!( request.messages[ 0 ].role, "system" );
      assert_eq!( request.messages[ 1 ].role, "user" );
      assert_eq!( request.temperature, Some( 0.7 ) );
      assert_eq!( request.max_tokens, Some( 1024 ) );
    }

    #[ test ]
    fn test_code_generation_template()
    {
      let template = RequestTemplate::code_generation( "gpt-4" );
      let request = template.build( "Write a function" );

      assert_eq!( request.model, "gpt-4" );
      assert_eq!( request.temperature, Some( 0.3 ) );
      assert_eq!( request.max_tokens, Some( 2048 ) );
    }

    #[ test ]
    fn test_creative_writing_template()
    {
      let template = RequestTemplate::creative_writing( "gpt-4" );
      let request = template.build( "Write a story" );

      assert_eq!( request.model, "gpt-4" );
      assert_eq!( request.temperature, Some( 0.9 ) );
      assert_eq!( request.max_tokens, Some( 2048 ) );
    }

    #[ test ]
    fn test_factual_qa_template()
    {
      let template = RequestTemplate::factual_qa( "gpt-4" );
      let request = template.build( "What is 2+2?" );

      assert_eq!( request.model, "gpt-4" );
      assert_eq!( request.temperature, Some( 0.2 ) );
      assert_eq!( request.max_tokens, Some( 512 ) );
    }

    #[ test ]
    fn test_summarization_template()
    {
      let template = RequestTemplate::summarization( "gpt-4" );
      let request = template.build( "Summarize this text" );

      assert_eq!( request.model, "gpt-4" );
      assert_eq!( request.temperature, Some( 0.3 ) );
      assert_eq!( request.max_tokens, Some( 500 ) );
    }

    #[ test ]
    fn test_with_prompt()
    {
      let template = RequestTemplate::chat( "gpt-4" )
        .with_prompt( "Custom prompt" );
      let request = template.build( "Hello" );

      if let Some( content ) = request.messages[ 0 ].content.as_ref()
      {
        match content
        {
          crate::components::chat_shared::ChatCompletionRequestMessageContent::Text( text ) =>
          {
            assert!( text.contains( "Custom prompt" ) );
          },
          crate::components::chat_shared::ChatCompletionRequestMessageContent::Parts( _ ) => panic!( "Expected text content" ),
        }
      }
      else
      {
        panic!( "Expected content to be present" );
      }
    }

    #[ test ]
    fn test_with_temperature()
    {
      let template = RequestTemplate::chat( "gpt-4" )
        .with_temperature( 0.5 );
      let request = template.build( "Hello" );

      assert_eq!( request.temperature, Some( 0.5 ) );
    }

    #[ test ]
    fn test_with_max_tokens()
    {
      let template = RequestTemplate::chat( "gpt-4" )
        .with_max_tokens( 500 );
      let request = template.build( "Hello" );

      assert_eq!( request.max_tokens, Some( 500 ) );
    }
  }
}

crate::mod_interface!
{
  exposed use
  {
    RequestTemplate,
  };
}
