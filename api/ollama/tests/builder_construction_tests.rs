//! Builder pattern unit tests for `api_ollama`
//! 
//! These tests verify fluent builder interfaces for constructing requests
//! without requiring server interaction, focusing purely on builder functionality.

#![ cfg( feature = "builder_patterns" ) ]

use api_ollama::{ 
  ChatRequestBuilder, 
  GenerateRequestBuilder, 
  EmbeddingsRequestBuilder,
  MessageRole
};
use std::collections::HashMap;

#[ test ]
fn test_builder_validation_errors()
{
  // Test missing required fields
  let result = ChatRequestBuilder::new()
    .user_message("Hello")
    .build(); // Missing model
  
  assert!(result.is_err(), "Builder should fail without model");
  
  let result = ChatRequestBuilder::new()
    .model("test-model")
    .build(); // Missing messages
  
  assert!(result.is_err(), "Builder should fail without messages");
  
  // Test empty model
  let result = ChatRequestBuilder::new()
    .model("")
    .user_message("Hello")
    .build();
  
  assert!(result.is_err(), "Builder should fail with empty model");
  
  // Test empty message content
  let result = ChatRequestBuilder::new()
    .model("test-model")
    .user_message("")
    .build();
  
  assert!(result.is_err(), "Builder should fail with empty message");
}

#[ test ]
fn test_builder_default_values()
{
  let request = ChatRequestBuilder::new()
    .model("test-model")
    .user_message("Hello")
    .build()
    .expect("Basic builder should work");
  
  // Check default values
  assert_eq!(request.stream, Some(false), "Stream should default to false for non-streaming");
  assert!(request.options.is_none(), "Options should default to None");
}

#[ test ]
fn test_builder_immutability()
{
  let builder1 = ChatRequestBuilder::new()
    .model("model1")
    .user_message("Hello");
  
  let builder2 = builder1.clone()
    .model("model2");
  
  let request1 = builder1.build().expect("Builder1 should work");
  let request2 = builder2.build().expect("Builder2 should work");
  
  assert_eq!(request1.model, "model1");
  assert_eq!(request2.model, "model2");
}

#[ test ]
fn test_chat_request_builder_message_structure()
{
  let request = ChatRequestBuilder::new()
    .model("test-model")
    .system_message("You are a helpful assistant.")
    .user_message("What is 2+2?")
    .assistant_message("2+2 equals 4.")
    .user_message("What about 3+3?")
    .build()
    .expect("Failed to build conversation request");
  
  assert_eq!(request.messages.len(), 4, "Should have 4 messages");
  assert_eq!(request.messages[0].role, MessageRole::System);
  assert_eq!(request.messages[1].role, MessageRole::User);
  assert_eq!(request.messages[2].role, MessageRole::Assistant);
  assert_eq!(request.messages[3].role, MessageRole::User);
}

#[ test ]
fn test_chat_request_builder_with_options()
{
  let mut options = HashMap::new();
  options.insert("temperature".to_string(), serde_json::Value::from(0.7));
  options.insert("top_p".to_string(), serde_json::Value::from(0.9));
  
  let request = ChatRequestBuilder::new()
    .model("test-model")
    .user_message("Tell me a short joke")
    .temperature(0.8)
    .top_p(0.9)
    .max_tokens(50)
    .options(options)
    .build()
    .expect("Failed to build chat request with options");
  
  assert_eq!(request.model, "test-model");
  assert!(request.options.is_some());
}

#[ test ]
fn test_generate_request_builder_basic()
{
  let request = GenerateRequestBuilder::new()
    .model("test-model")
    .prompt("Write a haiku about coding")
    .build()
    .expect("Failed to build generate request");
  
  assert_eq!(request.model, "test-model");
  assert_eq!(request.prompt, "Write a haiku about coding");
}

#[ test ]
fn test_generate_request_builder_with_options()
{
  let request = GenerateRequestBuilder::new()
    .model("test-model")
    .prompt("Say hello in one word")
    .temperature(0.1)
    .max_tokens(10)
    .stop_sequences(&[".", "!"])
    .build()
    .expect("Failed to build generate request with options");
  
  assert_eq!(request.model, "test-model");
  assert_eq!(request.prompt, "Say hello in one word");
  assert!(request.options.is_some());
}

#[ cfg( feature = "embeddings" ) ]
#[ test ]
fn test_embeddings_request_builder_basic()
{
  let request = EmbeddingsRequestBuilder::new()
    .model("test-model")
    .prompt("Hello world")
    .build()
    .expect("Failed to build embeddings request");
  
  assert_eq!(request.model, "test-model");
  assert_eq!(request.prompt, "Hello world");
}

#[ cfg( feature = "embeddings" ) ]
#[ test ]
fn test_embeddings_request_builder_with_options()
{
  let request = EmbeddingsRequestBuilder::new()
    .model("test-model")
    .prompt("Machine learning is fascinating")
    .temperature(0.2)
    .dimension(2048)
    .build()
    .expect("Failed to build embeddings request with options");
  
  assert_eq!(request.model, "test-model");
  assert_eq!(request.prompt, "Machine learning is fascinating");
  assert!(request.options.is_some());
}

#[ test ]
fn test_builder_method_chaining()
{
  // Test fluent method chaining
  let request = ChatRequestBuilder::new()
    .model("test-model")
    .system_message("You are a concise assistant")
    .user_message("What is Rust?")
    .temperature(0.5)
    .max_tokens(100)
    .build()
    .expect("Method chaining should work");
  
  assert_eq!(request.model, "test-model");
  assert_eq!(request.messages.len(), 2);
  assert!(request.options.is_some());
}

#[ test ]
fn test_builder_complex_conversation()
{
  let request = ChatRequestBuilder::new()
    .model("test-model")
    .system_message("You are a helpful math tutor. Answer questions step by step.")
    .user_message("What is 15 + 27?")
    .assistant_message("Let me solve this step by step:\n15 + 27 = 42")
    .user_message("Good! Now what is 42 divided by 6?")
    .temperature(0.3)
    .max_tokens(150)
    .build()
    .expect("Complex conversation builder should work");
  
  assert_eq!(request.messages.len(), 4);
  assert_eq!(request.messages[0].role, MessageRole::System);
  assert_eq!(request.messages[1].role, MessageRole::User);
  assert_eq!(request.messages[2].role, MessageRole::Assistant);
  assert_eq!(request.messages[3].role, MessageRole::User);
}

#[ cfg( feature = "streaming" ) ]
#[ test ]
fn test_chat_request_builder_streaming_flag()
{
  let request = ChatRequestBuilder::new()
    .model("test-model")
    .user_message("Count from 1 to 3")
    .streaming(true)
    .build()
    .expect("Failed to build streaming chat request");
  
  assert_eq!(request.stream, Some(true), "Should enable streaming");
}
