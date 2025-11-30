# docs

## Purpose

This directory contains supplementary technical documentation for the Claude API client, organized into design collections following documentation.rulebook.md standards.

Documentation is structured by design dimensions (operation/, pattern/, protocol/, etc.) with each collection containing:
- Master file (`readme.md`) with Collection Scope and Responsibility Table
- Instance files with NNN identifiers (001, 002, etc.)

## Responsibility

This table documents all entities in the docs/ directory, ensuring Complete Entity Coverage.

| Path | Purpose |
|------|---------|
| `readme.md` | Master documentation file with navigation and Complete Entity Coverage |
| `operation/` | Operational procedures collection - authentication, configuration, secret management |

## Collections

### operation/

Operational procedures for Claude API client.

**Master File**: `operation/readme.md`

**Instances**:
- `001_secret_loading.md` - Secret loading, authentication, and credential management procedures

## Navigation

For operational procedures (authentication, secret loading): see `operation/`

## Collection Organization Principles

Per documentation.rulebook.md:
- **Dimension-based structure**: Collections organized by design dimension (operation, pattern, protocol, etc.)
- **Instance granularity**: Each design concept in separate NNN-prefixed file
- **Master files required**: Each collection has readme.md with Scope, Responsibility, Overview
- **Complete Entity Coverage**: All files and directories documented in Responsibility Tables
