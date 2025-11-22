# Feature Implementation Roadmap

This document outlines potential features that could be fully implemented to replace current placeholder implementations or add new capabilities.

## High Priority - Direct Ollama API Support

### 1. Real WebSocket Streaming (`websocket_streaming`)

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

### 2. Token Counting (`count_tokens`)

- **Current:** Placeholder returning mock data
- **Missing:** Real token counting logic (if Ollama exposes API)
- **Effort:** Medium (1 week) - depends on Ollama API availability
- **Benefit:** Accurate token usage tracking and cost estimation
- **Implementation:**
  - Check if Ollama `/api/count` endpoint exists
  - If no API: implement client-side tokenization using model-specific tokenizers
  - Add batch token counting support
  - Implement cost estimation based on model pricing

### 3. Structured Logging (new feature)

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

## Medium Priority - Enhanced Client Features

### 4. Sync Streaming (`sync_api`)

- **Current:** `SyncOllamaClient` has non-streaming methods only
- **Missing:** `chat_stream()` and `generate_stream()` blocking iterators
- **Effort:** Medium (2-3 days)
- **Benefit:** Complete sync API parity with async
- **Implementation:**
  - Add `chat_stream()` returning `impl Iterator<Item = ChatResponse>`
  - Use `futures::executor::block_on_stream()` or custom blocking bridge
  - Handle backpressure and cancellation
  - Add timeout support

### 5. Input Validation (new feature)

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

### 6. Enhanced Function Calling (`tool_calling`)

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

## Low Priority - Advanced Features

### 7. Real Batch Operations (`batch_operations`)

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

### 8. Audio Processing (`audio_processing`)

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

### 9. Safety Settings (`safety_settings`)

- **Current:** Types defined, placeholder filtering
- **Missing:** Real content filtering logic
- **Effort:** Medium-High (1-2 weeks)
- **Benefit:** Content safety and compliance
- **Note:** May require external service integration (Ollama doesn't provide moderation)
- **Implementation:**
  - Integrate with external content moderation API (OpenAI Moderation, Perspective API)
  - Add pre-request content filtering
  - Add post-response safety checks
  - Implement compliance reporting
  - Add custom safety rules and blocklists

### 10. Model Deployment (`model_deployment`)

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

## Cannot Implement (API Limitations)

These features **cannot be implemented** due to Ollama API constraints or architectural principles:

- **Content Moderation**: Ollama doesn't provide moderation API (use external service)
- **Performance Metrics Module**: Beyond thin client scope (requires stateful aggregation)
- **Google Search Grounding**: Specific to Gemini API
- **Code Execution**: Specific to Gemini API

## Feature Implementation Guidelines

When implementing placeholder features:

1. **Check Ollama API first** - verify endpoint availability in Ollama documentation
2. **Maintain thin client principle** - no business logic, just HTTP communication
3. **Use feature flags** - keep features optional for minimal dependencies
4. **Add comprehensive tests** - integration tests with real Ollama server
5. **Document limitations** - clearly state if client-side approximations are used
