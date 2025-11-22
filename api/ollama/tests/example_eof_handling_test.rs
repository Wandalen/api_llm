//! Tests to verify examples handle EOF correctly without infinite loops.
//!
//! ## Bug Background (Issue #2)
//!
//! ### Root Cause
//! Interactive examples using `io::stdin().read_line()` were not properly handling
//! the EOF condition when running in non-interactive environments. When `read_line()`
//! returns `Ok(0)` (indicating EOF), the original code would print a message but
//! continue looping, causing infinite "You : " output.
//!
//! ### Why Not Caught
//! 1. Examples are typically tested manually in interactive terminals where EOF doesn't occur
//! 2. No automated tests existed for example executables
//! 3. CI environments dont run interactive examples by default
//!
//! ### Fix Applied
//! Modified all interactive examples to:
//! 1. Capture the return value from `read_line()` to check bytes read
//! 2. Explicitly check if `bytes_read == 0` (EOF condition)
//! 3. Print graceful exit message and **break** from the loop immediately
//!
//! ### Prevention
//! 1. Created automated tests that simulate EOF by providing empty stdin
//! 2. Tests verify examples exit gracefully without infinite loops
//! 3. Tests use timeouts to detect infinite loop conditions
//!
//! ### Pitfall
//! Interactive programs must always handle EOF explicitly. In Rust, `stdin().read_line()`
//! returns `Ok(0)` on EOF, not an error. Code that only checks for `Err` cases will
//! miss EOF and may loop infinitely. Always check the byte count returned : `if bytes_read == 0 { break; }`.

#[ cfg( feature = "enabled" ) ]
mod private
{
  use std::process::{ Command, Stdio };
  use core::time::Duration;

  /// Test that `ollama_chat_assistant` handles EOF gracefully without infinite loops.
  ///
  /// **Fix(issue-002)**: Added EOF detection to break from input loop immediately.
  /// **Root cause**: `stdin().read_line()` returns `Ok(0)` on EOF but code continued looping.
  /// **Pitfall**: Always check byte count from `read_line()`: `if bytes_read == 0 { break; }`.
  #[ tokio::test ]
  async fn test_chat_assistant_eof_handling()
  {
    let output = Command::new( "cargo" )
      .args( [ "run", "--all-features", "--example", "ollama_chat_assistant" ] )
      .stdin( Stdio::null() ) // Empty stdin to trigger EOF
      .stdout( Stdio::piped() )
      .stderr( Stdio::piped() )
      .output()
      .expect( "Failed to execute example" );

    let stdout = String::from_utf8_lossy( &output.stdout );

    // Verify example exits gracefully
    assert!( stdout.contains( "EOF" ) || stdout.contains( "No input available" ),
      "Example should detect EOF and exit gracefully. Got : {stdout}" );

    // Verify no infinite "You : " loop (should appear max 2 times : initial + after empty read)
    let you_count = stdout.matches( "You : " ).count();
    assert!( you_count <= 3,
      "Example appears to loop infinitely. 'You : ' appeared {you_count} times. Output : {stdout}" );
  }

  /// Test that `ollama_chat_interactive` handles EOF gracefully.
  ///
  /// **Fix(issue-002)**: Added EOF detection with immediate break.
  /// **Root cause**: Missing EOF check allowed infinite loop on empty stdin.
  /// **Pitfall**: In non-interactive mode, `read_line()` returns EOF immediately.
  #[ tokio::test ]
  async fn test_chat_interactive_eof_handling()
  {
    let output = Command::new( "cargo" )
      .args( [ "run", "--all-features", "--example", "ollama_chat_interactive" ] )
      .stdin( Stdio::null() )
      .stdout( Stdio::piped() )
      .stderr( Stdio::piped() )
      .output()
      .expect( "Failed to execute example" );

    let stdout = String::from_utf8_lossy( &output.stdout );

    assert!( stdout.contains( "EOF" ) || stdout.contains( "No input available" ),
      "Example should detect EOF. Got : {stdout}" );

    let you_count = stdout.matches( "You : " ).count();
    assert!( you_count <= 3,
      "Example appears to loop infinitely. 'You : ' appeared {you_count} times" );
  }

