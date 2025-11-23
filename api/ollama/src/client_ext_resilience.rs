//! OllamaClient extension for failover and health check functionality.
//!
//! Provides resilience capabilities including automatic failover to backup endpoints
//! and health monitoring for endpoint availability.

#[ cfg( any( feature = "failover", feature = "health_checks" ) ) ]
mod private
{
  use core::time::Duration;
  use crate::client::OllamaClient;
  #[ cfg( feature = "failover" ) ]
  use crate::failover::{ FailoverManager, FailoverPolicy, FailoverStats };
  #[ cfg( feature = "health_checks" ) ]
  use crate::health_checks::{ HealthCheckManager, HealthCheckConfig, HealthStatus, HealthMetrics };
  use crate::OllamaResult;

  /// Extension to OllamaClient for failover and health check capabilities
  impl OllamaClient
  {
    #[ cfg( feature = "failover" ) ]
    /// Create a new Ollama client with failover support
    #[ inline ]
    #[ must_use ]
    pub fn new_with_failover( endpoints : Vec< String >, timeout : Duration ) -> OllamaResult< Self >
    {
      let failover_manager = FailoverManager::new( endpoints, FailoverPolicy::Priority, timeout )?;
      let base_url = failover_manager.get_active_endpoint();

      let mut client = Self::new( base_url, timeout );
      client.failover_manager = Some( std::sync::Arc::new( std::sync::Mutex::new( failover_manager ) ) );
      Ok( client )
    }

    #[ cfg( feature = "failover" ) ]
    /// Create a new Ollama client with failover support and custom policy
    #[ inline ]
    #[ must_use ]
    pub fn new_with_failover_policy( endpoints : Vec< String >, timeout : Duration, policy : FailoverPolicy ) -> OllamaResult< Self >
    {
      let failover_manager = FailoverManager::new( endpoints, policy, timeout )?;
      let base_url = failover_manager.get_active_endpoint();

      let mut client = Self::new( base_url, timeout );
      client.failover_manager = Some( std::sync::Arc::new( std::sync::Mutex::new( failover_manager ) ) );
      Ok( client )
    }

    #[ cfg( feature = "failover" ) ]
    /// Get the currently active endpoint URL
    #[ inline ]
    #[ must_use ]
    pub fn get_active_endpoint( &self ) -> String
    {
      if let Some( ref manager ) = self.failover_manager
      {
        if let Ok( manager ) = manager.lock()
        {
          return manager.get_active_endpoint();
        }
      }
      self.base_url.clone()
    }

    #[ cfg( feature = "failover" ) ]
    /// Get the number of configured endpoints
    #[ inline ]
    #[ must_use ]
    pub fn get_endpoint_count( &self ) -> usize
    {
      if let Some( ref manager ) = self.failover_manager
      {
        if let Ok( manager ) = manager.lock()
        {
          return manager.get_endpoint_count();
        }
      }
      1 // Single endpoint
    }

    /// Check if a specific endpoint is healthy
    #[ cfg( feature = "failover" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn is_endpoint_healthy( &self, url : &str ) -> bool
    {
      if let Some( ref manager ) = self.failover_manager
      {
        if let Ok( manager ) = manager.lock()
        {
          return manager.is_endpoint_healthy( url );
        }
      }
      url == self.base_url // Single endpoint is always considered healthy
    }

    /// Mark an endpoint as healthy
    #[ cfg( feature = "failover" ) ]
    #[ inline ]
    pub fn mark_endpoint_healthy( &mut self, url : &str )
    {
      if let Some( ref manager ) = self.failover_manager
      {
        if let Ok( mut manager ) = manager.lock()
        {
          manager.mark_endpoint_healthy( url );
        }
      }
    }

    /// Mark an endpoint as unhealthy
    #[ cfg( feature = "failover" ) ]
    #[ inline ]
    pub fn mark_endpoint_unhealthy( &mut self, url : &str )
    {
      if let Some( ref manager ) = self.failover_manager
      {
        if let Ok( mut manager ) = manager.lock()
        {
          manager.mark_endpoint_unhealthy( url );
        }
      }
    }

