# tests

This directory contains all automated tests for the api_huggingface crate.

## test organization

Tests are organized by functional domain:

**Core API Tests:**
- `client_tests.rs` - Client initialization and configuration
- `inference_tests.rs` - Text generation API tests
- `embeddings_tests.rs` - Embeddings API tests
- `models_tests.rs` - Model management tests
- `providers_api_tests.rs` - Pro plan providers API tests
- `streaming_tests.rs` - Streaming response handling
- `validation_tests.rs` - Input validation tests
- `error_handling_tests.rs` - Error handling tests
- `retry_tests.rs` - Retry logic tests
- `components_tests.rs` - Shared component tests

**Example-Based Integration Tests:**

Domain-specific integration tests validating complete workflows:
- `chatbot_example_test.rs` - Conversational chatbot functionality
- `document_search_example_test.rs` - Semantic search functionality
- `content_generator_example_test.rs` - Content generation functionality
- `code_assistant_example_test.rs` - Code assistance functionality
- `qa_system_example_test.rs` - Question-answering functionality
- `translation_example_test.rs` - Translation functionality
- `sentiment_analysis_example_test.rs` - Sentiment analysis functionality
- `ai_tutor_example_test.rs` - Educational tutoring functionality

**Debugging & Utilities:**
- `debug_validation.rs` - Debugging utilities for test validation

## running tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test client_tests

# Run with integration features
cargo test --features integration

# Run without integration tests (faster for development)
cargo test --no-default-features

# Run specific test by name
cargo test test_client_build_with_valid_environment
```

## test requirements

**Integration Tests:**
Integration tests require valid HuggingFace API credentials. Tests will fail loudly if credentials are missing.

Setup:
```bash
# Get your API key from https://huggingface.co/settings/tokens
echo 'HUGGINGFACE_API_KEY=your-key-here' > ../../secret/-huggingface.sh
```

## test strategy

**Real API Testing:**
All integration tests use real HuggingFace API endpoints with authentic credentials. No mocking is used to ensure tests validate actual API behavior, compatibility, and real-world error conditions.

**Loud Failures:**
Tests fail explicitly with clear error messages when issues occur. Silent passes are prohibited - every test must validate actual functionality.

**Domain-Based Organization:**
Tests are organized by what they test (domain/functionality) rather than how they test it (unit vs integration).

## test plan

See `test_plan.md` for comprehensive test strategy, coverage details, and testing methodology.
