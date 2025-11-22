//! Model deployment and hosting capabilities for production environments.
//!
//! This module provides comprehensive deployment management including orchestration,
//! scaling, monitoring, and deployment strategies (blue-green, canary, rolling).

// Module declarations
pub mod strategies;
pub mod auto_scaling;
pub mod health;
pub mod orchestration;

mod private
{
  use serde::{ Deserialize, Serialize };
  use std::time::SystemTime;

  /// State of a model deployment
  #[ derive( Debug, Clone, PartialEq, Eq, Serialize, Deserialize ) ]
  pub enum DeploymentState
  {
    /// Deployment is being created
    Pending,
    /// Deployment is actively serving traffic
    Active,
    /// Deployment is being updated
    Updating,
    /// Deployment is being scaled
    Scaling,
    /// Deployment is being rolled back
    RollingBack,
    /// Deployment has failed
    Failed,
    /// Deployment has been terminated
    Terminated,
  }

  /// Deployment environment types
  #[ derive( Debug, Clone, PartialEq, Eq, Serialize, Deserialize ) ]
  pub enum DeploymentEnvironment
  {
    /// Development environment
    Development,
    /// Staging environment
    Staging,
    /// Production environment
    Production,
    /// Custom environment
    Custom( String ),
  }

  /// Deployment summary for monitoring and dashboards
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct DeploymentSummary
  {
    /// Deployment identifier
    pub deployment_id : String,
    /// Deployment name
    pub name : String,
    /// Model version
    pub version : String,
    /// Current state
    pub state : DeploymentState,
    /// Environment
    pub environment : DeploymentEnvironment,
    /// Number of instances
    pub instance_count : usize,
    /// CPU utilization percentage
    pub cpu_utilization : f64,
    /// Memory utilization percentage
    pub memory_utilization : f64,
    /// Error rate percentage
    pub error_rate : f64,
    /// Average response time in milliseconds
    pub response_time_ms : f64,
    /// Uptime percentage
    pub uptime_percentage : f64,
    /// Health status
    pub is_healthy : bool,
    /// Creation timestamp
    pub created_at : SystemTime,
    /// Total requests processed
    pub total_requests : u64,
  }

  // Re-exports from submodules
  pub use super::strategies::{ DeploymentStrategy, DeploymentCache };
  pub use super::auto_scaling::{
    ScalingConfig,
    ScalingConfigBuilder,
    ResourceConfig,
    ResourceConfigBuilder,
    IntelligentScaler,
    ScalingDecision,
  };
  pub use super::health::{
    DeploymentHealthCheckConfig,
    DeploymentHealthCheckConfigBuilder,
    MonitoringConfig,
    MonitoringConfigBuilder,
    DeploymentMetrics,
    PerformanceOptimizer,
    OptimizationRecommendation,
    OptimizationCategory,
    OptimizationPriority,
    ImpactEstimate,
    ImplementationEffort,
  };
  pub use super::orchestration::{
    ContainerConfig,
    ContainerConfigBuilder,
    OrchestrationConfig,
    ModelDeployment,
    DeploymentBuilder,
  };
}

::mod_interface::mod_interface!
{
  exposed use private::DeploymentState;
  exposed use private::DeploymentEnvironment;
  exposed use private::DeploymentStrategy;
  exposed use private::ScalingConfig;
  exposed use private::ScalingConfigBuilder;
  exposed use private::ResourceConfig;
  exposed use private::ResourceConfigBuilder;
  exposed use private::DeploymentHealthCheckConfig;
  exposed use private::DeploymentHealthCheckConfigBuilder;
  exposed use private::MonitoringConfig;
  exposed use private::MonitoringConfigBuilder;
  exposed use private::ContainerConfig;
  exposed use private::ContainerConfigBuilder;
  exposed use private::OrchestrationConfig;
  exposed use private::DeploymentMetrics;
  exposed use private::ModelDeployment;
  exposed use private::DeploymentBuilder;
  exposed use private::DeploymentSummary;
  exposed use private::DeploymentCache;
  exposed use private::IntelligentScaler;
  exposed use private::ScalingDecision;
  exposed use private::PerformanceOptimizer;
  exposed use private::OptimizationRecommendation;
  exposed use private::OptimizationCategory;
  exposed use private::OptimizationPriority;
  exposed use private::ImpactEstimate;
  exposed use private::ImplementationEffort;
}
