//! Ollama HTTP client implementation.
//!
//! Provides the main OllamaClient for interacting with Ollama API endpoints,
//! including core request functionality.

#[ cfg( feature = "enabled" ) ]
mod private
{
  use core::time::Duration;
  use super::super::*;
  use error_tools::format_err;

  /// Ollama HTTP client
  #[ derive( Debug, Clone ) ]
  pub struct OllamaClient
  {
    pub( crate ) base_url : String,
    pub( crate ) timeout : Duration,
    pub( crate ) client : reqwest::Client,
    #[ cfg( feature = "secret_management" ) ]
    pub( crate ) secret_store : Option< SecretStore >,
    #[ cfg( feature = "circuit_breaker" ) ]
    pub( crate ) circuit_breaker : Option< CircuitBreaker >,
    #[ cfg( feature = "request_caching" ) ]
    pub( crate ) request_cache : Option< RequestCache >,
    #[ cfg( feature = "general_diagnostics" ) ]
    pub( crate ) diagnostics_collector : Option< DiagnosticsCollector >,
    #[ cfg( feature = "failover" ) ]
    pub( crate ) failover_manager : Option< std::sync::Arc< std::sync::Mutex< FailoverManager > > >,
    #[ cfg( feature = "health_checks" ) ]
    pub( crate ) health_check_manager : Option< std::sync::Arc< std::sync::Mutex< crate::health_checks::HealthCheckManager > > >,
    #[ cfg( feature = "retry" ) ]
    pub( crate ) retry_client : Option< RetryableHttpClient >,
    #[ cfg( feature = "rate_limiting" ) ]
    pub( crate ) rate_limiter : Option< RateLimiter >,
    #[ cfg( feature = "audio_processing" ) ]
    pub( crate ) audio_config : Option< crate::audio::AudioProcessingConfig >,
    #[ cfg( feature = "cached_content" ) ]
    #[ allow( dead_code ) ]
    pub( crate ) content_cache_manager : Option< crate::cached_content::IntelligentCacheManager >,
  }

  impl OllamaClient
  {
    /// Create a new Ollama client with the given base URL and timeout
    ///
    /// Note : Timeout must be explicitly configured. Use recommended timeouts:
    /// - `OllamaClient::recommended_timeout_default()` for general use (120s)
    /// - `OllamaClient::recommended_timeout_fast()` for quick operations (30s)
    /// - `OllamaClient::recommended_timeout_slow()` for heavy models (300s)
    #[ inline ]
    #[ must_use ]
    pub fn new( base_url : String, timeout : Duration ) -> Self
    {
      Self
      {
        base_url,
        timeout,
        client : reqwest::Client::new(),
        #[ cfg( feature = "secret_management" ) ]
        secret_store : None,
        #[ cfg( feature = "circuit_breaker" ) ]
        circuit_breaker : None,
        #[ cfg( feature = "request_caching" ) ]
        request_cache : None,
        #[ cfg( feature = "general_diagnostics" ) ]
        diagnostics_collector : None,
        #[ cfg( feature = "failover" ) ]
        failover_manager : None,
        #[ cfg( feature = "health_checks" ) ]
        health_check_manager : None,
        #[ cfg( feature = "retry" ) ]
        retry_client : None,
        #[ cfg( feature = "rate_limiting" ) ]
        rate_limiter : None,
        #[ cfg( feature = "audio_processing" ) ]
        audio_config : None,
        #[ cfg( feature = "cached_content" ) ]
        content_cache_manager : None,
      }
    }

    /// Set custom timeout for requests
    #[ inline ]
    #[ must_use ]
    pub fn with_timeout( mut self, timeout : Duration ) -> Self
    {
      self.timeout = timeout;
      self
    }

    /// Recommended timeout for general use (120 seconds)
    ///
    /// This is suitable for most text generation and chat operations
    #[ inline ]
    #[ must_use ]
    pub fn recommended_timeout_default() -> Duration
    {
      Duration::from_secs( 120 )
    }

    /// Recommended timeout for fast operations (30 seconds)
    ///
    /// This is suitable for model listing and lightweight operations
    #[ inline ]
    #[ must_use ]
    pub fn recommended_timeout_fast() -> Duration
    {
      Duration::from_secs( 30 )
    }

    /// Recommended timeout for slow operations (300 seconds)
    ///
    /// This is suitable for heavy model operations and large generations
    #[ inline ]
    #[ must_use ]
    pub fn recommended_timeout_slow() -> Duration
    {
      Duration::from_secs( 300 )
    }

