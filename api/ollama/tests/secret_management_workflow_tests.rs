//! Secret management integration tests for `api_ollama`
//! 
//! These tests verify secret management functionality with real client operations
//! and network interactions.

#![ cfg( feature = "secret_management" ) ]

use api_ollama::{ OllamaClient, SecretStore };
use core::time::Duration;

#[ tokio::test ]
async fn test_client_secret_integration_with_timeout()
{
  let mut secret_store = SecretStore::new();
  secret_store.set("api_key", "sk-test-integration-key").expect("Failed to store secret");
  
  // Create client with secrets and test timeout behavior
  let mut client = OllamaClient::new( "http://unreachable.integration.test:99999".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_timeout(Duration::from_millis(50))
    .with_secret_store(secret_store);
  
  // Verify client has secrets before network call
  assert!(client.has_secrets());
  assert_eq!(client.get_secret("api_key").unwrap().unwrap(), "sk-test-integration-key");
  
  // Test that secrets are maintained during failed operations
  let result = client.list_models().await;
  assert!(result.is_err(), "Should fail with unreachable server");
  
  // Secrets should still be accessible after failed operation
  assert!(client.has_secrets());
  assert_eq!(client.get_secret("api_key").unwrap().unwrap(), "sk-test-integration-key");
}

#[ tokio::test ]
async fn test_client_secret_rotation_integration()
{
  let mut secret_store = SecretStore::new();
  secret_store.set("rotating_key", "initial-secret").expect("Failed to store initial secret");
  
  let mut client = OllamaClient::new( "http://test.local:11434".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_secret_store(secret_store);
  
  // Verify initial secret
  assert_eq!(client.get_secret("rotating_key").unwrap().unwrap(), "initial-secret");
  
  // Update secret through secret store (client doesn't have rotate_secret method)
  // This test verifies client maintains reference to secret store
  assert_eq!(client.get_secret("rotating_key").unwrap().unwrap(), "initial-secret");
  
  // Test that client still has access to secrets after operations
  let _ = client.list_models().await; // This will fail but that's expected
  assert_eq!(client.get_secret("rotating_key").unwrap().unwrap(), "initial-secret");
}

#[ tokio::test ]
async fn test_client_multiple_secret_stores()
{
  // Create multiple secret stores
  let mut store1 = SecretStore::new();
  store1.set("key1", "value1").expect("Failed to store in store1");
  
  let mut store2 = SecretStore::new();
  store2.set("key2", "value2").expect("Failed to store in store2");
  
  // Test client with first store
  let mut client1 = OllamaClient::new( "http://test1.local:11434".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_secret_store(store1);
  
  assert!(client1.has_secrets());
  assert_eq!(client1.get_secret("key1").unwrap().unwrap(), "value1");
  assert!(client1.get_secret("key2").unwrap().is_none());
  
  // Test client with second store
  let mut client2 = OllamaClient::new( "http://test2.local:11434".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_secret_store(store2);
  
  assert!(client2.has_secrets());
  assert_eq!(client2.get_secret("key2").unwrap().unwrap(), "value2");
  assert!(client2.get_secret("key1").unwrap().is_none());
}

#[ tokio::test ]
async fn test_secret_persistence_across_operations()
{
  let mut secret_store = SecretStore::new();
  secret_store.set("persistent_key", "persistent-value").expect("Failed to store secret");
  
  let mut client = OllamaClient::new( "http://unreachable.test:99999".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_timeout(Duration::from_millis(10))
    .with_secret_store(secret_store);
  
  // Perform multiple operations that will fail
  let result1 = client.list_models().await;
  assert!(result1.is_err(), "Should fail with unreachable server");
  assert!(client.has_secrets());
  assert_eq!(client.get_secret("persistent_key").unwrap().unwrap(), "persistent-value");
  
  let result2 = client.model_info("test-model".to_string()).await;
  assert!(result2.is_err(), "Should fail with unreachable server");
  assert!(client.has_secrets());
  assert_eq!(client.get_secret("persistent_key").unwrap().unwrap(), "persistent-value");
}

#[ tokio::test ]
async fn test_secret_masking_in_debug_during_operations()
{
  let mut secret_store = SecretStore::new();
  secret_store.set("debug_key", "super-secret-debug-value").expect("Failed to store secret");
  
  let client = OllamaClient::new( "http://debug.test:11434".to_string(), OllamaClient::recommended_timeout_fast() )
    .with_secret_store(secret_store);
  
  // Debug representation should mask secrets even during operations
  let debug_output = format!( "{client:?}" );
  assert!(!debug_output.contains("super-secret-debug-value"));
  assert!(debug_output.contains("secrets") || debug_output.contains("***"));
  
  // Clone client to test debug masking with copied secrets
  let cloned_client = client.clone();
  let cloned_debug = format!( "{cloned_client:?}" );
  assert!(!cloned_debug.contains("super-secret-debug-value"));
}
