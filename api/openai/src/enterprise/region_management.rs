//! Region Management Module
//!
//! This module handles multi-region deployment, failover, and latency optimization
//! for enterprise `OpenAI` API usage.

use serde::{ Deserialize, Serialize };

/// Available `OpenAI` API regions
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash ) ]
pub enum Region
{
  /// US East 1 (Primary)
  UsEast1,
  /// US West 2
  UsWest2,
  /// Europe West 1
  EuropeWest1,
  /// Asia Pacific Southeast 1
  AsiaPacificSoutheast1,
  /// Custom region with endpoint URL
  Custom( String ),
}

impl core::fmt::Display for Region
{
  #[ inline ]
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Region::UsEast1 => write!( f, "us-east-1" ),
      Region::UsWest2 => write!( f, "us-west-2" ),
      Region::EuropeWest1 => write!( f, "europe-west-1" ),
      Region::AsiaPacificSoutheast1 => write!( f, "asia-pacific-southeast-1" ),
      Region::Custom( url ) => write!( f, "custom:{url}" ),
    }
  }
}

/// Regional configuration and preferences
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct RegionConfig
{
  /// Primary region for requests
  pub primary_region : Region,
  /// Fallback regions in order of preference
  pub fallback_regions : Vec< Region >,
  /// Latency preferences
  pub latency_preferences : LatencyPreferences,
  /// Compliance requirements
  pub compliance_requirements : ComplianceRequirements,
  /// Enable automatic failover
  pub enable_automatic_failover : bool,
  /// Health check configuration
  pub health_check_config : HealthCheckConfig,
}

/// Latency optimization preferences
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct LatencyPreferences
{
  /// Maximum acceptable latency in milliseconds
  pub max_latency_ms : u32,
  /// Preferred latency in milliseconds
  pub preferred_latency_ms : u32,
  /// Enable latency-based routing
  pub enable_latency_routing : bool,
  /// Weight factor for latency vs other factors (0.0-1.0)
  pub latency_weight : f64,
}

/// Data compliance and regulatory requirements
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct ComplianceRequirements
{
  /// Require data residency in specific regions
  pub data_residency_regions : Vec< Region >,
  /// GDPR compliance required
  pub gdpr_required : bool,
  /// HIPAA compliance required
  pub hipaa_required : bool,
  /// SOC 2 compliance required
  pub soc2_required : bool,
  /// Additional compliance standards
  pub additional_standards : Vec< String >,
}

/// Health check configuration for regions
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct HealthCheckConfig
{
  /// Health check interval in seconds
  pub interval_seconds : u32,
  /// Timeout for health checks in seconds
  pub timeout_seconds : u32,
  /// Number of consecutive failures before marking unhealthy
  pub failure_threshold : u32,
  /// Number of consecutive successes before marking healthy
  pub success_threshold : u32,
  /// Enable detailed health metrics
  pub enable_detailed_metrics : bool,
}

/// Current status of a region
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct RegionStatus
{
  /// Region identifier
  pub region : Region,
  /// Whether region is currently healthy
  pub is_healthy : bool,
  /// Current latency in milliseconds
  pub latency_ms : Option< u32 >,
  /// Last health check timestamp
  pub last_check : u64,
  /// Error rate (0.0-1.0)
  pub error_rate : f64,
  /// Current load (0.0-1.0)
  pub current_load : f64,
  /// Additional status details
  pub details : String,
}

/// Comprehensive latency metrics across regions
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct LatencyMetrics
{
  /// Overall average latency in milliseconds
  pub avg_latency_ms : f64,
  /// Minimum recorded latency
  pub min_latency_ms : u32,
  /// Maximum recorded latency
  pub max_latency_ms : u32,
  /// Latency percentiles
  pub percentiles : LatencyPercentiles,
  /// Per-region latency breakdown
  pub region_metrics : Vec< RegionLatencyMetrics >,
}

/// Latency percentile measurements
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct LatencyPercentiles
{
  /// 50th percentile (median)
  pub p50 : f64,
  /// 90th percentile
  pub p90 : f64,
  /// 95th percentile
  pub p95 : f64,
  /// 99th percentile
  pub p99 : f64,
  /// 99.9th percentile
  pub p999 : f64,
}

/// Latency metrics for a specific region
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct RegionLatencyMetrics
{
  /// Region identifier
  pub region : Region,
  /// Average latency for this region
  pub avg_latency_ms : f64,
  /// Request count for this region
  pub request_count : u64,
  /// Success rate for this region
  pub success_rate : f64,
  /// Last updated timestamp
  pub last_updated : u64,
}

impl Default for LatencyPreferences
{
  #[ inline ]
  fn default() -> Self
  {
    Self
    {
      max_latency_ms : 5000,    // 5 seconds max
      preferred_latency_ms : 1000, // 1 second preferred
      enable_latency_routing : true,
      latency_weight : 0.7,     // 70% weight on latency
    }
  }
}