    /// Check if Ollama is available by pinging the tags endpoint
    #[ inline ]
    pub async fn is_available( &mut self ) -> bool
    {
      let url = format!( "{}/api/tags", self.base_url );
      let request_builder = self.client.get( &url );
      #[ cfg( feature = "secret_management" ) ]
      let request_builder = self.apply_authentication( request_builder );
      #[ cfg( not( feature = "secret_management" ) ) ]
      let request_builder = request_builder;

      match request_builder.send().await
      {
        Ok( response ) => response.status().is_success(),
        Err( _ ) => false,
      }
    }

    /// List available models
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response is invalid
    #[ inline ]
    pub async fn list_models( &mut self ) -> OllamaResult< TagsResponse >
    {
      let url = format!( "{}/api/tags", self.base_url );

      let request_builder = self.client.get( &url ).timeout( self.timeout );
      #[ cfg( feature = "secret_management" ) ]
      let request_builder = self.apply_authentication( request_builder );
      #[ cfg( not( feature = "secret_management" ) ) ]
      let request_builder = request_builder;

      let response = request_builder
        .send()
        .await
        .map_err( | e | format_err!( "Network error : {}", e ) )?;

      if !response.status().is_success()
      {
        return Err( format_err!( "API error {}: Failed to list models : {}", response.status().as_u16(), response.status() ) );
      }

      let tags : TagsResponse = response.json().await.map_err( | e | format_err!( "Parse error : {}", e ) )?;
      Ok( tags )
    }

    /// Send chat completion request
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response is invalid
    #[ inline ]
    #[ allow( clippy::too_many_lines ) ]
    pub async fn chat( &mut self, request : ChatRequest ) -> OllamaResult< ChatResponse >
    {
      // Validate request before processing
      #[ cfg( feature = "input_validation" ) ]
      {
        use crate::input_validation::Validate;
        if let Err( validation_errors ) = request.validate()
        {
          let error_messages : Vec< String > = validation_errors
            .iter()
            .map( | e | format!( "{}", e ) )
            .collect();
          return Err( format_err!( "Request validation failed : {}", error_messages.join( "; " ) ) );
        }
      }

      // Start performance metrics recording
      #[ cfg( feature = "general_diagnostics" ) ]
      let _start_time = std::time::Instant::now();

      // Check circuit breaker before making request
      #[ cfg( feature = "circuit_breaker" ) ]
      {
        if let Some( ref circuit_breaker ) = &self.circuit_breaker
        {
          if !circuit_breaker.can_execute()
          {
            return Err( format_err!( "Circuit breaker is open - requests are currently blocked" ) );
          }
        }
      }

      // Check rate limiter before making request
      #[ cfg( feature = "rate_limiting" ) ]
      {
        if let Some( ref rate_limiter ) = &self.rate_limiter
        {
          if !rate_limiter.should_allow_request()
          {
            return Err( format_err!( "Rate limit exceeded - request rejected" ) );
          }
        }
      }

      #[ cfg( feature = "failover" ) ]
      {
        // If failover is enabled, try multiple endpoints
        if let Some( ref failover_manager ) = &self.failover_manager
        {
          let max_attempts = failover_manager.lock().map( |manager| manager.get_endpoint_count() ).unwrap_or( 1 );
          let mut last_error = format_err!( "No endpoints available" );

          for _attempt in 0..max_attempts
          {
            let current_url = self.get_active_endpoint();
            let url = format!( "{current_url}/api/chat" );

            let request_builder = self.client
              .post( &url )
              .header( "Content-Type", "application/json" )
              .json( &request )
              .timeout( self.timeout );
            #[ cfg( feature = "secret_management" ) ]
            let request_builder = self.apply_authentication( request_builder );
            #[ cfg( not( feature = "secret_management" ) ) ]
            let request_builder = request_builder;

            match request_builder.send().await
            {
              Ok( response ) =>
              {
                if response.status().is_success()
                {
                  let chat_response : ChatResponse = response.json().await.map_err( | e | format_err!( "Parse error : {}", e ) )?;

                  // Record success in circuit breaker
                  #[ cfg( feature = "circuit_breaker" ) ]
                  {
                    if let Some( ref circuit_breaker ) = &self.circuit_breaker
                    {
                      circuit_breaker.record_success();
                    }
                  }

                  return Ok( chat_response );
                }
                last_error = format_err!( "API error {}: Chat request failed : {}", response.status().as_u16(), response.status() );
              },
              Err( e ) =>
              {
                last_error = format_err!( "Network error : {}", e );

                // Mark current endpoint as unhealthy
                if let Some( ref failover_manager ) = &self.failover_manager
                {
                  if let Ok( mut manager ) = failover_manager.lock()
                  {
                    manager.mark_endpoint_unhealthy( &current_url );
                    manager.select_next_healthy_endpoint();
                  }
                }
              }
            }
          }

          // Record failure in circuit breaker
          #[ cfg( feature = "circuit_breaker" ) ]
          {
            if let Some( ref mut circuit_breaker ) = &mut self.circuit_breaker
            {
              circuit_breaker.record_failure();
            }
          }

          // If we get here, all endpoints failed
          return Err( format_err!( "All failover endpoints failed. Last error : {}", last_error ) );
        }
      }

      // Non-failover path (original implementation)
      let url = format!( "{}/api/chat", self.base_url );

      let request_builder = self.client
        .post( &url )
        .header( "Content-Type", "application/json" )
        .json( &request )
        .timeout( self.timeout );
      #[ cfg( feature = "secret_management" ) ]
      let request_builder = self.apply_authentication( request_builder );
      #[ cfg( not( feature = "secret_management" ) ) ]
      let request_builder = request_builder;

      let response = request_builder
        .send()
        .await;

      match response
      {
        Ok( response ) =>
        {
          if response.status().is_success()
          {
            let chat_response : ChatResponse = response.json().await.map_err( | e | format_err!( "Parse error : {}", e ) )?;

            // Record success in circuit breaker
            #[ cfg( feature = "circuit_breaker" ) ]
            {
              if let Some( ref circuit_breaker ) = &self.circuit_breaker
              {
                circuit_breaker.record_success();
              }
            }

            Ok( chat_response )
          }
          else
          {
            let error = format_err!( "API error {}: Chat request failed : {}", response.status().as_u16(), response.status() );

            // Record failure in circuit breaker
            #[ cfg( feature = "circuit_breaker" ) ]
            {
              if let Some( ref circuit_breaker ) = &self.circuit_breaker
              {
                circuit_breaker.record_failure();
              }
            }

            Err( error )
          }
        },
        Err( e ) =>
        {
          let error = format_err!( "Network error : {}", e );

          // Record failure in circuit breaker
          #[ cfg( feature = "circuit_breaker" ) ]
          {
            if let Some( ref mut circuit_breaker ) = &mut self.circuit_breaker
            {
              circuit_breaker.record_failure();
            }
          }

          Err( error )
        }
      }
    }

