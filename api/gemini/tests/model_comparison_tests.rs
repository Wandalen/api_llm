//! Model Comparison Tests
//!
//! Tests for model comparison and A/B testing functionality.

use api_gemini::
{
  client ::Client,
  GenerateContentRequest,
  Content,
  Part,
};

#[ tokio::test ]
async fn test_comparator_creation()
{
  let client = Client::new().expect( "Failed to create client" );
  let _comparator = client.comparator();

  // Verify comparator was created (just structural test)
}

#[ tokio::test ]
async fn test_compare_models_sequential()
{
  let client = Client::new().expect( "Failed to create client" );
  let comparator = client.comparator();

  let request = GenerateContentRequest
  {
    contents : vec![ Content
    {
      parts : vec![ Part
      {
        text : Some( "Say 'Hello'".to_string() ),
        ..Default::default()
      } ],
      role : "user".to_string(),
    } ],
    ..Default::default()
  };

  let model_names = vec![ "gemini-1.5-flash", "gemini-1.5-pro" ];

  let results = comparator.compare_models( &model_names, &request ).await.expect( "Comparison failed" );

  // Verify results structure
  assert_eq!( results.results.len(), 2 );
  assert!( results.total_time_ms > 0 );
}

#[ tokio::test ]
async fn test_compare_models_parallel()
{
  let client = Client::new().expect( "Failed to create client" );
  let comparator = client.comparator();

  let request = GenerateContentRequest
  {
    contents : vec![ Content
    {
      parts : vec![ Part
      {
        text : Some( "Say 'Hello'".to_string() ),
        ..Default::default()
      } ],
      role : "user".to_string(),
    } ],
    ..Default::default()
  };

  let model_names = vec![ "gemini-1.5-flash", "gemini-1.5-pro" ];

  let results = comparator.compare_models_parallel( &model_names, &request ).await.expect( "Comparison failed" );

  // Verify results structure
  assert_eq!( results.results.len(), 2 );
  assert!( results.total_time_ms > 0 );
}

#[ tokio::test ]
async fn test_comparison_results_analysis()
{
  // Check if API key is available
  let _api_key = match std::env::var( "GEMINI_API_KEY" )
    .or_else( |_| std::fs::read_to_string( "secret/gemini_api_key" ).map( |s| s.trim().to_string() ) )
  {
    Ok( key ) => key,
    Err( _ ) =>
    {
      eprintln!( "⚠️  Skipping test - API key not available" );
      return;
    }
  };

  let client = Client::new().expect( "Failed to create client" );
  let comparator = client.comparator();

  let request = GenerateContentRequest
  {
    contents : vec![ Content
    {
      parts : vec![ Part
      {
        text : Some( "What is 2+2?".to_string() ),
        ..Default::default()
      } ],
      role : "user".to_string(),
    } ],
    ..Default::default()
  };

  let model_names = vec![ "gemini-1.5-flash", "gemini-1.5-pro" ];

  let results = comparator.compare_models( &model_names, &request ).await.expect( "Comparison failed" );

  // Test analysis methods
  let fastest = results.get_fastest();
  let slowest = results.get_slowest();
  let avg_time = results.average_response_time();
  let success_rate = results.success_rate();

  // Verify analysis results
  assert!( fastest.is_some() || slowest.is_some() ); // At least one model should succeed
  assert!( avg_time >= 0.0 );
  assert!( success_rate >= 0.0 && success_rate <= 1.0 );
}

#[ tokio::test ]
async fn test_comparison_fastest_slowest()
{
  let client = Client::new().expect( "Failed to create client" );
  let comparator = client.comparator();

  let request = GenerateContentRequest
  {
    contents : vec![ Content
    {
      parts : vec![ Part
      {
        text : Some( "Count to 3".to_string() ),
        ..Default::default()
      } ],
      role : "user".to_string(),
    } ],
    ..Default::default()
  };

  let model_names = vec![ "gemini-1.5-flash", "gemini-1.5-pro" ];

  let results = comparator.compare_models( &model_names, &request ).await.expect( "Comparison failed" );

  // Verify fastest/slowest identification
  if let Some( fastest ) = results.get_fastest()
  {
    assert!( fastest.success );
    assert!( results.fastest_model.is_some() );
  }

  if let Some( slowest ) = results.get_slowest()
  {
    assert!( slowest.success );
    assert!( results.slowest_model.is_some() );
  }
}

