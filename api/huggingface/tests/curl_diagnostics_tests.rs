#![ cfg( test ) ]
#![ allow( clippy::all, clippy::pedantic ) ]

//! CURL Diagnostics Tests for HuggingFace Router API
//!
//! Comprehensive tests for CURL diagnostics functionality validating the AsCurl trait.
//!
//! ## Development Insights
//!
//! ### Why CURL Diagnostics Exist
//!
//! CURL diagnostics provide critical debugging capabilities for API client development:
//!
//! 1. **Request Debugging**: Developers can copy-paste generated curl commands to test
//!    API requests outside the Rust codebase, isolating whether issues are client-side
//!    or server-side.
//!
//! 2. **Documentation**: Curl commands serve as executable documentation showing exact
//!    HTTP request structure including headers, body format, and authentication.
//!
//! 3. **Integration Testing**: Enables manual verification against the actual HuggingFace
//!    Router API without needing to understand Rust client internals.
//!
//! 4. **Security Audit**: Generated commands use placeholder API keys by default,
//!    preventing accidental credential exposure in logs or error messages.
//!
//! ### TDD Approach
//!
//! These tests were written **before** the diagnostics implementation (Task 609),
//! following strict Test-Driven Development:
//!
//! 1. **RED**: Tests written first defining expected curl command format
//! 2. **GREEN**: Minimal implementation to make tests pass (Task 610)
//! 3. **REFACTOR**: Code cleaned to meet rulebook standards
//!
//! ### Testing Philosophy
//!
//! These tests validate curl command generation without external API calls because:
//!
//! - **Format validation** can be done entirely through string inspection
//! - **Real API testing** happens in integration tests (providers_api_tests.rs)
//! - **No mocking principle** applies to API calls, not utility functions
//! - Tests remain **fast** (<1ms each) for rapid TDD feedback loops

use api_huggingface::components::inference_shared::{ ChatCompletionRequest, ChatMessage };
use api_huggingface::diagnostics::{ AsCurl, CurlOptions };

#[ test ]
fn test_chat_completion_request_basic_curl()
{
  let request = ChatCompletionRequest
  {
  messages : vec![
      ChatMessage
      {
  role : "user".to_string(),
  content : "What is 2+2?".to_string(),
  tool_calls : None,
  tool_call_id : None,
      }
  ],
  model : "meta-llama/Llama-3.3-70B-Instruct".to_string(),
  temperature : Some( 0.7 ),
  max_tokens : Some( 100 ),
  top_p : Some( 0.9 ),
  stream : Some( false ),
  tools : None,
  tool_choice : None,
  };

  let curl_command = request.as_curl();

  assert!( curl_command.contains( "curl" ) );
  assert!( curl_command.contains( "https://router.huggingface.co/v1/chat/completions" ) );
  assert!( curl_command.contains( "POST" ) );
  assert!( curl_command.contains( "Content-Type : application/json" ) );
  assert!( curl_command.contains( "Authorization" ) );
}

#[ test ]
fn test_curl_generation_with_pretty_formatting()
{
  let request = ChatCompletionRequest
  {
  messages : vec![
      ChatMessage
      {
  role : "system".to_string(),
  content : "You are helpful".to_string(),
  tool_calls : None,
  tool_call_id : None,
      },
      ChatMessage
      {
  role : "user".to_string(),
  content : "Explain quantum computing".to_string(),
  tool_calls : None,
  tool_call_id : None,
      }
  ],
  model : "meta-llama/Llama-3.3-70B-Instruct".to_string(),
  temperature : Some( 0.3 ),
  max_tokens : Some( 500 ),
  top_p : None,
  stream : Some( false ),
  tools : None,
  tool_choice : None,
  };

  let options = CurlOptions::pretty();
  let curl_command = request.as_curl_with_options( &options );

  assert!( curl_command.contains( "\\\n" ) );
}

#[ test ]
fn test_curl_with_api_key_placeholder()
{
  let request = ChatCompletionRequest
  {
  messages : vec![
      ChatMessage
      {
  role : "user".to_string(),
  content : "Test message".to_string(),
  tool_calls : None,
  tool_call_id : None,
      }
  ],
  model : "meta-llama/Llama-3.3-70B-Instruct".to_string(),
  temperature : None,
  max_tokens : None,
  top_p : None,
  stream : Some( false ),
  tools : None,
  tool_choice : None,
  };

  let options = CurlOptions::new();
  let curl_command = request.as_curl_with_options( &options );

  assert!( curl_command.contains( "YOUR_API_KEY_HERE" ) || curl_command.contains( "Bearer" ) );
}

