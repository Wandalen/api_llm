//! Cost-Based Enterprise Quota Management
//!
//! Client-side tracking and management of API usage costs for production deployments.
//!
//! # Features
//!
//! - Token usage tracking per request
//! - Cost calculation based on Gemini model pricing
//! - Configurable quota limits (daily/monthly/total)
//! - Usage metrics export (JSON format)
//! - Per-model cost tracking
//! - Thread-safe for concurrent access
//!
//! # Benefits
//!
//! - Cost control and budget management
//! - Usage visibility and reporting
//! - Proactive cost monitoring
//! - Department/team cost allocation

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{ Serialize, Deserialize };
use chrono::Utc;

/// Usage metrics for a specific time period
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct UsageMetrics
{
  /// Total number of API requests
  pub request_count : u64,
  /// Total input tokens consumed
  pub input_tokens : u64,
  /// Total output tokens consumed
  pub output_tokens : u64,
  /// Total cost in USD (calculated from tokens)
  pub total_cost : f64,
  /// Timestamp of first request in this period (Unix timestamp)
  pub period_start : i64,
  /// Timestamp of last request in this period (Unix timestamp)
  pub period_end : i64,
}

impl UsageMetrics
{
  /// Create new empty usage metrics
  #[ inline ]
  #[ must_use ]
  pub fn new() -> Self
  {
    let now = Utc::now().timestamp();
    Self
    {
      request_count : 0,
      input_tokens : 0,
      output_tokens : 0,
      total_cost : 0.0,
      period_start : now,
      period_end : now,
    }
  }

  /// Record a new request
  #[ inline ]
  pub fn record_request( &mut self, input_tokens : u64, output_tokens : u64, cost : f64 )
  {
    self.request_count += 1;
    self.input_tokens += input_tokens;
    self.output_tokens += output_tokens;
    self.total_cost += cost;
    self.period_end = Utc::now().timestamp();
  }

  /// Get total tokens (input + output)
  #[ inline ]
  #[ must_use ]
  pub fn total_tokens( &self ) -> u64
  {
    self.input_tokens + self.output_tokens
  }
}

impl Default for UsageMetrics
{
  #[ inline ]
  fn default() -> Self
  {
    Self::new()
  }
}

/// Quota limits configuration
#[ derive( Debug, Clone, PartialEq ) ]
pub struct CostQuotaConfig
{
  /// Maximum requests per day (None = unlimited)
  pub daily_request_limit : Option< u64 >,
  /// Maximum tokens per day (None = unlimited)
  pub daily_token_limit : Option< u64 >,
  /// Maximum cost per day in USD (None = unlimited)
  pub daily_cost_limit : Option< f64 >,
  /// Maximum requests per month (None = unlimited)
  pub monthly_request_limit : Option< u64 >,
  /// Maximum tokens per month (None = unlimited)
  pub monthly_token_limit : Option< u64 >,
  /// Maximum cost per month in USD (None = unlimited)
  pub monthly_cost_limit : Option< f64 >,
}

impl CostQuotaConfig
{
  /// Create new quota config with no limits
  #[ inline ]
  #[ must_use ]
  pub fn new() -> Self
  {
    Self
    {
      daily_request_limit : None,
      daily_token_limit : None,
      daily_cost_limit : None,
      monthly_request_limit : None,
      monthly_token_limit : None,
      monthly_cost_limit : None,
    }
  }

  /// Set daily request limit
  #[ inline ]
  #[ must_use ]
  pub fn with_daily_requests( mut self, limit : u64 ) -> Self
  {
    self.daily_request_limit = Some( limit );
    self
  }

  /// Set daily token limit
  #[ inline ]
  #[ must_use ]
  pub fn with_daily_tokens( mut self, limit : u64 ) -> Self
  {
    self.daily_token_limit = Some( limit );
    self
  }

  /// Set daily cost limit in USD
  #[ inline ]
  #[ must_use ]
  pub fn with_daily_cost( mut self, limit : f64 ) -> Self
  {
    self.daily_cost_limit = Some( limit );
    self
  }

  /// Set monthly request limit
  #[ inline ]
  #[ must_use ]
  pub fn with_monthly_requests( mut self, limit : u64 ) -> Self
  {
    self.monthly_request_limit = Some( limit );
    self
  }

  /// Set monthly token limit
  #[ inline ]
  #[ must_use ]
  pub fn with_monthly_tokens( mut self, limit : u64 ) -> Self
  {
    self.monthly_token_limit = Some( limit );
    self
  }