    /// Send text generation request
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response is invalid
    #[ inline ]
    pub async fn generate( &mut self, request : GenerateRequest ) -> OllamaResult< GenerateResponse >
    {
      // Validate request before processing
      #[ cfg( feature = "input_validation" ) ]
      {
        use crate::input_validation::Validate;
        if let Err( validation_errors ) = request.validate()
        {
          let error_messages : Vec< String > = validation_errors
            .iter()
            .map( | e | format!( "{}", e ) )
            .collect();
          return Err( format_err!( "Request validation failed : {}", error_messages.join( "; " ) ) );
        }
      }

      // Check circuit breaker before making request
      #[ cfg( feature = "circuit_breaker" ) ]
      {
        if let Some( ref circuit_breaker ) = &self.circuit_breaker
        {
          if !circuit_breaker.can_execute()
          {
            return Err( format_err!( "Circuit breaker is open - requests are currently blocked" ) );
          }
        }
      }

      // Check rate limiter before making request
      #[ cfg( feature = "rate_limiting" ) ]
      {
        if let Some( ref rate_limiter ) = &self.rate_limiter
        {
          if !rate_limiter.should_allow_request()
          {
            return Err( format_err!( "Rate limit exceeded - request rejected" ) );
          }
        }
      }

      let url = format!( "{}/api/generate", self.base_url );

      let request_builder = self.client
        .post( &url )
        .header( "Content-Type", "application/json" )
        .json( &request )
        .timeout( self.timeout );
      #[ cfg( feature = "secret_management" ) ]
      let request_builder = self.apply_authentication( request_builder );
      #[ cfg( not( feature = "secret_management" ) ) ]
      let request_builder = request_builder;

      let response = request_builder
        .send()
        .await;

      match response
      {
        Ok( response ) =>
        {
          if response.status().is_success()
          {
            let generate_response : GenerateResponse = response.json().await.map_err( | e | format_err!( "Parse error : {}", e ) )?;

            // Record success in circuit breaker
            #[ cfg( feature = "circuit_breaker" ) ]
            {
              if let Some( ref circuit_breaker ) = &self.circuit_breaker
              {
                circuit_breaker.record_success();
              }
            }

            Ok( generate_response )
          }
          else
          {
            let error = format_err!( "API error {}: Generate request failed : {}", response.status().as_u16(), response.status() );

            // Record failure in circuit breaker
            #[ cfg( feature = "circuit_breaker" ) ]
            {
              if let Some( ref circuit_breaker ) = &self.circuit_breaker
              {
                circuit_breaker.record_failure();
              }
            }

            Err( error )
          }
        },
        Err( e ) =>
        {
          let error = format_err!( "Network error : {}", e );

          // Record failure in circuit breaker
          #[ cfg( feature = "circuit_breaker" ) ]
          {
            if let Some( ref mut circuit_breaker ) = &mut self.circuit_breaker
            {
              circuit_breaker.record_failure();
            }
          }

          Err( error )
        }
      }
    }

