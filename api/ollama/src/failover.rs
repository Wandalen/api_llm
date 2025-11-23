//! Failover implementation for handling multiple endpoints.

#[ cfg( any( feature = "failover", feature = "health_checks" ) ) ]
mod private
{
  use core::time::Duration;
  use std::sync::atomic::AtomicUsize;
  use error_tools::untyped::{ format_err, Result as OllamaResult };

  #[ cfg( feature = "failover" ) ]
  /// Failover policy enum defining how failover should be handled
  #[ derive( Debug, Clone, PartialEq ) ]
  pub enum FailoverPolicy
  {
    /// Round-robin selection of endpoints
    RoundRobin,
    /// Priority-based selection (always prefer first healthy endpoint)
    Priority,
  }

  /// Endpoint health status
  #[ derive( Debug, Clone, PartialEq ) ]
  #[ allow( dead_code ) ]
  pub enum EndpointHealth
  {
    /// Endpoint is healthy and available
    Healthy,
    /// Endpoint is degraded but still usable
    Degraded,
    /// Endpoint is unhealthy and should not be used
    Unhealthy,
    /// Endpoint status is unknown (not yet checked)
    Unknown,
  }

  #[ cfg( feature = "failover" ) ]
  /// Endpoint information with health tracking
  #[ derive( Debug, Clone ) ]
  #[ allow( dead_code ) ]
  pub struct EndpointInfo
  {
    /// Endpoint URL
    pub url : String,
    /// Current health status
    pub health : EndpointHealth,
    /// Last successful request timestamp
    pub last_success : Option< std::time::Instant >,
    /// Last failure timestamp
    pub last_failure : Option< std::time::Instant >,
    /// Total requests made to this endpoint
    pub total_requests : u64,
    /// Total failures for this endpoint
    pub total_failures : u64,
    /// Average response time in milliseconds
    pub avg_response_time : Duration,
  }

  #[ cfg( feature = "failover" ) ]
  #[ allow( dead_code ) ]
  impl EndpointInfo
  {
    /// Create a new endpoint info with healthy status
    #[ inline ]
    #[ must_use ]
    pub fn new( url : String ) -> Self
    {
      Self
      {
        url,
        health : EndpointHealth::Healthy,
        last_success : None,
        last_failure : None,
        total_requests : 0,
        total_failures : 0,
        avg_response_time : Duration::from_millis( 0 ),
      }
    }

    /// Mark endpoint as healthy
    #[ inline ]
    pub fn mark_healthy( &mut self )
    {
      self.health = EndpointHealth::Healthy;
      self.last_success = Some( std::time::Instant::now() );
    }

    /// Mark endpoint as unhealthy
    #[ inline ]
    pub fn mark_unhealthy( &mut self )
    {
      self.health = EndpointHealth::Unhealthy;
      self.last_failure = Some( std::time::Instant::now() );
      self.total_failures += 1;
    }

    /// Update request statistics
    #[ inline ]
    pub fn update_request_stats( &mut self, response_time : Duration )
    {
      self.total_requests += 1;

      // Update average response time
      if self.total_requests == 1
      {
        self.avg_response_time = response_time;
      }
      else
      {
        let total_ms = self.avg_response_time.as_millis() as u64 * ( self.total_requests - 1 );
        let new_avg_ms = ( total_ms + response_time.as_millis() as u64 ) / self.total_requests;
        self.avg_response_time = Duration::from_millis( new_avg_ms );
      }
    }

    /// Check if endpoint is healthy
    #[ inline ]
    #[ must_use ]
    pub fn is_healthy( &self ) -> bool
    {
      self.health == EndpointHealth::Healthy
    }
  }

  #[ cfg( feature = "failover" ) ]
  /// Failover statistics and monitoring data
  #[ derive( Debug, Clone ) ]
  pub struct FailoverStats
  {
    /// Total number of failovers performed
    pub total_failovers : u64,
    /// Total requests processed
    pub total_requests : u64,
    /// Current active endpoint index
    pub active_endpoint_index : usize,
    /// Total endpoints configured
    pub total_endpoints : usize,
  }

  #[ cfg( feature = "failover" ) ]
  impl FailoverStats
  {
    /// Create new failover stats
    #[ inline ]
    #[ must_use ]
    pub fn new( total_endpoints : usize ) -> Self
    {
      Self
      {
        total_failovers : 0,
        total_requests : 0,
        active_endpoint_index : 0,
        total_endpoints,
      }
    }
  }

  #[ cfg( feature = "failover" ) ]
  /// Failover manager for handling multiple endpoints
  #[ derive( Debug ) ]
  #[ allow( dead_code ) ]
  pub struct FailoverManager
  {
    /// List of configured endpoints with health information
    endpoints : Vec< EndpointInfo >,
    /// Current active endpoint index
    current_index : AtomicUsize,
    /// Failover policy
    policy : FailoverPolicy,
    /// Statistics
    stats : std::sync::Mutex< FailoverStats >,
    /// Request timeout
    timeout : Duration,
  }

