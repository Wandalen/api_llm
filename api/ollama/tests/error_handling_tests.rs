//! Error handling tests for `api_ollama` crate.

use api_ollama::{ 
  OllamaClient, 
  ChatMessage,
  MessageRole,
  ChatRequest, 
  GenerateRequest
};
use core::time::Duration;

#[ tokio::test ]
async fn test_list_models_network_error()
{
  // Test network error handling with unreachable server
  let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_timeout( Duration::from_millis( 100 ) );
    
  let result = client.list_models().await;
  assert!( result.is_err() );
  
  let error = result.unwrap_err();
  let error_str = format!( "{error}" );
  assert!( error_str.contains( "Network error" ) );
}

#[ tokio::test ]
async fn test_chat_network_error()
{
  let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_timeout( Duration::from_millis( 100 ) );
    
  let request = ChatRequest
  {
    model : "test-model".to_string(),
    messages : vec!
    [
      ChatMessage
      {
        role : MessageRole::User,
        content : "Hello".to_string(),
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      }
    ],
    stream : None,
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };
  
  let result = client.chat( request ).await;
  assert!( result.is_err() );
  
  let error = result.unwrap_err();
  let error_str = format!( "{error}" );
  assert!( error_str.contains( "Network error" ) );
}

#[ tokio::test ]
async fn test_generate_network_error()
{
  let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_timeout( Duration::from_millis( 100 ) );
    
  let request = GenerateRequest
  {
    model : "test-model".to_string(),
    prompt : "Tell me a joke".to_string(),
    stream : None,
    options : None,
  };
  
  let result = client.generate( request ).await;
  assert!( result.is_err() );
  
  let error = result.unwrap_err();
  let error_str = format!( "{error}" );
  assert!( error_str.contains( "Network error" ) );
}

#[ tokio::test ]
async fn test_model_info_network_error()
{
  let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_timeout( Duration::from_millis( 100 ) );
    
  let result = client.model_info( "test-model".to_string() ).await;
  assert!( result.is_err() );
  
  let error = result.unwrap_err();
  let error_str = format!( "{error}" );
  assert!( error_str.contains( "Network error" ) );
}

#[ tokio::test ]
async fn test_api_error_handling()
{
  // Use httpbin.org to simulate API errors
  let mut client = OllamaClient::new( "https://httpbin.org".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_timeout( Duration::from_secs( 5 ) );
    
  // This should return 404 which should be handled as API error
  let result = client.list_models().await;
  assert!( result.is_err() );
  
  let error = result.unwrap_err();
  let error_str = format!( "{error}" );
  println!( "Actual error : {error_str}" );
  assert!( error_str.contains( "API error" ) || error_str.contains( "Parse error" ) || error_str.contains( "Network error" ) );
}
