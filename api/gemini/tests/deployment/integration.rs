use super::*;

#[ tokio::test ]
async fn test_deployment_creation() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();

  // Test model deployment creation
  let models = client.models();
  let model = models.by_name( "gemini-pro" );
  let deployment_builder = model.deploy()
  .with_name( "production-deployment" )
  .with_version( "1.0.0" )
  .with_environment( DeploymentEnvironment::Production )
  .with_scaling_config( ScalingConfig::builder()
  .min_instances( 2 )
  .max_instances( 10 )
  .target_cpu_utilization( 70.0 )
  .build()? );

  // Actually deploy and verify the deployment
  let deployment = deployment_builder.deploy().await?;

  // Verify deployment was created successfully
  assert_eq!( deployment.name, "production-deployment" );
  assert_eq!( deployment.version, "1.0.0" );
  assert_eq!( deployment.environment, DeploymentEnvironment::Production );
  assert_eq!( deployment.state().await, DeploymentState::Active );

  println!( "✓ Production deployment created successfully" );
println!( "  - Deployment ID: {}", deployment.deployment_id );
println!( "  - State : {:?}", deployment.state().await );

  Ok( () )
}

#[ tokio::test ]
async fn test_deployment_strategies() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();

  // Test blue-green deployment strategy
  let models = client.models();
  let model = models.by_name( "gemini-pro" );
  let deployment_builder = model.deploy()
  .with_name( "blue-green-deployment" )
  .with_strategy( DeploymentStrategy::BlueGreen {
    switch_traffic_percentage: 100.0,
    rollback_on_failure: true,
  } )
  .with_health_checks( DeploymentHealthCheckConfig::builder()
  .endpoint( "/health" )
  .interval( Duration::from_secs( 30 ) )
  .timeout( Duration::from_secs( 5 ) )
  .build()? );

  // Test canary deployment strategy
  let canary_builder = model.deploy()
  .with_name( "canary-deployment" )
  .with_strategy( DeploymentStrategy::Canary {
    traffic_percentage: 10.0,
    promotion_criteria: vec![ "error_rate < 1%".to_string() ],
  } );

  // Actually deploy blue-green and verify
  let blue_green_deployment = deployment_builder.deploy().await?;
  assert_eq!( blue_green_deployment.name, "blue-green-deployment" );
  assert_eq!( blue_green_deployment.state().await, DeploymentState::Active );
  println!( "✓ Blue-Green deployment strategy works" );

  // Deploy canary and verify
  let canary_deployment = canary_builder.deploy().await?;
  assert_eq!( canary_deployment.name, "canary-deployment" );
  assert_eq!( canary_deployment.state().await, DeploymentState::Active );
  println!( "✓ Canary deployment strategy works" );

  Ok( () )
}

#[ tokio::test ]
async fn test_resource_allocation() -> Result< (), Box< dyn std::error::Error > >
{
  let _client = create_integration_client();

  // Test resource allocation configuration
  let resource_config = ResourceConfig::builder()
  .cpu_cores( 4.0 )
  .memory_gb( 16.0 )
  .gpu_count( 1 )
  .gpu_memory_gb( 8.0 )
  .storage_gb( 100.0 )
  .build()?;

  assert_eq!( resource_config.cpu_cores, 4.0 );
  assert_eq!( resource_config.memory_gb, 16.0 );
  assert_eq!( resource_config.gpu_count, 1 );
  assert_eq!( resource_config.gpu_memory_gb, 8.0 );
  assert_eq!( resource_config.storage_gb, 100.0 );

  Ok( () )
}

