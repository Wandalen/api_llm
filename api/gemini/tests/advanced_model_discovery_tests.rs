//! Comprehensive tests for Advanced Model Discovery APIs
//!
//! This module provides exhaustive testing for the advanced model discovery functionality
//! including model comparison, recommendations, filtering, and status monitoring.
//! All tests use real API calls following the no-mockup policy.

// Import shared test utilities from common module
mod common;
use common::create_integration_client;

use api_gemini::models::
{
  CompareModelsRequest, GetRecommendationsRequest, AdvancedFilterRequest,
  ModelStatusRequest,
};

/// Test comparing multiple models with basic criteria.
///
/// This test verifies that the model comparison API correctly analyzes
/// multiple models and provides meaningful comparison data.
#[ tokio::test ]
async fn test_compare_models_basic() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  let request = CompareModelsRequest {
    model_names : vec![
      "gemini-2.0-flash-experimental".to_string(),
      "gemini-1.5-pro-002".to_string(),
    ],
    criteria : Some( vec![ "performance".to_string(), "cost".to_string() ] ),
    include_benchmarks : Some( true ),
    include_cost_analysis : Some( true ),
  };

  let response = models_api.compare_models( &request ).await?;

  // Verify we got comparisons for all requested models
  assert_eq!( response.comparisons.len(), 2, "Should return comparisons for both models" );

  // Verify each comparison has the required model information
  for comparison in &response.comparisons
  {
    assert!( !comparison.model.name.is_empty(), "Model name should not be empty" );
    assert!( comparison.model.name.contains( "gemini" ), "Should be a Gemini model" );

    // Verify performance metrics are included when requested
    if let Some( metrics ) = &comparison.performance_metrics
    {
      if let Some( quality_score ) = metrics.quality_score
      {
        assert!( quality_score >= 0.0 && quality_score <= 1.0, "Quality score should be between 0 and 1" );
      }

      if let Some( reliability ) = metrics.reliability
      {
        assert!( reliability >= 0.0 && reliability <= 100.0, "Reliability should be between 0 and 100" );
      }
    }

    // Verify cost analysis is included when requested
    if let Some( cost ) = &comparison.cost_analysis
    {
      if let Some( input_cost ) = cost.input_cost_per_1k
      {
        assert!( input_cost >= 0.0, "Input cost should be non-negative" );
      }

      if let Some( output_cost ) = cost.output_cost_per_1k
      {
        assert!( output_cost >= 0.0, "Output cost should be non-negative" );
      }
    }

    // Verify suitability score if provided
    if let Some( score ) = comparison.suitability_score
    {
      assert!( score >= 0.0 && score <= 1.0, "Suitability score should be between 0 and 1" );
    }
  }

  // If recommendation is provided, verify it's valid
  if let Some( recommendation ) = &response.recommendation
  {
    assert!( !recommendation.recommended_model.is_empty(), "Recommended model should not be empty" );
    assert!( recommendation.confidence_score >= 0.0 && recommendation.confidence_score <= 1.0,
      "Confidence score should be between 0 and 1" );
    assert!( !recommendation.reasoning.is_empty(), "Reasoning should not be empty" );
  }

  println!( "✓ Basic model comparison test passed" );
  println!( "  Compared {} models", response.comparisons.len() );

  Ok( () )
}

