//! Cost Management Module
//!
//! This module handles cost tracking, budget management, and cost analytics
//! for enterprise `OpenAI` API usage.

use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

/// Cost tracking and budget management
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct CostTracker
{
  /// Current daily spend in USD
  pub daily_spend : f64,
  /// Current monthly spend in USD
  pub monthly_spend : f64,
  /// Budget limits configuration
  pub budget_limits : BudgetLimits,
  /// Active cost alerts
  pub cost_alerts : Vec< CostAlert >,
  /// Detailed usage breakdown
  pub usage_breakdown : UsageBreakdown,
}

/// Budget limits and alert thresholds
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct BudgetLimits
{
  /// Daily spending limit in USD
  pub daily_limit : Option< f64 >,
  /// Monthly spending limit in USD
  pub monthly_limit : Option< f64 >,
  /// Alert threshold as percentage of limit (0.0-1.0)
  pub alert_threshold : f64,
  /// Hard limit enforcement (stop requests when exceeded)
  pub enforce_hard_limit : bool,
  /// Cost optimization settings
  pub optimization_settings : CostOptimizationSettings,
}

/// Cost alert notification
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct CostAlert
{
  /// Alert type
  pub alert_type : AlertType,
  /// Alert severity level
  pub severity : AlertSeverity,
  /// Alert message
  pub message : String,
  /// Timestamp when alert was triggered
  pub timestamp : u64,
  /// Current spend amount that triggered alert
  pub current_spend : f64,
  /// Limit that was exceeded
  pub limit : f64,
}

/// Types of cost alerts
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub enum AlertType
{
  /// Daily limit approaching
  DailyLimitApproaching,
  /// Daily limit exceeded
  DailyLimitExceeded,
  /// Monthly limit approaching
  MonthlyLimitApproaching,
  /// Monthly limit exceeded
  MonthlyLimitExceeded,
}

/// Alert severity levels
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord ) ]
pub enum AlertSeverity
{
  /// Informational alert
  Info,
  /// Warning alert
  Warning,
  /// Critical alert requiring attention
  Critical,
}

/// Detailed usage and cost breakdown
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct UsageBreakdown
{
  /// Usage by time periods
  pub time_usage : Vec< TimeUsage >,
  /// Usage by token types
  pub token_usage : Vec< TokenUsage >,
  /// Usage by model types
  pub model_usage : HashMap<  String, f64  >,
  /// Usage by operation types
  pub operation_usage : HashMap<  String, f64  >,
}

/// Time-based usage tracking
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct TimeUsage
{
  /// Time period start (Unix timestamp)
  pub start_time : u64,
  /// Time period end (Unix timestamp)
  pub end_time : u64,
  /// Total requests in this period
  pub request_count : u64,
  /// Total cost in this period
  pub cost : f64,
}

/// Token usage tracking by type
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct TokenUsage
{
  /// Token type (input, output, etc.)
  pub token_type : String,
  /// Total tokens used
  pub count : u64,
  /// Cost per token
  pub cost_per_token : f64,
  /// Total cost for this token type
  pub total_cost : f64,
}

/// Cost optimization settings
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
#[ allow( clippy::struct_excessive_bools ) ]
pub struct CostOptimizationSettings
{
  /// Enable automatic cost optimization
  pub enabled : bool,
  /// Prefer cheaper models when possible
  pub prefer_cheaper_models : bool,
  /// Maximum acceptable latency increase for cost savings (in milliseconds)
  pub max_latency_increase_ms : u32,
  /// Enable request batching for cost efficiency
  pub enable_request_batching : bool,
  /// Enable response caching to reduce redundant requests
  pub enable_response_caching : bool,
}

impl Default for CostOptimizationSettings
{
  #[ inline ]
  fn default() -> Self
  {
    Self
    {
      enabled : false,
      prefer_cheaper_models : false,
      max_latency_increase_ms : 1000,
      enable_request_batching : true,
      enable_response_caching : true,
    }
  }
}

impl Default for BudgetLimits
{
  #[ inline ]
  fn default() -> Self
  {
    Self
    {
      daily_limit : None,
      monthly_limit : None,
      alert_threshold : 0.8, // Alert at 80% of limit
      enforce_hard_limit : false,
      optimization_settings : CostOptimizationSettings::default(),
    }
  }
}

impl Default for UsageBreakdown
{
  #[ inline ]
  fn default() -> Self
  {
    Self
    {
      time_usage : Vec::new(),
      token_usage : Vec::new(),
      model_usage : HashMap::new(),
      operation_usage : HashMap::new(),
    }
  }
}

