# API Ollama

Low-level HTTP client for the Ollama local LLM runtime API.

## 🎯 Architecture: Stateless HTTP Client

**This API crate is designed as a stateless HTTP client with zero persistence requirements.** It provides:
- Direct HTTP calls to the Ollama API
- In-memory operation state only (resets on restart)
- No external storage dependencies (databases, files, caches)
- No configuration persistence beyond environment variables

This ensures lightweight, containerized deployments and eliminates operational complexity.

## Overview

This crate provides direct HTTP communication with Ollama's REST API endpoints. It handles all request/response serialization, error handling, and streaming support for Ollama operations.

## Features

- **Chat Completions**: Send chat messages and receive responses
- **Text Generation**: Generate text from prompts  
- **Model Management**: List and get information about available models
- **Streaming Support**: Real-time streaming responses (with `streaming` feature)
- **Error Handling**: Comprehensive error types for different failure modes
- **Type Safety**: Strongly typed requests and responses

## Usage

```rust,no_run
use api_ollama::{OllamaClient, ChatRequest, ChatMessage, MessageRole};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = OllamaClient::new("http://localhost:11434".to_string(), std::time::Duration::from_secs(30));
    
    // Check if Ollama is available
    if !client.is_available().await {
        println!("Ollama is not available");
        return Ok(());
    }
    
    // List available models
    let models = client.list_models().await?;
    println!("Available models: {:?}", models);
    
    // Only proceed if models are available
    if models.models.is_empty() {
        println!("No models available, skipping chat test");
        return Ok(());
    }
    
    // Send chat request
    let request = ChatRequest {
        model: "llama3.2".to_string(),
        messages: vec![ChatMessage {
            role: MessageRole::User,
            content: "Hello!".to_string(),
            images: None,
            #[cfg(feature = "tool_calling")]
            tool_calls: None,
        }],
        stream: None,
        options: None,
        #[cfg(feature = "tool_calling")]
        tools: None,
        #[cfg(feature = "tool_calling")]
        tool_messages: None,
    };
    
    let response = client.chat(request).await?;
    println!("Response: {:?}", response);
    
    Ok(())
}
```

## Cargo Features

This crate uses extensive feature flags for granular dependency management and compile-time optimization.

### Core Features

| Feature | Status | Description |
|---------|--------|-------------|
| `enabled` | ✅ Implemented | Master switch - activates all dependencies and basic functionality |
| `streaming` | ✅ Implemented | Real-time streaming responses for chat and generate |
| `embeddings` | ✅ Implemented | Text embedding generation API |
| `vision_support` | ✅ Implemented | Image inputs for vision-capable models |
| `tool_calling` | ✅ Implemented | Function/tool calling support |
| `builder_patterns` | ✅ Implemented | Fluent builder APIs for requests |

### Enterprise Reliability

| Feature | Status | Description |
|---------|--------|-------------|
| `retry` | ✅ Implemented | Exponential backoff retry logic |
| `circuit_breaker` | ✅ Implemented | Circuit breaker pattern to prevent cascading failures |
| `rate_limiting` | ✅ Implemented | Token bucket and sliding window rate limiting |
| `failover` | ✅ Implemented | Automatic failover to backup endpoints |
| `health_checks` | ✅ Implemented | Endpoint health monitoring |
| `request_caching` | ✅ Implemented | Response caching with TTL |
| `general_diagnostics` | ✅ Implemented | Metrics collection and diagnostics |

### Streaming Enhancements

| Feature | Status | Description |
|---------|--------|-------------|
| `streaming_control` | ✅ Implemented | Pause/resume/cancel streaming operations |
| `websocket_streaming` | ⚠️ Placeholder | WebSocket-based real-time bidirectional streaming (types only) |

### Configuration & Workspace

| Feature | Status | Description |
|---------|--------|-------------|
| `workspace` | ✅ Implemented | Workspace-aware configuration loading |
| `secret_management` | ✅ Implemented | Secure API key and credential storage |
| `authentication` | ✅ Implemented | API authentication support |
| `dynamic_config` | ✅ Implemented | Runtime configuration management |

### API Patterns

| Feature | Status | Description |
|---------|--------|-------------|
| `sync_api` | ⚠️ Partial | Synchronous blocking API (no streaming support) |

### Advanced Features

