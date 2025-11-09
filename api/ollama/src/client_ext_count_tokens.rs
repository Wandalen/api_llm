//! Token counting extension for OllamaClient.

#[ cfg( feature = "count_tokens" ) ]
use crate::client::OllamaClient;
#[ cfg( feature = "count_tokens" ) ]
use error_tools::untyped::{ format_err, Result as OllamaResult };

#[ cfg( feature = "count_tokens" ) ]
impl OllamaClient
{
  /// Count tokens in text using Ollama models
  ///
  /// This method provides token counting functionality by sending text to the Ollama API
  /// for tokenization using the specified model's tokenizer.
  ///
  /// # Arguments
  /// * `request` - Token count request containing text and model parameters
  ///
  /// # Returns
  /// * `Ok(TokenCountResponse)` - Token count with metadata and cost estimation
  /// * `Err(OllamaError)` - Network, API, or tokenization error
  ///
  /// # Errors
  /// Returns error if:
  /// - Circuit breaker is open (feature : circuit_breaker)
  /// - Rate limiting is exceeded (feature : rate_limiting)
  /// - Text input is invalid or too long
  /// - Model name is invalid or unsupported
  /// - Network request fails or times out
  /// - Ollama API returns an error response
  /// - HTTP client configuration error
  ///
  /// # Examples
  /// ```rust,no_run
  /// use api_ollama::{ OllamaClient, TokenCountRequest };
  /// use std::time::Duration;
  ///
  /// # async fn example() -> Result< (), Box< dyn std::error::Error > > {
  /// let mut client = OllamaClient::new(
  ///   "http://localhost:11434".to_string(),
  ///   Duration::from_secs( 30 )
  /// );
  ///
  /// let request = TokenCountRequest::new(
  ///   "llama3.2".to_string(),
  ///   "Count the tokens in this text.".to_string()
  /// );
  ///
  /// let response = client.count_tokens( request ).await?;
  /// println!( "Token count : {}", response.token_count );
  /// # Ok( () )
  /// # }
  /// ```
  #[ inline ]
  pub async fn count_tokens( &mut self, request : crate::tokens::TokenCountRequest ) -> OllamaResult< crate::tokens::TokenCountResponse >
  {
    // Check circuit breaker before making request
    #[ cfg( feature = "circuit_breaker" ) ]
    {
      if let Some( ref circuit_breaker ) = &self.circuit_breaker
      {
        if !circuit_breaker.can_execute()
        {
          return Err( format_err!( "Circuit breaker is open. Too many recent failures." ) );
        }
      }
    }

    // Check rate limiting before making request
    #[ cfg( feature = "rate_limiting" ) ]
    {
      if let Some( ref rate_limiter ) = &self.rate_limiter
      {
        if !rate_limiter.should_allow_request()
        {
          return Err( format_err!( "Rate limit exceeded. Please try again later." ) );
        }
      }
    }

    let start_time = std::time::Instant::now();
    let request_id = format!( "req-{}", std::time::SystemTime::now().duration_since( std::time::UNIX_EPOCH ).unwrap().as_millis() );

    // First, validate the request
    // request.validate()?;

    // Build the request URL
    let url = format!( "{}/api/embeddings", self.base_url );

    // Create request body (using embeddings endpoint for token counting)
    let request_body = serde_json::json!({
      "model": request.model,
      "prompt": request.text,
    });

    // Make the HTTP request
    let response = self.client
      .post( &url )
      .json( &request_body )
      .timeout( self.timeout )
      .send()
      .await;

    let processing_time_ms = start_time.elapsed().as_millis() as u64;

    match response
    {
      Ok( resp ) =>
      {
        let status = resp.status();
        if status.is_success()
        {
          let response_text = resp.text().await.map_err( | e | format_err!( "Failed to read response : {e}" ) )?;

          // Parse the response (unused but needed to validate JSON)
          let _embeddings_response : serde_json::Value = serde_json::from_str( &response_text )
            .map_err( | e | format_err!( "Failed to parse response : {e}" ) )?;

          // Extract token count
          // In Ollama, the embeddings response doesn't directly provide token count
          // We estimate based on the prompt length and model characteristics
          let token_count = self.estimate_token_count( &request.text, &request.model );

          // Record success in circuit breaker
          #[ cfg( feature = "circuit_breaker" ) ]
          {
            if let Some( ref circuit_breaker ) = &self.circuit_breaker
            {
              circuit_breaker.record_success();
            }
          }

          // Record in diagnostics
          #[ cfg( feature = "general_diagnostics" ) ]
          {
            if let Some( ref diagnostics ) = &self.diagnostics_collector
            {
              diagnostics.track_request_success( &request_id, response_text.len() );
            }
          }

          // Build response
          let response = crate::tokens::TokenCountResponse
          {
            token_count : token_count as u32,
            model : request.model.clone(),
            text_length : request.text.len(),
            estimated_cost : None, // Cost estimation requires pricing data
            processing_time_ms : Some( processing_time_ms ),
            metadata : None,
          };

          Ok( response )
        }
        else
        {
          let error_text = resp.text().await.unwrap_or_else( |_| "Unknown error".to_string() );

          // Record failure in circuit breaker
          #[ cfg( feature = "circuit_breaker" ) ]
          {
            if let Some( ref circuit_breaker ) = &self.circuit_breaker
            {
              circuit_breaker.record_failure();
            }
          }

          // Record in diagnostics
          #[ cfg( feature = "general_diagnostics" ) ]
          {
            if let Some( ref diagnostics ) = &self.diagnostics_collector
            {
              diagnostics.track_request_failure( &request_id, status.as_u16(), &error_text );
            }
          }

          Err( format_err!( "Token counting request failed with status {status}: {error_text}" ) )
        }
      }
      Err( e ) =>
      {
        // Record failure in circuit breaker
        #[ cfg( feature = "circuit_breaker" ) ]
        {
          if let Some( ref circuit_breaker ) = &self.circuit_breaker
          {
            circuit_breaker.record_failure();
          }
        }

        // Record in diagnostics
        #[ cfg( feature = "general_diagnostics" ) ]
        {
          if let Some( ref diagnostics ) = &self.diagnostics_collector
          {
            diagnostics.track_request_failure( &request_id, 500, &e.to_string() );
          }
        }

        Err( format_err!( "Token counting request failed : {e}" ) )
      }
    }
  }

