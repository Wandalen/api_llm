//! Test server management for `api_ollama` integration tests.
//!
//! This module provides automatic Ollama server lifecycle management for tests:
//! - Starts a dedicated test server with minimal model on unique port per binary
//! - Ensures test isolation by preventing cross-binary resource contamination
//! - Automatically pulls tinyllama model if not available
//! - Cleans up server after all tests complete
//! - Provides detailed error messages with resolution steps
//!
//! # Test Isolation Strategy (issue-server-exhaustion-001)
//!
//! Each test binary gets its own Ollama server instance on a unique port calculated
//! from the binary name hash. This prevents server resource exhaustion that occurred
//! when all tests shared port 11435, causing intermittent timeout failures after ~67 tests.
//!
//! Port range: 11435-11534 (100 ports for test binaries)
//! Formula: `BASE_PORT` + (`hash(binary_name)` % `PORT_RANGE`)

use std::process::{ Command, Stdio, Child };
use std::sync::{ Arc, Mutex, OnceLock };
use core::time::Duration;
use std::time::Instant;
use api_ollama::OllamaClient;
use std::collections::hash_map::DefaultHasher;
use core::hash::{ Hash, Hasher };

/// Global server instance shared across all tests in a single binary
static TEST_SERVER: OnceLock< Arc< Mutex< Option< TestServer > > > > = OnceLock::new();

/// Test server configuration
const BASE_PORT: u16 = 11435; // Base port for test servers
const PORT_RANGE: u16 = 100; // Allow 100 different test binaries (11435-11534)
// Optimization(phase3-model-switch): Changed from tinyllama to smollm2:360m for 23% speed improvement
// Root cause: Tinyllama (1.1B params) had extreme variability (350s-780s) and slow chat responses (avg 2220ms)
// Solution: Benchmark tested 4 models - smollm2:360m fastest (2024ms overall, 1730ms chat avg = 1.23x speedup)
// Benchmark results: smollm2:360m (2024ms), qwen2.5:0.5b (2267ms), tinyllama (2485ms), gemma3:1b (2672ms)
// Pitfall: Smaller model (360M vs 1.1B) may have different behavior - verify all tests still pass
const TEST_MODEL: &str = "smollm2:360m"; // Fastest model from benchmark (23% faster than tinyllama)
const SERVER_STARTUP_TIMEOUT: Duration = Duration::from_secs(30);
const MODEL_PULL_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes for model download
// Fix(issue-model-loading-timeout-001): Increased from 60s -> 180s to handle slow model loading
// Root cause: First inference request after server start takes 60+ seconds for model loading (smollm2:360m)
// Ollama loads models into RAM on first request, not during server startup - this is unavoidable overhead
// Pitfall: Timeout must exceed worst-case model load time (60-120s observed) plus inference time (10-30s)
const QUICK_RESPONSE_TIMEOUT: Duration = Duration::from_secs(180); // 3 minutes for model loading + inference

// Fix(issue-server-exhaustion-001): Unique port per test binary prevents server resource exhaustion
// Root cause: All tests shared port 11435, causing server exhaustion after ~67 tests with intermittent timeout failures
// Pitfall: Shared test infrastructure without isolation creates flaky tests that fail based on execution order, not code bugs

