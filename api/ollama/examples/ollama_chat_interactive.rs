//! Simple interactive chat with AI including real streaming responses.
//!
//! This example demonstrates:
//! - Interactive chat loop where you can type and get responses
//! - Real streaming responses that appear as they're generated (with streaming feature)
//! - Simulated streaming fallback (without streaming feature)
//! - Clean conversation flow with history
//! - Proper error handling
//! - Production-ready chat application patterns
//! - Dynamic conversation history building using Ollama messages format
//! - Performance optimization with faster Ollama models
//!
//! ## Usage
//!
//! ```bash
//! # With real streaming (recommended)
//! cargo run --example ollama_chat_interactive --features streaming
//!
//! # With all features including streaming
//! cargo run --example ollama_chat_interactive --features full
//!
//! # Basic version with simulated streaming
//! cargo run --example ollama_chat_interactive
//! ```
//!
//! Type your messages and press Enter. Type 'quit', 'exit', or 'bye' to end.
//! Note : This is NOT for automated testing - it's for manual interactive use only.
//!
//! ## What You'll Learn
//!
//! - How to build production-ready interactive chat applications with Ollama
//! - Real-time streaming response handling with proper error recovery
//! - Dynamic conversation history management across user sessions
//! - Feature-conditional behavior based on streaming capabilities
//! - Performance optimization patterns for interactive applications
//! - Input/output error handling for terminal applications
//! - Ollama-specific optimizations and model selection strategies
//!
//! **Complexity**: ⭐⭐⭐⭐ (Advanced)
//!
//! This example is perfect for advanced developers building chat applications with real-time
//! interaction patterns, demonstrating production-ready streaming and conversation management.

#[ cfg( feature = "streaming" ) ]
use futures_util::StreamExt;
use api_ollama::{ OllamaClient, ChatRequest, ChatMessage, MessageRole };
use std::io::{ self, Write };
use std::collections::HashMap;

#[ tokio::main ]
#[ allow( clippy::too_many_lines ) ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
  // Initialize Ollama client with optimized configuration for interactive use
  let mut client = OllamaClient::new(
    "http://localhost:11434".to_string(),
    OllamaClient::recommended_timeout_default()
  );

  println!( "Interactive AI Chat" );
  println!( "==================" );
  println!( "Type your messages and press Enter." );
  println!( "Type 'quit', 'exit', or 'bye' to end the conversation.\n" );

  // Initialize conversation history for context retention
  let mut conversation_history = Vec::new();

  // Configure Ollama parameters for interactive chat (optimized for responsiveness)
  let mut options = HashMap::new();
  options.insert( "temperature".to_string(), serde_json::json!( 0.7 ) );
  options.insert( "top_k".to_string(), serde_json::json!( 40 ) );
  options.insert( "top_p".to_string(), serde_json::json!( 0.9 ) );
  options.insert( "num_predict".to_string(), serde_json::json!( 1024 ) ); // Reasonable response length

  loop
  {
    // Get user input
    print!( "You : " );
    io ::stdout().flush()?;

    let mut input = String::new();
    let bytes_read = match io::stdin().read_line( &mut input )
    {
      Ok( n ) => n,
      Err( e ) =>
      {
        println!( "\nError reading input : {e}" );
        break;
      }
    };

    // Handle EOF (no input available in non-interactive mode)
    if bytes_read == 0
    {
      println!( "\n👋 No input available (EOF). Exiting gracefully." );
      println!( "Note : Use this example in interactive terminal only." );
      println!( "Run : cargo run --example ollama_chat_interactive" );
      break;
    }

    let user_message = input.trim().to_string();

    // Handle exit commands
    if user_message.is_empty()
    {
      continue;
    }

    if matches!( user_message.to_lowercase().as_str(), "quit" | "exit" | "bye" )
    {
      println!( "\nGoodbye! Thanks for chatting!" );
      break;
    }

    // Add user message to conversation history
    conversation_history.push( ChatMessage
    {
      role : MessageRole::User,
      content : user_message,
      #[ cfg( feature = "vision_support" ) ]
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    });

    // Create chat request with full conversation history
    #[ cfg( feature = "streaming" ) ]
    let request = ChatRequest
    {
      model : "llama3.2:3b".to_string(), // Use faster 3B model for interactive performance
      messages : conversation_history.clone(),
      stream : Some( true ), // Enable streaming for real-time responses
      options : Some( serde_json::Value::Object( options.clone().into_iter().collect() ) ),
      #[ cfg( feature = "tool_calling" ) ]
      tools : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_messages : None,
    };

    print!( "\nAI: " );
    io ::stdout().flush()?;

    // Use real streaming if available, otherwise fallback to regular generation
    #[ cfg( feature = "streaming" ) ]
    {
      match client.chat_stream( request.clone() ).await
      {
        Ok( mut stream ) =>
        {
          let mut full_response = String::new();

          while let Some( chunk ) = stream.next().await
          {
            match chunk
            {
              Ok( streaming_response ) =>
              {
                if !streaming_response.message.content.is_empty()
                {
                  print!( "{}", streaming_response.message.content );
                  io ::stdout().flush()?;
                  full_response.push_str( &streaming_response.message.content );
                }

                // Check if streaming is complete
                if streaming_response.done
                {
                  break;
                }
              }
              Err( e ) =>
              {
                println!( "\nStreaming error : {e}" );
                break;
              }
            }
          }

          println!( "\n" );

          // Add AI response to conversation history
          if !full_response.is_empty()
          {
            conversation_history.push( ChatMessage
            {
              role : MessageRole::Assistant,
              content : full_response,
              #[ cfg( feature = "vision_support" ) ]
              images : None,
              #[ cfg( feature = "tool_calling" ) ]
              tool_calls : None,
            });
          }
        }
        Err( e ) =>
        {
          println!( "Streaming error : {e}" );
          println!( "Please try again or type 'quit' to exit.\n" );
        }
      }
    }

    #[ cfg( not( feature = "streaming" ) ) ]
    {
      // Create non-streaming request for fallback
      let non_streaming_request = ChatRequest
      {
        model : "llama3.2:3b".to_string(),
        messages : conversation_history.clone(),
        stream : Some( false ), // Disable streaming for fallback
        options : Some( serde_json::Value::Object( options.clone().into_iter().collect() ) ),
        #[ cfg( feature = "tool_calling" ) ]
        tools : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_messages : None,
      };

      match client.chat( non_streaming_request ).await
      {
        Ok( response ) =>
        {
          if !response.message.content.is_empty()
          {
            // Simulate streaming by printing words with small delays
            let words : Vec< &str > = response.message.content.split_whitespace().collect();
            for ( i, word ) in words.iter().enumerate()
            {
              print!( "{}", word );
              if i < words.len() - 1
              {
                print!( " " );
              }
              io ::stdout().flush()?;
              tokio ::time::sleep( tokio::time::Duration::from_millis( 80 ) ).await;
            }
            println!( "\n" );

            // Add AI response to conversation history
            conversation_history.push( ChatMessage
            {
              role : MessageRole::Assistant,
              content : response.message.content,
              #[ cfg( feature = "vision_support" ) ]
              images : None,
              #[ cfg( feature = "tool_calling" ) ]
              tool_calls : None,
            });
          }
          else
          {
            println!( "AI response contained no text." );
          }
        }
        Err( e ) =>
        {
          println!( "Error : {}", e );
          println!( "Please try again or type 'quit' to exit.\n" );
        }
      }
    }
  }

  Ok( () )
}