mod private
{
  //! Health check probes for production deployments.
  //!
  //! This module provides Kubernetes-style health checks (liveness, readiness)
  //! for monitoring API endpoint availability and performance.
  //!
  //! # Design Decisions
  //!
  //! ## Why `list_models()` as the Health Probe?
  //!
  //! The health check implementations use `client.models().list()` instead of
  //! alternatives for several reasons:
  //!
  //! 1. **Lightweight**: The models endpoint returns a small JSON array and
  //!    doesn't consume quota or incur costs like chat completions would.
  //!
  //! 2. **Authentication Validation**: The models endpoint requires valid API
  //!    authentication, so a successful response confirms the entire auth chain
  //!    is working (API key, headers, TLS).
  //!
  //! 3. **No Side Effects**: Unlike chat completions or other mutation endpoints,
  //!    listing models has no side effects and can be called repeatedly without
  //!    accumulating state.
  //!
  //! 4. **Actual API Exercise**: Some alternatives like TCP connectivity checks
  //!    don't validate that the API service is actually functional - just that
  //!    the network path exists.
  //!
  //! ## Alternatives Considered
  //!
  //! - **HEAD / or GET /**: Would test connectivity but not authentication or API functionality
  //! - **Dedicated health endpoint**: XAI API doesn't provide one
  //! - **Chat completion**: Too expensive and slow for frequent health checks
  //! - **Custom health route**: Would violate "Thin Client" principle by adding non-API functionality
  //!
  //! ## Health Status : 3-State Model
  //!
  //! Health checks return Healthy/Degraded/Unhealthy instead of binary pass/fail:
  //!
  //! ```text
  //! Healthy :   Response < 2000ms, successful
  //! Degraded :  Response ≥ 2000ms, successful (slow but working)
  //! Unhealthy : Request failed or error response
  //! ```
  //!
  //! **Rationale**: The Degraded state allows monitoring systems to detect
  //! performance issues before they become failures. This enables gradual
  //! degradation handling (e.g., reducing request rate) instead of binary
  //! failover decisions.
  //!
  //! ## 2000ms Threshold for Degraded
  //!
  //! The 2-second threshold for marking an endpoint as Degraded was chosen because:
  //!
  //! 1. **User Experience**: 2 seconds is a common UX threshold for "too slow"
  //! 2. **API SLA**: Typical LLM API SLAs target p95 < 1s for lightweight requests
  //! 3. **Early Warning**: Gives time to investigate before failures occur
  //! 4. **False Positive Balance**: High enough to avoid flagging normal variance
  //!
  //! This threshold is hardcoded (not configurable) to keep the API simple,
  //! following the "Thin Client" principle. Applications that need different
  //! thresholds can implement their own health checks using the same pattern.
  //!
  //! ## Liveness vs Readiness
  //!
  //! Following Kubernetes conventions:
  //!
  //! - **Liveness**: "Is the client still functional?" (always true for stateless HTTP client)
  //! - **Readiness**: "Can the client handle requests?" (checks API endpoint health)
  //!
  //! This distinction allows orchestrators to distinguish between "restart the pod"
  //! (liveness failure) and "stop sending traffic" (readiness failure) scenarios.

  use crate::Client;
  use crate::client_api_accessors::ClientApiAccessors;
  use crate::environment::XaiEnvironment;

  /// Health status of the API endpoint.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
  pub enum HealthStatus
  {
    /// Endpoint is healthy and responding.
    Healthy,

    /// Endpoint is degraded (slow response or partial failure).
    Degraded,

    /// Endpoint is unhealthy or unreachable.
    Unhealthy,
  }

  /// Result of a health check.
  #[ derive( Debug, Clone ) ]
  pub struct HealthCheckResult
  {
    /// Overall health status.
    pub status : HealthStatus,

    /// Response time in milliseconds.
    pub response_time_ms : u64,

    /// Optional error message if unhealthy.
    pub message : Option< String >,
  }

