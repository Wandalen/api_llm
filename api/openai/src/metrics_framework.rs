//! Metrics Collection Framework
//!
//! This module provides a comprehensive metrics collection and analysis framework for
//! the `OpenAI` API client. Following the "Thin Client, Rich API" principle, this module
//! offers configurable metrics collection without automatic behaviors.

use mod_interface::mod_interface;

mod private
{
  use crate::
  {
    error ::{ OpenAIError, Result },
    connection_manager ::{ ConnectionEfficiencyMetrics, PoolStatistics },
  };

  // Feature-gated imports
  #[ cfg( feature = "caching" ) ]
  use crate::response_cache::CacheStatistics;

  // Import circuit breaker stats when feature is enabled (temporarily disabled)
  // #[ cfg( feature = "circuit_breaker" ) ]
  // use crate::enhanced_circuit_breaker::CircuitBreakerStats;

  use std::
  {
    collections ::HashMap,
    sync ::Arc,
    time ::{ SystemTime, UNIX_EPOCH },
  };
  use core::
  {
    fmt ::Write,
    time ::Duration,
  };
  use std::time::Instant;
  use tokio::sync::RwLock;
  use serde::{ Serialize, Deserialize };

  /// Configuration for metrics collection behavior
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  #[ allow( clippy::struct_excessive_bools ) ]
  pub struct MetricsConfig
  {
    /// Whether to collect connection metrics
    pub collect_connection_metrics : bool,
    /// Whether to collect cache metrics
    pub collect_cache_metrics : bool,
    /// Whether to collect circuit breaker metrics
    pub collect_circuit_breaker_metrics : bool,
    /// Whether to collect request timing metrics
    pub collect_timing_metrics : bool,
    /// Whether to collect error metrics
    pub collect_error_metrics : bool,
    /// Maximum number of metric entries to retain
    pub max_entries : usize,
    /// Interval for automatic metric collection
    pub collection_interval : Duration,
    /// Data retention period
    pub retention_period : Duration,
    /// Whether to enable real-time metric streaming
    pub enable_streaming : bool,
  }

