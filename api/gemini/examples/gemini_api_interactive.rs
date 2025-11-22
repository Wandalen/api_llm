//! Simple interactive chat with AI including real streaming responses.
//!
//! This example demonstrates:
//! - Interactive chat loop where you can type and get responses  
//! - Real streaming responses that appear as they're generated (with streaming feature)
//! - Simulated streaming fallback (without streaming feature)
//! - Clean conversation flow with history
//! - Proper error handling
//!
//! Usage:
//! ```bash
//! # With real streaming (recommended)
//! cargo run --example gemini_chat_interactive --features streaming
//! 
//! # With all features including streaming
//! cargo run --example gemini_chat_interactive --features full
//! 
//! # Basic version with simulated streaming
//! cargo run --example gemini_chat_interactive
//! ```
//! 
//! Type your messages and press Enter. Type 'quit', 'exit', or 'bye' to end.
//! Note : This is NOT for automated testing - it's for manual interactive use only.

#[ cfg( feature = "streaming" ) ]
use futures::StreamExt;
use api_gemini::{ client::Client, models::* };
use std::io::{ self, Write };

#[ tokio::main ]
#[ allow( clippy::too_many_lines ) ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
  let client = Client::new()?;
  
  println!( "Interactive AI Chat" );
  println!( "==================" );
  println!( "Type your messages and press Enter." );
  println!( "Type 'quit', 'exit', or 'bye' to end the conversation.\n" );
  
  let mut conversation_history = Vec::new();
  
  loop
  {
    // Get user input
    print!( "You: " );
    io ::stdout().flush()?;
  
    let mut input = String::new();
    match io::stdin().read_line( &mut input )
    {
      Ok( 0 ) =>
      {
        // EOF reached (e.g., when input is not available in non-interactive mode)
        println!( "\nNo input available. Use this example in interactive terminal only." );
        println!( "Run: cargo run --example gemini_chat_interactive" );
        break;
      }
    Ok( _ ) => {}
      Err( e ) =>
      {
      println!( "\nError reading input : {e}" );
        break;
      }
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
  
    // Add user message to conversation
    conversation_history.push( Content
    {
      role: "user".to_string(),
      parts: vec![ Part
      {
        text: Some( user_message ),
        inline_data: None,
        function_call: None,
        function_response: None,
        ..Default::default()
      }],
    });
  
    // Generate AI response
    let request = GenerateContentRequest
    {
      contents: conversation_history.clone(),
      generation_config: Some( GenerationConfig
      {
        temperature: Some( 0.7 ),
        max_output_tokens: Some( 1024 ),
        top_p: Some( 0.9 ),
        top_k: Some( 40 ),
        candidate_count: Some( 1 ),
        stop_sequences: None,
      }),
      safety_settings: None,
      tools: None,
      tool_config: None,
      system_instruction: None,
      cached_content: None,
    };
  
    print!( "\nAI: " );
    io ::stdout().flush()?;
  
    // Use real streaming if available, otherwise fallback to regular generation
    #[ cfg( feature = "streaming" ) ]
    {
      match client.models().by_name( "gemini-1.5-flash-latest" ).generate_content_stream( &request ).await
      {
        Ok( stream ) =>
        {
          let mut full_response = String::new();
          let mut response_content: Option< Content > = None;
          
          // Pin the stream to use it with StreamExt
          futures ::pin_mut!( stream );
          
          while let Some( chunk ) = stream.next().await
          {
            match chunk
            {
              Ok( streaming_response ) =>
              {
                if let Some( candidates ) = streaming_response.candidates
                {
                  if let Some( candidate ) = candidates.first()
                  {
                    // candidate.content is already a Content, not Option< Content >
                    if let Some( part ) = candidate.content.parts.first()
                    {
                      if let Some( text ) = &part.text
                      {
                      print!( "{text}" );
                        io ::stdout().flush()?;
                        full_response.push_str( text );
                      }
                    }
                    // Store the content for conversation history
                    if response_content.is_none()
                    {
                      response_content = Some( candidate.content.clone() );
                    }
                  }
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
          
          // Add response to conversation history
          if let Some( content ) = response_content
          {
            conversation_history.push( content );
          }
          else if !full_response.is_empty()
          {
            // Fallback : create content from accumulated text
            conversation_history.push( Content
            {
              role: "model".to_string(),
              parts: vec![ Part
              {
                text: Some( full_response ),
                inline_data: None,
                function_call: None,
                function_response: None,
                ..Default::default()
              }],
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
      match client.models().by_name( "gemini-1.5-flash-latest" ).generate_content( &request ).await
      {
        Ok( response ) =>
        {
          if let Some( candidate ) = response.candidates.first()
          {
            if let Some( part ) = candidate.content.parts.first()
            {
              if let Some( text ) = &part.text
              {
                // Simulate streaming by printing words with small delays
                let words: Vec< &str > = text.split_whitespace().collect();
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
                conversation_history.push( candidate.content.clone() );
              }
              else
              {
                println!( "AI response contained no text." );
              }
            }
            else
            {
              println!( "AI response had no parts." );
            }
          }
          else
          {
            println!( "AI generated no response candidates." );
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