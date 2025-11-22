//! Basic chat conversation example with the Claude API.
//!
//! This example demonstrates:
//! - Simple client initialization from environment variables
//! - Creating a basic text generation request
//! - Sending a single-turn conversation to the AI
//! - Handling responses and extracting generated text
//! - Basic error handling patterns
//!
//! ## Usage
//!
//! ```bash
//! # Set your API key
//! export ANTHROPIC_API_KEY="your-api-key-here"
//!
//! # Run the example
//! cargo run --example claude_api_basic
//! ```
//!
//! ## What You'll Learn
//!
//! - How to initialize the Claude client
//! - Basic request structure for text generation
//! - How to access generated content from responses
//! - Essential error handling for API calls
//!
//! This is perfect for beginners to understand the basic flow of using the Claude API.

use api_claude::{ Client, CreateMessageRequest, Message, Role, Content };

#[ tokio::main( flavor = "current_thread" ) ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
  // Initialize the client with API key from environment variable
  println!( "Initializing Claude client from environment..." );
  let client = Client::from_env()?;

  // Create a simple conversation request
  let request = CreateMessageRequest::builder()
    .model( "claude-sonnet-4-5-20250929".to_string() )
    .max_tokens( 1024 )
    .messages( vec![
      Message
      {
        role : Role::User,
        content : vec![ Content::Text {
          r#type : "text".to_string(),
          text : "Hello! Can you explain what artificial intelligence is in simple terms?".to_string(),
        } ],
        cache_control : None,
      }
    ] )
    .temperature( 0.7 )
    .build();

  // Always show the JSON payload for transparency
  println!( "=== Request JSON Payload ===" );
  println!( "{}", serde_json::to_string_pretty( &request )? );
  println!( "=== End JSON Payload ===\n" );

  // Show the exact request being sent for API transparency
  #[ cfg( feature = "curl-diagnostics" ) ]
  {
    println!( "=== Exact Curl Command Being Executed ===" );
    // Note : Claude API doesn't have AsCurl trait yet, so this is a placeholder
    println!( "curl -X POST https://api.anthropic.com/v1/messages \\" );
    println!( "  -H \"Content-Type : application/json\" \\" );
    println!( "  -H \"x-api-key : $ANTHROPIC_API_KEY\" \\" );
    println!( "  -H \"anthropic-version : 2023-06-01\" \\" );
    println!( "  -d '{}'", serde_json::to_string( &request )? );
    println!( "=== End Curl Command ===\n" );
  }

  println!( "Sending request to Claude API..." );

  // Generate content using the Claude model
  let response = client.create_message( request ).await?;

  // Process and display the response
  if let Some( content ) = response.content.first()
  {
    if content.r#type == "text"
    {
      if let Some( text ) = &content.text
      {
        println!( "\n=== Claude Response ===" );
        println!( "{text}" );
      }
    }
    else
    {
      println!( "Non-text content received (unexpected for this example)" );
    }
  }
  else
  {
    println!( "No response received from the API." );
  }

  // Display stop reason if available
  if let Some( stop_reason ) = &response.stop_reason
  {
    println!( "\nStop reason : {stop_reason}" );
  }

  // Display usage metadata
  let usage = &response.usage;
  println!( "\n=== Token Usage ===" );
  println!( "Input tokens : {}", usage.input_tokens );
  println!( "Output tokens : {}", usage.output_tokens );
  println!( "Total tokens : {}", usage.total_tokens() );

  // Display model information
  println!( "\nModel used : {}", response.model );

  Ok( () )
}