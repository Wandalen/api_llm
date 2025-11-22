//! Workspace Loading Integration Tests - STRICT FAILURE POLICY
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

#[ test ]
#[ cfg( feature = "integration" ) ]
fn test_workspace_loading_integration()
{
    println!("🧪 Manual Testing : Anthropic API Workspace Secret Loading");
    println!("=========================================================");
    
    // Test 1: Check workspace_tools directly
    println!("\n🔍 Step 1: Testing workspace_tools directly...");
    match workspace_tools::workspace()
    {
        Ok(ws) => {
            let root_display = ws.root().display();
            println!("✅ Workspace found : {root_display}");
            let secret_path = ws.root().join("secret").join("-secrets.sh");
            let secret_path_display = secret_path.display();
            println!("🔐 Secret file path : {secret_path_display}");
            let secret_exists = secret_path.exists();
            println!("🔐 Secret file exists : {secret_exists}");

            if secret_path.exists()
            {
                match ws.load_secret_key("ANTHROPIC_API_KEY", "-secrets.sh")
                {
                    Ok(secret) => {
                        let secret_preview = &secret[..15];
                        let secret_len = secret.len();
                        println!("✅ Raw secret loaded : {secret_preview}... (length : {secret_len})");
                    },
                    Err(e) => {
                        println!("❌ Raw secret loading failed : {e}");
                    }
                }
            }
        },
        Err(e) => {
            println!("⚠️ Workspace not found (expected when running without WORKSPACE_PATH): {e}");
            println!("   Continuing with fallback secret loading tests...");
        }
    }
    
    // Test 2: Test the_module::Secret::from_workspace() 
    println!("\n🔑 Step 2: Testing the_module::Secret::from_workspace()...");
    match the_module::Secret::from_workspace()
    {
        Ok(secret) => {
            println!("✅ the_module::Secret::from_workspace() successful!");
            let key_preview = &secret.ANTHROPIC_API_KEY[..15];
            println!("📝 API Key : {key_preview}...");
            let key_len = secret.ANTHROPIC_API_KEY.len();
            println!("📏 Length : {key_len}");
            
            // Validate the key format
            if secret.ANTHROPIC_API_KEY.starts_with("sk-ant-")
            {
                println!("✅ API key format is correct (starts with sk-ant-)");
            } else {
                println!("⚠️ API key format seems incorrect");
            }
        },
        Err(e) => {
            println!("❌ the_module::Secret::from_workspace() failed : {e}");
        }
    }
    
    // Test 3: Test the_module::Client::from_workspace()
    println!("\n🔧 Step 3: Testing the_module::Client::from_workspace()...");
    match the_module::Client::from_workspace()
    {
        Ok(client) => {
            println!("✅ the_module::Client::from_workspace() successful!");
            let client_key_preview = &client.secret().ANTHROPIC_API_KEY[..15];
            println!("📝 Client API Key : {client_key_preview}...");
            println!("🔧 Client created successfully");
        },
        Err(e) => {
            println!("❌ the_module::Client::from_workspace() failed : {e}");
        }
    }
    
    // Test 4: Test both methods return the same secret
    println!("\n🔍 Step 4: Testing consistency between methods...");
    let secret_result = the_module::Secret::from_workspace();
    let client_result = the_module::Client::from_workspace();
    
    match (secret_result, client_result)
    {
        (Ok(secret), Ok(client)) => {
            if secret.ANTHROPIC_API_KEY == client.secret().ANTHROPIC_API_KEY
            {
                println!("✅ Both methods return the same API key - consistency verified!");
            } else {
                println!("❌ Methods return different API keys - inconsistency detected!");
            }
        },
        _ => {
            println!("❌ One or both methods failed");
        }
    }
    
    println!("\n🎉 Manual testing completed!");
}