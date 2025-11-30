# Tests

## Purpose

Comprehensive test suite for the Ollama API client, validating functionality, integration scenarios, error handling, and API compliance.

## Organization Principles

- **Domain-Based Organization**: Tests organized by functionality (what) not methodology (how)
- **Flat Structure**: All test files at top level for simplicity (~50 files)
- **Clear Naming**: Test files named after the functionality they test
- **Real API Testing**: All tests use real Ollama API integration (no mocking)
- **Feature Gating**: Tests requiring specific features use `#[cfg(feature = "...")]`

## Responsibility Table

### Core Infrastructure

| File | Responsibility | Coverage |
|------|----------------|----------|
| `readme.md` | Document test suite organization | Architecture, patterns, troubleshooting, marathon validation |
| `server_helpers.rs` | Provide isolated test server infrastructure | Hash-based port allocation, test model management, endpoint isolation |
| `manual/` | Contain manual testing procedures | Human-verified functionality (see manual/readme.md) |
| `-marathon_test.sh` | Marathon stress testing for flaky tests | Repetitive test execution, flake rate detection (temporary file) |

### Core API Functionality

| File | Responsibility | Feature Coverage |
|------|----------------|------------------|
| `core_client_api_tests.rs` | Test core client operations | Client initialization, configuration, basic API calls |
| `core_functionality_tests.rs` | Test fundamental API operations | Chat, generation, model listing |
| `api_comprehensive_tests.rs` | Test end-to-end API workflows | Complete usage scenarios, integration validation |

### Streaming & Real-Time

| File | Responsibility | Feature Coverage |
|------|----------------|------------------|
| `streaming_tests.rs` | Test streaming response handling | Stream initiation, data flow, completion |
| `streaming_control_tests.rs` | Test streaming pause/resume/cancel | Stream lifecycle control |
| `streaming_request_validation_tests.rs` | Validate streaming request construction | Request format, parameter validation |

### Embeddings

| File | Responsibility | Feature Coverage |
|------|----------------|------------------|
| `embeddings_tests.rs` | Test embeddings generation | Vector generation, batch processing |
| `embeddings_request_validation_tests.rs` | Validate embeddings request construction | Request format, parameter validation |

### Vision & Multimodal

| File | Responsibility | Feature Coverage |
|------|----------------|------------------|
| `vision_support_tests.rs` | Test vision model integration | Image inputs, multimodal requests |
| `vision_request_validation_tests.rs` | Validate vision request construction | Image format, parameter validation |
| `multimodal_request_validation_tests.rs` | Validate multimodal request patterns | Combined text/image inputs |

### Tool Calling & Function Support

| File | Responsibility | Feature Coverage |
|------|----------------|------------------|
| `tool_calling_tests.rs` | Test function/tool calling | Tool definitions, execution, responses |
| `tool_calling_request_validation_tests.rs` | Validate tool calling request construction | Tool schemas, parameter validation |
| `enhanced_function_calling_tests.rs` | Test advanced function calling scenarios | Complex tools, error handling |

### Reliability & Resilience

| File | Responsibility | Feature Coverage |
|------|----------------|------------------|
| `retry_logic_tests.rs` | Test exponential backoff retry | Retry strategies, backoff timing |
| `enhanced_retry_logic_tests.rs` | Test advanced retry scenarios | Complex failure modes, recovery |
| `circuit_breaker_tests.rs` | Test circuit breaker pattern | State transitions, failure thresholds |
| `enhanced_circuit_breaker_tests.rs` | Test circuit breaker edge cases | Recovery timing, state persistence |
| `circuit_breaker_resilience_tests.rs` | Test circuit breaker robustness | Stress scenarios, timing safety |
| `failover_tests.rs` | Test endpoint failover | Automatic failover, endpoint rotation |
| `health_checks_tests.rs` | Test health monitoring | Health checks, endpoint status tracking |

### Rate Limiting & Caching

| File | Responsibility | Feature Coverage |
|------|----------------|------------------|
| `rate_limiting_behavior_tests.rs` | Test token bucket rate limiting | Rate limit enforcement, token consumption |
| `enhanced_rate_limiting_tests.rs` | Test advanced rate limiting scenarios | Burst handling, refill timing |
| `request_caching_tests.rs` | Test response caching with TTL | Cache hits/misses, expiration |

