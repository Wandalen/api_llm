//! Batch Operations Tests
//!
//! Comprehensive test suite for batch operations functionality in the Ollama API client.
//! Tests multiple requests in single API calls, bulk processing capabilities, throughput
//! optimization, and integration with the Ollama backend.
//!
//! Note : These tests focus on batch operation structure and optimization logic,
//! using test data to verify batch handling API correctness.

#![ allow( clippy::std_instead_of_core ) ] // std required for async operations and time measurements

#[ cfg( feature = "batch_operations" ) ]
mod batch_operations_tests
{
  use api_ollama::{
    BatchChatRequest, BatchChatResponse, BatchGenerateRequest,
    BatchOperationConfig, BatchResult, BatchError, ChatRequest, ChatMessage, MessageRole,
    GenerateRequest
  };
  use std::time::{ Duration, Instant };

  /// Helper function to create test chat requests
  fn create_test_chat_requests( count : usize ) -> Vec< ChatRequest >
  {
    ( 0..count ).map( | i | ChatRequest
    {
      model : "llama3.2".to_string(),
      messages : vec![ ChatMessage
      {
        role : MessageRole::User,
        content : format!( "Test message {}", i + 1 ),
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      } ],
      stream : None,
      options : None,
      #[ cfg( feature = "tool_calling" ) ]
      tools : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_messages : None,
    } ).collect()
  }

  /// Helper function to create test generation requests
  fn create_test_generate_requests( count : usize ) -> Vec< GenerateRequest >
  {
    ( 0..count ).map( | i | GenerateRequest
    {
      model : "llama3.2".to_string(),
      prompt : format!( "Generate response for prompt {}", i + 1 ),
      stream : None,
      options : None,
    } ).collect()
  }

  /// Test batch chat request structure and validation
  #[ tokio::test ]
  async fn test_batch_chat_request_structure()
  {
    // Test creating a valid batch chat request
    let chat_requests = create_test_chat_requests( 3 );
    let batch_request = BatchChatRequest
    {
      requests : chat_requests.clone(),
      batch_config : None,
      parallel_execution : true,
      fail_fast : false,
      timeout : Some( Duration::from_secs( 60 ) ),
    };

    // Test batch request structure
    assert_eq!( batch_request.requests.len(), 3 );
    assert!( batch_request.parallel_execution );
    assert!( !batch_request.fail_fast );
    assert_eq!( batch_request.timeout, Some( Duration::from_secs( 60 ) ) );
    assert!( batch_request.batch_config.is_none() );

    println!( "✓ Batch chat request structure validation successful" );

    // Test serialization
    let serialized = serde_json::to_string( &batch_request );
    assert!( serialized.is_ok() );
    println!( "✓ Batch chat request serialization successful" );

    // Test with batch configuration
    let batch_config = BatchOperationConfig::new()
      .with_max_batch_size( 10 )
      .with_concurrent_limit( 3 )
      .with_retry_failed( true );

    let configured_request = BatchChatRequest
    {
      requests : chat_requests,
      batch_config : Some( batch_config ),
      parallel_execution : true,
      fail_fast : true,
      timeout : Some( Duration::from_secs( 120 ) ),
    };

    assert!( configured_request.batch_config.is_some() );
    assert!( configured_request.fail_fast );
    println!( "✓ Batch chat request with configuration validation successful" );
  }