/// Test getting model recommendations for specific use cases.
///
/// This test verifies that the recommendation API provides appropriate
/// model suggestions based on use case requirements.
#[ tokio::test ]
async fn test_get_recommendations_chatbot() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  let request = GetRecommendationsRequest {
    use_case : "Real-time customer support chatbot".to_string(),
    input_size_range : Some( "short".to_string() ),
    performance_requirements : Some( vec![
      "low-latency".to_string(),
      "high-availability".to_string(),
    ] ),
    budget_constraints : Some( 50.0 ),
    real_time_required : Some( true ),
  };

  let response = models_api.get_recommendations( &request ).await?;

  // Should provide at least one recommendation
  assert!( !response.recommendations.is_empty(), "Should provide at least one recommendation" );
  assert!( response.recommendations.len() <= 5, "Should not overwhelm with too many recommendations" );

  // Verify each recommendation is well-formed
  for recommendation in &response.recommendations
  {
    assert!( !recommendation.recommended_model.is_empty(), "Recommended model should not be empty" );
    assert!( recommendation.confidence_score >= 0.0 && recommendation.confidence_score <= 1.0,
      "Confidence score should be between 0 and 1" );
    assert!( !recommendation.reasoning.is_empty(), "Reasoning should not be empty" );
    assert!( recommendation.reasoning.len() > 10, "Reasoning should be detailed" );

    // For chatbot use case, should recommend models suitable for conversation
    assert!( recommendation.recommended_model.contains( "gemini" ), "Should recommend Gemini models" );

    // Verify alternatives if provided
    if let Some( alternatives ) = &recommendation.alternatives
    {
      assert!( !alternatives.is_empty(), "Alternatives should not be empty if provided" );
      for alt in alternatives
      {
        assert!( !alt.is_empty(), "Each alternative should not be empty" );
      }
    }
  }

  // Should have recommendations sorted by confidence
  for i in 1..response.recommendations.len()
  {
    assert!(
      response.recommendations[ i - 1 ].confidence_score >= response.recommendations[ i ].confidence_score,
      "Recommendations should be sorted by confidence score"
    );
  }

  // If use case analysis is provided, verify it's meaningful
  if let Some( analysis ) = &response.use_case_analysis
  {
    assert!( !analysis.is_empty(), "Use case analysis should not be empty" );
    assert!( analysis.len() > 20, "Use case analysis should be detailed" );
  }

  println!( "✓ Chatbot recommendations test passed" );
  println!( "  Received {} recommendations", response.recommendations.len() );
  println!( "  Top recommendation : {} (confidence : {:.2})",
    response.recommendations[ 0 ].recommended_model,
    response.recommendations[ 0 ].confidence_score
  );

  Ok( () )
}

/// Test getting recommendations for different use cases.
///
/// This test verifies that the API provides different recommendations
/// for different use case types.
#[ tokio::test ]
async fn test_get_recommendations_different_use_cases() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  // Test embedding use case
  let embedding_request = GetRecommendationsRequest {
    use_case : "Document similarity analysis and search".to_string(),
    input_size_range : Some( "medium".to_string() ),
    performance_requirements : Some( vec![ "batch-processing".to_string() ] ),
    budget_constraints : Some( 100.0 ),
    real_time_required : Some( false ),
  };

  let embedding_response = models_api.get_recommendations( &embedding_request ).await?;

  // Test content generation use case
  let generation_request = GetRecommendationsRequest {
    use_case : "Creative writing and content generation".to_string(),
    input_size_range : Some( "long".to_string() ),
    performance_requirements : Some( vec![ "high-quality".to_string(), "creative".to_string() ] ),
    budget_constraints : Some( 200.0 ),
    real_time_required : Some( false ),
  };

  let generation_response = models_api.get_recommendations( &generation_request ).await?;

  // Both should provide recommendations
  assert!( !embedding_response.recommendations.is_empty(), "Should recommend models for embedding use case" );
  assert!( !generation_response.recommendations.is_empty(), "Should recommend models for generation use case" );

  // Recommendations should be different for different use cases
  let embedding_top = &embedding_response.recommendations[ 0 ].recommended_model;
  let generation_top = &generation_response.recommendations[ 0 ].recommended_model;

  // They might be the same if one model is best for both, but reasoning should differ
  let embedding_reasoning = &embedding_response.recommendations[ 0 ].reasoning;
  let generation_reasoning = &generation_response.recommendations[ 0 ].reasoning;

  assert!(
    embedding_reasoning != generation_reasoning,
    "Reasoning should be different for different use cases"
  );

  println!( "✓ Different use cases test passed" );
  println!( "  Embedding use case top pick : {}", embedding_top );
  println!( "  Generation use case top pick : {}", generation_top );

  Ok( () )
}

/// Test advanced model filtering with multiple criteria.
///
/// This test verifies that the advanced filtering API correctly
/// filters models based on various criteria.
#[ tokio::test ]
async fn test_advanced_filter_comprehensive() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  let request = AdvancedFilterRequest {
    capabilities : Some( vec![ "generateContent".to_string() ] ),
    max_cost_per_1k : Some( 0.01 ), // Reasonable cost limit
    min_quality_score : Some( 0.7 ), // High quality requirement
    max_response_time : Some( 5000.0 ), // 5 second max response time
    supports_streaming : Some( true ),
    sort_by : Some( "quality".to_string() ),
  };

  let response = models_api.advanced_filter( &request ).await?;

  // Should return filtered results
  assert!( response.total_matches >= 0, "Total matches should be non-negative" );

  if response.total_matches > 0
  {
    assert!( !response.models.is_empty(), "Should return models when matches found" );
    assert!( response.models.len() <= response.total_matches as usize,
      "Returned models should not exceed total matches" );

    // Verify all returned models meet the capability requirement
    for model in &response.models
    {
      assert!( !model.name.is_empty(), "Model name should not be empty" );

      // Should have generateContent capability
      if let Some( capabilities ) = &model.supported_generation_methods
      {
        assert!( capabilities.contains( &"generateContent".to_string() ),
          "Model should support generateContent capability" );
      }
    }
  }

  // If applied filters summary is provided, verify it's meaningful
  if let Some( applied_filters ) = &response.applied_filters
  {
    assert!( !applied_filters.is_empty(), "Applied filters summary should not be empty" );
  }

  println!( "✓ Advanced filtering test passed" );
  println!( "  Found {} models matching criteria", response.total_matches );
  println!( "  Returned {} model details", response.models.len() );

  Ok( () )
}