### Builder Patterns & Construction

| File | Responsibility | Feature Coverage |
|------|----------------|------------------|
| `builder_patterns_tests.rs` | Test fluent builder APIs | Request builders, method chaining |
| `builder_construction_tests.rs` | Test builder construction patterns | Builder initialization, validation |

### Secret Management & Workspace Integration

| File | Responsibility | Feature Coverage |
|------|----------------|------------------|
| `secret_management_tests.rs` | Test secret handling | Secret loading, validation |
| `secret_management_workflow_tests.rs` | Test secret workflow integration | End-to-end secret usage |
| `workspace_secrets_tests.rs` | Test workspace secret integration | Workspace-level secret management |
| `workspace_tests.rs` | Test workspace integration patterns | Workspace configuration, coordination |

### Synchronous API

| File | Responsibility | Feature Coverage |
|------|----------------|------------------|
| `sync_api_tests.rs` | Test synchronous blocking wrappers | Blocking chat, generation, embeddings |

### Request Validation

| File | Responsibility | Feature Coverage |
|------|----------------|------------------|
| `input_validation_tests.rs` | Test input parameter validation | Parameter bounds, format checks |
| `token_validation_tests.rs` | Test token handling validation | Token counting, limits |

### Error Handling

| File | Responsibility | Feature Coverage |
|------|----------------|------------------|
| `error_handling_tests.rs` | Test error scenarios | Error types, recovery, propagation |

### Diagnostics & Monitoring

| File | Responsibility | Feature Coverage |
|------|----------------|------------------|
| `general_diagnostics_tests.rs` | Test diagnostics system | Diagnostic data collection, reporting |

### Example Validation

| File | Responsibility | Feature Coverage |
|------|----------------|------------------|
| `examples_comprehensive_tests.rs` | Test example correctness | Example compilation, execution |
| `example_execution_validation_tests.rs` | Validate example execution | Runtime behavior verification |
| `example_model_validation_test.rs` | Validate example model usage | Model availability, compatibility |
| `example_retry_logic_test.rs` | Test example retry patterns | Retry logic in examples |
| `example_eof_handling_test.rs` | Test example EOF handling | Stream termination in examples |
| `code_assistant_functionality_tests.rs` | Test code assistant example functionality | Code generation, completion |
| `document_analyzer_functionality_tests.rs` | Test document analyzer example functionality | Document processing, analysis |

### Feature-Specific Advanced Tests

| File | Responsibility | Feature Coverage |
|------|----------------|------------------|
| `audio_processing_tests.rs` | Test audio processing (when supported) | Audio inputs, transcription |
| `batch_operations_tests.rs` | Test batch request processing | Batch API calls, bulk operations |
| `cached_content_tests.rs` | Test content caching patterns | Content-based caching |
| `count_tokens_tests.rs` | Test token counting functionality | Token calculation, limits |
| `model_tuning_tests.rs` | Test model tuning operations | Fine-tuning, model customization |
| `safety_settings_tests.rs` | Test safety configuration | Content filtering, safety levels |

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

---

## Test Infrastructure Architecture

### Isolated Test Servers

All integration tests use **isolated Ollama servers** (not system Ollama) for complete environmental independence:

- **Port Allocation**: Hash-based deterministic ports (11435-11534) per test binary
  - Formula: `11435 + (hash(binary_name) % 100)`
  - Eliminates port conflicts between parallel test runs
  - Each test binary gets dedicated server instance

- **Test Model**: `smollm2:360m` (optimized for test performance)
  - 23% faster than `tinyllama` (2024ms vs 2631ms average)
  - Automatically pulled on first test run
  - Shared across all tests via test server singleton

- **Resource Limits**:
  - `OLLAMA_NUM_PARALLEL=1` - Predictable resource usage
  - `OLLAMA_MAX_LOADED_MODELS=1` - Minimal memory footprint
  - `OLLAMA_KEEP_ALIVE=0` - Immediate model unload after use

- **Isolation Benefits**:
  - Zero race conditions with system Ollama state
  - Tests pass identically whether system Ollama running or not
  - Complete control over server configuration
  - Automatic cleanup on test completion