impl Default for CostTracker
{
  #[ inline ]
  fn default() -> Self
  {
    Self
    {
      daily_spend : 0.0,
      monthly_spend : 0.0,
      budget_limits : BudgetLimits::default(),
      cost_alerts : Vec::new(),
      usage_breakdown : UsageBreakdown::default(),
    }
  }
}

impl CostTracker
{
  /// Create new cost tracker with default settings
  #[ must_use ]
  #[ inline ]
  pub fn new() -> Self
  {
    Self::default()
  }

  /// Create cost tracker with custom budget limits
  #[ must_use ]
  #[ inline ]
  pub fn with_limits( daily_limit : Option< f64 >, monthly_limit : Option< f64 > ) -> Self
  {
    Self
    {
      budget_limits : BudgetLimits
      {
        daily_limit,
        monthly_limit,
        ..BudgetLimits::default()
      },
      ..Self::default()
    }
  }

  /// Update spending and check for alerts
  ///
  /// # Panics
  ///
  /// Panics if the system time is before the Unix epoch when creating alert timestamps.
  #[ inline ]
  pub fn update_spending( &mut self, daily_delta : f64, monthly_delta : f64 ) -> Vec< CostAlert >
  {
    self.daily_spend += daily_delta;
    self.monthly_spend += monthly_delta;

    let mut new_alerts = Vec::new();

    // Check daily limits
    if let Some( daily_limit ) = self.budget_limits.daily_limit
    {
      let threshold = daily_limit * self.budget_limits.alert_threshold;

      if self.daily_spend >= daily_limit
      {
        new_alerts.push( CostAlert
        {
          alert_type : AlertType::DailyLimitExceeded,
          severity : AlertSeverity::Critical,
          message : format!( "Daily spending limit exceeded : ${:.2}", self.daily_spend ),
          timestamp : std::time::SystemTime::now()
            .duration_since( std::time::UNIX_EPOCH )
            .unwrap()
            .as_secs(),
          current_spend : self.daily_spend,
          limit : daily_limit,
        } );
      }
      else if self.daily_spend >= threshold
      {
        new_alerts.push( CostAlert
        {
          alert_type : AlertType::DailyLimitApproaching,
          severity : AlertSeverity::Warning,
          message : format!( "Daily spending approaching limit : ${:.2}", self.daily_spend ),
          timestamp : std::time::SystemTime::now()
            .duration_since( std::time::UNIX_EPOCH )
            .unwrap()
            .as_secs(),
          current_spend : self.daily_spend,
          limit : daily_limit,
        } );
      }
    }

    // Check monthly limits
    if let Some( monthly_limit ) = self.budget_limits.monthly_limit
    {
      let threshold = monthly_limit * self.budget_limits.alert_threshold;

      if self.monthly_spend >= monthly_limit
      {
        new_alerts.push( CostAlert
        {
          alert_type : AlertType::MonthlyLimitExceeded,
          severity : AlertSeverity::Critical,
          message : format!( "Monthly spending limit exceeded : ${:.2}", self.monthly_spend ),
          timestamp : std::time::SystemTime::now()
            .duration_since( std::time::UNIX_EPOCH )
            .unwrap()
            .as_secs(),
          current_spend : self.monthly_spend,
          limit : monthly_limit,
        } );
      }
      else if self.monthly_spend >= threshold
      {
        new_alerts.push( CostAlert
        {
          alert_type : AlertType::MonthlyLimitApproaching,
          severity : AlertSeverity::Warning,
          message : format!( "Monthly spending approaching limit : ${:.2}", self.monthly_spend ),
          timestamp : std::time::SystemTime::now()
            .duration_since( std::time::UNIX_EPOCH )
            .unwrap()
            .as_secs(),
          current_spend : self.monthly_spend,
          limit : monthly_limit,
        } );
      }
    }

    self.cost_alerts.extend( new_alerts.clone() );
    new_alerts
  }

  /// Reset daily spending (typically called at day boundary)
  #[ inline ]
  pub fn reset_daily_spending( &mut self )
  {
    self.daily_spend = 0.0;
    // Remove daily alerts
    self.cost_alerts.retain( | alert |
      !matches!( alert.alert_type, AlertType::DailyLimitApproaching | AlertType::DailyLimitExceeded )
    );
  }

