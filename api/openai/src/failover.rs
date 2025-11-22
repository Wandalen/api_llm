//! Failover Module
//!
//! This module provides stateless failover utilities for `OpenAI` API requests.
//! Following the "Thin Client, Rich API" principle, this module offers failover patterns
//! and endpoint management tools without automatic behaviors or magic thresholds.

mod private
{
  use std::
  {
    sync ::{ Arc, Mutex },
    time ::Instant,
  };
  use core::time::Duration;
  use serde::{ Deserialize, Serialize };
  use tokio::sync::mpsc;

  /// Endpoint health status
  #[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
  pub enum EndpointHealth
  {
    /// Endpoint is healthy and available
    Healthy,
    /// Endpoint is degraded but still usable
    Degraded,
    /// Endpoint is unhealthy and should be avoided
    Unhealthy,
    /// Endpoint health is unknown
    Unknown,
  }

  /// Failover strategy for selecting endpoints
  #[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
  pub enum FailoverStrategy
  {
    /// Round-robin through available endpoints
    RoundRobin,
    /// Priority-based selection (highest priority first)
    Priority,
    /// Random selection from available endpoints
    Random,
    /// Sticky to first healthy endpoint
    Sticky,
  }

  /// Endpoint configuration for failover
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct FailoverEndpoint
  {
    /// Unique identifier for the endpoint
    pub id : String,
    /// Endpoint URL
    pub url : String,
    /// Priority level (higher = more preferred)
    pub priority : i32,
    /// Maximum timeout for requests to this endpoint
    pub timeout : Duration,
    /// Current health status
    pub health : EndpointHealth,
    /// Last health check timestamp
    #[ serde( skip, default = "Instant::now" ) ]
    pub last_checked : Instant,
  }

  impl FailoverEndpoint
  {
    /// Create a new failover endpoint
    #[ inline ]
    #[ must_use ]
    pub fn new( id : String, url : String, priority : i32, timeout : Duration ) -> Self
    {
      Self
      {
        id,
        url,
        priority,
        timeout,
        health : EndpointHealth::Unknown,
        last_checked : Instant::now(),
      }
    }

    /// Check if the endpoint is available (healthy or degraded)
    #[ inline ]
    #[ must_use ]
    pub fn is_available( &self ) -> bool
    {
      matches!( self.health, EndpointHealth::Healthy | EndpointHealth::Degraded )
    }

    /// Check if the endpoint is preferred (healthy only)
    #[ inline ]
    #[ must_use ]
    pub fn is_preferred( &self ) -> bool
    {
      matches!( self.health, EndpointHealth::Healthy )
    }

    /// Update the health status of the endpoint
    #[ inline ]
    pub fn update_health( &mut self, health : EndpointHealth )
    {
      self.health = health;
      self.last_checked = Instant::now();
    }

    /// Get time since last health check
    #[ inline ]
    #[ must_use ]
    pub fn time_since_check( &self ) -> Duration
    {
      self.last_checked.elapsed()
    }
  }

  /// Failover configuration and policy
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct FailoverConfig
  {
    /// Strategy for selecting endpoints
    pub strategy : FailoverStrategy,
    /// Maximum number of retry attempts
    pub max_retries : u32,
    /// Base delay between retries (in milliseconds)
    pub retry_delay_ms : u64,
    /// Maximum delay between retries (in milliseconds)
    pub max_retry_delay_ms : u64,
    /// Health check interval (in milliseconds)
    pub health_check_interval_ms : u64,
    /// Timeout for failover operations (in milliseconds)
    pub failover_timeout_ms : u64,
  }

