# Media API Tests

## Purpose
Tests for Gemini Media API functionality including file upload, storage, retrieval, processing, and management operations.

## Organization Principles
- mod.rs: Shared test utilities and unit tests
- basic_operations.rs: Core CRUD operations (upload, list, get, delete, multimodal)
- advanced_features.rs: Advanced file handling (large files, multiple types, search, processing, versioning)
- reliability.rs: Error handling, pagination, and concurrent operations
- Domain-based organization (not methodology-based)

## Navigation Guide
- Basic file operations: basic_operations.rs
- Advanced file handling: advanced_features.rs
- Edge cases and reliability: reliability.rs
- Unit tests and helpers: mod.rs
