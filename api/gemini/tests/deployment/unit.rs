use super::*;

#[ test ]
fn test_deployment_state_enum()
{
  assert_eq!( DeploymentState::Pending, DeploymentState::Pending );
  assert_ne!( DeploymentState::Active, DeploymentState::Failed );
  assert_ne!( DeploymentState::RollingBack, DeploymentState::Terminated );
}

#[ test ]
fn test_scaling_config_builder() -> Result< (), Box< dyn std::error::Error > >
{
  let config = ScalingConfig::builder()
  .min_instances( 1 )
  .max_instances( 5 )
  .target_cpu_utilization( 80.0 )
  .target_memory_utilization( 75.0 )
  .scale_up_cooldown( Duration::from_secs( 300 ) )
  .scale_down_cooldown( Duration::from_secs( 600 ) )
  .build()?;

  assert_eq!( config.min_instances, 1 );
  assert_eq!( config.max_instances, 5 );
  assert_eq!( config.target_cpu_utilization, 80.0 );
  assert_eq!( config.target_memory_utilization, 75.0 );
  assert_eq!( config.scale_up_cooldown, Duration::from_secs( 300 ) );
  assert_eq!( config.scale_down_cooldown, Duration::from_secs( 600 ) );

  Ok( () )
}

#[ test ]
fn test_scaling_config_validation()
{
  // Invalid min instances (zero)
  let result = ScalingConfig::builder()
  .min_instances( 0 )
  .build();
  match result
  {
    Err( Error::ConfigurationError( msg ) ) => {
      assert!( msg.contains( "Minimum instances must be greater than 0" ) );
    println!( "✓ Zero minimum instances properly rejected : {}", msg );
    },
    _ => panic!( "Zero minimum instances should be rejected" ),
  }

  // Invalid max instances (less than min)
  let result = ScalingConfig::builder()
  .min_instances( 5 )
  .max_instances( 3 )
  .build();
  match result
  {
    Err( Error::ConfigurationError( msg ) ) => {
      assert!( msg.contains( "Maximum instances must be greater than or equal to minimum instances" ) );
    println!( "✓ Invalid min/max instance relationship rejected : {}", msg );
    },
    _ => panic!( "Invalid min/max instances should be rejected" ),
  }

  // Invalid CPU utilization (over 100%)
  let result = ScalingConfig::builder()
  .target_cpu_utilization( 150.0 )
  .build();
  match result
  {
    Err( Error::ConfigurationError( msg ) ) => {
      assert!( msg.contains( "Target CPU utilization must be between 0 and 100" ) );
    println!( "✓ Invalid CPU utilization rejected : {}", msg );
    },
    _ => panic!( "Invalid CPU utilization should be rejected" ),
  }

  // Invalid memory utilization (negative)
  let result = ScalingConfig::builder()
  .target_memory_utilization( -10.0 )
  .build();
  match result
  {
    Err( Error::ConfigurationError( msg ) ) => {
      assert!( msg.contains( "Target memory utilization must be between 0 and 100" ) );
    println!( "✓ Invalid memory utilization rejected : {}", msg );
    },
    _ => panic!( "Invalid memory utilization should be rejected" ),
  }
}

#[ test ]
fn test_resource_config_builder() -> Result< (), Box< dyn std::error::Error > >
{
  let config = ResourceConfig::builder()
  .cpu_cores( 2.0 )
  .memory_gb( 8.0 )
  .build()?;

  assert_eq!( config.cpu_cores, 2.0 );
  assert_eq!( config.memory_gb, 8.0 );

  Ok( () )
}

#[ test ]
fn test_deployment_strategy_types()
{
  let rolling_strategy = DeploymentStrategy::Rolling {
    max_unavailable_percentage: 25.0,
    max_surge_percentage: 25.0,
  };

  let blue_green_strategy = DeploymentStrategy::BlueGreen {
    switch_traffic_percentage: 100.0,
    rollback_on_failure: true,
  };

  let canary_strategy = DeploymentStrategy::Canary {
    traffic_percentage: 5.0,
    promotion_criteria: vec![ "success_rate > 99%".to_string() ],
  };

  match rolling_strategy
  {
  DeploymentStrategy::Rolling { max_unavailable_percentage, max_surge_percentage } => {
      assert_eq!( max_unavailable_percentage, 25.0 );
      assert_eq!( max_surge_percentage, 25.0 );
    },
    _ => panic!( "Expected rolling strategy" ),
  }

  match blue_green_strategy
  {
  DeploymentStrategy::BlueGreen { switch_traffic_percentage, rollback_on_failure } => {
      assert_eq!( switch_traffic_percentage, 100.0 );
      assert!( rollback_on_failure );
    },
    _ => panic!( "Expected blue-green strategy" ),
  }

  match canary_strategy
  {
  DeploymentStrategy::Canary { traffic_percentage, promotion_criteria } => {
      assert_eq!( traffic_percentage, 5.0 );
      assert_eq!( promotion_criteria, vec![ "success_rate > 99%" ] );
    },
    _ => panic!( "Expected canary strategy" ),
  }
}