  impl Default for FailoverConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        strategy : FailoverStrategy::Priority,
        max_retries : 3,
        retry_delay_ms : 1000,
        max_retry_delay_ms : 30000,
        health_check_interval_ms : 30000,
        failover_timeout_ms : 10000,
      }
    }
  }

  /// Failover context representing the current state of a failover attempt
  #[ derive( Debug, Clone ) ]
  pub struct FailoverContext
  {
    /// Current attempt number (1-indexed)
    pub attempt : u32,
    /// Endpoint being attempted
    pub endpoint : FailoverEndpoint,
    /// Time when the attempt started
    pub started_at : Instant,
    /// Previous failed endpoints in this context
    pub failed_endpoints : Vec< String >,
  }

  impl FailoverContext
  {
    /// Create a new failover context
    #[ inline ]
    #[ must_use ]
    pub fn new( endpoint : FailoverEndpoint ) -> Self
    {
      Self
      {
        attempt : 1,
        endpoint,
        started_at : Instant::now(),
        failed_endpoints : Vec::new(),
      }
    }

    /// Create next attempt with different endpoint
    #[ inline ]
    #[ must_use ]
    pub fn next_attempt( mut self, endpoint : FailoverEndpoint ) -> Self
    {
      self.failed_endpoints.push( self.endpoint.id.clone() );
      self.attempt += 1;
      self.endpoint = endpoint;
      self.started_at = Instant::now();
      self
    }

    /// Check if maximum retries exceeded
    #[ inline ]
    #[ must_use ]
    pub fn is_exhausted( &self, max_retries : u32 ) -> bool
    {
      self.attempt > max_retries
    }

    /// Get elapsed time for current attempt
    #[ inline ]
    #[ must_use ]
    pub fn elapsed( &self ) -> Duration
    {
      self.started_at.elapsed()
    }

    /// Check if endpoint was already tried
    #[ inline ]
    #[ must_use ]
    pub fn was_endpoint_tried( &self, endpoint_id : &str ) -> bool
    {
      self.failed_endpoints.contains( &endpoint_id.to_string() ) || self.endpoint.id == endpoint_id
    }
  }

  /// Failover manager for endpoint selection and health tracking
  #[ derive( Debug ) ]
  pub struct FailoverManager
  {
    /// Configuration for failover behavior
    config : FailoverConfig,
    /// List of available endpoints
    endpoints : Vec< FailoverEndpoint >,
    /// Round-robin index for round-robin strategy
    round_robin_index : Arc< Mutex< usize > >,
  }

  impl FailoverManager
  {
    /// Create a new failover manager
    #[ inline ]
    #[ must_use ]
    pub fn new( config : FailoverConfig, endpoints : Vec< FailoverEndpoint > ) -> Self
    {
      Self
      {
        config,
        endpoints,
        round_robin_index : Arc::new( Mutex::new( 0 ) ),
      }
    }

    /// Get the failover configuration
    #[ inline ]
    #[ must_use ]
    pub fn config( &self ) -> &FailoverConfig
    {
      &self.config
    }

    /// Get all endpoints
    #[ inline ]
    #[ must_use ]
    pub fn endpoints( &self ) -> &Vec< FailoverEndpoint >
    {
      &self.endpoints
    }

    /// Update the health of a specific endpoint
    #[ inline ]
    pub fn update_endpoint_health( &mut self, endpoint_id : &str, health : EndpointHealth )
    {
      if let Some( endpoint ) = self.endpoints.iter_mut().find( | e | e.id == endpoint_id )
      {
        endpoint.update_health( health );
      }
    }

    /// Get the next endpoint according to the failover strategy
    #[ inline ]
    #[ must_use ]
    pub fn select_endpoint( &self, context : Option< &FailoverContext > ) -> Option< FailoverEndpoint >
    {
      let available_endpoints : Vec< &FailoverEndpoint > = self.endpoints
        .iter()
        .filter( | e | e.is_available() )
        .filter( | e | context.map_or( true, | ctx | !ctx.was_endpoint_tried( &e.id ) ) )
        .collect();

      if available_endpoints.is_empty()
      {
        return None;
      }

      match self.config.strategy
      {
        FailoverStrategy::RoundRobin => self.select_round_robin( &available_endpoints ),
        FailoverStrategy::Priority => Self::select_priority( &available_endpoints ),
        FailoverStrategy::Random => Self::select_random( &available_endpoints ),
        FailoverStrategy::Sticky => Self::select_sticky( &available_endpoints ),
      }
    }

    /// Select endpoint using round-robin strategy
    fn select_round_robin( &self, endpoints : &[ &FailoverEndpoint ] ) -> Option< FailoverEndpoint >
    {
      if endpoints.is_empty()
      {
        return None;
      }

      let mut index = self.round_robin_index.lock().unwrap();
      let selected = endpoints[ *index % endpoints.len() ].clone();
      *index = ( *index + 1 ) % endpoints.len();
      Some( selected )
    }

    /// Select endpoint using priority strategy
    fn select_priority( endpoints : &[ &FailoverEndpoint ] ) -> Option< FailoverEndpoint >
    {
      endpoints
        .iter()
        .max_by_key( | e | e.priority )
        .map( | e | ( *e ).clone() )
    }

    /// Select endpoint using random strategy
    fn select_random( endpoints : &[ &FailoverEndpoint ] ) -> Option< FailoverEndpoint >
    {
      if endpoints.is_empty()
      {
        return None;
      }

      // Simple pseudo-random selection based on current time
      let index = ( Instant::now().elapsed().as_nanos() as usize ) % endpoints.len();
      Some( endpoints[ index ].clone() )
    }

    /// Select endpoint using sticky strategy (first healthy)
    fn select_sticky( endpoints : &[ &FailoverEndpoint ] ) -> Option< FailoverEndpoint >
    {
      endpoints
        .iter()
        .find( | e | e.is_preferred() )
        .or_else( || endpoints.first() )
        .map( | e | ( *e ).clone() )
    }

    /// Get healthy endpoints count
    #[ inline ]
    #[ must_use ]
    pub fn healthy_endpoint_count( &self ) -> usize
    {
      self.endpoints.iter().filter( | e | e.is_preferred() ).count()
    }

    /// Get available endpoints count
    #[ inline ]
    #[ must_use ]
    pub fn available_endpoint_count( &self ) -> usize
    {
      self.endpoints.iter().filter( | e | e.is_available() ).count()
    }

    /// Calculate retry delay with exponential backoff
    #[ inline ]
    #[ must_use ]
    pub fn calculate_retry_delay( &self, attempt : u32 ) -> Duration
    {
      let base_delay = Duration::from_millis( self.config.retry_delay_ms );
      let max_delay = Duration::from_millis( self.config.max_retry_delay_ms );

      // Exponential backoff : base_delay * 2^(attempt-1)
      let multiplier = 2_u64.saturating_pow( attempt.saturating_sub( 1 ) );
      let calculated_delay = base_delay.saturating_mul( u32::try_from( multiplier ).unwrap_or( u32::MAX ) );

      core ::cmp::min( calculated_delay, max_delay )
    }
  }

  /// Failover execution utilities
  #[ derive( Debug ) ]
  pub struct FailoverExecutor;

  impl FailoverExecutor
  {
    /// Execute a function with failover logic
    ///
    /// # Errors
    /// Returns an error if all endpoints fail, no endpoints are available, or the operation fails on all retry attempts.
    ///
    /// # Panics
    ///
    /// This function should not panic under normal circumstances as the context is always initialized before use.
    #[ inline ]
    pub async fn execute_with_failover< T, E, F, Fut >(
      manager : &FailoverManager,
      operation : F,
    ) -> Result< T, FailoverError< E > >
    where
      F : Fn( FailoverContext ) -> Fut + Send + Sync,
      Fut : core::future::Future< Output = Result< T, E > > + Send,
      E : Send + Sync + 'static,
    {
      let mut context = None;

      for attempt in 1..=manager.config.max_retries
      {
        // Select next endpoint
        let Some( endpoint ) = manager.select_endpoint( context.as_ref() ) else {
          return Err( FailoverError::NoAvailableEndpoints );
        };

        // Create or update context
        context = Some( match context
        {
          Some( ctx ) => ctx.next_attempt( endpoint ),
          None => FailoverContext::new( endpoint ),
        });

        let ctx = context.as_ref().unwrap();

        // Execute operation
        match operation( ctx.clone() ).await
        {
          Ok( result ) => return Ok( result ),
          Err( error ) =>
          {
            if attempt == manager.config.max_retries
            {
              return Err( FailoverError::AllEndpointsFailed( Box::new( error ) ) );
            }

            // Calculate delay before next attempt
            let delay = manager.calculate_retry_delay( attempt );
            tokio ::time::sleep( delay ).await;
          }
        }
      }

      Err( FailoverError::MaxRetriesExceeded )
    }

    /// Create a failover event notifier
    #[ inline ]
    #[ must_use ]
    pub fn create_failover_notifier() -> ( FailoverEventSender, FailoverEventReceiver )
    {
      let ( tx, rx ) = mpsc::unbounded_channel();
      ( FailoverEventSender { sender : tx }, FailoverEventReceiver { receiver : rx } )
    }

    /// Validate failover configuration
    ///
    /// # Errors
    /// Returns an error if the configuration contains invalid values such as zero retries or zero delay.
    #[ inline ]
    pub fn validate_config( config : &FailoverConfig ) -> Result< (), String >
    {
      if config.max_retries == 0
      {
        return Err( "max_retries must be greater than 0".to_string() );
      }

      if config.retry_delay_ms == 0
      {
        return Err( "retry_delay_ms must be greater than 0".to_string() );
      }

      if config.max_retry_delay_ms < config.retry_delay_ms
      {
        return Err( "max_retry_delay_ms must be greater than or equal to retry_delay_ms".to_string() );
      }

      Ok( () )
    }

    /// Create a basic failover manager with default configuration
    #[ inline ]
    #[ must_use ]
    pub fn create_basic_manager( endpoints : Vec< ( String, String, i32 ) > ) -> FailoverManager
    {
      let failover_endpoints : Vec< FailoverEndpoint > = endpoints
        .into_iter()
        .map( | ( id, url, priority ) |
        {
          FailoverEndpoint::new( id, url, priority, Duration::from_secs( 30 ) )
        })
        .collect();

      FailoverManager::new( FailoverConfig::default(), failover_endpoints )
    }
  }

  /// Failover error types
  #[ derive( Debug ) ]
  pub enum FailoverError< E >
  {
    /// No endpoints are available for failover
    NoAvailableEndpoints,
    /// All endpoints failed during failover attempts
    AllEndpointsFailed( Box< E > ),
    /// Maximum retry attempts exceeded
    MaxRetriesExceeded,
    /// Configuration validation failed
    ConfigurationError( String ),
  }

  impl< E > core::fmt::Display for FailoverError< E >
  where
    E : core::fmt::Display,
  {
    #[ inline ]
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      match self
      {
        FailoverError::NoAvailableEndpoints => write!( f, "No endpoints are available for failover" ),
        FailoverError::AllEndpointsFailed( error ) => write!( f, "All endpoints failed : {error}" ),
        FailoverError::MaxRetriesExceeded => write!( f, "Maximum retry attempts exceeded" ),
        FailoverError::ConfigurationError( msg ) => write!( f, "Configuration error : {msg}" ),
      }
    }
  }

  impl< E > std::error::Error for FailoverError< E >
  where
    E : std::error::Error + 'static,
  {
    #[ inline ]
    fn source( &self ) -> Option< &( dyn std::error::Error + 'static ) >
    {
      match self
      {
        FailoverError::AllEndpointsFailed( error ) => Some( error.as_ref() ),
        _ => None,
      }
    }
  }

  /// Failover event types
  #[ derive( Debug, Clone ) ]
  pub enum FailoverEvent
  {
    /// Endpoint health changed
    HealthChanged
    {
      /// ID of the endpoint that changed
      endpoint_id : String,
      /// Previous health status
      old_health : EndpointHealth,
      /// New health status
      new_health : EndpointHealth,
    },
    /// Failover attempt started
    FailoverStarted
    {
      /// ID of the endpoint being attempted
      endpoint_id : String,
      /// Attempt number
      attempt : u32,
    },
    /// Failover attempt succeeded
    FailoverSucceeded
    {
      /// ID of the successful endpoint
      endpoint_id : String,
      /// Attempt number that succeeded
      attempt : u32,
    },
    /// Failover attempt failed
    FailoverFailed
    {
      /// ID of the failed endpoint
      endpoint_id : String,
      /// Attempt number that failed
      attempt : u32,
      /// Error message
      error : String,
    },
    /// All endpoints exhausted
    AllEndpointsExhausted
    {
      /// Total number of attempts made
      total_attempts : u32,
    },
  }

  /// Sender for failover events
  #[ derive( Debug, Clone ) ]
  pub struct FailoverEventSender
  {
    sender : mpsc::UnboundedSender< FailoverEvent >,
  }

  impl FailoverEventSender
  {
    /// Send a failover event
    ///
    /// # Errors
    /// Returns an error if the event channel is closed or the receiver has been dropped.
    #[ inline ]
    pub fn send_event( &self, event : FailoverEvent ) -> Result< (), &'static str >
    {
      self.sender.send( event ).map_err( | _ | "Failed to send failover event" )
    }

    /// Send health change event
    ///
    /// # Errors
    /// Returns an error if the event cannot be sent due to channel closure or receiver unavailability.
    #[ inline ]
    pub fn send_health_change( &self, endpoint_id : String, old_health : EndpointHealth, new_health : EndpointHealth ) -> Result< (), &'static str >
    {
      self.send_event( FailoverEvent::HealthChanged { endpoint_id, old_health, new_health } )
    }

    /// Send failover started event
    ///
    /// # Errors
    /// Returns an error if the event cannot be sent due to channel closure or receiver unavailability.
    #[ inline ]
    pub fn send_failover_started( &self, endpoint_id : String, attempt : u32 ) -> Result< (), &'static str >
    {
      self.send_event( FailoverEvent::FailoverStarted { endpoint_id, attempt } )
    }
  }

  /// Receiver for failover events
  #[ derive( Debug ) ]
  pub struct FailoverEventReceiver
  {
    receiver : mpsc::UnboundedReceiver< FailoverEvent >,
  }

  impl FailoverEventReceiver
  {
    /// Try to receive a failover event (non-blocking)
    #[ inline ]
    pub fn try_recv( &mut self ) -> Option< FailoverEvent >
    {
      self.receiver.try_recv().ok()
    }

    /// Receive next failover event (blocking)
    #[ inline ]
    pub async fn recv( &mut self ) -> Option< FailoverEvent >
    {
      self.receiver.recv().await
    }
  }
}

crate ::mod_interface!
{
  exposed use private::EndpointHealth;
  exposed use private::FailoverStrategy;
  exposed use private::FailoverEndpoint;
  exposed use private::FailoverConfig;
  exposed use private::FailoverContext;
  exposed use private::FailoverManager;
  exposed use private::FailoverExecutor;
  exposed use private::FailoverError;
  exposed use private::FailoverEvent;
  exposed use private::FailoverEventSender;
  exposed use private::FailoverEventReceiver;
}