/// Calculate unique test port for this test binary based on binary name hash
///
/// This ensures each test binary gets its own isolated Ollama server instance,
/// preventing resource exhaustion and enabling parallel test execution.
///
/// # Returns
/// Port number in range [`BASE_PORT`, `BASE_PORT` + `PORT_RANGE`)
fn get_test_port() -> u16
{
  let binary_name = std::env::current_exe()
    .ok()
    .and_then( |path| path.file_name().map( |n| n.to_string_lossy().to_string() ) )
    .unwrap_or_else( || "default_test".to_string() );

  let mut hasher = DefaultHasher::new();
  binary_name.hash( &mut hasher );
  let hash = hasher.finish();

  // Cast is safe: modulo ensures result < PORT_RANGE (u16::MAX)
  #[ allow( clippy::cast_possible_truncation ) ]
  let offset = ( hash % u64::from( PORT_RANGE ) ) as u16;
  BASE_PORT + offset
}

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

    let test_port = get_test_port();
    println!( "🚀 Starting Ollama test server on port {test_port}..." );

    // Start Ollama server with custom port unique to this test binary
    // Resource limits prevent memory exhaustion when running many test binaries:
    // - OLLAMA_NUM_PARALLEL=1: Only process 1 request at a time
    // - OLLAMA_MAX_LOADED_MODELS=1: Only keep 1 model in memory
    // - OLLAMA_KEEP_ALIVE=0: Unload models immediately when idle
    let mut process = Command::new("ollama")
      .args(["serve"])
      .env("OLLAMA_HOST", format!( "127.0.0.1:{test_port}" ))
      .env("OLLAMA_NUM_PARALLEL", "1")
      .env("OLLAMA_MAX_LOADED_MODELS", "1")
      .env("OLLAMA_KEEP_ALIVE", "0")
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()
      .map_err(|e| format!(
        "Failed to start Ollama server : {e}\n\nResolution steps:\n1. Install Ollama : curl -fsSL https://ollama.ai/install.sh | sh\n2. Ensure Ollama is in PATH\n3. Run 'ollama --version' to verify installation"
      ) )?;

    // Extended timeout for integration tests: Ollama server can be slow under load (model processing, concurrent requests)
    // Fix(issue-builder-timeout-001): Increased from 300s -> 570s -> 650s -> 680s -> 720s -> 750s to handle extremely variable tinyllama responses
    // Root cause: Chat endpoint responses take 12+ minutes, extremely variable (350s-780s for identical requests)
    // Observed: test_builder_authentication_integration varies wildly (216s -> 678s -> 781s timeout) with no code changes
    // Tinyllama performance under concurrent load is unpredictable - some requests take 13+ minutes
    // Pitfall: Client timeout must be less than nextest slow-timeout (780s for builder tests, 600s for others) but generous for variable responses
    let mut client = OllamaClient::new( format!( "http://127.0.0.1:{test_port}" ), Duration::from_secs(750) ); // 12.5 minutes for extremely variable tinyllama responses

    // Wait for server to be ready
    let start_time = Instant::now();
    loop
    {
      if start_time.elapsed() > SERVER_STARTUP_TIMEOUT
      {
        let _ = process.kill();
        return Err( format!(
          "Ollama server failed to start within {timeout} seconds\n\nResolution steps:\n1. Check if port {port} is already in use\n2. Verify Ollama installation\n3. Check system resources (RAM/disk space)",
          timeout = SERVER_STARTUP_TIMEOUT.as_secs(),
          port = test_port
        ) );
      }

      if client.is_available().await
      {
        println!( "✅ Ollama test server ready on port {test_port}" );
        break;
      }

      tokio ::time::sleep(Duration::from_millis(500)).await;
    }

    let mut server = TestServer { process, port : test_port, client };

    // Ensure test model is available
    server.ensure_test_model_available().await?;

    // Fix(issue-resource-exhaustion-002): Removed test_quick_response() validation
    // Root cause: Model loading takes 60-180s on first inference, blocking server initialization
    // This caused resource exhaustion when multiple test binaries ran in parallel (each waited 60-180s)
    // Solution: Skip validation during init - actual tests will fail loudly if server doesn't work
    // Pitfall: First test per binary will be slower (expected - unavoidable model loading overhead)

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
    println!( "🔍 Checking if test model '{TEST_MODEL}' is available..." );
    
    // Check if model is already available
    match self.client.list_models().await
    {
      Ok(models) => 
      {
        if models.models.iter().any(|m| m.name.starts_with(TEST_MODEL))
        {
          println!( "✅ Test model '{TEST_MODEL}' already available" );
          return Ok(());
        }
      }
      Err(_) =>
      {
        return Err( format!(
          "Failed to communicate with test server\n\nResolution steps:\n1. Verify Ollama server is running\n2. Check network connectivity\n3. Ensure port {} is accessible",
          self.port
        ) );
      }
    }

    println!( "⬇️ Pulling test model '{TEST_MODEL}' (this may take several minutes)..." );

    // Pull the minimal test model
    let pull_start = Instant::now();
    let pull_result = Command::new("ollama")
      .args(["pull", TEST_MODEL])
      .env("OLLAMA_HOST", format!( "127.0.0.1:{}", self.port ))
      .output();
      
    match pull_result
    {
      Ok(output) if output.status.success() => 
      {
        println!( "✅ Test model '{TEST_MODEL}' pulled successfully in {:.1}s", pull_start.elapsed().as_secs_f64() );
      }
      Ok(output) =>
      {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err( format!(
          "Failed to pull test model '{TEST_MODEL}': {stderr}\n\nResolution steps:\n1. Check internet connectivity\n2. Verify Ollama registry access\n3. Ensure sufficient disk space\n4. Try manual pull : ollama pull {TEST_MODEL}"
        ) );
      }
      Err(e) =>
      {
        return Err( format!(
          "Failed to execute model pull : {e}\n\nResolution steps:\n1. Verify Ollama CLI is available\n2. Check PATH configuration\n3. Try manual pull : ollama pull {TEST_MODEL}"
        ) );
      }
    }
    
    if pull_start.elapsed() > MODEL_PULL_TIMEOUT
    {
      return Err( format!(
        "Model pull timed out after {timeout} seconds\n\nResolution steps:\n1. Check internet speed\n2. Retry with better connection\n3. Consider using cached model",
        timeout = MODEL_PULL_TIMEOUT.as_secs()
      ) );
    }
    
    // Verify model is now available
    match self.client.list_models().await
    {
      Ok(models) if models.models.iter().any(|m| m.name.starts_with(TEST_MODEL)) => 
      {
        println!( "✅ Test model '{TEST_MODEL}' verified and ready for testing" );
        Ok(())
      }
      _ => Err( format!(
        "Test model '{TEST_MODEL}' not found after pull\n\nResolution steps:\n1. Check Ollama model registry\n2. Verify model pull completed\n3. Try : ollama list"
      ) )
    }
  }
  
  /// Test if server can respond quickly to a simple request
  ///
  /// **DEPRECATED**: This validation is no longer used during server initialization.
  ///
  /// # Why Removed (issue-resource-exhaustion-002)
  ///
  /// This method was removed from server initialization because:
  /// - Model loading takes 60-180s on first inference request (unavoidable Ollama behavior)
  /// - Blocked `TestServer::start()` for 60-180s per test binary during validation
  /// - With multiple test binaries running in parallel, caused cumulative resource exhaustion
  /// - Redundant: Actual tests will fail immediately if server isn't working (satisfies "fail loudly" principle)
  ///
  /// # Current Strategy
  ///
  /// - Server availability validated by `is_available()` check (lightweight, fast)
  /// - Model loading happens during first actual test instead (no net time loss, just moved)
  /// - Tests fail loudly if server doesn't work - no silent failures
  ///
  /// Kept for reference and potential future use with different strategy.
  ///
  /// # Errors
  /// Returns an error if the server takes too long to respond
  #[allow(dead_code)]
  async fn test_quick_response(&mut self) -> Result< (), String >
  {
    println!( "🚀 Testing server quick response..." );
    
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
        println!( "✅ Server responding quickly ({:.2}s)", elapsed.as_secs_f64() );
        Ok(())
      }
      Ok(Err(e)) =>
      {
        let elapsed = start_time.elapsed();
        Err( format!( "Server failed to respond correctly ({:.2}s): {e}\n\nResolution steps:\n1. Check Ollama server logs\n2. Verify model is loaded correctly\n3. Check system resources (RAM/CPU)\n4. Try restarting Ollama server", elapsed.as_secs_f64() ) )
      }
      Err(_) =>
      {
        let elapsed = start_time.elapsed();
        Err( format!( "Server timed out after {:.2}s\n\nResolution steps:\n1. Check system resources (RAM/CPU)\n2. Try smaller model\n3. Increase QUICK_RESPONSE_TIMEOUT\n4. Check Ollama server logs", elapsed.as_secs_f64() ) )
      }
    }
  }
  
  /// Get client configured for this test server
  #[ must_use ]
  #[ allow( dead_code ) ]
  pub fn client(&self) -> &OllamaClient
  {
    &self.client
  }

  /// Get the test model name
  #[ must_use ]
  #[ allow( dead_code ) ]
  pub fn test_model() -> &'static str
  {
    TEST_MODEL
  }

  /// Get the port this test server is listening on
  #[ must_use ]
  #[ allow( dead_code ) ]
  pub fn port(&self) -> u16
  {
    self.port
  }
}

