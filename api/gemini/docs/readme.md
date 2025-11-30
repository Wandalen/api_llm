# Documentation

## Purpose

This directory contains detailed API documentation, operational guides, and technical references for the api_gemini crate, organized into design collections following documentation.rulebook.md standards.

Documentation is structured by design dimensions (pattern/, api/, operation/, protocol/) with each collection containing:
- Master file (`readme.md`) with Collection Scope and Responsibility Table
- Instance files with NNN identifiers (001, 002, etc.)

## Responsibility

This table documents all entities in the docs/ directory, ensuring Complete Entity Coverage.

| Path | Purpose |
|------|---------  |
| `readme.md` | Master documentation file with navigation and Complete Entity Coverage |
| `pattern/` | Pattern design collection - reusable implementation patterns for common use cases |
| `api/` | API design collection - comprehensive API coverage, endpoints, features, test statistics |
| `operation/` | Operational procedures collection - cookbook reference, usage examples, testing guidelines |
| `protocol/` | Protocol design collection - streaming API format discovery and protocol-level decisions |

## Collections

### pattern/

Implementation patterns and reusable code examples for common Gemini API use cases.

**Master File**: `pattern/readme.md`

**Instances**:
- `001_patterns.md` - Reusable patterns: quick response, error-resilient, batch processing

### api/

API design, endpoints, coverage, and implementation status.

**Master File**: `api/readme.md`

**Instances**:
- `001_coverage.md` - Comprehensive API coverage: core endpoints, advanced features, enterprise capabilities, test statistics

### operation/

Operational procedures for feature usage, cookbook reference, and testing.

**Master File**: `operation/readme.md`

**Instances**:
- `001_cookbook.md` - Official Google Gemini cookbook reference with 64 quickstarts and examples
- `002_usage_examples.md` - Practical code examples demonstrating api_gemini crate usage
- `003_testing.md` - Testing strategy, NO MOCKUP TESTS policy, integration test configuration

### protocol/

Protocol design and implementation decisions at the API communication level.

**Master File**: `protocol/readme.md`

**Instances**:
- `001_streaming_format.md` - Streaming API format discovery, investigation, resolution from SSE to JSON array buffering

## Navigation

**For API Usage**:
- Start with main `readme.md` for quick start
- Reference `api/001_coverage.md` for comprehensive API coverage
- Check `operation/002_usage_examples.md` for practical code examples

**For Implementation Patterns**:
- See `pattern/001_patterns.md` for reusable code patterns
- See `operation/001_cookbook.md` for official Google recipes

**For Testing and Protocol**:
- See `operation/003_testing.md` for testing guidelines and NO MOCKUP policy
- See `protocol/001_streaming_format.md` for streaming implementation insights

**For Implementation Details**:
- See `spec.md` for requirements and architecture
- See source code documentation for implementation decisions
- See `tests/` for usage examples and edge cases

## Documentation Principles

- **API Coverage**: WHAT functionality is available (endpoints, features, status)
- **Usage Examples**: HOW to use the library (code patterns, practical examples)
- **Cookbook**: OFFICIAL recipes and quickstarts from Google Gemini
- **Testing**: QUALITY guidelines (NO MOCKUP policy, integration tests)
- **Protocol**: WHY protocol decisions were made (format discovery, debugging)
- **Collections**: Organized by design dimension (pattern, api, operation, protocol)
- **Abstract First**: Documentation focuses on concepts, not implementation details
- **Complete Coverage**: All documents listed in Responsibility Tables

This separation ensures each document has clear purpose and audience.
