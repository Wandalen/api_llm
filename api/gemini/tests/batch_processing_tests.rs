//! Comprehensive tests for Gemini API batch processing functionality.
//!
//! This module tests the batch processing APIs including:
//! - Batch content generation (`batchGenerateContent`)
//! - Batch embeddings (`batchEmbedContents`)
//! - Error handling and edge cases
//! - Performance characteristics
//!
//! **NOTE**: All tests in this file are currently disabled due to batch API endpoint issues.
//! See individual test permission comments for details.

// Import shared test utilities from common module
mod common;
use common::create_integration_client;

use api_gemini::models::*;

/// Test helper for creating sample content
fn create_sample_content( text : &str ) -> Content
{
  Content {
    role : "user".to_string(),
    parts : vec![
      Part {
        text : Some( text.to_string() ),
        inline_data : None,
        file_data : None,
        function_call : None,
        function_response : None,
        video_metadata : None,
      }
    ],
  }
}

/// Test helper for creating sample generate content request
fn create_sample_generate_request( text : &str ) -> GenerateContentRequest
{
  GenerateContentRequest {
    contents : vec![ create_sample_content( text ) ],
    tools : None,
    tool_config : None,
    safety_settings : None,
    system_instruction : None,
    generation_config : Some( GenerationConfig {
      temperature : Some( 0.7 ),
      top_p : Some( 0.8 ),
      top_k : Some( 40 ),
      candidate_count : Some( 1 ),
      max_output_tokens : Some( 100 ),
      stop_sequences : None,
    }),
    cached_content : None,
  }
}

/// Test helper for creating sample embed content request
fn create_sample_embed_request( text : &str ) -> EmbedContentRequest
{
  EmbedContentRequest {
    content : create_sample_content( text ),
    task_type : Some( "RETRIEVAL_DOCUMENT".to_string() ),
    title : Some( "Test Document".to_string() ),
    output_dimensionality : None,
  }
}

#[ tokio::test ]
async fn test_batch_generate_content_single_request() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  // Create a single request in batch
  let requests = vec![
    create_sample_generate_request( "Explain artificial intelligence in one sentence." )
  ];

  let batch_request = BatchGenerateContentRequest { requests };

  let response = models_api.batch_generate_content( "gemini-2.0-flash-experimental", &batch_request ).await?;

  // Verify response structure
  assert_eq!( response.responses.len(), 1, "Should return exactly one response" );

  let first_response = &response.responses[0];
  assert!( !first_response.candidates.is_empty(), "Should have at least one candidate" );

  let first_candidate = &first_response.candidates[0];
  // Content is now a direct struct, not an Option

  let content = &first_candidate.content;
  assert!( !content.parts.is_empty(), "Content should have parts" );

  let first_part = &content.parts[0];
  if let Some( ref text ) = first_part.text
  {
    assert!( !text.is_empty(), "Generated text should not be empty" );
    println!( "Generated text : {}", text );
  }

  println!( "✅ Single batch generate content request successful" );
  Ok( () )
}

#[ tokio::test ]
async fn test_batch_generate_content_multiple_requests() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  // Create multiple requests in batch
  let requests = vec![
    create_sample_generate_request( "What is machine learning?" ),
    create_sample_generate_request( "What is deep learning?" ),
    create_sample_generate_request( "What is natural language processing?" ),
  ];

  let batch_request = BatchGenerateContentRequest { requests };

  let response = models_api.batch_generate_content( "gemini-2.0-flash-experimental", &batch_request ).await?;

  // Verify response structure
  assert_eq!( response.responses.len(), 3, "Should return exactly three responses" );

  for ( i, single_response ) in response.responses.iter().enumerate()
  {
    assert!( !single_response.candidates.is_empty(), "Response {} should have at least one candidate", i );

    let first_candidate = &single_response.candidates[0];
    // Content is now a direct struct, not an Option

    let content = &first_candidate.content;
    assert!( !content.parts.is_empty(), "Response {} content should have parts", i );

    let first_part = &content.parts[0];
    if let Some( ref text ) = first_part.text
    {
      assert!( !text.is_empty(), "Response {} generated text should not be empty", i );
      println!( "Response {}: {}", i, text );
    }
  }

  println!( "✅ Multiple batch generate content requests successful" );
  Ok( () )
}


