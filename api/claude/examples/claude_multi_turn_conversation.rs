//! Multi-turn conversation example with the Claude API.
//!
//! **Complexity:** ⭐⭐⭐ (Intermediate)
//!
//! This example demonstrates:
//! - Pre-scripted conversation scenario with context building
//! - Conversation history building and maintenance across multiple API calls
//! - Role alternation (user → assistant → user → assistant)
//! - Context retention across turns for coherent conversation flow
//! - Natural conversation display format (User : / AI:)
//! - Technical notes about conversation mechanics and state management
//! - Educational guidance on managing conversation context effectively
//!
//! ## Usage
//!
//! ```bash
//! # Set your API key
//! export ANTHROPIC_API_KEY="your-api-key-here"
//!
//! # Run the example
//! cargo run --example claude_multi_turn_conversation
//! ```
//!
//! ## What You'll Learn
//!
//! - How to build conversation history programmatically
//! - Managing conversation context across multiple API calls
//! - Role-based message structure in Claude conversations
//! - Best practices for maintaining coherent conversation flow
//! - Understanding how conversation history affects AI responses
//!
//! **Target Audience**: Intermediate developers learning conversation patterns
//!
//! This example showcases context-aware conversation management essential for
//! building chat applications and interactive AI systems.

use api_claude::{ Client, CreateMessageRequest, Message, Role, Content };

#[ tokio::main( flavor = "current_thread" ) ]
#[ allow( clippy::too_many_lines ) ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
  // Initialize the client with API key from environment variable
  println!( "Initializing Claude client for multi-turn conversation..." );
  let client = Client::from_env()?;

  // === TURN 1: Initial Question ===
  println!( "\n=== TURN 1: Starting Travel Planning Conversation ===\n" );

  let turn1_message = "I'm planning a trip to Japan for 2 weeks in spring. What are some must-visit places?";
  println!( "User : {turn1_message}" );

  // Start with initial conversation history
  let mut conversation_history = vec![
    Message {
      role : Role::User,
      content : vec![ Content::Text {
        r#type : "text".to_string(),
        text : turn1_message.to_string(),
      } ],
      cache_control : None,
    }
  ];

  // Create first request with conversation history
  let request1 = CreateMessageRequest::builder()
    .model( "claude-sonnet-4-5-20250929".to_string() )
    .max_tokens( 800 )
    .messages( conversation_history.clone() )
    .temperature( 0.7 )
    .build();

  // Make first API call
  let response1 = client.create_message( request1 ).await?;

  // Extract and display response
  let ai_response1 = response1.content.first()
    .and_then( |content| content.text.as_ref() )
    .ok_or( "No text content in response" )?;

  println!( "\nAI: {ai_response1}" );

  // Add AI response to conversation history
  conversation_history.push( Message {
    role : Role::Assistant,
    content : vec![ Content::Text {
      r#type : "text".to_string(),
      text : ai_response1.clone(),
    } ],
    cache_control : None,
  });

  // === TURN 2: Follow-up Question ===
  println!( "\n=== TURN 2: Follow-up About Cherry Blossoms ===\n" );

  let turn2_message = "That sounds amazing! When exactly is the best time to see cherry blossoms, and which of those places you mentioned are best for hanami?";
  println!( "User : {turn2_message}" );

  // Add second user message to conversation history
  conversation_history.push( Message {
    role : Role::User,
    content : vec![ Content::Text {
      r#type : "text".to_string(),
      text : turn2_message.to_string(),
    } ],
    cache_control : None,
  });

  // Create second request with updated conversation history
  let request2 = CreateMessageRequest::builder()
    .model( "claude-sonnet-4-5-20250929".to_string() )
    .max_tokens( 800 )
    .messages( conversation_history.clone() )
    .temperature( 0.7 )
    .build();

  // Make second API call
  let response2 = client.create_message( request2 ).await?;

  // Extract and display response
  let ai_response2 = response2.content.first()
    .and_then( |content| content.text.as_ref() )
    .ok_or( "No text content in response" )?;

  println!( "\nAI: {ai_response2}" );

  // Add AI response to conversation history
  conversation_history.push( Message {
    role : Role::Assistant,
    content : vec![ Content::Text {
      r#type : "text".to_string(),
      text : ai_response2.clone(),
    } ],
    cache_control : None,
  });

  // === TURN 3: Specific Planning Question ===
  println!( "\n=== TURN 3: Specific Planning Details ===\n" );

  let turn3_message = "Perfect timing! Could you help me plan a 3-day itinerary for Tokyo that includes both traditional temples and modern attractions?";
  println!( "User : {turn3_message}" );

  // Add third user message to conversation history
  conversation_history.push( Message {
    role : Role::User,
    content : vec![ Content::Text {
      r#type : "text".to_string(),
      text : turn3_message.to_string(),
    } ],
    cache_control : None,
  });

  // Create third request with full conversation history
  let request3 = CreateMessageRequest::builder()
    .model( "claude-sonnet-4-5-20250929".to_string() )
    .max_tokens( 1000 )
    .messages( conversation_history.clone() )
    .temperature( 0.7 )
    .build();

  // Make third API call
  let response3 = client.create_message( request3 ).await?;

  // Extract and display response
  let ai_response3 = response3.content.first()
    .and_then( |content| content.text.as_ref() )
    .ok_or( "No text content in response" )?;

  println!( "\nAI: {ai_response3}" );

  // === Technical Analysis and Educational Notes ===
  println!( "\n" );
  println!( "🔧 === Technical Analysis : Conversation Mechanics ===" );
  println!( "• Total conversation turns : 3 (user-ai-user-ai-user-ai)" );
  println!( "• Messages in final history : {} entries", conversation_history.len() + 1 );
  println!( "• Context accumulation : Each API call includes all previous messages" );
  println!( "• Role alternation : Strictly alternating User/Assistant roles maintained" );
  println!( "• Context retention : AI references previous conversation context in each response" );

  // Display cumulative token usage
  let total_input_tokens = response1.usage.input_tokens + response2.usage.input_tokens + response3.usage.input_tokens;
  let total_output_tokens = response1.usage.output_tokens + response2.usage.output_tokens + response3.usage.output_tokens;

  println!( "\n💰 === Cumulative Resource Usage ===" );
  println!( "Total input tokens : {total_input_tokens} (across 3 API calls)" );
  println!( "Total output tokens : {total_output_tokens}" );
  println!( "Total tokens : {}", total_input_tokens + total_output_tokens );
  println!( "⚠️  Note : Input tokens increase with each turn due to growing conversation history" );

  println!( "\n📚 === Educational Notes : Context Management ===" );
  println!( "1. **Growing Context**: Each API call sends the entire conversation history," );
  println!( "   which means the input token cost increases with each turn." );
  println!();
  println!( "2. **Context Coherence**: The AI maintains awareness of previous topics" );
  println!( "   (Japan trip, cherry blossoms, Tokyo planning) across all turns." );
  println!();
  println!( "3. **Role Structure**: Claude requires strict alternating User/Assistant roles." );
  println!( "   Multiple consecutive messages from the same role are not permitted." );
  println!();
  println!( "4. **Memory Management**: For long conversations, consider context trimming" );
  println!( "   strategies to manage token costs while maintaining conversation quality." );
  println!();
  println!( "5. **State Management**: The conversation_history Vec acts as our" );
  println!( "   conversation state, growing with each exchange." );

  Ok( () )
}