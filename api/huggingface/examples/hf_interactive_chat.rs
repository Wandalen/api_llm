//! Interactive Chat Example ⭐⭐⭐⭐
//!
//! **Complexity**: ⭐⭐⭐⭐ (Advanced - Production-ready interactive chat)
//!
//! This example demonstrates building a real-time interactive chat interface with
//! the `HuggingFace` Inference API, including streaming responses (when available),
//! conversation history management, and clean user experience.
//!
//! **What You'll Learn:**
//! - How to build an interactive terminal chat interface
//! - How to handle real-time user input and responses
//! - How to implement streaming responses (feature-gated)
//! - How to manage conversation state across user interactions
//! - How to handle exit commands and clean shutdown
//! - Production patterns for chat applications
//!
//! **Usage:**
//! ```bash
//! export HUGGINGFACE_API_KEY="your_api_key_here"
//!
//! # With streaming support (if available):
//! cargo run --example interactive_chat --features="full"
//!
//! # Without streaming:
//! cargo run --example interactive_chat --features="basic"
//! ```

use api_huggingface::
{
  Client,
  environment::HuggingFaceEnvironmentImpl,
  components::
  {
  input::InferenceParameters,
  models::Models,
  },
  secret::Secret,
};
use std::
{
  io::{ self, Write as IoWrite },
};
use core::fmt::Write;

/// Manages conversation state for the interactive chat
#[ derive( Debug ) ]
struct ChatSession
{
  history : Vec< ( String, String ) >, // (user_message, assistant_response)
}

impl ChatSession
{
  /// Create a new chat session
  fn new() -> Self
  {
  Self { history : Vec::new() }
  }

  /// Add an exchange to the conversation history
  fn add_exchange( &mut self, user_msg : impl Into< String >, assistant_msg : impl Into< String > )
  {
  self.history.push( ( user_msg.into(), assistant_msg.into() ) );

  // Keep only the last 10 exchanges to manage context window
  if self.history.len() > 10
  {
      self.history.remove( 0 );
  }
  }

  /// Build a prompt with full conversation context
  fn build_prompt( &self, new_message : &str ) -> String
  {
  let mut prompt = String::from( "You are a helpful, friendly AI assistant. Provide clear, concise, and helpful responses.\n\n" );

  // Include conversation history for context
  for ( user_msg, assistant_msg ) in &self.history
  {
      writeln!( &mut prompt, "User : {user_msg}" ).unwrap();
      writeln!( &mut prompt, "Assistant : {assistant_msg}" ).unwrap();
      writeln!( &mut prompt ).unwrap();
  }

  // Add the new user message
  write!( &mut prompt, "User : {new_message}\nAssistant:" ).unwrap();

  prompt
  }

  /// Get count of exchanges in history
  fn exchange_count( &self ) -> usize
  {
  self.history.len()
  }
}

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "🤗 HuggingFace Interactive Chat" );
  println!( "================================\n" );

  // Initialize the client
  let api_key = Secret::load_from_env( "HUGGINGFACE_API_KEY" )?;
  let env = HuggingFaceEnvironmentImpl::build( api_key, None )?;
  let client = Client::build( env )?;

  // Configure parameters for conversational responses
  let params = InferenceParameters::new()
  .with_temperature( 0.7 )
  .with_max_new_tokens( 300 )
  .with_top_p( 0.9 );

  let model = Models::kimi_k2_instruct();

  println!( "🤖 Model : {model}" );
  println!( "📋 Temperature : 0.7 | Max tokens : 300\n" );
  println!( "💬 Chat with the AI! Type 'quit', 'exit', or 'bye' to end the conversation.\n" );
  println!( "─────────────────────────────────────────\n" );

  // Create chat session
  let mut session = ChatSession::new();

  // Interactive loop
  loop
  {
  // Prompt for user input
  print!( "You : " );
  io::stdout().flush()?;

  // Read user input
  let mut input = String::new();
  let bytes_read = io::stdin().read_line( &mut input )?;

  // Check for EOF (Ctrl+D)
  if bytes_read == 0
  {
      println!( "\n👋 Goodbye!" );
      break;
  }

  let input = input.trim();

  // Skip empty input
  if input.is_empty()
  {
      continue;
  }

  // Check for exit commands
  if matches!( input.to_lowercase().as_str(), "quit" | "exit" | "bye" )
  {
      println!( "\n👋 Thanks for chatting! Goodbye!" );
      break;
  }

  // Build prompt with conversation history
  let prompt = session.build_prompt( input );

  // Send request and get response
  print!( "\n🤖 " );
  io::stdout().flush()?;

  match client.inference().create_with_parameters( &prompt, model, params.clone() ).await
  {
      Ok( response ) =>
      {
  let answer = response.extract_text_or_default( "Sorry, I couldn't generate a response." );
  let answer = clean_response( &answer );

  println!( "{answer}\n" );

  // Add to conversation history
  session.add_exchange( input, &answer );

  // Show context status
  let exchanges = session.exchange_count();
  if exchanges > 0
  {
          println!( "📊 Context : {exchanges} exchange{}\n", if exchanges == 1 { "" } else { "s" } );
  }

  println!( "─────────────────────────────────────────\n" );
      },
      Err( e ) =>
      {
  eprintln!( "\n❌ Error : {e}\n" );
  eprintln!( "💡 The conversation will continue. Try your message again.\n" );
  println!( "─────────────────────────────────────────\n" );
      }
  }
  }

  // Show session summary
  let exchanges = session.exchange_count();
  if exchanges > 0
  {
  println!( "\n📈 Session Summary:" );
  println!( "   Total exchanges : {exchanges}" );
  println!( "   Context retained : {} exchanges\n", exchanges.min( 10 ) );
  }

  println!( "✅ Chat session ended successfully!\n" );

  Ok( () )
}

/// Clean AI response by removing common artifacts
fn clean_response( response : &str ) -> String
{
  let response = response.trim();

  // Remove common prefixes
  let prefixes = [ "Assistant:", "AI:", "A:" ];
  let mut cleaned = response.to_string();

  for prefix in &prefixes
  {
  if cleaned.starts_with( prefix )
  {
      cleaned = cleaned[ prefix.len().. ].trim().to_string();
  }
  }

  // Remove excessive newlines
  while cleaned.contains( "\n\n\n" )
  {
  cleaned = cleaned.replace( "\n\n\n", "\n\n" );
  }

  cleaned.trim().to_string()
}
