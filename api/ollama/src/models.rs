//! Request and response models for Ollama API client.

#[ cfg( feature = "enabled" ) ]
mod private
{
  use serde::{ Serialize, Deserialize };
  use std::collections::HashMap;

  // =====================================
  // Core Message Types
  // =====================================

  /// A message in a conversation
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct Message
  {
    /// The role of the message sender
    pub role : MessageRole,
    /// The content of the message
    pub content : String,
    /// Optional images for multimodal models
    #[ cfg( feature = "vision_support" ) ]
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub images : Option< Vec< String > >,
  }

  /// Role of the message sender in a conversation
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub enum MessageRole
  {
    /// Message from the user
    #[ serde( rename = "user" ) ]
    User,
    /// Message from the AI assistant
    #[ serde( rename = "assistant" ) ]
    Assistant,
    /// System message for context
    #[ serde( rename = "system" ) ]
    System,
  }

  /// A chat message with extended properties
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ChatMessage
  {
    /// The role of the message sender
    pub role : MessageRole,
    /// The content of the message
    pub content : String,
    /// Optional images for vision models
    #[ cfg( feature = "vision_support" ) ]
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub images : Option< Vec< String > >,
    /// Optional tool calls for function calling models
    #[ cfg( feature = "tool_calling" ) ]
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tool_calls : Option< Vec< ToolCall > >,
  }

  /// Tool call for function calling models
  #[ cfg( feature = "tool_calling" ) ]
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ToolCall
  {
    /// Tool function name
    pub function : String,
    /// Tool arguments as JSON
    pub arguments : serde_json::Value,
  }

  // =====================================
  // Chat API Types
  // =====================================

  /// Request for chat completions
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ChatRequest
  {
    /// Model name to use for the chat
    pub model : String,
    /// List of messages in the conversation
    pub messages : Vec< ChatMessage >,
    /// Enable streaming response
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub stream : Option< bool >,
    /// Sampling temperature (0.0 to 2.0)
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub temperature : Option< f32 >,
    /// Top-p nucleus sampling (0.0 to 1.0)
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub top_p : Option< f32 >,
    /// Top-k sampling (integer)
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub top_k : Option< u32 >,
    /// Maximum tokens to generate
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub max_tokens : Option< u32 >,
    /// Stop sequences
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub stop : Option< Vec< String > >,
    /// System prompt
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub system : Option< String >,
    /// Chat template
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub template : Option< String >,
    /// Additional options
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub options : Option< HashMap<  String, serde_json::Value  > >,
    /// Format for the response
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub format : Option< String >,
    /// Whether to keep the model loaded
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub keep_alive : Option< String >,
  }

  /// Response from chat completions
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ChatResponse
  {
    /// Model that generated the response
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub model : Option< String >,
    /// Creation timestamp
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub created_at : Option< String >,
    /// The generated message
    pub message : ChatMessage,
    /// Whether the response is complete
    pub done : bool,
    /// Reason for completion
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub done_reason : Option< String >,
    /// Total processing duration in nanoseconds
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub total_duration : Option< u64 >,
    /// Model loading duration in nanoseconds
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub load_duration : Option< u64 >,
    /// Number of tokens in prompt
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub prompt_eval_count : Option< u32 >,
    /// Prompt evaluation duration in nanoseconds
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub prompt_eval_duration : Option< u64 >,
    /// Number of tokens in response
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub eval_count : Option< u32 >,
    /// Response generation duration in nanoseconds
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub eval_duration : Option< u64 >,
  }

  // =====================================
  // Generate API Types
  // =====================================

  /// Request for text generation
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct GenerateRequest
  {
    /// Model name to use for generation
    pub model : String,
    /// Input prompt
    pub prompt : String,
    /// Enable streaming response
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub stream : Option< bool >,
    /// System prompt
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub system : Option< String >,
    /// Generation template
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub template : Option< String >,
    /// Context from previous generation
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub context : Option< Vec< u8 > >,
    /// Additional generation options
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub options : Option< HashMap<  String, serde_json::Value  > >,
    /// Response format
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub format : Option< String >,
    /// Keep alive duration
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub keep_alive : Option< String >,
    /// Images for multimodal models
    #[ cfg( feature = "vision_support" ) ]
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub images : Option< Vec< String > >,
  }

