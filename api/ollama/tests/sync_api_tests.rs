//! Synchronous API tests for `api_ollama`
//!
//! These tests verify the synchronous wrapper functionality
//! and blocking operations.

#![ cfg( feature = "sync_api" ) ]
#![ allow( clippy::std_instead_of_core ) ] // std required for sync operations

use api_ollama::{ OllamaClient, SyncOllamaClient, SyncApiConfig, SyncRuntimeManager };
use std::time::Duration;
use std::thread;

#[ test ]
fn test_sync_client_creation()
{
  let sync_client = SyncOllamaClient::new("http://localhost:11434", OllamaClient::recommended_timeout_fast()).unwrap();
  assert!(sync_client.base_url().contains("localhost"));
  assert_eq!(sync_client.timeout(), Duration::from_secs(30));
}

#[ test ]
fn test_sync_client_configuration()
{
  let config = SyncApiConfig::builder()
    .base_url("http://localhost:11434")
    .timeout(Duration::from_secs(60))
    .thread_pool_size(4)
    .enable_keepalive(true)
    .build()
    .unwrap();

  let sync_client = SyncOllamaClient::with_config(config).unwrap();
  assert_eq!(sync_client.timeout(), Duration::from_secs(60));
  assert_eq!(sync_client.thread_pool_size(), 4);
  assert!(sync_client.keepalive_enabled());
}

#[ test ]
fn test_sync_runtime_manager()
{
  let manager = SyncRuntimeManager::new(2);
  assert_eq!(manager.thread_count(), 2);
  assert!(manager.is_healthy());

  let handle = manager.spawn_blocking(|| {
    thread ::sleep(Duration::from_millis(100));
    42
  }).unwrap();

  let result = handle.join().unwrap();
  assert_eq!(result, 42);
}

/// Test sync client `list_models` operation.
///
/// **Fix(issue-missing-test-server-002)**: Converted to use isolated test server.
/// **Root cause**: Test connected to system Ollama (localhost:11434) causing fragile external dependency.
/// **Pitfall**: Sync tests must also use test server infrastructure for isolation.
///
/// **Fix(issue-sync-runtime-nesting-001)**: Separate async server setup from sync API call.
/// **Root cause**: Sync client's `block_on()` panics when called from within ANY async runtime context, even in spawned thread.
/// **Pitfall**: Must completely exit async context (drop runtime) before calling sync API - separate into two phases.
#[ test ]
fn test_sync_list_models()
{
  // Fix(issue-sync-runtime-nesting-001): Spawn thread and separate server setup from sync call
  // Phase 1: Use runtime in scoped block to get server URL, then drop runtime
  // Phase 2: Call sync API with NO active runtime
  let handle = std::thread::spawn(|| {
    // Import server helpers
    #[ path = "server_helpers.rs" ]
    mod server_helpers;
    use server_helpers::get_test_server;

    // Phase 1: Server setup in async context - runtime dropped after this block
    let server_url =
    {
      let rt = tokio::runtime::Runtime::new().unwrap();
      rt.block_on(async {
        let server_arc = get_test_server().await.expect("Failed to start test server");
        let server_guard = server_arc.lock().unwrap();
        let test_server = server_guard.as_ref().expect("Test server not initialized");
        let url = format!("http://127.0.0.1:{}", test_server.port());
        drop(server_guard);
        url
      })
      // Runtime is dropped here when exiting scope
    };

    // Phase 2: Call sync API OUTSIDE async context - no runtime active
    let mut sync_client = SyncOllamaClient::new(&server_url, OllamaClient::recommended_timeout_fast()).unwrap();

    let result = sync_client.list_models();
    assert!(result.is_ok(), "Sync list_models should succeed - network/timeout failures must fail test loudly");

    let response = result.unwrap();
    assert!(!response.models.is_empty(), "Should have at least one model (test model)");
  });

  handle.join().expect("Sync API test thread panicked");
}

#[ test ]
fn test_sync_with_timeout()
{
  let config = SyncApiConfig::builder()
    .base_url("http://localhost:11434")
    .timeout(Duration::from_millis(100))
    .build()
    .unwrap();

  let sync_client = SyncOllamaClient::with_config(config).unwrap();

  // Test that timeout is configured correctly
  assert_eq!(sync_client.timeout(), Duration::from_millis(100));
}

#[ test ]
fn test_sync_threading_safety()
{
  let sync_client = SyncOllamaClient::new("http://localhost:11434", OllamaClient::recommended_timeout_fast()).unwrap();
  let handles : Vec< _ > = ( 0..3 ).map( | _ | {
    let client = sync_client.clone();
    thread ::spawn(move || {
      // Just test the client creation and configuration in threads
      client.timeout()
    })
  }).collect();

  for handle in handles
  {
    let result = handle.join().unwrap();
    assert_eq!(result, Duration::from_secs(30));
  }
}

#[ test ]
fn test_sync_performance_benchmark()
{
  let sync_client = SyncOllamaClient::new("http://localhost:11434", OllamaClient::recommended_timeout_fast()).unwrap();
  let start = std::time::Instant::now();

  // Just test client creation performance
  for _ in 0..10
  {
    let _ = sync_client.timeout();
  }

  let duration = start.elapsed();
  assert!(duration < Duration::from_secs(1));
}

#[ test ]
fn test_sync_async_integration()
{
  let async_client = OllamaClient::new( "http://localhost:11434".to_string(), OllamaClient::recommended_timeout_fast() );
  let sync_client = SyncOllamaClient::from_async(async_client).unwrap();

  // Test that the sync client was created from async client
  assert!(sync_client.base_url().contains("localhost"));
}
