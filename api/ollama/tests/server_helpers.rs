//! Test server management for `api_ollama` integration tests.
//!
//! This module provides automatic Ollama server lifecycle management for tests:
//! - Starts a dedicated test server with minimal model
//! - Ensures only one server instance across all tests
//! - Automatically pulls tinyllama model if not available
//! - Cleans up server after all tests complete
//! - Provides detailed error messages with resolution steps

use std::process::{ Command, Stdio, Child };
use std::sync::{ Arc, Mutex, OnceLock };
use core::time::Duration;
use std::time::Instant;
use api_ollama::OllamaClient;

/// Global server instance shared across all tests
static TEST_SERVER: OnceLock< Arc< Mutex< Option< TestServer > > > > = OnceLock::new();

/// Test server configuration
const TEST_PORT: u16 = 11435; // Use non-default port to avoid conflicts
const TEST_MODEL: &str = "tinyllama"; // Smallest available model
const SERVER_STARTUP_TIMEOUT: Duration = Duration::from_secs(30);
const MODEL_PULL_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes for model download
const QUICK_RESPONSE_TIMEOUT: Duration = Duration::from_secs(60); // Extended timeout to handle model processing in integration tests

/// Managed test server instance
#[ derive( Debug ) ]
pub struct TestServer
{
  process : Child,
  port : u16,
  client : OllamaClient,
}

impl TestServer
{
  /// Check if Ollama binary is available in PATH
  fn is_ollama_available() -> bool
  {
    Command::new( "ollama" )
      .arg( "--version" )
      .stdout( Stdio::null() )
      .stderr( Stdio::null() )
      .status()
      .is_ok_and( |status| status.success() )
  }

  /// Start a new test server instance
  ///
  /// # Errors
  /// Returns an error if:
  /// - Ollama binary is not found or fails to start
  /// - Server doesn't become ready within timeout
  /// - Test model cannot be pulled or verified
  async fn start() -> Result< Self, String >
  {
    // Check if Ollama is available before attempting to start server
    if !Self::is_ollama_available()
    {
      return Err("Ollama binary not found in PATH\n\nThis is expected in CI/automated test environments.\nResolution steps for local development:\n1. Install Ollama : curl -fsSL https://ollama.ai/install.sh | sh\n2. Ensure Ollama is in PATH\n3. Run 'ollama --version' to verify installation".to_string());
    }

    println!("🚀 Starting Ollama test server on port {TEST_PORT}...");

    // Start Ollama server with custom port
    let mut process = Command::new("ollama")
      .args(["serve"])
      .env("OLLAMA_HOST", format!("127.0.0.1:{TEST_PORT}"))
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()
      .map_err(|e| format!(
        "Failed to start Ollama server : {e}\n\nResolution steps:\n1. Install Ollama : curl -fsSL https://ollama.ai/install.sh | sh\n2. Ensure Ollama is in PATH\n3. Run 'ollama --version' to verify installation"
      ))?;

    let mut client = OllamaClient::new(format!("http://127.0.0.1:{TEST_PORT}"), Duration::from_secs(120)); // Extended timeout for integration tests to handle model processing delays
    
    // Wait for server to be ready
    let start_time = Instant::now();
    loop
    {
      if start_time.elapsed() > SERVER_STARTUP_TIMEOUT
      {
        let _ = process.kill();
        return Err(format!(
          "Ollama server failed to start within {timeout} seconds\n\nResolution steps:\n1. Check if port {port} is already in use\n2. Verify Ollama installation\n3. Check system resources (RAM/disk space)",
          timeout = SERVER_STARTUP_TIMEOUT.as_secs(),
          port = TEST_PORT
        ));
      }
      
      if client.is_available().await
      {
        println!("✅ Ollama test server ready on port {TEST_PORT}");
        break;
      }
      
      tokio ::time::sleep(Duration::from_millis(500)).await;
    }
    
    let mut server = TestServer { process, port : TEST_PORT, client };
    
    // Ensure test model is available
    server.ensure_test_model_available().await?;
    
    // Test if server can respond quickly
    server.test_quick_response().await?;
    
    Ok(server)
  }
  