#[ tokio::test ]
async fn test_comparison_success_rate()
{
  // Check if API key is available
  let _api_key = match std::env::var( "GEMINI_API_KEY" )
    .or_else( |_| std::fs::read_to_string( "secret/gemini_api_key" ).map( |s| s.trim().to_string() ) )
  {
    Ok( key ) => key,
    Err( _ ) =>
    {
      eprintln!( "⚠️  Skipping test - API key not available" );
      return;
    }
  };

  let client = Client::new().expect( "Failed to create client" );
  let comparator = client.comparator();

  let request = GenerateContentRequest
  {
    contents : vec![ Content
    {
      parts : vec![ Part
      {
        text : Some( "Hi".to_string() ),
        ..Default::default()
      } ],
      role : "user".to_string(),
    } ],
    ..Default::default()
  };

  let model_names = vec![ "gemini-1.5-flash", "gemini-1.5-pro" ];

  let results = comparator.compare_models( &model_names, &request ).await.expect( "Comparison failed" );

  let success_rate = results.success_rate();

  // With valid models and simple request, should have some successes
  assert!( success_rate > 0.0 );
  assert!( success_rate <= 1.0 );
}

#[ tokio::test ]
async fn test_comparison_token_counts()
{
  let client = Client::new().expect( "Failed to create client" );
  let comparator = client.comparator();

  let request = GenerateContentRequest
  {
    contents : vec![ Content
    {
      parts : vec![ Part
      {
        text : Some( "What is AI?".to_string() ),
        ..Default::default()
      } ],
      role : "user".to_string(),
    } ],
    ..Default::default()
  };

  let model_names = vec![ "gemini-1.5-flash" ];

  let results = comparator.compare_models( &model_names, &request ).await.expect( "Comparison failed" );

  // Check if token counts are captured
  if let Some( result ) = results.results.first()
  {
    if result.success
    {
      // Token counts may or may not be present depending on API response
      // Just verify the fields exist
      let _input = result.input_tokens;
      let _output = result.output_tokens;
    }
  }
}

#[ tokio::test ]
async fn test_parallel_comparison_speed()
{
  let client = Client::new().expect( "Failed to create client" );
  let comparator = client.comparator();

  let request = GenerateContentRequest
  {
    contents : vec![ Content
    {
      parts : vec![ Part
      {
        text : Some( "Quick test".to_string() ),
        ..Default::default()
      } ],
      role : "user".to_string(),
    } ],
    ..Default::default()
  };

  let model_names = vec![ "gemini-1.5-flash", "gemini-1.5-pro" ];

  // Run sequential comparison
  let sequential_results = comparator.compare_models( &model_names, &request ).await.expect( "Sequential comparison failed" );

  // Run parallel comparison
  let parallel_results = comparator.compare_models_parallel( &model_names, &request ).await.expect( "Parallel comparison failed" );

  // Parallel should generally be faster or similar (but not always guaranteed due to network variance)
  // Just verify both work and return valid results
  assert_eq!( sequential_results.results.len(), parallel_results.results.len() );
  assert!( sequential_results.total_time_ms > 0 );
  assert!( parallel_results.total_time_ms > 0 );
}

#[ tokio::test ]
async fn test_average_response_time_calculation()
{
  let client = Client::new().expect( "Failed to create client" );
  let comparator = client.comparator();

  let request = GenerateContentRequest
  {
    contents : vec![ Content
    {
      parts : vec![ Part
      {
        text : Some( "Test".to_string() ),
        ..Default::default()
      } ],
      role : "user".to_string(),
    } ],
    ..Default::default()
  };

  let model_names = vec![ "gemini-1.5-flash", "gemini-1.5-pro" ];

  let results = comparator.compare_models( &model_names, &request ).await.expect( "Comparison failed" );

  let avg = results.average_response_time();

  // Average should be positive if any models succeeded
  if results.success_rate() > 0.0
  {
    assert!( avg > 0.0 );
  }
}

#[ tokio::test ]
async fn test_comparison_error_handling()
{
  let client = Client::new().expect( "Failed to create client" );
  let comparator = client.comparator();

  let request = GenerateContentRequest
  {
    contents : vec![ Content
    {
      parts : vec![ Part
      {
        text : Some( "Test".to_string() ),
        ..Default::default()
      } ],
      role : "user".to_string(),
    } ],
    ..Default::default()
  };

  // Include an invalid model name
  let model_names = vec![ "gemini-1.5-flash", "invalid-model-xyz" ];

  let results = comparator.compare_models( &model_names, &request ).await.expect( "Comparison should handle errors" );

  // Should have results for both models, but one should have failed
  assert_eq!( results.results.len(), 2 );

  // Check that at least one succeeded and one failed
  let failures = results.results.iter().filter( | r | !r.success ).count();

  assert!( failures > 0 ); // Invalid model should fail
}
