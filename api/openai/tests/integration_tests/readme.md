# Integration Tests

## Purpose

Real API integration tests that validate OpenAI API interactions with actual API calls.

## Organization Principles

- **Environment setup**: Integration test environment configuration (environment.rs)
- **Response operations**: Tests for response creation and streaming (response_creation.rs, response_management.rs)
- **Shared utilities**: Common integration test helpers (shared.rs)
- **Domain organization**: Tests organized by API domain/feature

## Navigation Guide

- For environment setup and configuration: `environment.rs`
- For response creation tests: `response_creation.rs`
- For response management tests: `response_management.rs`
- For shared integration test utilities: `shared.rs`
- For module organization: `mod.rs`
