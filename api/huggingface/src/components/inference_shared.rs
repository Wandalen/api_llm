//! Shared components for `HuggingFace` inference API.

use serde::{ Deserialize, Serialize };
use super::input::InferenceParameters;
use super::output::InferenceOutput;

/// Chat message for the new Router API
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ChatMessage
{
  /// Role of the message sender (user, assistant, system, tool)
  pub role : String,

  /// Content of the message
  pub content : String,

  /// Tool calls made by the assistant (only for role="assistant")
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub tool_calls : Option< Vec< ToolCall > >,

  /// ID of the tool call this message is responding to (only for role="tool")
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub tool_call_id : Option< String >,
}

/// A tool call made by the model
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ToolCall
{
  /// Unique identifier for this tool call
  pub id : String,

  /// Type of tool (always "function" for now)
  #[ serde( rename = "type" ) ]
  pub tool_type : String,

  /// Function call details
  pub function : FunctionCall,
}

/// Function call details
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct FunctionCall
{
  /// Name of the function to call
  pub name : String,

  /// JSON string of function arguments
  pub arguments : String,
}

/// Chat completions request (new Router API format)
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ChatCompletionRequest
{
  /// Array of messages in the conversation
  pub messages : Vec< ChatMessage >,

  /// Model identifier
  pub model : String,

  /// Temperature for randomness (0.0 - 2.0)
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub temperature : Option< f32 >,

  /// Maximum tokens to generate
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub max_tokens : Option< u32 >,

  /// Top-p nucleus sampling
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub top_p : Option< f32 >,

  /// Whether to stream the response
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub stream : Option< bool >,

  /// List of tools the model may call
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub tools : Option< Vec< ToolDefinition > >,

  /// Controls which (if any) tool is called by the model
  /// - "auto": model decides whether to call tools (default)
  /// - "none": model will not call any tools
  /// - "required": model must call one or more tools
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub tool_choice : Option< String >,
}

/// Tool definition for function calling
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ToolDefinition
{
  /// Type of tool (always "function")
  #[ serde( rename = "type" ) ]
  pub tool_type : String,

  /// Function definition
  pub function : crate::components::tools::Tool,
}

/// Chat completions response (new Router API format)
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ChatCompletionResponse
{
  /// Unique identifier for the completion
  pub id : String,

  /// Object type (always "chat.completion")
  pub object : String,

  /// Unix timestamp of creation
  pub created : i64,

  /// Model used for completion
  pub model : String,

  /// Array of completion choices
  pub choices : Vec< ChatChoice >,

  /// Token usage information
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub usage : Option< ChatUsage >,

  /// System fingerprint (optional, ignored)
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub system_fingerprint : Option< String >,

  /// Service tier (optional, ignored)
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub service_tier : Option< String >,

  /// Usage breakdown (optional, ignored)
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub usage_breakdown : Option< serde_json::Value >,

  /// Provider-specific data (optional, ignored)
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub x_groq : Option< serde_json::Value >,
}

/// Individual completion choice
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ChatChoice
{
  /// Index of this choice
  pub index : u32,

  /// The generated message
  pub message : ChatMessage,

  /// Reason for completion finishing
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub finish_reason : Option< String >,

  /// Logprobs (optional, ignored)
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub logprobs : Option< serde_json::Value >,
}

/// Token usage statistics
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ChatUsage
{
  /// Tokens in the prompt
  pub prompt_tokens : u32,

  /// Tokens in the completion
  pub completion_tokens : u32,

  /// Total tokens used
  pub total_tokens : u32,

  /// Queue time (optional, provider-specific)
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub queue_time : Option< f64 >,

  /// Prompt processing time (optional, provider-specific)
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub prompt_time : Option< f64 >,

  /// Completion generation time (optional, provider-specific)
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub completion_time : Option< f64 >,

  /// Total time (optional, provider-specific)
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub total_time : Option< f64 >,
}

/// Request for text generation inference
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct InferenceRequest
{
  /// Input text or prompt
  pub inputs : String,
  
  /// Inference parameters
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub parameters : Option< InferenceParameters >,
  
  /// Options for the request
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub options : Option< InferenceOptions >,
}

impl InferenceRequest
{
  /// Create a new inference request
  #[ inline ]
  #[ must_use ]
  pub fn new( inputs : impl Into< String > ) -> Self
  {
  Self
  {
      inputs : inputs.into(),
      parameters : None,
      options : None,
  }
  }
  
  /// Set parameters
  #[ inline ]
  #[ must_use ]
  pub fn with_parameters( mut self, parameters : InferenceParameters ) -> Self
  {
  self.parameters = Some( parameters );
  self
  }
  
  /// Set options
  #[ inline ]
  #[ must_use ]
  pub fn with_options( mut self, options : InferenceOptions ) -> Self
  {
  self.options = Some( options );
  self
  }
}

