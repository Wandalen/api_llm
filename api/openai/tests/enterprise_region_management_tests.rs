//! Region Management Module Tests
//!
//! Tests for multi-region deployment, failover, and latency optimization functionality.

use api_openai::enterprise::
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

#[ tokio::test ]
async fn test_region_variants()
{
  let regions = vec![
    Region::UsEast1,
    Region::UsWest2,
    Region::EuropeWest1,
    Region::AsiaPacificSoutheast1,
    Region::Custom( "https://custom.openai.com".to_string() ),
  ];

  assert_eq!( regions.len(), 5 );

  // Test serialization/deserialization
  for region in regions
  {
    let json = serde_json::to_string( &region ).expect( "Serialization should work" );
    let deserialized : Region = serde_json::from_str( &json ).expect( "Deserialization should work" );
    assert_eq!( region, deserialized );
  }
}

#[ tokio::test ]
async fn test_region_base_urls()
{
  assert_eq!( Region::UsEast1.base_url(), "https://api.openai.com" );
  assert_eq!( Region::UsWest2.base_url(), "https://api-west.openai.com" );
  assert_eq!( Region::EuropeWest1.base_url(), "https://api-eu.openai.com" );
  assert_eq!( Region::AsiaPacificSoutheast1.base_url(), "https://api-asia.openai.com" );
  assert_eq!( Region::Custom( "https://custom.com".to_string() ).base_url(), "https://custom.com" );
}

#[ tokio::test ]
async fn test_region_display_names()
{
  assert_eq!( Region::UsEast1.display_name(), "US East 1" );
  assert_eq!( Region::UsWest2.display_name(), "US West 2" );
  assert_eq!( Region::EuropeWest1.display_name(), "Europe West 1" );
  assert_eq!( Region::AsiaPacificSoutheast1.display_name(), "Asia Pacific Southeast 1" );
  assert_eq!( Region::Custom( "test".to_string() ).display_name(), "Custom Region" );
}

#[ tokio::test ]
async fn test_gdpr_compliance()
{
  assert!( !Region::UsEast1.supports_gdpr() );
  assert!( !Region::UsWest2.supports_gdpr() );
  assert!( Region::EuropeWest1.supports_gdpr() );
  assert!( !Region::AsiaPacificSoutheast1.supports_gdpr() );
  assert!( !Region::Custom( "test".to_string() ).supports_gdpr() );
}

#[ tokio::test ]
async fn test_latency_zones()
{
  assert_eq!( Region::UsEast1.latency_zone(), "North America" );
  assert_eq!( Region::UsWest2.latency_zone(), "North America" );
  assert_eq!( Region::EuropeWest1.latency_zone(), "Europe" );
  assert_eq!( Region::AsiaPacificSoutheast1.latency_zone(), "Asia Pacific" );
  assert_eq!( Region::Custom( "test".to_string() ).latency_zone(), "Custom" );
}

#[ tokio::test ]
async fn test_region_config_defaults()
{
  let config = RegionConfig::default();

  assert_eq!( config.primary_region, Region::UsEast1 );
  assert_eq!( config.fallback_regions.len(), 2 );
  assert!( config.fallback_regions.contains( &Region::UsWest2 ) );
  assert!( config.fallback_regions.contains( &Region::EuropeWest1 ) );
  assert!( config.enable_automatic_failover );
}

#[ tokio::test ]
async fn test_region_config_builder()
{
  let config = RegionConfig::with_primary_region( Region::EuropeWest1 )
    .add_fallback_region( Region::UsEast1 )
    .add_fallback_region( Region::UsWest2 )
    .with_max_latency( 2000 );

  assert_eq!( config.primary_region, Region::EuropeWest1 );
  assert!( config.fallback_regions.contains( &Region::UsEast1 ) );
  assert!( config.fallback_regions.contains( &Region::UsWest2 ) );
  assert_eq!( config.latency_preferences.max_latency_ms, 2000 );
}

#[ tokio::test ]
async fn test_gdpr_compliance_configuration()
{
  let config = RegionConfig::default().with_gdpr_compliance();

  assert!( config.compliance_requirements.gdpr_required );
  assert_eq!( config.primary_region, Region::EuropeWest1 ); // Should switch to GDPR-compliant region

  // Should only keep GDPR-compliant regions in fallback list
  for region in &config.fallback_regions
  {
    assert!( region.supports_gdpr() );
  }
}

#[ tokio::test ]
async fn test_data_residency_requirements()
{
  let allowed_regions = vec![ Region::UsEast1, Region::UsWest2 ];
  let config = RegionConfig::default().with_data_residency( allowed_regions.clone() );

  assert_eq!( config.compliance_requirements.data_residency_regions, allowed_regions );

  let compliant_regions = config.get_compliant_regions();
  for region in compliant_regions
  {
    assert!( allowed_regions.contains( &region ) );
  }
}

#[ tokio::test ]
#[ allow( clippy::float_cmp ) ]
async fn test_latency_preferences()
{
  let preferences = LatencyPreferences::default();

  assert_eq!( preferences.max_latency_ms, 5000 );
  assert_eq!( preferences.preferred_latency_ms, 1000 );
  assert!( preferences.enable_latency_routing );
  assert_eq!( preferences.latency_weight, 0.7 );
}

#[ tokio::test ]
async fn test_health_check_config()
{
  let config = HealthCheckConfig::default();

  assert_eq!( config.interval_seconds, 30 );
  assert_eq!( config.timeout_seconds, 10 );
  assert_eq!( config.failure_threshold, 3 );
  assert_eq!( config.success_threshold, 2 );
  assert!( config.enable_detailed_metrics );
}

