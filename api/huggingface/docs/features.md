# api_huggingface Feature Status

Complete reference of implemented and planned features for the HuggingFace API client library.

## ✅ Implemented Features

### Core APIs

| Feature | Status | Tier | Description |
|---------|--------|------|-------------|
| **Router API** | ✅ Complete | 1 | Chat completions with Pro models (Kimi-K2, Llama-3, Mistral, CodeLlama) |
| **Chat Completions** | ✅ Complete | 1 | Multi-turn conversations with role-based messages |
| **Text Generation** | ✅ Complete | 1 | Legacy inference API support |
| **Embeddings API** | ✅ Complete | 1 | Generate embeddings with similarity calculations |
| **Model Management** | ✅ Complete | 1 | Query model info, availability, status |
| **Streaming** | ✅ Complete | 1 | Real-time streaming responses |
| **Streaming Control** | ✅ Complete | 2 | Pause, resume, and cancel streaming operations with runtime control |
| **Function Calling** | ✅ Complete | 1 | OpenAI-compatible function calling with tool definitions and choice controls |

### Enterprise Features

| Feature | Status | Tier | Description |
|---------|--------|------|-------------|
| **Circuit Breaker** | ✅ Complete | 2 | Automatic failure detection and recovery |
| **Rate Limiting** | ✅ Complete | 2 | Token bucket rate limiting (per-second, per-minute, per-hour) |
| **Failover** | ✅ Complete | 2 | Multi-endpoint failover with strategies (Priority, RoundRobin, Random, Sticky) |
| **Health Checks** | ✅ Complete | 2 | Background endpoint health monitoring |
| **Dynamic Config** | ✅ Complete | 2 | Runtime configuration updates with watchers and rollback |
| **Caching** | ✅ Complete | 2 | LRU caching with TTL and statistics |
| **Performance Metrics** | ✅ Complete | 2 | Request latency, throughput, error rate tracking |
| **Token Counting** | ✅ Complete | 2 | Token estimation with multiple strategies |

### Multimodal APIs

| Feature | Status | Tier | Description |
|---------|--------|------|-------------|
| **Vision - Classification** | ✅ Complete | 2 | Image classification with confidence scores |
| **Vision - Detection** | ✅ Complete | 2 | Object detection with bounding boxes |
| **Vision - Captioning** | ✅ Complete | 2 | Image-to-text caption generation |
| **Audio - ASR** | ✅ Complete | 2 | Automatic speech recognition (transcription) |
| **Audio - TTS** | ✅ Complete | 2 | Text-to-speech generation |
| **Audio - Classification** | ✅ Complete | 2 | Audio classification |
| **Audio - Transformation** | ✅ Complete | 2 | Audio-to-audio (noise reduction, enhancement) |

### Development Features

| Feature | Status | Tier | Description |
|---------|--------|------|-------------|
| **Async/Await** | ✅ Complete | 1 | Full Tokio async support |
| **Sync API** | ✅ Complete | 2 | Blocking wrappers (`sync` feature) |
| **Explicit Retry** | ✅ Complete | 1 | Configurable retry logic (no automatic retries) |
| **CURL Diagnostics** | ✅ Complete | 1 | Generate curl commands for debugging |
| **Type Safety** | ✅ Complete | 1 | Comprehensive Rust types for all operations |
| **Error Handling** | ✅ Complete | 1 | Detailed error types with recovery guidance |
| **Model Constants** | ✅ Complete | 1 | Convenient model identifier constants |

## Legend

- **Tier 1**: Core functionality, production-ready, fully standardized
- **Tier 2**: Enterprise features, production-ready, contracts standardized

## ❌ Not Implemented

The following HuggingFace Inference API capabilities are **not implemented** in this crate:

### Vision APIs (Advanced)

- Image Segmentation
- Text-to-Image Generation
- Zero-Shot Image Classification
- Image Feature Extraction

### Audio APIs (Advanced)

- Audio Segmentation
- Audio Feature Extraction

### Multimodal APIs

- Visual Question Answering
- Document Question Answering
- Image-Text-to-Text
- Video-Text-to-Text
- Audio-Text-to-Text

### Advanced NLP APIs

- Fill-Mask
- Zero-Shot Classification
- Token Classification (NER)
- Text Ranking
- Table Question Answering
- Tabular Classification/Regression

### Infrastructure Management

- Dedicated Inference Endpoint lifecycle
- Endpoint health checks, logs, metrics

## Implementation Focus

This library focuses on **conversational AI and text embeddings**. The unimplemented features above are specialized ML APIs that may require different endpoint architectures or may not be compatible with the Router API.

## Cargo Features

Control which functionality is included in your build:

### Core Features

- `default`: Core functionality only
- `full`: All features including integration tests

### Capability Features

- `inference-streaming`: Streaming support for text generation
- `streaming-control`: Pause, resume, and cancel operations for controlled streams
- `embeddings-similarity`: Similarity calculation utilities
- `sync`: Synchronous API wrappers

### Testing Features

- `integration`: Enable integration tests with real API calls

### Feature Combination Examples

```toml
# Minimal build (async only)
api_huggingface = "0.2.0"

# With streaming support
api_huggingface = { version = "0.2.0", features = ["inference-streaming"] }

# Synchronous API
api_huggingface = { version = "0.2.0", features = ["sync"] }

# Everything (recommended for development)
api_huggingface = { version = "0.2.0", features = ["full"] }
```
