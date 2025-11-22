//! # Use Case 4: Real-time Streaming Conversations
//! 
//! This example demonstrates streaming responses for real-time conversational
//! experiences. Streaming is crucial for responsive user interfaces where users
//! expect to see responses appear progressively rather than waiting for complete
//! responses.
//! 
//! ## Real-world applications:
//! - Live chat interfaces and messaging apps
//! - Interactive voice assistants with real-time feedback
//! - Streaming content generation for writing tools
//! - Real-time code completion and suggestions
//! - Live translation services
//! - Interactive gaming NPCs with dynamic dialogue
//! 
//! ## To run this example:
//! ```bash
//! # Make sure Ollama is running with a model installed
//! ollama pull llama3.2
//! cargo run --example streaming_chat --all-features
//! ```

#[ cfg( feature = "streaming" ) ]
use api_ollama::{ OllamaClient, ChatRequest, ChatMessage, MessageRole };
#[ cfg( feature = "streaming" ) ]
use futures_util::StreamExt;
#[ cfg( feature = "streaming" ) ]
use std::io::{ self, Write };

#[ cfg( feature = "streaming" ) ]
async fn handle_streaming_response(
  client : &mut OllamaClient,
  request : ChatRequest,
  conversation_history : &mut Vec< ChatMessage >
) -> Result< (), Box< dyn core::error::Error > >
{
  print!( "🤖 Assistant : " );
  io ::stdout().flush()?;
  
  // Stream the response
  let mut full_response = String::new();
  match client.chat_stream( request ).await
  {
    Ok( mut stream ) =>
    {
      while let Some( chunk ) = stream.next().await
      {
        match chunk
        {
          Ok( response ) =>
          {
            if !response.done
            {
              let content = response.message.content;
              print!( "{content}" );
              io ::stdout().flush()?;
              full_response.push_str( &content );
            }
          }
          Err( e ) =>
          {
            eprintln!( "\n❌ Stream error : {e}" );
            break;
          }
        }
      }
      println!( "\n" ); // New line after response
      
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
    }
  }
  
  Ok( () )
}

#[ cfg( feature = "streaming" ) ]
async fn setup_client_and_model() -> Result< ( OllamaClient, String ), Box< dyn core::error::Error > >
{
  // Initialize client
  let mut client = OllamaClient::new( "http://localhost:11434".to_string(), OllamaClient::recommended_timeout_fast() );
  
  // Check if Ollama is available
  if !client.is_available().await
  {
    eprintln!( "❌ Ollama server is not available. Please start Ollama and try again." );
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

#[ cfg( feature = "streaming" ) ]
#[ tokio::main ]
#[ allow( clippy::too_many_lines ) ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "⚡ Real-time Streaming Chat Assistant" );
  println!( "====================================" );

  // Fix(issue-eof-before-server-002): Check for EOF BEFORE connecting to server
  // Root cause: EOF check was inside main loop after server setup, so empty stdin caused server connection failure before reaching EOF handling
  // Pitfall: Interactive examples should handle "no input" gracefully without requiring external dependencies (server)

  // Check for input availability before attempting server connection
  println!( "\n💬 Streaming chat started! You'll see responses appear in real-time." );
  println!( "Type 'quit' to exit, 'demo' for demonstration scenarios.\n" );
  print!( "You : " );
  io ::stdout().flush()?;

  let mut first_input = String::new();
  let bytes_read = io::stdin().read_line( &mut first_input )?;

  // Handle EOF immediately - no server needed if there's no input
  if bytes_read == 0
  {
    println!( "\n👋 No input available (EOF). Exiting gracefully." );
    println!( "Note : Use this example in interactive terminal only." );
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

  // NOW set up server connection (only after confirming we have input)
  let ( mut client, model_name ) = setup_client_and_model().await?;

  // Initialize conversation history
  let mut conversation_history = vec![
    ChatMessage
    {
      role : MessageRole::System,
      content : "You are a helpful assistant. Provide engaging, informative responses. \
                 When appropriate, use examples and ask follow-up questions to keep \
                 the conversation flowing.".to_string(),
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    }
  ];

  // Handle demo scenario if requested
  if first_input.eq_ignore_ascii_case( "demo" )
  {
    run_demo_scenarios( &mut client, &model_name ).await?;
  }
  else
  {
    // Add first user message to conversation
    conversation_history.push( ChatMessage
    {
      role : MessageRole::User,
      content : first_input.to_string(),
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    } );

    // Prepare streaming chat request
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

    // Handle the streaming response for first input
    handle_streaming_response( &mut client, request, &mut conversation_history ).await?;
  }

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
      println!( "Note : Use this example in interactive terminal only." );
      break;
    }

    let user_input = user_input.trim();

    // Handle special commands
    if user_input.eq_ignore_ascii_case( "quit" ) || user_input.eq_ignore_ascii_case( "exit" )
    {
      println!( "👋 Goodbye!" );
      break;
    }

    if user_input.eq_ignore_ascii_case( "demo" )
    {
      run_demo_scenarios( &mut client, &model_name ).await?;
      continue;
    }

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
      stream : Some( true ), // Enable streaming
      options : None,
      #[ cfg( feature = "tool_calling" ) ]
      tools : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_messages : None,
    };

    // Handle the streaming response
    handle_streaming_response( &mut client, request, &mut conversation_history ).await?;

    // Manage conversation history length
    if conversation_history.len() > 21
    {
      conversation_history.drain( 1..conversation_history.len() - 20 );
    }
  }

  Ok( () )
}

