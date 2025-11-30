# Async Patterns Documentation

## Overview

This document establishes standardized async patterns for the OpenAI API client to ensure consistency, performance, and maintainability.

## Current Status

The codebase currently uses native `async fn` throughout (76 async functions total), avoiding `async_trait` for better performance. However, there are inconsistencies in return type patterns that need standardization.

## Standardized Async Patterns

### 1. Preferred Pattern: Fully Typed Functions

**✅ Use this pattern for all new implementations:**

```rust
pub async fn create(&self, request: CreateRequestType) -> Result<ResponseType>
{
  self.client.post("endpoint", &request).await
}

pub async fn retrieve(&self, id: &str) -> Result<ResponseType>
{
  let path = format!("endpoint/{id}");
  self.client.get(&path).await
}

pub async fn list(&self, query: Option< QueryType >) -> Result< ListResponseType >
{
  let path = "endpoint";
  if let Some(q) = query
  {
    self.client.get_with_query(&path, &q).await
  }
  else
  {
    self.client.get(&path).await
  }
}
```

### 2. Legacy Pattern: Untyped JSON (To Be Migrated)

**❌ Avoid this pattern (24 functions currently use this):**

```rust
// Legacy pattern - needs migration to typed version
pub async fn create(&self, request: serde_json::Value) -> Result<serde_json::Value>
{
  self.client.post("endpoint", &request).await
}
```

### 3. Streaming Pattern

**✅ Use this pattern for streaming responses:**

```rust
pub async fn create_stream(&self, request: RequestType) -> Result<mpsc::Receiver<Result<EventType>>>
{
  self.client.post_stream("endpoint", &request).await
}
```

## Error Handling Standards

### Consistent Error Propagation

All async functions should use the same error handling pattern:

```rust
pub async fn operation(&self) -> Result<ResponseType>
{
  // Direct error propagation using ?
  let result = self.client.request().await?;
  Ok(result)
}
```

### No Error Conversion

Avoid manual error conversion - let the `?` operator handle error propagation:

```rust
// ✅ Good - automatic error propagation
self.client.post("endpoint", &request).await

// ❌ Bad - manual error handling
match self.client.post("endpoint", &request).await {
  Ok(result) => Ok(result),
  Err(e) => Err(e),
}
```

## Performance Considerations

### Native Async Functions

- **Use**: `async fn` for all async operations
- **Avoid**: `#[async_trait]` unless required for trait objects

Benefits of native async:
- Zero-cost abstractions
- Better compiler optimization
- Clearer error messages
- No heap allocation for futures

### Efficient Stream Handling

For streaming operations, use bounded channels to prevent memory issues:

```rust
pub async fn create_stream(&self, request: RequestType) -> Result<mpsc::Receiver<Result<EventType>>>
{
  // Channel buffer size should be reasonable (e.g., 100-1000)
  self.client.post_stream("endpoint", &request).await
}
```

## Migration Strategy

### Priority Order for Type Migration

1. **High-frequency endpoints** (responses, chat, etc.)
2. **Core functionality** (authentication, models)
3. **Specialized features** (fine-tuning, assistants, etc.)

### Migration Process

1. **Define typed structs** for request/response
2. **Create new typed async methods** alongside existing ones
3. **Update examples and tests** to use typed methods
4. **Mark untyped methods as deprecated**
5. **Remove untyped methods** in next major version

## Implementation Examples

### Fully Typed Implementation

```rust
// responses.rs - Good example of fully typed async pattern
impl<'client, E> Responses<'client, E>
where
  E: OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
{
  pub async fn create(&self, request: CreateResponseRequest) -> Result<ResponseObject>
  {
    self.client.post("responses", &request).await
  }

  pub async fn retrieve(&self, response_id: &str) -> Result<ResponseObject>
  {
    let path = format!("responses/{response_id}");
    self.client.get(&path).await
  }
}
```

### Mixed Implementation (Needs Standardization)

```rust
// vector_stores.rs - Example of mixed patterns that need standardization
impl<'client, E> VectorStores<'client, E>
where
  E: OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
{
  // ❌ Untyped - should be migrated to typed structs
  pub async fn create(&self, request: serde_json::Value) -> Result<serde_json::Value>
  {
    self.client.post("/vector_stores", &request).await
  }
}
```

## Testing Async Functions

### Integration Tests

All async functions should have integration tests following this pattern:

```rust
#[tokio::test]
async fn test_async_operation() -> Result<()>
{
  let client = create_test_client().await?;
  let request = create_test_request();

  let result = client.operation(request).await?;

  assert_valid_response(&result);
  Ok(())
}
```

### Mock Testing

For unit tests, use the existing isolation framework:

```rust
#[tokio::test]
async fn test_operation_with_mock()
{
  let isolated_client = IsolatedClient::new("test_name", false)?;
  let client = isolated_client.client();

  // Test will fail with network error (expected for mock)
  let result = client.operation(request).await;
  assert!(result.is_err());
}
```

## Summary

- **Use native `async fn`** for all async operations (already standardized ✅)
- **Prefer fully typed request/response structs** over `serde_json::Value`
- **Use consistent error propagation** with `?` operator
- **Follow established patterns** for streaming and list operations
- **Migrate untyped functions** to typed equivalents over time

The codebase already uses efficient native async functions. The main standardization needed is migrating from untyped JSON patterns to fully typed patterns for consistency and type safety.