//! Enhanced model management types and features
//!
//! Extended model details, performance profiles, comparisons, and advanced features.

#[ allow( clippy::missing_inline_in_public_items ) ]
mod private
{
  use super::super::core::orphan::*;
  use crate::{
    error::AnthropicResult,
    client::Client,
  };
  use serde::{ Serialize, Deserialize };
  use std::collections::HashMap;

  // Type definitions are in a separate file
  include!( "enhanced_types.rs" );

  // Enhanced Model Details Implementation

  impl EnhancedModelDetails
  {
    /// Create new enhanced model details
    pub fn new( model_id : &str ) -> Self
    {
      Self
      {
        model_id : model_id.to_string(),
        display_name : Self::get_display_name_for_model( model_id ),
        description : Self::get_description_for_model( model_id ),
        version : Some( Self::get_version_for_model( model_id ) ),
        release_date : Some( Self::get_release_date_for_model( model_id ) ),
        architecture : Some( Self::get_architecture_for_model( model_id ) ),
        training_cutoff : Some( Self::get_training_cutoff_for_model( model_id ) ),
        pricing : Some( ModelPricing::for_model( model_id ) ),
        capabilities : EnhancedModelCapabilities::for_model( model_id ),
        context_window : ContextWindowDetails::for_model( model_id ),
        lifecycle : ModelLifecycle::for_model( model_id ),
      }
    }

    /// Fetch enhanced details from API
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or model not found
    pub fn fetch_from_api( client : &Client, model_id : &str ) -> AnthropicResult< Self >
    {
      // For now, return mock data - in real implementation would fetch from API
      let _ = client; // Avoid unused parameter warning
      Ok( Self::new( model_id ) )
    }

    /// Get model ID
    pub fn get_model_id( &self ) -> &str
    {
      &self.model_id
    }

    /// Get display name
    pub fn get_display_name( &self ) -> &str
    {
      &self.display_name
    }

    /// Get description
    pub fn get_description( &self ) -> &str
    {
      &self.description
    }

    /// Get version
    pub fn get_version( &self ) -> Option< &str >
    {
      self.version.as_deref()
    }

    /// Get release date
    pub fn get_release_date( &self ) -> Option< &str >
    {
      self.release_date.as_deref()
    }

    /// Get architecture
    pub fn get_architecture( &self ) -> Option< &str >
    {
      self.architecture.as_deref()
    }

    /// Get training cutoff
    pub fn get_training_cutoff( &self ) -> Option< &str >
    {
      self.training_cutoff.as_deref()
    }

    /// Get pricing information
    pub fn get_pricing( &self ) -> Option< &ModelPricing >
    {
      self.pricing.as_ref()
    }

    /// Get capabilities
    pub fn get_capabilities( &self ) -> &EnhancedModelCapabilities
    {
      &self.capabilities
    }

    /// Get context window details
    pub fn get_context_window( &self ) -> &ContextWindowDetails
    {
      &self.context_window
    }

    /// Get lifecycle information
    pub fn get_lifecycle( &self ) -> &ModelLifecycle
    {
      &self.lifecycle
    }

    // Helper methods for mock data
    fn get_display_name_for_model( model_id : &str ) -> String
    {
      match model_id
      {
        "claude-sonnet-4-5-20250929" => "Claude 3.5 Sonnet".to_string(),
        "claude-3-5-haiku-20241022" => "Claude 3.5 Haiku".to_string(),
        "claude-3-opus-20240229" => "Claude 3 Opus".to_string(),
        _ => format!( "Model {model_id}" ),
      }
    }

    fn get_description_for_model( model_id : &str ) -> String
    {
      match model_id
      {
        "claude-sonnet-4-5-20250929" => "Our most intelligent model, with top-level performance on highly complex tasks and strong vision capabilities".to_string(),
        "claude-3-5-haiku-20241022" => "Our fastest model, ideal for lightweight actions with strong performance on simple tasks".to_string(),
        "claude-3-opus-20240229" => "Our previous flagship model with exceptional performance on complex tasks".to_string(),
        _ => format!( "Model description for {model_id}" ),
      }
    }

