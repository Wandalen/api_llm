mod private
{
  //! Automatic failover support for multi-endpoint deployments.
  //!
  //! This module provides health tracking and automatic rotation between
  //! multiple API endpoints to ensure high availability.
  //!
  //! # Design Decisions
  //!
  //! ## Why `Arc< Mutex< FailoverState > >`?
  //!
  //! The failover state must be shared between the Client and multiple concurrent
  //! requests while allowing mutation of health metrics. Using Arc< Mutex<> > instead
  //! of alternatives:
  //!
  //! - **`Arc< RwLock<> >`**: Would provide better read concurrency, but failover
  //!   operations are fast (just index increments) and contention is rare since
  //!   most operations are reads of `current_endpoint` which we cache.
  //!
  //! - **`AtomicUsize` for index**: Would avoid locks for index reads but wouldn't
  //!   solve the problem of updating health metrics (failure counts, timestamps),
  //!   which require coordinated updates.
  //!
  //! - **Message passing**: Would add async complexity and latency to every request.
  //!
  //! Mutex is simpler and the lock is held for microseconds (just updating counters).
  //!
  //! ## Health State Transitions
  //!
  //! Endpoints follow a 3-state model:
  //!
  //! ```text
  //! Healthy ──(1st failure)──> Degraded ──(max failures)──> Unhealthy
  //!    ↑                                                          │
  //!    └─────────────────(success or cooldown)────────────────────┘
  //! ```
  //!
  //! **Rationale**: The Degraded state allows the system to detect intermittent
  //! issues without immediately marking endpoints as unavailable. This reduces
  //! unnecessary failovers for transient network glitches.
  //!
  //! ## Auto-Rotation vs Manual Control
  //!
  //! Auto-rotation (`auto_rotate : true`) automatically switches to backup endpoints
  //! when the current one becomes unhealthy. This is disabled by default because:
  //!
  //! 1. **Explicit Control**: Aligns with "Thin Client, Rich API" principle - the
  //!    application decides when failover occurs, not the library.
  //!
  //! 2. **Testing**: Manual control makes failover behavior predictable in tests.
  //!
  //! 3. **Debugging**: Applications may want to observe failures before switching.
  //!
  //! However, production deployments typically enable auto-rotation for automatic
  //! recovery without application intervention.
  //!
  //! ## Cooldown Period
  //!
  //! Unhealthy endpoints cannot be retried until `retry_after` duration elapses.
  //! After cooldown, they transition to Degraded (not Healthy) to allow gradual
  //! recovery validation.
  //!
  //! **Rationale**: Prevents rapid retry storms that could overwhelm a recovering
  //! endpoint. The gradual recovery (Degraded first) allows the system to detect
  //! if the endpoint is truly recovered before fully trusting it again.

  use std::sync::{ Arc, Mutex };
  use std::time::{ Duration, Instant };

  /// Endpoint health status.
  #[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
  pub enum EndpointHealth
  {
    /// Endpoint is healthy and available.
    Healthy,

    /// Endpoint is degraded but usable.
    Degraded,

    /// Endpoint is unhealthy and should be avoided.
    Unhealthy,
  }

  /// Failover configuration.
  ///
  /// Configures automatic endpoint failover behavior.
  ///
  /// # Examples
  ///
  /// ```
  /// use api_xai::FailoverConfig;
  /// use std::time::Duration;
  ///
  /// let config = FailoverConfig::default()
  ///   .with_max_failures( 3 )
  ///   .with_retry_after( Duration::from_secs( 60 ) );
  /// ```
  #[ derive( Debug, Clone ) ]
  pub struct FailoverConfig
  {
    /// Maximum consecutive failures before marking endpoint unhealthy.
    pub max_failures : usize,

    /// Duration to wait before retrying unhealthy endpoint.
    pub retry_after : Duration,

    /// Whether to automatically rotate endpoints on failure.
    pub auto_rotate : bool,
  }

  impl Default for FailoverConfig
  {
    fn default() -> Self
    {
      Self
      {
        max_failures : 3,
        retry_after : Duration::from_secs( 60 ),
        auto_rotate : true,
      }
    }
  }

  impl FailoverConfig
  {
    /// Sets maximum failures before marking unhealthy.
    #[ must_use ]
    pub fn with_max_failures( mut self, failures : usize ) -> Self
    {
      self.max_failures = failures;
      self
    }

    /// Sets retry delay for unhealthy endpoints.
    #[ must_use ]
    pub fn with_retry_after( mut self, duration : Duration ) -> Self
    {
      self.retry_after = duration;
      self
    }

    /// Enables or disables automatic rotation.
    #[ must_use ]
    pub fn with_auto_rotate( mut self, enable : bool ) -> Self
    {
      self.auto_rotate = enable;
      self
    }
  }

  /// Endpoint state tracking.
  #[ derive( Debug, Clone ) ]
  struct EndpointState
  {
    base_url : String,
    consecutive_failures : usize,
    last_failure_time : Option< Instant >,
    health : EndpointHealth,
  }

  /// Failover manager for multiple XAI endpoints.
  ///
  /// Manages automatic failover between multiple XAI API endpoints,
  /// tracking health and rotating on failures.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use api_xai::{ FailoverManager, FailoverConfig };
  ///
  /// let manager = FailoverManager::new(
  ///   vec![
  ///     "https://api.x.ai/v1/".to_string(),
  ///     "https://api-backup.x.ai/v1/".to_string(),
  ///   ],
  ///   FailoverConfig::default()
  /// );
  ///
  /// // Get current endpoint
  /// let endpoint = manager.current_endpoint();
  /// ```
  #[ derive( Debug, Clone ) ]
  pub struct FailoverManager
  {
    config : FailoverConfig,
    state : Arc< Mutex< FailoverState > >,
  }

  #[ derive( Debug ) ]
  struct FailoverState
  {
    endpoints : Vec< EndpointState >,
    current_index : usize,
  }

  impl FailoverManager
  {
    /// Creates a new failover manager with multiple endpoints.
    ///
    /// # Arguments
    ///
    /// * `endpoints` - List of base URLs for XAI API endpoints
    /// * `config` - Failover configuration
    ///
    /// # Panics
    ///
    /// Panics if endpoints list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::{ FailoverManager, FailoverConfig };
    ///
    /// let manager = FailoverManager::new(
    ///   vec![ "https://api.x.ai/v1/".to_string() ],
    ///   FailoverConfig::default()
    /// );
    /// ```
    pub fn new( endpoints : Vec< String >, config : FailoverConfig ) -> Self
    {
      assert!( !endpoints.is_empty(), "Must provide at least one endpoint" );

      let endpoint_states = endpoints
        .into_iter()
        .map( | base_url | EndpointState
        {
          base_url,
          consecutive_failures : 0,
          last_failure_time : None,
          health : EndpointHealth::Healthy,
        } )
        .collect();

      Self
      {
        config,
        state : Arc::new( Mutex::new( FailoverState
        {
          endpoints : endpoint_states,
          current_index : 0,
        } ) ),
      }
    }

    /// Returns the current endpoint URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::{ FailoverManager, FailoverConfig };
    ///
    /// let manager = FailoverManager::new(
    ///   vec![ "https://api.x.ai/v1/".to_string() ],
    ///   FailoverConfig::default()
    /// );
    ///
    /// let endpoint = manager.current_endpoint();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn current_endpoint( &self ) -> String
    {
      let state = self.state.lock().unwrap();
      state.endpoints[ state.current_index ].base_url.clone()
    }

    /// Records a successful request to current endpoint.
    ///
    /// Resets failure counter and marks endpoint as healthy.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn record_success( &self )
    {
      let mut state = self.state.lock().unwrap();
      let current_idx = state.current_index;
      let endpoint = &mut state.endpoints[ current_idx ];
      endpoint.consecutive_failures = 0;
      endpoint.health = EndpointHealth::Healthy;
    }

    /// Records a failed request to current endpoint.
    ///
    /// Increments failure counter and may trigger rotation if configured.
    ///
    /// # Returns
    ///
    /// `true` if endpoint was rotated, `false` otherwise.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn record_failure( &self ) -> bool
    {
      let mut state = self.state.lock().unwrap();
      let current_idx = state.current_index;
      let endpoint = &mut state.endpoints[ current_idx ];

      endpoint.consecutive_failures += 1;
      endpoint.last_failure_time = Some( Instant::now() );

      // Mark unhealthy if threshold reached
      if endpoint.consecutive_failures >= self.config.max_failures
      {
        endpoint.health = EndpointHealth::Unhealthy;
      }
      else if endpoint.consecutive_failures > 0
      {
        endpoint.health = EndpointHealth::Degraded;
      }

      // Auto-rotate if enabled and endpoint is unhealthy
      if self.config.auto_rotate && endpoint.health == EndpointHealth::Unhealthy
      {
        self.rotate_internal( &mut state );
        true
      }
      else
      {
        false
      }
    }

    /// Manually rotates to next available endpoint.
    ///
    /// Skips unhealthy endpoints unless all are unhealthy.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::{ FailoverManager, FailoverConfig };
    ///
    /// let manager = FailoverManager::new(
    ///   vec![
    ///     "https://api.x.ai/v1/".to_string(),
    ///     "https://backup.x.ai/v1/".to_string(),
    ///   ],
    ///   FailoverConfig::default()
    /// );
    ///
    /// manager.rotate();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn rotate( &self )
    {
      let mut state = self.state.lock().unwrap();
      self.rotate_internal( &mut state );
    }

    fn rotate_internal( &self, state : &mut FailoverState )
    {
      let total = state.endpoints.len();
      let start_idx = state.current_index;

      // Try to find a healthy endpoint
      for offset in 1..=total
      {
        let idx = ( start_idx + offset ) % total;
        let endpoint = &mut state.endpoints[ idx ];

        // Check if unhealthy endpoint can be retried
        if endpoint.health == EndpointHealth::Unhealthy
        {
          if let Some( last_failure ) = endpoint.last_failure_time
          {
            if last_failure.elapsed() >= self.config.retry_after
            {
              // Retry period elapsed, mark as degraded
              endpoint.health = EndpointHealth::Degraded;
              endpoint.consecutive_failures = 0;
            }
            else
            {
              continue; // Still in cooldown
            }
          }
        }

        // Use this endpoint
        state.current_index = idx;
        return;
      }

      // All endpoints unhealthy, use next anyway (circular)
      state.current_index = ( start_idx + 1 ) % total;
    }

    /// Returns health status of all endpoints.
    ///
    /// # Examples
    ///
    /// ```
    /// use api_xai::{ FailoverManager, FailoverConfig };
    ///
    /// let manager = FailoverManager::new(
    ///   vec![ "https://api.x.ai/v1/".to_string() ],
    ///   FailoverConfig::default()
    /// );
    ///
    /// let health = manager.endpoint_health();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn endpoint_health( &self ) -> Vec< ( String, EndpointHealth ) >
    {
      let state = self.state.lock().unwrap();
      state
        .endpoints
        .iter()
        .map( | e | ( e.base_url.clone(), e.health ) )
        .collect()
    }

    /// Returns index of current endpoint.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn current_index( &self ) -> usize
    {
      self.state.lock().unwrap().current_index
    }

    /// Returns total number of endpoints.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn endpoint_count( &self ) -> usize
    {
      self.state.lock().unwrap().endpoints.len()
    }

    /// Resets all endpoints to healthy state.
    ///
    /// # Panics
    ///
    /// Panics if the internal mutex is poisoned.
    pub fn reset( &self )
    {
      let mut state = self.state.lock().unwrap();
      for endpoint in &mut state.endpoints
      {
        endpoint.consecutive_failures = 0;
        endpoint.last_failure_time = None;
        endpoint.health = EndpointHealth::Healthy;
      }
      state.current_index = 0;
    }
  }
}

crate::mod_interface!
{
  exposed use
  {
    EndpointHealth,
    FailoverConfig,
    FailoverManager,
  };
}