#[ tokio::test ]
#[ allow( clippy::float_cmp ) ]
async fn test_region_status_creation()
{
  let healthy_status = RegionStatus::healthy( Region::UsEast1, 150 );

  assert_eq!( healthy_status.region, Region::UsEast1 );
  assert!( healthy_status.is_healthy );
  assert_eq!( healthy_status.latency_ms, Some( 150 ) );
  assert_eq!( healthy_status.error_rate, 0.0 );
  assert_eq!( healthy_status.current_load, 0.5 );
  assert_eq!( healthy_status.details, "Healthy" );

  let unhealthy_status = RegionStatus::unhealthy( Region::UsWest2, "Connection timeout".to_string() );

  assert_eq!( unhealthy_status.region, Region::UsWest2 );
  assert!( !unhealthy_status.is_healthy );
  assert_eq!( unhealthy_status.latency_ms, None );
  assert_eq!( unhealthy_status.error_rate, 1.0 );
  assert_eq!( unhealthy_status.current_load, 0.0 );
  assert_eq!( unhealthy_status.details, "Connection timeout" );
}

#[ tokio::test ]
async fn test_region_selection_algorithm()
{
  let config = RegionConfig::default();
  let statuses = vec![
    RegionStatus::healthy( Region::UsEast1, 100 ),     // Low latency, should be preferred
    RegionStatus::healthy( Region::UsWest2, 200 ),     // Higher latency
    RegionStatus::unhealthy( Region::EuropeWest1, "Network issues".to_string() ), // Unhealthy, should be excluded
  ];

  let selected = config.select_optimal_region( &statuses );
  assert_eq!( selected, Some( Region::UsEast1 ) ); // Should select lowest latency healthy region
}

#[ tokio::test ]
async fn test_region_selection_with_compliance()
{
  let config = RegionConfig::default()
    .with_gdpr_compliance();

  let statuses = vec![
    RegionStatus::healthy( Region::UsEast1, 50 ),      // Fastest but not GDPR-compliant
    RegionStatus::healthy( Region::EuropeWest1, 150 ), // GDPR-compliant
    RegionStatus::healthy( Region::UsWest2, 100 ),     // Fast but not GDPR-compliant
  ];

  let selected = config.select_optimal_region( &statuses );
  assert_eq!( selected, Some( Region::EuropeWest1 ) ); // Should select GDPR-compliant region
}

#[ tokio::test ]
async fn test_region_score_calculation()
{
  let config = RegionConfig::default();

  // Create status with good metrics
  let _good_status = RegionStatus
  {
    region : Region::UsEast1,
    is_healthy : true,
    latency_ms : Some( 100 ), // Within preferred latency
    last_check : 1_234_567_890,
    error_rate : 0.01,        // Low error rate
    current_load : 0.3,       // Low load
    details : "Good".to_string(),
  };

  // Create status with poor metrics
  let _poor_status = RegionStatus
  {
    region : Region::UsWest2,
    is_healthy : true,
    latency_ms : Some( 3000 ), // High latency
    last_check : 1_234_567_890,
    error_rate : 0.15,         // High error rate
    current_load : 0.9,        // High load
    details : "Poor".to_string(),
  };

  // TODO: These tests require making calculate_region_score public or adding public accessors
  // For now, just test that the config was created successfully
  assert_eq!( config.primary_region.to_string(), "us-east-1" );
}

#[ tokio::test ]
#[ allow( clippy::float_cmp ) ]
async fn test_latency_metrics_structure()
{
  let percentiles = LatencyPercentiles
  {
    p50 : 100.0,
    p90 : 200.0,
    p95 : 300.0,
    p99 : 500.0,
    p999 : 800.0,
  };

  let region_metrics = vec![
    RegionLatencyMetrics
    {
      region : Region::UsEast1,
      avg_latency_ms : 120.0,
      request_count : 1000,
      success_rate : 0.99,
      last_updated : 1_234_567_890,
    },
    RegionLatencyMetrics
    {
      region : Region::EuropeWest1,
      avg_latency_ms : 180.0,
      request_count : 500,
      success_rate : 0.98,
      last_updated : 1_234_567_890,
    },
  ];

  let metrics = LatencyMetrics
  {
    avg_latency_ms : 140.0,
    min_latency_ms : 80,
    max_latency_ms : 300,
    percentiles,
    region_metrics,
  };

  assert_eq!( metrics.avg_latency_ms, 140.0 );
  assert_eq!( metrics.min_latency_ms, 80 );
  assert_eq!( metrics.max_latency_ms, 300 );
  assert_eq!( metrics.percentiles.p99, 500.0 );
  assert_eq!( metrics.region_metrics.len(), 2 );
}

#[ tokio::test ]
async fn test_compliance_requirements_default()
{
  let requirements = ComplianceRequirements::default();

  assert!( requirements.data_residency_regions.is_empty() );
  assert!( !requirements.gdpr_required );
  assert!( !requirements.hipaa_required );
  assert!( !requirements.soc2_required );
  assert!( requirements.additional_standards.is_empty() );
}

#[ tokio::test ]
async fn test_region_config_serialization()
{
  let config = RegionConfig::default();

  let json = serde_json::to_string( &config ).expect( "Serialization should work" );
  let deserialized : RegionConfig = serde_json::from_str( &json ).expect( "Deserialization should work" );

  assert_eq!( config.primary_region, deserialized.primary_region );
  assert_eq!( config.fallback_regions, deserialized.fallback_regions );
  assert_eq!( config.enable_automatic_failover, deserialized.enable_automatic_failover );
}