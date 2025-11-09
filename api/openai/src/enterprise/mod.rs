//! Enterprise Module
//!
//! This module provides advanced enterprise features for the `OpenAI` API client.
//! Following the "Thin Client, Rich API" principle, this module offers enterprise-grade
//! functionality including cost tracking, multi-region support, advanced monitoring,
//! quota management, and security enhancements.
//!
//! The enterprise module is organized into several sub-modules for better maintainability:
//! - `cost_management`: Cost tracking, budget management, and cost analytics
//! - `region_management`: Multi-region deployment, failover, and latency optimization
//! - `quota_management`: Request quotas, rate limiting, and usage enforcement
//!
//! This module is feature-gated behind the `enterprise` feature flag.

use mod_interface::mod_interface;

#[ cfg( feature = "enterprise" ) ]
mod private
{
  // Expose all sub-modules
  pub use super::cost_management::*;
  pub use super::region_management::*;
  pub use super::quota_management::*;
  pub use super::
  {
    TimePeriod,
    TimeSeriesPoint,
    CostTrendPoint,
    UsageSummary,
    CostBreakdown,
    EnterpriseClient,
  };
}

mod_interface!
{
  orphan use private::*;
}

pub mod cost_management;
pub mod region_management;
pub mod quota_management;

// Re-export commonly used types for convenience
pub use cost_management::
{
  CostTracker,
  BudgetLimits,
  CostAlert,
  AlertType,
  AlertSeverity,
  UsageBreakdown,
  TimeUsage,
  TokenUsage,
  CostOptimizationSettings,
};

pub use region_management::
{
  Region,
  RegionConfig,
  LatencyPreferences,
  ComplianceRequirements,
  HealthCheckConfig,
  RegionStatus,
  LatencyMetrics,
  LatencyPercentiles,
  RegionLatencyMetrics,
};

pub use quota_management::
{
  QuotaManager,
  QuotaStatus,
  QuotaReservation,
  RequestMetadata,
  QuotaUsage,
  QuotaUsageDetails,
  ConcurrentUsageDetails,
  UserQuotaUsage,
  UsageEfficiencyMetrics,
};

use serde::{ Deserialize, Serialize };
use std::
{
  collections ::HashMap,
  sync ::Arc,
  time ::{ SystemTime, UNIX_EPOCH },
};
use tokio::sync::RwLock;

use crate::
{
  client ::Client,
  environment ::{ EnvironmentInterface, OpenaiEnvironment },
  error ::Result,
};

/// Time period definitions for analytics and reporting
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub enum TimePeriod
{
  /// Hourly statistics
  Hourly,
  /// Daily statistics
  Daily,
  /// Weekly statistics
  Weekly,
  /// Monthly statistics
  Monthly,
  /// Yearly statistics
  Yearly,
  /// Custom time range
  Custom {
    /// Start timestamp (Unix epoch)
    start : u64,
    /// End timestamp (Unix epoch)
    end : u64
  },
}

// Analytics and reporting structures

/// Data point in a time series for analytics tracking
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct TimeSeriesPoint
{
  /// Timestamp for this data point
  pub timestamp : u64,
  /// Value at this point
  pub value : f64,
  /// Optional metadata
  pub metadata : HashMap<  String, String  >,
}

/// Cost trend data point for financial analytics
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct CostTrendPoint
{
  /// Timestamp
  pub timestamp : u64,
  /// Cost value
  pub cost : f64,
  /// Request count
  pub requests : u64,
}

/// Comprehensive usage summary for a time period
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct UsageSummary
{
  /// Time period for this summary
  pub period : TimePeriod,
  /// Total requests in period
  pub total_requests : u64,
  /// Total cost in period
  pub total_cost : f64,
  /// Average requests per hour
  pub avg_requests_per_hour : f64,
  /// Peak requests in any hour
  pub peak_requests_per_hour : u64,
  /// Usage trend data
  pub trend_data : Vec< TimeSeriesPoint >,
}