impl Default for ComplianceRequirements
{
  #[ inline ]
  fn default() -> Self
  {
    Self
    {
      data_residency_regions : Vec::new(),
      gdpr_required : false,
      hipaa_required : false,
      soc2_required : false,
      additional_standards : Vec::new(),
    }
  }
}

impl Default for HealthCheckConfig
{
  #[ inline ]
  fn default() -> Self
  {
    Self
    {
      interval_seconds : 30,
      timeout_seconds : 10,
      failure_threshold : 3,
      success_threshold : 2,
      enable_detailed_metrics : true,
    }
  }
}

impl Default for RegionConfig
{
  #[ inline ]
  fn default() -> Self
  {
    Self
    {
      primary_region : Region::UsEast1,
      fallback_regions : vec![ Region::UsWest2, Region::EuropeWest1 ],
      latency_preferences : LatencyPreferences::default(),
      compliance_requirements : ComplianceRequirements::default(),
      enable_automatic_failover : true,
      health_check_config : HealthCheckConfig::default(),
    }
  }
}

impl Region
{
  /// Get the base URL for this region
  #[ must_use ]
  #[ inline ]
  pub fn base_url( &self ) -> &str
  {
    match self
    {
      Region::UsEast1 => "https://api.openai.com",
      Region::UsWest2 => "https://api-west.openai.com",
      Region::EuropeWest1 => "https://api-eu.openai.com",
      Region::AsiaPacificSoutheast1 => "https://api-asia.openai.com",
      Region::Custom( url ) => url,
    }
  }

  /// Get the display name for this region
  #[ must_use ]
  #[ inline ]
  pub fn display_name( &self ) -> &str
  {
    match self
    {
      Region::UsEast1 => "US East 1",
      Region::UsWest2 => "US West 2",
      Region::EuropeWest1 => "Europe West 1",
      Region::AsiaPacificSoutheast1 => "Asia Pacific Southeast 1",
      Region::Custom( _ ) => "Custom Region",
    }
  }

  /// Check if region supports GDPR compliance
  #[ must_use ]
  #[ inline ]
  pub fn supports_gdpr( &self ) -> bool
  {
    matches!( self, Region::EuropeWest1 )
  }

  /// Get expected latency zone for region (for routing optimization)
  #[ must_use ]
  #[ inline ]
  pub fn latency_zone( &self ) -> &str
  {
    match self
    {
      Region::UsEast1 | Region::UsWest2 => "North America",
      Region::EuropeWest1 => "Europe",
      Region::AsiaPacificSoutheast1 => "Asia Pacific",
      Region::Custom( _ ) => "Custom",
    }
  }
}

impl RegionConfig
{
  /// Create new region config with primary region
  #[ must_use ]
  #[ inline ]
  pub fn with_primary_region( primary_region : Region ) -> Self
  {
    Self
    {
      primary_region,
      ..Self::default()
    }
  }

  /// Add fallback region
  #[ must_use ]
  #[ inline ]
  pub fn add_fallback_region( mut self, region : Region ) -> Self
  {
    if !self.fallback_regions.contains( &region ) && region != self.primary_region
    {
      self.fallback_regions.push( region );
    }
    self
  }

  /// Set maximum acceptable latency
  #[ must_use ]
  #[ inline ]
  pub fn with_max_latency( mut self, max_latency_ms : u32 ) -> Self
  {
    self.latency_preferences.max_latency_ms = max_latency_ms;
    self
  }

  /// Enable GDPR compliance
  #[ must_use ]
  #[ inline ]
  pub fn with_gdpr_compliance( mut self ) -> Self
  {
    self.compliance_requirements.gdpr_required = true;
    // Ensure only GDPR-compliant regions are used
    self.fallback_regions.retain( Region::supports_gdpr );
    if !self.primary_region.supports_gdpr()
    {
      self.primary_region = Region::EuropeWest1;
    }
    self
  }

  /// Add data residency requirement
  #[ must_use ]
  #[ inline ]
  pub fn with_data_residency( mut self, regions : Vec< Region > ) -> Self
  {
    self.compliance_requirements.data_residency_regions = regions;
    self
  }

  /// Get all usable regions based on compliance requirements
  #[ must_use ]
  #[ inline ]
  pub fn get_compliant_regions( &self ) -> Vec< Region >
  {
    let mut regions = vec![ self.primary_region.clone() ];
    regions.extend( self.fallback_regions.clone() );

    // Filter by data residency requirements
    if !self.compliance_requirements.data_residency_regions.is_empty()
    {
      regions.retain( | region |
        self.compliance_requirements.data_residency_regions.contains( region )
      );
    }

    // Filter by GDPR requirements
    if self.compliance_requirements.gdpr_required
    {
      regions.retain( Region::supports_gdpr );
    }

    regions
  }

