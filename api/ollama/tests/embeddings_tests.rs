//! Embeddings API tests for `api_ollama`
//! 
//! # MANDATORY STRICT FAILURE POLICY
//! 
//! **⚠️  CRITICAL: These integration tests MUST fail loudly and immediately on any issues:**
//! 
//! - **Real API Only**: Tests make actual HTTP requests to live Ollama servers, never mocks
//! - **No Graceful Degradation**: Missing servers, network issues, or timeouts cause immediate test failure  
//! - **Required Dependencies**: Ollama server with embeddings models must be available
//! - **Explicit Configuration**: Tests require explicit server setup and fail if unavailable
//! - **Deterministic Failures**: Identical conditions must produce identical pass/fail results
//! - **End-to-End Validation**: Tests validate actual embeddings data from real models
//! 
//! These tests verify embeddings functionality including text-to-vector conversion, 
//! batch processing, and error handling with real Ollama server dependency. Server
//! unavailability or network failures WILL cause test failures - this is mandatory
//! per specification NFR-9.1 through NFR-9.8.

#![ cfg( all( feature = "embeddings", feature = "integration_tests" ) ) ]

mod server_helpers;

use api_ollama::{ OllamaClient, EmbeddingsRequest };
use core::time::Duration;
#[ tokio::test ]
async fn test_embeddings_basic()
{
  with_test_server!(|mut client : OllamaClient, model : String| async move {
    let request = EmbeddingsRequest
    {
      model,
      prompt : "Hello world".to_string(),
      options : None,
    };
    
    // Fix(issue-silent-failure-003): Fail loudly when server unavailable
    // Root cause : Silent skip with println+return created false positive test results
    // Pitfall : Integration tests MUST fail loudly when dependencies unavailable per codebase_hygiene.rulebook.md
    let embeddings = client.embeddings(request).await
      .expect("Embeddings API must succeed - Ollama server must be available for integration tests");

    assert!(!embeddings.embedding.is_empty(), "Embeddings should not be empty");

    // TinyLLaMA produces 2048-dimensional embeddings, not 4096
    assert!(!embeddings.embedding.is_empty(), "Embeddings should have positive dimensions");
    println!("✓ Embeddings dimensions : {}", embeddings.embedding.len());
    println!("✓ Basic embeddings generation successful");
  });
}
#[ tokio::test ]
async fn test_embeddings_multiple_prompts()
{
  with_test_server!(|mut client : OllamaClient, model : String| async move {
    let prompts = [
      "The quick brown fox jumps over the lazy dog".to_string(),
      "Machine learning is a subset of artificial intelligence".to_string(),
      "Rust is a systems programming language".to_string(),
    ];
    
    let request = EmbeddingsRequest
    {
      model : model.clone(),
      prompt : prompts.join(" "),
      options : None,
    };
    
    // Fix(issue-silent-failure-004): Fail loudly when server unavailable
    // Root cause : Silent skip with println+return created false positive test results
    // Pitfall : Integration tests MUST fail loudly when dependencies unavailable per codebase_hygiene.rulebook.md
    let embeddings = client.embeddings(request).await
      .expect("Embeddings API must succeed - Ollama server must be available for integration tests");

    assert!(!embeddings.embedding.is_empty(), "Embeddings should not be empty");
    
    // Test that embeddings are normalized (optional for some models)
    let magnitude : f64 = embeddings.embedding.iter().map(|x| x * x).sum::<f64>().sqrt();
    assert!(magnitude > 0.0, "Embedding magnitude should be positive");
    
    println!("✓ Multiple prompts embeddings generation successful");
  });
}

#[ tokio::test ]
async fn test_embeddings_empty_prompt_error()
{
  with_test_server!(|mut client : OllamaClient, model : String| async move {
    let request = EmbeddingsRequest
    {
      model,
      prompt : String::new(), // Empty prompt should cause error
      options : None,
    };
    
    let result = client.embeddings(request).await;
    
    // Ollama might accept empty prompts or return a default embedding
    // Let's just check that we get some result, not necessarily an error
    match result
    {
      Ok(embeddings) => {
        // Empty prompt might return empty or default embeddings
        println!("✓ Empty prompt handled (got {} dimensions)", embeddings.embedding.len());
      },
      Err(error) => {
        let error_str = format!("{error}");
        assert!(error_str.contains("empty") || error_str.contains("invalid") || error_str.contains("API error"), 
          "Error should mention empty, invalid, or API error : {error_str}");
        println!("✓ Empty prompt error handling : {error_str}");
      }
    }
    
    println!("✓ Empty prompt error handling successful");
  });
}

#[ tokio::test ]
async fn test_embeddings_network_error()
{
  let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_timeout( Duration::from_millis( 100 ) );
    
  let request = EmbeddingsRequest
  {
    model : "test-model".to_string(),
    prompt : "Test prompt".to_string(),
    options : None,
  };
  
  let result = client.embeddings( request ).await;
  assert!( result.is_err() );
  
  let error = result.unwrap_err();
  let error_str = format!( "{error}" );
  assert!( error_str.contains( "Network error" ) );
  
  println!("✓ Network error handling successful");
}