#[ test ]
fn test_deployment_metrics()
{
  let metrics = DeploymentMetrics::default();

  // Test that default values are reasonable
  assert_eq!( metrics.response_time_us.load(std::sync::atomic::Ordering::Relaxed), 0 );
  assert_eq!( metrics.total_requests.load(std::sync::atomic::Ordering::Relaxed), 0 );
  assert_eq!( metrics.total_errors.load(std::sync::atomic::Ordering::Relaxed), 0 );
  assert!( metrics.last_updated_us.load(std::sync::atomic::Ordering::Relaxed) > 0, "Timestamp should be initialized to current time" );
}

#[ test ]
fn test_orchestration_config_types()
{
  let k8s_config = OrchestrationConfig::Kubernetes {
    namespace: "default".to_string(),
    cluster: "main".to_string(),
    service_account: "default".to_string(),
  };

  let docker_config = OrchestrationConfig::Docker {
    network: "bridge".to_string(),
    volumes: vec![ "/data:/app/data".to_string() ],
  };

  match k8s_config
  {
  OrchestrationConfig::Kubernetes { namespace, cluster, service_account } => {
      assert_eq!( namespace, "default" );
      assert_eq!( cluster, "main" );
      assert_eq!( service_account, "default" );
    },
    _ => panic!( "Expected Kubernetes config" ),
  }

  match docker_config
  {
  OrchestrationConfig::Docker { network, volumes } => {
      assert_eq!( network, "bridge" );
      assert_eq!( volumes, vec![ "/data:/app/data" ] );
    },
    _ => panic!( "Expected Docker config" ),
  }
}

/// Test resource configuration validation edge cases
#[ test ]
fn test_resource_config_validation()
{
  // Invalid CPU cores (zero)
  let result = ResourceConfig::builder()
  .cpu_cores( 0.0 )
  .build();
  match result
  {
    Err( Error::ConfigurationError( msg ) ) => {
      assert!( msg.contains( "CPU cores must be greater than 0" ) );
    println!( "✓ Invalid CPU cores rejected : {}", msg );
    },
    _ => panic!( "Invalid CPU cores should be rejected" ),
  }

  // Invalid memory (negative)
  let result = ResourceConfig::builder()
  .memory_gb( -1.0 )
  .build();
  match result
  {
    Err( Error::ConfigurationError( msg ) ) => {
      assert!( msg.contains( "Memory must be greater than 0" ) );
    println!( "✓ Invalid memory rejected : {}", msg );
    },
    _ => panic!( "Invalid memory should be rejected" ),
  }
}

/// Test health check configuration validation
#[ test ]
fn test_health_check_config_validation()
{
  // Valid configuration
  let config = DeploymentHealthCheckConfig::builder()
  .endpoint( "/api/health" )
  .interval( Duration::from_secs( 15 ) )
  .timeout( Duration::from_secs( 3 ) )
  .failure_threshold( 5 )
  .success_threshold( 2 )
  .build()
  .expect( "Valid health check config should build" );

  assert_eq!( config.endpoint, "/api/health" );
  assert_eq!( config.interval, Duration::from_secs( 15 ) );
  assert_eq!( config.timeout, Duration::from_secs( 3 ) );
  println!( "✓ Valid health check configuration built successfully" );

  // Empty endpoint should fail
  let result = DeploymentHealthCheckConfig::builder()
  .endpoint( "" )
  .build();
  match result
  {
    Err( Error::ConfigurationError( msg ) ) => {
      assert!( msg.contains( "Health check endpoint cannot be empty" ) );
    println!( "✓ Empty endpoint rejected : {}", msg );
    },
    _ => panic!( "Empty endpoint should be rejected" ),
  }
}