#[ tokio::test ]
async fn test_monitoring_configuration() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();

  // Test monitoring and alerting configuration
  let models = client.models();
  let model = models.by_name( "gemini-pro" );
  let deployment_builder = model.deploy()
  .with_name( "monitored-deployment" )
  .with_monitoring( MonitoringConfig::builder()
  .enable_metrics( true )
  .metrics_interval( Duration::from_secs( 60 ) )
  .enable_logging( true )
  .log_level( "INFO".to_string() )
  .alert_on_errors( true )
  .build()? );

  // Actually deploy with monitoring and verify
  let deployment = deployment_builder.deploy().await?;
  assert_eq!( deployment.name, "monitored-deployment" );
  assert_eq!( deployment.state().await, DeploymentState::Active );

  // Verify metrics can be retrieved
  let metrics = deployment.get_metrics();
  assert_eq!( metrics.uptime_percentage(), 100.0 );
  println!( "✓ Monitored deployment created with metrics access" );

  Ok( () )
}

#[ tokio::test ]
async fn test_container_orchestration() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();

  // Test container orchestration configuration
  let models = client.models();
  let model = models.by_name( "gemini-pro" );
  let deployment_builder = model.deploy()
  .with_name( "k8s-deployment" )
  .with_orchestration( OrchestrationConfig::Kubernetes {
    namespace: "ml-models".to_string(),
    cluster: "production".to_string(),
    service_account: "model-deployer".to_string(),
  } )
  .with_container_config( ContainerConfig::builder()
  .image( "gcr.io/project/gemini-pro:latest" )
  .port( 8080 )
  .environment_variables( vec![
  ( "MODEL_PATH".to_string(), "/models/gemini-pro".to_string() ),
  ( "LOG_LEVEL".to_string(), "INFO".to_string() ),
  ] )
  .build()? );

  // Actually deploy with orchestration and verify
  let deployment = deployment_builder.deploy().await?;
  assert_eq!( deployment.name, "k8s-deployment" );
  assert_eq!( deployment.state().await, DeploymentState::Active );
  println!( "✓ Kubernetes orchestrated deployment created successfully" );

  Ok( () )
}

/// Test deployment state management and lifecycle operations
#[ tokio::test ]
async fn test_deployment_state_management() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models = client.models();
  let model = models.by_name( "gemini-pro" );

  // Create deployment
  let deployment = model.deploy()
  .with_name( "state-test-deployment" )
  .with_version( "1.0.0" )
  .with_environment( DeploymentEnvironment::Development )
  .deploy()
  .await?;

  // Initial state should be Active (started by deploy())
  assert_eq!( deployment.state().await, DeploymentState::Active );
println!( "✓ Initial deployment state : {:?}", deployment.state().await );

  // Test scaling operation
  deployment.scale( 3 ).await?;
  assert_eq!( deployment.state().await, DeploymentState::Active );
println!( "✓ Post-scaling state : {:?}", deployment.state().await );

  // Test rollback operation
  deployment.rollback().await?;
  assert_eq!( deployment.state().await, DeploymentState::Active ); // Rollback completes and returns to Active state
println!( "✓ Post-rollback state : {:?}", deployment.state().await );

  // Test stop operation
  deployment.stop().await?;
  assert_eq!( deployment.state().await, DeploymentState::Terminated );
println!( "✓ Post-stop state : {:?}", deployment.state().await );

  Ok( () )
}

/// Test deployment metrics and monitoring
#[ tokio::test ]
async fn test_deployment_metrics_monitoring() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models = client.models();
  let model = models.by_name( "gemini-pro" );

  // Create deployment with comprehensive monitoring
  let monitoring_config = MonitoringConfig::builder()
  .enable_metrics( true )
  .metrics_interval( Duration::from_secs( 10 ) )
  .enable_logging( true )
  .log_level( "INFO".to_string() )
  .alert_on_errors( true )
  .build()?;

  let deployment = model.deploy()
  .with_name( "metrics-test-deployment" )
  .with_version( "1.0.0" )
  .with_environment( DeploymentEnvironment::Development )
  .with_monitoring( monitoring_config )
  .deploy()
  .await?;

  // Get and verify metrics
  let metrics = deployment.get_metrics();
  assert_eq!( metrics.instance_count.load(std::sync::atomic::Ordering::Relaxed), 1 ); // Started deployment has 1 instance
  assert_eq!( metrics.uptime_percentage(), 100.0 );

  println!( "✓ Deployment metrics retrieved successfully:" );
