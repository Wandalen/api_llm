//! Enterprise features for production deployments.
//!
//! This module provides enterprise-grade functionality including:
//! - Quota management and cost tracking
//! - Usage enforcement and monitoring
//! - Per-user and per-model tracking

pub mod quota_management;

#[ cfg( feature = "enterprise_quota" ) ]
pub mod cost_quota;

pub use quota_management::
{
  QuotaManager,
  QuotaStatus,
  QuotaReservation,
  RequestMetadata,
  QuotaUsage,
  QuotaUsageDetails,
  ConcurrentUsageDetails,
  UserQuotaUsage,
  UsageEfficiencyMetrics,
};

#[ cfg( feature = "enterprise_quota" ) ]
pub use cost_quota::
{
  CostQuotaManager,
  CostQuotaConfig,
  CostQuotaExceededError,
  ModelPricing,
  UsageMetrics as CostUsageMetrics,
};
