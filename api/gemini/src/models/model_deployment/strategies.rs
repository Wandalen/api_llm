//! Deployment strategies and caching for performance optimization

use std::sync::{ Arc, RwLock };
use std::sync::atomic::{ AtomicU64, Ordering };
use std::time::SystemTime;
use std::collections::HashMap;

use super::DeploymentSummary;

/// Deployment strategy types
#[ derive( Debug, Clone ) ]
pub enum DeploymentStrategy
{
  /// Rolling update strategy
  Rolling {
    /// Maximum percentage of pods that can be unavailable
    max_unavailable_percentage : f64,
    /// Maximum percentage of pods that can be created above desired count
    max_surge_percentage : f64,
  },
  /// Blue-green deployment strategy
  BlueGreen {
    /// Percentage of traffic to switch to new version
    switch_traffic_percentage : f64,
    /// Whether to automatically rollback on failure
    rollback_on_failure : bool,
  },
  /// Canary deployment strategy
  Canary {
    /// Initial percentage of traffic for canary
    traffic_percentage : f64,
    /// Criteria for promoting canary to full deployment
    promotion_criteria : Vec< String >,
  },
}

/// Deployment caching strategy for performance optimization
#[ derive( Debug ) ]
pub struct DeploymentCache
{
  /// Maximum cache size
  max_size : usize,
  /// Cache TTL in seconds
  ttl_seconds : u64,
  /// Cached deployment summaries with timestamps
  cache : Arc< RwLock< HashMap<  String, ( DeploymentSummary, SystemTime )  > > >,
  /// Cache hit count
  hits : AtomicU64,
  /// Cache miss count
  misses : AtomicU64,
}

impl DeploymentCache
{
  /// Create a new deployment cache
  pub fn new( max_size : usize, ttl_seconds : u64 ) -> Self
  {
    Self {
      max_size,
      ttl_seconds,
      cache : Arc::new( RwLock::new( HashMap::new() ) ),
      hits : AtomicU64::new( 0 ),
      misses : AtomicU64::new( 0 ),
    }
  }

  /// Get a deployment summary from cache
  pub fn get( &self, deployment_id : &str ) -> Option< DeploymentSummary >
  {
    let cache = self.cache.read().unwrap();
    if let Some( ( deployment_summary, timestamp ) ) = cache.get( deployment_id )
    {
      let age = SystemTime::now()
        .duration_since( *timestamp )
        .unwrap_or_default()
        .as_secs();

      if age <= self.ttl_seconds
      {
        self.hits.fetch_add( 1, Ordering::Relaxed );
        return Some( deployment_summary.clone() );
      }
    }

    self.misses.fetch_add( 1, Ordering::Relaxed );
    None
  }

  /// Put a deployment summary in cache
  pub fn put( &self, deployment_id : String, deployment_summary : DeploymentSummary )
  {
    let mut cache = self.cache.write().unwrap();

    // Evict expired entries if cache is full
    if cache.len() >= self.max_size
    {
      let now = SystemTime::now();
      cache.retain( | _, ( _, timestamp ) | {
        now.duration_since( *timestamp )
          .unwrap_or_default()
          .as_secs() <= self.ttl_seconds
      } );

      // If still full, remove oldest entry
      if cache.len() >= self.max_size
      {
        if let Some( oldest_key ) = cache
          .iter()
          .min_by_key( | ( _, ( _, timestamp ) ) | timestamp )
          .map( | ( key, _ ) | key.clone() )
        {
          cache.remove( &oldest_key );
        }
      }
    }

    cache.insert( deployment_id, ( deployment_summary, SystemTime::now() ) );
  }

  /// Get cache statistics
  pub fn stats( &self ) -> ( u64, u64, f64 )
  {
    let hits = self.hits.load( Ordering::Relaxed );
    let misses = self.misses.load( Ordering::Relaxed );
    let total = hits + misses;
    let hit_rate = if total > 0 { hits as f64 / total as f64 * 100.0 } else { 0.0 };
    ( hits, misses, hit_rate )
  }

  /// Clear the cache
  pub fn clear( &self )
  {
    self.cache.write().unwrap().clear();
    self.hits.store( 0, Ordering::Relaxed );
    self.misses.store( 0, Ordering::Relaxed );
  }
}