  /// Performs a health check on the XAI API endpoint.
  ///
  /// Makes a lightweight API call (listing models) to verify the endpoint
  /// is reachable and responding correctly.
  ///
  /// # Arguments
  ///
  /// * `client` - The API client to check
  ///
  /// # Returns
  ///
  /// Returns a `HealthCheckResult` with status, response time, and optional error message.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use api_xai::{ Client, XaiEnvironmentImpl, Secret, health_check };
  ///
  /// # async fn example() -> Result< (), Box< dyn std::error::Error > > {
  /// let secret = Secret::load_with_fallbacks( "XAI_API_KEY" )?;
  /// let env = XaiEnvironmentImpl::new( secret )?;
  /// let client = Client::build( env )?;
  ///
  /// let result = health_check( &client ).await;
  /// println!( "Health : {:?}", result.status );
  /// println!( "Response time : {}ms", result.response_time_ms );
  /// # Ok( () )
  /// # }
  /// ```
  #[ allow( clippy::cast_possible_truncation) ]  // Bounded by min(u64::MAX), response times won't overflow
  pub async fn health_check< E >( client : &Client< E > ) -> HealthCheckResult
  where
    E : XaiEnvironment + Send + Sync + 'static,
  {
    let start = std::time::Instant::now();

    // Make a lightweight API call to check health
    let result = client.models().list().await;

    let elapsed = start.elapsed();
    let response_time_ms = elapsed.as_millis().min( u128::from(u64::MAX) ) as u64;

    match result
    {
      Ok( _ ) =>
      {
        // Check if response was slow (> 2 seconds = degraded)
        if response_time_ms > 2000
        {
          HealthCheckResult
          {
            status : HealthStatus::Degraded,
            response_time_ms,
            message : Some( format!( "Slow response : {response_time_ms}ms" ) ),
          }
        }
        else
        {
          HealthCheckResult
          {
            status : HealthStatus::Healthy,
            response_time_ms,
            message : None,
          }
        }
      }
      Err( e ) =>
      {
        HealthCheckResult
        {
          status : HealthStatus::Unhealthy,
          response_time_ms,
          message : Some( format!( "Health check failed : {e}" ) ),
        }
      }
    }
  }

  /// Performs a quick liveness check.
  ///
  /// Verifies the client is configured and can make basic requests.
  /// This is a lightweight check suitable for Kubernetes liveness probes.
  ///
  /// # Arguments
  ///
  /// * `client` - The API client to check
  ///
  /// # Returns
  ///
  /// Returns `true` if the endpoint responds successfully, `false` otherwise.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use api_xai::{ Client, XaiEnvironmentImpl, Secret, liveness_check };
  ///
  /// # async fn example() -> Result< (), Box< dyn std::error::Error > > {
  /// let secret = Secret::load_with_fallbacks( "XAI_API_KEY" )?;
  /// let env = XaiEnvironmentImpl::new( secret )?;
  /// let client = Client::build( env )?;
  ///
  /// if liveness_check( &client ).await {
  ///   println!( "Service is alive" );
  /// }
  /// # Ok( () )
  /// # }
  /// ```
  pub async fn liveness_check< E >( client : &Client< E > ) -> bool
  where
    E : XaiEnvironment + Send + Sync + 'static,
  {
    let result = health_check( client ).await;
    !matches!( result.status, HealthStatus::Unhealthy )
  }

  /// Performs a readiness check.
  ///
  /// Verifies the endpoint is ready to handle requests with acceptable performance.
  /// This is suitable for Kubernetes readiness probes.
  ///
  /// # Arguments
  ///
  /// * `client` - The API client to check
  ///
  /// # Returns
  ///
  /// Returns `true` if the endpoint is healthy (not degraded or unhealthy), `false` otherwise.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use api_xai::{ Client, XaiEnvironmentImpl, Secret, readiness_check };
  ///
  /// # async fn example() -> Result< (), Box< dyn std::error::Error > > {
  /// let secret = Secret::load_with_fallbacks( "XAI_API_KEY" )?;
  /// let env = XaiEnvironmentImpl::new( secret )?;
  /// let client = Client::build( env )?;
  ///
  /// if readiness_check( &client ).await {
  ///   println!( "Service is ready to handle requests" );
  /// }
  /// # Ok( () )
  /// # }
  /// ```
  pub async fn readiness_check< E >( client : &Client< E > ) -> bool
  where
    E : XaiEnvironment + Send + Sync + 'static,
  {
    let result = health_check( client ).await;
    matches!( result.status, HealthStatus::Healthy )
  }
}

crate::mod_interface!
{
  exposed use
  {
    HealthStatus,
    HealthCheckResult,
    health_check,
    liveness_check,
    readiness_check,
  };
}
