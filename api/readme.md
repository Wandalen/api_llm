# API Crates

[![stable](https://raster.shields.io/static/v1?label=stability&message=stable&color=green&logoColor=eee)](https://github.com/emersion/stability-badges#stable)

Collection of Rust API clients for major LLM providers with enterprise reliability features.

## 🎯 Architecture: Stateless HTTP Clients

**All API crates are designed as stateless HTTP clients with zero persistence requirements.** They provide:
- Direct HTTP calls to respective LLM provider APIs
- In-memory operation state only (resets on restart)
- No external storage dependencies (databases, files, caches)
- No configuration persistence beyond environment variables

This ensures lightweight, containerized deployments and eliminates operational complexity.

## 🏛️ Governing Principle: "Thin Client, Rich API"

**Expose all server-side functionality transparently while maintaining zero client-side intelligence or automatic behaviors.**

Key principles:
- **API Transparency**: One-to-one mapping with provider APIs without hidden behaviors
- **Zero Client Intelligence**: No automatic decision-making or magic thresholds
- **Explicit Control**: Developer decides when, how, and why operations occur
- **Information vs Action**: Clear separation between data retrieval and state changes

## Scope

### In Scope
- Text generation (single and multi-turn conversations)
- Streaming responses (SSE and WebSocket where applicable)
- Function/tool calling with full schema support
- Vision and multimodal inputs
- Audio processing (speech-to-text, text-to-speech)
- Embedding generation
- Model listing and information
- Token counting
- Batch operations
- Enterprise reliability (retry, circuit breaker, rate limiting, failover, health checks)

### Out of Scope
- High-level abstractions or unified interfaces
- Provider switching or fallback logic
- Business logic or application features
- Persistent state management

## Crates Overview

| Crate | Provider | Status | Tests |
|-------|----------|--------|-------|
| [api_gemini](gemini/) | Google Gemini | Production | 485 |
| [api_openai](openai/) | OpenAI | Production | 643 |
| [api_claude](claude/) | Anthropic Claude | Production | 435 |
| [api_ollama](ollama/) | Ollama (Local) | Production | 378 |
| [api_huggingface](huggingface/) | HuggingFace | Production | 534 |
| [api_xai](xai/) | xAI Grok | Production | 127 |

## Feature Matrix

| Feature | api_gemini | api_openai | api_claude | api_ollama | api_huggingface | api_xai |
|---------|------------|------------|------------|------------|-----------------|---------|
| **Core Features** |||||||
| Text Generation | Yes | Yes | Yes | Yes | Yes | Yes |
| Streaming | Yes | Yes | Yes | Yes | Yes | Yes |
| Function Calling | Yes | Yes | Yes | Yes | Yes | Yes |
| Vision/Multimodal | Yes | Yes | Yes | Yes | Yes | No |
| Audio Processing | Yes | Yes | No | Yes | Yes | No |
| Embeddings | Yes | Yes | Partial | Yes | Yes | No |
| Model Listing | Yes | Yes | Yes | Yes | Yes | Yes |
| Count Tokens | Yes | No | Yes | Yes | Yes | Yes |
| Cached Content | Yes | Yes | Yes | Yes | Yes | Yes |
| Batch Operations | Yes | Yes | Yes | Yes | Yes | Yes |
| **Enterprise Reliability** |||||||
| Retry Logic | Yes | Yes | Yes | Yes | Yes | Yes |
| Circuit Breaker | Yes | Yes | Yes | Yes | Yes | Yes |
| Rate Limiting | Yes | Yes | Yes | Yes | Yes | Yes |
| Failover | Yes | Yes | Yes | Yes | Yes | Yes |
| Health Checks | Yes | Yes | Yes | Yes | Yes | Yes |
| Request Caching | Yes | Yes | Yes | Yes | Yes | Yes |
| **Streaming Enhancements** |||||||
| HTTP Streaming | Yes | Yes | Yes | Yes | Yes | Yes |
| WebSocket Streaming | Yes | Yes | No | Yes | No | No |
| Streaming Control | Yes | Yes | Yes | Yes | Yes | No |
| **API Patterns** |||||||
| Async API | Yes | Yes | Yes | Yes | Yes | Yes |
| Sync API | Yes | Yes | Yes | Yes | Yes | Yes |
| Sync Streaming | Yes | Yes | Yes | Yes | Yes | Yes |
| **Testing & Observability** |||||||
| Integration Tests | Yes | Yes | Yes | Yes | Yes | Yes |
| Structured Logging | Yes | Yes | Yes | Yes | Yes | Yes |
| CURL Diagnostics | Yes | Yes | Yes | Yes | Yes | Yes |

**Legend:**
- **Yes** - Implemented and tested
- **Partial** - Limited implementation
- **No** - Not available (API limitation or not implemented)

## Standard Examples

All API crates provide consistent example programs:

| Example | Description |
|---------|-------------|
| `{api}_chat_basic.rs` | Basic chat interaction |
| `{api}_chat_interactive.rs` | Interactive terminal chat |
| `{api}_chat_cached_interactive.rs` | Cached interactive chat |

## Testing Policy

All API crates follow the zero-tolerance mock policy:
- Real API integration tests only
- No mock HTTP requests or simulated responses
- Integration tests require valid API credentials
- Tests fail loudly when credentials unavailable

## Documentation

- **[api_gemini](gemini/)** - Google Gemini API client
- **[api_openai](openai/)** - OpenAI API client
- **[api_claude](claude/)** - Anthropic Claude API client
- **[api_ollama](ollama/)** - Ollama local API client
- **[api_huggingface](huggingface/)** - HuggingFace Inference API client
- **[api_xai](xai/)** - xAI Grok API client

## Contributing

1. Follow the "Thin Client, Rich API" governing principle
2. Use 2-space indentation consistently
3. Add real API integration tests (no mocking)
4. Update feature matrix when adding capabilities
5. Ensure zero clippy warnings: `cargo clippy -- -D warnings`