  /// Response from text generation
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct GenerateResponse
  {
    /// Model that generated the response
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub model : Option< String >,
    /// Creation timestamp
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub created_at : Option< String >,
    /// Generated response text
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub response : Option< String >,
    /// Whether generation is complete
    #[ serde( default ) ]
    pub done : bool,
    /// Context for next generation
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub context : Option< Vec< u8 > >,
    /// Total processing duration in nanoseconds
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub total_duration : Option< u64 >,
    /// Model loading duration in nanoseconds
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub load_duration : Option< u64 >,
    /// Number of tokens in prompt
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub prompt_eval_count : Option< u32 >,
    /// Prompt evaluation duration in nanoseconds
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub prompt_eval_duration : Option< u64 >,
    /// Number of tokens in response
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub eval_count : Option< u32 >,
    /// Response generation duration in nanoseconds
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub eval_duration : Option< u64 >,
  }

  // =====================================
  // Model Management Types
  // =====================================

  /// Information about a model
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ModelInfo
  {
    /// Model name
    pub name : String,
    /// Model size in bytes
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub size : Option< u64 >,
    /// Model family/architecture
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub family : Option< String >,
    /// Parameter count
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub parameter_size : Option< String >,
    /// Quantization level
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub quantization_level : Option< String >,
    /// Model details
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub details : Option< ModelDetails >,
    /// Model digest/hash
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub digest : Option< String >,
    /// Modified timestamp
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub modified_at : Option< String >,
  }

  /// Detailed model information
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ModelDetails
  {
    /// Model format (e.g., "gguf")
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub format : Option< String >,
    /// Model family
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub family : Option< String >,
    /// Model families
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub families : Option< Vec< String > >,
    /// Parameter size
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub parameter_size : Option< String >,
    /// Quantization level
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub quantization_level : Option< String >,
  }

  /// Model entry in the list
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ModelEntry
  {
    /// Model name
    pub name : String,
    /// Model digest
    pub digest : String,
    /// Model size in bytes
    pub size : u64,
    /// Modified timestamp
    pub modified_at : String,
    /// Model details
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub details : Option< ModelDetails >,
  }

  /// Response from listing models
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct TagsResponse
  {
    /// List of available models
    pub models : Vec< ModelEntry >,
  }

  /// Request to show model information
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ShowModelRequest
  {
    /// Model name to show information for
    pub name : String,
    /// Whether to include verbose details
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub verbose : Option< bool >,
  }

  /// Request to pull a model
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct PullModelRequest
  {
    /// Model name to pull
    pub name : String,
    /// Enable insecure connections
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub insecure : Option< bool >,
    /// Enable streaming progress
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub stream : Option< bool >,
  }

  /// Request to push a model
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct PushModelRequest
  {
    /// Model name to push
    pub name : String,
    /// Enable insecure connections
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub insecure : Option< bool >,
    /// Enable streaming progress
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub stream : Option< bool >,
  }

  /// Request to delete a model
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct DeleteModelRequest
  {
    /// Model name to delete
    pub name : String,
  }

  // =====================================
  // Embeddings API Types
  // =====================================

  /// Request for generating embeddings
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct EmbeddingsRequest
  {
    /// Model name to use for embeddings
    pub model : String,
    /// Input text or texts
    pub prompt : String,
    /// Additional options
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub options : Option< HashMap<  String, serde_json::Value  > >,
    /// Keep alive duration
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub keep_alive : Option< String >,
  }

  /// Response from embeddings generation
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct EmbeddingsResponse
  {
    /// Generated embedding vectors
    pub embedding : Vec< f64 >,
  }

  // =====================================
  // Utility Types
  // =====================================

  /// Progress information for model operations
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ProgressInfo
  {
    /// Status message
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub status : Option< String >,
    /// Progress digest
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub digest : Option< String >,
    /// Total size in bytes
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub total : Option< u64 >,
    /// Completed size in bytes
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub completed : Option< u64 >,
  }

  /// Error information from API responses
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ApiError
  {
    /// Error message
    pub error : String,
  }

