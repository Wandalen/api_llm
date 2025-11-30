# Implementation Roadmap

Feature implementation priorities and guidelines for api_ollama.

## Feature Status Overview

### Implemented Features
- `enabled` - Master switch for basic functionality
- `streaming` - Real-time streaming responses
- `embeddings` - Text embedding generation
- `vision_support` - Image inputs for vision models
- `tool_calling` - Function/tool calling support
- `builder_patterns` - Fluent builder APIs
- `retry` - Exponential backoff retry logic
- `circuit_breaker` - Circuit breaker pattern
- `rate_limiting` - Token bucket rate limiting
- `failover` - Automatic endpoint failover
- `health_checks` - Endpoint health monitoring
- `request_caching` - Response caching with TTL
- `general_diagnostics` - Metrics and diagnostics
- `streaming_control` - Pause/resume/cancel streaming
- `workspace` - Workspace-aware configuration
- `secret_management` - Credential storage
- `authentication` - API authentication
- `dynamic_config` - Runtime configuration

### Placeholder Features (Types Only)
- `websocket_streaming` - WebSocket bidirectional streaming
- `model_details` - Enhanced model metadata
- `model_tuning` - Model fine-tuning
- `model_deployment` - Deployment orchestration
- `audio_processing` - Speech-to-text and TTS
- `count_tokens` - Token counting
- `cached_content` - Content caching
- `batch_operations` - Batch processing
- `safety_settings` - Content filtering

### Partial Features
- `sync_api` - Synchronous API (no streaming support)

## High Priority Implementation

### 1. WebSocket Streaming

**Current State**: Types and connection pool structures defined

**Missing**: Actual WebSocket connection to Ollama `/api/ws` endpoint

**Implementation**:
- Connect to `ws://localhost:11434/api/ws`
- Implement message framing and heartbeat
- Add connection pooling and auto-reconnect
- Integrate with existing streaming types

**Effort**: High (1-2 weeks)

### 2. Token Counting

**Current State**: Placeholder returning mock data

**Implementation**:
- Check if Ollama `/api/count` endpoint exists
- If no API: implement client-side tokenization
- Add batch token counting support
- Implement cost estimation

**Effort**: Medium (1 week)

### 3. Structured Logging

**Current State**: Not implemented

**Implementation**:
- Add `tracing` dependency with feature flag
- Instrument all HTTP methods
- Add span tracking for request lifecycle
- Emit structured events with context

**Effort**: Medium (1-2 days)

## Medium Priority Implementation

### 4. Sync Streaming

**Current State**: `SyncOllamaClient` has non-streaming methods only

**Implementation**:
- Add `chat_stream()` returning Iterator
- Use blocking bridge for async streams
- Handle backpressure and cancellation
- Add timeout support

**Effort**: Medium (2-3 days)

### 5. Input Validation

**Current State**: Scattered validation

**Implementation**:
- Create centralized validation module
- Validate request fields
- Check data formats and parameter ranges
- Return detailed validation errors

**Effort**: Medium-High (1 week)

### 6. Enhanced Function Calling

**Current State**: Basic ToolDefinition and ToolCall types

**Implementation**:
- Add procedural macro for ToolDefinition
- Runtime parameter validation
- Built-in execution framework
- Type-safe parameter extraction

**Effort**: High (1-2 weeks)

## Low Priority Implementation

### 7. Batch Operations

**Implementation**:
- Check if Ollama has batch endpoint
- Improve concurrent implementation
- Add batch size limits and chunking
- Implement partial failure handling

**Effort**: Medium (1 week)

### 8. Audio Processing

**Implementation**:
- Investigate Ollama audio model support
- Implement multipart/form-data upload
- Add audio format validation
- Implement streaming audio responses

**Effort**: High (2-3 weeks)

### 9. Safety Settings

**Note**: May require external service integration

**Implementation**:
- Integrate with external moderation API
- Add pre-request content filtering
- Add post-response safety checks
- Implement custom safety rules

**Effort**: Medium-High (1-2 weeks)

### 10. Model Deployment

**Note**: May be beyond thin client scope

**Implementation**:
- Integrate with Ollama model management API
- Add model download and installation
- Implement version management
- Add deployment health checks

**Effort**: Very High (3-4 weeks)

## Cannot Implement (API Limitations)

- **Content Moderation**: Ollama lacks moderation API
- **Performance Metrics Module**: Beyond thin client scope
- **Google Search Grounding**: Specific to Gemini API
- **Code Execution**: Specific to Gemini API

## Implementation Guidelines

1. **Check Ollama API first** - Verify endpoint availability
2. **Maintain thin client principle** - No business logic
3. **Use feature flags** - Keep features optional
4. **Add comprehensive tests** - Integration tests with real Ollama
5. **Document limitations** - State client-side approximations clearly
