//! Basic unit tests for `api_ollama` crate functionality.

use api_ollama::{ 
  OllamaClient, 
  ChatMessage,
  MessageRole,
  ChatRequest, 
  GenerateRequest
};
use core::time::Duration;

#[ test ]
fn test_ollama_client_new()
{
  let client = OllamaClient::new( "http://test.local:11434".to_string(), OllamaClient::recommended_timeout_fast() );
  // We can't directly test private fields but we can test the client was created
  // by attempting to use it (though it will fail without a real server)
  let _ = client;
}

#[ test ]
fn test_ollama_client_default()
{
  let client = OllamaClient::default();
  // Test that default client is created successfully
  let _ = client;
}

#[ test ]
fn test_ollama_client_with_timeout()
{
  let client = OllamaClient::new( "http://test.local:11434".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_timeout( Duration::from_secs( 60 ) );
  let _ = client;
}

#[ test ]
fn test_message_creation()
{
  let message = ChatMessage
  {
    role : MessageRole::User,
    content : "Hello, world!".to_string(),
    images : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_calls : None,
  };
  
  assert_eq!( message.role, MessageRole::User );
  assert_eq!( message.content, "Hello, world!" );
}

#[ test ]
fn test_chat_request_creation()
{
  let messages = vec!
  [
    ChatMessage
    {
      role : MessageRole::User,
      content : "Hello".to_string(),
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    }
  ];
  
  let request = ChatRequest
  {
    model : "test-model".to_string(),
    messages,
    stream : Some( false ),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };
  
  assert_eq!( request.model, "test-model" );
  assert_eq!( request.messages.len(), 1 );
  assert_eq!( request.stream, Some( false ) );
}

#[ test ]
fn test_generate_request_creation()
{
  let request = GenerateRequest
  {
    model : "test-model".to_string(),
    prompt : "Tell me a joke".to_string(),
    stream : Some( false ),
    options : None,
  };
  
  assert_eq!( request.model, "test-model" );
  assert_eq!( request.prompt, "Tell me a joke" );
  assert_eq!( request.stream, Some( false ) );
}

#[ test ]
fn test_serialization_deserialization()
{
  let message = ChatMessage
  {
    role : MessageRole::Assistant,
    content : "Hello there!".to_string(),
    images : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_calls : None,
  };
  
  // Test that ChatMessage can be serialized and deserialized
  let serialized = serde_json::to_string( &message ).expect( "Failed to serialize message" );
  let deserialized : ChatMessage = serde_json::from_str( &serialized ).expect( "Failed to deserialize message" );
  
  assert_eq!( deserialized.role, message.role );
  assert_eq!( deserialized.content, message.content );
}

#[ tokio::test ]
async fn test_client_is_available_unreachable_server()
{
  // Test with unreachable server - should return false
  let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_timeout( Duration::from_millis( 100 ) );
    
  let result = client.is_available().await;
  assert!( !result );
}