| Feature | Status | Description |
|---------|--------|-------------|
| `model_details` | ⚠️ Placeholder | Enhanced model metadata and lifecycle tracking (types only) |
| `model_tuning` | ⚠️ Placeholder | Model fine-tuning capabilities (types only) |
| `model_deployment` | ⚠️ Placeholder | Model deployment orchestration (types only) |
| `audio_processing` | ⚠️ Placeholder | Speech-to-text and text-to-speech (types only) |
| `count_tokens` | ⚠️ Placeholder | Token counting with cost estimation (types only) |
| `cached_content` | ⚠️ Placeholder | Intelligent content caching (types only) |
| `batch_operations` | ⚠️ Placeholder | Batch request processing (simplified logic) |
| `safety_settings` | ⚠️ Placeholder | Content filtering and harm prevention (types only) |

### Meta Features

| Feature | Description |
|---------|-------------|
| `default` | Enables `full` for convenience |
| `full` | Enables all features including placeholders |
| `integration` | Enables integration tests (requires live Ollama server) |
| `integration_tests` | External integration test support |
| `advanced` | Advanced production features with additional dependencies |

**Legend:**
- ✅ **Implemented**: Fully functional with real HTTP API calls
- ⚠️ **Placeholder**: Types and structs defined, minimal/mock functionality
- ⚠️ **Partial**: Core functionality works, some features missing

## Implementable Features

These features **could be fully implemented** to replace current placeholder implementations or add new capabilities:

### High Priority - Direct Ollama API Support

1. **Real WebSocket Streaming** (`websocket_streaming`)
   - **Current:** Types and connection pool structures defined
   - **Missing:** Actual WebSocket connection to Ollama `/api/ws` endpoint
   - **Effort:** High (1-2 weeks)
   - **Benefit:** True bidirectional real-time streaming for multi-turn conversations
   - **Dependencies:** `tokio-tungstenite` (already imported)
   - **Implementation:**
     - Connect to `ws://localhost:11434/api/ws`
     - Implement message framing and heartbeat
     - Add connection pooling and auto-reconnect
     - Integrate with existing streaming types

2. **Token Counting** (`count_tokens`)
   - **Current:** Placeholder returning mock data
   - **Missing:** Real token counting logic (if Ollama exposes API)
   - **Effort:** Medium (1 week) - depends on Ollama API availability
   - **Benefit:** Accurate token usage tracking and cost estimation
   - **Implementation:**
     - Check if Ollama `/api/count` endpoint exists
     - If no API: implement client-side tokenization using model-specific tokenizers
     - Add batch token counting support
     - Implement cost estimation based on model pricing

3. **Structured Logging** (new feature)
   - **Current:** Not implemented
   - **Missing:** Structured event logging with tracing
   - **Effort:** Medium (1-2 days)
   - **Benefit:** Production observability and debugging
   - **Dependencies:** `tracing` (not yet added)
   - **Implementation:**
     - Add `tracing` dependency with `structured_logging` feature flag
     - Instrument all HTTP methods (chat, generate, embeddings, streaming)
     - Add span tracking for request lifecycle
     - Emit structured events with context (request_id, model, duration, status)
     - Support trace propagation for distributed systems

### Medium Priority - Enhanced Client Features

4. **Sync Streaming** (`sync_api`)
   - **Current:** `SyncOllamaClient` has non-streaming methods only
   - **Missing:** `chat_stream()` and `generate_stream()` blocking iterators
   - **Effort:** Medium (2-3 days)
   - **Benefit:** Complete sync API parity with async
   - **Implementation:**
     - Add `chat_stream()` returning `impl Iterator<Item = ChatResponse>`
     - Use `futures::executor::block_on_stream()` or custom blocking bridge
     - Handle backpressure and cancellation
     - Add timeout support

5. **Input Validation** (new feature)
   - **Current:** Scattered validation in various modules
   - **Missing:** Centralized validation framework
   - **Effort:** Medium-High (1 week)
   - **Benefit:** Better error messages and request safety
   - **Implementation:**
     - Create `input_validation.rs` module with validation traits
     - Validate ChatRequest, GenerateRequest, EmbeddingsRequest fields
     - Check image data formats, audio formats, model names
     - Validate parameter ranges (temperature, top_p, etc.)
     - Return detailed validation errors with field-level messages

