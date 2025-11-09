# Internal Implementation

## Purpose
Internal HTTP implementation layer with enterprise-grade reliability patterns. This module provides the core HTTP execution engine with optional circuit breaker, rate limiting, and retry logic for production-ready API clients.

## Organization Principles
- **Single Module Design**: All HTTP logic consolidated in `http.rs` (1,476 lines)
- **Feature-Gated Implementations**: Enterprise reliability patterns enabled via cargo features
- **Zero Overhead**: Features compile to zero overhead when disabled
- **Thread-Safe State Management**: Uses Arc<Mutex<>> for shared state across concurrent requests

## Navigation Guide
- **Core HTTP Execution**: `http.rs:142-253` - Base execute() function with logging and error handling
- **Request Building**: `http.rs:262-324` - build_request() with serialization and headers
- **Response Processing**: `http.rs:372-434` - process_response() with error classification
- **Retry Logic** (feature = "retry"): `http.rs:584-794` - Exponential backoff with jitter
- **Circuit Breaker** (feature = "circuit_breaker"): `http.rs:796-1048` - State machine (Closed/Open/HalfOpen)
- **Rate Limiting** (feature = "rate_limiting"): `http.rs:1050-1281` - Token bucket & sliding window algorithms
- **Unified Execution**: `http.rs:1313-1475` - Enterprise features integration

## Implementation Highlights

### Retry Logic (210 lines)
- Exponential backoff with configurable multiplier
- Optional jitter to prevent thundering herd
- Max elapsed time limiting
- Smart retryable error classification

### Circuit Breaker (253 lines)
- Three-state machine: Closed → Open → HalfOpen
- Configurable failure/success thresholds
- Automatic timeout-based recovery
- Comprehensive metrics collection

### Rate Limiting (231 lines)
- Two algorithms: Token Bucket (burst support) and Sliding Window
- Configurable requests per second
- Thread-safe token refill
- Real-time metrics tracking

## Performance Characteristics
- **Low Overhead**: <5ms per request when reliability features disabled
- **Minimal Allocations**: Efficient request/response handling
- **Concurrent Safe**: All state protected by Arc<Mutex<>>