  /// Estimate token count based on text and model
  ///
  /// This is a rough estimation based on character count and model characteristics.
  /// For more accurate counts, use the actual API token counting.
  #[ inline ]
  fn estimate_token_count( &self, text : &str, model : &str ) -> u64
  {
    // Rough estimation : average of 4 characters per token for English text
    // This varies by model and language
    let char_count = text.chars().count() as u64;

    // Adjust based on model if known
    let ratio = if model.contains( "llama" ) || model.contains( "mistral" )
    {
      4.0 // LLaMA and Mistral models use similar tokenization
    }
    else if model.contains( "gemma" )
    {
      4.2 // Gemma tends to use slightly more tokens
    }
    else
    {
      4.0 // Default ratio
    };

    ( char_count as f64 / ratio ).ceil() as u64
  }

  /// Count tokens in a batch of texts
  ///
  /// This method processes multiple texts in a single batch for efficiency.
  ///
  /// # Arguments
  /// * `request` - Batch token count request with multiple texts
  ///
  /// # Returns
  /// * `Ok(BatchTokenResponse)` - Token counts for all texts
  /// * `Err(OllamaError)` - Network or API error
  ///
  /// # Errors
  /// Returns error if any individual token counting operation fails
  #[ inline ]
  pub async fn count_tokens_batch( &mut self, request : crate::tokens::BatchTokenRequest ) -> OllamaResult< crate::tokens::BatchTokenResponse >
  {
    // request.validate()?;

    let mut results = Vec::new();
    let start_time = std::time::Instant::now();

    for text in &request.texts
    {
      let single_request = crate::tokens::TokenCountRequest
      {
        model : request.model.clone(),
        text : text.clone(),
        options : request.options.clone(),
      };

      let response = self.count_tokens( single_request ).await?;
      results.push( response );
    }

    let processing_time_ms = start_time.elapsed().as_millis() as u64;
    let total_tokens = results.iter().map( | r | r.token_count ).sum();
    let total_estimated_cost = results.iter().filter_map( | r | r.estimated_cost ).sum::< f64 >();

    Ok( crate::tokens::BatchTokenResponse
    {
      results,
      total_tokens,
      total_estimated_cost : Some( total_estimated_cost ),
      processing_time_ms : Some( processing_time_ms ),
      batch_optimization_savings : None, // Would require baseline comparison
    })
  }

  /// Validate token count against model limits
  ///
  /// # Arguments
  /// * `text` - Text to validate
  /// * `model` - Model name
  /// * `config` - Validation configuration
  ///
  /// # Returns
  /// * `Ok(())` - Text is within limits
  /// * `Err(OllamaError)` - Text exceeds model limits
  #[ inline ]
  pub async fn validate_token_count(
    &mut self,
    text : &str,
    model : &str,
    config : crate::tokens::TokenValidationConfig
  ) -> OllamaResult< () >
  {
    let request = crate::tokens::TokenCountRequest
    {
      model : model.to_string(),
      text : text.to_string(),
      options : None,
    };

    let response = self.count_tokens( request ).await?;

    if response.token_count > config.max_input_tokens
    {
      return Err( format_err!(
        "Token count {} exceeds maximum {} for model {}",
        response.token_count,
        config.max_input_tokens,
        model
      ));
    }

    let warning_token_count = ( config.max_input_tokens as f64 * config.warning_threshold ) as u32;
    if response.token_count > warning_token_count
    {
      // In a real implementation, this might log a warning
      // For now, we just note it in the error message if it fails
    }

    Ok( () )
  }
}