  /// Test that `ollama_chat_cached_interactive` handles EOF gracefully.
  ///
  /// **Fix(issue-002)**: Added EOF detection in interactive input loop.
  /// **Root cause**: EOF handling missing from input processing logic.
  /// **Pitfall**: Cached examples still need proper EOF handling despite caching layer.
  #[ tokio::test ]
  async fn test_cached_interactive_eof_handling()
  {
    let output = Command::new( "cargo" )
      .args( [ "run", "--all-features", "--example", "ollama_chat_cached_interactive" ] )
      .stdin( Stdio::null() )
      .stdout( Stdio::piped() )
      .stderr( Stdio::piped() )
      .output()
      .expect( "Failed to execute example" );

    let stdout = String::from_utf8_lossy( &output.stdout );

    assert!( stdout.contains( "EOF" ) || stdout.contains( "No input available" ),
      "Example should detect EOF. Got : {stdout}" );

    // This example uses "> " as prompt
    let prompt_count = stdout.matches( "> " ).count();
    assert!( prompt_count <= 3,
      "Example appears to loop infinitely. Prompt appeared {prompt_count} times" );
  }

  /// Test that `ollama_chat_streaming` handles EOF gracefully.
  ///
  /// **Fix(issue-002)**: Added EOF detection in streaming input loop.
  /// **Root cause**: Streaming feature didn't prevent EOF loop issue.
  /// **Pitfall**: Streaming examples need same EOF handling as non-streaming.
  #[ tokio::test ]
  async fn test_streaming_eof_handling()
  {
    let output = Command::new( "cargo" )
      .args( [ "run", "--all-features", "--example", "ollama_chat_streaming" ] )
      .stdin( Stdio::null() )
      .stdout( Stdio::piped() )
      .stderr( Stdio::piped() )
      .output()
      .expect( "Failed to execute example" );

    let stdout = String::from_utf8_lossy( &output.stdout );

    assert!( stdout.contains( "EOF" ) || stdout.contains( "No input available" ),
      "Example should detect EOF. Got : {stdout}" );

    let you_count = stdout.matches( "You : " ).count();
    assert!( you_count <= 3,
      "Example appears to loop infinitely. 'You : ' appeared {you_count} times" );
  }

  /// Test that `ollama_multimodal_vision` handles EOF gracefully.
  ///
  /// **Fix(issue-002)**: Added EOF detection in vision example's interactive mode.
  /// **Root cause**: Vision examples had same EOF handling gap as other interactive examples.
  /// **Pitfall**: Multimodal examples with file I/O still need proper stdin EOF handling.
  #[ tokio::test ]
  async fn test_multimodal_vision_eof_handling()
  {
    let output = Command::new( "cargo" )
      .args( [ "run", "--all-features", "--example", "ollama_multimodal_vision" ] )
      .stdin( Stdio::null() )
      .stdout( Stdio::piped() )
      .stderr( Stdio::piped() )
      .output()
      .expect( "Failed to execute example" );

    let stdout = String::from_utf8_lossy( &output.stdout );

    // This example may show EOF in interactive section
    // Verify it doesn't loop infinitely by checking prompt count
    let prompt_count = stdout.matches( "Enter your question" ).count();
    assert!( prompt_count <= 2,
      "Example appears to loop infinitely. Prompt appeared {prompt_count} times" );
  }

  /// Test that examples complete quickly when given EOF (dont hang).
  ///
  /// This test uses a timeout to verify that examples complete within a reasonable
  /// time when given EOF, rather than hanging indefinitely due to infinite loops.
  ///
  /// **Pitfall**: Tests need timeout protection to catch infinite loops in CI.
  #[ tokio::test ]
  async fn test_example_completes_quickly_on_eof()
  {
    let result = tokio::time::timeout(
      Duration::from_secs( 30 ), // Allow time for compilation + execution
      tokio ::task::spawn_blocking( ||
      {
        Command::new( "cargo" )
          .args( [ "run", "--all-features", "--example", "ollama_chat_assistant" ] )
          .stdin( Stdio::null() )
          .output()
          .expect( "Failed to execute" )
      })
    ).await;

    match result
    {
      Ok( Ok( output ) ) =>
      {
        // Success - example completed within timeout
        let stdout = String::from_utf8_lossy( &output.stdout );
        assert!( stdout.contains( "EOF" ) || stdout.contains( "No input available" ),
          "Example should detect EOF gracefully" );
      },
      Ok( Err( e ) ) => panic!( "Example execution failed : {e}" ),
      Err( e ) => panic!( "Example timed out after 30s - likely infinite loop bug not fixed : {e}" ),
    }
  }
}