  /// Set monthly cost limit in USD
  #[ inline ]
  #[ must_use ]
  pub fn with_monthly_cost( mut self, limit : f64 ) -> Self
  {
    self.monthly_cost_limit = Some( limit );
    self
  }
}

impl Default for CostQuotaConfig
{
  #[ inline ]
  fn default() -> Self
  {
    Self::new()
  }
}

/// Cost calculator for Gemini models (cost per million tokens)
///
/// Pricing as of 2025 - check Google AI pricing page for current rates
#[ derive( Debug, Clone, Copy, PartialEq ) ]
pub struct ModelPricing
{
  /// Cost per million input tokens in USD
  pub input_cost_per_million : f64,
  /// Cost per million output tokens in USD
  pub output_cost_per_million : f64,
}

impl ModelPricing
{
  /// Get pricing for a Gemini model
  ///
  /// # Arguments
  ///
  /// * `model` - Model name (e.g., "gemini-1.5-pro", "gemini-1.5-flash")
  ///
  /// # Returns
  ///
  /// Pricing information for the model (defaults to Flash pricing if unknown)
  #[ inline ]
  #[ must_use ]
  pub fn for_model( model : &str ) -> Self
  {
    // Extract base model name (remove version suffix if present)
    let model_lower = model.to_lowercase();

    if model_lower.contains( "gemini-1.5-pro" ) || model_lower.contains( "gemini-pro" )
    {
      // Gemini 1.5 Pro pricing (prompts ≤128k tokens)
      Self
      {
        input_cost_per_million : 1.25,
        output_cost_per_million : 5.0,
      }
    }
    else if model_lower.contains( "gemini-1.5-flash" ) || model_lower.contains( "gemini-flash" )
    {
      // Gemini 1.5 Flash pricing (prompts ≤128k tokens)
      Self
      {
        input_cost_per_million : 0.075,
        output_cost_per_million : 0.30,
      }
    }
    else if model_lower.contains( "gemini-exp" ) || model_lower.contains( "experimental" )
    {
      // Experimental models - free tier (0 cost)
      Self
      {
        input_cost_per_million : 0.0,
        output_cost_per_million : 0.0,
      }
    }
    else
    {
      // Default to Flash pricing for unknown models
      Self
      {
        input_cost_per_million : 0.075,
        output_cost_per_million : 0.30,
      }
    }
  }

  /// Calculate cost for given token counts
  ///
  /// # Arguments
  ///
  /// * `input_tokens` - Number of input tokens
  /// * `output_tokens` - Number of output tokens
  ///
  /// # Returns
  ///
  /// Total cost in USD
  #[ inline ]
  #[ must_use ]
  pub fn calculate_cost( &self, input_tokens : u64, output_tokens : u64 ) -> f64
  {
    let input_cost = ( input_tokens as f64 / 1_000_000.0 ) * self.input_cost_per_million;
    let output_cost = ( output_tokens as f64 / 1_000_000.0 ) * self.output_cost_per_million;
    input_cost + output_cost
  }
}

/// Quota violation error
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct CostQuotaExceededError
{
  /// Description of which quota was exceeded
  pub message : String,
}

impl std::fmt::Display for CostQuotaExceededError
{
  #[ inline ]
  fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
  {
    write!( f, "Cost quota exceeded : {}", self.message )
  }
}

impl std::error::Error for CostQuotaExceededError
{
}

/// Cost-based quota manager for tracking and enforcing usage limits
#[ derive( Debug, Clone ) ]
pub struct CostQuotaManager
{
  config : CostQuotaConfig,
  daily_metrics : Arc< RwLock< UsageMetrics > >,
  monthly_metrics : Arc< RwLock< UsageMetrics > >,
  per_model_metrics : Arc< RwLock< HashMap< String, UsageMetrics > > >,
}

impl CostQuotaManager
{
  /// Create new cost quota manager with configuration
  #[ inline ]
  #[ must_use ]
  pub fn new( config : CostQuotaConfig ) -> Self
  {
    Self
    {
      config,
      daily_metrics : Arc::new( RwLock::new( UsageMetrics::new() ) ),
      monthly_metrics : Arc::new( RwLock::new( UsageMetrics::new() ) ),
      per_model_metrics : Arc::new( RwLock::new( HashMap::new() ) ),
    }
  }

