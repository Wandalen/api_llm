# X.AI Grok API Examples

## Purpose

Usage demonstrations for the X.AI Grok API client showing real-world patterns and best practices.

## Responsibility Table

| File | Responsibility | Use Case |
|------|----------------|----------|
| `readme.md` | Document example organization | Running examples, prerequisites, troubleshooting |
| `basic_chat.rs` | Demonstrate basic chat completion | Simplest possible usage, getting started |
| `streaming_chat.rs` | Demonstrate SSE streaming | Real-time streaming responses |
| `interactive_chat.rs` | Demonstrate multi-turn conversations | Interactive chat with conversation context |
| `cached_interactive_chat.rs` | Demonstrate response caching | LRU caching integration |
| `list_models.rs` | Demonstrate model listing | Model discovery and details |
| `tool_calling.rs` | Demonstrate function calling | Tool definition and execution |
| `enhanced_tools_demo.rs` | Demonstrate enhanced tool features | Parallel execution, helpers |
| `client_side_enhancements.rs` | Demonstrate client-side features | Token counting, validation, CURL diagnostics |
| `enterprise_features.rs` | Demonstrate reliability features | Retry, circuit breaker, rate limiting |
| `failover_demo.rs` | Demonstrate endpoint failover | Multi-endpoint rotation |

## Prerequisites

**IMPORTANT:** You need an X.AI API key to run these examples.

### Step 1: Get API Key

Sign up at https://x.ai/ and obtain an API key from the developer console.

### Step 2: Set Environment Variable

```bash
export XAI_API_KEY="your-api-key-here"
```

### Step 3: Run Examples

```bash
# Basic examples
cargo run --example basic_chat --features full
cargo run --example streaming_chat --features full
cargo run --example list_models --features full

# Interactive examples
cargo run --example interactive_chat --features full
cargo run --example cached_interactive_chat --features full

# Advanced examples
cargo run --example tool_calling --features full
cargo run --example enhanced_tools_demo --features full
cargo run --example enterprise_features --features full
cargo run --example failover_demo --features full

# Client-side features
cargo run --example client_side_enhancements --features full
```

## Troubleshooting

### Examples fail with "API key not found"

**Cause:** XAI_API_KEY environment variable not set

**Fix:**
```bash
export XAI_API_KEY="your-api-key-here"
```

### Examples fail with "Unauthorized" or "Invalid API key"

**Cause:** API key is invalid or expired

**Fix:**
- Verify your API key at https://x.ai/
- Generate a new API key if needed
- Ensure no extra spaces in the environment variable

### Examples timeout or fail with network errors

**Cause:** X.AI API unavailable or rate limited

**Fix:**
- Check https://status.x.ai/ for API status
- Add retry logic (see `enterprise_features.rs` example)
- Reduce request rate if hitting limits

### Examples hang on first run

**Cause:** Large model loading or network latency

**Solution:** Wait 30-60 seconds. First requests may be slower due to model loading on X.AI's servers.

## Feature Requirements

Some examples require specific features to be enabled:

| Example | Required Features |
|---------|-------------------|
| `streaming_chat.rs` | `streaming` |
| `tool_calling.rs` | `tool_calling` |
| `enhanced_tools_demo.rs` | `enhanced_tools` |
| `cached_interactive_chat.rs` | `caching` |
| `enterprise_features.rs` | `retry`, `circuit_breaker`, `rate_limiting` |
| `failover_demo.rs` | `failover` |
| `client_side_enhancements.rs` | `count_tokens`, `input_validation`, `curl_diagnostics` |

Using `--features full` enables all features automatically.
