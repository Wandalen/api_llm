# Documentation

This directory contains detailed API documentation and feature reference for the api_huggingface crate.

## Organization

Documentation is organized into focused files:

- `api_reference.md` - Comprehensive API documentation covering:
  - Client operations (Text Generation, Embeddings, Model Management)
  - Popular models constants
  - Environment configuration
  - Error handling patterns
  - Parameters and options
  - Response types
  - Advanced features (Sync API, CURL diagnostics)

- `features.md` - Complete feature tables and cargo features:
  - Feature status tracking (Core APIs, Enterprise Features, Multimodal APIs)
  - Feature tier classification (Tier 1 vs Tier 2)
  - Not implemented features list
  - Cargo features documentation with examples

## Navigation

**For API Usage**:
- Start with main `readme.md` for quick start
- Reference `api_reference.md` for detailed API patterns
- Check `features.md` for feature availability

**For Implementation Details**:
- See `spec.md` for requirements and architecture
- See source code documentation for implementation decisions
- See `tests/` for usage examples and edge cases

## Documentation Principles

- **API Reference**: HOW to use the library (usage patterns, examples)
- **Features**: WHAT functionality is available (status, cargo flags)
- **Specification**: WHY design decisions were made (requirements, architecture)

This separation ensures each document has clear purpose and audience.
