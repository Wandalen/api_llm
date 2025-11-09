//! Tests for example error handling patterns
//!
//! This test suite verifies that all examples follow proper error handling patterns
//! and dont use anti-patterns like `.expect()` for environment variable loading.
//!
//! ## Bug History
//!
//! ### Issue #1: Inconsistent Error Handling in API Key Loading (issue-manual-testing-001)
//!
//! **Root Cause:**
//! The `chat_cached_interactive.rs` example used `std::env::var().expect()` for API key
//! loading, which causes panic on missing environment variables instead of returning
//! a proper error. This is inconsistent with other examples that use either
//! `Secret::load_from_env()` or proper Result-based error handling.
//!
//! **Why Not Caught:**
//! - No automated test verifying error handling patterns in examples
//! - Examples were not systematically reviewed for consistency
//! - Clippy doesnt flag `.expect()` usage as an error by default
//!
//! **Fix Applied:**
//! Replaced:
//! ```rust
//! let api_key = std::env::var("HUGGINGFACE_API_KEY")
//!   .expect("HUGGINGFACE_API_KEY environment variable must be set");
//! let secret = Secret::new(api_key);
//! ```
//!
//! With:
//! ```rust
//! let api_key = Secret::load_from_env("HUGGINGFACE_API_KEY")?;
//! ```
//!
//! This change:
//! - Returns proper Result type instead of panicking
//! - Consistent with simple examples (chat.rs, `interactive_chat.rs`)
//! - Provides better error messages through `HuggingFaceError`
//! - Follows Rust best practices for error handling in examples
//!
//! **Prevention:**
//! - Added this test suite to verify no examples use `.expect()` for environment variables
//! - Added test to verify consistent secret loading patterns
//! - Added test to verify all examples return Result from main
//!
//! **Pitfall:**
//! Using `.expect()` in examples is particularly problematic because examples serve as
//! learning material and templates. Users copying this pattern would incorporate
//! poor error handling into their own code. Always use `?` operator or proper
//! error handling in examples to demonstrate best practices.

use std::{ fs, path::Path };

/// Test that no example uses .`expect()` for environment variable loading
///
/// This is an anti-pattern in examples as it demonstrates poor error handling.
#[ test ]
fn test_no_expect_for_env_vars()
{
  let examples_dir = Path::new( "examples" );
  assert!( examples_dir.exists(), "examples directory should exist" );

  let mut violations = Vec::new();

  for entry in fs::read_dir( examples_dir ).expect( "Failed to read examples directory" )
  {
  let entry = entry.expect( "Failed to read directory entry" );
  let path = entry.path();

  if path.extension().and_then( | s | s.to_str() ) == Some( "rs" )
  {
      let content = fs::read_to_string( &path ).expect( "Failed to read file" );
      let filename = path.file_name().expect( "[test_no_expect_for_env_vars] Path should have filename component - check examples directory structure" ).to_str().expect( "[test_no_expect_for_env_vars] Filename should be valid UTF-8 - check filesystem encoding" );

      // Check for std::env::var().expect() pattern
      if content.contains( "std::env::var" ) && content.contains( ".expect(" )
      {
  // More precise check : look for env::var followed by expect
  let lines : Vec< &str > = content.lines().collect();
  for ( i, line ) in lines.iter().enumerate()
  {
          if line.contains( "std::env::var" ) || line.contains( "env::var" )
          {
      // Check next few lines for .expect()
      let line_num = i + 1;
      for next_line in lines.iter().skip( i ).take( 5 )
      {
              if next_line.contains( ".expect(" )
              {
        violations.push( format!(
                  "{filename}:{line_num}: Found env::var with .expect() - use Secret::load_from_env() or proper error handling instead"
        ) );
        break;
              }
      }
          }
  }
      }
  }
  }

  assert!(violations.is_empty(),
      "Found {} examples using .expect() for environment variables:\n{}",
      violations.len(),
      violations.join( "\n" )
  );
}

/// Test that all examples return Result from main
///
/// Examples should demonstrate proper error handling by returning Result.
#[ test ]
fn test_examples_return_result()
{
  let examples_dir = Path::new( "examples" );
  assert!( examples_dir.exists(), "examples directory should exist" );

  let mut violations = Vec::new();

  for entry in fs::read_dir( examples_dir ).expect( "Failed to read examples directory" )
  {
  let entry = entry.expect( "Failed to read directory entry" );
  let path = entry.path();

  if path.extension().and_then( | s | s.to_str() ) == Some( "rs" )
  {
      let content = fs::read_to_string( &path ).expect( "Failed to read file" );
      let filename = path.file_name().expect( "[test_examples_return_result] Path should have filename component - check examples directory structure" ).to_str().expect( "[test_examples_return_result] Filename should be valid UTF-8 - check filesystem encoding" );

      // Check for async fn main() -> Result pattern
      let has_result_return = content.contains( "async fn main() -> Result< " )
  || content.contains( "fn main() - > Result< " );

      if !has_result_return
      {
  violations.push( format!(
          "{filename}: main() should return Result< (), Box< dyn std::error::Error > > for proper error handling"
  ) );
      }
  }
  }

  assert!(violations.is_empty(), 
      "Found {} examples not returning Result from main:\n{}",
      violations.len(),
      violations.join( "\n" )
  );
}