#[ cfg( feature = "streaming" ) ]
async fn run_demo_scenarios( client : &mut OllamaClient, model_name : &str ) -> Result< (), Box< dyn core::error::Error > >
{
  println!( "\n🎭 Demonstration Scenarios" );
  println!( "=========================" );
  
  let demo_scenarios = vec![
    (
      "Creative Writing",
      "Write a short, dramatic story about a detective discovering a mysterious letter. \
       Make it engaging and suspenseful.",
    ),
    (
      "Technical Explanation", 
      "Explain how neural networks work, using analogies that a non-technical person \
       could understand. Include real-world examples.",
    ),
    (
      "Problem Solving",
      "I have a small apartment and want to create a home office space. \
       What are some creative, space-efficient solutions?",
    ),
  ];
  
  for ( scenario_name, prompt ) in demo_scenarios
  {
    println!( "\n🎬 Scenario : {scenario_name}" );
    println!( "Prompt : {prompt}" );
    println!( "─────────────────────────────────────────" );
    print!( "🤖 Streaming Response : " );
    io ::stdout().flush().unwrap();
    
    let request = ChatRequest
    {
      model : model_name.to_string(),
      messages : vec![ ChatMessage
      {
        role : MessageRole::User,
        content : prompt.to_string(),
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      } ],
      stream : Some( true ),
      options : None,
      #[ cfg( feature = "tool_calling" ) ]
      tools : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_messages : None,
    };
    
    match client.chat_stream( request ).await
    {
      Ok( mut stream ) =>
      {
        while let Some( chunk ) = stream.next().await
        {
          match chunk
          {
            Ok( response ) =>
            {
              if !response.done
              {
                let content = &response.message.content;
                print!( "{content}" );
                io ::stdout().flush().unwrap();
                
                // Add small delay to make streaming effect more visible
                tokio ::time::sleep( tokio::time::Duration::from_millis( 10 ) ).await;
              }
            }
            Err( e ) =>
            {
              eprintln!( "\n❌ Stream error : {e}" );
              break;
            }
          }
        }
        println!( "\n" );
      }
      Err( e ) =>
      {
        eprintln!( "❌ Error in demo scenario : {e}" );
      }
    }
  }
  
  println!( "✅ Demo scenarios complete!" );
  println!( "💡 Notice how responses appear progressively, creating a more" );
  println!( "   engaging and responsive user experience compared to waiting" );
  println!( "   for complete responses.\n" );
  
  Ok( () )
}

// Fallback for when streaming feature is not enabled
#[ cfg( not( feature = "streaming" ) ) ]
#[ tokio::main ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "❌ This example requires the 'streaming' feature to be enabled." );
  println!( "Please run with : cargo run --example streaming_chat --features streaming" );
  println!( "Or use --all-features to enable all features." );
  
  Ok( () )
}