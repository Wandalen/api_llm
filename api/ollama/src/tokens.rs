//! Token counting functionality for the Ollama API client
//!
//! This module provides comprehensive token counting capabilities including:
//! - Token estimation for text inputs before API calls
//! - Cost calculation based on token counts
//! - Input validation using token limits
//! - Batch operation optimization based on token counts
//!
//! All functionality follows the "Thin Client, Rich API" governing principle,
//! providing explicit control with transparent API mapping to Ollama endpoints.

use serde::{ Serialize, Deserialize };

/// Request structure for counting tokens in text input
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct TokenCountRequest
{
  /// Model name to use for token counting (e.g., "llama3.2")
  pub model : String,
  /// Text to count tokens for
  pub text : String,
  /// Additional tokenization options
  pub options : Option< serde_json::Value >,
}

/// Response structure for token counting results
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct TokenCountResponse
{
  /// Number of tokens in the input text
  pub token_count : u32,
  /// Model used for token counting
  pub model : String,
  /// Length of input text in characters
  pub text_length : usize,
  /// Estimated cost for processing this many tokens
  pub estimated_cost : Option< f64 >,
  /// Time taken to count tokens in milliseconds
  pub processing_time_ms : Option< u64 >,
  /// Additional metadata from token counting
  pub metadata : Option< serde_json::Value >,
}

/// Cost estimation structure based on token counts
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct CostEstimation
{
  /// Number of input tokens
  pub input_tokens : u32,
  /// Estimated number of output tokens
  pub estimated_output_tokens : u32,
  /// Cost per input token
  pub input_cost_per_token : f64,
  /// Cost per output token
  pub output_cost_per_token : f64,
  /// Total estimated cost for the operation
  pub total_estimated_cost : f64,
  /// Currency for cost calculation
  pub currency : String,
  /// Model name for cost calculation
  pub model : String,
}

/// Request structure for batch token counting
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct BatchTokenRequest
{
  /// Model name to use for all token counting
  pub model : String,
  /// List of texts to count tokens for
  pub texts : Vec< String >,
  /// Additional tokenization options
  pub options : Option< serde_json::Value >,
  /// Whether to include cost estimation in results
  pub estimate_costs : bool,
}

/// Response structure for batch token counting
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct BatchTokenResponse
{
  /// Individual token count results for each text
  pub results : Vec< TokenCountResponse >,
  /// Total tokens across all texts
  pub total_tokens : u32,
  /// Total estimated cost for all texts
  pub total_estimated_cost : Option< f64 >,
  /// Total processing time in milliseconds
  pub processing_time_ms : Option< u64 >,
  /// Optimization savings from batch processing (percentage)
  pub batch_optimization_savings : Option< f64 >,
}

/// Configuration for token validation and limits
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct TokenValidationConfig
{
  /// Maximum allowed input tokens
  pub max_input_tokens : u32,
  /// Maximum allowed output tokens
  pub max_output_tokens : u32,
  /// Model's context window size
  pub model_context_window : u32,
  /// Warning threshold as percentage of limit (0.0 to 1.0)
  pub warning_threshold : f64,
  /// Whether to enforce limits strictly
  pub enforce_limits : bool,
  /// Strategy for text truncation : "start", "end", "middle"
  pub truncation_strategy : String,
}

/// Model-specific token counting capabilities and costs
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ModelTokenCapabilities
{
  /// Model name
  pub model_name : String,
  /// Context window size in tokens
  pub context_window : u32,
  /// Whether model supports function calling
  pub supports_function_calling : bool,
  /// Average tokens per word for this model
  pub average_tokens_per_word : f64,
  /// Maximum input tokens for this model
  pub max_input_tokens : u32,
  /// Maximum output tokens for this model
  pub max_output_tokens : u32,
  /// Cost per input token
  pub cost_per_input_token : f64,
  /// Cost per output token
  pub cost_per_output_token : f64,
  /// Tokenizer type used by this model
  pub tokenizer_type : String,
}

impl TokenCountRequest
{
  /// Create a new token count request
  #[ inline ]
  #[ must_use ]
  pub fn new( model : String, text : String ) -> Self
  {
    Self
    {
      model,
      text,
      options : None,
    }
  }

  /// Create a token count request with options
  #[ inline ]
  #[ must_use ]
  pub fn with_options( mut self, options : serde_json::Value ) -> Self
  {
    self.options = Some( options );
    self
  }

  /// Get the estimated token count using simple heuristics
  /// This is a rough estimate : typically 1 token per 4 characters for English text
  #[ inline ]
  #[ must_use ]
  pub fn estimate_tokens( &self ) -> u32
  {
    // Simple heuristic : 1 token per 4 characters, minimum 1 token
    ( self.text.len() / 4 ).max( 1 ) as u32
  }
}

