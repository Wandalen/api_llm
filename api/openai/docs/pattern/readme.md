# Design Patterns Collection

## Collection Scope

This collection documents reusable design patterns used throughout the api_openai crate. Each pattern instance describes a specific architectural or implementation pattern that solves recurring design challenges.

**In Scope**:
- Async programming patterns and concurrency strategies
- Builder patterns using Former derive macros
- Client architecture patterns
- Error handling patterns

**Out of Scope**:
- Implementation details (belongs in source code comments)
- API-specific protocols (belongs in protocol/ collection)
- Operational procedures (belongs in operation/ collection)

## Responsibility

| Instance | Purpose |
|----------|---------|
| `001_async_patterns.md` | Async programming patterns, tokio best practices, and concurrency strategies |

## Overview

| ID | Pattern Name | Category | Complexity | Status |
|----|-------------|----------|------------|--------|
| 001 | Async Patterns | Concurrency | Medium | Active |

## Collection Guidelines

### Instance Numbering
- Use sequential NNN identifiers (001, 002, 003, ...)
- Never reuse identifiers even after deletion
- Maintain chronological order of creation

### Abstract-First Principle
All pattern documentation must:
- Focus on conceptual understanding, not implementation
- Avoid language-specific code examples in main documentation
- Use pseudocode or abstract notation where needed
- Reference actual implementations via links to source code

### Required Sections per Instance
1. **Pattern Name**: Clear, descriptive name
2. **Intent**: What problem does this pattern solve?
3. **Motivation**: Why is this pattern needed?
4. **Structure**: Abstract structure description
5. **Participants**: Key components and their roles
6. **Collaborations**: How components interact
7. **Consequences**: Trade-offs and considerations
8. **Known Uses**: Where this pattern is applied in the crate

## Related Collections

- **operation/**: Operational procedures that may use these patterns
- **api/**: API contracts that implement these patterns *(when created)*
- **protocol/**: Communication protocols that leverage these patterns *(when created)*
