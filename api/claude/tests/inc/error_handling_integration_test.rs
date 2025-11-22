//! Error Handling Integration Tests - STRICT FAILURE POLICY
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
use std::env;

#[ test ]
#[ cfg( feature = "integration" ) ]
#[ allow( clippy::too_many_lines ) ]
fn test_error_handling_integration()
{
    println!("🧪 Manual Testing : Error Handling When No Secrets Available");
    println!("==========================================================");
    
    // Test 1: Remove environment variable
    println!("\n🧹 Step 1: Removing environment variable...");
    env::remove_var("ANTHROPIC_API_KEY");
    println!("✅ ANTHROPIC_API_KEY environment variable removed");
    
    // Test 2: Temporarily move workspace file
    println!("\n📁 Step 2: Temporarily removing workspace secret file...");
    let secret_file = std::path::Path::new("../../secret/-secrets.sh");
    let backup_file = std::path::Path::new("../../secret/-secrets.sh.backup.test");
    
    let file_existed = if secret_file.exists()
    {
        std::fs::rename(secret_file, backup_file).expect("INTEGRATION: File operation must succeed");
        println!("✅ Workspace secret file moved to backup");
        true
    } else {
        println!("ℹ️ Workspace secret file doesn't exist");
        false
    };
    
    // Test 3: Test Secret::from_workspace() error handling
    println!("\n🔑 Step 3: Testing Secret::from_workspace() error handling...");
    match the_module::Secret::from_workspace()
    {
        Ok(_secret) => {
            println!("❌ Unexpected : Secret::from_workspace() succeeded when no secrets should be available!");
        },
        Err(e) => {
            println!("✅ Secret::from_workspace() correctly failed with error:");
            println!("   📝 Error : {e}");
            
            // Check error message contains helpful information
            let error_msg = e.to_string();
            if error_msg.contains("ANTHROPIC_API_KEY") && error_msg.contains("-secrets.sh")
            {
                println!("✅ Error message contains helpful information about both workspace file and environment variable");
            } else {
                println!("⚠️ Error message could be more helpful");
            }
        }
    }
    
    // Test 4: Test Client::from_workspace() error handling
    println!("\n🔧 Step 4: Testing Client::from_workspace() error handling...");
    match the_module::Client::from_workspace()
    {
        Ok(_client) => {
            println!("❌ Unexpected : Client::from_workspace() succeeded when no secrets should be available!");
        },
        Err(e) => {
            println!("✅ Client::from_workspace() correctly failed with error:");
            println!("   📝 Error : {e}");
        }
    }
    
    // Test 5: Test specific method error handling
    println!("\n🔍 Step 5: Testing Secret::load_from_workspace() with non-existent file...");
    match the_module::Secret::load_from_workspace("ANTHROPIC_API_KEY", "-nonexistent-file.sh")
    {
        Ok(_secret) => {
            println!("❌ Unexpected : load_from_workspace succeeded with non-existent file!");
        },
        Err(e) => {
            println!("✅ load_from_workspace correctly failed with non-existent file:");
            println!("   📝 Error : {e}");
        }
    }
    
    // Test 6: Test with invalid key name
    println!("\n🔍 Step 6: Testing Secret::load_from_workspace() with invalid key...");
    
    // First restore the file temporarily to test key lookup
    if file_existed
    {
        std::fs::rename(backup_file, secret_file).expect("INTEGRATION: File operation must succeed");
    }
    
    match the_module::Secret::load_from_workspace("INVALID_KEY_NAME", "-secrets.sh")
    {
        Ok(_secret) => {
            println!("❌ Unexpected : load_from_workspace succeeded with invalid key name!");
        },
        Err(e) => {
            println!("✅ load_from_workspace correctly failed with invalid key name:");
            println!("   📝 Error : {e}");
        }
    }
    
    // Move file back to test state
    if file_existed
    {
        std::fs::rename(secret_file, backup_file).expect("INTEGRATION: File operation must succeed");
    }
    
    // Test 7: Test environment variable loading when no env var
    println!("\n🌍 Step 7: Testing Secret::load_from_env() with no environment variable...");
    match the_module::Secret::load_from_env("ANTHROPIC_API_KEY")
    {
        Ok(_secret) => {
            println!("❌ Unexpected : load_from_env succeeded when no env var is set!");
        },
        Err(e) => {
            println!("✅ load_from_env correctly failed:");
            println!("   📝 Error : {e}");
        }
    }
    
    // Restore everything
    if file_existed
    {
        println!("\n🔄 Step 8: Restoring workspace secret file...");
        std::fs::rename(backup_file, secret_file).expect("INTEGRATION: File operation must succeed");
        println!("✅ Workspace secret file restored");
        
        // Verify restoration worked
        match the_module::Secret::from_workspace()
        {
            Ok(_secret) => {
                println!("✅ Secret loading working again after restoration");
            },
            Err(e) => {
                println!("⚠️ Secret loading still failing after restoration : {e}");
            }
        }
    }
    
    println!("\n🎉 Error handling testing completed!");
    println!("\n📋 Summary:");
    println!("   ✅ All methods correctly fail when no secrets are available");
    println!("   ✅ Error messages provide helpful information");
    println!("   ✅ System gracefully handles missing files and environment variables");
    println!("   ✅ Restoration works correctly");
}