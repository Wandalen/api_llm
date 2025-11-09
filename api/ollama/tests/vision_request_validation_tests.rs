//! Vision support unit tests for `api_ollama`
//! 
//! These tests verify vision/multimodal request construction and validation
//! without requiring server interaction.

#![ cfg( feature = "vision_support" ) ]

use api_ollama::{
  ChatRequest, 
  ChatMessage,
  MessageRole
};

#[ test ]
fn test_message_with_image_data()
{
  let image_data = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChAGA";
  
  let message = ChatMessage {
    role : MessageRole::User,
    content : "What's in this image?".to_string(),
    images : Some(vec![image_data.to_string()]),
    #[ cfg( feature = "tool_calling" ) ]
    tool_calls : None,
  };
  
  assert!(message.images.is_some());
  assert_eq!(message.images.as_ref().unwrap().len(), 1);
  assert_eq!(message.images.as_ref().unwrap()[0], image_data);
}

#[ test ]
fn test_message_with_multiple_images()
{
  let images = vec![
    "image1_base64_data".to_string(),
    "image2_base64_data".to_string(),
    "image3_base64_data".to_string(),
  ];
  
  let message = ChatMessage {
    role : MessageRole::User,
    content : "Compare these images".to_string(),
    images : Some(images.clone()),
    #[ cfg( feature = "tool_calling" ) ]
    tool_calls : None,
  };
  
  assert!(message.images.is_some());
  assert_eq!(message.images.as_ref().unwrap().len(), 3);
  assert_eq!(message.images.as_ref().unwrap(), &images);
}

#[ test ]
fn test_vision_chat_request()
{
  let messages = vec![
    ChatMessage {
      role : MessageRole::User,
      content : "Describe this image".to_string(),
      images : Some(vec!["base64_image_data".to_string()]),
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    }
  ];
  
  let request = ChatRequest {
    model : "vision-model".to_string(),
    messages,
    stream : Some(false),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };
  
  assert_eq!(request.model, "vision-model");
  assert!(request.messages[0].images.is_some());
}

#[ test ]
fn test_vision_message_serialization()
{
  let message = ChatMessage {
    role : MessageRole::User,
    content : "Analyze image".to_string(),
    images : Some(vec!["test_image_data".to_string()]),
    #[ cfg( feature = "tool_calling" ) ]
    tool_calls : None,
  };
  
  let serialized = serde_json::to_string(&message).expect("Failed to serialize");
  let deserialized : ChatMessage = serde_json::from_str(&serialized)
    .expect("Failed to deserialize");
  
  assert_eq!(deserialized.images, message.images);
  assert_eq!(deserialized.content, message.content);
}

#[ test ]
fn test_empty_images_array()
{
  let message = ChatMessage {
    role : MessageRole::User,
    content : "Text only message".to_string(),
    images : Some(vec![]), // Empty but present
    #[ cfg( feature = "tool_calling" ) ]
    tool_calls : None,
  };
  
  assert!(message.images.is_some());
  assert!(message.images.as_ref().unwrap().is_empty());
}

#[ test ]
fn test_no_images()
{
  let message = ChatMessage {
    role : MessageRole::User,
    content : "Pure text message".to_string(),
    images : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_calls : None,
  };
  
  assert!(message.images.is_none());
}
