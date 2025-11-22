//! Health check types and implementation for monitoring endpoint availability.
//!
//! Provides health monitoring, status tracking, and background health checking
//! capabilities for Ollama endpoints.

#[ cfg( feature = "health_checks" ) ]
mod private
{
  use core::time::Duration;
  use std::sync::{ Arc, Mutex };
  use super::super::*;
  use error_tools::format_err;

  /// Health check strategy options
  #[ derive( Debug, Clone, PartialEq ) ]
  pub enum HealthCheckStrategy
  {
    /// Simple ping check
    Ping,
    /// Lightweight API call check
    ApiCall,
    /// Version endpoint check
    VersionCheck,
  }

  /// Configuration for health check behavior
  #[ derive( Debug, Clone ) ]
  pub struct HealthCheckConfig
  {
    /// Interval between health checks
    interval : Duration,
    /// Timeout for each health check
    timeout : Duration,
    /// Health check strategy to use
    strategy : HealthCheckStrategy,
    /// Number of consecutive failures before marking as unhealthy
    failure_threshold : u32,
    /// Number of consecutive successes required for recovery
    recovery_threshold : u32,
    /// Whether to integrate with circuit breaker
    circuit_breaker_integration : bool,
  }

  impl HealthCheckConfig
  {
    /// Create new health check configuration with defaults
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        interval : Duration::from_secs( 30 ),
        timeout : Duration::from_secs( 5 ),
        strategy : HealthCheckStrategy::Ping,
        failure_threshold : 3,
        recovery_threshold : 2,
        circuit_breaker_integration : false,
      }
    }

    /// Set health check interval
    #[ inline ]
    #[ must_use ]
    pub fn with_interval( mut self, interval : Duration ) -> Self
    {
      self.interval = interval;
      self
    }

    /// Set health check timeout
    #[ inline ]
    #[ must_use ]
    pub fn with_timeout( mut self, timeout : Duration ) -> Self
    {
      self.timeout = timeout;
      self
    }

    /// Set health check strategy
    #[ inline ]
    #[ must_use ]
    pub fn with_strategy( mut self, strategy : HealthCheckStrategy ) -> Self
    {
      self.strategy = strategy;
      self
    }

    /// Set failure threshold
    #[ inline ]
    #[ must_use ]
    pub fn with_failure_threshold( mut self, threshold : u32 ) -> Self
    {
      self.failure_threshold = threshold;
      self
    }

    /// Set recovery threshold
    #[ inline ]
    #[ must_use ]
    pub fn with_recovery_threshold( mut self, threshold : u32 ) -> Self
    {
      self.recovery_threshold = threshold;
      self
    }

    /// Enable circuit breaker integration
    #[ inline ]
    #[ must_use ]
    pub fn with_circuit_breaker_integration( mut self, enabled : bool ) -> Self
    {
      self.circuit_breaker_integration = enabled;
      self
    }

    /// Get interval
    #[ inline ]
    #[ must_use ]
    pub fn interval( &self ) -> Duration
    {
      self.interval
    }

    /// Get timeout
    #[ inline ]
    #[ must_use ]
    pub fn timeout( &self ) -> Duration
    {
      self.timeout
    }

    /// Get strategy
    #[ inline ]
    #[ must_use ]
    pub fn strategy( &self ) -> &HealthCheckStrategy
    {
      &self.strategy
    }

    /// Get failure threshold
    #[ inline ]
    #[ must_use ]
    pub fn failure_threshold( &self ) -> u32
    {
      self.failure_threshold
    }

    /// Get recovery threshold
    #[ inline ]
    #[ must_use ]
    pub fn recovery_threshold( &self ) -> u32
    {
      self.recovery_threshold
    }

    /// Check if circuit breaker integration is enabled
    #[ inline ]
    #[ must_use ]
    pub fn circuit_breaker_integration( &self ) -> bool
    {
      self.circuit_breaker_integration
    }

    /// Validate configuration
    #[ inline ]
    pub fn validate( &self ) -> OllamaResult< () >
    {
      if self.interval < Duration::from_millis( 100 )
      {
        return Err( format_err!( "Health check interval must be at least 100ms" ) );
      }

      if self.timeout >= self.interval
      {
        return Err( format_err!( "Health check timeout must be less than interval" ) );
      }

      if self.failure_threshold == 0
      {
        return Err( format_err!( "Failure threshold must be greater than 0" ) );
      }

      if self.recovery_threshold == 0
      {
        return Err( format_err!( "Recovery threshold must be greater than 0" ) );
      }

      Ok( () )
    }
  }

  impl Default for HealthCheckConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Health status information for an endpoint
  #[ derive( Debug, Clone ) ]
  pub struct HealthStatus
  {
    /// Overall health of the endpoint
    overall_health : EndpointHealth,
    /// Total health checks performed
    total_checks : u64,
    /// Number of successful checks
    successful_checks : u64,
    /// Number of failed checks
    failed_checks : u64,
    /// Response times for recent checks
    response_times : Vec< Duration >,
    /// Last check timestamp
    last_check_time : Option< std::time::Instant >,
    /// Whether circuit breaker is open
    circuit_breaker_open : bool,
    /// Consecutive failure count
    consecutive_failures : u32,
    /// Consecutive success count
    consecutive_successes : u32,
  }

  impl HealthStatus
  {
    /// Create new health status
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        overall_health : EndpointHealth::Unknown,
        total_checks : 0,
        successful_checks : 0,
        failed_checks : 0,
        response_times : Vec::new(),
        last_check_time : None,
        circuit_breaker_open : false,
        consecutive_failures : 0,
        consecutive_successes : 0,
      }
    }

    /// Get overall health
    #[ inline ]
    #[ must_use ]
    pub fn overall_health( &self ) -> EndpointHealth
    {
      self.overall_health.clone()
    }

    /// Get total checks performed
    #[ inline ]
    #[ must_use ]
    pub fn total_checks( &self ) -> u64
    {
      self.total_checks
    }

    /// Get successful checks count
    #[ inline ]
    #[ must_use ]
    pub fn successful_checks( &self ) -> u64
    {
      self.successful_checks
    }

    /// Get failed checks count
    #[ inline ]
    #[ must_use ]
    pub fn failed_checks( &self ) -> u64
    {
      self.failed_checks
    }

    /// Get response times
    #[ inline ]
    #[ must_use ]
    pub fn get_response_times( &self ) -> &Vec< Duration >
    {
      &self.response_times
    }

    /// Check if circuit breaker is open
    #[ inline ]
    #[ must_use ]
    pub fn circuit_breaker_open( &self ) -> bool
    {
      self.circuit_breaker_open
    }

    /// Record successful health check
    #[ inline ]
    pub fn record_success( &mut self, response_time : Duration )
    {
      self.total_checks += 1;
      self.successful_checks += 1;
      self.consecutive_failures = 0;
      self.consecutive_successes += 1;
      self.response_times.push( response_time );
      self.last_check_time = Some( std::time::Instant::now() );

      // Keep only last 10 response times
      if self.response_times.len() > 10
      {
        self.response_times.remove( 0 );
      }

      // Update health status based on recent performance
      if self.consecutive_successes >= 2
      {
        self.overall_health = EndpointHealth::Healthy;
        self.circuit_breaker_open = false;
      }
    }

    /// Record failed health check
    #[ inline ]
    pub fn record_failure( &mut self, failure_threshold : u32 )
    {
      self.total_checks += 1;
      self.failed_checks += 1;
      self.consecutive_successes = 0;
      self.consecutive_failures += 1;
      self.last_check_time = Some( std::time::Instant::now() );

      // Update health status based on consecutive failures
      if self.consecutive_failures >= failure_threshold
      {
        self.overall_health = EndpointHealth::Unhealthy;
      }
      else if self.consecutive_failures > 1
      {
        self.overall_health = EndpointHealth::Degraded;
      }
    }

    /// Mark circuit breaker as open
    #[ inline ]
    pub fn set_circuit_breaker_open( &mut self, open : bool )
    {
      self.circuit_breaker_open = open;
    }
  }

  impl Default for HealthStatus
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Health metrics for monitoring and reporting
  #[ derive( Debug, Clone ) ]
  pub struct HealthMetrics
  {
    /// Total health checks performed
    pub total_checks : u64,
    /// Average response time
    pub average_response_time : Option< Duration >,
    /// Uptime percentage
    pub uptime_percentage : f64,
    /// Last successful check time
    pub last_successful_check : Option< std::time::Instant >,
    /// Health check start time
    pub monitoring_start_time : std::time::Instant,
  }

  impl HealthMetrics
  {
    /// Create new health metrics
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        total_checks : 0,
        average_response_time : None,
        uptime_percentage : 0.0,
        last_successful_check : None,
        monitoring_start_time : std::time::Instant::now(),
      }
    }
  }

  impl Default for HealthMetrics
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Health check manager for background monitoring
  #[ derive( Debug ) ]
  pub struct HealthCheckManager
  {
    /// Health check configuration
    config : HealthCheckConfig,
    /// Current health status
    status : Arc< Mutex< HealthStatus > >,
    /// Health metrics
    metrics : Arc< Mutex< HealthMetrics > >,
    /// Background task handle
    task_handle : Option< tokio::task::JoinHandle< () > >,
    /// Shutdown signal sender
    shutdown_tx : Option< tokio::sync::oneshot::Sender< () > >,
    /// Endpoint URL for health checks
    endpoint_url : String,
    /// HTTP client for health checks
    client : reqwest::Client,
    /// Simulated failure flag for testing
    simulate_failure : Arc< core::sync::atomic::AtomicBool >,
  }

  impl HealthCheckManager
  {
    /// Create new health check manager
    #[ inline ]
    #[ must_use ]
    pub fn new( endpoint_url : String, config : HealthCheckConfig ) -> OllamaResult< Self >
    {
      config.validate()?;

      let client = reqwest::Client::builder()
        .timeout( config.timeout )
        .build()
        .map_err( | e | format_err!( "Failed to create HTTP client : {}", e ) )?;

      Ok( Self
      {
        config,
        status : Arc::new( Mutex::new( HealthStatus::new() ) ),
        metrics : Arc::new( Mutex::new( HealthMetrics::new() ) ),
        task_handle : None,
        shutdown_tx : None,
        endpoint_url,
        client,
        simulate_failure : Arc::new( std::sync::atomic::AtomicBool::new( false ) ),
      })
    }

    /// Start background health monitoring
    #[ inline ]
    pub async fn start_monitoring( &mut self )
    {
      if self.task_handle.is_some()
      {
        return; // Already running
      }

      let ( shutdown_tx, mut shutdown_rx ) = tokio::sync::oneshot::channel();
      self.shutdown_tx = Some( shutdown_tx );

      let status = self.status.clone();
      let metrics = self.metrics.clone();
      let config = self.config.clone();
      let endpoint_url = self.endpoint_url.clone();
      let client = self.client.clone();
      let simulate_failure = self.simulate_failure.clone();

      let handle = tokio::spawn( async move {
        let mut interval = tokio::time::interval( config.interval );

        loop
        {
          tokio ::select! {
            _ = interval.tick() => {
              let start_time = std::time::Instant::now();
              let success = if simulate_failure.load( std::sync::atomic::Ordering::Relaxed )
              {
                false
              }
              else
              {
                Self::perform_health_check( &client, &endpoint_url, &config ).await
              };

              let response_time = start_time.elapsed();

              // Update status and metrics
              if let Ok( mut status ) = status.lock()
              {
                if success
                {
                  status.record_success( response_time );
                }
                else
                {
                  status.record_failure( config.failure_threshold );

                  // Trigger circuit breaker if integration is enabled and health is now unhealthy
                  if config.circuit_breaker_integration() && status.overall_health() == EndpointHealth::Unhealthy
                  {
                    status.set_circuit_breaker_open( true );
                  }
                }
              }

              if let Ok( mut metrics ) = metrics.lock()
              {
                metrics.total_checks += 1;
                if success
                {
                  metrics.last_successful_check = Some( start_time );
                }

                // Calculate uptime percentage
                let _total_duration = start_time.duration_since( metrics.monitoring_start_time );
                if let Ok( status ) = status.lock()
                {
                  if status.total_checks() > 0
                  {
                    metrics.uptime_percentage = ( status.successful_checks() as f64 / status.total_checks() as f64 ) * 100.0;
                  }

                  // Calculate average response time
                  let response_times = status.get_response_times();
                  if !response_times.is_empty()
                  {
                    let total : Duration = response_times.iter().sum();
                    metrics.average_response_time = Some( total / response_times.len() as u32 );
                  }
                }
              }
            }
            _ = &mut shutdown_rx => {
              break;
            }
          }
        }
      });

      self.task_handle = Some( handle );
    }

    /// Stop background health monitoring
    #[ inline ]
    pub async fn stop_monitoring( &mut self )
    {
      if let Some( shutdown_tx ) = self.shutdown_tx.take()
      {
        let _ = shutdown_tx.send( () );
      }

      if let Some( handle ) = self.task_handle.take()
      {
        let _ = handle.await;
      }
    }

    /// Get current health status
    #[ inline ]
    #[ must_use ]
    pub fn get_health_status( &self ) -> HealthStatus
    {
      self.status.lock().map( |status| status.clone() ).unwrap_or_default()
    }

    /// Get health metrics
    #[ inline ]
    #[ must_use ]
    pub fn get_health_metrics( &self ) -> HealthMetrics
    {
      self.metrics.lock().map( |metrics| metrics.clone() ).unwrap_or_default()
    }

    /// Simulate endpoint failure for testing
    #[ inline ]
    pub fn simulate_endpoint_failure( &self )
    {
      self.simulate_failure.store( true, std::sync::atomic::Ordering::Relaxed );
    }

    /// Restore endpoint for testing
    #[ inline ]
    pub fn restore_endpoint( &self )
    {
      self.simulate_failure.store( false, std::sync::atomic::Ordering::Relaxed );
    }

    /// Perform a single health check
    async fn perform_health_check( client : &reqwest::Client, endpoint_url : &str, config : &HealthCheckConfig ) -> bool
    {
      match config.strategy
      {
        HealthCheckStrategy::Ping =>
        {
          // Simple ping - just try to connect
          let url = format!( "{endpoint_url}/api/tags" );
          client.get( &url ).send().await.is_ok()
        },
        HealthCheckStrategy::ApiCall =>
        {
          // Try a lightweight API call
          let url = format!( "{endpoint_url}/api/tags" );
          match client.get( &url ).send().await
          {
            Ok( response ) => response.status().is_success(),
            Err( _ ) => false,
          }
        },
        HealthCheckStrategy::VersionCheck =>
        {
          // Try to get version information
          let url = format!( "{endpoint_url}/api/version" );
          match client.get( &url ).send().await
          {
            Ok( response ) => response.status().is_success(),
            Err( _ ) => false,
          }
        },
      }
    }
  }
}

#[ cfg( feature = "health_checks" ) ]
crate ::mod_interface!
{
  exposed use
  {
    HealthCheckStrategy,
    HealthCheckConfig,
    HealthStatus,
    HealthMetrics,
  };
  orphan use private::HealthCheckManager;
}
