//! # Use Case 2: Document Analysis and Summarization
//! 
//! This example demonstrates using local LLMs for document analysis, summarization,
//! and content extraction. This is extremely valuable for processing large amounts
//! of text data privately without sending sensitive information to external APIs.
//! 
//! ## Real-world applications:
//! - Legal document review and summarization
//! - Research paper analysis  
//! - Business report processing
//! - Content moderation and classification
//! - Privacy-sensitive document processing
//! 
//! ## To run this example:
//! ```bash
//! # Make sure Ollama is running with a model installed
//! ollama pull llama3.2
//! echo "Your document content here..." > -sample_document.txt
//! cargo run --example document_analyzer --all-features
//! ```

use api_ollama::{ OllamaClient, ChatRequest, ChatMessage, MessageRole };
use std::fs;
use std::path::Path;

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

async fn perform_document_analysis(
  client : &mut OllamaClient,
  model_name : &str,
  document : &str
) -> Result< (), Box< dyn core::error::Error > >
{
  // Analysis tasks to perform
  let analysis_tasks = [
    ( "Summary", "Please provide a concise summary of this document in 2-3 sentences." ),
    ( "Key Points", "Extract the 3-5 most important key points from this document as bullet points." ),
    ( "Sentiment", "What is the overall sentiment or tone of this document? Is it optimistic, cautious, neutral, or concerned?" ),
    ( "Categories", "What categories or topics does this document cover? Provide relevant tags." ),
    ( "Action Items", "Are there any implied action items or recommendations in this document?" ),
  ];
  
  // Perform each analysis task with delays to prevent connection exhaustion
  for ( i, ( task_name, prompt ) ) in analysis_tasks.iter().enumerate()
  {
    // Add delay between requests to prevent overwhelming the server
    if i > 0
    {
      tokio ::time::sleep( tokio::time::Duration::from_millis( 500 ) ).await;
    }

    println!( "🔍 Analyzing : {task_name}" );
    println!( "─────────────────────────" );

    let messages = vec![
      ChatMessage
      {
        role : MessageRole::System,
        content : "You are an expert document analyst. Provide clear, accurate, and insightful analysis.".to_string(),
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      },
      ChatMessage
      {
        role : MessageRole::User,
        content : format!( "{prompt}\n\nDocument:\n{document}" ),
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      }
    ];

    let request = ChatRequest
    {
      model : model_name.to_string(),
      messages,
      stream : Some( false ),
      options : None,
      #[ cfg( feature = "tool_calling" ) ]
      tools : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_messages : None,
    };

    // Retry logic with exponential backoff
    let mut attempts : u32 = 0;
    let max_attempts : u32 = 3;
    let mut success = false;

    while attempts < max_attempts && !success
    {
      match client.chat( request.clone() ).await
      {
        Ok( response ) =>
        {
          let content = &response.message.content;
          println!( "{content}" );
          println!( "\n" );
          success = true;
        }
        Err( e ) =>
        {
          attempts += 1;
          if attempts < max_attempts
          {
            let delay = 1000 * ( 2_u64.pow( attempts ) ); // Exponential backoff
            eprintln!( "⚠️  Attempt {attempts} failed for {task_name}: {e}" );
            eprintln!( "   Retrying in {delay}ms..." );
            tokio ::time::sleep( tokio::time::Duration::from_millis( delay ) ).await;
          }
          else
          {
            eprintln!( "❌ Error analyzing {task_name} after {max_attempts} attempts : {e}" );
          }
        }
      }
    }
  }
  
  Ok( () )
}

async fn generate_document_statistics(
  client : &mut OllamaClient,
  model_name : &str,
  document : &str
) -> Result< (), Box< dyn core::error::Error > >
{
  // Add delay before this final request
  tokio ::time::sleep( tokio::time::Duration::from_millis( 500 ) ).await;

  println!( "📊 Document Processing Statistics" );
  println!( "================================" );

  let stats_prompt = format!(
    "Analyze this document and provide statistics:\n\
     - Approximate word count\n\
     - Reading level/complexity\n\
     - Main subjects discussed\n\
     - Document type classification\n\n\
     Document:\n{document}"
  );

  let request = ChatRequest
  {
    model : model_name.to_string(),
    messages : vec![ ChatMessage
    {
      role : MessageRole::User,
      content : stats_prompt,
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    } ],
    stream : Some( false ),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  // Retry logic with exponential backoff
  let mut attempts : u32 = 0;
  let max_attempts : u32 = 3;
  let mut success = false;

  while attempts < max_attempts && !success
  {
    match client.chat( request.clone() ).await
    {
      Ok( response ) =>
      {
        let content = &response.message.content;
        println!( "{content}" );
        success = true;
      }
      Err( e ) =>
      {
        attempts += 1;
        if attempts < max_attempts
        {
          let delay = 1000 * ( 2_u64.pow( attempts ) ); // Exponential backoff
          eprintln!( "⚠️  Attempt {attempts} failed for statistics : {e}" );
          eprintln!( "   Retrying in {delay}ms..." );
          tokio ::time::sleep( tokio::time::Duration::from_millis( delay ) ).await;
        }
        else
        {
          eprintln!( "❌ Error generating statistics after {max_attempts} attempts : {e}" );
        }
      }
    }
  }

  Ok( () )
}

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "📄 Document Analysis and Summarization Tool" );
  println!( "===========================================" );
  
  let ( mut client, model_name ) = setup_client_and_model().await?;
  
  // Sample document (in real use, this would be loaded from files)
  let sample_document = if Path::new( "-sample_document.txt" ).exists()
  {
    fs ::read_to_string( "-sample_document.txt" )?
  }
  else
  {
    // Create a sample document for demonstration
    let sample = r"
Artificial Intelligence (AI) has rapidly evolved from a theoretical concept to a transformative 
technology that impacts nearly every aspect of modern life. Machine learning, a subset of AI, 
enables computers to learn and improve from experience without being explicitly programmed.

Deep learning, which uses neural networks with multiple layers, has revolutionized fields such 
as image recognition, natural language processing, and autonomous systems. Companies across 
industries are leveraging AI to automate processes, enhance decision-making, and create new 
products and services.

However, the rapid advancement of AI also presents challenges. Concerns about job displacement, 
algorithmic bias, data privacy, and the concentration of AI capabilities in the hands of a few 
large corporations have sparked important discussions about responsible AI development.

The future of AI holds immense promise, with potential breakthroughs in healthcare, climate 
change mitigation, scientific research, and education. As we continue to develop these 
technologies, it is crucial to ensure they are developed and deployed in ways that benefit 
humanity as a whole while minimizing potential risks and negative consequences.
    ".trim().to_string();
    
    fs ::write( "-sample_document.txt", &sample )?;
    println!( "📝 Created sample document : -sample_document.txt" );
    sample
  };
  
  let char_count = sample_document.len();
  println!( "📖 Document loaded : {char_count} characters" );
  println!( "\n" );
  
  // Perform document analysis using helper function
  perform_document_analysis( &mut client, &model_name, &sample_document ).await?;
  
  // Generate document statistics using helper function  
  generate_document_statistics( &mut client, &model_name, &sample_document ).await?;
  
  println!( "\n✅ Document analysis complete!" );
  println!( "💡 In production, you could:" );
  println!( "   - Process multiple documents in batch" );
  println!( "   - Extract structured data to databases" );
  println!( "   - Generate automated reports" );
  println!( "   - Classify documents by type/topic" );
  
  Ok( () )
}