#[ tokio::test ]
async fn test_batch_embed_contents_single_request() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  // Create a single embedding request in batch
  let requests = vec![
    create_sample_embed_request( "Artificial intelligence is transforming various industries." )
  ];

  let batch_request = BatchEmbedContentsRequest { requests };

  let response = models_api.batch_embed_contents( "text-embedding-004", &batch_request ).await?;

  // Verify response structure
  assert_eq!( response.embeddings.len(), 1, "Should return exactly one embedding" );

  let first_embedding = &response.embeddings[0];
  assert!( !first_embedding.values.is_empty(), "Embedding should have values" );
  assert!( first_embedding.values.len() >= 100, "Embedding should have at least 100 dimensions" );

  // Verify all values are finite numbers
  for ( i, &value ) in first_embedding.values.iter().enumerate()
  {
    assert!( value.is_finite(), "Embedding value {} should be finite", i );
  }

  println!( "✅ Single batch embed content request successful, embedding has {} dimensions", first_embedding.values.len() );
  Ok( () )
}


#[ tokio::test ]
async fn test_batch_embed_contents_multiple_requests() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  // Create multiple embedding requests in batch
  let requests = vec![
    create_sample_embed_request( "Machine learning enables computers to learn from data." ),
    create_sample_embed_request( "Deep learning uses neural networks with multiple layers." ),
    create_sample_embed_request( "Natural language processing helps computers understand human language." ),
  ];

  let batch_request = BatchEmbedContentsRequest { requests };

  let response = models_api.batch_embed_contents( "text-embedding-004", &batch_request ).await?;

  // Verify response structure
  assert_eq!( response.embeddings.len(), 3, "Should return exactly three embeddings" );

  let mut embedding_dimensions = Vec::new();

  for ( i, embedding ) in response.embeddings.iter().enumerate()
  {
    assert!( !embedding.values.is_empty(), "Embedding {} should have values", i );
    assert!( embedding.values.len() >= 100, "Embedding {} should have at least 100 dimensions", i );

    embedding_dimensions.push( embedding.values.len() );

    // Verify all values are finite numbers
    for ( j, &value ) in embedding.values.iter().enumerate()
    {
      assert!( value.is_finite(), "Embedding {} value {} should be finite", i, j );
    }
  }

  // Verify all embeddings have the same dimensionality
  let first_dim = embedding_dimensions[0];
  for ( i, &dim ) in embedding_dimensions.iter().enumerate()
  {
    assert_eq!( dim, first_dim, "All embeddings should have the same dimensionality, but embedding {} has {} vs expected {}", i, dim, first_dim );
  }

  println!( "✅ Multiple batch embed content requests successful, {} embeddings with {} dimensions each", response.embeddings.len(), first_dim );
  Ok( () )
}


#[ tokio::test ]
async fn test_batch_generate_content_empty_request() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  // Create empty batch request
  let batch_request = BatchGenerateContentRequest { requests : vec![] };

  let result = models_api.batch_generate_content( "gemini-2.0-flash-experimental", &batch_request ).await;

  // Should handle empty request gracefully (either succeed with empty response or return appropriate error)
  match result
  {
    Ok( response ) =>
    {
      assert_eq!( response.responses.len(), 0, "Empty request should return empty response" );
      println!( "✅ Empty batch request handled gracefully with empty response" );
    },
    Err( e ) =>
    {
      println!( "✅ Empty batch request handled with error : {:?}", e );
      // This is also acceptable behavior
    }
  }

  Ok( () )
}


#[ tokio::test ]
async fn test_batch_embed_contents_empty_request() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  // Create empty batch request
  let batch_request = BatchEmbedContentsRequest { requests : vec![] };

  let result = models_api.batch_embed_contents( "text-embedding-004", &batch_request ).await;

  // Should handle empty request gracefully (either succeed with empty response or return appropriate error)
  match result
  {
    Ok( response ) =>
    {
      assert_eq!( response.embeddings.len(), 0, "Empty request should return empty response" );
      println!( "✅ Empty batch embedding request handled gracefully with empty response" );
    },
    Err( e ) =>
    {
      println!( "✅ Empty batch embedding request handled with error : {:?}", e );
      // This is also acceptable behavior
    }
  }

  Ok( () )
}


#[ tokio::test ]
async fn test_batch_generate_content_large_batch() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  // Create a larger batch (10 requests) to test scalability
  let requests : Vec< GenerateContentRequest > = ( 1..=10 )
    .map( |i| create_sample_generate_request( &format!( "Explain concept number {} in one sentence.", i ) ) )
    .collect();

  let batch_request = BatchGenerateContentRequest { requests };

  let response = models_api.batch_generate_content( "gemini-2.0-flash-experimental", &batch_request ).await?;

  // Verify response structure
  assert_eq!( response.responses.len(), 10, "Should return exactly ten responses" );

  let mut successful_responses = 0;

  for ( i, single_response ) in response.responses.iter().enumerate()
  {
    if !single_response.candidates.is_empty()
    {
      let first_candidate = &single_response.candidates[0];
      let content = &first_candidate.content;
      if !content.parts.is_empty()
      {
        if let Some( ref text ) = content.parts[0].text
        {
          if !text.is_empty()
          {
            successful_responses += 1;
            println!( "Response {}: {} chars", i, text.len() );
          }
        }
      }
    }
  }

  assert!( successful_responses >= 8, "At least 80% of responses should be successful, got {}/10", successful_responses );

  println!( "✅ Large batch generate content request successful ({}/10 responses)", successful_responses );
  Ok( () )
}


