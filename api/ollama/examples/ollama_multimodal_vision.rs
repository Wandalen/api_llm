//! # Use Case 5: Multimodal AI - Image Analysis and Vision
//! 
//! This example demonstrates using vision-capable models to analyze images,
//! extract information, and provide descriptions. This opens up powerful
//! applications in automation, accessibility, and content analysis.
//! 
//! ## Real-world applications:
//! - Automated image content moderation
//! - Accessibility tools (image descriptions for visually impaired)
//! - Medical image analysis and diagnostics
//! - Quality control in manufacturing  
//! - Document processing (OCR and analysis)
//! - Security and surveillance systems
//! - E-commerce product analysis
//! - Educational content creation
//! 
//! ## To run this example:
//! ```bash
//! # Make sure Ollama is running with a vision-capable model
//! ollama pull llava  # or another vision model
//! cargo run --example multimodal_vision --all-features
//! ```

#[ cfg( feature = "vision_support" ) ]
use api_ollama::{ OllamaClient, ChatRequest, ChatMessage, MessageRole };

async fn setup_vision_client_and_model() -> Result< ( OllamaClient, String ), Box< dyn core::error::Error > >
{
  // Initialize client
  let mut client = OllamaClient::new( "http://localhost:11434".to_string(), OllamaClient::recommended_timeout_fast() );
  
  // Check if Ollama is available
  if !client.is_available().await
  {
    eprintln!( "❌ Ollama server is not available. Please start Ollama and try again." );
    std ::process::exit( 1 );
  }
  
  // Get available models - look for vision-capable models
  let models = client.list_models().await?;
  if models.models.is_empty()
  {
    eprintln!( "❌ No models available. Please install a vision model first." );
    eprintln!( "   Install with : ollama pull llava" );
    eprintln!( "   Or : ollama pull llava:13b" );
    std ::process::exit( 1 );
  }
  
  // Try to find a vision-capable model
  let model_name = models.models
    .iter()
    .find( | m | 
      m.name.to_lowercase().contains( "llava" ) || 
      m.name.to_lowercase().contains( "vision" ) ||
      m.name.to_lowercase().contains( "multimodal" )
    )
    .map_or_else( || {
      let fallback_model = &models.models[ 0 ].name;
      println!( "⚠️  No vision-specific model found. Using : {fallback_model}" );
      println!( "   For best results with images, install : ollama pull llava" );
      // Prefer smarter models over tinyllama  
      let preferred_models = [ "llama3.2:3b", "llama3.2:8b", "qwen2.5:7b", "llama3.1:8b" ];
      
      preferred_models
        .iter()
        .find( |&preferred| models.models.iter().any( |m| m.name == *preferred ) )
        .map_or_else(|| models.models[ 0 ].name.clone(), |&name| name.to_string())
    }, | m | m.name.clone() );
    
  println!( "✅ Using model : {model_name}" );
  
  Ok( ( client, model_name ) )
}