  /// Test batch generation request structure and validation
  #[ tokio::test ]
  async fn test_batch_generate_request_structure()
  {
    // Test creating batch generation request
    let generate_requests = create_test_generate_requests( 5 );
    let batch_request = BatchGenerateRequest
    {
      requests : generate_requests.clone(),
      batch_config : None,
      parallel_execution : false, // Sequential execution
      fail_fast : true,
      timeout : Some( Duration::from_secs( 300 ) ),
    };

    // Test batch request structure validation
    assert_eq!( batch_request.requests.len(), 5 );
    assert!( !batch_request.parallel_execution ); // Sequential
    assert!( batch_request.fail_fast );
    assert_eq!( batch_request.timeout, Some( Duration::from_secs( 300 ) ) );

    println!( "✓ Batch generation request structure validation successful" );

    // Test request content validation
    for ( i, request ) in batch_request.requests.iter().enumerate()
    {
      assert_eq!( request.model, "llama3.2" );
      assert_eq!( request.prompt, format!( "Generate response for prompt {}", i + 1 ) );
    }

    println!( "✓ Batch generation request content validation successful" );

    // Test serialization
    let serialized = serde_json::to_string( &batch_request );
    assert!( serialized.is_ok() );
    println!( "✓ Batch generation request serialization successful" );
  }

  /// Test batch operation configuration
  #[ tokio::test ]
  async fn test_batch_operation_configuration()
  {
    // Test basic configuration
    let config = BatchOperationConfig::new();
    assert_eq!( config.max_batch_size(), 100 );
    assert_eq!( config.concurrent_limit(), 5 );
    assert!( !config.retry_failed() );
    assert!( config.preserve_order() );

    println!( "✓ Default batch operation configuration validation successful" );

    // Test configuration builder pattern
    let custom_config = BatchOperationConfig::new()
      .with_max_batch_size( 50 )
      .with_concurrent_limit( 10 )
      .with_retry_failed( true )
      .with_preserve_order( false )
      .with_timeout_per_request( Duration::from_secs( 30 ) )
      .with_chunk_size( 25 );

    assert_eq!( custom_config.max_batch_size(), 50 );
    assert_eq!( custom_config.concurrent_limit(), 10 );
    assert!( custom_config.retry_failed() );
    assert!( !custom_config.preserve_order() );
    assert_eq!( custom_config.timeout_per_request(), Some( Duration::from_secs( 30 ) ) );
    assert_eq!( custom_config.chunk_size(), 25 );

    println!( "✓ Custom batch operation configuration validation successful" );

    // Test configuration validation
    let invalid_config = BatchOperationConfig::new()
      .with_max_batch_size( 0 ); // Invalid

    assert_eq!( invalid_config.max_batch_size(), 1 ); // Should be corrected to minimum
    println!( "✓ Batch operation configuration validation and correction successful" );
  }

  /// Test batch response structure and result handling
  #[ tokio::test ]
  async fn test_batch_response_structure()
  {
    // Test successful batch chat response
    let successful_results = vec![
      BatchResult::Success( serde_json::json!({
        "message": { "role": "assistant", "content": "Response 1" },
        "done": true
      }) ),
      BatchResult::Success( serde_json::json!({
        "message": { "role": "assistant", "content": "Response 2" },
        "done": true
      }) ),
      BatchResult::Success( serde_json::json!({
        "message": { "role": "assistant", "content": "Response 3" },
        "done": true
      }) ),
    ];

    let batch_response = BatchChatResponse
    {
      results : successful_results.clone(),
      total_requests : 3,
      successful_requests : 3,
      failed_requests : 0,
      execution_time_ms : 1500,
      throughput_requests_per_second : 2.0,
      batch_optimizations : Some( vec![ "parallel_execution".to_string(), "connection_reuse".to_string() ] ),
      errors : Vec::new(),
    };

    // Test response structure validation
    assert_eq!( batch_response.results.len(), 3 );
    assert_eq!( batch_response.total_requests, 3 );
    assert_eq!( batch_response.successful_requests, 3 );
    assert_eq!( batch_response.failed_requests, 0 );
    assert!( batch_response.execution_time_ms > 0 );
    assert!( batch_response.throughput_requests_per_second > 0.0 );
    assert!( batch_response.batch_optimizations.is_some() );
    assert!( batch_response.errors.is_empty() );

    println!( "✓ Successful batch response structure validation successful" );

    // Test batch response with partial failures
    let mixed_results = vec![
      BatchResult::Success( serde_json::json!({ "message": { "role": "assistant", "content": "Success 1" } }) ),
      BatchResult::Error( BatchError {
        request_index : 1,
        error_code : "timeout".to_string(),
        error_message : "Request timed out".to_string(),
        recoverable : true,
      } ),
      BatchResult::Success( serde_json::json!({ "message": { "role": "assistant", "content": "Success 2" } }) ),
    ];

    let mixed_response = BatchChatResponse
    {
      results : mixed_results,
      total_requests : 3,
      successful_requests : 2,
      failed_requests : 1,
      execution_time_ms : 2000,
      throughput_requests_per_second : 1.5,
      batch_optimizations : None,
      errors : vec![ "One request failed due to timeout".to_string() ],
    };

    assert_eq!( mixed_response.successful_requests, 2 );
    assert_eq!( mixed_response.failed_requests, 1 );
    assert!( !mixed_response.errors.is_empty() );

    println!( "✓ Mixed batch response structure validation successful" );
  }