  // =====================================
  // Implementation blocks
  // =====================================

  impl ChatRequest
  {
    /// Create a new chat request with the given model and messages
    #[ inline ]
    #[ must_use ]
    pub fn new( model : String, messages : Vec< ChatMessage > ) -> Self
    {
      Self
      {
        model,
        messages,
        stream : None,
        temperature : None,
        top_p : None,
        top_k : None,
        max_tokens : None,
        stop : None,
        system : None,
        template : None,
        options : None,
        format : None,
        keep_alive : None,
      }
    }

    /// Set streaming mode
    #[ inline ]
    #[ must_use ]
    pub fn with_stream( mut self, stream : bool ) -> Self
    {
      self.stream = Some( stream );
      self
    }

    /// Set temperature for sampling
    #[ inline ]
    #[ must_use ]
    pub fn with_temperature( mut self, temperature : f32 ) -> Self
    {
      self.temperature = Some( temperature );
      self
    }

    /// Set system prompt
    #[ inline ]
    #[ must_use ]
    pub fn with_system( mut self, system : String ) -> Self
    {
      self.system = Some( system );
      self
    }
  }

  impl GenerateRequest
  {
    /// Create a new generate request with the given model and prompt
    #[ inline ]
    #[ must_use ]
    pub fn new( model : String, prompt : String ) -> Self
    {
      Self
      {
        model,
        prompt,
        stream : None,
        system : None,
        template : None,
        context : None,
        options : None,
        format : None,
        keep_alive : None,
        #[ cfg( feature = "vision_support" ) ]
        images : None,
      }
    }

    /// Set streaming mode
    #[ inline ]
    #[ must_use ]
    pub fn with_stream( mut self, stream : bool ) -> Self
    {
      self.stream = Some( stream );
      self
    }

    /// Set system prompt
    #[ inline ]
    #[ must_use ]
    pub fn with_system( mut self, system : String ) -> Self
    {
      self.system = Some( system );
      self
    }

    /// Set context from previous generation
    #[ inline ]
    #[ must_use ]
    pub fn with_context( mut self, context : Vec< u8 > ) -> Self
    {
      self.context = Some( context );
      self
    }
  }

  impl ChatMessage
  {
    /// Create a new user message
    #[ inline ]
    #[ must_use ]
    pub fn user( content : String ) -> Self
    {
      Self
      {
        role : MessageRole::User,
        content,
        #[ cfg( feature = "vision_support" ) ]
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      }
    }

    /// Create a new assistant message
    #[ inline ]
    #[ must_use ]
    pub fn assistant( content : String ) -> Self
    {
      Self
      {
        role : MessageRole::Assistant,
        content,
        #[ cfg( feature = "vision_support" ) ]
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      }
    }

    /// Create a new system message
    #[ inline ]
    #[ must_use ]
    pub fn system( content : String ) -> Self
    {
      Self
      {
        role : MessageRole::System,
        content,
        #[ cfg( feature = "vision_support" ) ]
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      }
    }
  }

  impl EmbeddingsRequest
  {
    /// Create a new embeddings request
    #[ inline ]
    #[ must_use ]
    pub fn new( model : String, prompt : String ) -> Self
    {
      Self
      {
        model,
        prompt,
        options : None,
        keep_alive : None,
      }
    }
  }
}

#[ cfg( feature = "enabled" ) ]
crate ::mod_interface!
{
  exposed use private::Message;
  exposed use private::MessageRole;
  exposed use private::ChatMessage;
  exposed use private::ChatRequest;
  exposed use private::ChatResponse;
  exposed use private::GenerateRequest;
  exposed use private::GenerateResponse;
  exposed use private::ModelInfo;
  exposed use private::ModelDetails;
  exposed use private::ModelEntry;
  exposed use private::TagsResponse;
  exposed use private::ShowModelRequest;
  exposed use private::PullModelRequest;
  exposed use private::PushModelRequest;
  exposed use private::DeleteModelRequest;
  exposed use private::EmbeddingsRequest;
  exposed use private::EmbeddingsResponse;
  exposed use private::ProgressInfo;
  exposed use private::ApiError;

  #[ cfg( feature = "tool_calling" ) ]
  exposed use private::ToolCall;
}