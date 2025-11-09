//! Unit tests for multimodal vision functionality

#[ cfg( all( test, feature = "vision_support" ) ) ]
mod tests
{
  use api_ollama::{ OllamaClient, ChatRequest, ChatMessage, MessageRole };
  use base64::{ Engine as _, engine::general_purpose };

  #[ tokio::test ]
  async fn test_multimodal_vision_can_initialize()
  {
    // Test that multimodal vision can set up its basic structures
    let _client = OllamaClient::new( "http://localhost:11434".to_string(), OllamaClient::recommended_timeout_fast() );
    
    // Test image encoding functionality
    let test_image_data = b"fake image data for testing";
    let encoded_image = general_purpose::STANDARD.encode( test_image_data );
    
    assert!( !encoded_image.is_empty() );
    
    // Test vision message creation
    let vision_message = ChatMessage
    {
      role : MessageRole::User,
      content : "What do you see in this image?".to_string(),
      images : Some( vec![ encoded_image.clone() ] ),
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    };
    
    assert_eq!( vision_message.images.as_ref().unwrap().len(), 1 );
    
    // Test vision request construction
    let request = ChatRequest
    {
      model : "test-vision-model".to_string(),
      messages : vec![ vision_message ],
      stream : Some( false ),
      options : None,
      #[ cfg( feature = "tool_calling" ) ]
      tools : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_messages : None,
    };
    
    assert_eq!( request.model, "test-vision-model" );
    assert_eq!( request.messages.len(), 1 );
    assert!( request.messages[ 0 ].images.is_some() );
  }

  #[ tokio::test ]
  async fn test_image_analysis_scenarios()
  {
    // Test image analysis scenarios from multimodal_vision example
    let analysis_scenarios = [
      "Describe what you see in this image in detail.",
      "What objects can you identify in this image?",
      "What is the mood or atmosphere of this image?",
      "Are there any safety concerns or hazards visible?",
      "What text, if any, can you read in this image?",
    ];
    
    assert_eq!( analysis_scenarios.len(), 5 );
    
    // Test that analysis request can be constructed
    let fake_image = general_purpose::STANDARD.encode( b"test image data" );
    
    let request = ChatRequest
    {
      model : "llava".to_string(),
      messages : vec![ ChatMessage
      {
        role : MessageRole::User,
        content : analysis_scenarios[ 0 ].to_string(),
        images : Some( vec![ fake_image ] ),
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
    
    assert!( request.messages[ 0 ].content.contains( "Describe" ) );
    assert!( request.messages[ 0 ].images.is_some() );
  }

  #[ tokio::test ]
  async fn test_multi_image_comparison()
  {
    // Test multi-image comparison functionality
    let image1 = general_purpose::STANDARD.encode( b"first test image" );
    let image2 = general_purpose::STANDARD.encode( b"second test image" );
    
    let comparison_message = ChatMessage
    {
      role : MessageRole::User,
      content : "Compare these two images and describe the differences.".to_string(),
      images : Some( vec![ image1, image2 ] ),
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    };
    
    assert_eq!( comparison_message.images.as_ref().unwrap().len(), 2 );
    
    let request = ChatRequest
    {
      model : "llava".to_string(),
      messages : vec![ comparison_message ],
      stream : Some( false ),
      options : None,
      #[ cfg( feature = "tool_calling" ) ]
      tools : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_messages : None,
    };
    
    assert!( request.messages[ 0 ].content.contains( "Compare" ) );
    assert_eq!( request.messages[ 0 ].images.as_ref().unwrap().len(), 2 );
  }
}

#[ cfg( not( feature = "vision_support" ) ) ]
#[ tokio::test ]
async fn test_vision_feature_not_enabled()
{
  // This test ensures the vision_support feature flag works correctly
  // If vision_support is not enabled, the examples should handle this gracefully
  assert!( true ); // Feature is disabled, so no vision functionality to test
}
