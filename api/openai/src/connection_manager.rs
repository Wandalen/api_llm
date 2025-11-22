//! Advanced HTTP Connection Management System
//!
//! This module provides sophisticated connection pooling, health monitoring,
//! and adaptive connection management for optimal HTTP client performance.

use mod_interface::mod_interface;

#[ allow(clippy::std_instead_of_core) ] // Duration/Instant not available in core
mod private
{
  use std::
  {
    collections ::{ HashMap, VecDeque },
    sync ::Arc,
    time ::{ Duration, Instant },
  };
  use core::sync::atomic::{ AtomicU64, AtomicUsize, Ordering };
  use tokio::sync::{ RwLock, Mutex };
  use reqwest::Client as HttpClient;
  use serde::{ Serialize, Deserialize };

  /// Configuration for advanced connection management
  #[ derive( Debug, Clone ) ]
  pub struct ConnectionConfig
  {
    /// Maximum connections per host
    pub max_connections_per_host : usize,
    /// Minimum connections to maintain per host
    pub min_connections_per_host : usize,
    /// Connection idle timeout
    pub idle_timeout : Duration,
    /// Connection health check interval
    pub health_check_interval : Duration,
    /// Enable adaptive pooling based on usage patterns
    pub adaptive_pooling : bool,
    /// Connection warming for frequently used endpoints
    pub enable_connection_warming : bool,
    /// Maximum time to wait for a connection
    pub connection_wait_timeout : Duration,
  }

