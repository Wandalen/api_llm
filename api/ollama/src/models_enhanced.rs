//! Enhanced model details types for Ollama API.
//!
//! Provides comprehensive model metadata, lifecycle tracking, and performance metrics.

#[ cfg( feature = "model_details" ) ]
mod private
{
  use core::time::Duration;

  /// Enhanced model details with comprehensive metadata
  #[ derive( Debug, Clone ) ]
  pub struct EnhancedModelDetails
  {
    name : String,
    metadata : ModelMetadata,
    lifecycle_status : ModelLifecycle,
    download_progress : f64,
    performance_metrics : Option< ModelPerformanceMetrics >,
  }

  impl EnhancedModelDetails
  {
    /// Create new enhanced model details
    #[ inline ]
    #[ must_use ]
    pub fn new( name : impl Into< String > ) -> Self
    {
      Self
      {
        name : name.into(),
        metadata : ModelMetadata::default(),
        lifecycle_status : ModelLifecycle::NotFound,
        download_progress : 0.0,
        performance_metrics : None,
      }
    }

    /// Set model metadata
    #[ inline ]
    #[ must_use ]
    pub fn with_metadata( mut self, metadata : ModelMetadata ) -> Self
    {
      self.metadata = metadata;
      self
    }

    /// Set lifecycle status
    #[ inline ]
    #[ must_use ]
    pub fn with_lifecycle_status( mut self, status : ModelLifecycle ) -> Self
    {
      self.lifecycle_status = status;
      self
    }

    /// Set download progress
    #[ inline ]
    #[ must_use ]
    pub fn with_download_progress( mut self, progress : f64 ) -> Self
    {
      self.download_progress = progress;
      self
    }

    /// Set performance metrics
    #[ inline ]
    #[ must_use ]
    pub fn with_performance_metrics( mut self, metrics : ModelPerformanceMetrics ) -> Self
    {
      self.performance_metrics = Some( metrics );
      self
    }

    /// Get model name
    #[ inline ]
    pub fn name( &self ) -> &str
    {
      &self.name
    }

    /// Get model metadata
    #[ inline ]
    pub fn metadata( &self ) -> &ModelMetadata
    {
      &self.metadata
    }

    /// Get lifecycle status
    #[ inline ]
    pub fn lifecycle_status( &self ) -> &ModelLifecycle
    {
      &self.lifecycle_status
    }

    /// Get download progress
    #[ inline ]
    #[ must_use ]
    pub fn download_progress( &self ) -> f64
    {
      self.download_progress
    }

    /// Get performance metrics
    #[ inline ]
    #[ must_use ]
    pub fn performance_metrics( &self ) -> Option< &ModelPerformanceMetrics >
    {
      self.performance_metrics.as_ref()
    }
  }

  /// Comprehensive model metadata
  #[ derive( Debug, Clone, Default ) ]
  pub struct ModelMetadata
  {
    /// Model name identifier
    pub name : String,
    /// Model version tag
    pub tag : String,
    /// Model size in bytes
    pub size : u64,
    /// Model content digest/hash
    pub digest : String,
    /// Last modification timestamp
    pub modified_at : String,
    /// Model file format
    pub format : String,
    /// Model family classification
    pub family : String,
    /// List of related model families
    pub families : Vec< String >,
    /// Parameter count as string
    pub parameter_size : String,
    /// Quantization level applied
    pub quantization_level : String,
    /// Model architecture type
    pub architecture : String,
    /// License information
    pub license : String,
    /// Model template configuration
    pub template : String,
    /// Default system prompt
    pub system_prompt : Option< String >,
    /// Additional model parameters
    pub parameters : std::collections::HashMap<  String, serde_json::Value  >,
  }

  /// Model lifecycle status
  #[ derive( Debug, Clone, PartialEq ) ]
  pub enum ModelLifecycle
  {
    /// Model is ready for use
    Ready,
    /// Model is currently loading
    Loading,
    /// Model is being downloaded
    Downloading,
    /// Model is in error state
    Error,
    /// Model not found
    NotFound,
  }

  /// Model operation types
  #[ derive( Debug, Clone ) ]
  pub enum ModelOperation
  {
    /// Model pull operation
    Pull,
    /// Model push operation
    Push,
    /// Model delete operation
    Delete,
    /// Model load operation
    Load,
    /// Model unload operation
    Unload,
    /// Model inference operation
    Inference,
  }

  /// Model performance metrics
  #[ derive( Debug, Clone ) ]
  pub struct ModelPerformanceMetrics
  {
    /// Average token generation rate
    pub average_tokens_per_second : f64,
    /// Peak memory usage in bytes
    pub peak_memory_usage : u64,
    /// Duration of last inference operation
    pub last_inference_time : Duration,
    /// Total number of inference operations
    pub total_inference_count : u64,
  }
}

#[ cfg( feature = "model_details" ) ]
crate ::mod_interface!
{
  exposed use
  {
    EnhancedModelDetails,
    ModelMetadata,
    ModelLifecycle,
    ModelOperation,
    ModelPerformanceMetrics,
  };
}