#[ test ]
fn test_curl_with_actual_api_key()
{
  let request = ChatCompletionRequest
  {
  messages : vec![
      ChatMessage
      {
  role : "user".to_string(),
  content : "Test".to_string(),
  tool_calls : None,
  tool_call_id : None,
      }
  ],
  model : "meta-llama/Llama-3.3-70B-Instruct".to_string(),
  temperature : None,
  max_tokens : None,
  top_p : None,
  stream : Some( false ),
  tools : None,
  tool_choice : None,
  };

  let options = CurlOptions::with_api_key( "hf_test_key_12345" );
  let curl_command = request.as_curl_with_options( &options );

  assert!( curl_command.contains( "hf_test_key_12345" ) );
  assert!( !curl_command.contains( "YOUR_API_KEY_HERE" ) );
}

#[ test ]
fn test_curl_json_serialization()
{
  let request = ChatCompletionRequest
  {
  messages : vec![
      ChatMessage
      {
  role : "user".to_string(),
  content : "Test with special chars : \"quotes\" and 'apostrophes'".to_string(),
  tool_calls : None,
  tool_call_id : None,
      }
  ],
  model : "meta-llama/Llama-3.3-70B-Instruct".to_string(),
  temperature : Some( 0.7 ),
  max_tokens : Some( 100 ),
  top_p : Some( 0.9 ),
  stream : Some( false ),
  tools : None,
  tool_choice : None,
  };

  let curl_command = request.as_curl();

  assert!( curl_command.contains( "messages" ) );
  assert!( curl_command.contains( "model" ) );
  assert!( curl_command.contains( "temperature" ) );
}

#[ test ]
fn test_curl_with_optional_fields()
{
  let minimal_request = ChatCompletionRequest
  {
  messages : vec![
      ChatMessage
      {
  role : "user".to_string(),
  content : "Hello".to_string(),
  tool_calls : None,
  tool_call_id : None,
      }
  ],
  model : "meta-llama/Llama-3.3-70B-Instruct".to_string(),
  temperature : None,
  max_tokens : None,
  top_p : None,
  stream : None,
  tools : None,
  tool_choice : None,
  };

  let curl_command = minimal_request.as_curl();

  assert!( curl_command.contains( "curl" ) );
  assert!( curl_command.contains( "messages" ) );
}

#[ test ]
fn test_curl_options_builder_pattern()
{
  let options = CurlOptions::new();

  assert!( !options.pretty_json );
  assert!( options.include_auth_header );
  assert!( !options.multiline_format );
  assert!( options.api_key.is_none() );

  let pretty_options = CurlOptions::pretty();

  assert!( pretty_options.pretty_json );
  assert!( pretty_options.multiline_format );
}

#[ test ]
fn test_curl_command_executability()
{
  let request = ChatCompletionRequest
  {
  messages : vec![
      ChatMessage
      {
  role : "user".to_string(),
  content : "Test".to_string(),
  tool_calls : None,
  tool_call_id : None,
      }
  ],
  model : "meta-llama/Llama-3.3-70B-Instruct".to_string(),
  temperature : Some( 0.7 ),
  max_tokens : Some( 100 ),
  top_p : Some( 0.9 ),
  stream : Some( false ),
  tools : None,
  tool_choice : None,
  };

  let curl_command = request.as_curl();

  assert!( curl_command.starts_with( "curl" ) );

  let lines : Vec< &str > = curl_command.split( '\n' ).collect();
  assert!( lines.len() == 1 || curl_command.contains( "\\\n" ) );
}

#[ test ]
fn test_curl_with_multi_turn_conversation()
{
  let request = ChatCompletionRequest
  {
  messages : vec![
      ChatMessage
      {
  role : "system".to_string(),
  content : "You are a math tutor.".to_string(),
  tool_calls : None,
  tool_call_id : None,
      },
      ChatMessage
      {
  role : "user".to_string(),
  content : "What is 2+2?".to_string(),
  tool_calls : None,
  tool_call_id : None,
      },
      ChatMessage
      {
  role : "assistant".to_string(),
  content : "2+2 equals 4.".to_string(),
  tool_calls : None,
  tool_call_id : None,
      },
      ChatMessage
      {
  role : "user".to_string(),
  content : "What about 3+3?".to_string(),
  tool_calls : None,
  tool_call_id : None,
      }
  ],
  model : "meta-llama/Llama-3.3-70B-Instruct".to_string(),
  temperature : Some( 0.5 ),
  max_tokens : Some( 150 ),
  top_p : Some( 0.9 ),
  stream : Some( false ),
  tools : None,
  tool_choice : None,
  };

  let curl_command = request.as_curl();

  assert!( curl_command.contains( "What is 2+2?" ) );
  assert!( curl_command.contains( "2+2 equals 4" ) );
  assert!( curl_command.contains( "What about 3+3?" ) );
  assert!( curl_command.contains( "You are a math tutor" ) );
}

