# Manual Testing

Manual testing procedures and scripts for api_openai.

## Purpose

Provides manual testing scripts for scenarios that require human interaction, real-time observation, or cannot be easily automated. These tests validate interactive behavior, streaming functionality, and provide debugging utilities for development.

## Test Scripts

### test_debug.sh

Quick debugging test for streaming chatbot functionality with variable assignment and arithmetic queries.

**Purpose:** Validates streaming chatbot behavior with multiple interaction types including greetings, variable assignments, and calculations.

**Usage:**
```bash
cd tests/manual
./test_debug.sh
```

**Test Sequence:**
- Sends "hello" greeting
- Assigns variable "x=13"
- Queries "what is 2+2"
- Sends "quit" to exit

### test_manual_streaming.sh

Manual validation of streaming response behavior with creative content generation.

**Purpose:** Tests streaming chatbot with longer-form creative content (poem generation) to observe real-time streaming behavior.

**Usage:**
```bash
cd tests/manual
./test_manual_streaming.sh
```

**Test Scenario:**
- Requests creative writing (poem about Rust programming)
- Observes streaming chunk delivery
- Verifies graceful termination

### test_streaming.sh

Basic streaming functionality validation with minimal input.

**Purpose:** Simple controlled test of streaming chatbot with basic interaction to verify core streaming mechanics.

**Usage:**
```bash
cd tests/manual
./test_streaming.sh
```

**Test Flow:**
- Basic "hello" greeting
- Immediate "quit" to test minimal interaction path

### test_runner.sh

Comprehensive test runner for secret error path validation.

**Purpose:** Executes secret error path tests with proper workspace configuration, handling the crate's exclusion from workspace.

**Usage:**
```bash
cd tests/manual
./test_runner.sh
```

**Features:**
- Sets up temporary workspace-like environment
- Configures CARGO_TARGET_DIR for build artifacts
- Ensures workspace dependencies are available
- Runs secret error path tests with verbose output
- Captures results to test_output.log

## Requirements

### Environment
- Valid OPENAI_API_KEY in workspace secrets (`../../../secret/-secrets.sh`)
- Network connectivity to OpenAI API
- Rust toolchain and cargo installed

### Dependencies
- Workspace dependencies must be built (test_runner.sh handles this automatically)
- Example binaries must be available (for streaming tests)

## Manual Testing Checklist

Interactive tests to perform manually:

- [ ] **Streaming Response Visualization** - Observe real-time chunk delivery
- [ ] **Error Path Validation** - Verify secret loading failures are handled correctly
- [ ] **Multi-turn Conversation** - Test stateful interaction patterns
- [ ] **Interactive Debugging** - Use test_debug.sh for development troubleshooting
- [ ] **Network Resilience** - Test behavior under connection issues

## Notes

**Example Availability:** Some test scripts reference the `streaming_chatbot` example. Verify this example exists or update scripts to use available streaming examples like:
- `openai_interactive_chat.rs`
- `openai_multi_turn_conversation.rs`
- `openai_cached_interactive_chat.rs`

**Workspace Configuration:** The test_runner.sh handles the special case where api_openai may be excluded from workspace test runs, providing isolated test execution.

## Troubleshooting

**Script Fails with "example not found":**
```bash
# List available examples
cargo run --example 2>&1 | grep "Available examples"

# Update script to use correct example name
```

**Secret Loading Errors:**
```bash
# Verify secret file exists
ls -la ../../../secret/-secrets.sh

# Check OPENAI_API_KEY is defined
source ../../../secret/-secrets.sh && echo $OPENAI_API_KEY
```
