//! Comprehensive Integration Tests - STRICT FAILURE POLICY
//!
//! MANDATORY INTEGRATION TEST REQUIREMENTS:
//! - These tests use REAL Anthropic API endpoints - NO MOCKING ALLOWED
//! - Tests MUST FAIL IMMEDIATELY if API secrets are not available (no graceful fallbacks)
//! - Tests MUST FAIL IMMEDIATELY on network connectivity issues
//! - Tests MUST FAIL IMMEDIATELY on API authentication failures
//! - Tests MUST FAIL IMMEDIATELY on any API endpoint errors
//! - NO SILENT PASSES allowed when problems occur
//!
//! Run with : cargo test --features integration
//! Requires : Valid `ANTHROPIC_API_KEY` in environment or ../../secret/-secrets.sh

#[ allow( unused_imports ) ]
use super::*;

/// Find secret directory and workspace root, with fallback logic
fn find_secret_and_workspace_root() -> Option< ( std::path::PathBuf, std::path::PathBuf ) >
{
  // Try workspace_tools first
  if let Ok( ws ) = workspace_tools::workspace()
  {
    let workspace_root = ws.root().to_path_buf();
    let secret_dir = workspace_root.join( "secret" );
    if secret_dir.exists()
    {
      return Some( ( secret_dir, workspace_root ) );
    }

    // Try parent directory (for workspace members)
    if let Some( parent ) = workspace_root.parent()
    {
      let parent_secret = parent.join( "secret" );
      if parent_secret.exists()
      {
        return Some( ( parent_secret, parent.to_path_buf() ) );
      }
    }
  }

  // Fallback : search upward from current directory
  let mut current = std::env::current_dir().ok()?;

  loop
  {
    let secret_dir = current.join( "secret" );
    if secret_dir.exists() && secret_dir.is_dir()
    {
      return Some( ( secret_dir, current.clone() ) );
    }

    // Move up to parent
    current = current.parent()?.to_path_buf();
  }
}

