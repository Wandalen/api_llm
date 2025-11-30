# Documentation

## Purpose

This directory contains detailed API documentation and feature reference for the api_huggingface crate, organized into design collections following documentation.rulebook.md standards.

Documentation is structured by design dimensions (api/, operation/, etc.) with each collection containing:
- Master file (`readme.md`) with Collection Scope and Responsibility Table
- Instance files with NNN identifiers (001, 002, etc.)

## Responsibility

This table documents all entities in the docs/ directory, ensuring Complete Entity Coverage.

| Path | Purpose |
|------|---------|
| `readme.md` | Master documentation file with navigation and Complete Entity Coverage |
| `api/` | API design collection - comprehensive API reference, endpoints, usage patterns |
| `operation/` | Operational procedures collection - feature management, cargo features, status tracking |

## Collections

### api/

API design, endpoints, and usage patterns for the api_huggingface crate.

**Master File**: `api/readme.md`

**Instances**:
- `001_reference.md` - Comprehensive API reference covering client operations, models, environment config, error handling

### operation/

Operational procedures for feature management and configuration.

**Master File**: `operation/readme.md`

**Instances**:
- `001_features.md` - Complete feature tables, cargo features documentation, feature tier classification

## Navigation

**For API Usage**:
- Start with main `readme.md` for quick start
- Reference `api/001_reference.md` for detailed API patterns
- Check `operation/001_features.md` for feature availability

**For Implementation Details**:
- See `spec.md` for requirements and architecture
- See source code documentation for implementation decisions
- See `tests/` for usage examples and edge cases

## Documentation Principles

- **API Reference**: HOW to use the library (usage patterns, examples)
- **Features**: WHAT functionality is available (status, cargo flags)
- **Specification**: WHY design decisions were made (requirements, architecture)
- **Collections**: Organized by design dimension (api, operation, pattern, protocol, etc.)
- **Abstract First**: Documentation focuses on concepts, not implementation details
- **Complete Coverage**: All documents listed in Responsibility Tables

This separation ensures each document has clear purpose and audience.
