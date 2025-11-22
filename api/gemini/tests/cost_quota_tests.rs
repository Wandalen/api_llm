//! Cost-Based Enterprise Quota Management Tests
//!
//! Comprehensive tests for cost tracking, quota enforcement, and usage monitoring.

#[ cfg( feature = "enterprise_quota" ) ]
mod cost_quota_tests
{
  use api_gemini::
  {
    CostQuotaManager,
    CostQuotaConfig,
    CostQuotaExceededError,
    ModelPricing,
    CostUsageMetrics,
  };

  #[ test ]
  fn test_usage_metrics_creation()
  {
    let metrics = CostUsageMetrics::new();
    assert_eq!( metrics.request_count, 0 );
    assert_eq!( metrics.input_tokens, 0 );
    assert_eq!( metrics.output_tokens, 0 );
    assert_eq!( metrics.total_cost, 0.0 );
  }

  #[ test ]
  fn test_usage_metrics_record_request()
  {
    let mut metrics = CostUsageMetrics::new();
    metrics.record_request( 1_000, 500, 0.05 );

    assert_eq!( metrics.request_count, 1 );
    assert_eq!( metrics.input_tokens, 1_000 );
    assert_eq!( metrics.output_tokens, 500 );
    assert_eq!( metrics.total_cost, 0.05 );
    assert_eq!( metrics.total_tokens(), 1_500 );
  }

  #[ test ]
  fn test_usage_metrics_multiple_requests()
  {
    let mut metrics = CostUsageMetrics::new();
    metrics.record_request( 1_000, 500, 0.05 );
    metrics.record_request( 2_000, 1_000, 0.10 );

    assert_eq!( metrics.request_count, 2 );
    assert_eq!( metrics.input_tokens, 3_000 );
    assert_eq!( metrics.output_tokens, 1_500 );
  assert!( ( metrics.total_cost - 0.15 ).abs() < 0.0001, "Expected cost 0.15, got {}", metrics.total_cost );
  }

  #[ test ]
  fn test_quota_config_defaults()
  {
    let config = CostQuotaConfig::new();
    assert!( config.daily_request_limit.is_none() );
    assert!( config.daily_token_limit.is_none() );
    assert!( config.daily_cost_limit.is_none() );
    assert!( config.monthly_request_limit.is_none() );
    assert!( config.monthly_token_limit.is_none() );
    assert!( config.monthly_cost_limit.is_none() );
  }

