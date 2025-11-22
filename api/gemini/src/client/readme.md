# Client Module

High-level client implementation for the Gemini API.

## Overview

This module provides the main `Client` struct and related functionality for interacting
with Google's Gemini API. It implements the "Thin Client, Rich API" principle with
process-stateless design.

## Module Structure

```
client/
├── mod.rs              - Public module interface and exports
├── core.rs             - Core Client struct and HTTP operations (1,148 lines)
├── api_interfaces.rs   - API method signatures and implementations (2,181 lines)
├── builder.rs          - Builder pattern for client configuration
├── config.rs           - Client configuration management
└── sync.rs             - Synchronous client wrapper for blocking operations
```

## Key Components

### Client (core.rs)

The main async client implementation with:
- HTTP client management (reqwest-based)
- API key authentication
- Base URL configuration
- Request/response handling
- Feature-gated enterprise capabilities (retry, circuit breaker, rate limiting)

### Builder Pattern (builder.rs)

Fluent API for client configuration:
- API key configuration
- Base URL customization
- Timeout settings
- Feature-specific configuration (retry policies, circuit breaker thresholds)

### API Interfaces (api_interfaces.rs)

All API method implementations organized by endpoint family:
- Models API (list, get model info)
- Content Generation API
- Embeddings API
- Token Counting API
- And more...

### Synchronous Client (sync.rs)

Blocking wrapper around async client for synchronous contexts.

## Usage Examples

### Async Client (Recommended)

```rust
use api_gemini::Client;

#[tokio::main]
async fn main() -> Result< (), Box< dyn std::error::Error > >
{
  // Create client (loads API key from environment or workspace secrets)
  let client = Client::new()?;

  // Use API
  let models = client.models().list().await?;

  Ok( () )
}
```

### Builder Pattern

```rust
use api_gemini::Client;

let client = Client::builder()
  .api_key( "your-api-key".to_string() )
  .base_url( "https://custom-endpoint.googleapis.com".to_string() )
  .timeout( std::time::Duration::from_secs( 30 ) )
  .build()?;
```

### Synchronous Client

```rust
use api_gemini::Client;

fn main() -> Result< (), Box< dyn std::error::Error > >
{
  let client = Client::new()?.into_sync();

  // Blocking API calls
  let models = client.models().list()?;

  Ok( () )
}
```

## Architecture Principles

### Process-Stateless Design

The client maintains **runtime state** only (connection pools, circuit breaker state,
rate limiting counters). All state dies when the process terminates.

**Allowed** (runtime):
- Connection pools
- Circuit breaker state (in-memory)
- Rate limiting token buckets
- Temporary retry counters

**Prohibited** (persistent):
- File-based storage
- Database persistence
- Configuration that survives restart

### Thin Client Principle

Every method maps directly to a Gemini API endpoint with:
- No automatic behaviors
- No hidden decision-making
- Explicit configuration for all enterprise features
- Transparent method naming (e.g., `execute_with_retries()`)

## Feature Gates

Enterprise features are **explicitly configured** via Cargo features:

- `retry` - Exponential backoff retry logic
- `circuit_breaker` - Failure threshold management
- `rate_limiting` - Request throttling
- `failover` - Multi-endpoint support
- `health_checks` - Endpoint health monitoring

All features have **zero overhead** when disabled.

## Testing

See `tests/` directory for comprehensive integration tests using real API calls
(no mocking policy).

## See Also

- Main crate documentation: `../lib.rs`
- API models: `../models/`
- Error handling: `../error/`
- HTTP layer: `../internal/http/`
