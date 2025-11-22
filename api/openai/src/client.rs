// src/client.rs
//! This module defines the `Client` structure for interacting with the `OpenAI` API.
//! It provides methods for making various API requests, handling authentication,
//! and managing HTTP communication.

// Re-export API accessor trait so it's available wherever Client is used
pub use crate::client_api_accessors::ClientApiAccessors;

/// Define a private namespace for all its items.
mod private
{
  // Use crate root for base access
  use crate::
  {
    environment ::{ OpenaiEnvironment, EnvironmentInterface },
    diagnostics ::DiagnosticsCollector,
    request_cache ::ApiRequestCache,
  };

  // Import enhanced functionality when features are enabled
  #[ cfg( feature = "retry" ) ]
  use crate::enhanced_retry::EnhancedRetryConfig;

  #[ cfg( feature = "circuit_breaker" ) ]
  use crate::enhanced_circuit_breaker::{ EnhancedCircuitBreakerConfig, EnhancedCircuitBreaker };

  #[ cfg( feature = "rate_limiting" ) ]
  use crate::enhanced_rate_limiting::{ EnhancedRateLimitingConfig, EnhancedRateLimiter };

  // External crates
  use reqwest::Client as HttpClient;
  use std::sync::Arc;

  /// The main client for interacting with the `OpenAI` API.
  ///
  /// Provides methods for accessing different API categories like
  /// assistants, chat, embeddings, etc.
  ///
  /// # Governing Principle : "Thin Client, Rich API"
  ///
  /// This client follows the "Thin Client, Rich API" principle - it exposes all
  /// server-side functionality transparently while maintaining zero client-side
  /// intelligence or automatic behaviors. All operations are explicit and
  /// developer-controlled, with clear separation between information retrieval
  /// and state-changing actions.
  ///
  /// # Example
  ///
  /// ```no_run
  /// use api_openai::{ Client, environment::{ OpenaiEnvironmentImpl, OpenAIRecommended }, Secret, ClientApiAccessors };
  ///
  /// # async fn example() -> Result<(), Box< dyn core::error::Error > > {
  /// let secret = Secret::load_from_env("OPENAI_API_KEY")?;
  /// let env = OpenaiEnvironmentImpl::build(secret, None, None, OpenAIRecommended::base_url().to_string(), OpenAIRecommended::realtime_base_url().to_string())?;
  /// let client = Client::build(env)?;
  ///
  /// // Use the client to access different APIs
  /// let models = client.models().list().await?;
  /// # Ok(())
  /// # }
  /// ```
  #[ derive( Debug, Clone ) ]
  pub struct Client< E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// The HTTP client used for making requests.
    pub http_client : HttpClient,
    /// The `OpenAI` environment configuration.
    pub environment : E,
    /// Optional diagnostics collector for monitoring requests.
    pub diagnostics : Option< Arc< DiagnosticsCollector > >,
    /// Optional request cache for API responses.
    pub cache : Option< Arc< ApiRequestCache > >,

    // Feature-gated enhanced reliability configurations and instances
    #[ cfg( feature = "retry" ) ]
    /// Optional retry configuration for reliability.
    pub retry_config : Option< EnhancedRetryConfig >,

    #[ cfg( feature = "circuit_breaker" ) ]
    /// Optional circuit breaker configuration for fault tolerance.
    pub circuit_breaker_config : Option< EnhancedCircuitBreakerConfig >,
    #[ cfg( feature = "circuit_breaker" ) ]
    /// Optional circuit breaker instance.
    pub circuit_breaker : Option< EnhancedCircuitBreaker >,

    #[ cfg( feature = "rate_limiting" ) ]
    /// Optional rate limiting configuration.
    pub rate_limiting_config : Option< EnhancedRateLimitingConfig >,
    #[ cfg( feature = "rate_limiting" ) ]
    /// Optional rate limiter instance.
    pub rate_limiter : Option< EnhancedRateLimiter >,
  }

} // end mod private

crate ::mod_interface!
{
  // Expose all structs defined in this module
  exposed use
  {
    Client,
  };
}
