#!/usr/bin/env rust-script
//! Manual test for workspace_tools secret loading

use std::env;

fn main() -> Result<(), Box< dyn std::error::Error >>
{
    println!("🧪 Manual Testing: workspace_tools Secret Loading");
    println!("=================================================");
    
    // Test 1: Check current directory
    println!("\n📍 Current directory: {}", env::current_dir()?.display());
    
    // Test 2: Try to use workspace_tools directly
    println!("\n🔍 Testing workspace_tools directly...");
    match workspace_tools::workspace()
    {
        Ok(ws) => {
            println!("✅ Workspace found: {}", ws.root().display());
            
            // Test 3: Check if secret directory exists
            let secret_dir = ws.root().join("secret");
            println!("🔐 Secret directory: {}", secret_dir.display());
            println!("🔐 Secret dir exists: {}", secret_dir.exists());
            
            // Test 4: Try to load secret
            println!("\n🔑 Testing secret loading...");
            match ws.load_secret_key("ANTHROPIC_API_KEY", "-secrets.sh")
            {
                Ok(secret) => {
                    println!("✅ Secret loaded successfully!");
                    println!("📝 Secret starts with: {}...", &secret[..20]);
                    println!("📏 Secret length: {}", secret.len());
                },
                Err(e) => {
                    println!("❌ Secret loading failed: {}", e);
                }
            }
        },
        Err(e) => {
            println!("❌ Workspace not found: {}", e);
        }
    }
    
    // Test 5: Try environment variable fallback
    println!("\n🌍 Testing environment variable fallback...");
    match env::var("ANTHROPIC_API_KEY")
    {
        Ok(env_secret) => {
            println!("✅ Environment variable found!");
            println!("📝 Env secret starts with: {}...", &env_secret[..20]);
        },
        Err(_) => {
            println!("ℹ️ No environment variable set (expected)");
        }
    }
    
    Ok(())
}