  #[ cfg( feature = "failover" ) ]
  impl FailoverManager
  {
    /// Create a new failover manager with the given endpoints and policy
    #[ inline ]
    #[ must_use ]
    pub fn new( endpoints : Vec< String >, policy : FailoverPolicy, timeout : Duration ) -> OllamaResult< Self >
    {
      if endpoints.is_empty()
      {
        return Err( format_err!( "At least one endpoint must be provided" ) );
      }

      // Validate all endpoints are valid URLs
      for endpoint in &endpoints
      {
        url ::Url::parse( endpoint ).map_err( |e| format_err!( "Invalid URL {}: {}", endpoint, e ) )?;
      }

      let endpoint_infos : Vec< EndpointInfo > = endpoints.into_iter()
        .map( EndpointInfo::new )
        .collect();

      let stats = FailoverStats::new( endpoint_infos.len() );

      Ok( Self
      {
        endpoints : endpoint_infos,
        current_index : AtomicUsize::new( 0 ),
        policy,
        stats : std::sync::Mutex::new( stats ),
        timeout,
      })
    }

    /// Get the currently active endpoint URL
    #[ inline ]
    #[ must_use ]
    pub fn get_active_endpoint( &self ) -> String
    {
      let index = self.current_index.load( std::sync::atomic::Ordering::Acquire );
      if index < self.endpoints.len()
      {
        self.endpoints[ index ].url.clone()
      }
      else
      {
        // Fallback to first endpoint if index is out of bounds
        self.endpoints[ 0 ].url.clone()
      }
    }

    /// Get the number of configured endpoints
    #[ inline ]
    #[ must_use ]
    pub fn get_endpoint_count( &self ) -> usize
    {
      self.endpoints.len()
    }

    /// Check if a specific endpoint is healthy
    #[ inline ]
    #[ must_use ]
    pub fn is_endpoint_healthy( &self, url : &str ) -> bool
    {
      self.endpoints.iter()
        .find( |endpoint| endpoint.url == url )
        .map_or( false, |endpoint| endpoint.is_healthy() )
    }

    /// Mark an endpoint as healthy (requires mutable access)
    #[ inline ]
    pub fn mark_endpoint_healthy( &mut self, url : &str )
    {
      if let Some( endpoint ) = self.endpoints.iter_mut().find( |e| e.url == url )
      {
        endpoint.mark_healthy();
      }
      // Recalculate active endpoint for priority policy
      if self.policy == FailoverPolicy::Priority
      {
        self.select_next_healthy_endpoint();
      }
    }

    /// Mark an endpoint as unhealthy (requires mutable access)
    #[ inline ]
    pub fn mark_endpoint_unhealthy( &mut self, url : &str )
    {
      if let Some( endpoint ) = self.endpoints.iter_mut().find( |e| e.url == url )
      {
        endpoint.mark_unhealthy();
      }
      self.select_next_healthy_endpoint();
    }

    /// Rotate to the next endpoint (for round-robin policy)
    #[ inline ]
    pub fn rotate_endpoint( &mut self )
    {
      let current = self.current_index.load( std::sync::atomic::Ordering::Acquire );
      let next = ( current + 1 ) % self.endpoints.len();
      self.current_index.store( next, std::sync::atomic::Ordering::Release );
    }

    /// Select the next healthy endpoint based on the failover policy
    pub fn select_next_healthy_endpoint( &mut self )
    {
      match self.policy
      {
        FailoverPolicy::RoundRobin =>
        {
          // Find next healthy endpoint in round-robin fashion
          let current = self.current_index.load( std::sync::atomic::Ordering::Acquire );
          for i in 1..=self.endpoints.len()
          {
            let index = ( current + i ) % self.endpoints.len();
            if self.endpoints[ index ].is_healthy()
            {
              self.current_index.store( index, std::sync::atomic::Ordering::Release );
              break;
            }
          }
        }
        FailoverPolicy::Priority =>
        {
          // Find first healthy endpoint (highest priority)
          for ( index, endpoint ) in self.endpoints.iter().enumerate()
          {
            if endpoint.is_healthy()
            {
              self.current_index.store( index, std::sync::atomic::Ordering::Release );
              break;
            }
          }
        }
      }

      // Update failover stats
      if let Ok( mut stats ) = self.stats.lock()
      {
        stats.total_failovers += 1;
        stats.active_endpoint_index = self.current_index.load( std::sync::atomic::Ordering::Acquire );
      }
    }

    /// Get failover statistics
    #[ inline ]
    #[ must_use ]
    pub fn get_failover_stats( &self ) -> FailoverStats
    {
      self.stats.lock().map( |stats| stats.clone() ).unwrap_or_else( |_|
      {
        FailoverStats::new( self.endpoints.len() )
      })
    }
  }
}

#[ cfg( any( feature = "failover", feature = "health_checks" ) ) ]
crate ::mod_interface!
{
  exposed use private::EndpointHealth;

  #[ cfg( feature = "failover" ) ]
  exposed use private::FailoverPolicy;
  #[ cfg( feature = "failover" ) ]
  exposed use private::FailoverStats;
  #[ cfg( feature = "failover" ) ]
  exposed use private::EndpointInfo;
  #[ cfg( feature = "failover" ) ]
  exposed use private::FailoverManager;
}
