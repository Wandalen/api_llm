//! Basic chat conversation example with the Ollama API.
//!
//! This example demonstrates:
//! - Simple client initialization with explicit configuration
//! - Creating a basic chat request with model selection
//! - Sending a single-turn conversation to the AI
//! - Handling responses and extracting generated text
//! - Request transparency with JSON payload display
//! - Token usage tracking from Ollama response metadata
//! - Basic error handling patterns
//!
//! ## Usage
//!
//! ```bash
//! # Make sure Ollama is running and has a model installed
//! ollama pull llama3.2
//!
//! # Run the example
//! cargo run --example chat
//! ```
//!
//! ## What You'll Learn
//!
//! - How to initialize the Ollama client with explicit configuration
//! - Basic request structure for chat completions
//! - How to configure model parameters (temperature, etc.)
//! - How to access generated content from responses
//! - Request transparency and debugging techniques
//! - Essential error handling for API calls
//!
//! **Complexity**: ⭐ (Basic)
//!
//! This is perfect for beginners to understand the basic flow of using the Ollama API
//! following the "Thin Client, Rich API" principle with complete transparency.

use api_ollama::{ OllamaClient, ChatRequest, ChatMessage, MessageRole };
use std::collections::HashMap;

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
  // Initialize the client with explicit configuration (no magic defaults)
  let mut client = OllamaClient::new(
    "http://localhost:11434".to_string(),
    OllamaClient::recommended_timeout_default() // 120 seconds - explicit timeout
  );

  // Create model parameters for Ollama (explicit configuration)
  let mut options = HashMap::new();
  options.insert( "temperature".to_string(), serde_json::json!( 0.7 ) );
  options.insert( "top_k".to_string(), serde_json::json!( 40 ) );
  options.insert( "top_p".to_string(), serde_json::json!( 0.95 ) );
  options.insert( "num_predict".to_string(), serde_json::json!( 1024 ) ); // max_tokens equivalent

  // Create a simple conversation request
  let request = ChatRequest
  {
    model : "llama3.2:3b".to_string(), // Ollama model selection with explicit tag
    messages : vec!
    [
      ChatMessage
      {
        role : MessageRole::User,
        content : "Hello! Can you explain what artificial intelligence is in simple terms?".to_string(),
        #[ cfg( feature = "vision_support" ) ]
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      }
    ],
    stream : Some( false ), // Non-streaming response
    options : Some( serde_json::Value::Object( options.into_iter().collect() ) ),
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  // Note : Curl generation would be available with diagnostics features

  // Always show the JSON payload for transparency (Thin Client principle)
  println!( "=== Request JSON Payload ===" );
  println!( "{}", serde_json::to_string_pretty( &request )? );
  println!( "=== End JSON Payload ===\n" );

  println!( "Sending request to Ollama API..." );

  // Generate content using the specified Ollama model
  let response = client.chat( request ).await?;

  // Process and display the response
  println!( "\n=== Ollama Response ===" );
  println!( "{}", response.message.content );

  // Show response metadata
  println!( "\n=== Response Metadata ===" );
  if let Some( model ) = &response.model
  {
    println!( "Model : {model}" );
  }
  if let Some( created_at ) = &response.created_at
  {
    println!( "Created at : {created_at}" );
  }
  println!( "Done : {}", response.done );

  // Display usage metadata if available (Ollama token tracking)
  if let Some( eval_count ) = response.eval_count
  {
    println!( "\n=== Token Usage ===" );
    println!( "Response tokens (eval_count): {eval_count}" );
  }
  if let Some( prompt_eval_count ) = response.prompt_eval_count
  {
    println!( "Prompt tokens (prompt_eval_count): {prompt_eval_count}" );
  }
  if let Some( total_duration ) = response.total_duration
  {
    println!( "Total duration : {total_duration} ms" );
  }
  if let Some( eval_duration ) = response.eval_duration
  {
    println!( "Eval duration : {eval_duration} ms" );
  }

  println!( "\n=== Example Complete ===" );
  println!( "This example demonstrated:" );
  println!( "✓ Explicit client configuration (no magic defaults)" );
  println!( "✓ Request transparency with JSON payload display" );
  println!( "✓ Model parameter configuration for Ollama" );
  println!( "✓ Single-turn conversation handling" );
  println!( "✓ Response processing and token usage tracking" );
  println!( "✓ Error handling with proper Result types" );

  Ok( () )
}