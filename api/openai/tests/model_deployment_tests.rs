//! Model deployment tests
//!
//! This module contains comprehensive tests for the model deployment functionality,
//! testing all core deployment types, configurations, and utilities.

#![ allow( clippy::float_cmp ) ] // Acceptable in tests for exact value checking

use api_openai::*;

#[ tokio::test ]
async fn test_deployment_status_variants()
{
  let preparing = DeploymentStatus::Preparing;
  assert!( matches!( preparing, DeploymentStatus::Preparing ) );

  let deploying = DeploymentStatus::Deploying;
  assert!( matches!( deploying, DeploymentStatus::Deploying ) );

  let active = DeploymentStatus::Active;
  assert!( matches!( active, DeploymentStatus::Active ) );

  let scaling = DeploymentStatus::Scaling;
  assert!( matches!( scaling, DeploymentStatus::Scaling ) );

  let updating = DeploymentStatus::Updating;
  assert!( matches!( updating, DeploymentStatus::Updating ) );

  let rolling_back = DeploymentStatus::RollingBack;
  assert!( matches!( rolling_back, DeploymentStatus::RollingBack ) );

  let paused = DeploymentStatus::Paused;
  assert!( matches!( paused, DeploymentStatus::Paused ) );

  let failed = DeploymentStatus::Failed( "Test error".to_string() );
  assert!( matches!( failed, DeploymentStatus::Failed( _ ) ) );

  let terminating = DeploymentStatus::Terminating;
  assert!( matches!( terminating, DeploymentStatus::Terminating ) );

  let terminated = DeploymentStatus::Terminated;
  assert!( matches!( terminated, DeploymentStatus::Terminated ) );
}

#[ tokio::test ]
async fn test_deployment_strategy_variants()
{
  let rolling = DeploymentStrategy::Rolling { max_surge : 2, max_unavailable : 1 };
  assert!( matches!( rolling, DeploymentStrategy::Rolling { .. } ) );

  let blue_green = DeploymentStrategy::BlueGreen { traffic_split : 50 };
  assert!( matches!( blue_green, DeploymentStrategy::BlueGreen { .. } ) );

  let canary = DeploymentStrategy::Canary { initial_traffic : 10, final_traffic : 100, evaluation_duration_ms : 300_000 };
  assert!( matches!( canary, DeploymentStrategy::Canary { .. } ) );

  let recreate = DeploymentStrategy::Recreate;
  assert!( matches!( recreate, DeploymentStrategy::Recreate ) );
}

#[ tokio::test ]
async fn test_resource_requirements()
{
  let requirements = ResourceRequirements
  {
    cpu : 4.0,
    memory_mb : 8192,
    storage_gb : 100,
    gpu : Some( 1 ),
  };

  assert_eq!( requirements.cpu, 4.0 );
  assert_eq!( requirements.memory_mb, 8192 );
  assert_eq!( requirements.storage_gb, 100 );
  assert_eq!( requirements.gpu, Some( 1 ) );
}

#[ tokio::test ]
async fn test_auto_scaling_config()
{
  let config = AutoScalingConfig
  {
    min_replicas : 2,
    max_replicas : 10,
    target_cpu_percent : 70,
    target_memory_percent : 80,
    scale_up_cooldown_s : 300,
    scale_down_cooldown_s : 600,
  };

  assert_eq!( config.min_replicas, 2 );
  assert_eq!( config.max_replicas, 10 );
  assert_eq!( config.target_cpu_percent, 70 );
  assert_eq!( config.target_memory_percent, 80 );
  assert_eq!( config.scale_up_cooldown_s, 300 );
  assert_eq!( config.scale_down_cooldown_s, 600 );
}

