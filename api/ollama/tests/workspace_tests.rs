//! Workspace integration tests for `api_ollama`
//! 
//! These tests verify workspace integration functionality when the workspace feature is enabled.

#![ cfg( feature = "workspace" ) ]

use api_ollama::{ OllamaClient, WorkspaceConfig };
use std::env;
use std::fs;
use std::path::Path;

#[ tokio::test ]
async fn test_workspace_config_from_file()
{
  // Create a temporary workspace config
  let config_content = r#"
[ollama]
server_url = "http://workspace-ollama:11434"
default_model = "workspace-llama"
timeout_secs = 300
"#;

  let temp_dir = env::temp_dir();
  let config_path = temp_dir.join("test_workspace.toml");
  fs ::write(&config_path, config_content).expect("Failed to write test config");

  // Load workspace config
  let workspace_config = WorkspaceConfig::from_file(&config_path)
    .expect("Failed to load workspace config");

  assert_eq!(workspace_config.server_url(), "http://workspace-ollama:11434");
  assert_eq!(workspace_config.default_model().unwrap(), "workspace-llama");
  assert_eq!(workspace_config.timeout_secs(), Some(300));

  // Cleanup
  let _ = fs::remove_file(&config_path);
}

#[ tokio::test ]
async fn test_client_from_workspace()
{
  // Create workspace config
  let config_content = r#"
[ollama]
server_url = "http://test-workspace:9999"
timeout_secs = 60
"#;

  let temp_dir = env::temp_dir();
  let config_path = temp_dir.join("test_workspace2.toml");
  fs ::write(&config_path, config_content).expect("Failed to write test config");

  // Create client from workspace config
  let client = OllamaClient::from_workspace(&config_path)
    .expect("Failed to create client from workspace config");

  // Verify client configuration
  assert_eq!(client.base_url(), "http://test-workspace:9999");

  // Cleanup
  let _ = fs::remove_file(&config_path);
}

#[ tokio::test ]
async fn test_workspace_config_auto_discovery()
{
  // Create config in current directory
  let config_content = r#"
[ollama]
server_url = "http://auto-discovered:8888"
"#;

  let current_dir = env::current_dir().expect("Failed to get current dir");
  let config_path = current_dir.join("ollama.toml");
  fs ::write(&config_path, config_content).expect("Failed to write config");

  // Auto-discover workspace config
  let workspace_config = WorkspaceConfig::auto_discover()
    .expect("Failed to auto-discover workspace config");

  assert_eq!(workspace_config.server_url(), "http://auto-discovered:8888");

  // Cleanup
  let _ = fs::remove_file(&config_path);
}

#[ tokio::test ]
async fn test_client_from_auto_workspace()
{
  // Create client using auto-discovery
  let result = OllamaClient::from_auto_workspace();

  // Should either succeed with config found, or fail gracefully
  match result
  {
    Ok(_client) => {
      // Config was found and client created successfully
    },
    Err(e) => {
      // Should be a specific "no workspace config found" error
      assert!(e.to_string().contains("workspace") || e.to_string().contains("config"));
    }
  }
}

#[ tokio::test ]
async fn test_workspace_with_authentication()
{
  let config_content = r#"
[ollama]
server_url = "http://auth-workspace:11434"
api_key = "workspace-key-123"
"#;

  let temp_dir = env::temp_dir();
  let config_path = temp_dir.join("test_workspace_auth.toml");
  fs ::write(&config_path, config_content).expect("Failed to write test config");

  // Load workspace config to verify authentication data is parsed
  let workspace_config = WorkspaceConfig::from_file(&config_path)
    .expect("Failed to load workspace config with authentication");

  // Verify that API key is present in configuration
  assert_eq!(workspace_config.api_key().unwrap(), "workspace-key-123");

  // Create client with workspace authentication (should succeed even without auth implementation)
  let client = OllamaClient::from_workspace(&config_path)
    .expect("Failed to create authenticated workspace client");

  // Verify client was created with correct server URL
  assert_eq!(client.base_url(), "http://auth-workspace:11434");

  // Cleanup
  let _ = fs::remove_file(&config_path);
}

#[ tokio::test ] 
async fn test_workspace_fallback_gracefully()
{
  // Try to load non-existent config
  let non_existent = Path::new("/tmp/non-existent-workspace.toml");
  let result = WorkspaceConfig::from_file(non_existent);

  match result
  {
    Ok(_) => panic!("Should not succeed with non-existent file"),
    Err(e) => {
      // Should be a descriptive error about file not found
      assert!(e.to_string().contains("file") || e.to_string().contains("not found"));
    }
  }
}

#[ tokio::test ]
async fn test_workspace_model_preferences()
{
  let config_content = r#"
[ollama]
server_url = "http://localhost:11434"
default_model = "preferred-model"
models = ["model1", "model2", "preferred-model"]
"#;

  let temp_dir = env::temp_dir();
  let config_path = temp_dir.join("test_workspace_models.toml");
  fs ::write(&config_path, config_content).expect("Failed to write test config");

  let workspace_config = WorkspaceConfig::from_file(&config_path)
    .expect("Failed to load workspace config");

  assert_eq!(workspace_config.default_model().unwrap(), "preferred-model");
  assert_eq!(workspace_config.preferred_models().len(), 3);
  assert!(workspace_config.preferred_models().contains(&"model1".to_string()));

  // Cleanup
  let _ = fs::remove_file(&config_path);
}
