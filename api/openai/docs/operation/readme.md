# Operational Procedures Collection

## Collection Scope

This collection documents operational procedures for maintaining, releasing, and managing the api_openai crate. Each operation instance describes a specific procedure, workflow, or process.

**In Scope**:
- Release management procedures (versioning, changelog, publishing)
- Deployment procedures
- Monitoring and observability operations
- Maintenance workflows
- Configuration management procedures

**Out of Scope**:
- Design patterns (belongs in pattern/ collection)
- API protocols (belongs in protocol/ collection)
- Development workflows (may belong in separate development/ collection)

## Responsibility

| Instance | Purpose |
|----------|---------|
| `001_semantic_versioning.md` | Semantic versioning strategy, version number management, and release procedures |

## Overview

| ID | Operation Name | Category | Frequency | Automation |
|----|---------------|----------|-----------|------------|
| 001 | Semantic Versioning | Release Management | Per Release | Manual |

## Collection Guidelines

### Instance Numbering
- Use sequential NNN identifiers (001, 002, 003, ...)
- Never reuse identifiers even after deletion
- Maintain chronological order of creation

### Abstract-First Principle
All operation documentation must:
- Focus on procedure and workflow, not tool-specific commands
- Describe WHAT and WHY, not HOW (implementation)
- Use abstract process notation where possible
- Reference specific tooling in separate implementation notes

### Required Sections per Instance
1. **Operation Name**: Clear, descriptive name
2. **Purpose**: What does this operation accomplish?
3. **Trigger Conditions**: When is this operation performed?
4. **Prerequisites**: What must be true before starting?
5. **Procedure**: Abstract workflow steps
6. **Success Criteria**: How to verify completion?
7. **Rollback Procedure**: How to undo if necessary?
8. **Frequency**: How often is this operation performed?

## Related Collections

- **pattern/**: Design patterns that support these operations
- **protocol/**: Communication protocols used in operations *(when created)*
- **lifecycle/**: Component lifecycles that these operations manage *(when created)*