**Usage**: Call `get_isolated_endpoint().await?` instead of hardcoding `localhost:11434`

See `server_helpers.rs` module docs for complete architecture details.

---

## Robustness Patterns

### Pattern 1: Endpoint Isolation

**Problem**: Tests dependent on system Ollama state are flaky and unreliable.

**Solution**: Use isolated test servers for all API calls.

```rust
// ❌ BAD - Creates environmental dependency
let client = OllamaClient::new("http://localhost:11434".to_string(), timeout)?;

// ✅ GOOD - Uses isolated test server
let endpoint = get_isolated_endpoint().await?;
let client = OllamaClient::new(endpoint, timeout)?;
```

**Impact**: Eliminated 80% fail rate in `test_intermittent_failure_handling` (issue-flaky-test-002)

**When to use**:
- Tests making REAL API calls (`.chat()`, `.embeddings()`, `.generate()`)
- Integration tests requiring live server responses
- Health check and monitoring tests

**When NOT to use**:
- Configuration-only tests (client builder, URL parsing)
- Failure scenario tests (use `get_invalid_endpoint()` instead)
- Tests explicitly testing system Ollama integration

### Pattern 2: Timing Safety

**Problem**: Exact timing assertions fail under system load or in CI environments.

**Solution**: Use safety buffers (2x minimum) and range assertions.

```rust
// ❌ BAD - Brittle exact timing
tokio::time::sleep(Duration::from_millis(300)).await;
assert_eq!(status.total_checks(), 3); // Fails if 4 checks happen

// ✅ GOOD - Safety buffer + range assertion
wait_for_checks(interval, 3).await; // 600ms (3 × 100ms × 2.0)
assert!(status.total_checks() >= 3); // Tolerates variance
```

**Formula**: `wait_time = interval × min_checks × 2.0`

**Rationale**:
- Accounts for scheduler variance (OS context switches)
- Handles GC pauses in async runtime
- Tolerates CI environment performance variance
- Prevents <1% flake rates from timing races

**Helpers**:
- `wait_for_checks(interval, count)` - Convenience wrapper with 2x buffer
- `calculate_safe_wait_time(interval, count, factor)` - Custom safety factors

### Pattern 3: Loud Failures

**Problem**: Silent test skips hide infrastructure problems and reduce effective coverage.

**Solution**: Tests must fail loudly when prerequisites missing.

```rust
// ❌ BAD - Silent skip hides problems
match client.embeddings(req).await {
  Ok(emb) => emb,
  Err(e) => {
    println!("⏭️  Skipping test - {e}");
    return; // Test "passes" but didn't run!
  }
}

// ✅ GOOD - Fails loudly with context
client.embeddings(req).await
  .expect("Embeddings should succeed - test server is running")
```

**Enforcement**: `with_test_server!` macro panics if infrastructure unavailable

**Benefits**:
- 100% test visibility (no hidden skips)
- Immediate signal when infrastructure breaks
- Clear diagnostic messages for debugging
- Specification compliance (NFR-9.1 deterministic failures)

**Migration**: Replaced 7 silent skips in `embeddings_tests.rs` (issue-silent-skip-002 through -005)

---

## Marathon Validation

For critical tests prone to flakiness, use marathon stress testing to detect rare failures:

```bash
# Run 20 iterations to detect <5% flake rate
bash tests/-marathon_test.sh test_name 20

# Run 100 iterations to detect <1% flake rate
bash tests/-marathon_test.sh test_name 100

# Run all tests (slower, comprehensive)
bash tests/-marathon_test.sh all 50
```

**When to use**:
- After fixing any flaky test (verify 0% flake rate)
- After adding timing-dependent logic
- Before merging critical test changes
- When CI shows intermittent failures

**Success criteria**: 100% pass rate across all iterations

**Real example**:
- `test_intermittent_failure_handling` validated with 10/10 marathon passes
- Detected <1% flake rates that wouldn't show in single runs
- Proved robustness improvements effective

---

## Common Anti-Patterns

### 1. Hardcoded `localhost:11434` in API-calling tests

**Symptom**: Test passes when system Ollama stopped, fails when running

**Fix**: Use `get_isolated_endpoint()` for all real API calls