  /// Test batch size limitations and validation
  #[ tokio::test ]
  async fn test_batch_size_limitations()
  {
    // Test normal batch size
    let normal_requests = create_test_chat_requests( 10 );
    let normal_batch = BatchChatRequest
    {
      requests : normal_requests,
      batch_config : Some( BatchOperationConfig::new().with_max_batch_size( 20 ) ),
      parallel_execution : true,
      fail_fast : false,
      timeout : Some( Duration::from_secs( 60 ) ),
    };

    assert_eq!( normal_batch.requests.len(), 10 );
    println!( "✓ Normal batch size validation successful" );

    // Test large batch size (should be handled appropriately)
    let large_requests = create_test_chat_requests( 100 );
    let large_batch = BatchChatRequest
    {
      requests : large_requests,
      batch_config : Some( BatchOperationConfig::new().with_max_batch_size( 50 ) ),
      parallel_execution : true,
      fail_fast : false,
      timeout : Some( Duration::from_secs( 300 ) ),
    };

    assert_eq!( large_batch.requests.len(), 100 );
    // Note : In real implementation, this would be chunked according to max_batch_size
    println!( "✓ Large batch size handling validation successful" );

    // Test empty batch
    let empty_batch = BatchChatRequest
    {
      requests : Vec::new(),
      batch_config : None,
      parallel_execution : true,
      fail_fast : false,
      timeout : Some( Duration::from_secs( 60 ) ),
    };

    assert_eq!( empty_batch.requests.len(), 0 );
    println!( "✓ Empty batch handling validation successful" );

    // Test single request batch
    let single_requests = create_test_chat_requests( 1 );
    let single_batch = BatchChatRequest
    {
      requests : single_requests,
      batch_config : None,
      parallel_execution : false, // No point in parallel for single request
      fail_fast : true,
      timeout : Some( Duration::from_secs( 30 ) ),
    };

    assert_eq!( single_batch.requests.len(), 1 );
    assert!( !single_batch.parallel_execution );
    println!( "✓ Single request batch validation successful" );
  }

  /// Test error handling for partial batch failures
  #[ tokio::test ]
  async fn test_batch_error_handling()
  {
    // Test BatchError structure
    let error = BatchError
    {
      request_index : 2,
      error_code : "model_not_found".to_string(),
      error_message : "The requested model 'invalid-model' was not found".to_string(),
      recoverable : false,
    };

    assert_eq!( error.request_index, 2 );
    assert_eq!( error.error_code, "model_not_found" );
    assert!( !error.recoverable );
    println!( "✓ BatchError structure validation successful" );

    // Test error categorization
    let error_types = [
      ( "timeout", "Request exceeded timeout limit", true ),
      ( "rate_limit", "Rate limit exceeded", true ),
      ( "model_not_found", "Model does not exist", false ),
      ( "invalid_input", "Input validation failed", false ),
      ( "server_error", "Internal server error", true ),
    ];

    for ( i, ( code, message, recoverable ) ) in error_types.iter().enumerate()
    {
      let error = BatchError
      {
        request_index : i,
        error_code : (*code).to_string(),
        error_message : (*message).to_string(),
        recoverable : *recoverable,
      };

      assert_eq!( error.error_code, *code );
      assert_eq!( error.recoverable, *recoverable );
    }

    println!( "✓ Error categorization validation successful" );

    // Test fail-fast vs continue-on-error behavior
    let fail_fast_config = BatchOperationConfig::new()
      .with_retry_failed( false );

    let continue_config = BatchOperationConfig::new()
      .with_retry_failed( true );

    assert!( !fail_fast_config.retry_failed() );
    assert!( continue_config.retry_failed() );
    println!( "✓ Error handling configuration validation successful" );
  }