#[ tokio::test ]
async fn test_deployment_stats()
{
  let stats = DeploymentStats
  {
    total : 10,
    active : 8,
    failed : 1,
    preparing : 1,
    deploying : 0,
    scaling : 0,
    total_replicas : 24,
    healthy_replicas : 22,
  };

  assert_eq!( stats.total, 10 );
  assert_eq!( stats.active, 8 );
  assert_eq!( stats.failed, 1 );
  assert_eq!( stats.preparing, 1 );
  assert_eq!( stats.total_replicas, 24 );
  assert_eq!( stats.healthy_replicas, 22 );
}

#[ tokio::test ]
async fn test_resource_requirements_default()
{
  let default_resources = ResourceRequirements::default();

  assert_eq!( default_resources.cpu, 1.0 );
  assert_eq!( default_resources.memory_mb, 2048 );
  assert_eq!( default_resources.storage_gb, 10 );
  assert_eq!( default_resources.gpu, None );
}

#[ tokio::test ]
async fn test_auto_scaling_config_default()
{
  let default_autoscaling = AutoScalingConfig::default();

  assert_eq!( default_autoscaling.min_replicas, 1 );
  assert_eq!( default_autoscaling.max_replicas, 10 );
  assert_eq!( default_autoscaling.target_cpu_percent, 70 );
  assert_eq!( default_autoscaling.target_memory_percent, 80 );
  assert_eq!( default_autoscaling.scale_up_cooldown_s, 300 );
  assert_eq!( default_autoscaling.scale_down_cooldown_s, 600 );
}

#[ tokio::test ]
async fn test_deployment_event_notifier()
{
  let ( _sender, _receiver ) = ModelDeploymentUtils::create_event_notifier();
  // Basic test to ensure the notifier can be created without errors
}

#[ tokio::test ]
async fn test_deployment_manager_creation()
{
  let config = DeploymentManagerConfig::default();
  let manager = DeploymentManager::new( config );
  // Basic test to ensure the manager can be created
  let stats = manager.deployment_stats();
  assert_eq!( stats.total, 0 ); // New manager should have no deployments
}

#[ tokio::test ]
async fn test_serialization_roundtrip()
{
  let status = DeploymentStatus::Active;
  let serialized = serde_json::to_string( &status ).unwrap();
  let deserialized : DeploymentStatus = serde_json::from_str( &serialized ).unwrap();
  assert!( matches!( deserialized, DeploymentStatus::Active ) );
}

#[ tokio::test ]
async fn test_deployment_strategy_serialization()
{
  let strategy = DeploymentStrategy::Rolling { max_surge : 2, max_unavailable : 1 };
  let serialized = serde_json::to_string( &strategy ).unwrap();
  let deserialized : DeploymentStrategy = serde_json::from_str( &serialized ).unwrap();
  assert!( matches!( deserialized, DeploymentStrategy::Rolling { .. } ) );
}

#[ tokio::test ]
async fn test_resource_requirements_serialization()
{
  let resources = ResourceRequirements
  {
    cpu : 4.0,
    memory_mb : 8192,
    storage_gb : 100,
    gpu : Some( 1 ),
  };

  let serialized = serde_json::to_string( &resources ).unwrap();
  let deserialized : ResourceRequirements = serde_json::from_str( &serialized ).unwrap();

  assert_eq!( deserialized.cpu, 4.0 );
  assert_eq!( deserialized.memory_mb, 8192 );
  assert_eq!( deserialized.storage_gb, 100 );
  assert_eq!( deserialized.gpu, Some( 1 ) );
}

#[ tokio::test ]
async fn test_auto_scaling_config_serialization()
{
  let config = AutoScalingConfig
  {
    min_replicas : 2,
    max_replicas : 10,
    target_cpu_percent : 70,
    target_memory_percent : 80,
    scale_up_cooldown_s : 300,
    scale_down_cooldown_s : 600,
  };

  let serialized = serde_json::to_string( &config ).unwrap();
  let deserialized : AutoScalingConfig = serde_json::from_str( &serialized ).unwrap();

  assert_eq!( deserialized.min_replicas, 2 );
  assert_eq!( deserialized.max_replicas, 10 );
  assert_eq!( deserialized.target_cpu_percent, 70 );
  assert_eq!( deserialized.target_memory_percent, 80 );
  assert_eq!( deserialized.scale_up_cooldown_s, 300 );
  assert_eq!( deserialized.scale_down_cooldown_s, 600 );
}