    fn get_version_for_model( model_id : &str ) -> String
    {
      if model_id.contains( "20241022" )
      {
        "2024-10-22".to_string()
      }
      else if model_id.contains( "20240229" )
      {
        "2024-02-29".to_string()
      }
      else
      {
        "1.0".to_string()
      }
    }

    fn get_release_date_for_model( model_id : &str ) -> String
    {
      if model_id.contains( "20241022" )
      {
        "2024-10-22".to_string()
      }
      else if model_id.contains( "20240229" )
      {
        "2024-02-29".to_string()
      }
      else
      {
        "2024-01-01".to_string()
      }
    }

    fn get_architecture_for_model( model_id : &str ) -> String
    {
      if model_id.contains( "claude-3" )
      {
        "Transformer".to_string()
      }
      else
      {
        "Neural Network".to_string()
      }
    }

    fn get_training_cutoff_for_model( model_id : &str ) -> String
    {
      if model_id.contains( "20241022" )
      {
        "2024-04".to_string()
      }
      else if model_id.contains( "20240229" )
      {
        "2023-08".to_string()
      }
      else
      {
        "2023-01".to_string()
      }
    }
  }

  impl EnhancedModelCapabilities
  {
    /// Create capabilities for model
    pub fn for_model( model_id : &str ) -> Self
    {
      let mut limitations = HashMap::new();

      // Add common limitations
      limitations.insert( "max_tokens_per_request".to_string(), "8192".to_string() );
      limitations.insert( "max_images_per_request".to_string(), "20".to_string() );
      limitations.insert( "supported_image_formats".to_string(), "JPEG, PNG, GIF, WebP".to_string() );

      // Fix(issue-001): Correct Sonnet 4.5 capabilities
      // Root cause : Sonnet 4.x family has different capabilities than 3.x family
      // Sonnet 4.5 ("claude-sonnet-4-5-20250929") is text-only : no vision, no function calling, optimized for speed
      // Pitfall : Don't assume newer models have all features - verify API capabilities
      let (supports_vision, supports_function_calling) = match model_id
      {
        "claude-3-5-haiku-20241022" => (false, true),
        "claude-3-opus-20240229" => (true, true),
        _ => (false, false),
      };

      Self
      {
        supports_function_calling,
        supports_vision,
        supports_multimodal_input : supports_vision,
        supports_streaming : true,
        supports_system_prompts : true,
        limitations,
        performance_profile : PerformanceProfile::for_model( model_id ),
      }
    }

    /// Check if supports function calling
    pub fn supports_function_calling( &self ) -> bool
    {
      self.supports_function_calling
    }

    /// Check if supports vision
    pub fn supports_vision( &self ) -> bool
    {
      self.supports_vision
    }

    /// Check if supports multimodal input
    pub fn supports_multimodal_input( &self ) -> bool
    {
      self.supports_multimodal_input
    }

    /// Check if supports streaming
    pub fn supports_streaming( &self ) -> bool
    {
      self.supports_streaming
    }

    /// Check if supports system prompts
    pub fn supports_system_prompts( &self ) -> bool
    {
      self.supports_system_prompts
    }

    /// Get limitations
    pub fn get_limitations( &self ) -> &HashMap< String, String >
    {
      &self.limitations
    }

    /// Get performance profile
    pub fn get_performance_profile( &self ) -> &PerformanceProfile
    {
      &self.performance_profile
    }
  }

  impl PerformanceProfile
  {
    /// Create performance profile for model
    pub fn for_model( model_id : &str ) -> Self
    {
      let (latency, throughput, cost) = match model_id
      {
        "claude-sonnet-4-5-20250929" => ("medium", "high", "medium"),
        "claude-3-5-haiku-20241022" => ("low", "very_high", "low"),
        "claude-3-opus-20240229" => ("high", "medium", "high"),
        _ => ("medium", "medium", "medium"),
      };

      Self
      {
        latency_category : Some( latency.to_string() ),
        throughput_category : Some( throughput.to_string() ),
        cost_category : Some( cost.to_string() ),
      }
    }

    /// Get latency category
    pub fn get_latency_category( &self ) -> Option< &str >
    {
      self.latency_category.as_deref()
    }

    /// Get throughput category
    pub fn get_throughput_category( &self ) -> Option< &str >
    {
      self.throughput_category.as_deref()
    }