/// Test container configuration validation
#[ test ]
fn test_container_config_validation()
{
  // Valid configuration
  let env_vars = vec![
  ( "NODE_ENV".to_string(), "production".to_string() ),
  ( "PORT".to_string(), "8080".to_string() ),
  ];

  let config = ContainerConfig::builder()
  .image( "my-app:v1.2.3" )
  .port( 8080 )
  .environment_variables( env_vars.clone() )
  .volumes( vec![ "/data:/app/data".to_string() ] )
  .command( vec![ "node".to_string(), "server.js".to_string() ] )
  .working_directory( "/app" )
  .build()
  .expect( "Valid container config should build" );

  assert_eq!( config.image, "my-app:v1.2.3" );
  assert_eq!( config.port, 8080 );
  assert_eq!( config.environment_variables, env_vars );
  println!( "✓ Valid container configuration built successfully" );

  // Empty image should fail
  let result = ContainerConfig::builder()
  .image( "" )
  .build();
  match result
  {
    Err( Error::ConfigurationError( msg ) ) => {
      assert!( msg.contains( "Container image cannot be empty" ) );
    println!( "✓ Empty container image rejected : {}", msg );
    },
    _ => panic!( "Empty container image should be rejected" ),
  }
}

/// Test monitoring configuration builder
#[ test ]
fn test_monitoring_config_builder()
{
  let mut labels = HashMap::new();
  labels.insert( "team".to_string(), "ml-platform".to_string() );
  labels.insert( "env".to_string(), "staging".to_string() );

  let config = MonitoringConfig::builder()
  .enable_metrics( true )
  .metrics_interval( Duration::from_secs( 30 ) )
  .enable_logging( true )
  .log_level( "DEBUG".to_string() )
  .alert_on_errors( true )
  .metric_labels( labels.clone() )
  .build()
  .expect( "Valid monitoring config should build" );

  assert!( config.enable_metrics );
  assert_eq!( config.metrics_interval, Duration::from_secs( 30 ) );
  assert!( config.enable_logging );
  assert_eq!( config.log_level, "DEBUG" );
  assert!( config.alert_on_errors );
  assert_eq!( config.metric_labels, labels );

  println!( "✓ Monitoring configuration built successfully" );
}

/// Test deployment state transitions and cloning
#[ test ]
fn test_deployment_state_comprehensive()
{
  let states = vec![
  DeploymentState::Pending,
  DeploymentState::Active,
  DeploymentState::Updating,
  DeploymentState::Scaling,
  DeploymentState::RollingBack,
  DeploymentState::Failed,
  DeploymentState::Terminated,
  ];

  for state in states
  {
    // Verify states can be cloned and compared
    let cloned_state = state.clone();
    assert_eq!( state, cloned_state );

    // Verify debug formatting works
  let debug_str = format!( "{:?}", state );
    assert!( !debug_str.is_empty() );
  }

  println!( "✓ All deployment states work correctly" );
}

/// Test deployment environment comprehensive coverage
#[ test ]
fn test_deployment_environment_comprehensive()
{
  let environments = vec![
  DeploymentEnvironment::Development,
  DeploymentEnvironment::Staging,
  DeploymentEnvironment::Production,
  DeploymentEnvironment::Custom( "test-env".to_string() ),
  DeploymentEnvironment::Custom( "integration".to_string() ),
  ];

  for env in environments
  {
    let cloned_env = env.clone();
    assert_eq!( env, cloned_env );

    // Test debug formatting
  let debug_str = format!( "{:?}", env );
    assert!( !debug_str.is_empty() );
  }

  println!( "✓ All deployment environments work correctly" );
}

/// Test deployment metrics defaults and structure
#[ test ]
fn test_deployment_metrics_comprehensive()
{
  let default_metrics = DeploymentMetrics::default();
  assert_eq!( default_metrics.response_time_us.load(std::sync::atomic::Ordering::Relaxed), 0 );
  assert_eq!( default_metrics.total_requests.load(std::sync::atomic::Ordering::Relaxed), 0 );
  assert_eq!( default_metrics.total_errors.load(std::sync::atomic::Ordering::Relaxed), 0 );
  assert!( default_metrics.last_updated_us.load(std::sync::atomic::Ordering::Relaxed) > 0, "Timestamp should be initialized to current time" );

  // Test that we can create a new instance
  let custom_metrics = DeploymentMetrics::default();

  // Verify metrics structure works with atomic access
  assert_eq!( custom_metrics.response_time_us.load(std::sync::atomic::Ordering::Relaxed), 0 );
  assert_eq!( custom_metrics.total_requests.load(std::sync::atomic::Ordering::Relaxed), 0 );
  assert_eq!( custom_metrics.total_errors.load(std::sync::atomic::Ordering::Relaxed), 0 );

  println!( "✓ Deployment metrics structure and defaults work correctly" );
}
}