  impl Default for MetricsConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        collect_connection_metrics : true,
        collect_cache_metrics : true,
        collect_circuit_breaker_metrics : true,
        collect_timing_metrics : true,
        collect_error_metrics : true,
        max_entries : 10000,
        collection_interval : Duration::from_secs( 10 ),
        retention_period : Duration::from_secs( 3600 ), // 1 hour
        enable_streaming : false,
      }
    }
  }

  /// Comprehensive metrics snapshot
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct MetricsSnapshot
  {
    /// Timestamp when metrics were collected
    pub timestamp : u64,
    /// Connection-related metrics
    pub connection_metrics : Option< ConnectionMetrics >,
    /// Cache-related metrics
    pub cache_metrics : Option< CacheMetrics >,
    /// Circuit breaker metrics
    #[ cfg( feature = "circuit_breaker" ) ]
    pub circuit_breaker_metrics : Option< CircuitBreakerMetrics >,
    /// Request timing metrics
    pub timing_metrics : Option< TimingMetrics >,
    /// Error metrics
    pub error_metrics : Option< ErrorMetrics >,
  }

  /// Connection performance metrics
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ConnectionMetrics
  {
    /// Overall efficiency score
    pub efficiency_score : f64,
    /// Connection reuse ratio
    pub connection_reuse_ratio : f64,
    /// Average pool utilization
    pub average_pool_utilization : f64,
    /// Total requests served
    pub total_requests_served : u64,
    /// Average response time in seconds
    pub average_response_time_seconds : f64,
    /// Current number of active connections
    pub active_connections : usize,
    /// Connection health score
    pub health_score : f64,
  }

  /// Cache performance metrics
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct CacheMetrics
  {
    /// Total cache requests
    pub total_requests : u64,
    /// Cache hits
    pub cache_hits : u64,
    /// Cache misses
    pub cache_misses : u64,
    /// Hit ratio (0.0 to 1.0)
    pub hit_ratio : f64,
    /// Current cached entries
    pub current_entries : usize,
    /// Total cached bytes
    pub total_cached_bytes : usize,
    /// Average TTL in seconds
    pub average_ttl_seconds : f64,
    /// Cache efficiency score
    pub efficiency_score : f64,
  }

  /// Circuit breaker metrics
  #[ cfg( feature = "circuit_breaker" ) ]
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct CircuitBreakerMetrics
  {
    /// Current circuit breaker state
    pub state : String,
    /// Total requests through circuit breaker
    pub total_requests : u64,
    /// Total failures detected
    pub total_failures : u64,
    /// Number of times circuit breaker tripped
    pub trip_count : u64,
    /// Failure rate (0.0 to 1.0)
    pub failure_rate : f64,
    /// Time in current state (seconds)
    pub time_in_state_seconds : f64,
  }

  /// Request timing metrics
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct TimingMetrics
  {
    /// Average request duration in milliseconds
    pub average_duration_ms : f64,
    /// Minimum request duration in milliseconds
    pub min_duration_ms : f64,
    /// Maximum request duration in milliseconds
    pub max_duration_ms : f64,
    /// 95th percentile duration in milliseconds
    pub p95_duration_ms : f64,
    /// 99th percentile duration in milliseconds
    pub p99_duration_ms : f64,
    /// Total number of timed requests
    pub total_requests : u64,
  }

  /// Error tracking metrics
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ErrorMetrics
  {
    /// Total errors encountered
    pub total_errors : u64,
    /// Error breakdown by type
    pub error_types : HashMap<  String, u64  >,
    /// Error rate (errors per minute)
    pub error_rate_per_minute : f64,
    /// Most common error type
    pub most_common_error : Option< String >,
    /// Error trend (increasing, stable, decreasing)
    pub trend : String,
  }

  /// Time-series data point for metrics
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct MetricsDataPoint
  {
    /// Timestamp of the data point
    pub timestamp : u64,
    /// Metric name
    pub metric_name : String,
    /// Metric value
    pub value : f64,
    /// Additional tags/labels
    pub tags : HashMap<  String, String  >,
  }

  /// Metrics aggregation statistics
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct MetricsAggregation
  {
    /// Time period of aggregation
    pub period_seconds : u64,
    /// Start timestamp
    pub start_timestamp : u64,
    /// End timestamp
    pub end_timestamp : u64,
    /// Aggregated metrics
    pub aggregated_data : HashMap<  String, f64  >,
    /// Data quality score
    pub quality_score : f64,
  }

  /// Comprehensive metrics analysis report
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct MetricsAnalysisReport
  {
    /// Analysis timestamp
    pub timestamp : u64,
    /// Overall system health score (0.0 to 1.0)
    pub health_score : f64,
    /// Performance grade (A, B, C, D, F)
    pub performance_grade : String,
    /// Key performance indicators
    pub kpis : Vec< String >,
    /// Performance trends
    pub trends : Vec< String >,
    /// Identified issues
    pub issues : Vec< String >,
    /// Recommendations for improvement
    pub recommendations : Vec< String >,
    /// Risk assessment
    pub risk_level : String,
  }

  /// Central metrics collector and analyzer
  #[ derive( Debug ) ]
  pub struct MetricsCollector
  {
    /// Configuration
    config : MetricsConfig,
    /// Historical metrics data
    metrics_history : Arc< RwLock< Vec< MetricsSnapshot > > >,
    /// Request timing data
    timing_data : Arc< RwLock< Vec< f64 > > >,
    /// Error tracking
    error_counts : Arc< RwLock< HashMap<  String, u64  > > >,
    /// Collection start time
    start_time : Instant,
    /// Background collection task handle
    collection_handle : Option< tokio::task::JoinHandle< () > >,
  }

  impl Default for MetricsCollector
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::with_config( MetricsConfig::default() )
    }
  }

  impl MetricsCollector
  {
    /// Create a new metrics collector with default configuration
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self::default()
    }

    /// Create a new metrics collector with custom configuration
    #[ inline ]
    #[ must_use ]
    pub fn with_config( config : MetricsConfig ) -> Self
    {
      Self
      {
        config,
        metrics_history : Arc::new( RwLock::new( Vec::new() ) ),
        timing_data : Arc::new( RwLock::new( Vec::new() ) ),
        error_counts : Arc::new( RwLock::new( HashMap::new() ) ),
        start_time : Instant::now(),
        collection_handle : None,
      }
    }

    /// Start automatic metrics collection
    #[ inline ]
    pub fn start_collection( &mut self )
    {
      if self.config.collection_interval > Duration::ZERO
      {
        let metrics_history = Arc::clone( &self.metrics_history );
        let timing_data = Arc::clone( &self.timing_data );
        let _error_counts = Arc::clone( &self.error_counts );
        let config = self.config.clone();

        let handle = tokio::spawn( async move
        {
          let mut interval = tokio::time::interval( config.collection_interval );
          loop
          {
            interval.tick().await;

            // Perform automatic cleanup of old metrics
            let retention_cutoff = SystemTime::now()
              .duration_since( UNIX_EPOCH )
              .unwrap_or_default()
              .as_secs()
              .saturating_sub( config.retention_period.as_secs() );

            {
              let mut history = metrics_history.write().await;
              history.retain( | snapshot | snapshot.timestamp > retention_cutoff );

              // Limit history size
              if history.len() > config.max_entries
              {
                let excess = history.len() - config.max_entries;
                history.drain( 0..excess );
              }
            }

            {
              let mut timing = timing_data.write().await;
              if timing.len() > config.max_entries
              {
                let excess = timing.len() - config.max_entries;
                timing.drain( 0..excess );
              }
            }
          }
        } );

        self.collection_handle = Some( handle );
      }
    }

    /// Stop automatic metrics collection
    #[ inline ]
    pub fn stop_collection( &mut self )
    {
      if let Some( handle ) = self.collection_handle.take()
      {
        handle.abort();
      }
    }

    /// Record a request timing
    #[ inline ]
    pub async fn record_timing( &self, duration : Duration )
    {
      if self.config.collect_timing_metrics
      {
        let mut timing_data = self.timing_data.write().await;
        timing_data.push( duration.as_millis() as f64 );
      }
    }

    /// Record an error occurrence
    #[ inline ]
    pub async fn record_error( &self, error_type : &str )
    {
      if self.config.collect_error_metrics
      {
        let mut error_counts = self.error_counts.write().await;
        *error_counts.entry( error_type.to_string() ).or_insert( 0 ) += 1;
      }
    }

    /// Collect current metrics snapshot
    #[ inline ]
    pub async fn collect_snapshot(
      &self,
      connection_metrics : Option< &ConnectionEfficiencyMetrics >,
      pool_stats : Option< &Vec< PoolStatistics > >,
      #[ cfg( feature = "caching" ) ]
      cache_stats : Option< &CacheStatistics >,
      #[ cfg( not( feature = "caching" ) ) ]
      _cache_stats : Option< &() >,
      _circuit_breaker_stats : Option< &() >, // Placeholder until circuit breaker provides stats
    ) -> MetricsSnapshot
    {
      let timestamp = SystemTime::now()
        .duration_since( UNIX_EPOCH )
        .unwrap_or_default()
        .as_secs();

      let connection_metrics = if self.config.collect_connection_metrics
      {
        connection_metrics.map( | cm | Self::build_connection_metrics( cm, pool_stats ) )
      }
      else
      {
        None
      };

      #[ cfg( feature = "caching" ) ]
      let cache_metrics = if self.config.collect_cache_metrics
      {
        cache_stats.map( Self::build_cache_metrics )
      }
      else
      {
        None
      };

      #[ cfg( not( feature = "caching" ) ) ]
      let cache_metrics = None;

      // Circuit breaker metrics are temporarily disabled
      #[ cfg( feature = "circuit_breaker" ) ]
      let circuit_breaker_metrics = None;

      let timing_metrics = if self.config.collect_timing_metrics
      {
        Some( self.build_timing_metrics().await )
      }
      else
      {
        None
      };

      let error_metrics = if self.config.collect_error_metrics
      {
        Some( self.build_error_metrics().await )
      }
      else
      {
        None
      };

      MetricsSnapshot
      {
        timestamp,
        connection_metrics,
        cache_metrics,
        #[ cfg( feature = "circuit_breaker" ) ]
        circuit_breaker_metrics,
        timing_metrics,
        error_metrics,
      }
    }

    /// Store metrics snapshot in history
    #[ inline ]
    pub async fn store_snapshot( &self, snapshot : MetricsSnapshot )
    {
      let mut history = self.metrics_history.write().await;
      history.push( snapshot );

      // Maintain size limit
      if history.len() > self.config.max_entries
      {
        history.remove( 0 );
      }
    }

    /// Get metrics history
    #[ inline ]
    pub async fn get_history( &self ) -> Vec< MetricsSnapshot >
    {
      let history = self.metrics_history.read().await;
      history.clone()
    }

    /// Generate comprehensive analysis report
    #[ inline ]
    pub async fn generate_analysis_report( &self ) -> MetricsAnalysisReport
    {
      let history = self.metrics_history.read().await;
      let timestamp = SystemTime::now()
        .duration_since( UNIX_EPOCH )
        .unwrap_or_default()
        .as_secs();

      if history.is_empty()
      {
        return MetricsAnalysisReport
        {
          timestamp,
          health_score : 0.0,
          performance_grade : "N/A".to_string(),
          kpis : vec![ "No metrics data available".to_string() ],
          trends : vec![],
          issues : vec![ "Insufficient metrics data for analysis".to_string() ],
          recommendations : vec![ "Enable metrics collection and allow time for data accumulation".to_string() ],
          risk_level : "Unknown".to_string(),
        };
      }

      let latest = &history[ history.len() - 1 ];
      let mut health_scores = Vec::new();
      let mut kpis = Vec::new();
      let mut trends = Vec::new();
      let mut issues = Vec::new();
      let mut recommendations = Vec::new();

      // Analyze connection metrics
      if let Some( ref conn_metrics ) = latest.connection_metrics
      {
        health_scores.push( conn_metrics.health_score );
        kpis.push( format!( "Connection Efficiency : {:.1}%", conn_metrics.efficiency_score * 100.0 ) );

        if conn_metrics.efficiency_score < 0.7
        {
          issues.push( "Low connection efficiency detected".to_string() );
          recommendations.push( "Review connection pool configuration".to_string() );
        }
      }

      // Analyze cache metrics
      if let Some( ref cache_metrics ) = latest.cache_metrics
      {
        health_scores.push( cache_metrics.efficiency_score );
        kpis.push( format!( "Cache Hit Ratio : {:.1}%", cache_metrics.hit_ratio * 100.0 ) );

        if cache_metrics.hit_ratio < 0.5
        {
          issues.push( "Low cache hit ratio".to_string() );
          recommendations.push( "Review cache TTL settings and request patterns".to_string() );
        }
      }

      // Analyze error metrics
      if let Some( ref error_metrics ) = latest.error_metrics
      {
        kpis.push( format!( "Error Rate : {:.1}/min", error_metrics.error_rate_per_minute ) );

        if error_metrics.error_rate_per_minute > 5.0
        {
          issues.push( "High error rate detected".to_string() );
          recommendations.push( "Investigate root cause of errors".to_string() );
        }
      }

      // Calculate overall health score
      let health_score = if health_scores.is_empty()
      {
        0.5 // Neutral score when no health data available
      }
      else
      {
        health_scores.iter().sum::< f64 >() / health_scores.len() as f64
      };

      // Determine performance grade
      let performance_grade = match health_score
      {
        s if s >= 0.9 => "A",
        s if s >= 0.8 => "B",
        s if s >= 0.7 => "C",
        s if s >= 0.6 => "D",
        _ => "F",
      };

      // Determine risk level
      let risk_level = match health_score
      {
        s if s >= 0.8 => "Low",
        s if s >= 0.6 => "Medium",
        _ => "High",
      };

      // Analyze trends if we have enough history
      if history.len() >= 5
      {
        trends.push( "Trend analysis available".to_string() );
      }
      else
      {
        trends.push( "Insufficient data for trend analysis".to_string() );
      }

      MetricsAnalysisReport
      {
        timestamp,
        health_score,
        performance_grade : performance_grade.to_string(),
        kpis,
        trends,
        issues,
        recommendations,
        risk_level : risk_level.to_string(),
      }
    }

    /// Export metrics to JSON format
    ///
    /// # Errors
    ///
    /// Returns an error if JSON serialization fails.
    #[ inline ]
    pub async fn export_json( &self ) -> Result< String >
    {
      let history = self.get_history().await;
      serde_json ::to_string_pretty( &history )
        .map_err( | e | OpenAIError::Internal( format!( "Failed to export metrics to JSON: {e}" ) ).into() )
    }

    /// Export metrics to Prometheus format
    #[ inline ]
    pub async fn export_prometheus( &self ) -> String
    {
      let mut output = String::new();
      let history = self.metrics_history.read().await;

      if let Some( latest ) = history.last()
      {
        // Connection metrics
        if let Some( ref conn_metrics ) = latest.connection_metrics
        {
          output.push_str( "# HELP openai_connection_efficiency Connection efficiency score\n" );
          output.push_str( "# TYPE openai_connection_efficiency gauge\n" );
          let _ = writeln!( output, "openai_connection_efficiency {}", conn_metrics.efficiency_score );

          output.push_str( "# HELP openai_connection_reuse_ratio Connection reuse ratio\n" );
          output.push_str( "# TYPE openai_connection_reuse_ratio gauge\n" );
          let _ = writeln!( output, "openai_connection_reuse_ratio {}", conn_metrics.connection_reuse_ratio );
        }

        // Cache metrics
        if let Some( ref cache_metrics ) = latest.cache_metrics
        {
          output.push_str( "# HELP openai_cache_hit_ratio Cache hit ratio\n" );
          output.push_str( "# TYPE openai_cache_hit_ratio gauge\n" );
          let _ = writeln!( output, "openai_cache_hit_ratio {}", cache_metrics.hit_ratio );

          output.push_str( "# HELP openai_cache_entries Current cache entries\n" );
          output.push_str( "# TYPE openai_cache_entries gauge\n" );
          let _ = writeln!( output, "openai_cache_entries {}", cache_metrics.current_entries );
        }
      }

      output
    }

    /// Get current configuration
    #[ inline ]
    #[ must_use ]
    pub fn get_config( &self ) -> &MetricsConfig
    {
      &self.config
    }

    /// Update configuration
    #[ inline ]
    pub fn update_config( &mut self, new_config : MetricsConfig )
    {
      self.config = new_config;
    }

    // Helper methods for building specific metric types

    fn build_connection_metrics(
      conn_metrics : &ConnectionEfficiencyMetrics,
      pool_stats : Option< &Vec< PoolStatistics > >
    ) -> ConnectionMetrics
    {
      let active_connections = pool_stats
        .map_or( 0, | pools | pools.iter().map( | p | p.in_use_connections ).sum() );

      let health_score = if conn_metrics.efficiency_score > 0.8 { 0.9 } else { conn_metrics.efficiency_score };

      ConnectionMetrics
      {
        efficiency_score : conn_metrics.efficiency_score,
        connection_reuse_ratio : conn_metrics.connection_reuse_ratio,
        average_pool_utilization : conn_metrics.average_pool_utilization,
        total_requests_served : conn_metrics.total_requests_served,
        average_response_time_seconds : 0.0, // Note : Requires timing data collection infrastructure
        active_connections,
        health_score,
      }
    }

    #[ cfg( feature = "caching" ) ]
    fn build_cache_metrics( cache_stats : &CacheStatistics ) -> CacheMetrics
    {
      let efficiency_score = if cache_stats.hit_ratio > 0.8 { 0.9 } else { cache_stats.hit_ratio };

      CacheMetrics
      {
        total_requests : cache_stats.total_requests,
        cache_hits : cache_stats.cache_hits,
        cache_misses : cache_stats.cache_misses,
        hit_ratio : cache_stats.hit_ratio,
        current_entries : cache_stats.current_entries,
        total_cached_bytes : cache_stats.total_cached_bytes,
        average_ttl_seconds : cache_stats.average_ttl_seconds,
        efficiency_score,
      }
    }

    async fn build_timing_metrics( &self ) -> TimingMetrics
    {
      let timing_data = self.timing_data.read().await;

      if timing_data.is_empty()
      {
        return TimingMetrics
        {
          average_duration_ms : 0.0,
          min_duration_ms : 0.0,
          max_duration_ms : 0.0,
          p95_duration_ms : 0.0,
          p99_duration_ms : 0.0,
          total_requests : 0,
        };
      }

      let mut sorted_data = timing_data.clone();
      sorted_data.sort_by( | a, b | a.partial_cmp( b ).unwrap_or( core::cmp::Ordering::Equal ) );

      let average_duration_ms = sorted_data.iter().sum::< f64 >() / sorted_data.len() as f64;
      let min_duration_ms = sorted_data[ 0 ];
      let max_duration_ms = sorted_data[ sorted_data.len() - 1 ];

      let len = sorted_data.len();
      let p95_index = ((len * 95) / 100).min(len.saturating_sub(1));
      let p99_index = ((len * 99) / 100).min(len.saturating_sub(1));

      let p95_duration_ms = sorted_data.get( p95_index ).copied().unwrap_or( max_duration_ms );
      let p99_duration_ms = sorted_data.get( p99_index ).copied().unwrap_or( max_duration_ms );

      TimingMetrics
      {
        average_duration_ms,
        min_duration_ms,
        max_duration_ms,
        p95_duration_ms,
        p99_duration_ms,
        total_requests : sorted_data.len() as u64,
      }
    }

    async fn build_error_metrics( &self ) -> ErrorMetrics
    {
      let error_counts = self.error_counts.read().await;

      let total_errors = error_counts.values().sum();

      let most_common_error = error_counts
        .iter()
        .max_by_key( | ( _, count ) | *count )
        .map( | ( error_type, _ ) | error_type.clone() );

      let elapsed_minutes = self.start_time.elapsed().as_secs_f64() / 60.0;
      let error_rate_per_minute = if elapsed_minutes > 0.0
      {
        total_errors as f64 / elapsed_minutes
      }
      else
      {
        0.0
      };

      let trend = if error_rate_per_minute < 1.0
      {
        "Stable".to_string()
      }
      else if error_rate_per_minute < 5.0
      {
        "Increasing".to_string()
      }
      else
      {
        "Critical".to_string()
      };

      ErrorMetrics
      {
        total_errors,
        error_types : error_counts.clone(),
        error_rate_per_minute,
        most_common_error,
        trend,
      }
    }
  }

  impl Drop for MetricsCollector
  {
    #[ inline ]
    fn drop( &mut self )
    {
      self.stop_collection();
    }
  }

  #[ cfg( test ) ]
  mod tests
  {
    use super::*;

    #[ tokio::test ]
    async fn test_metrics_collector_creation()
    {
      let collector = MetricsCollector::new();
      assert!( collector.get_config().collect_connection_metrics );
    }

    #[ tokio::test ]
    async fn test_timing_recording()
    {
      let collector = MetricsCollector::new();
      collector.record_timing( Duration::from_millis( 100 ) ).await;

      let timing_metrics = collector.build_timing_metrics().await;
      assert_eq!( timing_metrics.total_requests, 1 );
      assert!( (timing_metrics.average_duration_ms - 100.0).abs() < f64::EPSILON, "Expected average_duration_ms to be approximately 100.0, got {}", timing_metrics.average_duration_ms );
    }

    #[ tokio::test ]
    async fn test_error_recording()
    {
      let collector = MetricsCollector::new();
      collector.record_error( "timeout" ).await;
      collector.record_error( "timeout" ).await;
      collector.record_error( "network" ).await;

      let error_metrics = collector.build_error_metrics().await;
      assert_eq!( error_metrics.total_errors, 3 );
      assert_eq!( error_metrics.most_common_error, Some( "timeout".to_string() ) );
    }

    #[ tokio::test ]
    async fn test_metrics_export()
    {
      let collector = MetricsCollector::new();
      let json_export = collector.export_json().await.unwrap();
      assert!( json_export.starts_with( '[' ) );

      let prometheus_export = collector.export_prometheus().await;
      assert!( prometheus_export.contains( "# HELP" ) || prometheus_export.is_empty() );
    }
  }
}

mod_interface!
{
  orphan use private::
  {
    MetricsConfig,
    MetricsSnapshot,
    ConnectionMetrics,
    CacheMetrics,
    TimingMetrics,
    ErrorMetrics,
    MetricsDataPoint,
    MetricsAggregation,
    MetricsAnalysisReport,
    MetricsCollector,
  };

  #[ cfg( feature = "circuit_breaker" ) ]
  orphan use private::
  {
    CircuitBreakerMetrics,
  };
}