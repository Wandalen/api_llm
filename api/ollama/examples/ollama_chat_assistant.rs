//! # Use Case 1: Interactive Chatbot Assistant
//! 
//! This example demonstrates building a conversational AI assistant that maintains
//! context across multiple exchanges. This is one of the most common use cases for
//! local LLMs - creating chatbots for customer support, personal assistants, or
//! interactive applications.
//! 
//! ## Real-world applications:
//! - Customer service chatbots
//! - Personal AI assistants  
//! - Educational tutoring systems
//! - Interactive documentation helpers
//! 
//! ## To run this example:
//! ```bash
//! # Make sure Ollama is running with a model installed
//! ollama pull llama3.2
//! cargo run --example ollama_chat_assistant --all-features
//! ```

use api_ollama::{ OllamaClient, ChatRequest, ChatMessage, MessageRole };
use std::io::{ self, Write };

async fn setup_client_and_model() -> Result< ( OllamaClient, String ), Box< dyn core::error::Error > >
{
  // Initialize client
  let mut client = OllamaClient::new( "http://localhost:11434".to_string(), OllamaClient::recommended_timeout_fast() );
  
  // Check if Ollama is available
  if !client.is_available().await
  {
    eprintln!( "❌ Ollama server is not available. Please start Ollama and try again." );
    eprintln!( "   Start with : ollama serve" );
    std ::process::exit( 1 );
  }
  
  // Get available models
  let models = client.list_models().await?;
  if models.models.is_empty()
  {
    eprintln!( "❌ No models available. Please install a model first." );
    eprintln!( "   Install with : ollama pull llama3.2" );
    std ::process::exit( 1 );
  }
  
  // Prefer smarter models over tinyllama
  let preferred_models = [ "llama3.2:3b", "llama3.2:8b", "qwen2.5:7b", "llama3.1:8b" ];
  
  let model_name = preferred_models
    .iter()
    .find( |&preferred| models.models.iter().any( |m| m.name == *preferred ) )
    .map_or_else(|| models.models[ 0 ].name.clone(), |&name| name.to_string());
    
  println!( "✅ Using model : {model_name}" );
  
  Ok( ( client, model_name ) )
}

async fn handle_chat_response(
  client : &mut OllamaClient, 
  request : ChatRequest,
  conversation_history : &mut Vec< ChatMessage >
) -> Result< (), Box< dyn core::error::Error > >
{
  print!( "🤖 Assistant : " );
  io ::stdout().flush()?;
  
  // Stream the response for better user experience
  #[ cfg( feature = "streaming" ) ]
  {
    let mut full_response = String::new();
    match client.chat_stream( request ).await
    {
      Ok( mut stream ) =>
      {
        use futures_util::StreamExt;
        
        while let Some( chunk ) = stream.next().await
        {
          match chunk
          {
            Ok( response ) =>
            {
              if response.done
              {
                // Final chunk - we're done
                break;
              }
              let content = response.message.content;
              print!( "{content}" );
              io ::stdout().flush()?;
              full_response.push_str( &content );
            }
            Err( e ) =>
            {
              eprintln!( "\n❌ Stream error : {e}" );
              break;
            }
          }
        }
        println!( "\n" );
        
        // Add complete response to conversation history
        if !full_response.is_empty()
        {
          conversation_history.push( ChatMessage
          {
            role : MessageRole::Assistant,
            content : full_response,
            images : None,
            #[ cfg( feature = "tool_calling" ) ]
            tool_calls : None,
          } );
        }
      }
      Err( e ) =>
      {
        eprintln!( "\n❌ Error : {e}" );
        eprintln!( "   Please check your Ollama installation and try again.\n" );
      }
    }
  }
  
  // Fallback for non-streaming builds
  #[ cfg( not( feature = "streaming" ) ) ]
  {
    match client.chat( request ).await
    {
      Ok( response ) =>
      {
        let assistant_message = response.message.content;
        println!( "{assistant_message}\n" );
        
        // Add assistant response to conversation history
        conversation_history.push( ChatMessage
        {
          role : MessageRole::Assistant,
          content : assistant_message,
          images : None,
          #[ cfg( feature = "tool_calling" ) ]
          tool_calls : None,
        } );
      }
      Err( e ) =>
      {
        eprintln!( "❌ Error : {e}" );
        eprintln!( "   Please check your Ollama installation and try again.\n" );
      }
    }
  }
  
  Ok( () )
}