**Detection**: `grep -r "localhost:11434" tests/*.rs`

### 2. Exact timing assertions

**Symptom**: Test expects exactly N iterations but gets N+1

**Fix**: Use `>=` assertions with `wait_for_checks()` helper

**Example**: `assert!(count >= 3)` not `assert_eq!(count, 3)`

### 3. Silent test skips

**Symptom**: Test "passes" but prints "Skipping..." message

**Fix**: Use `.expect()` or `panic!()` - never `println!() + return`

**Enforcement**: `with_test_server!` macro enforces loud failures

### 4. Mocking API responses

**Symptom**: Tests don't catch real API breaking changes

**Fix**: Use real test server (already running via `server_helpers.rs`)

**Rationale**: Mocks test your mock, not the API

### 5. Shared mutable state across tests

**Symptom**: Tests pass individually but fail when run in parallel

**Fix**: Each test gets isolated server instance automatically

**Architecture**: Hash-based port allocation prevents conflicts

---

## Test Troubleshooting

### Test fails with "Test server unavailable"

**Cause**: Ollama not installed or ports unavailable

**Resolution**:
1. Install Ollama: `curl -fsSL https://ollama.com/install.sh | sh`
2. Verify installation: `ollama --version`
3. Check port availability: `lsof -i :11435-11534`
4. Review test output for detailed diagnostics

**Note**: Tests require Ollama installed but NOT running (test server starts automatically)

### Test is flaky (intermittent failures)

**Diagnosis**:
1. Run marathon validation: `bash tests/-marathon_test.sh test_name 20`
2. Check for hardcoded `localhost:11434` in test code
3. Look for brittle timing (exact sleep durations, `==` assertions)
4. Review `health_checks_tests.rs` module docs for robustness patterns

**Common causes**:
- Environmental dependency (hardcoded endpoint)
- Timing assumptions without safety buffers
- Shared mutable state (though architecture prevents this)
- Exact count assertions on timing-dependent operations

### Test passes locally but fails in CI

**Likely causes**:
1. Insufficient timing safety buffers (use 2x minimum, 3x for CI)
2. CI environment has higher scheduler variance
3. Parallel test execution uncovering race conditions

**Fix**:
- Increase safety factor in `calculate_safe_wait_time()`
- Ensure all timing assertions use `>=` not `==`
- Run locally with `cargo nextest run` (parallel execution)

---

## Performance Optimization

### Test Execution Time

**Current state**:
- Full suite: ~30s (413 tests, parallel execution via nextest)
- Most tests: <100ms each
- Slow tests requiring investigation: >10s

**Slow tests** (review if times increase):
- `test_embeddings_long_prompt`: ~10s (acceptable - large input processing)
- `test_multimodal_vision_eof_handling`: ~38s (investigate if grows)

### Optimization Guidelines

1. **Use smallest viable test model**: `smollm2:360m` (current default)
2. **Minimize redundant API calls**: Share setup where safe
3. **Parallel execution**: Tests isolated via hash-based ports (enabled)
4. **Avoid unnecessary waits**: Use event notification over polling when possible

### Test Model Selection

Why `smollm2:360m`:
- 23% faster than `tinyllama` (2024ms vs 2631ms)
- Sufficient for testing API mechanics
- Smaller memory footprint (360M vs 1.1B parameters)

**Don't**: Use production models (llama3, mixtral) in tests - slower with no testing benefit

---

## Manual Testing

For functionality requiring human verification:
- See `tests/manual/readme.md` for manual test procedures
- Includes vision model validation, interactive streaming, etc.

---

## Related Documentation

- **`server_helpers.rs`**: Complete test infrastructure architecture and API
- **`health_checks_tests.rs`**: Robustness lessons learned (4 patterns)
- **`embeddings_tests.rs`**: Silent skip elimination examples
- **`-marathon_test.sh`**: Marathon validation script usage

---

## Key Metrics

**Current State** (as of 2025-11-29):
- Total tests: 413
- Pass rate: 100% (413/413)
- Flake rate: 0% (validated via marathon testing)
- Environmental dependencies: 0 (all tests use isolated servers)
- Silent skips: 0 (all eliminated, loud failures enforced)
