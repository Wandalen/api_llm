//! Model comparison and recommendation types for the Gemini API.

use serde::{ Deserialize, Serialize };
use super::core::Model;

/// Request for comparing multiple models.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct CompareModelsRequest
{
  /// List of model names to compare.
  pub model_names : Vec< String >,

  /// Comparison criteria to evaluate.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub criteria : Option< Vec< String > >,

  /// Whether to include performance benchmarks.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub include_benchmarks : Option< bool >,

  /// Whether to include cost analysis.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub include_cost_analysis : Option< bool >,
}

/// Response from comparing models.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct CompareModelsResponse
{
  /// List of model comparisons.
  pub comparisons : Vec< ModelComparison >,

  /// Overall recommendation if requested.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub recommendation : Option< ModelRecommendation >,
}

/// Comparison data for a single model.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct ModelComparison
{
  /// The model being compared.
  pub model : Model,

  /// Performance metrics for this model.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub performance_metrics : Option< PerformanceMetrics >,

  /// Cost analysis for this model.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub cost_analysis : Option< CostAnalysis >,

  /// Suitability score for the requested use case.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub suitability_score : Option< f64 >,
}

/// Performance metrics for a model.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct PerformanceMetrics
{
  /// Average response time in milliseconds.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub avg_response_time : Option< f64 >,

  /// Tokens processed per second.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub tokens_per_second : Option< f64 >,

  /// Quality score based on benchmarks.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub quality_score : Option< f64 >,

  /// Reliability percentage.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub reliability : Option< f64 >,
}

/// Cost analysis for a model.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct CostAnalysis
{
  /// Cost per 1000 input tokens.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub input_cost_per_1k : Option< f64 >,

  /// Cost per 1000 output tokens.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub output_cost_per_1k : Option< f64 >,

  /// Estimated monthly cost for typical usage.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub estimated_monthly_cost : Option< f64 >,

  /// Cost efficiency ranking compared to other models.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub cost_efficiency_rank : Option< i32 >,
}

/// Model recommendation based on use case.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct ModelRecommendation
{
  /// Recommended model name.
  pub recommended_model : String,

  /// Confidence score of the recommendation.
  pub confidence_score : f64,

  /// Reasoning for the recommendation.
  pub reasoning : String,

  /// Alternative models to consider.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub alternatives : Option< Vec< String > >,
}

/// Request for getting model recommendations.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct GetRecommendationsRequest
{
  /// Use case description for the recommendation.
  pub use_case : String,

  /// Expected input size range.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub input_size_range : Option< String >,

  /// Performance requirements.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub performance_requirements : Option< Vec< String > >,

  /// Budget constraints.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub budget_constraints : Option< f64 >,

  /// Whether real-time response is needed.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub real_time_required : Option< bool >,
}

/// Response containing model recommendations.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct GetRecommendationsResponse
{
  /// List of recommended models in priority order.
  pub recommendations : Vec< ModelRecommendation >,

  /// Analysis of the use case requirements.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub use_case_analysis : Option< String >,
}

/// Request for advanced model filtering.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct AdvancedFilterRequest
{
  /// Capabilities to filter by.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub capabilities : Option< Vec< String > >,

  /// Maximum cost per 1000 tokens.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub max_cost_per_1k : Option< f64 >,

  /// Minimum quality score.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub min_quality_score : Option< f64 >,

  /// Maximum response time in milliseconds.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub max_response_time : Option< f64 >,

  /// Whether model supports streaming.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub supports_streaming : Option< bool >,

  /// Sort criteria and direction.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub sort_by : Option< String >,
}

/// Response from advanced filtering.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct AdvancedFilterResponse
{
  /// Filtered and sorted models.
  pub models : Vec< Model >,

  /// Total number of models that matched the criteria.
  pub total_matches : i32,

  /// Applied filters summary.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub applied_filters : Option< String >,
}

/// Request for model availability status.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct ModelStatusRequest
{
  /// Model names to check status for.
  pub model_names : Vec< String >,

  /// Whether to include detailed health metrics.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub include_health_metrics : Option< bool >,
}

/// Response containing model status information.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct ModelStatusResponse
{
  /// Status for each requested model.
  pub model_statuses : Vec< ModelStatus >,

  /// Overall service health.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub service_health : Option< String >,
}

/// Status information for a single model.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct ModelStatus
{
  /// The model name.
  pub model_name : String,

  /// Current availability status.
  pub status : String,

  /// Health percentage if available.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub health_percentage : Option< f64 >,

  /// Last known issue if any.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub last_issue : Option< String >,

  /// Next maintenance window if scheduled.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub next_maintenance : Option< String >,
}