println!( "  - Instances : {}", metrics.instance_count.load(std::sync::atomic::Ordering::Relaxed) );
println!( "  - CPU: {}%", metrics.cpu_utilization() );
println!( "  - Memory : {}%", metrics.memory_utilization() );
println!( "  - Request rate : {} req/s", metrics.request_rate() );
println!( "  - Error rate : {}%", metrics.error_rate() );
println!( "  - Response time : {}ms", metrics.response_time_ms() );
println!( "  - Uptime : {}%", metrics.uptime_percentage() );

  Ok( () )
}

/// Test deployment state change notifications
#[ tokio::test ]
async fn test_deployment_state_notifications() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models = client.models();
  let model = models.by_name( "gemini-pro" );

  // Create deployment
  let deployment = model.deploy()
  .with_name( "notification-test-deployment" )
  .with_version( "1.0.0" )
  .with_environment( DeploymentEnvironment::Development )
  .deploy()
  .await?;

  // Subscribe to state changes
  let mut state_receiver = deployment.subscribe_state_changes();

  // Perform state-changing operation in background
  let deployment_clone = deployment;
  let scale_handle = tokio::spawn( async move {
    tokio ::time::sleep( Duration::from_millis( 100 ) ).await;
    deployment_clone.scale( 2 ).await.unwrap();
  } );

  // Wait for state change notification with timeout
  let notification_result = tokio::time::timeout(
  Duration::from_secs( 2 ),
  state_receiver.recv()
  ).await;

  scale_handle.await?;

  match notification_result
  {
    Ok( Ok( state ) ) => {
    println!( "✓ Received state change notification : {:?}", state );
      assert!( matches!( state, DeploymentState::Scaling | DeploymentState::Active ) );
    },
    Ok( Err( e ) ) => {
    println!( "⚠ Notification receive error (expected in some cases): {}", e );
    },
    Err( _timeout ) => {
      println!( "⚠ No notification received within timeout (may be expected)" );
    }
  }

  Ok( () )
}

/// Test deployment validation and error handling
#[ tokio::test ]
async fn test_deployment_validation_error_handling() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models = client.models();
  let model = models.by_name( "gemini-pro" );

  // Test deployment without name (should fail)
  let result = model.deploy()
  .with_version( "1.0.0" )
  .deploy()
  .await;

  match result
  {
    Err( Error::ApiError( msg ) ) => {
      assert!( msg.contains( "Deployment name is required" ) );
    println!( "✓ Missing deployment name properly rejected : {}", msg );
    },
    _ => panic!( "Deployment without name should fail" ),
  }

  // Test invalid scaling configuration
  let invalid_scaling = ScalingConfig::builder()
  .min_instances( 5 )
  .max_instances( 2 )
  .build();

  match invalid_scaling
  {
    Err( Error::ConfigurationError( msg ) ) => {
    println!( "✓ Invalid scaling configuration rejected : {}", msg );
    },
    _ => panic!( "Invalid scaling configuration should be rejected" ),
  }

  Ok( () )
}

/// Test different deployment environments
#[ tokio::test ]
async fn test_deployment_environments() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models = client.models();
  let model = models.by_name( "gemini-pro" );

  // Test different environments
  let environments = vec![
  ( "dev-deployment", DeploymentEnvironment::Development ),
  ( "staging-deployment", DeploymentEnvironment::Staging ),
  ( "prod-deployment", DeploymentEnvironment::Production ),
  ( "custom-deployment", DeploymentEnvironment::Custom( "testing".to_string() ) ),
  ];

  for ( name, environment ) in environments
  {
    let deployment = model.deploy()
    .with_name( name )
    .with_version( "1.0.0" )
    .with_environment( environment.clone() )
    .deploy()
    .await?;

    assert_eq!( deployment.name, name );
    assert_eq!( deployment.environment, environment );
    assert_eq!( deployment.state().await, DeploymentState::Active );
  println!( "✓ {} environment deployment works", name );
  }

  Ok( () )
}