  /// Reset monthly spending (typically called at month boundary)
  #[ inline ]
  pub fn reset_monthly_spending( &mut self )
  {
    self.monthly_spend = 0.0;
    // Remove monthly alerts
    self.cost_alerts.retain( | alert |
      !matches!( alert.alert_type, AlertType::MonthlyLimitApproaching | AlertType::MonthlyLimitExceeded )
    );
  }

  /// Get current cost efficiency ratio
  #[ must_use ]
  #[ inline ]
  pub fn get_cost_efficiency_ratio( &self ) -> f64
  {
    if self.usage_breakdown.time_usage.is_empty()
    {
      return 1.0;
    }

    let total_requests : u64 = self.usage_breakdown.time_usage.iter()
      .map( | usage | usage.request_count )
      .sum();

    if total_requests == 0
    {
      return 1.0;
    }

    // Calculate cost per request
    let cost_per_request = ( self.daily_spend + self.monthly_spend ) / total_requests as f64;

    // Industry baseline cost per request (example value)
    let baseline_cost_per_request = 0.01;

    baseline_cost_per_request / cost_per_request.max( 0.001 )
  }
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn test_cost_tracker_creation()
  {
    let tracker = CostTracker::new();
    assert!( tracker.daily_spend.abs() < f64::EPSILON, "Expected daily_spend to be approximately 0.0, got {}", tracker.daily_spend );
    assert!( tracker.monthly_spend.abs() < f64::EPSILON, "Expected monthly_spend to be approximately 0.0, got {}", tracker.monthly_spend );
    assert!( tracker.cost_alerts.is_empty() );
  }

  #[ test ]
  fn test_cost_tracker_with_limits()
  {
    let tracker = CostTracker::with_limits( Some( 100.0 ), Some( 1000.0 ) );
    assert_eq!( tracker.budget_limits.daily_limit, Some( 100.0 ) );
    assert_eq!( tracker.budget_limits.monthly_limit, Some( 1000.0 ) );
  }

  #[ test ]
  fn test_spending_alerts()
  {
    let mut tracker = CostTracker::with_limits( Some( 100.0 ), Some( 1000.0 ) );

    // Test warning alert - 85% of daily limit should trigger daily warning
    // but 85.0 on monthly limit of 1000.0 (8.5%) should not trigger monthly warning
    let alerts = tracker.update_spending( 85.0, 85.0 );
    assert_eq!( alerts.len(), 1 ); // Only daily warning
    assert!( alerts.iter().any( | a | matches!( a.alert_type, AlertType::DailyLimitApproaching ) ) );

    // Test both alerts by triggering monthly as well
    let mut tracker2 = CostTracker::with_limits( Some( 100.0 ), Some( 1000.0 ) );
    let alerts2 = tracker2.update_spending( 85.0, 850.0 ); // 85% of monthly limit too
    assert_eq!( alerts2.len(), 2 ); // Both daily and monthly warnings
    assert!( alerts2.iter().any( | a | matches!( a.alert_type, AlertType::DailyLimitApproaching ) ) );
    assert!( alerts2.iter().any( | a | matches!( a.alert_type, AlertType::MonthlyLimitApproaching ) ) );

    // Test critical alert
    let alerts = tracker.update_spending( 20.0, 20.0 );
    assert!( alerts.iter().any( | a | matches!( a.alert_type, AlertType::DailyLimitExceeded ) ) );
  }

  #[ test ]
  fn test_spending_reset()
  {
    let mut tracker = CostTracker::with_limits( Some( 100.0 ), Some( 1000.0 ) );
    tracker.update_spending( 150.0, 150.0 ); // Trigger alerts

    assert!( !tracker.cost_alerts.is_empty() );

    tracker.reset_daily_spending();
    assert!( tracker.daily_spend.abs() < f64::EPSILON, "Expected daily_spend to be approximately 0.0 after reset, got {}", tracker.daily_spend );

    // Daily alerts should be removed
    assert!( !tracker.cost_alerts.iter().any( | alert |
      matches!( alert.alert_type, AlertType::DailyLimitApproaching | AlertType::DailyLimitExceeded )
    ) );
  }

  #[ test ]
  fn test_cost_efficiency_ratio()
  {
    let mut tracker = CostTracker::new();
    tracker.daily_spend = 10.0;
    tracker.usage_breakdown.time_usage.push( TimeUsage
    {
      start_time : 0,
      end_time : 100,
      request_count : 100,
      cost : 10.0,
    } );

    let efficiency = tracker.get_cost_efficiency_ratio();
    assert!( efficiency > 0.0 );
  }
}