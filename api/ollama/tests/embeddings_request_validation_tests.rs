//! Embeddings API unit tests for `api_ollama`
//! 
//! These tests verify embeddings request construction and validation
//! without requiring server interaction.

#![ cfg( feature = "embeddings" ) ]

use api_ollama::{ EmbeddingsRequest };
use std::collections::HashMap;

#[ test ]
fn test_embeddings_request_creation()
{
  let request = EmbeddingsRequest
  {
    model : "test-model".to_string(),
    prompt : "Hello world".to_string(),
    options : None,
  };
  
  assert_eq!( request.model, "test-model" );
  assert_eq!( request.prompt, "Hello world" );
  assert!( request.options.is_none() );
}

#[ test ]
fn test_embeddings_request_with_options()
{
  let mut options = HashMap::new();
  options.insert( "temperature".to_string(), serde_json::Value::from( 0.5 ) );
  options.insert( "dimension".to_string(), serde_json::Value::from( 2048 ) );
  
  let request = EmbeddingsRequest
  {
    model : "embedding-model".to_string(),
    prompt : "Machine learning embeddings".to_string(),
    options : Some( options ),
  };
  
  assert_eq!( request.model, "embedding-model" );
  assert_eq!( request.prompt, "Machine learning embeddings" );
  assert!( request.options.is_some() );
  
  let opts = request.options.unwrap();
  assert_eq!( opts.get( "temperature" ).unwrap(), &serde_json::Value::from( 0.5 ) );
  assert_eq!( opts.get( "dimension" ).unwrap(), &serde_json::Value::from( 2048 ) );
}

#[ test ]
fn test_embeddings_request_serialization()
{
  let request = EmbeddingsRequest
  {
    model : "test-model".to_string(),
    prompt : "Test serialization".to_string(),
    options : None,
  };
  
  // Test that request can be serialized
  let serialized = serde_json::to_string( &request ).expect( "Failed to serialize request" );
  assert!( serialized.contains( "test-model" ) );
  assert!( serialized.contains( "Test serialization" ) );
}

#[ test ]
fn test_embeddings_request_empty_prompt()
{
  let request = EmbeddingsRequest
  {
    model : "test-model".to_string(),
    prompt : String::new(),
    options : None,
  };
  
  // Empty prompt should be allowed at request level
  // Validation happens at API level
  assert_eq!( request.prompt, "" );
  assert!( request.prompt.is_empty() );
}

#[ test ]
fn test_embeddings_request_long_prompt()
{
  let long_prompt = "word ".repeat( 1000 );
  
  let request = EmbeddingsRequest
  {
    model : "test-model".to_string(),
    prompt : long_prompt.clone(),
    options : None,
  };
  
  assert_eq!( request.prompt, long_prompt );
  assert!( request.prompt.len() >= 5000 );
}

#[ test ]
fn test_embeddings_request_special_characters()
{
  let special_prompt = "Hello! 你好 🌍 Привет مرحبا こんにちは";
  
  let request = EmbeddingsRequest
  {
    model : "multilingual-model".to_string(),
    prompt : special_prompt.to_string(),
    options : None,
  };
  
  assert_eq!( request.prompt, special_prompt );
  assert!( request.prompt.contains( "🌍" ) );
  assert!( request.prompt.contains( "你好" ) );
}
