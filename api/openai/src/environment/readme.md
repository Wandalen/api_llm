# Environment Configuration

## Purpose

Environment configuration abstractions for API clients, including base URLs, authentication, and custom endpoints.

## Organization Principles

- **Environment traits**: Trait definitions for environment configuration
- **Default implementations**: Standard environment configurations for OpenAI API
- **Custom endpoints**: Support for Azure OpenAI, OpenAI-compatible APIs, and corporate proxies
- **Authentication**: API key management and authentication configuration

## Navigation Guide

- For environment trait definitions and implementations: `mod.rs`
- For configuring official OpenAI endpoints: `OpenaiEnvironmentImpl` in `mod.rs`
- For custom base URL configuration: Environment builder methods
- For Azure OpenAI configuration: Environment with custom base URLs