impl CostEstimation
{
  /// Create a new cost estimation
  #[ inline ]
  #[ must_use ]
  pub fn new(
    input_tokens : u32,
    estimated_output_tokens : u32,
    input_cost_per_token : f64,
    output_cost_per_token : f64,
    model : String,
  ) -> Self
  {
    let total_estimated_cost = ( input_tokens as f64 * input_cost_per_token ) +
                              ( estimated_output_tokens as f64 * output_cost_per_token );

    Self
    {
      input_tokens,
      estimated_output_tokens,
      input_cost_per_token,
      output_cost_per_token,
      total_estimated_cost,
      currency : "USD".to_string(),
      model,
    }
  }

  /// Set the currency for cost calculation
  #[ inline ]
  #[ must_use ]
  pub fn with_currency( mut self, currency : String ) -> Self
  {
    self.currency = currency;
    self
  }

  /// Calculate cost savings percentage compared to another estimation
  #[ inline ]
  #[ must_use ]
  pub fn calculate_savings( &self, other : &CostEstimation ) -> f64
  {
    if other.total_estimated_cost == 0.0
    {
      return 0.0;
    }

    ( ( other.total_estimated_cost - self.total_estimated_cost ) / other.total_estimated_cost ) * 100.0
  }
}

impl TokenValidationConfig
{
  /// Create a new token validation configuration with defaults
  #[ inline ]
  #[ must_use ]
  pub fn new( max_input_tokens : u32, max_output_tokens : u32, model_context_window : u32 ) -> Self
  {
    Self
    {
      max_input_tokens,
      max_output_tokens,
      model_context_window,
      warning_threshold : 0.8,
      enforce_limits : true,
      truncation_strategy : "end".to_string(),
    }
  }

  /// Set the warning threshold
  #[ inline ]
  #[ must_use ]
  pub fn with_warning_threshold( mut self, threshold : f64 ) -> Self
  {
    self.warning_threshold = threshold.clamp( 0.0, 1.0 );
    self
  }

  /// Set whether to enforce limits
  #[ inline ]
  #[ must_use ]
  pub fn with_enforcement( mut self, enforce : bool ) -> Self
  {
    self.enforce_limits = enforce;
    self
  }

  /// Set the truncation strategy
  #[ inline ]
  #[ must_use ]
  pub fn with_truncation_strategy( mut self, strategy : String ) -> Self
  {
    self.truncation_strategy = strategy;
    self
  }

  /// Check if token count exceeds warning threshold
  #[ inline ]
  #[ must_use ]
  pub fn exceeds_warning_threshold( &self, token_count : u32 ) -> bool
  {
    token_count as f64 > ( self.max_input_tokens as f64 * self.warning_threshold )
  }

  /// Check if token count exceeds maximum limit
  #[ inline ]
  #[ must_use ]
  pub fn exceeds_limit( &self, token_count : u32 ) -> bool
  {
    token_count > self.max_input_tokens
  }
}

impl ModelTokenCapabilities
{
  /// Create model capabilities for a standard chat model
  #[ inline ]
  #[ must_use ]
  pub fn chat_model( model_name : String, context_window : u32 ) -> Self
  {
    Self
    {
      model_name,
      context_window,
      supports_function_calling : true,
      average_tokens_per_word : 1.3,
      max_input_tokens : ( context_window as f64 * 0.75 ) as u32, // 75% for input
      max_output_tokens : ( context_window as f64 * 0.25 ) as u32, // 25% for output
      cost_per_input_token : 0.0001,
      cost_per_output_token : 0.0002,
      tokenizer_type : "tiktoken".to_string(),
    }
  }

  /// Create model capabilities for a code model
  #[ inline ]
  #[ must_use ]
  pub fn code_model( model_name : String, context_window : u32 ) -> Self
  {
    Self
    {
      model_name,
      context_window,
      supports_function_calling : false,
      average_tokens_per_word : 1.5, // Code typically has more tokens per word
      max_input_tokens : ( context_window as f64 * 0.8 ) as u32, // 80% for input
      max_output_tokens : ( context_window as f64 * 0.2 ) as u32, // 20% for output
      cost_per_input_token : 0.00015,
      cost_per_output_token : 0.0003,
      tokenizer_type : "sentencepiece".to_string(),
    }
  }

  /// Estimate tokens for given text using model-specific average
  #[ inline ]
  #[ must_use ]
  pub fn estimate_tokens( &self, text : &str ) -> u32
  {
    let word_count = text.split_whitespace().count() as f64;
    ( word_count * self.average_tokens_per_word ).ceil() as u32
  }

  /// Calculate cost for given token counts
  #[ inline ]
  #[ must_use ]
  pub fn calculate_cost( &self, input_tokens : u32, output_tokens : u32 ) -> f64
  {
    ( input_tokens as f64 * self.cost_per_input_token ) +
    ( output_tokens as f64 * self.cost_per_output_token )
  }
}