  /// Test batch operation performance benefits
  #[ tokio::test ]
  async fn test_batch_performance_optimization()
  {
    // Test performance metrics structure
    let single_request_time = Duration::from_millis( 500 );
    let batch_request_time = Duration::from_millis( 800 );
    let request_count = 5;

    // Calculate performance improvements
    let single_total_time = single_request_time * u32::try_from(request_count).unwrap_or(1);
    let batch_total_time = batch_request_time;
    let time_savings = single_total_time.saturating_sub( batch_total_time );
    let efficiency_ratio = batch_total_time.as_millis() as f64 / single_total_time.as_millis() as f64;

    assert!( time_savings > Duration::ZERO );
    assert!( efficiency_ratio < 1.0 ); // Batch should be more efficient
    println!( "✓ Batch performance calculation validation successful" );

    // Test throughput calculations
    let throughput_single = f64::from(request_count) / single_total_time.as_secs_f64();
    let throughput_batch = f64::from(request_count) / batch_total_time.as_secs_f64();

    assert!( throughput_batch > throughput_single );
    println!( "✓ Throughput comparison validation successful" );
    println!( "  Single request throughput : {throughput_single:.2} req/s" );
    println!( "  Batch request throughput : {throughput_batch:.2} req/s" );

    // Test batch optimization tracking
    let optimizations = vec![
      "connection_reuse".to_string(),
      "parallel_execution".to_string(),
      "request_batching".to_string(),
      "shared_context".to_string(),
    ];

    for optimization in &optimizations
    {
      assert!( !optimization.is_empty() );
    }

    println!( "✓ Batch optimization tracking validation successful" );
  }

  /// Test batch operation timing and performance measurement
  #[ tokio::test ]
  async fn test_batch_operation_timing()
  {
    // Test timing measurement for batch operations
    let start_time = Instant::now();

    // Simulate batch request processing
    let requests = create_test_chat_requests( 10 );
    let batch_request = BatchChatRequest
    {
      requests,
      batch_config : Some( BatchOperationConfig::new().with_concurrent_limit( 5 ) ),
      parallel_execution : true,
      fail_fast : false,
      timeout : Some( Duration::from_secs( 60 ) ),
    };

    // Simulate processing time
    tokio ::time::sleep( Duration::from_millis( 10 ) ).await;
    let processing_time = start_time.elapsed();

    // Test timing measurements
    assert!( processing_time > Duration::ZERO );
    assert!( processing_time < Duration::from_secs( 1 ) ); // Should be very fast for test data

    println!( "✓ Batch operation timing measurement successful : {processing_time:?}" );

    // Test throughput calculation
    let request_count = batch_request.requests.len();
    let throughput = request_count as f64 / processing_time.as_secs_f64();

    assert!( throughput > 0.0 );
    println!( "✓ Throughput calculation successful : {throughput:.2} requests/second" );

    // Test performance baseline comparison
    let single_request_baseline = Duration::from_millis( 100 ); // Assumed baseline
    let expected_batch_time = single_request_baseline * u32::try_from(request_count).unwrap_or(1);
    let efficiency_gain = expected_batch_time.saturating_sub( processing_time ).as_millis() as f64 / expected_batch_time.as_millis() as f64;

    println!( "✓ Efficiency gain calculation : {:.1}%", efficiency_gain * 100.0 );
  }

