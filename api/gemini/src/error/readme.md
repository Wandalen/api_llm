# Error Handling

## Purpose
Comprehensive error type definitions and conversions for the api_gemini crate. This module provides a unified error enum that covers all possible failure scenarios when interacting with the Gemini API, including network errors, authentication failures, rate limiting, and API-specific errors.

## Organization Principles
- **Single Error Enum**: All error variants defined in `mod.rs` using the error_tools crate
- **Feature-Gated Variants**: Optional error types enabled by cargo features (circuit_breaker, caching, rate_limiting)
- **Standard Conversions**: From trait implementations for std::io::Error, serde_json::Error, and reqwest::Error
- **API Error Structures**: Structured error responses from Gemini API with ApiErrorResponse and ApiErrorDetails

## Navigation Guide
- **Core Error Enum**: `mod.rs` - The Error enum with 18+ variants covering all failure modes
- **API Error Types**: `mod.rs` - ApiErrorResponse and ApiErrorDetails for structured API errors
- **Error Conversions**: `mod.rs` - From trait implementations for automatic error conversion
- **Feature-Gated Errors**: See variants marked with `#[cfg(feature = "...")]` for optional error types

## Key Error Categories
- **Network**: NetworkError, TimeoutError
- **Authentication**: AuthenticationError (401/403)
- **API Errors**: ApiError, RateLimitError, ServerError, InvalidArgument
- **Serialization**: SerializationError, DeserializationError
- **Request Building**: RequestBuilding
- **Resource Access**: NotFound
- **Enterprise Features**: CircuitBreakerOpen, CacheError, RateLimited (feature-gated)