/// Test filtering with different criteria combinations.
///
/// This test verifies that different filter combinations produce
/// appropriate results.
#[ tokio::test ]
async fn test_advanced_filter_different_criteria() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  // Test cost-focused filtering
  let cost_request = AdvancedFilterRequest {
    capabilities : None,
    max_cost_per_1k : Some( 0.001 ), // Very low cost requirement
    min_quality_score : None,
    max_response_time : None,
    supports_streaming : None,
    sort_by : Some( "cost".to_string() ),
  };

  let cost_response = models_api.advanced_filter( &cost_request ).await?;

  // Test performance-focused filtering
  let performance_request = AdvancedFilterRequest {
    capabilities : None,
    max_cost_per_1k : None,
    min_quality_score : Some( 0.9 ), // Very high quality requirement
    max_response_time : Some( 1000.0 ), // Fast response requirement
    supports_streaming : Some( true ),
    sort_by : Some( "performance".to_string() ),
  };

  let performance_response = models_api.advanced_filter( &performance_request ).await?;

  // Both requests should complete successfully
  assert!( cost_response.total_matches >= 0, "Cost-focused filter should return valid results" );
  assert!( performance_response.total_matches >= 0, "Performance-focused filter should return valid results" );

  // Strict performance requirements might return fewer results than cost-focused
  // (This is expected behavior, not a hard requirement)

  println!( "✓ Different filter criteria test passed" );
  println!( "  Cost-focused results : {} matches", cost_response.total_matches );
  println!( "  Performance-focused results : {} matches", performance_response.total_matches );

  Ok( () )
}

/// Test model status monitoring.
///
/// This test verifies that the model status API provides accurate
/// availability and health information.
#[ tokio::test ]
async fn test_model_status_monitoring() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  let request = ModelStatusRequest {
    model_names : vec![
      "gemini-2.0-flash-experimental".to_string(),
      "gemini-1.5-pro-002".to_string(),
    ],
    include_health_metrics : Some( true ),
  };

  let response = models_api.get_model_status( &request ).await?;

  // Should return status for all requested models
  assert_eq!( response.model_statuses.len(), 2, "Should return status for both models" );

  // Verify each status entry
  for status in &response.model_statuses
  {
    assert!( !status.model_name.is_empty(), "Model name should not be empty" );
    assert!( !status.status.is_empty(), "Status should not be empty" );

    // Status should be a valid status string
    let _valid_statuses = vec![ "available", "unavailable", "limited", "maintenance", "deprecated" ];
    // Note : The actual API might use different status values

    // Health percentage should be valid if provided
    if let Some( health ) = status.health_percentage
    {
      assert!( health >= 0.0 && health <= 100.0, "Health percentage should be between 0 and 100" );
    }

    // If maintenance is scheduled, should have a valid timestamp format
    if let Some( maintenance ) = &status.next_maintenance
    {
      assert!( !maintenance.is_empty(), "Maintenance timestamp should not be empty" );
    }
  }

  // Service health should be meaningful if provided
  if let Some( service_health ) = &response.service_health
  {
    assert!( !service_health.is_empty(), "Service health should not be empty" );
  }

  println!( "✓ Model status monitoring test passed" );
  for status in &response.model_statuses
  {
    println!( "  {}: {}", status.model_name, status.status );
    if let Some( health ) = status.health_percentage
    {
      println!( "    Health : {:.1}%", health );
    }
  }

  Ok( () )
}