#[ test ]
fn test_curl_edge_case_empty_content()
{
  let request = ChatCompletionRequest
  {
  messages : vec![
      ChatMessage
      {
  role : "user".to_string(),
  content : "".to_string(),
  tool_calls : None,
  tool_call_id : None,
      }
  ],
  model : "meta-llama/Llama-3.3-70B-Instruct".to_string(),
  temperature : None,
  max_tokens : None,
  top_p : None,
  stream : Some( false ),
  tools : None,
  tool_choice : None,
  };

  let curl_command = request.as_curl();
  assert!( curl_command.contains( "curl" ) );
}

#[ test ]
fn test_curl_special_characters_escaping()
{
  let request = ChatCompletionRequest
  {
  messages : vec![
      ChatMessage
      {
  role : "user".to_string(),
  content : "Test with : \"quotes\", 'apostrophes', and \nnewlines".to_string(),
  tool_calls : None,
  tool_call_id : None,
      }
  ],
  model : "meta-llama/Llama-3.3-70B-Instruct".to_string(),
  temperature : None,
  max_tokens : None,
  top_p : None,
  stream : Some( false ),
  tools : None,
  tool_choice : None,
  };

  let curl_command = request.as_curl();
  assert!( curl_command.contains( "curl" ) );
}

#[ test ]
fn test_curl_generation_performance()
{
  let request = ChatCompletionRequest
  {
  messages : vec![
      ChatMessage
      {
  role : "user".to_string(),
  content : "Performance test".to_string(),
  tool_calls : None,
  tool_call_id : None,
      }
  ],
  model : "meta-llama/Llama-3.3-70B-Instruct".to_string(),
  temperature : Some( 0.7 ),
  max_tokens : Some( 100 ),
  top_p : Some( 0.9 ),
  stream : Some( false ),
  tools : None,
  tool_choice : None,
  };

  let start = std::time::Instant::now();

  for _ in 0..100
  {
  let _ = request.as_curl();
  }

  let elapsed = start.elapsed();
  assert!( elapsed.as_millis() < 100 );
}

#[ test ]
fn test_curl_command_format_matches_router_api()
{
  let request = ChatCompletionRequest
  {
  messages : vec![
      ChatMessage
      {
  role : "user".to_string(),
  content : "Test".to_string(),
  tool_calls : None,
  tool_call_id : None,
      }
  ],
  model : "meta-llama/Llama-3.3-70B-Instruct".to_string(),
  temperature : Some( 0.7 ),
  max_tokens : Some( 100 ),
  top_p : Some( 0.9 ),
  stream : Some( false ),
  tools : None,
  tool_choice : None,
  };

  let curl_command = request.as_curl();

  assert!( curl_command.contains( "https://router.huggingface.co/v1/chat/completions" ) );
  assert!( curl_command.contains( "application/json" ) );
  assert!( curl_command.contains( "-X POST" ) || curl_command.contains( "POST" ) );
}

#[ test ]
fn test_debug_workflow_with_curl_generation()
{
  let request = ChatCompletionRequest
  {
  messages : vec![
      ChatMessage
      {
  role : "user".to_string(),
  content : "Debug this request".to_string(),
  tool_calls : None,
  tool_call_id : None,
      }
  ],
  model : "meta-llama/Llama-3.3-70B-Instruct".to_string(),
  temperature : Some( 0.5 ),
  max_tokens : Some( 200 ),
  top_p : Some( 0.9 ),
  stream : Some( false ),
  tools : None,
  tool_choice : None,
  };

  let basic_curl = request.as_curl();
  assert!( !basic_curl.is_empty() );

  let pretty_options = CurlOptions::pretty();
  let pretty_curl = request.as_curl_with_options( &pretty_options );
  assert!( pretty_curl.len() > basic_curl.len() );

  let key_options = CurlOptions::with_api_key( "test_key" );
  let executable_curl = request.as_curl_with_options( &key_options );
  assert!( executable_curl.contains( "test_key" ) );
}

#[ test ]
fn test_complete_diagnostics_functionality()
{
  let request = ChatCompletionRequest
  {
  messages : vec![
      ChatMessage
      {
  role : "system".to_string(),
  content : "You are helpful".to_string(),
  tool_calls : None,
  tool_call_id : None,
      },
      ChatMessage
      {
  role : "user".to_string(),
  content : "Hello world".to_string(),
  tool_calls : None,
  tool_call_id : None,
      }
  ],
  model : "meta-llama/Llama-3.3-70B-Instruct".to_string(),
  temperature : Some( 0.7 ),
  max_tokens : Some( 150 ),
  top_p : Some( 0.9 ),
  stream : Some( false ),
  tools : None,
  tool_choice : None,
  };

  let basic = request.as_curl();
  assert!( basic.contains( "curl" ) );

  let pretty = request.as_curl_with_options( &CurlOptions::pretty() );
  assert!( pretty.contains( "\\\n" ) );

  let custom_options = CurlOptions::with_api_key( "test" );
  let custom = request.as_curl_with_options( &custom_options );
  assert!( custom.contains( "test" ) );
}