/// Batch inference request for multiple inputs
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct BatchInferenceRequest
{
  /// Input texts or prompts
  pub inputs : Vec< String >,
  
  /// Inference parameters
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub parameters : Option< InferenceParameters >,
  
  /// Options for the request
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub options : Option< InferenceOptions >,
}

impl BatchInferenceRequest
{
  /// Create a new batch inference request
  #[ inline ]
  #[ must_use ]
  pub fn new( inputs : Vec< String > ) -> Self
  {
  Self
  {
      inputs,
      parameters : None,
      options : None,
  }
  }
  
  /// Set parameters
  #[ inline ]
  #[ must_use ]
  pub fn with_parameters( mut self, parameters : InferenceParameters ) -> Self
  {
  self.parameters = Some( parameters );
  self
  }
}

/// Options for inference requests
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct InferenceOptions
{
  /// Use cache
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub use_cache : Option< bool >,
  
  /// Wait for model to load
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub wait_for_model : Option< bool >,
  
  /// Use GPU if available
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub use_gpu : Option< bool >,
}

impl Default for InferenceOptions
{
  #[ inline ]
  fn default() -> Self
  {
  Self::recommended()
  }
}

impl InferenceOptions
{
  /// Create new inference options with HuggingFace-recommended values.
  ///
  /// # Governing Principle Compliance
  ///
  /// This provides HuggingFace-recommended options without making them implicit defaults.
  /// Developers must explicitly choose to use these recommended values.
  #[ inline ]
  #[ must_use ]
  pub fn recommended() -> Self
  {
  Self
  {
      use_cache : Some( true ),      // Enable caching for better performance
      wait_for_model : Some( true ), // Wait for model loading if needed
      use_gpu : Some( true ),        // Use GPU acceleration when available
  }
  }

  /// Create new inference options (convenience wrapper)
  ///
  /// # Compatibility
  ///
  /// This method provides backward compatibility by delegating to `recommended()`.
  /// For explicit control, use `recommended()`, `empty()`, or `conservative()`.
  #[ inline ]
  #[ must_use ]
  pub fn new() -> Self
  {
  Self::recommended()
  }

  /// Create empty inference options requiring explicit configuration
  ///
  /// # Governing Principle Compliance
  ///
  /// This requires explicit configuration for all options, providing full transparency
  /// and control over inference behavior.
  #[ inline ]
  #[ must_use ]
  pub fn empty() -> Self
  {
  Self
  {
      use_cache : None,
      wait_for_model : None,
      use_gpu : None,
  }
  }

  /// Create conservative options for production environments
  #[ inline ]
  #[ must_use ]
  pub fn conservative() -> Self
  {
  Self
  {
      use_cache : Some( false ),     // Disable caching for consistent results
      wait_for_model : Some( false ), // Fail fast if model not available
      use_gpu : Some( false ),       // Use CPU for predictable performance
  }
  }
  
  /// Set `use_cache` option
  #[ inline ]
  #[ must_use ]
  pub fn with_use_cache( mut self, use_cache : bool ) -> Self
  {
  self.use_cache = Some( use_cache );
  self
  }
  
  /// Set `wait_for_model` option
  #[ inline ]
  #[ must_use ]
  pub fn with_wait_for_model( mut self, wait_for_model : bool ) -> Self
  {
  self.wait_for_model = Some( wait_for_model );
  self
  }
  
  /// Set `use_gpu` option
  #[ inline ]
  #[ must_use ]
  pub fn with_use_gpu( mut self, use_gpu : bool ) -> Self
  {
  self.use_gpu = Some( use_gpu );
  self
  }
}

/// Response wrapper for inference operations
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( untagged ) ]
pub enum InferenceResponse
{
  /// Single inference output
  Single( InferenceOutput ),
  /// Multiple inference outputs
  Batch( Vec< InferenceOutput > ),
  /// Summarization response
  Summarization( Vec< SummarizationOutput > ),
}

/// Summarization model response
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct SummarizationOutput
{
  /// Summarized text
  pub summary_text : String,
}

impl InferenceResponse
{
  /// Extract text content from any inference response type
  /// 
  /// Returns the first available text from the response, regardless of type
  #[ inline ]
  #[ must_use ]
  pub fn extract_text( &self ) -> Option< String >
  {
  match self
  {
      Self::Single( output ) => Some( output.generated_text.clone() ),
      Self::Batch( outputs ) => outputs.first().map( | o | o.generated_text.clone() ),
      Self::Summarization( summaries ) => summaries.first().map( | s | s.summary_text.clone() ),
  }
  }
  
  /// Extract text content with fallback message
  /// 
  /// Returns the text content or a default fallback message if no content is available
  #[ inline ]
  #[ must_use ]
  pub fn extract_text_or_default( &self, default : &str ) -> String
  {
  self.extract_text().unwrap_or_else( || default.to_string() )
  }
}