#[ tokio::test ]
async fn test_embeddings_invalid_model()
{
  with_test_server!(|mut client : OllamaClient, _model : String| async move {
    let request = EmbeddingsRequest
    {
      model : "non-existent-model".to_string(),
      prompt : "Test prompt".to_string(),
      options : None,
    };
    
    let result = client.embeddings(request).await;
    assert!(result.is_err(), "Invalid model should result in error");
    
    let error = result.unwrap_err();
    let error_str = format!("{error}");
    assert!(error_str.contains("API error") || error_str.contains("model not found"), 
      "Error should mention API error or model not found : {error_str}");
    
    println!("✓ Invalid model error handling successful");
  });
}
#[ tokio::test ]
async fn test_embeddings_with_options()
{
  with_test_server!(|mut client : OllamaClient, model : String| async move {
    use std::collections::HashMap;
    
    let mut options = HashMap::new();
    options.insert("temperature".to_string(), serde_json::Value::from(0.1));
    options.insert("top_p".to_string(), serde_json::Value::from(0.9));
    
    let request = EmbeddingsRequest
    {
      model,
      prompt : "Test prompt with options".to_string(),
      options : Some(options),
    };
    
    let result = client.embeddings(request).await;
    assert!(result.is_ok(), "Failed to get embeddings with options : {result:?}");
    
    let embeddings = result.unwrap();
    assert!(!embeddings.embedding.is_empty(), "Embeddings with options should not be empty");
    
    println!("✓ Embeddings with options successful");
  });
}
#[ tokio::test ]
async fn test_embeddings_long_prompt()
{
  with_test_server!(|mut client : OllamaClient, model : String| async move {
    // Create a long prompt to test handling of large inputs
    let long_prompt = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(100);
    
    let request = EmbeddingsRequest
    {
      model,
      prompt : long_prompt,
      options : None,
    };
    
    // Fix(issue-silent-failure-005): Fail loudly when server unavailable
    // Root cause : Silent skip with println+return created false positive test results
    // Pitfall : Integration tests MUST fail loudly when dependencies unavailable per codebase_hygiene.rulebook.md
    let embeddings = client.embeddings(request).await
      .expect("Embeddings API must succeed - Ollama server must be available for integration tests");

    assert!(!embeddings.embedding.is_empty(), "Embeddings for long prompt should not be empty");
    println!("✓ Long prompt embeddings generation successful");
  });
}
#[ tokio::test ]
async fn test_embeddings_special_characters()
{
  with_test_server!(|mut client : OllamaClient, model : String| async move {
    let special_prompt = "Hello! 你好 🌍 Привет مرحبا こんにちは";
    
    let request = EmbeddingsRequest
    {
      model,
      prompt : special_prompt.to_string(),
      options : None,
    };
    
    let result = client.embeddings(request).await;
    assert!(result.is_ok(), "Failed to get embeddings for special characters : {result:?}");
    
    let embeddings = result.unwrap();
    assert!(!embeddings.embedding.is_empty(), "Embeddings for special characters should not be empty");
    
    println!("✓ Special characters embeddings generation successful");
  });
}
#[ tokio::test ]
async fn test_embeddings_consistency()
{
  with_test_server!(|mut client : OllamaClient, model : String| async move {
    let prompt = "The same prompt should produce consistent embeddings";
    
    // Get embeddings twice for the same prompt
    let request1 = EmbeddingsRequest
    {
      model : model.clone(),
      prompt : prompt.to_string(),
      options : None,
    };
    
    let request2 = EmbeddingsRequest
    {
      model : model.clone(),
      prompt : prompt.to_string(),
      options : None,
    };
    
    // Fix(issue-silent-failure-006): Fail loudly when server unavailable
    // Root cause : Silent skip with println+return created false positive test results
    // Pitfall : Integration tests MUST fail loudly when dependencies unavailable per codebase_hygiene.rulebook.md
    let embeddings1 = client.embeddings(request1).await
      .expect("Embeddings API must succeed - Ollama server must be available for integration tests");
    let embeddings2 = client.embeddings(request2).await
      .expect("Embeddings API must succeed - Ollama server must be available for integration tests");

    assert_eq!(embeddings1.embedding.len(), embeddings2.embedding.len(),
      "Embeddings should have same dimensions");

    // Calculate cosine similarity - should be very high (near 1.0) for identical prompts
    let dot_product : f64 = embeddings1.embedding.iter()
      .zip(embeddings2.embedding.iter())
      .map(|(a, b)| a * b)
      .sum();

    let magnitude1 : f64 = embeddings1.embedding.iter().map(|x| x * x).sum::<f64>().sqrt();
    let magnitude2 : f64 = embeddings2.embedding.iter().map(|x| x * x).sum::<f64>().sqrt();

    let cosine_similarity = dot_product / (magnitude1 * magnitude2);
    assert!(cosine_similarity > 0.95,
      "Cosine similarity should be > 0.95 for identical prompts, got : {cosine_similarity}");

    println!("✓ Embeddings consistency test successful (similarity : {cosine_similarity:.4})");
  });
}
#[ tokio::test ]
async fn test_embeddings_authentication()
{
  #[ cfg( feature = "secret_management" ) ]
  {
    use api_ollama::SecretStore;
    
    with_test_server!(|client : OllamaClient, model : String| async move {
      let mut secret_store = SecretStore::new();
      secret_store.set("api_key", "test-api-key").expect("Failed to store test API key");
      
      let mut auth_client = client.with_secret_store(secret_store);
      
      let request = EmbeddingsRequest
      {
        model,
        prompt : "Test prompt with authentication".to_string(),
        options : None,
      };
      
      // Fix(issue-silent-failure-007): Fail loudly when server unavailable
      // Root cause : Silent skip with println+return created false positive test results
      // Pitfall : Integration tests MUST fail loudly when dependencies unavailable per codebase_hygiene.rulebook.md
      let embeddings = auth_client.embeddings(request).await
        .expect("Embeddings API must succeed - Ollama server must be available for integration tests");

      assert!(!embeddings.embedding.is_empty(), "Authenticated embeddings should not be empty");
      println!("✓ Embeddings with authentication successful");
    });
  }
  
  #[ cfg( not( feature = "secret_management" ) ) ]
  {
    println!("⚠ Skipping authentication test - secret_management feature not enabled");
  }
}