impl Drop for TestServer
{
  fn drop(&mut self)
  {
    let port = self.port;
    println!( "🛑 Shutting down Ollama test server on port {port}" );

    // Method 1: Try graceful kill via process handle
    let _ = self.process.kill();

    // Fix(issue-resource-exhaustion-002): Increased wait from 100ms -> 500ms for graceful shutdown
    // Root cause: Ollama needs time to flush model from RAM and release network sockets
    // Insufficient wait causes resource leaks when next test starts immediately
    // Pitfall: Too short delay leaves zombie processes and open ports
    std::thread::sleep(Duration::from_millis(500));

    // Method 2: Kill by port using lsof (finds processes listening on the port)
    let _ = Command::new("sh")
      .arg("-c")
      .arg( format!( "lsof -ti tcp:{port} 2>/dev/null | xargs -r kill -9 2>/dev/null || true" ) )
      .output();

    // Method 3: Kill by OLLAMA_HOST environment variable (catches the serve process)
    let _ = Command::new("pkill")
      .args(["-9", "-f", &format!( "OLLAMA_HOST=.*:{port}" )])
      .output();

    // Method 4: Kill any user-owned ollama runner processes
    // (Ollama spawns runner subprocesses that may outlive the serve process)
    let username = std::env::var("USER").unwrap_or_else(|_| "user1".to_string());
    let _ = Command::new("sh")
      .arg("-c")
      .arg( format!(
        "ps aux | grep '[o]llama' | grep '^{username}' | awk '{{print $2}}' | xargs -r kill -9 2>/dev/null || true"
      ) )
      .output();

    // Wait for processes to fully terminate and release resources
    std::thread::sleep(Duration::from_secs(1));

    // Final verification - wait for process handle
    let _ = self.process.wait();

    println!( "✅ Ollama server on port {port} cleanup completed" );
  }
}

