# Testing Strategy and Guidelines

## Testing Philosophy

**NO MOCKUP TESTS POLICY**: This crate follows a strict no-mockup policy for all testing:

- **Real Integration Tests Only**: All API functionality is tested against the actual Gemini API
- **No Mock Servers**: Tests use real HTTP calls to Google's production endpoints
- **No Mock Objects**: No synthetic test doubles or stub implementations
- **Explicit Failures**: Tests fail explicitly when API keys are unavailable (no silent mocking fallbacks)
- **Confidence in Reality**: Tests validate actual production behavior, not simulated responses

**Rationale**: Mockups hide integration failures, API changes, and real-world edge cases. Real API tests provide confidence that the client works in production environments.

## Integration Tests

**Integration tests are now enabled by default** and require a valid API key. They will fail explicitly if no token is available.

```bash
# Integration tests run by default - requires API key
cargo test

# To skip integration tests (run only unit tests)
cargo test --no-default-features
```

## Critical Integration Test Knowledge

**Important Timing Insights** (discovered through debugging infinite hangs):

- **Simple text generation**: ~0.5 seconds - fast and reliable
- **Safety settings requests**: ~15-17 seconds - significantly slower due to content analysis
- **Function calling**: ~2-4 seconds - moderate processing time
- **Multimodal requests**: ~3-8 seconds - varies by image complexity

## Test Timeout Strategy

```rust
use api_gemini::{ client::Client, models::* };

#[tokio::main]
async fn main() -> Result< (), Box< dyn std::error::Error > >
{
  let client = Client::new()?;
  let safety_request = GenerateContentRequest
  {
    contents: vec!
    [
      Content
      {
        parts: vec![ Part { text: Some( "Test content".to_string() ), ..Default::default() } ],
        role: "user".to_string(),
      }
    ],
    safety_settings: Some
    (
      vec!
      [
        SafetySetting
        {
          category: "HARM_CATEGORY_HARASSMENT".to_string(),
          threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
        }
      ]
    ),
    ..Default::default()
  };

  let result = tokio::time::timeout
  (
    std::time::Duration::from_secs( 25 ), // Accommodate safety processing
    client.models().by_name( "gemini-1.5-pro-latest" )
      .generate_content( &safety_request )
  ).await;

  Ok( () )
}
```

**Why different timeouts are needed:**
- Safety settings processing involves complex content analysis on Google's servers
- Using generic short timeouts (e.g., 10s) causes false test failures
- Client has 30s default timeout, but test-level timeouts provide better control

## Common Integration Test Pitfalls

**Don't do this:**
```rust
// Never skip failed tests - hides real issues
Err( _ ) =>
{
  println!( "Skipping due to timeout" );
  return; // BAD - makes tests unreliable
}
```

**Do this instead:**
```rust
// Fail clearly with actionable information
Err( _ ) => panic!( "API timeout after 25s - check network/quota" ),
```

**Other pitfalls to avoid:**
- **Environment race conditions**: Multiple tests modifying `GEMINI_API_KEY` simultaneously
- **Rate limiting**: Tests may fail during high API usage periods
- **Silent failures**: Integration tests now fail explicitly when no API key is available
- **Network dependencies**: Tests require internet connectivity

## Debugging Hanging Tests

1. **Isolate the issue**: Run single test with `--nocapture`
   ```bash
   cargo test --features integration test_safety_settings -- --nocapture
   ```

2. **Check API connectivity**: Test with faster endpoint first
   ```bash
   cargo test --features integration test_generate_content_simple
   ```

3. **Remove artificial timeouts**: Temporarily let client timeout (30s) reveal real error

4. **Verify API key**: Ensure key is valid and has remaining quota
   ```bash
   # Check if key works with simple request
   curl -H "Content-Type: application/json" \
        -H "x-goog-api-key: YOUR_KEY" \
        "https://generativelanguage.googleapis.com/v1beta/models"
   ```

## Test Environment Setup

**For CI/CD environments:**
```bash
# Set longer timeout for safety settings tests
export GEMINI_TEST_TIMEOUT=30

# Integration tests require API key (enabled by default)
if [ -z "$GEMINI_API_KEY" ]; then
    # Explicitly skip integration tests if no key available
    cargo test --no-default-features
    echo "Skipped integration tests - no GEMINI_API_KEY found"
else
    # Run full test suite including integration tests
    cargo test
    echo "All tests passed including integration tests"
fi
```

**For development:**
```bash
# Create secret/-secret.sh with your API key
echo 'GEMINI_API_KEY="your-key-here"' > secret/-secret.sh
chmod 600 secret/-secret.sh

# Run all tests including integration
cargo test --features integration
```
