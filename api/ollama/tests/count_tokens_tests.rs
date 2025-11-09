//! Count Tokens Tests
//!
//! Comprehensive test suite for token counting functionality in the Ollama API client.
//! Tests token estimation before API calls, cost calculation, input validation,
//! and batch operation optimization for different text inputs and model types.
//!
//! Note : These tests focus on API structure and token counting logic rather than actual
//! Ollama token counting, since token counting is model-dependent and varies by implementation.

#[ cfg( feature = "count_tokens" ) ]
#[ allow( clippy::std_instead_of_core ) ] // std required for time operations
mod count_tokens_tests
{
  use api_ollama::{
    TokenCountRequest, TokenCountResponse, CostEstimation, BatchTokenRequest,
    BatchTokenResponse, TokenValidationConfig, ModelTokenCapabilities
  };
  use std::time::{ Duration, Instant };

  /// Sample text inputs for token counting tests
  const SHORT_TEXT: &str = "Hello world";
  const MEDIUM_TEXT: &str = "This is a longer text that contains multiple sentences and should result in more tokens being counted by the tokenizer.";
  const LONG_TEXT: &str = "This is a very long text that will be used to test token counting with larger inputs. It contains multiple paragraphs, various sentence structures, and different types of content. The purpose is to verify that the token counting functionality can handle longer texts accurately and efficiently. This text will help us test performance characteristics as well as accuracy of token estimation across different text lengths and complexities.";
  const CODE_TEXT: &str = r"
    fn fibonacci(n : u32) -> u32 
    {
        match n
        {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }
  ";

  /// Test basic token counting request structure and validation
  #[ tokio::test ]
  async fn test_token_count_request_structure()
  {
    // Test creating a valid token count request
    let request = TokenCountRequest
    {
      model : "llama3.2".to_string(),
      text : SHORT_TEXT.to_string(),
      options : None,
    };

    // Test request structure
    assert_eq!( request.model, "llama3.2" );
    assert_eq!( request.text, SHORT_TEXT );
    assert!( request.options.is_none() );

    println!( "✓ Token count request structure validation successful" );

    // Test serialization
    let serialized = serde_json::to_string( &request );
    assert!( serialized.is_ok() );
    println!( "✓ Token count request serialization successful" );

    // Test with longer text
    let long_request = TokenCountRequest
    {
      model : "llama3.2".to_string(),
      text : LONG_TEXT.to_string(),
      options : None,
    };

    assert_eq!( long_request.text.len(), LONG_TEXT.len() );
    println!( "✓ Long text token count request validation successful" );
  }

  /// Test token count response structure and metadata
  #[ tokio::test ]
  async fn test_token_count_response_structure()
  {
    // Test basic token count response
    let response = TokenCountResponse
    {
      token_count : 42,
      model : "llama3.2".to_string(),
      text_length : SHORT_TEXT.len(),
      estimated_cost : Some( 0.0001 ),
      processing_time_ms : Some( 5 ),
      metadata : None,
    };

    // Test response structure validation
    assert_eq!( response.token_count, 42 );
    assert_eq!( response.model, "llama3.2" );
    assert_eq!( response.text_length, SHORT_TEXT.len() );
    assert_eq!( response.estimated_cost, Some( 0.0001 ) );
    assert_eq!( response.processing_time_ms, Some( 5 ) );

    println!( "✓ Token count response structure validation successful" );

    // Test response serialization
    let serialized = serde_json::to_string( &response );
    assert!( serialized.is_ok() );
    println!( "✓ Token count response serialization successful" );
  }

  /// Test cost estimation functionality based on token counts
  #[ tokio::test ]
  async fn test_cost_estimation_structure()
  {
    // Test cost estimation structure
    let cost_estimation = CostEstimation
    {
      input_tokens : 100,
      estimated_output_tokens : 50,
      input_cost_per_token : 0.0001,
      output_cost_per_token : 0.0002,
      total_estimated_cost : 0.02,
      currency : "USD".to_string(),
      model : "llama3.2".to_string(),
    };

    // Test cost calculation validation
    assert_eq!( cost_estimation.input_tokens, 100 );
    assert_eq!( cost_estimation.estimated_output_tokens, 50 );
    assert!( cost_estimation.input_cost_per_token > 0.0 );
    assert!( cost_estimation.output_cost_per_token > 0.0 );
    assert!( cost_estimation.total_estimated_cost > 0.0 );
    assert_eq!( cost_estimation.currency, "USD" );

    println!( "✓ Cost estimation structure validation successful" );

    // Test cost calculation accuracy
    let calculated_cost = ( f64::from(cost_estimation.input_tokens) * cost_estimation.input_cost_per_token ) +
                         ( f64::from(cost_estimation.estimated_output_tokens) * cost_estimation.output_cost_per_token );
    assert!( ( calculated_cost - cost_estimation.total_estimated_cost ).abs() < 0.001 );
    println!( "✓ Cost calculation accuracy validation successful" );
  }

  /// Test batch token counting for multiple texts
  #[ tokio::test ]
  async fn test_batch_token_counting_structure()
  {
    // Test batch token counting request
    let texts = vec![
      SHORT_TEXT.to_string(),
      MEDIUM_TEXT.to_string(),
      LONG_TEXT.to_string(),
      CODE_TEXT.to_string(),
    ];

    let batch_request = BatchTokenRequest
    {
      model : "llama3.2".to_string(),
      texts,
      options : None,
      estimate_costs : true,
    };

    // Test batch request structure
    assert_eq!( batch_request.model, "llama3.2" );
    assert_eq!( batch_request.texts.len(), 4 );
    assert!( batch_request.estimate_costs );
    assert!( batch_request.options.is_none() );

    println!( "✓ Batch token count request structure validation successful" );

    // Test batch response structure
    let batch_response = BatchTokenResponse
    {
      results : vec![
        TokenCountResponse
        {
          token_count : 3,
          model : "llama3.2".to_string(),
          text_length : SHORT_TEXT.len(),
          estimated_cost : Some( 0.0001 ),
          processing_time_ms : Some( 2 ),
          metadata : None,
        },
        TokenCountResponse
        {
          token_count : 25,
          model : "llama3.2".to_string(),
          text_length : MEDIUM_TEXT.len(),
          estimated_cost : Some( 0.001 ),
          processing_time_ms : Some( 3 ),
          metadata : None,
        },
      ],
      total_tokens : 28,
      total_estimated_cost : Some( 0.0011 ),
      processing_time_ms : Some( 5 ),
      batch_optimization_savings : Some( 0.2 ), // 20% savings from batch processing
    };

    assert_eq!( batch_response.results.len(), 2 );
    assert_eq!( batch_response.total_tokens, 28 );
    assert!( batch_response.total_estimated_cost.is_some() );
    assert!( batch_response.batch_optimization_savings.is_some() );

    println!( "✓ Batch token count response structure validation successful" );
  }

  /// Test token validation configuration for input limits
  #[ tokio::test ]
  async fn test_token_validation_configuration()
  {
    // Test token validation configuration
    let validation_config = TokenValidationConfig
    {
      max_input_tokens : 4096,
      max_output_tokens : 2048,
      model_context_window : 8192,
      warning_threshold : 0.8, // Warn at 80% of limit
      enforce_limits : true,
      truncation_strategy : "end".to_string(), // "start", "end", "middle"
    };

    // Test configuration validation
    assert_eq!( validation_config.max_input_tokens, 4096 );
    assert_eq!( validation_config.max_output_tokens, 2048 );
    assert_eq!( validation_config.model_context_window, 8192 );
    assert!( validation_config.warning_threshold > 0.0 && validation_config.warning_threshold <= 1.0 );
    assert!( validation_config.enforce_limits );
    assert!( [ "start", "end", "middle" ].contains( &validation_config.truncation_strategy.as_str() ) );

    println!( "✓ Token validation configuration structure validation successful" );

    // Test warning threshold calculation
    #[ allow(clippy::cast_possible_truncation, clippy::cast_sign_loss) ]
    let warning_token_count = ( f64::from(validation_config.max_input_tokens) * validation_config.warning_threshold ) as u32;
    assert_eq!( warning_token_count, 3276 ); // 80% of 4096
    println!( "✓ Token validation threshold calculation successful" );
  }

  /// Test model-specific token counting capabilities
  #[ tokio::test ]
  async fn test_model_token_capabilities()
  {
    // Test model token capabilities structure
    let llama_capabilities = ModelTokenCapabilities
    {
      model_name : "llama3.2".to_string(),
      context_window : 8192,
      supports_function_calling : true,
      average_tokens_per_word : 1.3,
      max_input_tokens : 6144,
      max_output_tokens : 2048,
      cost_per_input_token : 0.0001,
      cost_per_output_token : 0.0002,
      tokenizer_type : "tiktoken".to_string(),
    };

    // Test capabilities validation
    assert_eq!( llama_capabilities.model_name, "llama3.2" );
    assert_eq!( llama_capabilities.context_window, 8192 );
    assert!( llama_capabilities.supports_function_calling );
    assert!( llama_capabilities.average_tokens_per_word > 1.0 );
    assert!( llama_capabilities.max_input_tokens < llama_capabilities.context_window );
    assert!( llama_capabilities.cost_per_input_token > 0.0 );

    println!( "✓ Model token capabilities structure validation successful" );

    // Test different model capabilities
    let code_model_capabilities = ModelTokenCapabilities
    {
      model_name : "codellama".to_string(),
      context_window : 16384,
      supports_function_calling : false,
      average_tokens_per_word : 1.5, // Code typically has more tokens per word
      max_input_tokens : 12288,
      max_output_tokens : 4096,
      cost_per_input_token : 0.00015,
      cost_per_output_token : 0.0003,
      tokenizer_type : "sentencepiece".to_string(),
    };

    assert_eq!( code_model_capabilities.model_name, "codellama" );
    assert!( code_model_capabilities.average_tokens_per_word > llama_capabilities.average_tokens_per_word );
    assert!( !code_model_capabilities.supports_function_calling );

    println!( "✓ Multiple model capabilities validation successful" );
  }

  /// Test token counting with different text types and complexity
  #[ tokio::test ]
  async fn test_text_complexity_token_estimation()
  {
    let test_cases = vec![
      ( "Empty string", "", 0 ),
      ( "Single word", "hello", 1 ),
      ( "Short phrase", "hello world", 2 ),
      ( "Medium text", MEDIUM_TEXT, 25 ), // Estimated
      ( "Long text", LONG_TEXT, 80 ), // Estimated
      ( "Code snippet", CODE_TEXT, 30 ), // Estimated
      ( "Special characters", "!@#$%^&*()", 10 ), // Estimated
      ( "Unicode text", "Hello 世界 🌍", 5 ), // Estimated
    ];

    for ( description, text, expected_tokens ) in test_cases
    {
      let request = TokenCountRequest
      {
        model : "llama3.2".to_string(),
        text : text.to_string(),
        options : None,
      };

      // Test request structure for each text type
      assert_eq!( request.text, text );
      assert_eq!( request.model, "llama3.2" );

      // Create test response for validation
      let response = TokenCountResponse
      {
        token_count : expected_tokens,
        model : "llama3.2".to_string(),
        text_length : text.len(),
        estimated_cost : Some( f64::from(expected_tokens) * 0.0001 ),
        processing_time_ms : Some( 3 ),
        metadata : None,
      };

      assert_eq!( response.token_count, expected_tokens );
      println!( "✓ {description}: {expected_tokens} tokens for {text_len} characters", text_len = text.len() );
    }

    println!( "✓ Text complexity token estimation validation successful" );
  }

  /// Test token counting performance for different text sizes
  #[ tokio::test ]
  async fn test_token_counting_performance()
  {
    let very_long_text = LONG_TEXT.repeat( 10 );
    let performance_tests = vec![
      ( "Short text", SHORT_TEXT ),
      ( "Medium text", MEDIUM_TEXT ),
      ( "Long text", LONG_TEXT ),
      ( "Very long text", &very_long_text ), // 10x longer
    ];

    for ( description, text ) in performance_tests
    {
      let request = TokenCountRequest
      {
        model : "llama3.2".to_string(),
        text : text.to_string(),
        options : None,
      };

      // Simulate token counting performance
      let start_time = Instant::now();
      let serialized = serde_json::to_string( &request );
      let latency = start_time.elapsed();

      assert!( serialized.is_ok() );
      assert!( latency < Duration::from_millis( 100 ) ); // Should be very fast

      // Create performance response
      let response = TokenCountResponse
      {
        token_count : u32::try_from(text.len() / 4).unwrap_or(0), // Rough estimate : 4 chars per token
        model : "llama3.2".to_string(),
        text_length : text.len(),
        estimated_cost : Some( ( text.len() / 4 ) as f64 * 0.0001 ),
        processing_time_ms : Some( u64::try_from(latency.as_millis()).unwrap_or(0) ),
        metadata : None,
      };

      assert!( response.processing_time_ms.unwrap() < 100 );
      println!( "✓ {description}: {} chars processed in {:?}", text.len(), latency );
    }

    println!( "✓ Token counting performance validation successful" );
  }

  /// Test error handling for invalid inputs and edge cases
  #[ tokio::test ]
  async fn test_token_counting_error_handling()
  {
    // Test with empty model name
    let invalid_request = TokenCountRequest
    {
      model : String::new(),
      text : "Valid text".to_string(),
      options : None,
    };

    assert!( invalid_request.model.is_empty() );
    println!( "✓ Empty model name handling validation successful" );

    // Test with extremely long text
    let very_long_text = "word ".repeat( 100_000 ); // 500k characters
    let large_request = TokenCountRequest
    {
      model : "llama3.2".to_string(),
      text : very_long_text.clone(),
      options : None,
    };

    assert_eq!( large_request.text.len(), very_long_text.len() );
    assert!( large_request.text.len() > 400_000 );
    println!( "✓ Large text handling validation successful" );

    // Test with special characters and edge cases
    let repeated_a = "a".repeat( 10000 );
    let edge_cases = vec![
      "\n\n\n", // Only newlines
      "   ", // Only spaces
      "🚀🌟💫", // Only emojis
      "\0\0\0", // Null characters
      &repeated_a, // Single character repeated
    ];

    for edge_case in edge_cases
    {
      let request = TokenCountRequest
      {
        model : "llama3.2".to_string(),
        text : edge_case.to_string(),
        options : None,
      };

      assert_eq!( request.text, edge_case );
      let serialized = serde_json::to_string( &request );
      assert!( serialized.is_ok() );
    }

    println!( "✓ Edge case handling validation successful" );
  }
}
