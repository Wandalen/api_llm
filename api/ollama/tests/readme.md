# Tests

## Purpose

Comprehensive test suite for the Ollama API client, validating functionality, integration scenarios, error handling, and API compliance.

## Organization Principles

- **Domain-Based Organization**: Tests organized by functionality (what) not methodology (how)
- **Flat Structure**: All test files at top level for simplicity (~50 files)
- **Clear Naming**: Test files named after the functionality they test
- **Real API Testing**: All tests use real Ollama API integration (no mocking)
- **Feature Gating**: Tests requiring specific features use `#[cfg(feature = "...")]`

## Navigation Guide

- Circuit breaker functionality: `circuit_breaker_tests.rs`
- Integration scenarios: `integration_tests.rs`
- Builder patterns: `builder_patterns_tests.rs`
- Vision support: `vision_support_tests.rs`
- Tool calling: `tool_calling_tests.rs`
- Error handling: Files with `_tests.rs` suffix covering specific error scenarios

## Test Execution

```bash
# Run all tests
cargo test --all-features

# Run specific test file
cargo test --test integration_tests

# Run with real API (requires Ollama running)
OLLAMA_HOST=http://localhost:11434 cargo test --all-features
```
