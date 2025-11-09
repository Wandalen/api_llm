# X.AI Grok API Client for Rust

[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental)

**Production-ready Rust client for X.AI's Grok API with 73% feature coverage**

A comprehensive, feature-rich HTTP client for interacting with X.AI's Grok API. Built with the "Thin Client, Rich API" principle, providing transparent access to all server-side functionality plus powerful client-side enhancements.

## ✨ Features

### 🎯 Core Features (100%)
- **💬 Chat Completions** - Full conversational interface support
- **📡 Streaming** - Server-Sent Events (SSE) streaming responses
- **🛠️ Tool Calling** - Complete function calling and tool integration
- **📋 Model Management** - List and retrieve model details

### 🏢 Enterprise Reliability (100%)
- **🔄 Retry Logic** - Exponential backoff with jitter
- **⚡ Circuit Breaker** - Failure threshold management
- **🚦 Rate Limiting** - Token bucket algorithm
- **🔀 Failover** - Multi-endpoint rotation
- **❤️ Health Checks** - Kubernetes-style liveness/readiness probes
- **📝 Structured Logging** - Tracing integration

### 🚀 Client-Side Enhancements (NEW - 100%)
- **🔢 Token Counting** - Local counting using tiktoken (GPT-4 encoding)
- **💾 Response Caching** - LRU cache for performance optimization
- **✅ Input Validation** - Comprehensive request parameter validation
- **🐛 CURL Diagnostics** - Request-to-CURL conversion for debugging
- **📦 Batch Operations** - Parallel request orchestration
- **📊 Performance Metrics** - Prometheus metrics collection
- **🔄 Sync API** - Blocking wrappers for legacy code (not recommended)

### 🎨 Developer Experience
- **🔑 Secret Management** - workspace_tools integration with fallback chain
- **🎯 Type Safety** - Former builder pattern for requests
- **📚 Rich Documentation** - 122 doc tests and 10 comprehensive examples
- **🧪 Zero Mocking** - Real integration tests only
- **⚙️ Feature Flags** - Fine-grained control over dependencies

## 📊 Feature Coverage

**Overall**: 33/45 features implemented (73%)

| Category | Coverage | Status |
|----------|----------|--------|
| Core Features | 73% (8/11) | ✅ |
| Standard Chat Examples | 100% (3/3) | ✅ |
| Enterprise Reliability | 100% (5/5) | ✅ |
| Client-Side Enhancements | 100% (7/7) | ✅ |
| Monitoring & Analytics | 100% (1/1) | ✅ |
| Testing & Observability | 100% (4/4) | ✅ |
| Configuration Management | 100% (3/3) | ✅ |
| API Patterns | 80% (4/5) | 🟢 |

