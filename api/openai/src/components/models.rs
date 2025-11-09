//! Structures related to model information.

/// Define a private namespace for all its items.
mod private
{
  // Serde imports
  use serde::{ Serialize, Deserialize }; // Added Serialize

  /// Describes an `OpenAI` model offering that can be used with the API.
  ///
  /// # Used By
  /// - `/models` (GET - in `ListModelsResponse`)
  /// - `/models/{model}` (GET)
  /// - `DeleteModelResponse` (within `common.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct Model
  {
    /// The model identifier, which can be referenced in the API endpoints.
    pub id : String,
    /// The Unix timestamp (in seconds) when the model was created.
    pub created : i64,
    /// The object type, which is always "model".
    pub object : String,
    /// The organization that owns the model.
    pub owned_by : String,
  }

  /// Enhanced model with comprehensive metadata including pricing and capabilities.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct EnhancedModel
  {
    /// The model identifier, which can be referenced in the API endpoints.
    pub id : String,
    /// The Unix timestamp (in seconds) when the model was created.
    pub created : i64,
    /// The object type, which is always "model".
    pub object : String,
    /// The organization that owns the model.
    pub owned_by : String,
    /// Pricing information for the model (if available).
    pub pricing : Option< ModelPricing >,
    /// Model capabilities and technical specifications.
    pub capabilities : ModelCapabilities,
    /// Model limitations and rate limits.
    pub limitations : ModelLimitations,
    /// Model lifecycle and deprecation information.
    pub lifecycle : ModelLifecycle,
  }

  /// Pricing information for a model.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct ModelPricing
  {
    /// Cost per 1,000 input tokens.
    pub input_cost_per_1k_tokens : f64,
    /// Cost per 1,000 output tokens.
    pub output_cost_per_1k_tokens : f64,
    /// Currency for pricing (typically "USD").
    pub currency : String,
    /// Date when this pricing became effective.
    pub effective_date : String,
  }

  impl ModelPricing
  {
    /// Calculate the cost for a given number of input and output tokens.
    #[ inline ]
    #[ must_use ]
    pub fn calculate_cost( &self, input_tokens : u32, output_tokens : u32 ) -> f64
    {
      ( f64::from( input_tokens ) / 1000.0 ) * self.input_cost_per_1k_tokens +
      ( f64::from( output_tokens ) / 1000.0 ) * self.output_cost_per_1k_tokens
    }
  }

  /// Model capabilities and technical specifications.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct ModelCapabilities
  {
    /// Whether the model supports function calling.
    pub supports_function_calling : bool,
    /// Whether the model supports vision/image input.
    pub supports_vision : bool,
    /// Whether the model supports streaming responses.
    pub supports_streaming : bool,
    /// Maximum context window size in tokens.
    pub max_context_window : u32,
    /// Maximum output tokens in a single response.
    pub max_output_tokens : u32,
    /// List of supported input/output formats.
    pub supported_formats : Vec< String >,
  }

  /// Model limitations and rate limits.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct ModelLimitations
  {
    /// Rate limit in requests per minute.
    pub rate_limit_rpm : Option< u32 >,
    /// Rate limit in tokens per minute.
    pub rate_limit_tpm : Option< u32 >,
    /// Maximum concurrent requests allowed.
    pub concurrent_requests : Option< u32 >,
  }

  /// Model lifecycle and deprecation information.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct ModelLifecycle
  {
    /// Current status of the model.
    pub status : ModelStatus,
    /// Date when the model was deprecated (if applicable).
    pub deprecation_date : Option< String >,
    /// Date when the model will be sunset/removed (if applicable).
    pub sunset_date : Option< String >,
    /// ID of the replacement model (if applicable).
    pub replacement_model : Option< String >,
  }

  /// Enumeration of possible model statuses.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub enum ModelStatus
  {
    /// Model is actively supported and available.
    Active,
    /// Model is in beta/preview status.
    Beta,
    /// Model is deprecated but still available.
    Deprecated,
    /// Model has been sunset and is no longer available.
    Sunset,
  }

  impl core::fmt::Display for ModelStatus
  {
    #[ inline ]
    fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
    {
      match self
      {
        ModelStatus::Active => write!( f, "active" ),
        ModelStatus::Beta => write!( f, "beta" ),
        ModelStatus::Deprecated => write!( f, "deprecated" ),
        ModelStatus::Sunset => write!( f, "sunset" ),
      }
    }
  }

  /// Response containing a list of available models.
  ///
  /// # Used By
  /// - `/models` (GET)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct ListModelsResponse
  {
    /// The object type, which is always "list".
    pub object : String,
    /// A list of model objects.
    pub data : Vec< Model >,
  }

  /// Enhanced response containing a list of models with comprehensive metadata.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct EnhancedListModelsResponse
  {
    /// The object type, which is always "list".
    pub object : String,
    /// A list of enhanced model objects.
    pub data : Vec< EnhancedModel >,
    /// Response metadata with aggregated information.
    pub metadata : ResponseMetadata,
  }

  /// Metadata about the response containing aggregated model statistics.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct ResponseMetadata
  {
    /// Total number of models in the response.
    pub total_models : u32,
    /// Number of active models.
    pub active_models : u32,
    /// Number of deprecated models.
    pub deprecated_models : u32,
    /// Number of beta models.
    pub beta_models : u32,
  }

  /// Comparison between two models for analysis and decision-making.
  #[ derive( Debug, Clone ) ]
  pub struct ModelComparison
  {
    /// Ratio of context windows (`model_a` / `model_b`).
    pub context_window_ratio : f64,
    /// Cost efficiency ratio (`cost_a` / `cost_b`).
    pub cost_efficiency_ratio : f64,
    /// Difference in capability scores (`score_a` - `score_b`).
    pub capability_score_diff : i32,
  }

  impl ModelComparison
  {
    /// Compare two enhanced models and return comparison metrics.
    #[ inline ]
    #[ must_use ]
    pub fn compare( model_a : &EnhancedModel, model_b : &EnhancedModel ) -> Self
    {
      let context_ratio = f64::from( model_a.capabilities.max_context_window ) /
                         f64 ::from( model_b.capabilities.max_context_window );

      let cost_ratio = if let ( Some( pricing_a ), Some( pricing_b ) ) = ( &model_a.pricing, &model_b.pricing )
      {
        pricing_a.input_cost_per_1k_tokens / pricing_b.input_cost_per_1k_tokens
      }
      else
      {
        1.0
      };

      let score_a = calculate_capability_score( &model_a.capabilities );
      let score_b = calculate_capability_score( &model_b.capabilities );

      Self
      {
        context_window_ratio : context_ratio,
        cost_efficiency_ratio : cost_ratio,
        capability_score_diff : score_a - score_b,
      }
    }
  }

  /// Calculate a capability score for a model based on its features.
  fn calculate_capability_score( capabilities : &ModelCapabilities ) -> i32
  {
    let mut score = 0;
    if capabilities.supports_function_calling { score += 1; }
    if capabilities.supports_vision { score += 1; }
    if capabilities.supports_streaming { score += 1; }
    score
  }
} // end mod private

crate ::mod_interface!
{
  exposed use
  {
    Model,
    ListModelsResponse,
    EnhancedModel,
    EnhancedListModelsResponse,
    ModelPricing,
    ModelCapabilities,
    ModelLimitations,
    ModelLifecycle,
    ModelStatus,
    ResponseMetadata,
    ModelComparison,
  };
}