async fn run_image_analysis_scenarios(
  client : &mut OllamaClient,
  model_name : &str
) -> Result< (), Box< dyn core::error::Error > >
{
  // Image analysis scenarios
  let analysis_scenarios = vec![
    (
      "-sample_chart.txt",
      "Please analyze this chart/diagram. What data does it show? What insights can you extract?",
      "Chart Analysis"
    ),
    (
      "-sample_document.txt",
      "Can you read and extract the text from this document? Summarize the key information.",
      "Document OCR"
    ),
    (
      "-sample_scene.txt",
      "Describe this image in detail. What do you see? What's happening in the scene?",
      "Scene Description"
    ),
  ];
  
  // Analyze each image scenario
  for ( image_file, prompt, scenario_name ) in analysis_scenarios
  {
    println!( "\n🖼️  Scenario : {scenario_name}" );
    println!( "═══════════════════════════════════════════" );
    
    if let Ok( image_data ) = load_image_as_base64( image_file )
    {
      println!( "📁 Loaded image : {image_file}" );
      
      let messages = vec![
        ChatMessage
        {
          role : MessageRole::System,
          content : "You are an expert image analyst. Provide detailed, accurate descriptions \
                     and analysis of images. Focus on extracting useful information and insights.".to_string(),
          images : None,
          #[ cfg( feature = "tool_calling" ) ]
          tool_calls : None,
        },
        ChatMessage
        {
          role : MessageRole::User,
          content : prompt.to_string(),
          images : Some( vec![ image_data ] ),
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
      
      match client.chat( request ).await
      {
        Ok( response ) =>
        {
          println!( "🔍 Analysis Result:" );
          let content = &response.message.content;
          println!( "{content}" );
        }
        Err( e ) =>
        {
          eprintln!( "❌ Error analyzing image : {e}" );
          if e.to_string().contains( "vision" ) || e.to_string().contains( "multimodal" )
          {
            eprintln!( "   💡 This model may not support vision. Try : ollama pull llava" );
          }
        }
      }
    }
    else
    {
      println!( "⚠️  Could not load image : {image_file}" );
    }
  }
  
  Ok( () )
}

async fn run_multi_image_comparison(
  client : &mut OllamaClient,
  model_name : &str
) -> Result< (), Box< dyn core::error::Error > >
{
  println!( "\n🔄 Multi-Image Comparison Demo" );
  println!( "=============================" );
  
  let compare_images = vec![ "-sample_chart.txt", "-sample_document.txt" ];
  let mut image_data_list = Vec::new();
  
  for image_file in &compare_images
  {
    if let Ok( data ) = load_image_as_base64( image_file )
    {
      image_data_list.push( data );
    }
  }
  
  if image_data_list.len() >= 2
  {
    let comparison_request = ChatRequest
    {
      model : model_name.to_string(),
      messages : vec![ ChatMessage
      {
        role : MessageRole::User,
        content : "I'm showing you multiple images. Please compare and contrast them. \
                   What are the differences and similarities? What type of content is each?".to_string(),
        images : Some( image_data_list ),
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
    
    match client.chat( comparison_request ).await
    {
      Ok( response ) =>
      {
        println!( "🔍 Comparison Result:" );
        let content = &response.message.content;
        println!( "{content}" );
      }
      Err( e ) =>
      {
        eprintln!( "❌ Error in image comparison : {e}" );
      }
    }
  }
  
  Ok( () )
}

async fn run_interactive_analysis(
  client : &mut OllamaClient,
  model_name : &str
) -> Result< (), Box< dyn core::error::Error > >
{
  println!( "\n🎯 Interactive Image Analysis" );
  println!( "=============================" );
  println!( "You can now ask specific questions about images!" );
  println!( "Commands : 'quit' to exit, 'help' for more options\n" );
  
  loop
  {
    print!( "Enter your question about images (or 'quit'): " );
    std ::io::stdout().flush().unwrap();

    let mut input = String::new();
    let bytes_read = std::io::stdin().read_line( &mut input )?;

    // Handle EOF (no input available in non-interactive mode)
    if bytes_read == 0
    {
      println!( "\n👋 No input available (EOF). Exiting gracefully." );
      println!( "Note : Use this example in interactive terminal only." );
      break;
    }

    let input = input.trim();

    if input.eq_ignore_ascii_case( "quit" ) || input.eq_ignore_ascii_case( "exit" )
    {
      break;
    }

    if input.eq_ignore_ascii_case( "help" )
    {
      println!( "💡 Try these example questions:" );
      println!( "   - What colors are dominant in the chart?" );
      println!( "   - Can you read the text in the document?" );
      println!( "   - What objects do you see in the scene?" );
      println!( "   - Is there any text visible in any of the images?" );
      continue;
    }

    if input.is_empty()
    {
      continue;
    }
    
    // Use the first available sample image for interactive analysis
    if let Ok( image_data ) = load_image_as_base64( "-sample_scene.txt" )
    {
      let interactive_request = ChatRequest
      {
        model : model_name.to_string(),
        messages : vec![ ChatMessage
        {
          role : MessageRole::User,
          content : input.to_string(),
          images : Some( vec![ image_data ] ),
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
      
      match client.chat( interactive_request ).await
      {
        Ok( response ) =>
        {
          let content = &response.message.content;
          println!( "🤖 {content}\n" );
        }
        Err( e ) =>
        {
          eprintln!( "❌ Error : {e}\n" );
        }
      }
    }
  }
  
  Ok( () )
}
#[ cfg( feature = "vision_support" ) ]
use std::fs;
#[ cfg( feature = "vision_support" ) ]
use std::path::Path;
#[ cfg( feature = "vision_support" ) ]
use base64::{ Engine as _, engine::general_purpose };
#[ cfg( feature = "vision_support" ) ]
use std::io::Write;

#[ cfg( feature = "vision_support" ) ]
#[ tokio::main ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "👁️  Multimodal AI - Image Analysis and Vision" );
  println!( "============================================" );
  
  let ( mut client, model_name ) = setup_vision_client_and_model().await?;
  
  // Create sample images if they don't exist
  create_sample_images()?;
  
  // Run image analysis scenarios
  run_image_analysis_scenarios( &mut client, &model_name ).await?;
  
  // Run multi-image comparison demo
  run_multi_image_comparison( &mut client, &model_name ).await?;
  
  // Run interactive analysis
  run_interactive_analysis( &mut client, &model_name ).await?;
  
  println!( "✅ Vision analysis complete!" );
  println!( "💡 Multimodal AI enables:" );
  println!( "   - Automated image content analysis" );
  println!( "   - OCR and document processing" );
  println!( "   - Accessibility features" );
  println!( "   - Quality control and inspection" );
  println!( "   - Creative content generation" );
  
  Ok( () )
}

#[ cfg( feature = "vision_support" ) ]
fn create_sample_images() -> Result< (), Box< dyn core::error::Error > >
{
  // Create text representations of different types of images for demonstration
  // In a real application, these would be actual image files
  
  let sample_chart = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNDAwIiBoZWlnaHQ9IjMwMCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB3aWR0aD0iMTAwJSIgaGVpZ2h0PSIxMDAlIiBmaWxsPSJ3aGl0ZSIvPgogIDx0ZXh0IHg9IjIwMCIgeT0iMzAiIHRleHQtYW5jaG9yPSJtaWRkbGUiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxOCIgZm9udC13ZWlnaHQ9ImJvbGQiPlNhbGVzIERhdGEgMjAyMzwvdGV4dD4KICA8cmVjdCB4PSI1MCIgeT0iMTAwIiB3aWR0aD0iNjAiIGhlaWdodD0iMTAwIiBmaWxsPSIjNGY4MWJkIi8+CiAgPHJlY3QgeD0iMTQwIiB5PSI3MCIgd2lkdGg9IjYwIiBoZWlnaHQ9IjEzMCIgZmlsbD0iIzVkYjViOCIvPgogIDxyZWN0IHg9IjIzMCIgeT0iOTAiIHdpZHRoPSI2MCIgaGVpZ2h0PSIxMTAiIGZpbGw9IiNmZjc1NDMiLz4KICA8cmVjdCB4PSIzMjAiIHk9IjUwIiB3aWR0aD0iNjAiIGhlaWdodD0iMTUwIiBmaWxsPSIjZmZkMDAwIi8+CiAgPHRleHQgeD0iODAiIHk9IjIzMCIgdGV4dC1hbmNob3I9Im1pZGRsZSIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjEyIj5RMTwvdGV4dD4KICA8dGV4dCB4PSIxNzAiIHk9IjIzMCIgdGV4dC1hbmNob3I9Im1pZGRsZSIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjEyIj5RMjwvdGV4dD4KICA8dGV4dCB4PSIyNjAiIHk9IjIzMCIgdGV4dC1hbmNob3I9Im1pZGRsZSIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjEyIj5RMzwvdGV4dD4KICA8dGV4dCB4PSIzNTAiIHk9IjIzMCIgdGV4dC1hbmNob3I9Im1pZGRsZSIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjEyIj5RNDwvdGV4dD4KPC9zdmc+";
  
  fs ::write( "-sample_chart.txt", sample_chart )?;
  
  let sample_document = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNjAwIiBoZWlnaHQ9IjgwMCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB3aWR0aD0iMTAwJSIgaGVpZ2h0PSIxMDAlIiBmaWxsPSJ3aGl0ZSIvPgogIDx0ZXh0IHg9IjUwIiB5PSI4MCI+CiAgICA8dHNwYW4geD0iNTAiIGR5PSIwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMjQiIGZvbnQtd2VpZ2h0PSJib2xkIj5JTlZPSUNFPC90c3Bhbj4KICA8L3RleHQ+CiAgPHRleHQgeD0iNTAiIHk9IjE0MCI+CiAgICA8dHNwYW4geD0iNTAiIGR5PSIwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTYiPlRlY2ggU29sdXRpb25zIEluYy48L3RzcGFuPgogICAgPHRzcGFuIHg9IjUwIiBkeT0iMjUiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCI+MTIzIEJ1c2luZXNzIEF2ZTwvdHNwYW4+CiAgICA8dHNwYW4geD0iNTAiIGR5PSIyMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0Ij5DaXR5LCBTVCAxMjM0NTwvdHNwYW4+CiAgPC90ZXh0PgogIDx0ZXh0IHg9IjUwIiB5PSIyNDAiPgogICAgPHRzcGFuIHg9IjUwIiBkeT0iMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE2IiBmb250LXdlaWdodD0iYm9sZCI+QmlsbCBUbzo8L3RzcGFuPgogICAgPHRzcGFuIHg9IjUwIiBkeT0iMzAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCI+Sm9obiBTbWl0aDwvdHNwYW4+CiAgICA8dHNwYW4geD0iNTAiIGR5PSIyMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0Ij40NTYgQ2xpZW50IFJkPC90c3Bhbj4KICAgIDx0c3BhbiB4PSI1MCIgZHk9IjIwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiPkNpdHksIFNUIDY3ODkwPC90c3Bhbj4KICA8L3RleHQ+CiAgPHRleHQgeD0iNTAiIHk9IjQwMCI+CiAgICA8dHNwYW4geD0iNTAiIGR5PSIwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTYiIGZvbnQtd2VpZ2h0PSJib2xkIj5TZXJ2aWNlczogQ29uc3VsdGluZzwvdHNwYW4+CiAgICA8dHNwYW4geD0iNTAiIGR5PSIzMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0Ij5Ib3VyczogNDA8L3RzcGFuPgogICAgPHRzcGFuIHg9IjUwIiBkeT0iMjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCI+UmF0ZTogJDE1MC9ocjwvdHNwYW4+CiAgICA8dHNwYW4geD0iNTAiIGR5PSIzMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE2IiBmb250LXdlaWdodD0iYm9sZCI+VG90YWw6ICQ2LDAwMDwvdHNwYW4+CiAgPC90ZXh0Pgo8L3N2Zz4=";
  
  fs ::write( "-sample_document.txt", sample_document )?;
  
  let sample_scene = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iODAwIiBoZWlnaHQ9IjYwMCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8IS0tIFNreSAtLT4KICA8cmVjdCB3aWR0aD0iMTAwJSIgaGVpZ2h0PSI0MCUiIGZpbGw9IiM4N0NFRUIiLz4KICA8IS0tIEdyb3VuZCAtLT4KICA8cmVjdCB5PSI0MCUiIHdpZHRoPSIxMDAlIiBoZWlnaHQ9IjYwJSIgZmlsbD0iIzIyOEIyMiIvPgogIDwhLS0gU3VuIC0tPgogIDxjaXJjbGUgY3g9IjcwMCIgY3k9IjEwMCIgcj0iNjAiIGZpbGw9IiNGRkQ3MDAiLz4KICA8IS0tIEhvdXNlIC0tPgogIDxyZWN0IHg9IjMwMCIgeT0iMjAwIiB3aWR0aD0iMjAwIiBoZWlnaHQ9IjE4MCIgZmlsbD0iI0E1MkEyQSIvPgogIDwhLS0gUm9vZiAtLT4KICA8cG9seWdvbiBwb2ludHM9IjI4MCwyMDAgNDAwLDEyMCA1MjAsMjAwIiBmaWxsPSIjOEIwMDAwIi8+CiAgPCEtLSBEb29yIC0tPgogIDxyZWN0IHg9IjM3MCIgeT0iMzAwIiB3aWR0aD0iNjAiIGhlaWdodD0iODAiIGZpbGw9IiM2NTQzMjEiLz4KICA8IS0tIFdpbmRvd3MgLS0+CiAgPHJlY3QgeD0iMzIwIiB5PSIyMzAiIHdpZHRoPSI0MCIgaGVpZ2h0PSI0MCIgZmlsbD0iIzg3Q0VFQiIvPgogIDxyZWN0IHg9IjQ0MCIgeT0iMjMwIiB3aWR0aD0iNDAiIGhlaWdodD0iNDAiIGZpbGw9IiM4N0NFRUII+PC9yZWN0PgogIDwhLS0gVHJlZSAtLT4KICA8cmVjdCB4PSIxNDAiIHk9IjI4MCIgd2lkdGg9IjQwIiBoZWlnaHQ9IjEyMCIgZmlsbD0iIzY1NDMyMSIvPgogIDxjaXJjbGUgY3g9IjE2MCIgY3k9IjI0MCIgcj0iNzAiIGZpbGw9IiMyMjhCMjIiLz4KICA8IS0tIENsb3VkcyAtLT4KICA8ZWxsaXBzZSBjeD0iMjAwIiBjeT0iMTAwIiByeD0iODAiIHJ5PSI0MCIgZmlsbD0id2hpdGUiIG9wYWNpdHk9IjAuOCIvPgogIDxlbGxpcHNlIGN4PSI2MDAiIGN5PSI4MCIgcng9IjYwIiByeT0iMzAiIGZpbGw9IndoaXRlIiBvcGFjaXR5PSIwLjgiLz4KPC9zdmc+";
  
  fs ::write( "-sample_scene.txt", sample_scene )?;
  
  println!( "✅ Created sample image files for demonstration" );
  
  Ok( () )
}

#[ cfg( feature = "vision_support" ) ]
fn load_image_as_base64( file_path : &str ) -> Result< String, Box< dyn core::error::Error > >
{
  if !Path::new( file_path ).exists()
  {
    return Err( format!( "File not found : {file_path}" ).into() );
  }
  
  let data = fs::read_to_string( file_path )?;
  
  // If it's already base64 encoded data URL, extract the base64 part
  if data.starts_with( "data:" )
  {
    if let Some( base64_part ) = data.split( ',' ).nth( 1 )
    {
      return Ok( base64_part.to_string() );
    }
  }
  
  // Otherwise, assume it's binary data and encode it
  let binary_data = fs::read( file_path )?;
  Ok( general_purpose::STANDARD.encode( binary_data ) )
}

// Fallback for when vision features are not enabled
#[ cfg( not( feature = "vision_support" ) ) ]
#[ tokio::main ]
async fn main() -> Result< (), Box< dyn core::error::Error > >
{
  println!( "❌ This example requires the 'vision_support' feature to be enabled." );
  println!( "Please run with : cargo run --example multimodal_vision --features vision_support" );
  println!( "Or use --all-features to enable all features." );
  println!( "\nAlso make sure you have a vision-capable model installed:" );
  println!( "ollama pull llava" );
  
  Ok( () )
}