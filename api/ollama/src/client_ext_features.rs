//! OllamaClient extension for feature configuration.
//!
//! Provides configuration methods for circuit breaker, rate limiting,
//! request caching, and diagnostics features.

#[ cfg( any( feature = "circuit_breaker", feature = "rate_limiting", feature = "request_caching", feature = "general_diagnostics" ) ) ]
mod private
{
  use core::time::Duration;
  use core::hash::Hash;
  use crate::client::OllamaClient;
  use crate::OllamaResult;
  #[ cfg( feature = "circuit_breaker" ) ]
  use crate::circuit_breaker::{ CircuitBreaker, CircuitBreakerConfig, CircuitBreakerState };
  #[ cfg( feature = "rate_limiting" ) ]
  use crate::rate_limiter::{ RateLimiter, RateLimitingConfig };
  #[ cfg( feature = "request_caching" ) ]
  use crate::request_cache::{ RequestCache, RequestCacheConfig, CacheStats };
  #[ cfg( feature = "general_diagnostics" ) ]
  use crate::diagnostics::{ DiagnosticsCollector, DiagnosticsConfig, ComprehensiveReport };
  use crate::chat::{ ChatRequest, ChatResponse };

  /// Extension to OllamaClient for feature configuration
  impl OllamaClient
  {
    /// Configure circuit breaker for this client
    #[ cfg( feature = "circuit_breaker" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn with_circuit_breaker( mut self, config : CircuitBreakerConfig ) -> Self
    {
      self.circuit_breaker = Some( CircuitBreaker::new( config ) );
      self
    }

    /// Check if this client has circuit breaker configured
    #[ cfg( feature = "circuit_breaker" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn has_circuit_breaker( &self ) -> bool
    {
      self.circuit_breaker.is_some()
    }

    /// Get the circuit breaker state for this client
    #[ cfg( feature = "circuit_breaker" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn circuit_breaker_state( &self ) -> CircuitBreakerState
    {
      match &self.circuit_breaker
      {
        Some( cb ) => cb.state(),
        None => CircuitBreakerState::Closed, // No circuit breaker means always closed
      }
    }

    /// Configure rate limiter for this client
    #[ cfg( feature = "rate_limiting" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn with_rate_limiter( mut self, config : RateLimitingConfig ) -> Self
    {
      self.rate_limiter = RateLimiter::new( config ).ok();
      self
    }

    /// Check if this client has rate limiter configured
    #[ cfg( feature = "rate_limiting" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn has_rate_limiter( &self ) -> bool
    {
      self.rate_limiter.is_some()
    }

    /// Get rate limiter configuration
    #[ cfg( feature = "rate_limiting" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn rate_limiter_config( &self ) -> Option< &RateLimitingConfig >
    {
      self.rate_limiter.as_ref().map( |rl| rl.config() )
    }

    /// Reset rate limiter state
    #[ cfg( feature = "rate_limiting" ) ]
    #[ inline ]
    pub fn reset_rate_limiter( &self )
    {
      if let Some( ref rate_limiter ) = self.rate_limiter
      {
        rate_limiter.reset();
      }
    }

    /// Configure request cache for this client
    #[ cfg( feature = "request_caching" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn with_request_cache( mut self, config : RequestCacheConfig ) -> Self
    {
      self.request_cache = Some( RequestCache::new( config ) );
      self
    }

    /// Check if this client has caching enabled
    #[ cfg( feature = "request_caching" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn has_cache( &self ) -> bool
    {
      self.request_cache.is_some()
    }

    /// Get cache statistics
    #[ cfg( feature = "request_caching" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn cache_stats( &self ) -> CacheStats
    {
      match &self.request_cache
      {
        Some( cache ) => cache.stats(),
        None => CacheStats::default(),
      }
    }

    /// Manually cache a response
    #[ cfg( feature = "request_caching" ) ]
    #[ inline ]
    pub fn cache_response< T : Hash >( &self, request : &T, response : String, ttl : Option< Duration > )
    {
      if let Some( ref cache ) = self.request_cache
      {
        let key = cache.generate_key( request );
        cache.insert( key, response, ttl );
      }
    }

    /// Chat with automatic caching
    #[ cfg( feature = "request_caching" ) ]
    #[ inline ]
    pub async fn chat_cached( &mut self, request : ChatRequest ) -> OllamaResult< String >
    {
      // Check cache first
      let cache_key = if let Some( ref cache ) = self.request_cache
      {
        let key = cache.generate_key( &request );
        if let Some( cached_response ) = cache.get( &key )
        {
          return Ok( cached_response );
        }
        Some( key )
      }
      else
      {
        None
      };

      // Not in cache, make request
      let response = self.chat( request ).await?;
      let response_json = serde_json::to_string( &response )
        .map_err( |e| error_tools::format_err!( "Failed to serialize response : {}", e ) )?;

      // Cache the response
      if let ( Some( key ), Some( ref cache ) ) = ( cache_key, &self.request_cache )
      {
        cache.insert( key, response_json.clone(), None );
      }

      Ok( response_json )
    }

    /// Configure diagnostics for this client
    #[ cfg( feature = "general_diagnostics" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn with_diagnostics( mut self, config : DiagnosticsConfig ) -> Self
    {
      self.diagnostics_collector = Some( DiagnosticsCollector::new( config ) );
      self
    }

    /// Check if diagnostics are enabled
    #[ cfg( feature = "general_diagnostics" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn has_diagnostics( &self ) -> bool
    {
      self.diagnostics_collector.is_some()
    }

    /// Get comprehensive diagnostics report
    #[ cfg( feature = "general_diagnostics" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn diagnostics_report( &self ) -> ComprehensiveReport
    {
      match &self.diagnostics_collector
      {
        Some( collector ) => collector.generate_report(),
        None => ComprehensiveReport {
          total_requests : 0,
          successful_requests : 0,
          failed_requests : 0,
          error_rate : 0.0,
          success_rate : 0.0,
          average_response_time : Duration::from_secs( 0 ),
          total_bytes_transferred : 0,
          top_errors : Vec::new(),
          performance_trends : Vec::new(),
        },
      }
    }

    /// Get all CURL commands for diagnostics
    ///
    /// Note : Currently returns empty vec. Individual curl commands can be retrieved
    /// via DiagnosticsCollector::get_curl_command(request_id) for specific requests.
    #[ cfg( feature = "general_diagnostics" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn get_curl_commands( &self ) -> Vec< String >
    {
      // TODO: Implement request ID tracking to collect all curl commands
      Vec::new()
    }

    /// Chat with diagnostics tracking
    #[ cfg( feature = "general_diagnostics" ) ]
    #[ inline ]
    pub async fn chat_with_diagnostics( &mut self, request : ChatRequest ) -> OllamaResult< ChatResponse >
    {
      let request_id = format!( "chat_{}", fastrand::u64( .. ) );

      // Track request start
      if let Some( ref diagnostics ) = &self.diagnostics_collector
      {
        diagnostics.track_request_start_with_curl( &request_id, &request, &self.base_url );
      }

      // Make the request
      let result = self.chat( request ).await;

      // Track result
      if let Some( ref diagnostics ) = &self.diagnostics_collector
      {
        match &result
        {
          Ok( response ) =>
          {
            let response_size = serde_json::to_string( response )
              .map( |s| s.len() )
              .unwrap_or( 0 );
            diagnostics.track_request_success( &request_id, response_size );
          },
          Err( e ) =>
          {
            diagnostics.track_request_failure( &request_id, 500, &format!( "{}", e ) );
          },
        }
      }

      result
    }
  }
}