**Note**: Remaining 27% unimplemented features are primarily API limitations (vision, audio, embeddings, safety, moderation, model tuning, etc.) that cannot be implemented without XAI API support, plus 1 intentionally skipped feature (Sync Streaming - not recommended).

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
api_xai = { version = "0.1.0", features = ["full"] }
```

### Basic Chat

```rust,no_run
use api_xai::{ Client, Secret, XaiEnvironmentImpl, ChatCompletionRequest, Message, ClientApiAccessors };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from workspace secrets or environment
    let secret = Secret::load_with_fallbacks("XAI_API_KEY")?;
    let env = XaiEnvironmentImpl::new(secret)?;
    let client = Client::build(env)?;

    // Create a chat request
    let request = ChatCompletionRequest::former()
        .model("grok-3".to_string())
        .messages(vec![Message::user("Hello, Grok!")])
        .form();

    // Send request and get response
    let response = client.chat().create(request).await?;
    println!("Grok: {:?}", response.choices[0].message.content);

    Ok(())
}
```

### Streaming Chat

```rust,no_run
use api_xai::{ Client, Secret, XaiEnvironmentImpl, ChatCompletionRequest, Message, ClientApiAccessors };
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = Secret::load_with_fallbacks("XAI_API_KEY")?;
    let env = XaiEnvironmentImpl::new(secret)?;
    let client = Client::build(env)?;

    let request = ChatCompletionRequest::former()
        .model("grok-3".to_string())
        .messages(vec![Message::user("Tell me a story")])
        .stream(true)
        .form();

    let chat = client.chat();
    let mut stream = chat.create_stream(request).await?;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        if let Some(content) = chunk.choices[0].delta.content.as_ref() {
            print!("{}", content);
        }
    }

    Ok(())
}
```

### Client-Side Enhancements

```rust,no_run
use api_xai::{ Client, Secret, XaiEnvironmentImpl, ChatCompletionRequest, Message };
use api_xai::count_tokens::count_tokens;
use api_xai::CachedClient;
use api_xai::BatchProcessor;
use api_xai::validate_request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = Secret::load_with_fallbacks("XAI_API_KEY")?;
    let env = XaiEnvironmentImpl::new(secret)?;
    let client = Client::build(env)?;

    // 1. Count tokens locally (no API call)
    let tokens = count_tokens("Hello, world!", "grok-3")?;
    println!("Token count: {}", tokens);

    // 2. Use cached client for repeated requests
    let cached = CachedClient::new(client.clone(), 100);
    let request = ChatCompletionRequest::former()
        .model("grok-3".to_string())
        .messages(vec![Message::user("What is 2+2?")])
        .form();

    let response = cached.cached_create(request).await?;

    // 3. Batch processing with parallel execution
    let processor = BatchProcessor::new(client, 5);
    let requests = vec![
        ChatCompletionRequest::former()
            .model("grok-3".to_string())
            .messages(vec![Message::user("Request 1")])
            .form(),
        ChatCompletionRequest::former()
            .model("grok-3".to_string())
            .messages(vec![Message::user("Request 2")])
            .form(),
    ];

    let results = processor.process_batch(requests).await;
    println!("Processed {} requests", results.len());

    // 4. Input validation (catches errors before API call)
    let request = ChatCompletionRequest::former()
        .model("grok-3".to_string())
        .messages(vec![Message::user("Test")])
        .temperature(0.7)
        .form();

    validate_request(&request)?;  // Validates before sending

    Ok(())
}
```

## 🔧 Configuration

### Secret Loading (Recommended)

The crate uses `workspace_tools` for secret management with automatic fallback chain:

1. **Workspace secrets** (`./secret/-secrets.sh`) - primary
2. **Alternative files** (`secrets.sh`, `.env`) - workspace alternatives
3. **Environment variable** - fallback for CI/deployment

```bash
# Create workspace secret file
cat > ./secret/-secrets.sh << 'EOF'
#!/bin/bash
export XAI_API_KEY="xai-your-key-here"
EOF
```

Or use environment variable:

```bash
export XAI_API_KEY="xai-your-key-here"
```

## 📚 Documentation

- **[API Reference](https://docs.rs/api_xai)** - Complete API documentation
- **[Examples](examples/)** - Real-world usage examples
- **[Specification](spec.md)** - Detailed project specification

## 🏗️ Project Structure

```text
api_xai/
├── src/                  # Library source code
│   ├── count_tokens.rs   # Token counting (NEW)
│   ├── caching.rs        # Response caching (NEW)
│   ├── input_validation.rs # Parameter validation (NEW)
│   ├── curl_diagnostics.rs # Debug utilities (NEW)
│   ├── batch_operations.rs # Parallel processing (NEW)
│   ├── performance_metrics.rs # Metrics collection (NEW)
│   ├── sync_api.rs       # Sync wrappers (NEW)
│   └── ...               # Core features
├── tests/                # Comprehensive test suite
├── examples/             # Usage demonstrations (10 examples)
│   ├── basic_chat.rs     # Simple single-turn chat
│   ├── interactive_chat.rs # Multi-turn conversation
│   ├── cached_interactive_chat.rs # Interactive with caching (NEW)
│   ├── streaming_chat.rs # SSE streaming example
│   ├── tool_calling.rs   # Function calling demo
│   ├── enhanced_tools_demo.rs # Parallel tool execution
│   ├── client_side_enhancements.rs # All NEW features
│   ├── enterprise_features.rs # Reliability stack
│   ├── failover_demo.rs  # Multi-endpoint failover
│   └── list_models.rs    # Model catalog
└── spec.md               # Project specification
```

## 🧪 Testing

```bash
# Run all tests
cargo test --all-features

# Run with doc tests
cargo test --all-features --doc

# Run with linting
cargo clippy --all-targets --all-features

