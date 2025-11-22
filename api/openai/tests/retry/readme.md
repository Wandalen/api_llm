# retry

Enhanced retry logic tests organized by functional domain.

## Purpose

Comprehensive test suite for the enhanced retry mechanism, validating retry behavior, backoff calculation, error classification, and state management.

## Organization

Tests are organized by functional area:

- `config_tests.rs` - Configuration validation and builder patterns (6 tests)
- `backoff_tests.rs` - Exponential backoff and delay calculation (3 tests)
- `error_classification_tests.rs` - Retryable vs non-retryable error handling (2 tests)
- `state_tests.rs` - State management and thread safety (3 tests)
- `executor_tests.rs` - Main executor behavior and retry orchestration (5 tests)
- `integration_tests.rs` - Integration and advanced scenarios (2 tests)
- `mod.rs` - Shared infrastructure (config, state, executor, test harness)

## Shared Infrastructure

All test files import shared infrastructure from `mod.rs`:
- `EnhancedRetryConfig` - Configuration structure
- `RetryState` - State management
- `EnhancedRetryExecutor` - Retry execution logic
- `MockHttpClient` - Test harness for controlled failure scenarios

## Feature Gating

All retry tests are feature-gated behind `#[cfg(feature = "retry")]` to ensure zero overhead when the feature is disabled.

## Test Count

Total: 21 tests across 6 files
- Configuration: 6 tests
- Backoff: 3 tests
- Error Classification: 2 tests
- State Management: 3 tests
- Executor Behavior: 5 tests
- Integration: 2 tests

## Running Tests

```bash
# Run all retry tests
cargo test --test retry --features retry

# Run specific test file
cargo test --test retry::config_tests --features retry

# Run without retry feature (validates zero overhead)
cargo test --test retry
```
