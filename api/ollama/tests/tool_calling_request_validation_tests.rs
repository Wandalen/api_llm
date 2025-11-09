//! Tool calling unit tests for `api_ollama`
//! 
//! These tests verify tool calling request construction and validation
//! without requiring server interaction.

#![ cfg( feature = "tool_calling" ) ]

use api_ollama::{
  ChatRequest, 
  ChatMessage,
  MessageRole,
  ToolDefinition,
  ToolCall
};
use serde_json::json;

#[ test ]
fn test_tool_definition_creation()
{
  let tool = ToolDefinition {
    name : "get_weather".to_string(),
    description : "Get weather for a location".to_string(),
    parameters : json!({
      "type": "object",
      "properties": {
        "location": {
          "type": "string",
          "description": "The city name"
        }
      },
      "required": ["location"]
    }),
  };
  
  assert_eq!(tool.name, "get_weather");
  assert_eq!(tool.description, "Get weather for a location");
  assert!(!tool.parameters.is_null());
}

#[ test ]
fn test_tool_call_creation()
{
  let tool_call = ToolCall {
    id : "call_123".to_string(),
    function : json!({
      "name": "calculate",
      "arguments": {"x": 5, "y": 10}
    }),
  };
  
  assert_eq!(tool_call.id, "call_123");
  assert_eq!(tool_call.function["name"], "calculate");
}

#[ test ]
fn test_chat_request_with_tools()
{
  let tools = vec![
    ToolDefinition {
      name : "search".to_string(),
      description : "Search the web".to_string(),
      parameters : json!({
        "type": "object",
        "properties": {
          "query": {"type": "string"}
        }
      }),
    }
  ];
  
  let messages = vec![
    ChatMessage {
      role : MessageRole::User,
      content : "Search for Rust tutorials".to_string(),
      images : None,
      tool_calls : None,
    }
  ];
  
  let request = ChatRequest {
    model : "tool-model".to_string(),
    messages,
    stream : Some(false),
    options : None,
    tools : Some(tools),
    tool_messages : None,
  };
  
  assert!(request.tools.is_some());
  assert_eq!(request.tools.unwrap().len(), 1);
}

#[ test ]
fn test_message_with_tool_calls()
{
  let tool_calls = vec![
    ToolCall {
      id : "call_456".to_string(),
      function : json!({
        "name": "add_numbers",
        "arguments": {"a": 3, "b": 7}
      }),
    }
  ];
  
  let message = ChatMessage {
    role : MessageRole::Assistant,
    content : "I'll calculate that for you".to_string(),
    images : None,
    tool_calls : Some(tool_calls),
  };
  
  assert!(message.tool_calls.is_some());
  assert_eq!(message.tool_calls.unwrap().len(), 1);
}

#[ test ]
fn test_tool_serialization()
{
  let tool = ToolDefinition {
    name : "test_func".to_string(),
    description : "Test function".to_string(),
    parameters : json!({"param": "value"}),
  };
  
  let serialized = serde_json::to_string(&tool).expect("Failed to serialize tool");
  let deserialized : ToolDefinition = serde_json::from_str(&serialized)
    .expect("Failed to deserialize tool");
  
  assert_eq!(deserialized.name, tool.name);
  assert_eq!(deserialized.description, tool.description);
}