/// Clean up any orphaned Ollama test servers from previous runs
///
/// When test processes are killed forcefully (e.g., nextest timeout with SIGKILL),
/// the Drop implementation doesn't run, leaving orphaned Ollama servers running.
/// This function kills any user-owned Ollama servers AND runner processes before starting new tests.
///
/// Fix(issue-runner-cleanup-001): Added cleanup for ollama runner processes
/// Root cause: Only killed 'ollama serve' but left 'ollama runner' processes (each consuming 920MB RAM + 5-10 CPU cores)
/// Runner processes accumulate during parallel test execution, causing resource exhaustion and network timeouts
/// Pitfall: Always kill BOTH serve and runner processes - runners are the actual resource hogs
fn cleanup_orphaned_servers()
{
  // Fix(issue-parallel-cleanup-001): Only clean up THIS test binary's port to avoid killing other parallel test servers
  // Root cause: cleanup_orphaned_servers() killed ALL test ports (11435-11534), including servers from other running test binaries
  // When test-threads=8, multiple test binaries run in parallel, each with unique port from hash(binary_name)
  // Binary A's cleanup at startup would kill Binary B's running server, causing "error sending request" failures
  // Pitfall: Over-aggressive cleanup in parallel environments creates race conditions between test binaries
  let test_port = get_test_port();
  println!( "🧹 Cleaning up orphaned Ollama server on port {test_port}..." );

  // Kill any process listening on THIS test binary's port only
  // This preserves servers from other test binaries running in parallel
  let port_cleanup = Command::new("sh")
    .arg("-c")
    .arg( format!( "lsof -ti tcp:{test_port} 2>/dev/null | xargs -r kill -9 2>/dev/null || true" ) )
    .output();

  // Report cleanup status
  match port_cleanup
  {
    Ok(output) if output.status.success() =>
    {
      println!( "✅ Cleaned up orphaned server on port {test_port}" );
    }
    _ =>
    {
      // Cleanup failure is non-fatal - server may not exist (expected on first run)
      println!( "✅ No orphaned server found on port {test_port}" );
    }
  }

  // Wait for port to be fully released before starting new server
  std::thread::sleep(Duration::from_millis(500));
}