  /// Record usage and check quotas
  ///
  /// # Arguments
  ///
  /// * `model` - Model name used for the request
  /// * `input_tokens` - Number of input tokens consumed
  /// * `output_tokens` - Number of output tokens consumed
  ///
  /// # Errors
  ///
  /// Returns `CostQuotaExceededError` if any quota limit is exceeded
  #[ inline ]
  pub fn record_usage
  (
    &self,
    model : &str,
    input_tokens : u64,
    output_tokens : u64,
  ) -> Result< (), CostQuotaExceededError >
  {
    let pricing = ModelPricing::for_model( model );
    let cost = pricing.calculate_cost( input_tokens, output_tokens );

    // Check daily quotas before recording
    {
      let daily = self.daily_metrics.read();
      if let Some( limit ) = self.config.daily_request_limit
      {
        if daily.request_count >= limit
        {
          return Err( CostQuotaExceededError
          {
            message : format!( "Daily request limit of {limit} exceeded" ),
          } );
        }
      }
      if let Some( limit ) = self.config.daily_token_limit
      {
        if daily.total_tokens() + input_tokens + output_tokens > limit
        {
          return Err( CostQuotaExceededError
          {
            message : format!( "Daily token limit of {limit} exceeded" ),
          } );
        }
      }
      if let Some( limit ) = self.config.daily_cost_limit
      {
        if daily.total_cost + cost > limit
        {
          return Err( CostQuotaExceededError
          {
            message : format!( "Daily cost limit of ${limit:.2} exceeded" ),
          } );
        }
      }
    }

    // Check monthly quotas
    {
      let monthly = self.monthly_metrics.read();
      if let Some( limit ) = self.config.monthly_request_limit
      {
        if monthly.request_count >= limit
        {
          return Err( CostQuotaExceededError
          {
            message : format!( "Monthly request limit of {limit} exceeded" ),
          } );
        }
      }
      if let Some( limit ) = self.config.monthly_token_limit
      {
        if monthly.total_tokens() + input_tokens + output_tokens > limit
        {
          return Err( CostQuotaExceededError
          {
            message : format!( "Monthly token limit of {limit} exceeded" ),
          } );
        }
      }
      if let Some( limit ) = self.config.monthly_cost_limit
      {
        if monthly.total_cost + cost > limit
        {
          return Err( CostQuotaExceededError
          {
            message : format!( "Monthly cost limit of ${limit:.2} exceeded" ),
          } );
        }
      }
    }

    // Record usage
    {
      let mut daily = self.daily_metrics.write();
      daily.record_request( input_tokens, output_tokens, cost );
    }
    {
      let mut monthly = self.monthly_metrics.write();
      monthly.record_request( input_tokens, output_tokens, cost );
    }
    {
      let mut per_model = self.per_model_metrics.write();
      per_model
        .entry( model.to_string() )
        .or_default()
        .record_request( input_tokens, output_tokens, cost );
    }

    Ok( () )
  }

  /// Get current daily usage metrics
  #[ inline ]
  #[ must_use ]
  pub fn daily_usage( &self ) -> UsageMetrics
  {
    self.daily_metrics.read().clone()
  }

  /// Get current monthly usage metrics
  #[ inline ]
  #[ must_use ]
  pub fn monthly_usage( &self ) -> UsageMetrics
  {
    self.monthly_metrics.read().clone()
  }

  /// Get usage metrics for a specific model
  #[ inline ]
  #[ must_use ]
  pub fn model_usage( &self, model : &str ) -> Option< UsageMetrics >
  {
    self.per_model_metrics.read().get( model ).cloned()
  }

  /// Get all per-model usage metrics
  #[ inline ]
  #[ must_use ]
  pub fn all_model_usage( &self ) -> HashMap< String, UsageMetrics >
  {
    self.per_model_metrics.read().clone()
  }

  /// Reset daily metrics (call this at start of each day)
  #[ inline ]
  pub fn reset_daily( &mut self )
  {
    *self.daily_metrics.write() = UsageMetrics::new();
  }

  /// Reset monthly metrics (call this at start of each month)
  #[ inline ]
  pub fn reset_monthly( &mut self )
  {
    *self.monthly_metrics.write() = UsageMetrics::new();
  }