    /// Rotate to the next available endpoint
    #[ cfg( feature = "failover" ) ]
    #[ inline ]
    pub fn rotate_endpoint( &mut self )
    {
      if let Some( ref manager ) = self.failover_manager
      {
        if let Ok( mut manager ) = manager.lock()
        {
          manager.rotate_endpoint();
          self.base_url = manager.get_active_endpoint();
        }
      }
    }

    /// Get failover statistics
    #[ cfg( feature = "failover" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn get_failover_stats( &self ) -> FailoverStats
    {
      if let Some( ref manager ) = self.failover_manager
      {
        if let Ok( manager ) = manager.lock()
        {
          return manager.get_failover_stats();
        }
      }
      FailoverStats
      {
        total_failovers : 0,
        total_requests : 0,
        active_endpoint_index : 0,
        total_endpoints : 0,
      }
    }

    /// Create new Ollama client with health checks enabled
    #[ cfg( feature = "health_checks" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn new_with_health_checks( base_url : String, timeout : Duration, config : HealthCheckConfig ) -> OllamaResult< Self >
    {
      let mut client = Self::new( base_url, timeout );
      let health_manager = HealthCheckManager::new( client.base_url.clone(), config )?;
      client.health_check_manager = Some( std::sync::Arc::new( std::sync::Mutex::new( health_manager ) ) );
      Ok( client )
    }

    /// Check if health checks are enabled
    #[ cfg( feature = "health_checks" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn health_checks_enabled( &self ) -> bool
    {
      self.health_check_manager.is_some()
    }

    /// Start background health monitoring
    #[ cfg( feature = "health_checks" ) ]
    #[ inline ]
    pub async fn start_health_monitoring( &mut self )
    {
      if let Some( ref manager ) = self.health_check_manager
      {
        let mut guard = manager.lock().expect( "Failed to lock health check manager" );
        guard.start_monitoring().await;
      }
    }

    /// Stop background health monitoring
    #[ cfg( feature = "health_checks" ) ]
    #[ inline ]
    pub async fn stop_health_monitoring( &mut self )
    {
      if let Some( ref manager ) = self.health_check_manager
      {
        let mut guard = manager.lock().expect( "Failed to lock health check manager" );
        guard.stop_monitoring().await;
      }
    }

    /// Get current health status
    #[ cfg( feature = "health_checks" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn get_health_status( &self ) -> HealthStatus
    {
      if let Some( ref manager ) = self.health_check_manager
      {
        if let Ok( guard ) = manager.lock()
        {
          return guard.get_health_status();
        }
      }
      HealthStatus::default()
    }

    /// Get health metrics
    #[ cfg( feature = "health_checks" ) ]
    #[ inline ]
    #[ must_use ]
    pub fn get_health_metrics( &self ) -> HealthMetrics
    {
      if let Some( ref manager ) = self.health_check_manager
      {
        if let Ok( guard ) = manager.lock()
        {
          return guard.get_health_metrics();
        }
      }
      HealthMetrics::default()
    }

    /// Simulate endpoint failure for testing
    #[ cfg( feature = "health_checks" ) ]
    #[ inline ]
    pub fn simulate_endpoint_failure( &self )
    {
      if let Some( ref manager ) = self.health_check_manager
      {
        let guard = manager.lock().expect( "Failed to lock health check manager" );
        guard.simulate_endpoint_failure();
      }
    }

    /// Restore endpoint for testing
    #[ cfg( feature = "health_checks" ) ]
    #[ inline ]
    pub fn restore_endpoint( &self )
    {
      if let Some( ref manager ) = self.health_check_manager
      {
        let guard = manager.lock().expect( "Failed to lock health check manager" );
        guard.restore_endpoint();
      }
    }
  }
}
