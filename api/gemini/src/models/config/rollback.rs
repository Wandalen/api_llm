//! Configuration rollback analysis and safety checks
//!
//! This module provides tools for analyzing the impact of configuration rollbacks
//! and ensuring safe configuration changes.

use super::DynamicConfig;
use core::time::Duration;

/// Rollback analysis result for preview and safety checks
#[ derive( Debug, Clone ) ]
pub struct RollbackAnalysis
{
  /// Target configuration that would be restored
  pub target_config : DynamicConfig,
  /// List of fields that would change
  pub changed_fields : Vec< String >,
  /// Potential impact assessment
  pub impact_level : RollbackImpact,
  /// Safety warnings if any
  pub warnings : Vec< String >,
  /// Estimated rollback time
  pub estimated_duration : Duration,
  /// Whether the rollback is considered safe
  pub is_safe : bool,
}

/// Assessment of rollback impact level
#[ derive( Debug, Clone, PartialEq, Eq, PartialOrd, Ord ) ]
pub enum RollbackImpact
{
  /// Low impact - only minor settings changed
  Low,
  /// Medium impact - significant configuration changes
  Medium,
  /// High impact - major changes that could affect connectivity or functionality
  High,
  /// Critical impact - changes that could cause service disruption
  Critical,
}

impl RollbackAnalysis
{
  /// Analyze the impact of rolling back to a specific configuration
  pub fn analyze_rollback( current_config : &DynamicConfig, target_config : &DynamicConfig ) -> Self
  {
    let mut changed_fields = Vec::new();
    let mut warnings = Vec::new();
    let mut impact_level = RollbackImpact::Low;

    // Check URL changes (high impact)
    if current_config.base_url != target_config.base_url
    {
      changed_fields.push( "base_url".to_string() );
      warnings.push( "Base URL change may affect connectivity".to_string() );
      impact_level = RollbackImpact::High;
    }

    // Check timeout changes (medium impact)
    if current_config.timeout != target_config.timeout
    {
      changed_fields.push( "timeout".to_string() );
      if target_config.timeout < Duration::from_secs( 5 )
      {
        warnings.push( "Very short timeout may cause request failures".to_string() );
        impact_level = std::cmp::max( impact_level, RollbackImpact::Medium );
      }
    }

    // Check retry changes (medium impact)
    if current_config.retry_attempts != target_config.retry_attempts
    {
      changed_fields.push( "retry_attempts".to_string() );
      if target_config.retry_attempts == 0
      {
        warnings.push( "Zero retry attempts may reduce reliability".to_string() );
        impact_level = std::cmp::max( impact_level, RollbackImpact::Medium );
      }
    }

    // Check other fields (low impact)
    if current_config.enable_jitter != target_config.enable_jitter
    {
      changed_fields.push( "enable_jitter".to_string() );
    }
    if current_config.max_retry_delay != target_config.max_retry_delay
    {
      changed_fields.push( "max_retry_delay".to_string() );
    }
    if current_config.base_retry_delay != target_config.base_retry_delay
    {
      changed_fields.push( "base_retry_delay".to_string() );
    }
    if current_config.backoff_multiplier != target_config.backoff_multiplier
    {
      changed_fields.push( "backoff_multiplier".to_string() );
    }
    if current_config.source_priority != target_config.source_priority
    {
      changed_fields.push( "source_priority".to_string() );
    }

    // Check tag changes
    if current_config.tags != target_config.tags
    {
      changed_fields.push( "tags".to_string() );
    }

    // Determine safety
    let is_safe = impact_level != RollbackImpact::Critical && warnings.len() < 3;

    // Estimate duration based on complexity
    let estimated_duration = match impact_level
    {
      RollbackImpact::Low => Duration::from_millis( 100 ),
      RollbackImpact::Medium => Duration::from_millis( 500 ),
      RollbackImpact::High => Duration::from_secs( 1 ),
      RollbackImpact::Critical => Duration::from_secs( 3 ),
    };

    Self {
      target_config : target_config.clone(),
      changed_fields,
      impact_level,
      warnings,
      estimated_duration,
      is_safe,
    }
  }
}
