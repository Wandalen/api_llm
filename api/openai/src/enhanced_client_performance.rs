//! Performance Monitoring and Analysis for Enhanced `OpenAI` Client
//!
//! This module provides performance metrics, analysis, and reporting structures
//! for monitoring and optimizing `EnhancedClient` performance.

use mod_interface::mod_interface;

mod private
{
  use serde::{ Serialize, Deserialize };

  /// Connection performance metrics for monitoring
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ConnectionPerformanceReport
  {
    /// Overall efficiency metrics
    pub efficiency_metrics : crate::connection_manager::ConnectionEfficiencyMetrics,
    /// Per-pool statistics
    pub pool_stats : Vec< crate::connection_manager::PoolStatistics >,
    /// Performance analysis
    pub analysis : PerformanceAnalysis,
  }

  /// Analysis of connection performance
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct PerformanceAnalysis
  {
    /// Overall performance grade (A, B, C, D, F)
    pub grade : String,
    /// Key performance indicators
    pub kpis : Vec< String >,
    /// Recommendations for improvement
    pub recommendations : Vec< String >,
    /// Potential issues identified
    pub issues : Vec< String >,
  }

  /// Unified performance dashboard combining all components
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct UnifiedPerformanceDashboard
  {
    /// Overall performance score (0-100)
    pub overall_performance_score : f64,
    /// Connection performance metrics
    pub connection_performance : ConnectionPerformanceReport,
    /// Cache performance statistics (if available)
    #[ cfg( feature = "caching" ) ]
    pub cache_performance : Option< crate::response_cache::CacheStatistics >,
    /// Placeholder for cache performance when feature is disabled
    #[ cfg( not( feature = "caching" ) ) ]
    pub cache_performance : Option< () >,
    /// Metrics summary (if available)
    pub metrics_summary : Option< crate::metrics_framework::MetricsSnapshot >,
    /// Unified recommendations from all components
    pub recommendations : Vec< String >,
  }

  /// Analyze connection performance and provide recommendations
  #[ must_use ]
  #[ inline ]
  pub fn analyze_performance(
    efficiency : &crate::connection_manager::ConnectionEfficiencyMetrics,
    pools : &[ crate::connection_manager::PoolStatistics ],
  ) -> PerformanceAnalysis
  {
    let mut kpis = Vec::new();
    let mut recommendations = Vec::new();
    let mut issues = Vec::new();

    // Analyze efficiency score
    let grade = match efficiency.efficiency_score
    {
      s if s >= 0.9 => "A",
      s if s >= 0.8 => "B",
      s if s >= 0.7 => "C",
      s if s >= 0.6 => "D",
      _ => "F",
    };

    kpis.push( format!( "Efficiency Score : {:.1}%", efficiency.efficiency_score * 100.0 ) );
    kpis.push( format!( "Connection Reuse Ratio : {:.1}", efficiency.connection_reuse_ratio ) );
    kpis.push( format!( "Average Pool Utilization : {:.1}%", efficiency.average_pool_utilization * 100.0 ) );

    // Analyze connection reuse
    if efficiency.connection_reuse_ratio < 5.0
    {
      issues.push( "Low connection reuse - connections are not being reused efficiently".to_string() );
      recommendations.push( "Increase connection pool size or check for connection leaks".to_string() );
    }
    else if efficiency.connection_reuse_ratio > 100.0
    {
      issues.push( "Extremely high connection reuse - may indicate connection starvation".to_string() );
      recommendations.push( "Increase maximum connections per host".to_string() );
    }

    // Analyze pool utilization
    if efficiency.average_pool_utilization < 0.3
    {
      recommendations.push( "Pool utilization is low - consider reducing minimum connections".to_string() );
    }
    else if efficiency.average_pool_utilization > 0.9
    {
      issues.push( "High pool utilization - may cause connection wait times".to_string() );
      recommendations.push( "Increase maximum connections per host".to_string() );
    }

    // Analyze individual pools
    for pool in pools
    {
      if pool.current_utilization > 0.95
      {
        issues.push( format!( "Pool for {} is overutilized ({:.1}%)", pool.host, pool.current_utilization * 100.0 ) );
      }

      if pool.total_connections_created > pool.total_requests_served
      {
        issues.push( format!( "Pool for {} has poor connection reuse", pool.host ) );
      }
    }

    // General recommendations
    if efficiency.efficiency_score < 0.8
    {
      recommendations.push( "Consider tuning connection pool parameters".to_string() );
      recommendations.push( "Monitor connection health and cleanup intervals".to_string() );
    }

    PerformanceAnalysis
    {
      grade : grade.to_string(),
      kpis,
      recommendations,
      issues,
    }
  }
}

mod_interface!
{
  exposed use
  {
    ConnectionPerformanceReport,
    PerformanceAnalysis,
    UnifiedPerformanceDashboard,
    analyze_performance,
  };
}