/// Get or create the global test server instance
///
/// # Errors
/// Returns an error if the test server fails to start or initialize
pub async fn get_test_server() -> Result< Arc< Mutex< Option< TestServer > > >, String >
{
  // ALWAYS clean up orphaned servers at the start of EVERY test binary
  // This ensures cleanup happens even if previous test binaries crashed/timed out
  // Static variable ensures this only runs once per test binary process
  static CLEANUP_DONE: std::sync::Once = std::sync::Once::new();
  CLEANUP_DONE.call_once(|| {
    cleanup_orphaned_servers();
  });

  let server_arc = TEST_SERVER.get_or_init(|| Arc::new(Mutex::new(None))).clone();

  // Check if server needs to be initialized
  let needs_init = {
    let server_guard = server_arc.lock().map_err(|e| format!( "Failed to acquire test server mutex for initialization check : {e}" ))?;
    server_guard.is_none()
  };

  if needs_init
  {
    match TestServer::start().await
    {
      Ok(server) =>
      {
        let mut server_guard = server_arc.lock().map_err(|e| format!( "Failed to acquire test server mutex for initialization : {e}" ))?;
        *server_guard = Some(server);
        println!( "🎯 Test server initialized successfully" );
      }
      Err(e) =>
      {
        return Err( format!( "Failed to initialize test server : {e}" ) );
      }
    }
  }

  Ok(server_arc)
}

/// Test helper function to get a client for the managed test server
///
/// # Errors
/// Returns an error if the test server fails to start or initialize, or if mutex lock fails
#[ allow( dead_code ) ]
pub async fn get_test_client() -> Result< ( OllamaClient, String ), String >
{
  let server_arc = get_test_server().await?;
  let server_guard = server_arc.lock().map_err(|e| format!( "Failed to acquire test server mutex : {e}" ))?;
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
      Err( e ) =>
      {
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
  #[ allow( dead_code ) ]
  async fn test_server_lifecycle()
  {
    // Test that we can get a test server
    let result = get_test_client().await;

    // Skip gracefully if Ollama server is unavailable
    let (mut client, model) = match result
    {
      Ok(client_model) => client_model,
      Err(e) =>
      {
        println!( "⏭️  Skipping test - Ollama server unavailable: {e}" );
        return;
      }
    };

    // Test that server is responsive
    assert!(client.is_available().await, "Test server should be available");

    // Test that model is correct
    assert_eq!(model, TEST_MODEL, "Test model should be {TEST_MODEL}");

    println!( "✅ Test server lifecycle validated" );
  }
}
