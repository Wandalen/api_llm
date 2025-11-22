//! Integration tests for synchronous streaming
//!
//! # Important Note on API Deprecation
//!
//! The sync streaming functionality uses the old `HuggingFace` Inference API
//! at `/models/{model}` which has been deprecated. `HuggingFace` now requires
//! using the Router API at `/v1/chat/completions` with streaming.
//!
//! Error message from `HuggingFace`:
//! "<https://api-inference.huggingface.co> is no longer supported.
//! Please use <https://router.huggingface.co/hf-inference> instead."
//!
//! The integration tests have been removed because they test functionality
//! that cannot work with the current `HuggingFace` infrastructure. To restore
//! them, `Inference::create_stream()` must be updated to use the Router API
//! chat completions streaming format instead of the deprecated Inference API.
//!
//! ## Required Changes for Full Streaming Support
//!
//! 1. Update `Inference::create_stream()` to construct chat completion requests
//! 2. Set `stream: true` in the request payload
//! 3. POST to `/v1/chat/completions` instead of `/models/{model}`
//! 4. Parse SSE stream for chat completion chunks (delta format)

#![ cfg( feature = "sync" ) ]

/// Compile-time test that `SyncStream` implements Iterator
#[ test ]
fn test_sync_stream_type_safety()
{
  fn assert_iterator< T : Iterator >() {}
  assert_iterator::< api_huggingface::sync::SyncStream >();
}