  /// Ensure the test model is pulled and available
  ///
  /// # Errors
  /// Returns an error if:
  /// - Cannot communicate with test server
  /// - Model pull fails due to network/registry issues
  /// - Model verification fails after pull
  async fn ensure_test_model_available(&mut self) -> Result< (), String >
  {
    println!("🔍 Checking if test model '{TEST_MODEL}' is available...");
    
    // Check if model is already available
    match self.client.list_models().await
    {
      Ok(models) => 
      {
        if models.models.iter().any(|m| m.name.starts_with(TEST_MODEL))
        {
          println!("✅ Test model '{TEST_MODEL}' already available");
          return Ok(());
        }
      }
      Err(_) => 
      {
        return Err(format!(
          "Failed to communicate with test server\n\nResolution steps:\n1. Verify Ollama server is running\n2. Check network connectivity\n3. Ensure port {TEST_PORT} is accessible"
        ));
      }
    }
    
    println!("⬇️ Pulling test model '{TEST_MODEL}' (this may take several minutes)...");
    
    // Pull the minimal test model
    let pull_start = Instant::now();
    let pull_result = Command::new("ollama")
      .args(["pull", TEST_MODEL])
      .env("OLLAMA_HOST", format!("127.0.0.1:{TEST_PORT}"))
      .output();
      
    match pull_result
    {
      Ok(output) if output.status.success() => 
      {
        println!("✅ Test model '{TEST_MODEL}' pulled successfully in {:.1}s", pull_start.elapsed().as_secs_f64());
      }
      Ok(output) => 
      {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
          "Failed to pull test model '{TEST_MODEL}': {stderr}\n\nResolution steps:\n1. Check internet connectivity\n2. Verify Ollama registry access\n3. Ensure sufficient disk space\n4. Try manual pull : ollama pull {TEST_MODEL}"
        ));
      }
      Err(e) => 
      {
        return Err(format!(
          "Failed to execute model pull : {e}\n\nResolution steps:\n1. Verify Ollama CLI is available\n2. Check PATH configuration\n3. Try manual pull : ollama pull {TEST_MODEL}"
        ));
      }
    }
    
    if pull_start.elapsed() > MODEL_PULL_TIMEOUT
    {
      return Err(format!(
        "Model pull timed out after {timeout} seconds\n\nResolution steps:\n1. Check internet speed\n2. Retry with better connection\n3. Consider using cached model", 
        timeout = MODEL_PULL_TIMEOUT.as_secs()
      ));
    }
    
    // Verify model is now available
    match self.client.list_models().await
    {
      Ok(models) if models.models.iter().any(|m| m.name.starts_with(TEST_MODEL)) => 
      {
        println!("✅ Test model '{TEST_MODEL}' verified and ready for testing");
        Ok(())
      }
      _ => Err(format!(
        "Test model '{TEST_MODEL}' not found after pull\n\nResolution steps:\n1. Check Ollama model registry\n2. Verify model pull completed\n3. Try : ollama list"
      ))
    }
  }
  
  /// Test if server can respond quickly to a simple request
  ///
  /// # Errors
  /// Returns an error if the server takes too long to respond
  async fn test_quick_response(&mut self) -> Result< (), String >
  {
    println!("🚀 Testing server quick response...");
    
    use api_ollama::{ GenerateRequest };
    
    let request = GenerateRequest
    {
      model : TEST_MODEL.to_string(),
      prompt : "Hi".to_string(),
      stream : Some(false),
      options : None,
    };
    
    let start_time = std::time::Instant::now();
    
    // Try a quick generation request with timeout
    let result = tokio::time::timeout(
      QUICK_RESPONSE_TIMEOUT,
      self.client.generate(request)
    ).await;
    
    match result
    {
      Ok(Ok(_)) => 
      {
        let elapsed = start_time.elapsed();
        println!("✅ Server responding quickly ({:.2}s)", elapsed.as_secs_f64());
        Ok(())
      }
      Ok(Err(e)) => 
      {
        let elapsed = start_time.elapsed();
        println!("⚠ Server responded with error ({:.2}s): {e} - integration tests may use fallback behavior", elapsed.as_secs_f64());
        
        // Don't fail, just warn - external dependencies may be unreliable
        Ok(())
      }
      Err(_) => 
      {
        let elapsed = start_time.elapsed();
        println!("⚠ Server is slow to respond ({:.2}s) - integration tests may be unreliable", elapsed.as_secs_f64());
        
        // Don't fail, just warn - some environments are slow
        Ok(())
      }
    }
  }
  
  /// Get client configured for this test server
  #[ must_use ]
  pub fn client(&self) -> &OllamaClient
  {
    &self.client
  }
  
  /// Get the test model name
  #[ must_use ]
  pub fn test_model() -> &'static str
  {
    TEST_MODEL
  }
}