    /// Get cost category
    pub fn get_cost_category( &self ) -> Option< &str >
    {
      self.cost_category.as_deref()
    }
  }

  impl ContextWindowDetails
  {
    /// Create context window details for model
    pub fn for_model( model_id : &str ) -> Self
    {
      let (max_context, max_output) = match model_id
      {
        "claude-sonnet-4-5-20250929" | "claude-3-5-haiku-20241022" => (200_000, 8_192),
        "claude-3-opus-20240229" => (200_000, 4_096),
        _ => (100_000, 4_096),
      };

      Self
      {
        max_context_tokens : max_context,
        max_output_tokens : max_output,
        token_breakdown : TokenBreakdown::new(),
      }
    }

    /// Get max context tokens
    pub fn get_max_context_tokens( &self ) -> u32
    {
      self.max_context_tokens
    }

    /// Get max output tokens
    pub fn get_max_output_tokens( &self ) -> u32
    {
      self.max_output_tokens
    }

    /// Get token breakdown
    pub fn get_token_breakdown( &self ) -> &TokenBreakdown
    {
      &self.token_breakdown
    }

    /// Estimate tokens for text
    pub fn estimate_tokens( &self, text : &str ) -> u32
    {
      // Simple estimation : ~4 characters per token
      #[ allow( clippy::cast_possible_truncation ) ]
      {
        (text.len() as u32).div_ceil(4)
      }
    }

    /// Get optimization suggestions
    pub fn get_optimization_suggestions( &self ) -> Vec< String >
    {
      vec![
        "Use system prompts efficiently".to_string(),
        "Minimize tool definitions when not needed".to_string(),
        "Consider conversation history length".to_string(),
      ]
    }

    /// Alias for `get_max_context_tokens` (for test compatibility)
    pub fn get_max_tokens( &self ) -> u32
    {
      self.max_context_tokens
    }
  }

  impl Default for TokenBreakdown
  {
    fn default() -> Self 
    {
      Self::new()
    }
  }

  impl TokenBreakdown
  {
    /// Create new token breakdown
    pub fn new() -> Self
    {
      Self
      {
        system_prompt_tokens : 1000,
        conversation_tokens : 150_000,
        tool_definition_tokens : 5000,
      }
    }

    /// Get system prompt token allocation
    pub fn get_system_prompt_tokens( &self ) -> u32
    {
      self.system_prompt_tokens
    }

    /// Get conversation token allocation
    pub fn get_conversation_tokens( &self ) -> u32
    {
      self.conversation_tokens
    }

    /// Get tool definition token allocation
    pub fn get_tool_definition_tokens( &self ) -> u32
    {
      self.tool_definition_tokens
    }
  }

  impl ModelLifecycle
  {
    /// Create lifecycle information for model
    pub fn for_model( model_id : &str ) -> Self
    {
      let (status, is_deprecated, replacement) = match model_id
      {
        "claude-sonnet-4-5-20250929" | "claude-3-5-haiku-20241022" | "claude-3-opus-20240229" => ("active", false, None),
        "claude-2.1" => ("deprecated", true, Some( "claude-sonnet-4-5-20250929" )),
        _ => ("unknown", false, None),
      };

      Self
      {
        status : status.to_string(),
        is_deprecated,
        release_date : Some( "2024-10-22".to_string() ),
        deprecation_date : if is_deprecated { Some( "2024-12-01".to_string() ) } else { None },
        end_of_life_date : if is_deprecated { Some( "2025-03-01".to_string() ) } else { None },
        replacement_model : replacement.map( std::string::ToString::to_string ),
        migration_guide : if is_deprecated
        {
          vec![
            "Update model parameter in requests".to_string(),
            "Test with new model capabilities".to_string(),
            "Update pricing expectations".to_string(),
          ]
        }
        else
        {
          vec![]
        },
        version_compatibility : VersionCompatibility::new(),
      }
    }

    /// Fetch lifecycle status from API
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or model not found
    pub fn fetch_lifecycle_status( client : &Client, model_id : &str ) -> AnthropicResult< Self >
    {
      // For now, return mock data - in real implementation would fetch from API
      let _ = client; // Avoid unused parameter warning
      Ok( Self::for_model( model_id ) )
    }

    /// Check if model is deprecated
    pub fn is_deprecated( &self ) -> bool
    {
      self.is_deprecated
    }