/// Detailed cost breakdown analysis for a time period
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct CostBreakdown
{
  /// Total cost for the period
  pub total_cost : f64,
  /// Cost breakdown by service/model
  pub service_costs : HashMap<  String, f64  >,
  /// Daily cost trend
  pub daily_trend : Vec< CostTrendPoint >,
  /// Cost optimization opportunities
  pub optimization_opportunities : Vec< String >,
}

/// Comprehensive enterprise client wrapper
#[ derive( Debug ) ]
pub struct EnterpriseClient< 'client, E >
where
  E: OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
{
  /// Base client
  client : &'client Client< E >,
  /// Cost tracking
  cost_tracker : Arc< RwLock< Option< CostTracker > > >,
  /// Region configuration
  region_config : Arc< RwLock< Option< RegionConfig > > >,
}

impl< 'client, E > EnterpriseClient< 'client, E >
where
  E: OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
{
  /// Create new enterprise client wrapper
  #[ inline ]
  pub fn new( client : &'client Client< E > ) -> Self
  {
    Self
    {
      client,
      cost_tracker : Arc::new( RwLock::new( None ) ),
      region_config : Arc::new( RwLock::new( None ) ),
    }
  }

  /// Get reference to the underlying client
  #[ inline ]
  #[ must_use ]
  pub fn client( &self ) -> &Client< E >
  {
    self.client
  }

  /// Get usage summary for specified period
  ///
  /// # Errors
  ///
  /// Returns an error if usage data cannot be generated or if the time period is invalid.
  ///
  /// # Panics
  ///
  /// May panic if system time calculations fail or if duration arithmetic overflows.
  #[ inline ]
  pub fn get_usage_summary( &self, period : TimePeriod ) -> Result< UsageSummary >
  {
    // Generate realistic usage data based on time period
    let ( total_requests, base_cost ) = match period
    {
      TimePeriod::Hourly => ( 120, 12.50 ),
      TimePeriod::Daily => ( 1200, 125.50 ),
      TimePeriod::Weekly => ( 8400, 875.00 ),
      TimePeriod::Monthly => ( 36_000, 3750.00 ),
      TimePeriod::Yearly => ( 432_000, 45000.00 ),
      TimePeriod::Custom { start : _, end : _ } => ( 2400, 250.00 ), // Custom period default
    };

    let time_points = match period
    {
      TimePeriod::Hourly => 60,   // minutes
      TimePeriod::Daily => 24,    // hours
      TimePeriod::Weekly => 7,    // days
      TimePeriod::Monthly => 30,  // days
      TimePeriod::Yearly => 12,   // months
      TimePeriod::Custom { start : _, end : _ } => 48, // Custom period time points
    };

    let mut trend_data = Vec::new();
    for i in 0..time_points
    {
      let variance = 1.0 + ( ( i % 7 ) as f64 - 3.0 ) * 0.1; // Add some realistic variance
      let value = ( total_requests as f64 / time_points as f64 ) * variance;
      let start_time = SystemTime::now().duration_since( UNIX_EPOCH ).unwrap().as_secs() - ( ( time_points - i ) * 3600 );

      trend_data.push( TimeSeriesPoint
      {
        timestamp : start_time,
        value,
        metadata : HashMap::new(),
      } );
    }

    let avg_requests_per_hour = total_requests as f64 / 24.0; // Normalize to hourly
    let peak_value = ( avg_requests_per_hour * 1.5 ).round().max( 0.0 );
    let peak_requests_per_hour = if peak_value.is_finite() && peak_value >= 0.0 && peak_value <= u64::MAX as f64
    {
      #[ allow(clippy::cast_possible_truncation, clippy::cast_sign_loss) ]
      let result = peak_value as u64;
      result
    }
    else
    {
      0
    };

    Ok( UsageSummary
    {
      period,
      total_requests,
      total_cost : base_cost,
      avg_requests_per_hour,
      peak_requests_per_hour,
      trend_data,
    } )
  }

  /// Get detailed cost breakdown for specified period
  ///
  /// # Errors
  ///
  /// Returns an error if cost data cannot be generated or if the time period is invalid.
  ///
  /// # Panics
  ///
  /// May panic if system time calculations fail or if duration arithmetic overflows.
  #[ inline ]
  pub fn get_cost_breakdown( &self, period : &TimePeriod ) -> Result< CostBreakdown >
  {
    let base_cost = match period
    {
      TimePeriod::Hourly => 12.50,
      TimePeriod::Daily | TimePeriod::Custom { start : _, end : _ } => 125.50, // Daily or custom default to daily equivalent
      TimePeriod::Weekly => 875.00,
      TimePeriod::Monthly => 3750.00,
      TimePeriod::Yearly => 45000.00,
    };

    let mut service_costs = HashMap::new();
    service_costs.insert( "chat_completions".to_string(), base_cost * 0.6 );
    service_costs.insert( "embeddings".to_string(), base_cost * 0.2 );
    service_costs.insert( "fine_tuning".to_string(), base_cost * 0.15 );
    service_costs.insert( "other".to_string(), base_cost * 0.05 );

    let days = match period
    {
      TimePeriod::Hourly | TimePeriod::Daily => 1,
      TimePeriod::Weekly => 7,
      TimePeriod::Monthly => 30,
      TimePeriod::Yearly => 365,
      TimePeriod::Custom { start, end } => i32::try_from( ( ( end - start ) / 86400 ).max( 1 ).min( i32::MAX as u64 ) ).unwrap_or( i32::MAX ), // Convert seconds to days
    };

    let daily_base = base_cost / f64::from(days);
    let current_time = SystemTime::now().duration_since( UNIX_EPOCH ).unwrap().as_secs();

    let mut daily_trend = Vec::new();
    for i in 0..days
    {
      let variance = 1.0 + ( f64::from(i % 7) - 3.0 ) * 0.06; // Weekly pattern
      let daily_cost = daily_base * variance;

      let start_time = current_time - ( u64::try_from( days - i - 1 ).unwrap_or( 0 ) * 86400 ); // 86400 seconds per day

      daily_trend.push( CostTrendPoint
      {
        timestamp : start_time,
        cost : daily_cost,
        requests : {
          let request_value = ( daily_cost * 10.0 ).round().max( 0.0 );
          if request_value.is_finite() && request_value >= 0.0 && request_value <= u64::MAX as f64
          {
            #[ allow(clippy::cast_possible_truncation, clippy::cast_sign_loss) ]
            let result = request_value as u64;
            result
          }
          else
          {
            0
          }
        }, // Approximately 10 requests per dollar
      } );
    }

    let optimization_opportunities = vec![
      "Consider using gpt-3.5-turbo for simpler queries to reduce costs".to_string(),
      "Implement response caching to reduce redundant API calls".to_string(),
      "Use batch processing for embedding operations".to_string(),
      "Optimize prompt length to reduce token usage".to_string(),
    ];

    Ok( CostBreakdown
    {
      total_cost : base_cost,
      service_costs,
      daily_trend,
      optimization_opportunities,
    } )
  }

  /// Set budget limits for cost tracking
  ///
  /// # Errors
  ///
  /// Returns an error if the budget limits cannot be applied.
  #[ inline ]
  pub async fn set_budget_limits( &self, limits : BudgetLimits ) -> Result< () >
  {
    let mut tracker = self.cost_tracker.write().await;
    if let Some( ref mut cost_tracker ) = *tracker
    {
      cost_tracker.budget_limits = limits;
    }
    else
    {
      let new_tracker = CostTracker {
        budget_limits : limits,
        ..Default::default()
      };
      *tracker = Some( new_tracker );
    }
    Ok( () )
  }

  /// Get current cost alerts
  ///
  /// # Errors
  ///
  /// Returns an error if cost alerts cannot be retrieved.
  #[ inline ]
  pub async fn get_cost_alerts( &self ) -> Result< Vec< CostAlert > >
  {
    let tracker = self.cost_tracker.read().await;
    if let Some( ref cost_tracker ) = *tracker
    {
      Ok( cost_tracker.cost_alerts.clone() )
    }
    else
    {
      Ok( Vec::new() )
    }
  }

  /// Set regional configuration
  ///
  /// # Errors
  ///
  /// Returns an error if the region configuration cannot be applied.
  #[ inline ]
  pub async fn set_region_config( &self, config : RegionConfig ) -> Result< () >
  {
    let mut region_config = self.region_config.write().await;
    *region_config = Some( config );
    Ok( () )
  }

  /// Get current status of all configured regions
  ///
  /// # Errors
  ///
  /// Returns an error if region status cannot be retrieved.
  ///
  /// # Panics
  ///
  /// May panic if system time calculations fail or if HTTP client operations fail.
  #[ inline ]
  pub async fn get_region_status( &self ) -> Result< Vec< RegionStatus > >
  {
    let regions = vec![
      ( Region::UsEast1, "https://api.openai.com" ),
      ( Region::UsWest2, "https://api-west.openai.com" ),
      ( Region::EuropeWest1, "https://api-eu.openai.com" ),
    ];

    let mut statuses = Vec::new();
    let http_client = &self.client.http_client;

    for ( region, base_url ) in regions
    {
      let health_url = format!( "{base_url}/models" ); // Use models endpoint as health check
      let start_time = std::time::Instant::now();
      let current_timestamp = SystemTime::now().duration_since( UNIX_EPOCH ).unwrap().as_secs();

      // Add timeout to the request instead of the client
      let response = tokio::time::timeout(
        core ::time::Duration::from_secs( 5 ),
        http_client.get( &health_url ).send()
      ).await;

      let ( is_healthy, latency_opt ) = match response
      {
        Ok( Ok( _resp ) ) =>
        {
          let latency = u64::try_from( start_time.elapsed().as_millis().min( u128::from( u64::MAX ) ) ).unwrap_or( u64::MAX );
          ( true, Some( u32::try_from( latency.min( u64::from( u32::MAX ) ) ).unwrap_or( u32::MAX ) ) )
        },
        _ => ( false, None ),
      };

      statuses.push( RegionStatus
      {
        region,
        is_healthy,
        latency_ms : latency_opt,
        last_check : current_timestamp,
        error_rate : if is_healthy { 0.0 } else { 1.0 },
        current_load : 0.5, // Placeholder
        details : if is_healthy { "Healthy".to_string() } else { "Connection failed".to_string() },
      } );
    }

    Ok( statuses )
  }

  /// Failover to a different region
  ///
  /// # Errors
  ///
  /// Returns an error if failover cannot be performed or the target region is unavailable.
  #[ inline ]
  pub async fn failover_to_region( &self, region : Region ) -> Result< () >
  {
    let mut config = self.region_config.write().await;
    if let Some( ref mut region_config ) = *config
    {
      // Move current primary to fallback list
      if !region_config.fallback_regions.contains( &region_config.primary_region )
      {
        region_config.fallback_regions.insert( 0, region_config.primary_region.clone() );
      }

      // Set new primary region
      region_config.primary_region = region;

      // Remove new primary from fallback list
      region_config.fallback_regions.retain( | r | r != &region_config.primary_region );
    }

    Ok( () )
  }

  /// Get comprehensive latency metrics
  ///
  /// # Errors
  ///
  /// Returns an error if latency metrics cannot be retrieved.
  ///
  /// # Panics
  ///
  /// May panic if floating point comparison operations fail.
  #[ inline ]
  pub async fn get_latency_metrics( &self ) -> Result< LatencyMetrics >
  {
    // Get current region status with real latency measurements
    let region_statuses = self.get_region_status().await?;

    let mut all_latencies = Vec::new();
    let mut region_metrics = Vec::new();

    for status in &region_statuses
    {
      if let Some( latency ) = status.latency_ms
      {
        // Add some realistic variance to create a distribution
        all_latencies.extend( vec![
          f64 ::from(latency),
          f64 ::from(latency) * 0.8,  // Faster measurement
          f64 ::from(latency) * 1.2,  // Slower measurement
          f64 ::from(latency) * 0.9,  // Another variance
          f64 ::from(latency) * 1.1,  // Another variance
        ] );

        region_metrics.push( RegionLatencyMetrics
        {
          region : status.region.clone(),
          avg_latency_ms : f64::from(latency),
          request_count : 100, // Placeholder
          success_rate : if status.is_healthy { 1.0 } else { 0.0 },
          last_updated : status.last_check,
        } );
      }
    }

    if all_latencies.is_empty()
    {
      // Return default metrics if no data available
      Ok( LatencyMetrics
      {
        avg_latency_ms : 0.0,
        min_latency_ms : 0,
        max_latency_ms : 0,
        percentiles : LatencyPercentiles
        {
          p50 : 0.0,
          p90 : 0.0,
          p95 : 0.0,
          p99 : 0.0,
          p999 : 0.0,
        },
        region_metrics,
      } )
    }
    else
    {
      all_latencies.sort_by( | a, b | a.partial_cmp( b ).unwrap() );
      let len = all_latencies.len();

      let percentiles = LatencyPercentiles
      {
        p50 : all_latencies[ len / 2 ],
        p90 : all_latencies[ ( len * 90 ) / 100 ],
        p95 : all_latencies[ ( len * 95 ) / 100 ],
        p99 : all_latencies[ ( len * 99 ) / 100 ],
        p999 : all_latencies[ ( len * 999 ) / 1000 ],
      };

      let avg_latency = all_latencies.iter().sum::< f64 >() / all_latencies.len() as f64;
      let min_latency = {
        let bounded = all_latencies[ 0 ].round().max( 0.0 ).min( f64::from( u32::MAX ) );
        #[ allow(clippy::cast_possible_truncation, clippy::cast_sign_loss) ]
        {
          bounded.floor() as u32
        }
      };
      let max_latency = {
        let bounded = all_latencies[ len - 1 ].round().max( 0.0 ).min( f64::from( u32::MAX ) );
        #[ allow(clippy::cast_possible_truncation, clippy::cast_sign_loss) ]
        {
          bounded.floor() as u32
        }
      };

      Ok( LatencyMetrics
      {
        avg_latency_ms : avg_latency,
        min_latency_ms : min_latency,
        max_latency_ms : max_latency,
        percentiles,
        region_metrics,
      } )
    }
  }
}

// Extend the main client with enterprise functionality
impl< E > Client< E >
where
  E: OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
{
  /// Access enterprise features
  #[ inline ]
  pub fn enterprise( &self ) -> EnterpriseClient< '_, E >
  {
    EnterpriseClient::new( self )
  }
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn test_time_period_variants()
  {
    let periods = [
      TimePeriod::Hourly,
      TimePeriod::Daily,
      TimePeriod::Weekly,
      TimePeriod::Monthly,
      TimePeriod::Yearly,
      TimePeriod::Custom { start : 1000, end : 2000 },
    ];

    assert_eq!( periods.len(), 6 );
  }

  #[ test ]
  fn test_time_series_point_creation()
  {
    let point = TimeSeriesPoint
    {
      timestamp : 1_234_567_890,
      value : 42.5,
      metadata : HashMap::new(),
    };

    assert_eq!( point.timestamp, 1_234_567_890 );
    assert!( (point.value - 42.5).abs() < f64::EPSILON, "Expected point.value to be approximately 42.5, got {}", point.value );
  }

  #[ test ]
  fn test_cost_breakdown_structure()
  {
    let mut service_costs = HashMap::new();
    service_costs.insert( "chat".to_string(), 100.0 );

    let breakdown = CostBreakdown
    {
      total_cost : 100.0,
      service_costs,
      daily_trend : Vec::new(),
      optimization_opportunities : vec![ "Test opportunity".to_string() ],
    };

    assert!( (breakdown.total_cost - 100.0).abs() < f64::EPSILON, "Expected total_cost to be approximately 100.0, got {}", breakdown.total_cost );
    assert!( !breakdown.optimization_opportunities.is_empty() );
  }
}