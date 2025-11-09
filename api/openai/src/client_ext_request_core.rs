// src/client_ext_request_core.rs
//! Client request execution core methods.
//!
//! This module extends the `Client` with internal HTTP request execution
//! methods that handle retry logic, circuit breakers, and error handling.

mod private
{
  use crate::
  {
    client ::Client,
    environment ::{ OpenaiEnvironment, EnvironmentInterface },
    error ::{ OpenAIError, Result, ApiErrorWrap },
  };

  #[ cfg( feature = "retry" ) ]
  use crate::enhanced_retry::{ EnhancedRetryConfig, EnhancedRetryExecutor };

  #[ cfg( feature = "circuit_breaker" ) ]
  use crate::enhanced_circuit_breaker::{ EnhancedCircuitBreaker };

  impl< E > Client< E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Central HTTP request handler following "Thin Client, Rich API" principles
    ///
    /// This method makes a single HTTP request without any automatic behaviors.
    /// It handles basic error parsing but leaves retry decisions to the developer.
    /// Following the governing principle : zero client-side intelligence.
    pub(in crate) async fn execute_request< F, Fut >( &self, request_builder : F ) -> Result< reqwest::Response >
    where
      F : FnOnce() -> Fut + Send,
      Fut : core::future::Future< Output = core::result::Result< reqwest::Response, reqwest::Error > > + Send,
    {
      let response = request_builder().await
        .map_err( OpenAIError::from )?;

      let status = response.status();
      if status.is_client_error()
      {
        let bytes = response.bytes().await
          .map_err( OpenAIError::from )?;

        // Try to parse as API error first
        if let Ok( wrapped_error ) = serde_json::from_slice::< ApiErrorWrap >( &bytes )
        {
          Err( OpenAIError::Api( wrapped_error.error ).into() )
        }
        else
        {
          // If not valid JSON, create a generic error with the response content
          let body_text = String::from_utf8_lossy( &bytes );
          let error_msg = format!( "HTTP {status} error : {body_text}" );
          Err( OpenAIError::Http( error_msg ).into() )
        }
      }
      else if status.is_server_error()
      {
        Err( OpenAIError::Http( format!( "Server error : {status}" ) ).into() )
      }
      else
      {
        Ok( response )
      }
    }

    /// Enhanced HTTP request handler with optional retry and circuit breaker logic
    ///
    /// This method wraps `execute_request` with optional reliability features when configured.
    /// Supports retry, circuit breaker, or both combined. Follows "Thin Client, Rich API" principles.
    pub(in crate) async fn execute_request_with_retry< F, Fut >( &self, request_builder : F ) -> Result< reqwest::Response >
    where
      F : Fn() -> Fut + Send + Sync,
      Fut : core::future::Future< Output = core::result::Result< reqwest::Response, reqwest::Error > > + Send,
    {
      // Determine which reliability features are available and configured

      #[ cfg(feature = "rate_limiting") ]
      let has_rate_limiting = self.rate_limiter.is_some();
      #[ cfg(not(feature = "rate_limiting")) ]
      let has_rate_limiting = false;

      // Apply rate limiting as the outermost layer if enabled
      if has_rate_limiting
      {
        #[ cfg( feature = "rate_limiting" ) ]
        {
          if let Some( rate_limiter ) = &self.rate_limiter
          {
            return rate_limiter.execute( || async {
              self.execute_request_with_reliability_features( &request_builder ).await
            } ).await;
          }
        }
      }

      // Fall back to reliability features without rate limiting
      self.execute_request_with_reliability_features( &request_builder ).await
    }

    /// Execute request with retry and circuit breaker features (without rate limiting)
    pub(in crate) async fn execute_request_with_reliability_features< F, Fut >( &self, request_builder : &F ) -> Result< reqwest::Response >
    where
      F : Fn() -> Fut + Send + Sync,
      Fut : core::future::Future< Output = core::result::Result< reqwest::Response, reqwest::Error > > + Send,
    {
      // Determine which reliability features are available and configured
      #[ cfg(feature = "retry") ]
      let has_retry = self.retry_config.is_some();
      #[ cfg(not(feature = "retry")) ]
      let has_retry = false;

      #[ cfg(feature = "circuit_breaker") ]
      let has_circuit_breaker = self.circuit_breaker.is_some();
      #[ cfg(not(feature = "circuit_breaker")) ]
      let has_circuit_breaker = false;

      match ( has_retry, has_circuit_breaker )
      {
        // Both retry and circuit breaker enabled - circuit breaker wraps retry
        ( true, true ) =>
        {
          #[ cfg( all( feature = "retry", feature = "circuit_breaker" ) ) ]
          {
            if let ( Some( retry_config ), Some( circuit_breaker ) ) = ( &self.retry_config, &self.circuit_breaker )
            {
              return circuit_breaker.execute( || self.execute_with_retry( &request_builder, retry_config ) ).await;
            }
          }
          // Fallback to standard execution if feature compilation doesn't match runtime state
          self.execute_request( request_builder ).await
        },
        // Only retry enabled
        ( true, false ) =>
        {
          #[ cfg( feature = "retry" ) ]
          {
            if let Some( retry_config ) = &self.retry_config
            {
              return self.execute_with_retry( request_builder, retry_config ).await;
            }
          }
          self.execute_request( request_builder ).await
        },
        // Only circuit breaker enabled
        ( false, true ) =>
        {
          #[ cfg( feature = "circuit_breaker" ) ]
          {
            if let Some( circuit_breaker ) = &self.circuit_breaker
            {
              return self.execute_with_circuit_breaker( request_builder, circuit_breaker ).await;
            }
          }
          self.execute_request( request_builder ).await
        },
        // No reliability features enabled
        ( false, false ) =>
        {
          self.execute_request( request_builder ).await
        }
      }
    }

    /// Execute request with retry logic (only when retry feature is enabled)
    #[ cfg( feature = "retry" ) ]
    async fn execute_with_retry< F, Fut >( &self, request_builder : F, retry_config : &EnhancedRetryConfig ) -> Result< reqwest::Response >
    where
      F : Fn() -> Fut + Send + Sync,
      Fut : core::future::Future< Output = core::result::Result< reqwest::Response, reqwest::Error > > + Send,
    {
      let executor = EnhancedRetryExecutor::new( retry_config.clone() )
        .map_err( |e| OpenAIError::InvalidArgument( format!( "Invalid retry configuration : {e}" ) ) )?;

      executor.execute( || self.execute_request( &request_builder ) ).await
    }

    /// Execute request with circuit breaker logic (only when `circuit_breaker` feature is enabled)
    #[ cfg( feature = "circuit_breaker" ) ]
    async fn execute_with_circuit_breaker< F, Fut >( &self, request_builder : F, circuit_breaker : &EnhancedCircuitBreaker ) -> Result< reqwest::Response >
    where
      F : Fn() -> Fut + Send + Sync,
      Fut : core::future::Future< Output = core::result::Result< reqwest::Response, reqwest::Error > > + Send,
    {
      circuit_breaker.execute( || self.execute_request( &request_builder ) ).await
    }
  }

} // end mod private