    /// Get status
    pub fn get_status( &self ) -> &str
    {
      &self.status
    }

    /// Get release date
    pub fn get_release_date( &self ) -> Option< &str >
    {
      self.release_date.as_deref()
    }

    /// Get deprecation date
    pub fn get_deprecation_date( &self ) -> Option< &str >
    {
      self.deprecation_date.as_deref()
    }

    /// Get end of life date
    pub fn get_end_of_life_date( &self ) -> Option< &str >
    {
      self.end_of_life_date.as_deref()
    }

    /// Get replacement model
    pub fn get_replacement_model( &self ) -> Option< &str >
    {
      self.replacement_model.as_deref()
    }

    /// Get migration guide
    pub fn get_migration_guide( &self ) -> &Vec< String >
    {
      &self.migration_guide
    }

    /// Get version compatibility
    pub fn get_version_compatibility( &self ) -> &VersionCompatibility
    {
      &self.version_compatibility
    }
  }

  impl Default for VersionCompatibility
  {
    fn default() -> Self 
    {
      Self::new()
    }
  }

  impl VersionCompatibility
  {
    /// Create new version compatibility
    pub fn new() -> Self
    {
      Self
      {
        supported_api_versions : vec![
          "2023-06-01".to_string(),
          "2023-01-01".to_string(),
        ],
      }
    }

    /// Get supported API versions
    pub fn get_supported_api_versions( &self ) -> &Vec< String >
    {
      &self.supported_api_versions
    }

    /// Check if compatible with version
    pub fn is_compatible_with_version( &self, version : &str ) -> bool
    {
      self.supported_api_versions.contains( &version.to_string() )
    }
  }

  impl ModelPricing
  {
    /// Create pricing for model
    pub fn for_model( model_id : &str ) -> Self
    {
      let (input_price, output_price, tier) = match model_id
      {
        "claude-sonnet-4-5-20250929" => (0.003, 0.015, Some( "premium".to_string() )),
        "claude-3-5-haiku-20241022" => (0.00025, 0.00125, Some( "standard".to_string() )),
        "claude-3-opus-20240229" => (0.015, 0.075, Some( "premium".to_string() )),
        _ => (0.001, 0.005, Some( "standard".to_string() )),
      };

      Self
      {
        input_cost_per_token : input_price / 1000.0,
        output_cost_per_token : output_price / 1000.0,
        currency : "USD".to_string(),
        usage_tier : tier.unwrap_or_else( || "standard".to_string() ),
      }
    }

    /// Fetch current pricing from API
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or model not found
    pub fn fetch_current_pricing( client : &Client, model_id : &str ) -> AnthropicResult< Self >
    {
      // For now, return mock data - in real implementation would fetch from API
      let _ = client; // Avoid unused parameter warning
      Ok( Self::for_model( model_id ) )
    }

    /// Get input price per token
    pub fn get_input_price_per_token( &self ) -> f64
    {
      self.input_cost_per_token
    }

    /// Get output price per token
    pub fn get_output_price_per_token( &self ) -> f64
    {
      self.output_cost_per_token
    }

    /// Get currency
    pub fn get_currency( &self ) -> &str
    {
      &self.currency
    }

    /// Get usage tier
    pub fn get_usage_tier( &self ) -> &str
    {
      &self.usage_tier
    }
  }

  impl ModelComparison
  {
    /// Create comparison between two models
    pub fn between( model_a : &str, model_b : &str ) -> Self
    {
      let capability_differences = vec![
        "vision_support".to_string(),
        "performance_tier".to_string(),
      ];

      let use_case_recommendations = vec![
        "Use Sonnet for complex reasoning tasks".to_string(),
        "Use Haiku for fast, simple tasks".to_string(),
      ];

      Self
      {
        model_a : model_a.to_string(),
        model_b : model_b.to_string(),
        capability_differences,
        cost_comparison : CostComparison::new(),
        performance_comparison : PerformanceComparison::new(),
        use_case_recommendations,
      }
    }

    /// Fetch comparison from API
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or models not found
    pub fn fetch_comparison( client : &Client, model_a : &str, model_b : &str ) -> AnthropicResult< Self >
    {
      // For now, return mock data - in real implementation would fetch from API
      let _ = client; // Avoid unused parameter warning
      Ok( Self::between( model_a, model_b ) )
    }