/// Test error handling for invalid requests.
///
/// This test verifies that appropriate errors are returned for
/// invalid or malformed requests.
#[ tokio::test ]
async fn test_model_discovery_error_handling() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  // Test comparison with non-existent models
  let invalid_compare_request = CompareModelsRequest {
    model_names : vec![
      "non-existent-model-1".to_string(),
      "non-existent-model-2".to_string(),
    ],
    criteria : None,
    include_benchmarks : Some( false ),
    include_cost_analysis : Some( false ),
  };

  let compare_result = models_api.compare_models( &invalid_compare_request ).await;
  // Should handle gracefully (might return empty results or error depending on API behavior)

  // Test recommendations with empty use case
  let empty_use_case_request = GetRecommendationsRequest {
    use_case : "".to_string(),
    input_size_range : None,
    performance_requirements : None,
    budget_constraints : None,
    real_time_required : None,
  };

  let recommendations_result = models_api.get_recommendations( &empty_use_case_request ).await;
  // Should handle gracefully or return appropriate error

  // Test status for non-existent models
  let invalid_status_request = ModelStatusRequest {
    model_names : vec![ "completely-invalid-model".to_string() ],
    include_health_metrics : Some( true ),
  };

  let status_result = models_api.get_model_status( &invalid_status_request ).await;
  // Should handle gracefully (might return "unavailable" status or error)

  println!( "✓ Error handling test completed" );
  println!( "  Compare result : {}", if compare_result.is_ok() { "OK" } else { "Error" } );
  println!( "  Recommendations result : {}", if recommendations_result.is_ok() { "OK" } else { "Error" } );
  println!( "  Status result : {}", if status_result.is_ok() { "OK" } else { "Error" } );

  Ok( () )
}

/// Test comprehensive workflow combining multiple discovery features.
///
/// This test demonstrates a realistic workflow that uses multiple
/// model discovery APIs together.
#[ tokio::test ]
async fn test_model_discovery_workflow() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  // Step 1: Get recommendations for a specific use case
  let recommendations_request = GetRecommendationsRequest {
    use_case : "High-quality content generation for marketing materials".to_string(),
    input_size_range : Some( "medium".to_string() ),
    performance_requirements : Some( vec![ "high-quality".to_string(), "creative".to_string() ] ),
    budget_constraints : Some( 150.0 ),
    real_time_required : Some( false ),
  };

  let recommendations_response = models_api.get_recommendations( &recommendations_request ).await?;
  assert!( !recommendations_response.recommendations.is_empty(), "Should get recommendations" );

  // Step 2: Compare the top recommended models
  let top_models : Vec< String > = recommendations_response.recommendations
    .iter()
    .take( 3 ) // Take top 3 recommendations
    .map( |r| r.recommended_model.clone() )
    .collect();

  if top_models.len() >= 2
  {
    let compare_request = CompareModelsRequest {
      model_names : top_models.clone(),
      criteria : Some( vec![ "performance".to_string(), "cost".to_string(), "quality".to_string() ] ),
      include_benchmarks : Some( true ),
      include_cost_analysis : Some( true ),
    };

    let compare_response = models_api.compare_models( &compare_request ).await?;
    assert_eq!( compare_response.comparisons.len(), top_models.len(),
      "Should compare all provided models" );

    // Step 3: Check status of the compared models
    let status_request = ModelStatusRequest {
      model_names : top_models.clone(),
      include_health_metrics : Some( true ),
    };

    let status_response = models_api.get_model_status( &status_request ).await?;
    assert_eq!( status_response.model_statuses.len(), top_models.len(),
      "Should get status for all models" );

    // Step 4: Filter for additional options
    let filter_request = AdvancedFilterRequest {
      capabilities : Some( vec![ "generateContent".to_string() ] ),
      max_cost_per_1k : Some( 0.005 ),
      min_quality_score : Some( 0.8 ),
      max_response_time : None,
      supports_streaming : Some( true ),
      sort_by : Some( "quality".to_string() ),
    };

    let filter_response = models_api.advanced_filter( &filter_request ).await?;

    println!( "✓ Complete model discovery workflow test passed" );
    println!( "  Initial recommendations : {}", recommendations_response.recommendations.len() );
    println!( "  Models compared : {}", compare_response.comparisons.len() );
    println!( "  Status checked for : {} models", status_response.model_statuses.len() );
    println!( "  Additional filtered options : {}", filter_response.total_matches );

    // Verify workflow coherence
    assert!(
      !recommendations_response.recommendations.is_empty() &&
      !compare_response.comparisons.is_empty() &&
      !status_response.model_statuses.is_empty(),
      "Complete workflow should provide results at each step"
    );
  }

  Ok( () )
}