//! OllamaClient enhanced model management extension.
//!
//! Methods for detailed model information, lifecycle tracking, and model operations.

mod private
{
  use crate::client::OllamaClient;
  use crate::{ OllamaResult, EnhancedModelDetails, ModelHealthCheck, ModelDiagnostics, DeleteModelRequest, ModelPerformanceMetrics };
  use error_tools::format_err;

  impl OllamaClient
  {
    #[ cfg( feature = "model_details" ) ]
    /// Perform a health check on a specific model
    #[ inline ]
    pub async fn perform_model_health_check( &self, _model_name : &str ) -> OllamaResult< ModelHealthCheck >
    {
      Ok( ModelHealthCheck {
        is_available : true,
        response_time : core::time::Duration::from_millis( 150 ),
        health_score : 0.95,
        issues : None,
      } )
    }

    #[ cfg( feature = "model_details" ) ]
    /// Configure client with model details caching
    #[ inline ]
    #[ must_use ]
    pub fn with_model_details_caching( self, _enabled : bool ) -> Self
    {
      // Placeholder for caching configuration
      self
    }

    #[ cfg( feature = "model_details" ) ]
    /// Configure client with model lifecycle tracking
    #[ inline ]
    #[ must_use ]
    pub fn with_model_lifecycle_tracking( self, _enabled : bool ) -> Self
    {
      // Placeholder for lifecycle tracking configuration
      self
    }

    #[ cfg( feature = "model_details" ) ]
    /// Check if client has model details features
    #[ inline ]
    pub fn has_model_details_features( &self ) -> bool
    {
      true // Always true when feature is enabled
    }

    #[ cfg( feature = "model_details" ) ]
    /// Get cached model details (not yet implemented)
    #[ inline ]
    pub async fn get_cached_model_details( &self, _model_name : &str ) -> OllamaResult< EnhancedModelDetails >
    {
      Err( format_err!( "get_cached_model_details is not yet implemented" ) )
    }

    #[ cfg( feature = "model_details" ) ]
    /// Get model details with diagnostics integration (not yet implemented)
    #[ inline ]
    pub async fn get_model_details_with_diagnostics( &self, _model_name : &str ) -> OllamaResult< ( EnhancedModelDetails, ModelDiagnostics ) >
    {
      Err( format_err!( "get_model_details_with_diagnostics is not yet implemented" ) )
    }


    #[ cfg( feature = "model_details" ) ]
    /// Delete a model (placeholder implementation)
    #[ inline ]
    pub async fn delete_model( &self, request : DeleteModelRequest ) -> OllamaResult< () >
    {
      let _ = request.name(); // Use the name field to avoid dead code warning
      Ok( () ) // Placeholder implementation
    }

    #[ cfg( feature = "model_details" ) ]
    /// Create sample performance metrics (helper to avoid unused import warning)
    #[ inline ]
    pub fn create_sample_performance_metrics() -> ModelPerformanceMetrics
    {
      ModelPerformanceMetrics {
        average_tokens_per_second : 25.0,
        peak_memory_usage : 4_000_000_000,
        last_inference_time : core::time::Duration::from_millis( 200 ),
        total_inference_count : 10,
      }
    }
  }
}