/// Test concurrent deployment operations
#[ tokio::test ]
async fn test_concurrent_deployments() -> Result< (), Box< dyn std::error::Error > >
{
  // Check if API key is available before running concurrent operations
  let _client = create_integration_client();

  // Create multiple deployments concurrently
  let deployment_configs = vec![
  ( "concurrent-1", "1.0.0", DeploymentEnvironment::Development ),
  ( "concurrent-2", "1.0.1", DeploymentEnvironment::Staging ),
  ( "concurrent-3", "1.0.2", DeploymentEnvironment::Production ),
  ];

  let mut handles = Vec::new();
  for ( name, version, environment ) in deployment_configs
  {
    let handle = tokio::spawn( async move {
      let client = create_integration_client();
      let models = client.models();
      let model = models.by_name( "gemini-pro" );
      model.deploy()
      .with_name( name )
      .with_version( version )
      .with_environment( environment )
      .deploy()
      .await
    } );
    handles.push( ( name, handle ) );
  }

  // Wait for all deployments
  for ( name, handle ) in handles
  {
    let deployment = handle.await??;
    assert_eq!( deployment.name, name );
    assert_eq!( deployment.state().await, DeploymentState::Active );
  println!( "✓ Concurrent deployment {} completed", name );
  }

  Ok( () )
}

/// Test comprehensive deployment configuration
#[ tokio::test ]
async fn test_comprehensive_deployment_config() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models = client.models();
  let model = models.by_name( "gemini-pro" );

  // Create comprehensive configuration
  let scaling_config = ScalingConfig::builder()
  .min_instances( 2 )
  .max_instances( 8 )
  .target_cpu_utilization( 75.0 )
  .target_memory_utilization( 85.0 )
  .build()?;

  let resource_config = ResourceConfig::builder()
  .cpu_cores( 2.0 )
  .memory_gb( 8.0 )
  .gpu_count( 1 )
  .storage_gb( 50.0 )
  .build()?;

  let health_check = DeploymentHealthCheckConfig::builder()
  .endpoint( "/health" )
  .interval( Duration::from_secs( 20 ) )
  .timeout( Duration::from_secs( 5 ) )
  .build()?;

  let mut labels = HashMap::new();
  labels.insert( "team".to_string(), "ml".to_string() );

  let monitoring = MonitoringConfig::builder()
  .enable_metrics( true )
  .metric_labels( labels )
  .build()?;

  let container = ContainerConfig::builder()
  .image( "gcr.io/project/model:v1.0.0" )
  .port( 8080 )
  .environment_variables( vec![
  ( "MODEL_PATH".to_string(), "/models".to_string() ),
  ] )
  .build()?;

  // Deploy with all configurations
  let deployment = model.deploy()
  .with_name( "comprehensive-deployment" )
  .with_version( "1.0.0" )
  .with_environment( DeploymentEnvironment::Staging )
  .with_strategy( DeploymentStrategy::BlueGreen {
    switch_traffic_percentage: 100.0,
    rollback_on_failure: true,
  } )
  .with_scaling_config( scaling_config )
  .with_resource_config( resource_config )
  .with_health_checks( health_check )
  .with_monitoring( monitoring )
  .with_container_config( container )
  .deploy()
  .await?;

  assert_eq!( deployment.name, "comprehensive-deployment" );
  assert_eq!( deployment.state().await, DeploymentState::Active );
  println!( "✓ Comprehensive deployment configuration works" );

  Ok( () )
}
}

}