#[ tokio::main ]
#[ allow( clippy::too_many_lines ) ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "🤖 Interactive Chatbot Assistant" );
  println!( "================================" );

  // Fix(issue-eof-before-server-001): Check for EOF BEFORE connecting to server
  // Root cause: EOF check was inside main loop after server setup, so empty stdin caused server connection failure before reaching EOF handling
  // Pitfall: Interactive examples should handle "no input" gracefully without requiring external dependencies (server)

  // Check for input availability before attempting server connection
  println!( "\n💬 Chat started! Type 'quit' to exit.\n" );
  print!( "You : " );
  io ::stdout().flush()?;

  let mut first_input = String::new();
  let bytes_read = io::stdin().read_line( &mut first_input )?;

  // Handle EOF immediately - no server needed if there's no input
  if bytes_read == 0
  {
    println!( "\n👋 No input available (EOF). Exiting gracefully." );
    return Ok( () );
  }

  let first_input = first_input.trim();

  // Check for quit command on first input
  if first_input.eq_ignore_ascii_case( "quit" ) || first_input.eq_ignore_ascii_case( "exit" )
  {
    println!( "👋 Goodbye!" );
    return Ok( () );
  }

  // Skip if first input is empty
  if first_input.is_empty()
  {
    println!( "👋 No input provided. Exiting." );
    return Ok( () );
  }

  // Now that we have confirmed input, set up server connection
  let ( mut client, model_name ) = setup_client_and_model().await?;

  // Initialize conversation history
  let mut conversation_history = vec![
    ChatMessage
    {
      role : MessageRole::System,
      content : "You are a helpful, friendly, and knowledgeable assistant. \
                 Provide clear, concise answers while being conversational. \
                 If you don't know something, admit it honestly.".to_string(),
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    }
  ];

  // Add the first user input to conversation
  conversation_history.push( ChatMessage
  {
    role : MessageRole::User,
    content : first_input.to_string(),
    images : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_calls : None,
  } );

  // Process first input
  let request = ChatRequest
  {
    model : model_name.clone(),
    messages : conversation_history.clone(),
    stream : Some( true ),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  handle_chat_response( &mut client, request, &mut conversation_history ).await?;

  // Continue with normal loop for subsequent inputs
  loop
  {
    // Get user input
    print!( "You : " );
    io ::stdout().flush()?;

    let mut user_input = String::new();
    let bytes_read = io::stdin().read_line( &mut user_input )?;

    // Handle EOF (no input available in non-interactive mode)
    if bytes_read == 0
    {
      println!( "\n👋 No input available (EOF). Exiting gracefully." );
      break;
    }

    let user_input = user_input.trim();

    // Check for quit command
    if user_input.eq_ignore_ascii_case( "quit" ) || user_input.eq_ignore_ascii_case( "exit" )
    {
      println!( "👋 Goodbye!" );
      break;
    }

    // Skip empty input
    if user_input.is_empty()
    {
      continue;
    }
    
    // Add user message to conversation
    conversation_history.push( ChatMessage
    {
      role : MessageRole::User,
      content : user_input.to_string(),
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    } );
    
    // Prepare streaming chat request  
    let request = ChatRequest
    {
      model : model_name.clone(),
      messages : conversation_history.clone(),
      stream : Some( true ), // Enable streaming for better responsiveness
      options : None,
      #[ cfg( feature = "tool_calling" ) ]
      tools : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_messages : None,
    };

    handle_chat_response( &mut client, request, &mut conversation_history ).await?;
    
    // Limit conversation history to last 20 messages to manage context window
    if conversation_history.len() > 21 // 1 system + 20 conversation messages
    {
      conversation_history.drain( 1..conversation_history.len() - 20 );
    }
  }
  
  Ok( () )
}