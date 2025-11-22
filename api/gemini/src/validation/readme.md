# Validation Module

Request validation utilities for Gemini API requests.

## Overview

This module provides validation functions for API requests before they are sent to
the Gemini API. Validation catches common errors early, improving error messages
and reducing unnecessary API calls.

## Module Structure

Domain-based validation modules organized by API family:

```
validation/
├── mod.rs       - Module exports and common validation utilities
├── content.rs   - Content generation request validation
├── config.rs    - Configuration validation
├── tokens.rs    - Token counting request validation
└── tuning.rs    - Model tuning request validation
```

## Validation Categories

### Input Validation

- Non-empty required fields
- Valid enum values
- Proper data types
- Field length constraints

### Semantic Validation

- Logical consistency between fields
- Valid combinations of options
- Model capability compatibility

### Safety Validation

- Content safety settings validation
- Threshold value ranges
- Category-specific constraints

## Usage

Validation is applied **automatically** by the client before making API calls.
Developers don't need to call validation functions directly.

### Example (Internal)

```rust
// Client automatically validates before API call
pub async fn generate_content
(
  &self,
  request: &GenerateContentRequest,
) -> Result< GenerateContentResponse >
{
  // Validation happens here automatically
  validate_generate_content_request( request )?;

  // Then make API call
  self.http_client.post( url, request ).await
}
```

## Validation Errors

Validation errors return descriptive error messages via the `Error::InvalidArgument`
variant:

```rust
Error::InvalidArgument
(
  "GenerateContentRequest.contents cannot be empty"
)
```

## Design Principles

### Fail Fast

Validation errors are caught **before** making HTTP calls, saving time and API quota.

### Clear Messages

Error messages specify:
- Which field is invalid
- What the constraint is
- How to fix it

### No Silent Corrections

Validation **never silently corrects** invalid input. All issues are reported
explicitly.

## See Also

- Error types: `../error/`
- API models: `../models/`
- Client implementation: `../client/`