6. **Enhanced Function Calling** (`tool_calling`)
   - **Current:** Basic ToolDefinition and ToolCall types
   - **Missing:** Automatic parsing, validation, execution framework
   - **Effort:** High (1-2 weeks)
   - **Benefit:** Better developer experience with tool use
   - **Implementation:**
     - Add procedural macro to derive ToolDefinition from Rust functions
     - Runtime parameter validation against JSON schema
     - Built-in execution framework with error handling
     - Automatic response serialization
     - Type-safe parameter extraction

### Low Priority - Advanced Features

7. **Real Batch Operations** (`batch_operations`)
   - **Current:** Simplified concurrent logic
   - **Missing:** Proper batch API integration (if Ollama supports it)
   - **Effort:** Medium (1 week)
   - **Benefit:** Optimized multi-request processing
   - **Implementation:**
     - Check if Ollama has dedicated batch endpoint
     - If no API: improve current concurrent implementation
     - Add batch size limits and automatic chunking
     - Implement partial failure handling with retry
     - Add batch progress tracking

8. **Audio Processing** (`audio_processing`)
   - **Current:** Types defined, no actual audio APIs
   - **Missing:** Speech-to-text and text-to-speech integration
   - **Effort:** High (2-3 weeks) - depends on Ollama capabilities
   - **Benefit:** Voice interaction support
   - **Implementation:**
     - Investigate if Ollama supports audio models (Whisper, TTS)
     - If supported: implement multipart/form-data upload for audio
     - Add audio format validation (wav, mp3, ogg)
     - Implement streaming audio responses
     - Add voice chat with bidirectional audio

9. **Safety Settings** (`safety_settings`)
   - **Current:** Types defined, placeholder filtering
   - **Missing:** Real content filtering logic
   - **Effort:** Medium-High (1-2 weeks)
   - **Benefit:** Content safety and compliance
   - **Note:** May require external service integration (Ollama doesnt provide moderation)
   - **Implementation:**
     - Integrate with external content moderation API (OpenAI Moderation, Perspective API)
     - Add pre-request content filtering
     - Add post-response safety checks
     - Implement compliance reporting
     - Add custom safety rules and blocklists

10. **Model Deployment** (`model_deployment`)
    - **Current:** Types defined, no deployment logic
    - **Missing:** Model deployment orchestration
    - **Effort:** Very High (3-4 weeks)
    - **Benefit:** Automated model lifecycle management
    - **Note:** May be beyond thin client scope
    - **Implementation:**
      - Integrate with Ollama model management API
      - Add model download and installation
      - Implement model version management
      - Add deployment health checks
      - Support multi-instance deployment

### Cannot Implement (API Limitations)

These features **cannot be implemented** due to Ollama API constraints or architectural principles:

- **Content Moderation**: Ollama doesnt provide moderation API (use external service)
- **Performance Metrics Module**: Beyond thin client scope (requires stateful aggregation)
- **Google Search Grounding**: Specific to Gemini API
- **Code Execution**: Specific to Gemini API

### Feature Implementation Guidelines

When implementing placeholder features:
1. **Check Ollama API first** - verify endpoint availability in Ollama documentation
2. **Maintain thin client principle** - no business logic, just HTTP communication
3. **Use feature flags** - keep features optional for minimal dependencies
4. **Add comprehensive tests** - integration tests with real Ollama server
5. **Document limitations** - clearly state if client-side approximations are used

## Architecture

This crate is a pure HTTP client and does not contain any business logic or abstractions beyond what's needed for API communication. It's designed to be used by higher-level provider crates that implement unified interfaces.

## 🏛️ Governing Principle: "Thin Client, Rich API"

This library follows the **"Thin Client, Rich API"** principle, ensuring predictable, explicit, and transparent behavior:

### 1. **API Transparency**
- Every method directly corresponds to an Ollama API endpoint
- No hidden transformations or side effects
- Method names clearly indicate exact server calls

### 2. **Zero Client Intelligence**
- No automatic decision-making or behavior inference
- No configuration-driven automatic actions
- All behaviors are explicitly requested by developers

### 3. **Explicit Control**
- Developers have complete control over when and how API calls are made
- No background operations or automatic retries
- Clear separation between information retrieval and action methods

### 4. **Information vs Action**
- Information methods (like `list_models()`) only retrieve data
- Action methods (like `chat()`) only perform requested operations
- No methods that implicitly combine information gathering with actions