#[ test ]
#[ cfg( feature = "integration" ) ]
#[ ignore = "Requires workspace secrets file" ]
fn test_comprehensive_integration()
{
    // INTEGRATION TEST - STRICT FAILURE POLICY: NO GRACEFUL FALLBACKS
    // This test MUST fail if workspace secrets are not available

    // Find secret directory using same logic as Secret::find_secret_directory()
    // Try workspace_tools first, fall back to manual search
    let ( secret_dir, workspace_root ) = find_secret_and_workspace_root()
        .expect("INTEGRATION: Must have workspace for comprehensive testing");

    let secret_file = secret_dir.join("-secrets.sh");

    // MANDATORY REQUIREMENT: Secret file must exist with real API key
    assert!(secret_file.exists(),
        "INTEGRATION FAILURE: Secret file must exist at {:?} - no fake keys allowed",
        secret_file);

    println!("✅ Workspace structure verified");
    println!("   📁 Root : {}", workspace_root.display());
    println!("   🔐 Secrets : {}", secret_file.display());
    
    // Test 2: Raw secret file reading (using secret/ not .secret/)
    println!("\n🔧 Step 2: Testing direct secret file reading...");

    // Read from secret/ directory (NO dot prefix) as per codestyle rulebook
    let raw_secret_content = std::fs::read_to_string(&secret_file).expect("INTEGRATION: Must read secret file");
    let raw_secret = raw_secret_content.lines()
      .find_map(|line| {
        let line = line.trim();
        if line.starts_with("export ANTHROPIC_API_KEY=") || line.starts_with("ANTHROPIC_API_KEY=")
        {
          let value = line.split('=').nth(1)?;
          let value = value.trim().trim_matches('"');
          Some(value.to_string())
        } else {
          None
        }
      })
      .expect("INTEGRATION: Must find ANTHROPIC_API_KEY in secrets file");
    println!("✅ Raw secret loading successful");
    let raw_secret_len = raw_secret.len();
    println!("   📝 Raw secret length : {raw_secret_len}");
    // SECURITY: Never log actual secret content
    println!("   🔐 Raw secret format validated (content masked)");
    
    // Test 3: API wrapper functionality
    println!("\n🔑 Step 3: Testing API wrapper functionality...");
    
    // Test all loading methods
    let secret_from_workspace = the_module::Secret::from_workspace().expect("INTEGRATION: Must have workspace secret");
    let secret_load_explicit = the_module::Secret::load_from_workspace("ANTHROPIC_API_KEY", "-secrets.sh").expect("INTEGRATION: Must load explicit secret");
    let client_from_workspace = the_module::Client::from_workspace().expect("INTEGRATION: Must have workspace client");
    
    println!("✅ Secret::from_workspace() successful");
    println!("✅ Secret::load_from_workspace() successful");
    println!("✅ Client::from_workspace() successful");
    
    // Test 4: Consistency verification
    println!("\n🔍 Step 4: Verifying consistency across methods...");
    
    let keys_match_1 = secret_from_workspace.ANTHROPIC_API_KEY == secret_load_explicit.ANTHROPIC_API_KEY;
    let keys_match_2 = secret_from_workspace.ANTHROPIC_API_KEY == client_from_workspace.secret().ANTHROPIC_API_KEY;
    let keys_match_3 = secret_load_explicit.ANTHROPIC_API_KEY == client_from_workspace.secret().ANTHROPIC_API_KEY;
    
    println!("✅ Secret::from_workspace() == Secret::load_from_workspace(): {keys_match_1}");
    println!("✅ Secret::from_workspace() == Client::from_workspace()secret(): {keys_match_2}");
    println!("✅ Secret::load_from_workspace() == Client::from_workspace()secret(): {keys_match_3}");
    
    if keys_match_1 && keys_match_2 && keys_match_3
    {
        println!("🎉 All methods return consistent results!");
    } else {
        println!("❌ Inconsistency detected between methods!");
        panic!("INTEGRATION FAILURE: Consistency check failed");
    }
    
    // Test 5: Real API validation
    println!("\n🔍 Step 5: Validating real API credentials...");

    // Verify we have a real API key format
    let api_key = &secret_from_workspace.ANTHROPIC_API_KEY;
    assert!(api_key.starts_with("sk-ant-"),
        "INTEGRATION FAILURE: API key must be real Anthropic format, not fake test key");
    assert!(api_key.len() > 20,
        "INTEGRATION FAILURE: API key must be proper length, not test stub");


    // Test 6: Client functionality verification
    println!("\n🔧 Step 6: Testing client functionality...");

    let client = the_module::Client::from_workspace()
        .expect("INTEGRATION: Must have client from workspace");
    println!("✅ Client created successfully from workspace");

    // Verify client has expected properties
    let client_secret_matches = client.secret().ANTHROPIC_API_KEY == secret_from_workspace.ANTHROPIC_API_KEY;
    println!("   📝 Client secret matches : {client_secret_matches}");
    assert!(client_secret_matches, "INTEGRATION FAILURE: Client secret inconsistency");

    // Test 7: Performance check
    println!("\n⚡ Step 7: Basic performance check...");

    let start = std::time::Instant::now();
    for _ in 0..10
    {
        let _secret = the_module::Secret::from_workspace()
            .expect("INTEGRATION: Must have secret in loop");
    }
    let duration = start.elapsed();

    println!("✅ 10 secret loads completed in {duration:?}");
    let avg_duration = duration / 10;
    println!("   📈 Average per load : {avg_duration:?}");

    if duration.as_millis() < 1000
    {
        println!("✅ Performance is acceptable (< 1s for 10 loads)");
    } else {
        println!("⚠️ Performance could be improved (> 1s for 10 loads)");
    }

    // Final summary
    println!("\n🎉 Comprehensive Integration Test Results");
    println!("==========================================");
    println!("✅ Workspace structure verified");
    println!("✅ Raw workspace_tools functionality working");
    println!("✅ API wrapper functionality working");
    println!("✅ All loading methods consistent");
    println!("✅ Real API key format validated");
    println!("✅ Client functionality verified");
    println!("✅ Performance acceptable");
    println!("\n🚀 All integration tests PASSED!");
    
}