  impl Default for ConnectionConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        max_connections_per_host : 20,
        min_connections_per_host : 2,
        idle_timeout : Duration::from_secs( 120 ),
        health_check_interval : Duration::from_secs( 30 ),
        adaptive_pooling : true,
        enable_connection_warming : true,
        connection_wait_timeout : Duration::from_secs( 10 ),
      }
    }
  }

  /// Connection health status
  #[ derive( Debug, Clone, PartialEq ) ]
  pub enum ConnectionHealth
  {
    /// Connection is healthy and available
    Healthy,
    /// Connection is degraded but usable
    Degraded,
    /// Connection is unhealthy and should be replaced
    Unhealthy,
    /// Connection is being tested
    Testing,
  }

  /// Connection usage statistics
  #[ derive( Debug, Clone ) ]
  pub struct ConnectionStats
  {
    /// Number of requests served by this connection
    pub requests_served : u64,
    /// Last time this connection was used
    pub last_used : Instant,
    /// Connection creation time
    pub created_at : Instant,
    /// Average response time for this connection
    pub avg_response_time : Duration,
    /// Number of failures on this connection
    pub failure_count : u64,
    /// Current health status
    pub health : ConnectionHealth,
  }

  impl Default for ConnectionStats
  {
    #[ inline ]
    fn default() -> Self
    {
      let now = Instant::now();
      Self
      {
        requests_served : 0,
        last_used : now,
        created_at : now,
        avg_response_time : Duration::from_millis( 100 ),
        failure_count : 0,
        health : ConnectionHealth::Healthy,
      }
    }
  }

  /// Enhanced HTTP client with advanced connection management
  #[ derive( Debug ) ]
  pub struct ManagedConnection
  {
    /// The actual HTTP client
    pub client : HttpClient,
    /// Connection statistics and health
    pub stats : Arc< RwLock< ConnectionStats > >,
    /// Unique identifier for this connection
    pub id : String,
    /// Host this connection is optimized for
    pub host : String,
  }

  impl ManagedConnection
  {
    /// Create a new managed connection
    #[ inline ]
    #[ must_use ]
    pub fn new( client : HttpClient, host : String ) -> Self
    {
      Self
      {
        client,
        stats : Arc::new( RwLock::new( ConnectionStats::default() ) ),
        id : uuid::Uuid::new_v4().to_string(),
        host,
      }
    }

    /// Record successful request on this connection
    #[ inline ]
    pub async fn record_success( &self, response_time : Duration )
    {
      let mut stats = self.stats.write().await;
      stats.requests_served += 1;
      stats.last_used = Instant::now();

      // Update rolling average response time
      let current_avg = stats.avg_response_time.as_millis() as f64;
      let new_response = response_time.as_millis() as f64;
      let weight = (stats.requests_served - 1) as f64 / stats.requests_served as f64;
      let new_avg = current_avg * weight + new_response * (1.0 - weight);
      #[ allow( clippy::cast_possible_truncation, clippy::cast_sign_loss ) ]
      {
        stats.avg_response_time = Duration::from_millis( new_avg.max( 0.0 ) as u64 );
      }

      // Reset health to healthy on successful request
      if stats.health != ConnectionHealth::Healthy
      {
        stats.health = ConnectionHealth::Healthy;
        stats.failure_count = 0;
      }
    }

    /// Record failed request on this connection
    #[ inline ]
    pub async fn record_failure( &self )
    {
      let mut stats = self.stats.write().await;
      stats.failure_count += 1;
      stats.last_used = Instant::now();

      // Degrade health based on failure count
      stats.health = match stats.failure_count
      {
        1..=2 => ConnectionHealth::Degraded,
        _ => ConnectionHealth::Unhealthy,
      };
    }

    /// Check if connection is idle and should be cleaned up
    #[ inline ]
    pub async fn is_idle( &self, idle_timeout : Duration ) -> bool
    {
      let stats = self.stats.read().await;
      stats.last_used.elapsed() > idle_timeout
    }

    /// Get connection health score (0.0 = unhealthy, 1.0 = perfect)
    #[ inline ]
    pub async fn health_score( &self ) -> f64
    {
      let stats = self.stats.read().await;
      match stats.health
      {
        ConnectionHealth::Healthy => 1.0,
        ConnectionHealth::Degraded => 0.6,
        ConnectionHealth::Unhealthy => 0.1,
        ConnectionHealth::Testing => 0.5,
      }
    }
  }

  /// Connection pool manager for a specific host
  #[ derive( Debug ) ]
  pub struct HostConnectionPool
  {
    /// Host this pool manages
    pub host : String,
    /// Available connections
    pub available : Arc< Mutex< VecDeque< Arc< ManagedConnection > > > >,
    /// Currently in-use connections
    pub in_use : Arc< RwLock< HashMap< String, Arc< ManagedConnection > > > >,
    /// Pool configuration
    pub config : ConnectionConfig,
    /// Pool-level statistics
    pub pool_stats : Arc< RwLock< PoolStats > >,
  }

  /// Pool-level statistics
  #[ derive( Debug, Default ) ]
  pub struct PoolStats
  {
    /// Total connections created
    pub connections_created : AtomicU64,
    /// Total connections destroyed
    pub connections_destroyed : AtomicU64,
    /// Total requests served
    pub total_requests : AtomicU64,
    /// Average pool utilization
    pub avg_utilization : f64,
    /// Peak simultaneous connections
    pub peak_connections : AtomicUsize,
  }

  impl HostConnectionPool
  {
    /// Create new connection pool for host
    #[ inline ]
    #[ must_use ]
    pub fn new( host : String, config : ConnectionConfig ) -> Self
    {
      Self
      {
        host,
        available : Arc::new( Mutex::new( VecDeque::new() ) ),
        in_use : Arc::new( RwLock::new( HashMap::new() ) ),
        config,
        pool_stats : Arc::new( RwLock::new( PoolStats::default() ) ),
      }
    }

    /// Get a connection from the pool or create a new one
    ///
    /// # Errors
    ///
    /// Returns an error if a new connection cannot be created when the pool is empty
    /// or when all existing connections are unhealthy.
    #[ inline ]
    pub async fn get_connection( &self ) -> Result< Arc< ManagedConnection >, reqwest::Error >
    {
      // Try to get an available healthy connection
      if let Some( conn ) = self.get_healthy_connection().await
      {
        self.mark_in_use( &conn ).await;
        return Ok( conn );
      }

      // Create new connection if under limits
      if self.can_create_new_connection().await
      {
        let conn = self.create_new_connection().await?;
        self.mark_in_use( &conn ).await;
        return Ok( conn );
      }

      // Wait for connection to become available
      self.wait_for_connection().await
    }

    /// Get a healthy connection from available pool
    async fn get_healthy_connection( &self ) -> Option< Arc< ManagedConnection > >
    {
      let mut available = self.available.lock().await;
      let mut best_connection = None;
      let mut best_score = 0.0;

      // Find the healthiest available connection
      let mut to_remove = Vec::new();
      for (index, conn) in available.iter().enumerate()
      {
        let score = conn.health_score().await;
        if score > 0.5 && score > best_score
        {
          best_connection = Some( conn.clone() );
          best_score = score;
          to_remove.push( index );
        }
        else if score <= 0.1
        {
          // Mark unhealthy connections for removal
          to_remove.push( index );
        }
      }

      // Remove the selected connection and any unhealthy ones
      if let Some( index ) = to_remove.iter().max()
      {
        available.remove( *index );
      }

      best_connection
    }

    /// Check if we can create a new connection
    async fn can_create_new_connection( &self ) -> bool
    {
      let in_use_count = self.in_use.read().await.len();
      let available_count = self.available.lock().await.len();
      let total_connections = in_use_count + available_count;

      total_connections < self.config.max_connections_per_host
    }

    /// Create a new HTTP connection
    async fn create_new_connection( &self ) -> Result< Arc< ManagedConnection >, reqwest::Error >
    {
      let client = HttpClient::builder()
        .timeout( Duration::from_secs( 300 ) )
        .connect_timeout( Duration::from_secs( 30 ) )
        .pool_max_idle_per_host( self.config.max_connections_per_host )
        .pool_idle_timeout( self.config.idle_timeout )
        .tcp_keepalive( Duration::from_secs( 60 ) )
        .build()?;

      let connection = Arc::new( ManagedConnection::new( client, self.host.clone() ) );

      // Update statistics
      self.pool_stats.read().await.connections_created.fetch_add( 1, Ordering::Relaxed );

      Ok( connection )
    }

    /// Mark connection as in use
    async fn mark_in_use( &self, conn : &Arc< ManagedConnection > )
    {
      let mut in_use = self.in_use.write().await;
      in_use.insert( conn.id.clone(), conn.clone() );

      // Update peak connections if necessary
      let current_peak = self.pool_stats.read().await.peak_connections.load( Ordering::Relaxed );
      let current_count = in_use.len();
      if current_count > current_peak
      {
        self.pool_stats.read().await.peak_connections.store( current_count, Ordering::Relaxed );
      }
    }

    /// Wait for a connection to become available
    async fn wait_for_connection( &self ) -> Result< Arc< ManagedConnection >, reqwest::Error >
    {
      let start = Instant::now();
      while start.elapsed() < self.config.connection_wait_timeout
      {
        if let Some( conn ) = self.get_healthy_connection().await
        {
          self.mark_in_use( &conn ).await;
          return Ok( conn );
        }
        tokio ::time::sleep( Duration::from_millis( 10 ) ).await;
      }

      // Timeout - create connection anyway (will exceed pool limit)
      let conn = self.create_new_connection().await?;
      self.mark_in_use( &conn ).await;
      Ok( conn )
    }

    /// Return connection to available pool
    #[ inline ]
    pub async fn return_connection( &self, conn : Arc< ManagedConnection > )
    {
      // Remove from in-use
      {
        let mut in_use = self.in_use.write().await;
        in_use.remove( &conn.id );
      }

      // Check if connection is still healthy
      let health_score = conn.health_score().await;
      if health_score > 0.5
      {
        // Return to available pool
        let mut available = self.available.lock().await;
        available.push_back( conn );
      }
      else
      {
        // Connection is unhealthy, let it be dropped
        self.pool_stats.read().await.connections_destroyed.fetch_add( 1, Ordering::Relaxed );
      }
    }

    /// Cleanup idle and unhealthy connections
    #[ inline ]
    pub async fn cleanup_connections( &self )
    {
      let mut available = self.available.lock().await;
      let mut to_remove = Vec::new();

      for (index, conn) in available.iter().enumerate()
      {
        if conn.is_idle( self.config.idle_timeout ).await || conn.health_score().await <= 0.1
        {
          to_remove.push( index );
        }
      }

      // Remove from back to front to maintain indices
      for &index in to_remove.iter().rev()
      {
        available.remove( index );
        self.pool_stats.read().await.connections_destroyed.fetch_add( 1, Ordering::Relaxed );
      }

      // Ensure minimum connections if pool is empty
      if available.is_empty() && self.config.min_connections_per_host > 0
      {
        drop( available ); // Release lock before async operation
        for _ in 0..self.config.min_connections_per_host
        {
          if let Ok( conn ) = self.create_new_connection().await
          {
            let mut available = self.available.lock().await;
            available.push_back( conn );
          }
        }
      }
    }

    /// Get pool statistics
    #[ inline ]
    pub async fn get_stats( &self ) -> PoolStatistics
    {
      let stats = self.pool_stats.read().await;
      let available_count = self.available.lock().await.len();
      let in_use_count = self.in_use.read().await.len();

      PoolStatistics
      {
        host : self.host.clone(),
        available_connections : available_count,
        in_use_connections : in_use_count,
        total_connections_created : stats.connections_created.load( Ordering::Relaxed ),
        total_connections_destroyed : stats.connections_destroyed.load( Ordering::Relaxed ),
        total_requests_served : stats.total_requests.load( Ordering::Relaxed ),
        peak_connections : stats.peak_connections.load( Ordering::Relaxed ),
        current_utilization : if available_count + in_use_count > 0 { in_use_count as f64 / (available_count + in_use_count) as f64 } else { 0.0 },
      }
    }
  }

  /// Statistics for a connection pool
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct PoolStatistics
  {
    /// Host this pool serves
    pub host : String,
    /// Currently available connections
    pub available_connections : usize,
    /// Currently in-use connections
    pub in_use_connections : usize,
    /// Total connections created since start
    pub total_connections_created : u64,
    /// Total connections destroyed since start
    pub total_connections_destroyed : u64,
    /// Total requests served by this pool
    pub total_requests_served : u64,
    /// Peak number of simultaneous connections
    pub peak_connections : usize,
    /// Current pool utilization (0.0 to 1.0)
    pub current_utilization : f64,
  }

  /// Global connection manager
  #[ derive( Debug ) ]
  pub struct ConnectionManager
  {
    /// Per-host connection pools
    pools : Arc< RwLock< HashMap< String, Arc< HostConnectionPool > > > >,
    /// Global configuration
    config : ConnectionConfig,
    /// Background cleanup task handle
    cleanup_handle : Option< tokio::task::JoinHandle< () > >,
  }

  impl ConnectionManager
  {
    /// Create new connection manager
    #[ inline ]
    #[ must_use ]
    pub fn new( config : ConnectionConfig ) -> Self
    {
      Self
      {
        pools : Arc::new( RwLock::new( HashMap::new() ) ),
        config,
        cleanup_handle : None,
      }
    }

    /// Start background cleanup task
    #[ inline ]
    pub fn start_background_cleanup( &mut self )
    {
      let pools = Arc::clone( &self.pools );
      let cleanup_interval = self.config.health_check_interval;

      let handle = tokio::spawn( async move
      {
        let mut interval = tokio::time::interval( cleanup_interval );
        loop
        {
          interval.tick().await;

          let pools_guard = pools.read().await;
          let cleanup_tasks : Vec< _ > = pools_guard.values().map( | pool |
          {
            let pool_clone = pool.clone();
            tokio ::spawn( async move
            {
              pool_clone.cleanup_connections().await;
            } )
          } ).collect();
          drop( pools_guard );

          // Wait for all cleanup tasks
          for task in cleanup_tasks
          {
            let _ = task.await;
          }
        }
      } );

      self.cleanup_handle = Some( handle );
    }

    /// Get connection for specific host
    ///
    /// # Errors
    ///
    /// Returns an error if no connection can be obtained from the pool for the specified host.
    #[ inline ]
    pub async fn get_connection( &self, host : &str ) -> Result< Arc< ManagedConnection >, reqwest::Error >
    {
      let pool = self.get_or_create_pool( host ).await;
      pool.get_connection().await
    }

    /// Return connection to pool
    #[ inline ]
    pub async fn return_connection( &self, conn : Arc< ManagedConnection > )
    {
      if let Some( pool ) = self.get_pool( &conn.host ).await
      {
        pool.return_connection( conn ).await;
      }
    }

    /// Get or create pool for host
    async fn get_or_create_pool( &self, host : &str ) -> Arc< HostConnectionPool >
    {
      // Try to get existing pool
      {
        let pools = self.pools.read().await;
        if let Some( pool ) = pools.get( host )
        {
          return pool.clone();
        }
      }

      // Create new pool
      {
        let mut pools = self.pools.write().await;
        // Double-check in case another task created it
        if let Some( pool ) = pools.get( host )
        {
          return pool.clone();
        }

        let pool = Arc::new( HostConnectionPool::new( host.to_string(), self.config.clone() ) );
        pools.insert( host.to_string(), pool.clone() );
        pool
      }
    }

    /// Get existing pool for host
    async fn get_pool( &self, host : &str ) -> Option< Arc< HostConnectionPool > >
    {
      let pools = self.pools.read().await;
      pools.get( host ).cloned()
    }

    /// Get comprehensive statistics for all pools
    #[ inline ]
    pub async fn get_all_stats( &self ) -> Vec< PoolStatistics >
    {
      let pools = self.pools.read().await;
      let mut all_stats = Vec::new();

      for pool in pools.values()
      {
        all_stats.push( pool.get_stats().await );
      }

      all_stats
    }

    /// Get global connection efficiency metrics
    #[ inline ]
    pub async fn get_efficiency_metrics( &self ) -> ConnectionEfficiencyMetrics
    {
      let all_stats = self.get_all_stats().await;

      let total_requests : u64 = all_stats.iter().map( | s | s.total_requests_served ).sum();
      let total_connections_created : u64 = all_stats.iter().map( | s | s.total_connections_created ).sum();
      let total_connections_destroyed : u64 = all_stats.iter().map( | s | s.total_connections_destroyed ).sum();
      let avg_utilization : f64 = if all_stats.is_empty()
      {
        0.0
      }
      else
      {
        all_stats.iter().map( | s | s.current_utilization ).sum::< f64 >() / all_stats.len() as f64
      };

      let connection_reuse_ratio = if total_connections_created > 0
      {
        total_requests as f64 / total_connections_created as f64
      }
      else
      {
        0.0
      };

      ConnectionEfficiencyMetrics
      {
        total_requests_served : total_requests,
        total_connections_created,
        total_connections_destroyed,
        active_pools : all_stats.len(),
        average_pool_utilization : avg_utilization,
        connection_reuse_ratio,
        efficiency_score : Self::calculate_efficiency_score( connection_reuse_ratio, avg_utilization ),
      }
    }

    /// Calculate overall efficiency score (0.0 to 1.0)
    fn calculate_efficiency_score( reuse_ratio : f64, utilization : f64 ) -> f64
    {
      // Optimal reuse ratio is around 10-50 requests per connection
      let reuse_score = if (10.0..=50.0).contains(&reuse_ratio)
      {
        1.0
      }
      else if reuse_ratio > 50.0
      {
        1.0 - ((reuse_ratio - 50.0) / 100.0).min( 0.5 )
      }
      else
      {
        reuse_ratio / 10.0
      };

      // Optimal utilization is around 60-80%
      let utilization_score = if (0.6..=0.8).contains(&utilization)
      {
        1.0
      }
      else if utilization > 0.8
      {
        1.0 - (utilization - 0.8) * 2.5
      }
      else
      {
        utilization / 0.6
      };

      (reuse_score + utilization_score) / 2.0
    }
  }

  /// Overall connection efficiency metrics
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ConnectionEfficiencyMetrics
  {
    /// Total requests served across all pools
    pub total_requests_served : u64,
    /// Total connections created
    pub total_connections_created : u64,
    /// Total connections destroyed
    pub total_connections_destroyed : u64,
    /// Number of active connection pools
    pub active_pools : usize,
    /// Average utilization across all pools
    pub average_pool_utilization : f64,
    /// Ratio of requests to connections (higher = better reuse)
    pub connection_reuse_ratio : f64,
    /// Overall efficiency score (0.0 to 1.0)
    pub efficiency_score : f64,
  }

  impl Drop for ConnectionManager
  {
    #[ inline ]
    fn drop( &mut self )
    {
      if let Some( handle ) = self.cleanup_handle.take()
      {
        handle.abort();
      }
    }
  }
}

mod_interface!
{
  exposed use
  {
    ConnectionConfig,
    ConnectionHealth,
    ConnectionStats,
    ManagedConnection,
    HostConnectionPool,
    PoolStatistics,
    ConnectionManager,
    ConnectionEfficiencyMetrics,
  };
}