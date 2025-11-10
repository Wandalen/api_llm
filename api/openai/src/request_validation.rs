// src/request_validation.rs
//! Validate trait implementations for `OpenAI` request types

mod private
{
  use crate::input_validation::{ Validate, ValidationError, validators };
  use crate::components::chat_shared::ChatCompletionRequest;
  use crate::components::embeddings_request::CreateEmbeddingRequest;

  /// Implement Validate for `ChatCompletionRequest`
  impl Validate for ChatCompletionRequest
  {
    #[ inline ]
    fn validate( &self ) -> Result< (), Vec< ValidationError > >
    {
      let mut errors = Vec::new();

      // Validate model
      if let Err( e ) = validators::validate_model_name( &self.model )
      {
        errors.push( e );
      }

      // Validate messages
      if let Err( e ) = validators::validate_messages( &self.messages )
      {
        errors.push( e );
      }

      // Validate temperature if present
      if let Some( temperature ) = self.temperature
      {
        if let Err( e ) = validators::validate_temperature( temperature )
        {
          errors.push( e );
        }
      }

      // Validate top_p if present
      if let Some( top_p ) = self.top_p
      {
        if let Err( e ) = validators::validate_top_p( top_p )
        {
          errors.push( e );
        }
      }

      // Validate max_tokens if present
      if let Some( max_tokens ) = self.max_tokens
      {
        if let Err( e ) = validators::validate_max_tokens( max_tokens )
        {
          errors.push( e );
        }
      }

      // Validate n if present
      if let Some( n ) = self.n
      {
        if let Err( e ) = validators::validate_n( n )
        {
          errors.push( e );
        }
      }

      // Validate top_logprobs if present
      if let Some( top_logprobs ) = self.top_logprobs
      {
        if let Err( e ) = validators::validate_top_logprobs( top_logprobs )
        {
          errors.push( e );
        }
      }

      if errors.is_empty()
      {
        Ok( () )
      }
      else
      {
        Err( errors )
      }
    }
  }

  /// Implement Validate for `CreateEmbeddingRequest`
  impl Validate for CreateEmbeddingRequest
  {
    #[ inline ]
    fn validate( &self ) -> Result< (), Vec< ValidationError > >
    {
      let mut errors = Vec::new();

      // Validate model
      if let Err( e ) = validators::validate_model_name( &self.model )
      {
        errors.push( e );
      }

      // Validate input (can be single string or array)
      match &self.input
      {
        crate::components::embeddings_request::EmbeddingInput::Single( s ) =>
        {
          if let Err( e ) = validators::validate_embedding_input( s )
          {
            errors.push( e );
          }
        },
        crate::components::embeddings_request::EmbeddingInput::Multiple( arr ) =>
        {
          if arr.is_empty()
          {
            errors.push(
              ValidationError::new( "input", "Input array cannot be empty" )
                .with_constraint( "at least one input string required" )
            );
          }
          for ( idx, input_str ) in arr.iter().enumerate()
          {
            if let Err( mut e ) = validators::validate_embedding_input( input_str )
            {
              e.field = format!( "input[{idx}]" );
              errors.push( e );
            }
          }
        },
      }

      // Validate dimensions if present
      if let Some( dimensions ) = self.dimensions
      {
        if let Err( e ) = validators::validate_dimensions( dimensions )
        {
          errors.push( e );
        }
      }

      if errors.is_empty()
      {
        Ok( () )
      }
      else
      {
        Err( errors )
      }
    }
  }
}

crate ::mod_interface!
{
  // This module only contains trait impls, no types to expose
}