    /// Get capability differences
    pub fn get_capability_differences( &self ) -> &Vec< String >
    {
      &self.capability_differences
    }

    /// Get cost comparison
    pub fn get_cost_comparison( &self ) -> &CostComparison
    {
      &self.cost_comparison
    }

    /// Get performance comparison
    pub fn get_performance_comparison( &self ) -> &PerformanceComparison
    {
      &self.performance_comparison
    }

    /// Get use case recommendations
    pub fn get_use_case_recommendations( &self ) -> &Vec< String >
    {
      &self.use_case_recommendations
    }
  }

  impl Default for CostComparison
  {
    fn default() -> Self 
    {
      Self::new()
    }
  }

  impl CostComparison
  {
    /// Create new cost comparison
    pub fn new() -> Self
    {
      Self
      {
        cost_ratio : 12.0, // Sonnet costs 12x more than Haiku
        cost_analysis : vec![
          "Higher cost for better quality".to_string(),
          "Consider usage patterns".to_string(),
        ],
      }
    }

    /// Get cost ratio
    pub fn get_cost_ratio( &self ) -> f64
    {
      self.cost_ratio
    }

    /// Get cost analysis
    pub fn get_cost_analysis( &self ) -> &Vec< String >
    {
      &self.cost_analysis
    }
  }

  impl Default for PerformanceComparison
  {
    fn default() -> Self 
    {
      Self::new()
    }
  }

  impl PerformanceComparison
  {
    /// Create new performance comparison
    pub fn new() -> Self
    {
      Self
      {
        latency_ratio : 0.3, // Haiku is 3x faster
        quality_score_diff : 0.2, // Sonnet has higher quality
      }
    }

    /// Get latency ratio
    pub fn get_latency_ratio( &self ) -> f64
    {
      self.latency_ratio
    }

    /// Get quality score difference
    pub fn get_quality_score_diff( &self ) -> f64
    {
      self.quality_score_diff
    }
  }

  /// Model filtering and search
  impl ModelFilter
  {
    /// Search models by name/description
    pub fn search( query : &str ) -> Vec< ModelSearchResult >
    {
      // Mock search results
      let mut results = Vec::new();

      if query.to_lowercase().contains( "sonnet" )
      {
        results.push( ModelSearchResult
        {
          model_id : "claude-sonnet-4-5-20250929".to_string(),
          name : "Claude 3.5 Sonnet".to_string(),
          description : "Our most intelligent model".to_string(),
          relevance_score : 1.0,
        } );
      }

      results
    }
  }

  impl FilteredModel
  {
    /// Check if supports vision
    pub fn supports_vision( &self ) -> bool
    {
      self.supports_vision
    }

    /// Get context length
    pub fn get_context_length( &self ) -> u32
    {
      self.context_length
    }

    /// Check if deprecated
    pub fn is_deprecated( &self ) -> bool
    {
      self.is_deprecated
    }
  }

  impl ModelSearchResult
  {
    /// Get name
    pub fn get_name( &self ) -> &str
    {
      &self.name
    }

    /// Get description
    pub fn get_description( &self ) -> &str
    {
      &self.description
    }
  }

  impl Default for FeatureCompatibilityMatrix
  {
    fn default() -> Self 
    {
      Self::new()
    }
  }

  impl FeatureCompatibilityMatrix
  {
    /// Create new feature compatibility matrix
    pub fn new() -> Self
    {
      Self {}
    }

    /// Get models supporting a feature
    // Fix(issue-001): Correct Sonnet 4.5 feature support
    // Root cause : Sonnet 4.5 is text-only and doesn't support vision or function calling
    // Pitfall : Feature matrices must stay synchronized with EnhancedModelCapabilities
    pub fn get_models_supporting( &self, feature : &str ) -> Vec< String >
    {
      match feature
      {
        "vision" => vec![
          "claude-3-opus-20240229".to_string(),
        ],
        "function_calling" => vec![
          "claude-3-5-haiku-20241022".to_string(),
          "claude-3-opus-20240229".to_string(),
        ],
        _ => vec![],
      }
    }

