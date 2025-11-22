//! Multi-Turn Conversation Example ⭐⭐⭐
//!
//! **Complexity**: ⭐⭐⭐ (Intermediate - Demonstrates conversation context)
//!
//! This example demonstrates how to maintain conversation context across multiple
//! turns using the `HuggingFace` Inference API. Unlike chat-specific APIs, `HuggingFace`'s
//! text generation requires manual context management by including conversation
//! history in each request.
//!
//! **What You'll Learn:**
//! - How to maintain conversation context across multiple API calls
//! - How to build conversation history into prompts
//! - How to format multi-turn conversations for text generation models
//! - How to handle role alternation (user/assistant)
//! - Best practices for context management
//!
//! **Usage:**
//! ```bash
//! export HUGGINGFACE_API_KEY="your_api_key_here"
//! cargo run --example multi_turn_conversation --features="full"
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

/// Represents a single conversation turn
#[ derive( Debug, Clone ) ]
struct ConversationTurn
{
  role : String,
  content : String,
}

/// Manages conversation history and context
#[ derive( Debug ) ]
struct ConversationHistory
{
  turns : Vec< ConversationTurn >,
}

impl ConversationHistory
{
  /// Create a new conversation history
  fn new() -> Self
  {
  Self { turns : Vec::new() }
  }

  /// Add a user message to the conversation
  fn add_user_message( &mut self, content : impl Into< String > )
  {
  self.turns.push( ConversationTurn
  {
      role : "User".to_string(),
      content : content.into(),
  } );
  }

  /// Add an assistant message to the conversation
  fn add_assistant_message( &mut self, content : impl Into< String > )
  {
  self.turns.push( ConversationTurn
  {
      role : "AI".to_string(),
      content : content.into(),
  } );
  }

  /// Build a contextual prompt including all conversation history
  ///
  /// This is the key to multi-turn conversations with text generation APIs.
  /// We include all previous exchanges in the prompt so the model has full context.
  fn build_prompt( &self, new_user_message : &str ) -> String
  {
  use core::fmt::Write;

  let mut prompt = String::from( "You are a helpful AI assistant. Have a natural conversation with the user.\n\n" );

  // Add all previous turns
  for turn in &self.turns
  {
      writeln!( &mut prompt, "{}: {}", turn.role, turn.content ).unwrap();
  }

  // Add the new user message
  write!( &mut prompt, "User : {new_user_message}\nAI:" ).unwrap();

  prompt
  }

  /// Display the conversation in a readable format
  fn display( &self )
  {
  println!( "📜 Conversation History:" );
  println!( "─────────────────────────────────────────" );
  for ( i, turn ) in self.turns.iter().enumerate()
  {
      println!( "\nTurn {}:", i + 1 );
      println!( "{}: {}", turn.role, turn.content );
  }
  println!( "─────────────────────────────────────────\n" );
  }
}

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn std::error::Error > >
{
  println!( "🤗 HuggingFace Multi-Turn Conversation Example" );
  println!( "=============================================\n" );

  // Step 1: Initialize the client
  let api_key = Secret::load_from_env( "HUGGINGFACE_API_KEY" )?;
  let env = HuggingFaceEnvironmentImpl::build( api_key, None )?;
  let client = Client::build( env )?;
  println!( "✓ Client initialized\n" );

  // Step 2: Configure parameters for conversation
  let params = InferenceParameters::new()
  .with_temperature( 0.7 )
  .with_max_new_tokens( 200 )
  .with_top_p( 0.9 );

  let model = Models::kimi_k2_instruct();
  println!( "🤖 Model : {model}" );
  println!( "📋 Temperature : 0.7 | Max tokens : 200 | Top-p : 0.9\n" );

  // Step 3: Create conversation history tracker
  let mut history = ConversationHistory::new();

  println!( "🎬 Starting pre-scripted travel planning conversation...\n" );
  println!( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n" );

  // Turn 1: Initial question about travel destination
  println!( "Turn 1: Initial Travel Question" );
  println!( "─────────────────────────────────────────" );

  let turn1_message = "I want to plan a trip to Japan in spring. What should I know?";
  println!( "User : {turn1_message}\n" );

  let prompt1 = history.build_prompt( turn1_message );
  let response1 = client.inference().create_with_parameters( &prompt1, model, params.clone() ).await?;
  let answer1 = response1.extract_text_or_default( "No response generated" );
  let answer1 = clean_response( &answer1, turn1_message );

  println!( "AI: {answer1}\n" );

  // Save turn 1 to history
  history.add_user_message( turn1_message );
  history.add_assistant_message( &answer1 );

  println!( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n" );

  // Turn 2: Follow-up question about specific location (requires context from turn 1)
  println!( "Turn 2: Follow-up About Specific Location" );
  println!( "─────────────────────────────────────────" );

  let turn2_message = "What about visiting Kyoto specifically? What are the must-see places?";
  println!( "User : {turn2_message}\n" );

  // The prompt now includes the entire conversation history
  let prompt2 = history.build_prompt( turn2_message );
  let response2 = client.inference().create_with_parameters( &prompt2, model, params.clone() ).await?;
  let answer2 = response2.extract_text_or_default( "No response generated" );
  let answer2 = clean_response( &answer2, turn2_message );

  println!( "AI: {answer2}\n" );

  // Save turn 2 to history
  history.add_user_message( turn2_message );
  history.add_assistant_message( &answer2 );

  println!( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n" );

  // Turn 3: Question about timing (requires context from turns 1 and 2)
  println!( "Turn 3: Question About Best Time to Visit" );
  println!( "─────────────────────────────────────────" );

  let turn3_message = "When exactly in spring would be the best time for cherry blossoms there?";
  println!( "User : {turn3_message}\n" );

  // The prompt now includes all three turns of conversation
  let prompt3 = history.build_prompt( turn3_message );
  let response3 = client.inference().create_with_parameters( &prompt3, model, params.clone() ).await?;
  let answer3 = response3.extract_text_or_default( "No response generated" );
  let answer3 = clean_response( &answer3, turn3_message );

  println!( "AI: {answer3}\n" );

  // Save turn 3 to history
  history.add_user_message( turn3_message );
  history.add_assistant_message( &answer3 );

  println!( "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n" );

  // Display the complete conversation
  history.display();

  // Technical notes about the conversation
  println!( "📝 Technical Notes:" );
  println!( "─────────────────────────────────────────" );
  println!( "• Each API call included the full conversation history in the prompt" );
  println!( "• Turn 2 referenced 'Kyoto specifically', building on Turn 1's Japan context" );
  println!( "• Turn 3 used 'there' and 'cherry blossoms', requiring context from both previous turns" );
  println!( "• Context retention is achieved by concatenating all previous exchanges" );
  println!( "• The model can reference earlier parts of the conversation naturally\n" );

  println!( "💡 Best Practices:" );
  println!( "─────────────────────────────────────────" );
  println!( "• Limit history to recent turns (e.g., last 5-10) to avoid token limits" );
  println!( "• Include system instructions at the start of each prompt" );
  println!( "• Clean responses to remove prompt echoes" );
  println!( "• Track conversation state separately from API calls" );
  println!( "• Consider token costs when building long conversation histories\n" );

  println!( "✅ Multi-turn conversation example completed successfully!\n" );

  Ok( () )
}

/// Clean up AI response by removing prompt echoes and artifacts
fn clean_response( response : &str, _user_input : &str ) -> String
{
  let response = response.trim();

  // Remove common prefixes that might appear in responses
  let prefixes = [ "AI:", "Assistant:", "A:" ];

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