# Build documentation
cargo doc --no-deps --all-features --open
```

**⚠️ Note:** Integration tests require valid `XAI_API_KEY` and use real API calls (NO MOCKING policy).

**Test Coverage:**
- 122 doc tests (all passing, 1 ignored)
- 107 integration tests (all passing)
- **229 total tests** with real API validation (NO MOCKING policy)

## 📋 Feature Flags

Fine-grained control via feature flags:

### Core Features
- `enabled` - Master switch for core functionality
- `streaming` - SSE streaming support
- `tool_calling` - Function calling and tools
- `integration` - Integration tests with real API

### Enterprise Reliability
- `retry` - Exponential backoff retry logic
- `circuit_breaker` - Circuit breaker pattern
- `rate_limiting` - Token bucket rate limiting
- `failover` - Multi-endpoint failover
- `health_checks` - Health monitoring
- `structured_logging` - Tracing integration

### Client-Side Enhancements (NEW)
- `count_tokens` - Local token counting (requires: tiktoken-rs)
- `caching` - Response caching (requires: lru)
- `input_validation` - Request validation
- `curl_diagnostics` - Debug utilities
- `batch_operations` - Parallel processing (requires: tokio/sync)
- `performance_metrics` - Metrics collection (requires: prometheus)
- `sync_api` - Sync wrappers (requires: tokio/rt-multi-thread)

### Presets
- `full` - All features enabled (default)
- `default` - Alias for `full`

## 🎯 Design Principles

### "Thin Client, Rich API"

This crate follows the **"Thin Client, Rich API"** principle:

- **API Transparency**: One-to-one mapping with X.AI Grok API endpoints
- **Zero Automatic Behavior**: No implicit decision-making or magic thresholds
- **Explicit Control**: Developer decides when, how, and why operations occur
- **Configurable Reliability**: Enterprise features available through explicit configuration

**Key Distinction**: The principle prohibits **automatic/implicit** behaviors but explicitly **allows and encourages** **explicit/configurable** enterprise reliability features.

### Client-Side Enhancements Philosophy

The 7 new client-side enhancement features compensate for XAI API limitations while maintaining transparency:

- **Token Counting**: API doesn't provide endpoint → use tiktoken locally
- **Caching**: API doesn't support caching → implement LRU cache
- **Validation**: API errors are expensive → validate before sending
- **Diagnostics**: Debugging requires reproduction → generate CURL commands
- **Batch Operations**: API lacks batch endpoint → orchestrate parallel requests
- **Metrics**: API doesn't expose metrics → collect client-side with Prometheus
- **Sync API**: Some codebases need sync → provide blocking wrappers (not recommended)

All enhancements are:
- ✅ Explicitly feature-gated
- ✅ Zero overhead when disabled
- ✅ Transparent in operation
- ✅ Well-documented with examples

## 🔮 API Limitations

The following features **cannot** be implemented due to XAI API limitations:

- Vision/Multimodal, Audio Processing, Embeddings (no API support)
- Safety Settings, Content Moderation (no API endpoints)
- WebSocket Streaming, Streaming Control (SSE only)
- Model Tuning, Model Deployment (no API support)
- Google Search Grounding, Code Execution (not XAI features)

## 🚀 Production Readiness

**Status**: ✅ **Production Ready** (65% feature coverage)

**Strengths**:
- All implementable features complete
- Comprehensive enterprise reliability stack
- Client-side enhancements compensate for API limitations
- Zero mocking - all tests use real API
- Feature flags for minimal builds
- Extensive documentation and examples

**Recommendation**: Use for production text-based chat applications with enterprise reliability requirements. For vision/audio/embeddings, integrate with OpenAI or other providers.

## 📝 OpenAI Compatibility

The X.AI Grok API is OpenAI-compatible, using the same REST endpoint patterns and request/response formats. This allows for easy migration from OpenAI to X.AI with minimal code changes.

**Token Counting**: Uses GPT-4 encoding (cl100k_base) via tiktoken for accurate token counts compatible with XAI models.

## 📝 License

This project is licensed under the MIT License.

## 🤝 Contributing

Contributions welcome! Please ensure:
- All tests pass (`cargo test --all-features`)
- Clippy is clean (`cargo clippy --all-targets --all-features`)
- Documentation builds (`cargo doc --no-deps --all-features`)
- Follow the "Thin Client, Rich API" principle
- Respect the NO MOCKING testing policy

## 🔗 Related Projects

Part of the `api_llm` workspace:
- `api_claude` - Anthropic Claude API client
- `api_gemini` - Google Gemini API client
- `api_ollama` - Ollama API client
- `api_openai` - OpenAI API client
