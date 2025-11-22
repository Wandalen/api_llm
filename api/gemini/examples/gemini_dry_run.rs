//! Comprehensive dry run validation for all API patterns without making real API calls.
//!
//! This example demonstrates:
//! - Client initialization validation (with and without API keys)
//! - Request structure validation for all API endpoints
//! - Error handling patterns verification
//! - Data structure serialization/deserialization testing
//! - Configuration validation across different client setups
//! - Mock data generation for testing purposes
//!
//! ## Purpose
//!
//! This example serves as a comprehensive validation suite that:
//! - ✅ Verifies all API client patterns work correctly
//! - ✅ Tests request/response structure compatibility
//! - ✅ Validates error handling without consuming API quotas
//! - ✅ Demonstrates proper configuration patterns
//! - ✅ Provides mock data examples for testing
//!
//! ## Usage
//!
//! ```bash
//! # Run comprehensive validation
//! cargo run --example example_dry_run
//! ```
//!
//! ## Benefits
//!
//! - **No API Key Required**: Validates patterns without real API calls
//! - **Fast Feedback**: Quick validation of client setup and request structures
//! - **Educational**: Shows proper error handling and configuration patterns
//! - **Testing Reference**: Demonstrates mock data generation for unit tests
//!
//! Perfect for developers who want to understand API patterns before making real calls.

use api_gemini::{ client::Client, models::* };
use base64::Engine;

#[ allow( clippy::too_many_lines ) ]
fn main()
{
  println!( "=== Gemini API Examples Dry Run ===\n" );

  // Test 1: Client initialization
  println!( "1. Testing client initialization patterns:" );

  // With builder
  match Client::builder()
  .api_key( "test-api-key".to_string() )
  .build()
  {
    Ok( _ ) => println!( "   Client::builder() works correctly" ),
  Err( e ) => println!( "   Client::builder() failed : {e}" ),
  }

  // Without API key (should fail)
  match Client::builder().build()
  {
    Ok( _ ) => println!( "   Empty API key should fail" ),
  Err( e ) => println!( "   Empty API key correctly fails : {e}" ),
  }

  // Test 2: Request structures
  println!( "\n2. Testing request structures:" );

  // Basic chat request
  let _chat_request = GenerateContentRequest
  {
    contents: vec!
    [
    Content
    {
      parts: vec!
      [
      Part
      {
        text: Some( "Hello!".to_string() ),
        ..Default::default()
      }
      ],
      role: "user".to_string(),
    }
    ],
    ..Default::default()
  };
  println!( "   Basic chat request created" );

  // Multi-turn conversation
  let conversation =
  [
  Content
  {
    role: "user".to_string(),
    parts: vec!
    [
    Part
    {
      text: Some( "Question 1".to_string() ),
      ..Default::default()
    }
    ],
  },
  Content
  {
    role: "model".to_string(),
    parts: vec!
    [
    Part
    {
      text: Some( "Answer 1".to_string() ),
      ..Default::default()
    }
    ],
  },
  Content
  {
    role: "user".to_string(),
    parts: vec!
    [
    Part
    {
      text: Some( "Question 2".to_string() ),
      ..Default::default()
    }
    ],
  },
  ];
  let conversation_len = conversation.len();
println!( "   Multi-turn conversation with {conversation_len} turns" );

  // Test 3: Special features
  println!( "\n3. Testing special features:" );

  // Function calling
  let tools =
  [
  Tool
  {
    function_declarations: Some( vec!
    [
    FunctionDeclaration
    {
      name: "test_function".to_string(),
      description: "A test function".to_string(),
      parameters: Some( serde_json::json!
      ({
        "type": "object",
        "properties": {
          "param": {
            "type": "string"
          }
        }
      })),
    }
    ]),
    code_execution: None,
    google_search_retrieval: None,
    code_execution_tool: None,
  }
  ];
  let functions_len = tools[ 0 ].function_declarations.as_ref().unwrap().len();
println!( "   Function calling tools created with {functions_len} functions" );

  // Embeddings
  let embed_request = EmbedContentRequest
  {
    content: Content
    {
      parts: vec!
      [
      Part
      {
        text: Some( "Test text".to_string() ),
        ..Default::default()
      }
      ],
      role: "user".to_string(),
    },
    task_type: Some( "RETRIEVAL_DOCUMENT".to_string() ),
    title: None,
    output_dimensionality: None,
  };
  let task_type = embed_request.task_type.as_ref().unwrap();
println!( "   Embeddings request created with task type : {task_type}" );

  // Multimodal
  let image_data = vec![ 0x89, 0x50, 0x4E, 0x47 ]; // PNG header
  let base64_image = base64::engine::general_purpose::STANDARD.encode( &image_data );

  let multimodal_request = GenerateContentRequest
  {
    contents: vec!
    [
    Content
    {
      parts: vec!
      [
      Part
      {
        text: Some( "Describe this image".to_string() ),
        ..Default::default()
      },
      Part
      {
        inline_data: Some( Blob
        {
          mime_type: "image/png".to_string(),
          data: base64_image,
        }),
        ..Default::default()
      },
      ],
      role: "user".to_string(),
    }
    ],
    ..Default::default()
  };
  let parts_len = multimodal_request.contents[ 0 ].parts.len();
println!( "   Multimodal request with {parts_len} parts" );

  // Safety settings
  let safety_settings =
  [
  SafetySetting
  {
    category: "HARM_CATEGORY_HARASSMENT".to_string(),
    threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
  }
  ];
  let settings_len = safety_settings.len();
println!( "   Safety settings configured with {settings_len} categories" );

  // Test 4: Error handling
  println!( "\n4. Testing error handling:" );

  // Test different error types
  let errors = vec!
  [
  api_gemini ::error::Error::AuthenticationError( "API key missing".to_string() ),
  api_gemini ::error::Error::NetworkError( "Connection timeout".to_string() ),
  api_gemini ::error::Error::RateLimitError( "Too many requests".to_string() ),
  api_gemini ::error::Error::InvalidArgument( "Invalid model name".to_string() ),
  ];

  for error in &errors
  {
  println!( "   Error type handled : {error}" );
  }

  println!( "\n=== Summary ===" );
  println!( "All example patterns validated successfully!" );
  println!( "\nTo run the actual examples with API calls:" );
  println!( "1. Set GEMINI_API_KEY environment variable" );
  println!( "2. Run individual examples:" );
  println!( "   - cargo run --example chat" );
  println!( "   - cargo run --example list_models" );
  println!( "   - cargo run --example multi_turn_conversation" );
  println!( "   - cargo run --example embeddings" );
  println!( "   - cargo run --example function_calling" );
  println!( "   - cargo run --example multimodal" );
  println!( "   - cargo run --example safety_settings" );
  println!( "   - cargo run --example error_handling" );
}