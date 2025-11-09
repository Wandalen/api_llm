# Components

## Purpose

Shared data structures, request/response types, and common components used across OpenAI API endpoint implementations.

## Organization Principles

- **Shared types**: Common types used by multiple API endpoints (common.rs, input.rs, output.rs, query.rs)
- **API-specific shared types**: Endpoint-specific shared structures (e.g., assistants_shared/, realtime_shared/)
- **Request/Response structures**: Data types for API requests and responses
- **Nested organization**: Complex APIs with many types use subdirectories (assistants_shared/, realtime_shared/, tools/)

## Navigation Guide

- For common types used across APIs: `common.rs`
- For input/output types: `input.rs`, `output.rs`
- For query parameters: `query.rs`
- For assistants API types: `assistants_shared/`
- For realtime API types: `realtime_shared/`
- For administration types: `administration_shared.rs`
- For audio types: `audio.rs`
- For batch operations: `batch_shared.rs`
- For chat types: `chat_shared.rs`
- For embeddings types: `embeddings.rs`, `embeddings_request.rs`
- For file operations: `files.rs`
- For fine-tuning: `fine_tuning_shared.rs`
- For images: `images.rs`
- For models: `models.rs`
- For moderations: `moderations.rs`
- For responses API: `responses.rs`
- For tool definitions: `tools/`
- For uploads: `uploads.rs`
- For usage tracking: `usage_shared.rs`
- For vector stores: `vector_stores_shared.rs`
- For audit logs: `audit_logs_shared.rs`