  /// Export all metrics as JSON
  ///
  /// # Errors
  ///
  /// Returns an error if JSON serialization fails
  #[ inline ]
  pub fn export_json( &self ) -> Result< String, serde_json::Error >
  {
    let data = serde_json::json!
    ({
      "daily" : self.daily_usage(),
      "monthly" : self.monthly_usage(),
      "per_model" : self.all_model_usage(),
    });
    serde_json ::to_string_pretty( &data )
  }
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn test_usage_metrics_creation()
  {
    let metrics = UsageMetrics::new();
    assert_eq!( metrics.request_count, 0 );
    assert_eq!( metrics.input_tokens, 0 );
    assert_eq!( metrics.output_tokens, 0 );
    assert_eq!( metrics.total_cost, 0.0 );
  }

  #[ test ]
  fn test_usage_metrics_record()
  {
    let mut metrics = UsageMetrics::new();
    metrics.record_request( 100, 50, 0.01 );

    assert_eq!( metrics.request_count, 1 );
    assert_eq!( metrics.input_tokens, 100 );
    assert_eq!( metrics.output_tokens, 50 );
    assert_eq!( metrics.total_cost, 0.01 );
    assert_eq!( metrics.total_tokens(), 150 );
  }

  #[ test ]
  fn test_model_pricing_gemini_pro()
  {
    let pricing = ModelPricing::for_model( "gemini-1.5-pro" );
    assert_eq!( pricing.input_cost_per_million, 1.25 );
    assert_eq!( pricing.output_cost_per_million, 5.0 );
  }

  #[ test ]
  fn test_model_pricing_gemini_flash()
  {
    let pricing = ModelPricing::for_model( "gemini-1.5-flash" );
    assert_eq!( pricing.input_cost_per_million, 0.075 );
    assert_eq!( pricing.output_cost_per_million, 0.30 );
  }

  #[ test ]
  fn test_cost_calculation()
  {
    let pricing = ModelPricing::for_model( "gemini-1.5-flash" );
    let cost = pricing.calculate_cost( 1_000_000, 500_000 );
    // Expected : (1M/1M)*0.075 + (500k/1M)*0.30 = 0.075 + 0.15 = 0.225
    assert!( ( cost - 0.225 ).abs() < 0.0001 );
  }

  #[ test ]
  fn test_cost_quota_manager_creation()
  {
    let config = CostQuotaConfig::new();
    let manager = CostQuotaManager::new( config );

    let daily = manager.daily_usage();
    assert_eq!( daily.request_count, 0 );
    assert_eq!( daily.total_cost, 0.0 );
  }

  #[ test ]
  fn test_cost_quota_manager_record_usage()
  {
    let config = CostQuotaConfig::new();
    let manager = CostQuotaManager::new( config );

    let result = manager.record_usage( "gemini-1.5-flash", 1_000, 500 );
    assert!( result.is_ok() );

    let daily = manager.daily_usage();
    assert_eq!( daily.request_count, 1 );
    assert_eq!( daily.input_tokens, 1_000 );
    assert_eq!( daily.output_tokens, 500 );
  }

  #[ test ]
  fn test_daily_request_limit()
  {
    let config = CostQuotaConfig::new().with_daily_requests( 1 );
    let manager = CostQuotaManager::new( config );

    // First request should succeed
    assert!( manager.record_usage( "gemini-1.5-flash", 100, 50 ).is_ok() );

    // Second request should fail
    let result = manager.record_usage( "gemini-1.5-flash", 100, 50 );
    assert!( result.is_err() );
    assert!( result.unwrap_err().message.contains( "Daily request limit" ) );
  }

  #[ test ]
  fn test_daily_cost_limit()
  {
    let config = CostQuotaConfig::new().with_daily_cost( 0.01 );
    let manager = CostQuotaManager::new( config );

    // Large request that exceeds cost limit
    let result = manager.record_usage( "gemini-1.5-flash", 100_000, 50_000 );
    assert!( result.is_err() );
    assert!( result.unwrap_err().message.contains( "Daily cost limit" ) );
  }

  #[ test ]
  fn test_per_model_tracking()
  {
    let config = CostQuotaConfig::new();
    let manager = CostQuotaManager::new( config );

    manager.record_usage( "gemini-1.5-pro", 1_000, 500 ).unwrap();
    manager.record_usage( "gemini-1.5-flash", 2_000, 1_000 ).unwrap();

    let pro_usage = manager.model_usage( "gemini-1.5-pro" ).unwrap();
    assert_eq!( pro_usage.input_tokens, 1_000 );

    let flash_usage = manager.model_usage( "gemini-1.5-flash" ).unwrap();
    assert_eq!( flash_usage.input_tokens, 2_000 );
  }
}
