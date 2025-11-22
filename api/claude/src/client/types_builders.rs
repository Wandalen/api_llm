// Builder implementations for client types
//
// This file is included into types.rs to reduce file size while keeping
// builder implementations alongside their type definitions.

impl ClientConfigBuilder
{
  /// Create new builder (no defaults)
  pub fn new() -> Self
  {
    Self
    {
      base_url : None,
      api_version : None,
      request_timeout : None,
      user_agent : None,
    }
  }

  /// Create builder with recommended values pre-filled
  pub fn with_recommended() -> Self
  {
    Self
    {
      base_url : Some( ANTHROPIC_API_BASE_URL.to_string() ),
      api_version : Some( ANTHROPIC_API_VERSION.to_string() ),
      request_timeout : Some( Duration::from_secs( 60 ) ),
      user_agent : Some( ANTHROPIC_USER_AGENT.to_string() ),
    }
  }

  /// Set base URL
  #[ must_use ]
  pub fn base_url< S : Into< String > >( mut self, base_url : S ) -> Self
  {
    self.base_url = Some( base_url.into() );
    self
  }

  /// Set API version
  #[ must_use ]
  pub fn api_version< S : Into< String > >( mut self, api_version : S ) -> Self
  {
    self.api_version = Some( api_version.into() );
    self
  }

  /// Set request timeout
  #[ must_use ]
  pub fn timeout( mut self, timeout : Duration ) -> Self
  {
    self.request_timeout = Some( timeout );
    self
  }

  /// Set user agent
  #[ must_use ]
  pub fn user_agent< S : Into< String > >( mut self, user_agent : S ) -> Self
  {
    self.user_agent = Some( user_agent.into() );
    self
  }

  /// Build the configuration (requires all values to be explicitly set)
  ///
  /// # Errors
  ///
  /// Returns `AnthropicError::InvalidArgument` if any required configuration values are not set
  pub fn build( self ) -> Result< ClientConfig, AnthropicError >
  {
    let base_url = self.base_url
      .ok_or_else( || AnthropicError::InvalidArgument( "base_url must be explicitly configured".to_string() ) )?;

    let api_version = self.api_version
      .ok_or_else( || AnthropicError::InvalidArgument( "api_version must be explicitly configured".to_string() ) )?;

    let request_timeout = self.request_timeout
      .ok_or_else( || AnthropicError::InvalidArgument( "request_timeout must be explicitly configured".to_string() ) )?;

    let user_agent = self.user_agent
      .ok_or_else( || AnthropicError::InvalidArgument( "user_agent must be explicitly configured".to_string() ) )?;

    Ok( ClientConfig
    {
      base_url,
      api_version,
      request_timeout,
      user_agent,
    })
  }
}

impl CreateMessageRequestBuilder
{
  /// Set the model to use for generation
  #[ inline ]
  #[ must_use ]
  pub fn model< S : Into< String > >( mut self, model : S ) -> Self
  {
    self.model = Some( model.into() );
    self
  }

  /// Set the maximum number of tokens to generate
  #[ inline ]
  #[ must_use ]
  pub fn max_tokens( mut self, max_tokens : u32 ) -> Self
  {
    self.max_tokens = Some( max_tokens );
    self
  }

  /// Add a message to the conversation
  #[ inline ]
  #[ must_use ]
  pub fn message( mut self, message : Message ) -> Self
  {
    self.messages.push( message );
    self
  }

  /// Add multiple messages to the conversation
  #[ inline ]
  #[ must_use ]
  pub fn messages( mut self, messages : Vec< Message > ) -> Self
  {
    self.messages.extend( messages );
    self
  }

  /// Set the system prompt (convenience method for simple string prompts)
  #[ inline ]
  #[ must_use ]
  pub fn system< S : Into< String > >( mut self, system : S ) -> Self
  {
    self.system = Some( vec![ SystemContent::text( system ) ] );
    self
  }

  /// Set the system prompt with cache control
  #[ inline ]
  #[ must_use ]
  pub fn system_with_cache( mut self, text : String, cache_control : CacheControl ) -> Self
  {
    self.system = Some( vec![ SystemContent
    {
      r#type : "text".to_string(),
      text,
      cache_control : Some( cache_control ),
    } ] );
    self
  }

  /// Set system prompt blocks directly
  #[ inline ]
  #[ must_use ]
  pub fn system_blocks( mut self, blocks : Vec< SystemContent > ) -> Self
  {
    self.system = Some( blocks );
    self
  }

  /// Set the temperature for sampling
  #[ inline ]
  #[ must_use ]
  pub fn temperature( mut self, temperature : f32 ) -> Self
  {
    self.temperature = Some( temperature );
    self
  }

  /// Set whether to stream the response
  #[ inline ]
  #[ must_use ]
  pub fn stream( mut self, stream : bool ) -> Self
  {
    self.stream = Some( stream );
    self
  }

  /// Set tools available for the model to use
  #[ cfg( feature = "tools" ) ]
  #[ inline ]
  #[ must_use ]
  pub fn tools( mut self, tools : Vec< ToolDefinition > ) -> Self
  {
    self.tools = Some( tools );
    self
  }

  /// Set how the model should use tools
  #[ cfg( feature = "tools" ) ]
  #[ inline ]
  #[ must_use ]
  pub fn tool_choice( mut self, tool_choice : ToolChoice ) -> Self
  {
    self.tool_choice = Some( tool_choice );
    self
  }

  /// Build the `CreateMessageRequest` (for backward compatibility)
  ///
  /// # Panics
  ///
  /// Panics if required fields are missing
  #[ inline ]
  #[ must_use ]
  pub fn build( self ) -> CreateMessageRequest
  {
    CreateMessageRequest
    {
      model : self.model.expect( "Model is required" ),
      max_tokens : self.max_tokens.expect( "Max tokens is required" ),
      messages : self.messages,
      system : self.system,
      temperature : self.temperature,
      stream : self.stream,
      #[ cfg( feature = "tools" ) ]
      tools : self.tools,
      #[ cfg( feature = "tools" ) ]
      tool_choice : self.tool_choice,
    }
  }

  /// Build and validate the `CreateMessageRequest`
  ///
  /// # Errors
  ///
  /// Returns an error if required fields are missing or validation fails
  #[ inline ]
  pub fn build_validated( self ) -> AnthropicResult< CreateMessageRequest >
  {
    let request = CreateMessageRequest
    {
      model : self.model.ok_or_else( ||
        AnthropicError::InvalidRequest( "model must be explicitly specified (use RECOMMENDED_MODEL for guidance)".to_string() )
      )?,
      max_tokens : self.max_tokens.ok_or_else( ||
        AnthropicError::InvalidRequest( "max_tokens is required".to_string() )
      )?,
      messages : self.messages,
      system : self.system,
      temperature : self.temperature,
      stream : self.stream,
      #[ cfg( feature = "tools" ) ]
      tools : self.tools,
      #[ cfg( feature = "tools" ) ]
      tool_choice : self.tool_choice,
    };

    request.validate()?;
    Ok( request )
  }
}