  /// Select best region based on current metrics
  #[ must_use ]
  #[ inline ]
  pub fn select_optimal_region( &self, region_statuses : &[ RegionStatus ] ) -> Option< Region >
  {
    let compliant_regions = self.get_compliant_regions();
    let mut candidate_regions : Vec< _ > = region_statuses.iter()
      .filter( | status | compliant_regions.contains( &status.region ) && status.is_healthy )
      .collect();

    if candidate_regions.is_empty()
    {
      return None;
    }

    // Sort by scoring algorithm
    candidate_regions.sort_by( | a, b |
    {
      let score_a = self.calculate_region_score( a );
      let score_b = self.calculate_region_score( b );
      score_b.partial_cmp( &score_a ).unwrap_or( core::cmp::Ordering::Equal )
    } );

    Some( candidate_regions[ 0 ].region.clone() )
  }

  /// Calculate score for region selection (higher is better)
  fn calculate_region_score( &self, status : &RegionStatus ) -> f64
  {
    let mut score = 0.0;

    // Latency component (higher score for lower latency)
    if let Some( latency ) = status.latency_ms
    {
      let latency_score = if latency <= self.latency_preferences.preferred_latency_ms
      {
        1.0
      }
      else if latency <= self.latency_preferences.max_latency_ms
      {
        1.0 - f64::from( latency - self.latency_preferences.preferred_latency_ms )
          / f64::from( self.latency_preferences.max_latency_ms - self.latency_preferences.preferred_latency_ms )
      }
      else
      {
        0.0
      };
      score += latency_score * self.latency_preferences.latency_weight;
    }

    // Error rate component (higher score for lower error rate)
    let error_score = 1.0 - status.error_rate;
    score += error_score * 0.2;

    // Load component (higher score for lower load)
    let load_score = 1.0 - status.current_load;
    score += load_score * 0.1;

    score
  }
}

impl RegionStatus
{
  /// Create healthy region status
  ///
  /// # Panics
  /// Panics if the system time is before the Unix epoch.
  #[ must_use ]
  #[ inline ]
  pub fn healthy( region : Region, latency_ms : u32 ) -> Self
  {
    Self
    {
      region,
      is_healthy : true,
      latency_ms : Some( latency_ms ),
      last_check : std::time::SystemTime::now()
        .duration_since( std::time::UNIX_EPOCH )
        .unwrap()
        .as_secs(),
      error_rate : 0.0,
      current_load : 0.5, // Moderate load
      details : "Healthy".to_string(),
    }
  }

  /// Create unhealthy region status
  ///
  /// # Panics
  /// Panics if the system time is before the Unix epoch.
  #[ must_use ]
  #[ inline ]
  pub fn unhealthy( region : Region, reason : String ) -> Self
  {
    Self
    {
      region,
      is_healthy : false,
      latency_ms : None,
      last_check : std::time::SystemTime::now()
        .duration_since( std::time::UNIX_EPOCH )
        .unwrap()
        .as_secs(),
      error_rate : 1.0,
      current_load : 0.0,
      details : reason,
    }
  }
}

#[ cfg( test ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn test_region_base_urls()
  {
    assert_eq!( Region::UsEast1.base_url(), "https://api.openai.com" );
    assert_eq!( Region::EuropeWest1.base_url(), "https://api-eu.openai.com" );
    assert_eq!( Region::Custom( "https://custom.com".to_string() ).base_url(), "https://custom.com" );
  }

  #[ test ]
  fn test_region_gdpr_support()
  {
    assert!( !Region::UsEast1.supports_gdpr() );
    assert!( Region::EuropeWest1.supports_gdpr() );
  }

  #[ test ]
  fn test_region_config_with_gdpr()
  {
    let config = RegionConfig::default().with_gdpr_compliance();
    assert!( config.compliance_requirements.gdpr_required );
    assert_eq!( config.primary_region, Region::EuropeWest1 );
  }

  #[ test ]
  fn test_compliant_regions_filtering()
  {
    let config = RegionConfig::default()
      .with_data_residency( vec![ Region::UsEast1, Region::UsWest2 ] );

    let compliant = config.get_compliant_regions();
    assert!( compliant.contains( &Region::UsEast1 ) );
    assert!( !compliant.contains( &Region::EuropeWest1 ) );
  }

  #[ test ]
  fn test_region_selection()
  {
    let config = RegionConfig::default();
    let statuses = vec![
      RegionStatus::healthy( Region::UsEast1, 100 ),
      RegionStatus::healthy( Region::UsWest2, 200 ),
      RegionStatus::unhealthy( Region::EuropeWest1, "Network issues".to_string() ),
    ];

    let selected = config.select_optimal_region( &statuses );
    assert_eq!( selected, Some( Region::UsEast1 ) ); // Should select lowest latency
  }

  #[ test ]
  fn test_region_config_builder()
  {
    let config = RegionConfig::with_primary_region( Region::EuropeWest1 )
      .add_fallback_region( Region::UsEast1 )
      .with_max_latency( 2000 );

    assert_eq!( config.primary_region, Region::EuropeWest1 );
    assert!( config.fallback_regions.contains( &Region::UsEast1 ) );
    assert_eq!( config.latency_preferences.max_latency_ms, 2000 );
  }
}