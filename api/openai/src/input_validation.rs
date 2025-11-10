// src/input_validation.rs
//! Input validation module for `OpenAI` API requests
//!
//! Provides comprehensive pre-request parameter validation to catch errors before API calls.
//! Validation rules are based on `OpenAI` API documentation and best practices.

mod private
{
  use std::fmt;

  /// Validation error containing detailed information about what failed
  #[ derive( Debug, Clone, PartialEq ) ]
  pub struct ValidationError
  {
    /// Field name that failed validation
    pub field : String,
    /// Human-readable error message
    pub message : String,
    /// Optional value that failed (for debugging)
    pub value : Option< String >,
    /// Optional constraint that was violated
    pub constraint : Option< String >,
  }

  impl fmt::Display for ValidationError
  {
    #[ inline ]
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "Validation error for field '{}' : {}", self.field, self.message )?;
      if let Some( ref value ) = self.value
      {
        write!( f, " (value : {value})" )?;
      }
      if let Some( ref constraint ) = self.constraint
      {
        write!( f, " (constraint : {constraint})" )?;
      }
      Ok( () )
    }
  }

  impl std::error::Error for ValidationError {}

  impl ValidationError
  {
    /// Create a new validation error
    #[ inline ]
    pub fn new( field : impl Into< String >, message : impl Into< String > ) -> Self
    {
      Self
      {
        field : field.into(),
        message : message.into(),
        value : None,
        constraint : None,
      }
    }

    /// Set the value that failed validation
    #[ inline ]
    #[ must_use ]
    pub fn with_value( mut self, value : impl Into< String > ) -> Self
    {
      self.value = Some( value.into() );
      self
    }

    /// Set the constraint that was violated
    #[ inline ]
    #[ must_use ]
    pub fn with_constraint( mut self, constraint : impl Into< String > ) -> Self
    {
      self.constraint = Some( constraint.into() );
      self
    }
  }

  /// Trait for validating request parameters before API calls
  pub trait Validate
  {
    /// Validate the request parameters
    ///
    /// # Errors
    ///
    /// Returns a Vec of validation errors if validation fails
    ///
    /// Returns `Ok(())` if validation passes, or a Vec of validation errors if it fails.
    fn validate( &self ) -> Result< (), Vec< ValidationError > >;
  }

  /// Validators module containing reusable validation functions
  pub mod validators
  {
    use super::ValidationError;

    /// Validate model name (non-empty, reasonable length)
    ///
    /// # Errors
    ///
    /// Returns error if model name is empty or exceeds 256 characters
    #[ inline ]
    pub fn validate_model_name( model : &str ) -> Result< (), ValidationError >
    {
      if model.is_empty()
      {
        return Err(
          ValidationError::new( "model", "Model name cannot be empty" )
            .with_constraint( "non-empty string" )
        );
      }

      if model.len() > 256
      {
        return Err(
          ValidationError::new( "model", "Model name exceeds maximum length" )
            .with_value( format!( "{} chars", model.len() ) )
            .with_constraint( "max 256 characters" )
        );
      }

      Ok( () )
    }

    /// Validate temperature parameter (0.0 to 2.0)
    ///
    /// # Errors
    ///
    /// Returns error if temperature is outside the range 0.0-2.0
    #[ inline ]
    pub fn validate_temperature( temperature : f32 ) -> Result< (), ValidationError >
    {
      if !( 0.0..=2.0 ).contains( &temperature )
      {
        return Err(
          ValidationError::new( "temperature", "Temperature must be between 0.0 and 2.0" )
            .with_value( temperature.to_string() )
            .with_constraint( "0.0 <= temperature <= 2.0" )
        );
      }
      Ok( () )
    }

    /// Validate `top_p` parameter (0.0 to 1.0)
    ///
    /// # Errors
    ///
    /// Returns error if `top_p` is outside the range 0.0-1.0
    #[ inline ]
    pub fn validate_top_p( top_p : f32 ) -> Result< (), ValidationError >
    {
      if !( 0.0..=1.0 ).contains( &top_p )
      {
        return Err(
          ValidationError::new( "top_p", "top_p must be between 0.0 and 1.0" )
            .with_value( top_p.to_string() )
            .with_constraint( "0.0 <= top_p <= 1.0" )
        );
      }
      Ok( () )
    }

    /// Validate `max_tokens` (must be positive and reasonable)
    ///
    /// # Errors
    ///
    /// Returns error if `max_tokens` is non-positive or exceeds 131072
    #[ inline ]
    pub fn validate_max_tokens( max_tokens : i32 ) -> Result< (), ValidationError >
    {
      if max_tokens <= 0
      {
        return Err(
          ValidationError::new( "max_tokens", "max_tokens must be positive" )
            .with_value( max_tokens.to_string() )
            .with_constraint( "max_tokens > 0" )
        );
      }

      // OpenAI's largest context windows are around 128k tokens
      if max_tokens > 131_072
      {
        return Err(
          ValidationError::new( "max_tokens", "max_tokens exceeds reasonable limit" )
            .with_value( max_tokens.to_string() )
            .with_constraint( "max_tokens <= 131072" )
        );
      }

      Ok( () )
    }

    /// Validate n parameter (number of completions, must be positive)
    ///
    /// # Errors
    ///
    /// Returns error if n is non-positive or exceeds 10
    #[ inline ]
    pub fn validate_n( n : i32 ) -> Result< (), ValidationError >
    {
      if n <= 0
      {
        return Err(
          ValidationError::new( "n", "n must be positive" )
            .with_value( n.to_string() )
            .with_constraint( "n > 0" )
        );
      }

      if n > 10
      {
        return Err(
          ValidationError::new( "n", "n exceeds reasonable limit" )
            .with_value( n.to_string() )
            .with_constraint( "n <= 10" )
        );
      }

      Ok( () )
    }

    /// Validate messages array (non-empty)
    ///
    /// # Errors
    ///
    /// Returns error if messages array is empty
    #[ inline ]
    pub fn validate_messages< T >( messages : &[ T ] ) -> Result< (), ValidationError >
    {
      if messages.is_empty()
      {
        return Err(
          ValidationError::new( "messages", "Messages array cannot be empty" )
            .with_constraint( "at least one message required" )
        );
      }
      Ok( () )
    }

    /// Validate `top_logprobs` (must be between 0 and 20)
    ///
    /// # Errors
    ///
    /// Returns error if `top_logprobs` is outside the range 0-20
    #[ inline ]
    pub fn validate_top_logprobs( top_logprobs : i32 ) -> Result< (), ValidationError >
    {
      if !( 0..=20 ).contains( &top_logprobs )
      {
        return Err(
          ValidationError::new( "top_logprobs", "top_logprobs must be between 0 and 20" )
            .with_value( top_logprobs.to_string() )
            .with_constraint( "0 <= top_logprobs <= 20" )
        );
      }
      Ok( () )
    }

    /// Validate input string or array for embeddings (non-empty)
    ///
    /// # Errors
    ///
    /// Returns error if input is empty or exceeds maximum length
    #[ inline ]
    pub fn validate_embedding_input( input : &str ) -> Result< (), ValidationError >
    {
      if input.is_empty()
      {
        return Err(
          ValidationError::new( "input", "Embedding input cannot be empty" )
            .with_constraint( "non-empty string or array" )
        );
      }

      // OpenAI has a limit of ~8k tokens per input
      if input.len() > 500_000
      {
        return Err(
          ValidationError::new( "input", "Embedding input exceeds maximum length" )
            .with_value( format!( "{} chars", input.len() ) )
            .with_constraint( "max ~500k characters" )
        );
      }

      Ok( () )
    }

    /// Validate dimensions parameter for embeddings (must be positive)
    ///
    /// # Errors
    ///
    /// Returns error if dimensions is zero or exceeds 4096
    #[ inline ]
    pub fn validate_dimensions( dimensions : u32 ) -> Result< (), ValidationError >
    {
      if dimensions == 0
      {
        return Err(
          ValidationError::new( "dimensions", "dimensions must be positive" )
            .with_value( dimensions.to_string() )
            .with_constraint( "dimensions > 0" )
        );
      }

      // Most embedding models use dimensions between 128 and 3072
      if dimensions > 4096
      {
        return Err(
          ValidationError::new( "dimensions", "dimensions exceeds typical limits" )
            .with_value( dimensions.to_string() )
            .with_constraint( "dimensions <= 4096" )
        );
      }

      Ok( () )
    }
  }
}

crate ::mod_interface!
{
  exposed use
  {
    ValidationError,
    Validate,
    validators,
  };
}