#[ tokio::test ]
async fn test_deployment_status_cloning()
{
  let original = DeploymentStatus::Failed( "Test error".to_string() );
  let cloned = original.clone();

  assert!( matches!( cloned, DeploymentStatus::Failed( _ ) ) );
  if let DeploymentStatus::Failed( msg ) = cloned
  {
    assert_eq!( msg, "Test error" );
  }
}

#[ tokio::test ]
async fn test_deployment_strategy_cloning()
{
  let original = DeploymentStrategy::Canary { initial_traffic : 10, final_traffic : 100, evaluation_duration_ms : 300_000 };
  let cloned = original.clone();

  assert!( matches!( cloned, DeploymentStrategy::Canary { .. } ) );
  if let DeploymentStrategy::Canary { initial_traffic, final_traffic, evaluation_duration_ms } = cloned
  {
    assert_eq!( initial_traffic, 10 );
    assert_eq!( final_traffic, 100 );
    assert_eq!( evaluation_duration_ms, 300_000 );
  }
}

#[ tokio::test ]
async fn test_resource_requirements_cloning()
{
  let original = ResourceRequirements
  {
    cpu : 8.0,
    memory_mb : 16384,
    storage_gb : 500,
    gpu : Some( 2 ),
  };

  let cloned = original.clone();

  assert_eq!( cloned.cpu, 8.0 );
  assert_eq!( cloned.memory_mb, 16384 );
  assert_eq!( cloned.storage_gb, 500 );
  assert_eq!( cloned.gpu, Some( 2 ) );
}

#[ tokio::test ]
async fn test_auto_scaling_config_cloning()
{
  let original = AutoScalingConfig
  {
    min_replicas : 3,
    max_replicas : 15,
    target_cpu_percent : 65,
    target_memory_percent : 75,
    scale_up_cooldown_s : 240,
    scale_down_cooldown_s : 480,
  };

  let cloned = original.clone();

  assert_eq!( cloned.min_replicas, 3 );
  assert_eq!( cloned.max_replicas, 15 );
  assert_eq!( cloned.target_cpu_percent, 65 );
  assert_eq!( cloned.target_memory_percent, 75 );
  assert_eq!( cloned.scale_up_cooldown_s, 240 );
  assert_eq!( cloned.scale_down_cooldown_s, 480 );
}

#[ tokio::test ]
async fn test_deployment_stats_cloning()
{
  let original = DeploymentStats
  {
    total : 5,
    active : 4,
    failed : 0,
    preparing : 1,
    deploying : 0,
    scaling : 0,
    total_replicas : 12,
    healthy_replicas : 10,
  };

  let cloned = original.clone();

  assert_eq!( cloned.total, 5 );
  assert_eq!( cloned.active, 4 );
  assert_eq!( cloned.failed, 0 );
  assert_eq!( cloned.preparing, 1 );
  assert_eq!( cloned.total_replicas, 12 );
  assert_eq!( cloned.healthy_replicas, 10 );
}

#[ tokio::test ]
async fn test_module_exports_availability()
{
  // This test ensures all the main types are exported and accessible
  let _ = DeploymentStatus::Preparing;
  let _ = DeploymentStrategy::Recreate;
  let _ = ResourceRequirements::default();
  let _ = AutoScalingConfig::default();
  let _ = DeploymentStats {
    total : 0,
    active : 0,
    failed : 0,
    preparing : 0,
    deploying : 0,
    scaling : 0,
    total_replicas : 0,
    healthy_replicas : 0,
  };
  let _ = DeploymentManager::new( DeploymentManagerConfig::default() );
  let ( _, _ ) = ModelDeploymentUtils::create_event_notifier();
}