//! Additional model information types for Ollama API.
//!
//! Provides supplementary model structures for comprehensive information,
//! recommendations, diagnostics, and storage details.

#[ cfg( feature = "model_details" ) ]
mod private
{
  use core::time::Duration;
  use super::super::*;

  /// Comprehensive model information
  #[ derive( Debug, Clone ) ]
  pub struct ComprehensiveModelInfo
  {
    /// Model size in bytes
    pub size_bytes : u64,
    /// Human-readable size format
    pub size_human_readable : String,
    /// Model family classification
    pub family : String,
    /// List of related model families
    pub families : Vec< String >,
    /// Parameter count as string
    pub parameter_size : String,
    /// Numerical parameter count
    pub parameter_count : u64,
    /// Model architecture type
    pub architecture : String,
    /// Quantization level applied
    pub quantization_level : String,
    /// Model file format
    pub format : String,
    /// List of supported features
    pub supported_features : Vec< String >,
    /// Context window length
    pub context_length : u32,
    /// Maximum sequence length
    pub max_sequence_length : u32,
  }

  /// Model family recommendation
  #[ derive( Debug, Clone ) ]
  pub struct ModelRecommendation
  {
    /// Recommended model name
    pub model_name : String,
    /// Reason for recommendation
    pub reason : String,
    /// Similarity score (0.0-1.0)
    pub similarity_score : f64,
  }

  /// Model lifecycle status information
  #[ derive( Debug, Clone ) ]
  pub struct ModelLifecycleStatus
  {
    /// Current lifecycle state
    pub current_state : ModelLifecycle,
    /// Last usage timestamp
    pub last_used_at : Option< String >,
    /// Duration of last loading operation
    pub last_loading_duration : Option< Duration >,
    /// Total usage count
    pub usage_count : u64,
  }

  /// Model operation history entry
  #[ derive( Debug, Clone ) ]
  pub struct ModelOperationHistoryEntry
  {
    /// Type of operation performed
    pub operation_type : ModelOperation,
    /// Operation timestamp
    pub timestamp : String,
    /// Operation duration
    pub duration : Duration,
  }

  /// Model health check result
  #[ derive( Debug, Clone ) ]
  pub struct ModelHealthCheck
  {
    /// Whether model is available
    pub is_available : bool,
    /// Response time for health check
    pub response_time : Duration,
    /// Health score (0.0-1.0)
    pub health_score : f64,
    /// List of identified issues
    pub issues : Option< Vec< String > >,
  }

  /// Local model storage information
  #[ derive( Debug, Clone ) ]
  pub struct LocalModelStorageInfo
  {
    /// Total number of models stored
    pub total_models : u32,
    /// Total size of all models in bytes
    pub total_size_bytes : u64,
    /// Local storage path
    pub storage_path : String,
    /// Available storage space in bytes
    pub available_space_bytes : u64,
  }

  /// Model diagnostics information
  #[ derive( Debug, Clone ) ]
  pub struct ModelDiagnostics
  {
    /// Total request count
    pub request_count : u64,
  }
}

#[ cfg( feature = "model_details" ) ]
crate ::mod_interface!
{
  exposed use
  {
    ComprehensiveModelInfo,
    ModelRecommendation,
    ModelLifecycleStatus,
    ModelOperationHistoryEntry,
    ModelHealthCheck,
    LocalModelStorageInfo,
    ModelDiagnostics,
  };
}