    /// Get model information
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response is invalid
    #[ inline ]
    pub async fn model_info( &mut self, name : String ) -> OllamaResult< ModelInfo >
    {
      let url = format!( "{}/api/show", self.base_url );
      let request = serde_json::json!({ "name": name });

      let request_builder = self.client
        .post( &url )
        .header( "Content-Type", "application/json" )
        .json( &request )
        .timeout( self.timeout );
      #[ cfg( feature = "secret_management" ) ]
      let request_builder = self.apply_authentication( request_builder );
      #[ cfg( not( feature = "secret_management" ) ) ]
      let request_builder = request_builder;

      let response = request_builder
        .send()
        .await
        .map_err( | e | format_err!( "Network error : {}", e ) )?;

      if !response.status().is_success()
      {
        return Err( format_err!( "API error {}: Model info request failed : {}", response.status().as_u16(), response.status() ) );
      }

      let model_info : ModelInfo = response.json().await.map_err( | e | format_err!( "Parse error : {}", e ) )?;
      Ok( model_info )
    }

    /// Generate embeddings for text input
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response is invalid
    #[ cfg( feature = "embeddings" ) ]
    #[ inline ]
    pub async fn embeddings( &mut self, request : EmbeddingsRequest ) -> OllamaResult< EmbeddingsResponse >
    {
      // Validate request before processing
      #[ cfg( feature = "input_validation" ) ]
      {
        use crate::input_validation::Validate;
        if let Err( validation_errors ) = request.validate()
        {
          let error_messages : Vec< String > = validation_errors
            .iter()
            .map( | e | format!( "{}", e ) )
            .collect();
          return Err( format_err!( "Request validation failed : {}", error_messages.join( "; " ) ) );
        }
      }

      let url = format!( "{}/api/embeddings", self.base_url );

      let request_builder = self.client
        .post( &url )
        .header( "Content-Type", "application/json" )
        .json( &request )
        .timeout( self.timeout );
      #[ cfg( feature = "secret_management" ) ]
      let request_builder = self.apply_authentication( request_builder );
      #[ cfg( not( feature = "secret_management" ) ) ]
      let request_builder = request_builder;

      let response = request_builder
        .send()
        .await
        .map_err( | e | format_err!( "Network error : {}", e ) )?;

      if !response.status().is_success()
      {
        return Err( format_err!( "API error {}: Embeddings request failed : {}", response.status().as_u16(), response.status() ) );
      }

      let embeddings_response : EmbeddingsResponse = response.json().await.map_err( | e | format_err!( "Parse error : {}", e ) )?;
      Ok( embeddings_response )
    }

    /// Get the base URL of this client
    #[ inline ]
    #[ must_use ]
    pub fn base_url( &self ) -> &str
    {
      &self.base_url
    }

    /// Check if this client has authentication configured
    #[ inline ]
    #[ must_use ]
    pub fn has_authentication( &self ) -> bool
    {
      #[ cfg( feature = "secret_management" ) ]
      {
        if let Some( store ) = &self.secret_store
        {
          return store.contains( "api_key" ) || store.contains( "auth_token" );
        }
      }
      false
    }
  }

  impl Default for OllamaClient
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new( "http://localhost:11434".to_string(), Self::recommended_timeout_default() )
    }
  }
}

#[ cfg( feature = "enabled" ) ]
crate ::mod_interface!
{
  exposed use
  {
    OllamaClient,
  };
}