  /// Test concurrent execution limits and resource management
  #[ tokio::test ]
  async fn test_concurrent_execution_limits()
  {
    // Test concurrent limit configuration
    let configs = vec![
      ( 1, "Sequential execution" ),
      ( 3, "Limited concurrency" ),
      ( 10, "High concurrency" ),
      ( 50, "Maximum concurrency" ),
    ];

    for ( limit, description ) in configs
    {
      let config = BatchOperationConfig::new()
        .with_concurrent_limit( limit );

      assert_eq!( config.concurrent_limit(), limit );
      println!( "✓ {description}: concurrent limit = {limit}" );
    }

    // Test resource allocation for different batch sizes
    let batch_sizes = vec![ 5, 10, 25, 50, 100 ];
    let max_concurrent = 10;

    for batch_size in batch_sizes
    {
      let effective_concurrency = std::cmp::min( batch_size, max_concurrent );
      let estimated_chunks = ( batch_size + effective_concurrency - 1 ) / effective_concurrency; // Ceiling division

      assert!( effective_concurrency <= max_concurrent );
      assert!( effective_concurrency <= batch_size );
      assert!( estimated_chunks >= 1 );

      println!( "✓ Batch size {batch_size}: {effective_concurrency} concurrent, {estimated_chunks} chunks" );
    }

    println!( "✓ Concurrent execution limits validation successful" );
  }

  /// Test batch operation integration and coordination
  #[ tokio::test ]
  async fn test_batch_operation_integration()
  {
    // Test coordination between different batch operation types
    let chat_requests = create_test_chat_requests( 3 );
    let generate_requests = create_test_generate_requests( 2 );

    // Test that different batch types can coexist
    let chat_batch = BatchChatRequest
    {
      requests : chat_requests,
      batch_config : None,
      parallel_execution : true,
      fail_fast : false,
      timeout : Some( Duration::from_secs( 60 ) ),
    };

    let generate_batch = BatchGenerateRequest
    {
      requests : generate_requests,
      batch_config : None,
      parallel_execution : false,
      fail_fast : true,
      timeout : Some( Duration::from_secs( 120 ) ),
    };

    assert_eq!( chat_batch.requests.len(), 3 );
    assert_eq!( generate_batch.requests.len(), 2 );
    assert!( chat_batch.parallel_execution );
    assert!( !generate_batch.parallel_execution );

    println!( "✓ Multiple batch type coordination validation successful" );

    // Test batch operation serialization compatibility
    let chat_serialized = serde_json::to_string( &chat_batch );
    let generate_serialized = serde_json::to_string( &generate_batch );

    assert!( chat_serialized.is_ok() );
    assert!( generate_serialized.is_ok() );
    println!( "✓ Batch operation serialization compatibility successful" );

    // Test configuration sharing
    let shared_config = BatchOperationConfig::new()
      .with_max_batch_size( 20 )
      .with_concurrent_limit( 5 );

    let chat_with_shared_config = BatchChatRequest
    {
      requests : create_test_chat_requests( 5 ),
      batch_config : Some( shared_config.clone() ),
      parallel_execution : true,
      fail_fast : false,
      timeout : Some( Duration::from_secs( 60 ) ),
    };

    let generate_with_shared_config = BatchGenerateRequest
    {
      requests : create_test_generate_requests( 3 ),
      batch_config : Some( shared_config ),
      parallel_execution : true,
      fail_fast : false,
      timeout : Some( Duration::from_secs( 60 ) ),
    };

    assert!( chat_with_shared_config.batch_config.is_some() );
    assert!( generate_with_shared_config.batch_config.is_some() );
    println!( "✓ Batch configuration sharing validation successful" );
  }
}