#[ tokio::test ]
async fn test_batch_embed_contents_large_batch() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  // Create a larger batch (10 requests) to test scalability
  let requests : Vec< EmbedContentRequest > = ( 1..=10 )
    .map( |i| create_sample_embed_request( &format!( "This is document number {} about various topics.", i ) ) )
    .collect();

  let batch_request = BatchEmbedContentsRequest { requests };

  let response = models_api.batch_embed_contents( "text-embedding-004", &batch_request ).await?;

  // Verify response structure
  assert_eq!( response.embeddings.len(), 10, "Should return exactly ten embeddings" );

  let mut successful_embeddings = 0;
  let mut embedding_dimensions = Vec::new();

  for ( i, embedding ) in response.embeddings.iter().enumerate()
  {
    if !embedding.values.is_empty() && embedding.values.iter().all( |&v| v.is_finite() )
    {
      successful_embeddings += 1;
      embedding_dimensions.push( embedding.values.len() );
      println!( "Embedding {}: {} dimensions", i, embedding.values.len() );
    }
  }

  assert!( successful_embeddings >= 8, "At least 80% of embeddings should be successful, got {}/10", successful_embeddings );

  // Verify dimensionality consistency
  if !embedding_dimensions.is_empty()
  {
    let first_dim = embedding_dimensions[0];
    for &dim in &embedding_dimensions
    {
      assert_eq!( dim, first_dim, "All embeddings should have consistent dimensionality" );
    }
  }

  println!( "✅ Large batch embed content request successful ({}/10 embeddings)", successful_embeddings );
  Ok( () )
}


#[ tokio::test ]
async fn test_batch_generate_content_invalid_model() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  // Create request with invalid model
  let requests = vec![
    create_sample_generate_request( "Test content" )
  ];

  let batch_request = BatchGenerateContentRequest { requests };

  let result = models_api.batch_generate_content( "invalid-model-name", &batch_request ).await;

  assert!( result.is_err(), "Should return error for invalid model" );

  if let Err( e ) = result
  {
    println!( "✅ Invalid model error handled correctly : {:?}", e );
  }

  Ok( () )
}


#[ tokio::test ]
async fn test_batch_embed_contents_invalid_model() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  // Create request with invalid model
  let requests = vec![
    create_sample_embed_request( "Test content" )
  ];

  let batch_request = BatchEmbedContentsRequest { requests };

  let result = models_api.batch_embed_contents( "invalid-embedding-model", &batch_request ).await;

  assert!( result.is_err(), "Should return error for invalid model" );

  if let Err( e ) = result
  {
    println!( "✅ Invalid embedding model error handled correctly : {:?}", e );
  }

  Ok( () )
}

#[ cfg( feature = "performance" ) ]

#[ tokio::test ]
async fn test_batch_vs_individual_performance() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let models_api = client.models();

  let test_texts = vec![
    "Machine learning fundamentals",
    "Deep learning concepts",
    "Natural language processing",
  ];

  // Measure individual requests
  let start_individual = std::time::Instant::now();
  let mut individual_responses = Vec::new();

  for text in &test_texts
  {
    let request = create_sample_embed_request( text );
    let response = models_api.by_name( "text-embedding-004" ).embed_content( &request ).await?;
    individual_responses.push( response );
  }

  let individual_duration = start_individual.elapsed();

  // Measure batch request
  let start_batch = std::time::Instant::now();

  let requests : Vec< EmbedContentRequest > = test_texts.iter()
    .map( |text| create_sample_embed_request( text ) )
    .collect();

  let batch_request = BatchEmbedContentsRequest { requests };
  let batch_response = models_api.batch_embed_contents( "text-embedding-004", &batch_request ).await?;

  let batch_duration = start_batch.elapsed();

  // Verify results are equivalent
  assert_eq!( individual_responses.len(), batch_response.embeddings.len() );

  // Batch should generally be faster or comparable
  let speedup_ratio = individual_duration.as_millis() as f64 / batch_duration.as_millis() as f64;

  println!( "Individual requests : {}ms", individual_duration.as_millis() );
  println!( "Batch request : {}ms", batch_duration.as_millis() );
  println!( "Speedup ratio : {:.2}x", speedup_ratio );

  // Batch should be at least as fast (or up to 20% slower due to overhead, which is acceptable)
  assert!( speedup_ratio >= 0.8, "Batch processing should not be significantly slower than individual requests" );

  println!( "✅ Batch processing performance validated" );
  Ok( () )
}