# API Models

## Purpose
Comprehensive collection of all request/response data structures and API implementations for the Google Gemini API. This module contains the complete type system for content generation, embeddings, model management, and advanced features like streaming, deployment, and media handling.

## Organization Principles
- **Domain-Driven Organization**: Files grouped by API domain and functionality
- **Core vs. Advanced**: Basic API types in `api.rs`, advanced features in dedicated modules
- **Feature Gating**: Optional functionality behind cargo features
- **Optimized Variants**: Performance-optimized implementations suffixed with `_optimized`
- **Real vs. Experimental**: Production-ready implementations vs. stub implementations awaiting Gemini API

## Navigation Guide

### Core API Types
- **api.rs** (2,962 lines) - Foundation request/response types:
  - Content generation (GenerateContentRequest, GenerateContentResponse)
  - Embeddings (EmbedContentRequest, EmbedContentResponse)
  - Model management (Model, ListModelsResponse)
  - Safety settings and function calling
  - Token counting and caching

### Configuration & Deployment
- **config.rs** (3,135 lines) - Dynamic configuration system:
  - Multi-source configuration (env, files, remote APIs)
  - Hot-reload with file watching
  - Configuration rollback and versioning
  - Change propagation via events

- **model_deployment.rs** (1,901 lines) - Production deployment:
  - Multiple strategies (BlueGreen, Canary, Rolling, Recreate, AB)
  - Auto-scaling based on metrics
  - Health monitoring and automatic failover
  - Container orchestration (Kubernetes/Docker)

### Streaming & Real-Time
- **streaming_control.rs** (1,341 lines) - Stream management:
  - Pause, resume, and cancel operations
  - Buffer management for paused streams
  - Thread-safe atomic state control

- **websocket_streaming.rs** (587 lines) - WebSocket integration:
  - Bidirectional real-time communication
  - Connection pooling and management

- **websocket_streaming_optimized.rs** (791 lines) - Performance-optimized WebSocket

### Media & Content Management
- **media_optimization.rs** (1,051 lines) - Media API:
  - File upload with validation
  - Metadata management
  - Pagination support
  - Multimodal integration

### Enterprise Features
- **health.rs** - Endpoint health verification
- **failover.rs** (551 lines) - Multi-endpoint failover
- **batch.rs** - Batch operations (stub)
- **model_tuning.rs** (691 lines) - Model fine-tuning

### Experimental (Stub Implementations)
⚠️ **Note**: These return mock data awaiting Gemini API endpoint availability

- **workspaces.rs** (1,356 lines) - Workspace management (13 methods, caching implemented)
- **semantic_retrieval.rs** (1,459 lines) - Semantic search (partial implementation)
- **semantic_retrieval_optimized.rs** (786 lines) - Optimized semantic search

### Module Structure
- **mod.rs** (2,119 lines) - Public API surface and re-exports

## File Size Distribution
```
5 files >2000 lines: api.rs, config.rs, mod.rs, model_deployment.rs (marked for refactoring)
7 files 1000-2000 lines: semantic_retrieval.rs, workspaces.rs, streaming_control.rs,
                         validation.rs, media_optimization.rs, websocket_streaming_optimized.rs
12 files <1000 lines: Smaller focused modules
```

## Implementation Status
- ✅ **Production Ready**: Core API, streaming, deployment, media, configuration
- ⚠️ **Experimental**: Workspaces, semantic retrieval (complete structure, mock data)
- 🚧 **Stub**: Batch operations (structure only)