/// Test that examples use consistent secret loading patterns
///
/// Valid patterns:
/// 1. Simple : `Secret::load_from_env("HUGGINGFACE_API_KEY`")?
/// 2. Advanced : `std::env::var().or_else()` with `workspace_tools` fallback
///
/// Invalid patterns:
/// - `std::env::var().unwrap()`
/// - `std::env::var().expect()`
#[ test ]
fn test_consistent_secret_loading()
{
  let examples_dir = Path::new( "examples" );
  assert!( examples_dir.exists(), "examples directory should exist" );

  for entry in fs::read_dir( examples_dir ).expect( "Failed to read examples directory" )
  {
  let entry = entry.expect( "Failed to read directory entry" );
  let path = entry.path();

  if path.extension().and_then( | s | s.to_str() ) == Some( "rs" )
  {
      let content = fs::read_to_string( &path ).expect( "Failed to read file" );
      let filename = path.file_name().expect( "[test_consistent_secret_loading] Path should have filename component - check examples directory structure" ).to_str().expect( "[test_consistent_secret_loading] Filename should be valid UTF-8 - check filesystem encoding" );

      // If example uses environment variables for API key, verify the pattern
      if content.contains( "HUGGINGFACE_API_KEY" )
      {
  let uses_secret_load = content.contains( "Secret::load_from_env" );
  let uses_workspace_fallback = content.contains( "workspace_tools" )
          && content.contains( ".or_else(" );
  let uses_unwrap = content.contains( "std::env::var" )
          && ( content.contains( ".unwrap()" ) || content.contains( ".expect(" ) );

  // Must use one of the valid patterns
  let uses_valid_pattern = uses_secret_load || uses_workspace_fallback;

  assert!(
          !uses_unwrap || uses_workspace_fallback,
          "{filename}: Uses .unwrap() or .expect() without proper error handling. \
           Use Secret::load_from_env() or or_else() pattern instead."
  );

  assert!(
          uses_valid_pattern,
          "{filename}: Should use either Secret::load_from_env() or workspace_tools fallback pattern"
  );
      }
  }
  }
}

/// Test that example names in Cargo.toml match actual files
///
/// This verifies that all example files are properly registered and runnable.
#[ test ]
fn test_examples_registered_in_cargo_toml()
{
  let examples_dir = Path::new( "examples" );
  let cargo_toml = fs::read_to_string( "Cargo.toml" ).expect( "Failed to read Cargo.toml" );

  let mut unregistered = Vec::new();

  for entry in fs::read_dir( examples_dir ).expect( "Failed to read examples directory" )
  {
  let entry = entry.expect( "Failed to read directory entry" );
  let path = entry.path();

  if path.extension().and_then( | s | s.to_str() ) == Some( "rs" )
  {
      let filename = path.file_name().expect( "[test_examples_registered_in_cargo_toml] Path should have filename component - check examples directory structure" ).to_str().expect( "[test_examples_registered_in_cargo_toml] Filename should be valid UTF-8 - check filesystem encoding" );
      let expected_path_in_toml = format!( "examples/{filename}" );

      if !cargo_toml.contains( &expected_path_in_toml )
      {
  unregistered.push( filename.to_string() );
      }
  }
  }

  assert!(unregistered.is_empty(), 
      "Found {} examples not registered in Cargo.toml:\n{}\n\n\
       Add [[example]] entries for these files.",
      unregistered.len(),
      unregistered.join( "\n" )
  );
}

/// Test that no examples use hardcoded secrets
///
/// Examples should always load secrets from environment, never hardcode them.
#[ test ]
fn test_no_hardcoded_secrets()
{
  let examples_dir = Path::new( "examples" );
  let mut violations = Vec::new();

  for entry in fs::read_dir( examples_dir ).expect( "Failed to read examples directory" )
  {
  let entry = entry.expect( "Failed to read directory entry" );
  let path = entry.path();

  if path.extension().and_then( | s | s.to_str() ) == Some( "rs" )
  {
      let content = fs::read_to_string( &path ).expect( "Failed to read file" );
      let filename = path.file_name().expect( "[test_no_hardcoded_secrets] Path should have filename component - check examples directory structure" ).to_str().expect( "[test_no_hardcoded_secrets] Filename should be valid UTF-8 - check filesystem encoding" );

      // Look for patterns that might indicate hardcoded API keys
      let suspicious_patterns = [
  r#"api_key = "hf_"#,
  r#"api_key = "sk_"#,
  r#"HUGGINGFACE_API_KEY" => "#,
  r#"Secret::new("hf_"#,
      ];

      for pattern in &suspicious_patterns
      {
  if content.contains( pattern )
  {
          violations.push( format!(
      "{filename}: Possible hardcoded secret found (pattern : {pattern})"
          ) );
  }
      }
  }
  }

  assert!(violations.is_empty(), 
      "Found potential hardcoded secrets in examples:\n{}",
      violations.join( "\n" )
  );
}