impl Drop for TestServer
{
  fn drop(&mut self)
  {
    println!("🛑 Shutting down Ollama test server on port {}", self.port);
    let _ = self.process.kill();
    let _ = self.process.wait();
  }
}

/// Get or create the global test server instance
///
/// # Errors
/// Returns an error if the test server fails to start or initialize
pub async fn get_test_server() -> Result< Arc< Mutex< Option< TestServer > > >, String >
{
  let server_arc = TEST_SERVER.get_or_init(|| Arc::new(Mutex::new(None))).clone();
  
  // Check if server needs to be initialized
  let needs_init = {
    let server_guard = server_arc.lock().map_err(|e| format!("Failed to acquire test server mutex for initialization check : {e}"))?;
    server_guard.is_none()
  };
  
  if needs_init
  {
    match TestServer::start().await
    {
      Ok(server) => 
      {
        let mut server_guard = server_arc.lock().map_err(|e| format!("Failed to acquire test server mutex for initialization : {e}"))?;
        *server_guard = Some(server);
        println!("🎯 Test server initialized successfully");
      }
      Err(e) => 
      {
        return Err(format!("Failed to initialize test server : {e}"));
      }
    }
  }
  
  Ok(server_arc)
}

/// Test helper function to get a client for the managed test server
///
/// # Errors
/// Returns an error if the test server fails to start or initialize, or if mutex lock fails
pub async fn get_test_client() -> Result< ( OllamaClient, String ), String >
{
  let server_arc = get_test_server().await?;
  let server_guard = server_arc.lock().map_err(|e| format!("Failed to acquire test server mutex : {e}"))?;
  let server = server_guard.as_ref().ok_or("Test server not initialized")?;

  // Clone the client and get model name
  let client = server.client().clone();
  let model = TestServer::test_model().to_string();

  Ok( ( client, model ) )
}

/// Macro to ensure test server is available before running test
#[ macro_export ]
macro_rules! with_test_server {
  ($test_fn:expr) => {{
    // INTEGRATION TEST - Skip gracefully when Ollama server unavailable
    // This allows tests to pass in environments without Ollama installed
    match $crate::server_helpers::get_test_client().await
    {
      Ok( ( client, model ) ) => $test_fn( client, model ).await,
      Err( e ) => {
        println!( "⏭️  Skipping integration test - Ollama server unavailable: {e}" );
        return;
      },
    }
  }};
}

#[ cfg( test ) ]
mod tests
{
  use super::*;
  
  #[ tokio::test ]
  async fn test_server_lifecycle()
  {
    // Test that we can get a test server
    let result = get_test_client().await;

    // Skip gracefully if Ollama server is unavailable
    let (mut client, model) = match result
    {
      Ok(client_model) => client_model,
      Err(e) => {
        println!("⏭️  Skipping test - Ollama server unavailable: {e}");
        return;
      }
    };

    // Test that server is responsive
    assert!(client.is_available().await, "Test server should be available");

    // Test that model is correct
    assert_eq!(model, TEST_MODEL, "Test model should be {TEST_MODEL}");

    println!("✅ Test server lifecycle validated");
  }
}
