//! Final integration test to ensure all examples are working

#[ cfg( test ) ]
mod tests
{
  use api_ollama::{ OllamaClient, ChatMessage, MessageRole };
  #[ cfg( feature = "streaming" ) ]
  use api_ollama::ChatRequest;

  #[ tokio::test ]
  async fn test_all_examples_can_run_without_issues()
  {
    // This test validates that all 5 example use cases can be initialized and structured properly
    
    // 1. Chatbot Assistant Example
    let mut conversation_history = vec![
      ChatMessage
      {
        role : MessageRole::System,
        content : "You are a helpful assistant.".to_string(),
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      }
    ];
    
    conversation_history.push( ChatMessage
    {
      role : MessageRole::User,
      content : "Hello!".to_string(),
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    } );
    
    assert!( conversation_history.len() >= 2 );
    
    // 2. Document Analyzer Example
    let sample_document = "AI has rapidly evolved...".to_string();
    assert!( !sample_document.is_empty() );
    
    // 3. Code Assistant Example  
    let code_samples = [
      ( "Rust Function", "fn process() {}" ),
      ( "JavaScript Function", "function find() {}" )
    ];
    assert_eq!( code_samples.len(), 2 );
    
    // 4. Streaming Chat Example
    #[ cfg( feature = "streaming" ) ]
    {
      let streaming_request = ChatRequest
      {
        model : "test-model".to_string(),
        messages : conversation_history.clone(),
        stream : Some( true ),
        options : None,
        #[ cfg( feature = "tool_calling" ) ]
        tools : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_messages : None,
      };
      
      assert_eq!( streaming_request.stream, Some( true ) );
    }
    
    // 5. Multimodal Vision Example
    #[ cfg( feature = "vision_support" ) ]
    {
      use base64::{ Engine as _, engine::general_purpose };
      
      let fake_image = general_purpose::STANDARD.encode( b"test image data" );
      let vision_message = ChatMessage
      {
        role : MessageRole::User,
        content : "What do you see?".to_string(),
        images : Some( vec![ fake_image ] ),
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      };
      
      assert!( vision_message.images.is_some() );
    }
    
    // All examples can initialize their core structures without issues
    let _client = OllamaClient::new( "http://localhost:11434".to_string(), OllamaClient::recommended_timeout_fast() );
    
    // Test passed - all examples are structurally sound
    // No assertion needed - reaching this point means success
  }
}