  #[ test ]
  fn test_quota_config_builder()
  {
    let config = CostQuotaConfig::new()
    .with_daily_requests( 1000 )
    .with_daily_tokens( 1_000_000 )
    .with_daily_cost( 10.0 )
    .with_monthly_requests( 30_000 )
    .with_monthly_tokens( 30_000_000 )
    .with_monthly_cost( 300.0 );

    assert_eq!( config.daily_request_limit, Some( 1000 ) );
    assert_eq!( config.daily_token_limit, Some( 1_000_000 ) );
    assert_eq!( config.daily_cost_limit, Some( 10.0 ) );
    assert_eq!( config.monthly_request_limit, Some( 30_000 ) );
    assert_eq!( config.monthly_token_limit, Some( 30_000_000 ) );
    assert_eq!( config.monthly_cost_limit, Some( 300.0 ) );
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
  fn test_model_pricing_experimental()
  {
    let pricing = ModelPricing::for_model( "gemini-exp-1206" );
    assert_eq!( pricing.input_cost_per_million, 0.0 );
    assert_eq!( pricing.output_cost_per_million, 0.0 );
  }

  #[ test ]
  fn test_model_pricing_unknown_defaults_to_flash()
  {
    let pricing = ModelPricing::for_model( "unknown-model" );
    assert_eq!( pricing.input_cost_per_million, 0.075 );
    assert_eq!( pricing.output_cost_per_million, 0.30 );
  }

  #[ test ]
  fn test_cost_calculation_flash()
  {
    let pricing = ModelPricing::for_model( "gemini-1.5-flash" );
    let cost = pricing.calculate_cost( 1_000_000, 500_000 );
    // Expected : (1M/1M)*0.075 + (500k/1M)*0.30 = 0.075 + 0.15 = 0.225
    assert!( ( cost - 0.225 ).abs() < 0.0001 );
  }

  #[ test ]
  fn test_cost_calculation_pro()
  {
    let pricing = ModelPricing::for_model( "gemini-1.5-pro" );
    let cost = pricing.calculate_cost( 1_000_000, 500_000 );
    // Expected : (1M/1M)*1.25 + (500k/1M)*5.0 = 1.25 + 2.5 = 3.75
    assert!( ( cost - 3.75 ).abs() < 0.0001 );
  }

  #[ test ]
  fn test_quota_manager_creation()
  {
    let config = CostQuotaConfig::new();
    let manager = CostQuotaManager::new( config );

    let daily = manager.daily_usage();
    assert_eq!( daily.request_count, 0 );
    assert_eq!( daily.total_cost, 0.0 );
  }

  #[ test ]
  fn test_quota_manager_record_usage()
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
  fn test_daily_request_limit_enforcement()
  {
    let config = CostQuotaConfig::new().with_daily_requests( 2 );
    let manager = CostQuotaManager::new( config );

    // First two requests should succeed
    assert!( manager.record_usage( "gemini-1.5-flash", 100, 50 ).is_ok() );
    assert!( manager.record_usage( "gemini-1.5-flash", 100, 50 ).is_ok() );

    // Third request should fail
    let result = manager.record_usage( "gemini-1.5-flash", 100, 50 );
    assert!( result.is_err() );
    assert!( result.unwrap_err().message.contains( "Daily request limit" ) );
  }

  #[ test ]
  fn test_daily_token_limit_enforcement()
  {
    let config = CostQuotaConfig::new().with_daily_tokens( 2_000 );
    let manager = CostQuotaManager::new( config );

    // Request within limit should succeed
    assert!( manager.record_usage( "gemini-1.5-flash", 1_000, 500 ).is_ok() );

    // Request that would exceed limit should fail
    let result = manager.record_usage( "gemini-1.5-flash", 1_000, 500 );
    assert!( result.is_err() );
    assert!( result.unwrap_err().message.contains( "Daily token limit" ) );
  }

  #[ test ]
  fn test_daily_cost_limit_enforcement()
  {
    let config = CostQuotaConfig::new().with_daily_cost( 0.01 );
    let manager = CostQuotaManager::new( config );

    // Large request that exceeds cost limit should fail
    let result = manager.record_usage( "gemini-1.5-flash", 100_000, 50_000 );
    assert!( result.is_err() );
    assert!( result.unwrap_err().message.contains( "Daily cost limit" ) );
  }

  #[ test ]
  fn test_monthly_request_limit_enforcement()
  {
    let config = CostQuotaConfig::new().with_monthly_requests( 1 );
    let manager = CostQuotaManager::new( config );

    // First request should succeed
    assert!( manager.record_usage( "gemini-1.5-flash", 100, 50 ).is_ok() );

    // Second request should fail
    let result = manager.record_usage( "gemini-1.5-flash", 100, 50 );
    assert!( result.is_err() );
    assert!( result.unwrap_err().message.contains( "Monthly request limit" ) );
  }

  #[ test ]
  fn test_per_model_usage_tracking()
  {
    let config = CostQuotaConfig::new();
    let manager = CostQuotaManager::new( config );

    // Record usage for different models
    manager.record_usage( "gemini-1.5-pro", 1_000, 500 ).unwrap();
    manager.record_usage( "gemini-1.5-flash", 2_000, 1_000 ).unwrap();
    manager.record_usage( "gemini-1.5-pro", 500, 250 ).unwrap();

    // Check per-model tracking
    let pro_usage = manager.model_usage( "gemini-1.5-pro" ).unwrap();
    assert_eq!( pro_usage.request_count, 2 );
    assert_eq!( pro_usage.input_tokens, 1_500 );

    let flash_usage = manager.model_usage( "gemini-1.5-flash" ).unwrap();
    assert_eq!( flash_usage.request_count, 1 );
    assert_eq!( flash_usage.input_tokens, 2_000 );
  }

  #[ test ]
  fn test_all_model_usage()
  {
    let config = CostQuotaConfig::new();
    let manager = CostQuotaManager::new( config );

    manager.record_usage( "gemini-1.5-pro", 1_000, 500 ).unwrap();
    manager.record_usage( "gemini-1.5-flash", 2_000, 1_000 ).unwrap();

    let all_usage = manager.all_model_usage();
    assert_eq!( all_usage.len(), 2 );
    assert!( all_usage.contains_key( "gemini-1.5-pro" ) );
    assert!( all_usage.contains_key( "gemini-1.5-flash" ) );
  }

  #[ test ]
  fn test_daily_and_monthly_tracking()
  {
    let config = CostQuotaConfig::new();
    let manager = CostQuotaManager::new( config );

    manager.record_usage( "gemini-1.5-flash", 1_000, 500 ).unwrap();

    let daily = manager.daily_usage();
    let monthly = manager.monthly_usage();

    // Both should have the same usage
    assert_eq!( daily.request_count, 1 );
    assert_eq!( monthly.request_count, 1 );
    assert_eq!( daily.input_tokens, monthly.input_tokens );
  }

  #[ test ]
  fn test_reset_daily_metrics()
  {
    let config = CostQuotaConfig::new();
    let mut manager = CostQuotaManager::new( config );

    manager.record_usage( "gemini-1.5-flash", 1_000, 500 ).unwrap();
    assert_eq!( manager.daily_usage().request_count, 1 );

    manager.reset_daily();
    assert_eq!( manager.daily_usage().request_count, 0 );

    // Monthly should still have data
    assert_eq!( manager.monthly_usage().request_count, 1 );
  }

  #[ test ]
  fn test_reset_monthly_metrics()
  {
    let config = CostQuotaConfig::new();
    let mut manager = CostQuotaManager::new( config );

    manager.record_usage( "gemini-1.5-flash", 1_000, 500 ).unwrap();
    assert_eq!( manager.monthly_usage().request_count, 1 );

    manager.reset_monthly();
    assert_eq!( manager.monthly_usage().request_count, 0 );

    // Daily should still have data
    assert_eq!( manager.daily_usage().request_count, 1 );
  }

  #[ test ]
  fn test_export_json()
  {
    let config = CostQuotaConfig::new();
    let manager = CostQuotaManager::new( config );

    manager.record_usage( "gemini-1.5-flash", 1_000, 500 ).unwrap();

    let json = manager.export_json().unwrap();
    assert!( json.contains( "daily" ) );
    assert!( json.contains( "monthly" ) );
    assert!( json.contains( "per_model" ) );
    assert!( json.contains( "gemini-1.5-flash" ) );
  }

  #[ test ]
  fn test_quota_error_display()
  {
    let error = CostQuotaExceededError
    {
      message: "Daily cost limit of $10.00 exceeded".to_string(),
    };

  let display = format!( "{}", error );
    assert!( display.contains( "Cost quota exceeded" ) );
    assert!( display.contains( "Daily cost limit" ) );
  }

  #[ test ]
  fn test_usage_metrics_default_trait()
  {
    let metrics = CostUsageMetrics::default();
    assert_eq!( metrics.request_count, 0 );
    assert_eq!( metrics.total_cost, 0.0 );
  }

  #[ test ]
  fn test_quota_config_default_trait()
  {
    let config = CostQuotaConfig::default();
    assert!( config.daily_request_limit.is_none() );
    assert!( config.daily_cost_limit.is_none() );
  }
}

// Compilation test removed - if this module compiles, the test suite passes
// Empty tests that only verify compilation are unnecessary and violate
// "Loud Failures" principle (they silently pass without testing anything)