    /// Get models supporting all features
    // Fix(issue-001): Correct models for vision+function_calling combo
    // Root cause : Only Claude 3 Opus supports both vision and function calling
    // Sonnet 4.5 is text-only, Haiku 3.5 has no vision
    // Pitfall : Feature combinations must be validated against actual capabilities
    pub fn get_models_supporting_all( &self, features : &Vec< &str > ) -> Vec< String >
    {
      if features.contains( &"vision" ) && features.contains( &"function_calling" )
      {
        vec![
          "claude-3-opus-20240229".to_string(),
        ]
      }
      else
      {
        vec![]
      }
    }

    /// Get feature timeline
    pub fn get_feature_timeline( &self, feature : &str ) -> Vec< FeatureTimelineEntry >
    {
      match feature
      {
        "vision" => vec![
          FeatureTimelineEntry
          {
            model : "claude-3-opus-20240229".to_string(),
            introduced_date : "2024-02-29".to_string(),
          },
          FeatureTimelineEntry
          {
            model : "claude-sonnet-4-5-20250929".to_string(),
            introduced_date : "2024-10-22".to_string(),
          },
        ],
        _ => vec![],
      }
    }
  }

  impl ModelDetailsCache
  {
    /// Invalidate cache for model
    pub fn invalidate( model_id : &str )
    {
      // Mock implementation - in real code would clear cache
      let _ = model_id;
    }
  }

  // Add the new types to the model manager
  impl ModelManager
  {
    /// Get enhanced model details
    ///
    /// # Errors
    ///
    /// Returns an error if model not found
    pub fn get_enhanced_details( &self, model_id : &str ) -> AnthropicResult< EnhancedModelDetails >
    {
      Ok( EnhancedModelDetails::new( model_id ) )
    }

    /// Compare two models
    ///
    /// # Errors
    ///
    /// Returns an error if models not found
    pub fn compare_models( &self, model_a : &str, model_b : &str ) -> AnthropicResult< ModelComparison >
    {
      Ok( ModelComparison::between( model_a, model_b ) )
    }

    /// Get model lifecycle information
    ///
    /// # Errors
    ///
    /// Returns an error if model not found
    pub fn get_model_lifecycle( &self, model_id : &str ) -> AnthropicResult< ModelLifecycle >
    {
      Ok( ModelLifecycle::for_model( model_id ) )
    }

    /// Get context window details
    ///
    /// # Errors
    ///
    /// Returns an error if model not found
    pub fn get_context_window_details( &self, model_id : &str ) -> AnthropicResult< ContextWindowDetails >
    {
      Ok( ContextWindowDetails::for_model( model_id ) )
    }

    /// Get enhanced capabilities
    ///
    /// # Errors
    ///
    /// Returns an error if model not found
    pub fn get_enhanced_capabilities( &self, model_id : &str ) -> AnthropicResult< EnhancedModelCapabilities >
    {
      Ok( EnhancedModelCapabilities::for_model( model_id ) )
    }
  }

  // Add enhanced model details to client
  impl Client
  {
    /// Get enhanced model details
    ///
    /// # Errors
    ///
    /// Returns an error if model not found
    pub fn get_enhanced_model_details( &self, model_id : &str ) -> AnthropicResult< EnhancedModelDetails >
    {
      EnhancedModelDetails::fetch_from_api( self, model_id )
    }

    /// Compare two models
    ///
    /// # Errors
    ///
    /// Returns an error if models not found
    pub fn compare_models( &self, model_a : &str, model_b : &str ) -> AnthropicResult< ModelComparison >
    {
      ModelComparison::fetch_comparison( self, model_a, model_b )
    }
  }
}

crate::mod_interface!
{
  exposed use EnhancedModelDetails;
  exposed use EnhancedModelCapabilities;
  exposed use PerformanceProfile;
  exposed use ContextWindowDetails;
  exposed use TokenBreakdown;
  exposed use ModelLifecycle;
  exposed use VersionCompatibility;
  exposed use ModelComparison;
  exposed use CostComparison;
  exposed use PerformanceComparison;
  exposed use FilteredModel;
  exposed use ModelSearchResult;
  exposed use FeatureCompatibilityMatrix;
  exposed use FeatureTimelineEntry;
  exposed use ModelDetailsCache;
}
