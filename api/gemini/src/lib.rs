//! # Gemini API Client for Rust
//!
//! A Rust client library for Google's Gemini API.
//!
//! ## Governing Principle : "Thin Client, Rich API"
//!
//! This client exposes all server-side functionality transparently while maintaining
//! zero client-side intelligence or automatic behaviors. The client is a transparent
//! window to the Gemini API, not a smart assistant that makes decisions for developers.
//!
//! Key principles:
//! - **API Transparency**: One-to-one mapping with Gemini API endpoints
//! - **Zero Client Intelligence**: No automatic behaviors or magic thresholds
//! - **Explicit Control**: Developer decides when, how, and why operations occur
//! - **Information vs Action**: Clear separation between data retrieval and state changes
//!
//! ## Enterprise Reliability Features
//!
//! The following enterprise reliability features are **explicitly allowed** when implemented
//! with explicit configuration and transparent operation:
//!
//! - **Configurable Retry Logic**: Exponential backoff with explicit configuration
//! - **Circuit Breaker Pattern**: Failure threshold management with transparent state
//! - **Rate Limiting**: Request throttling with explicit rate configuration
//! - **Failover Support**: Multi-endpoint configuration and automatic switching
//! - **Health Checks**: Periodic endpoint health verification and monitoring
//!
//! ## State Management Policy
//!
//! **✅ ALLOWED: Runtime-Stateful, Process-Stateless**
//! - Connection pools, circuit breaker state, rate limiting buckets
//! - Retry logic state, failover state, health check state
//! - Runtime state that dies with the process
//! - No persistent storage or cross-process state
//!
//! **❌ PROHIBITED: Process-Persistent State**
//! - File storage, databases, configuration accumulation
//! - State that survives process restarts
//!
//! **Implementation Requirements**:
//! - Feature gating behind cargo features (`retry`, `circuit_breaker`, `rate_limiting`, `failover`, `health_checks`)
//! - Explicit configuration required (no automatic enabling)
//! - Transparent method naming (e.g., `execute_with_retries()`, `execute_with_circuit_breaker()`)
//! - Zero overhead when features disabled

#![ doc( html_root_url = "https://docs.rs/api_gemini/latest/api_gemini/" ) ]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]

/// Client module containing the main Client struct and builder pattern
pub mod client;

/// Models module containing all API request and response data structures
pub mod models;

/// Cached content API implementation
/// Error handling types and utilities
pub mod error;

/// Internal implementation details (exposed for testing)
pub mod internal;

/// Diagnostics module for debugging and development tools
#[ cfg( feature = "diagnostics_curl" ) ]
pub mod diagnostics;

/// Input validation utilities for API requests
pub mod validation;

/// WebSocket streaming integration for real-time bidirectional communication
#[ cfg( feature = "websocket_streaming" ) ]
pub mod websocket;

/// Batch Mode API for async job-based processing with 50% cost discount
#[ cfg( feature = "batch_operations" ) ]
pub mod batch_api;

/// Enterprise features for production deployments
#[ cfg( feature = "enterprise_quota" ) ]
pub mod enterprise;

/// Model comparison and A/B testing functionality
#[ cfg( feature = "model_comparison" ) ]
pub mod comparison;

/// Request templates for common use cases
#[ cfg( feature = "request_templates" ) ]
pub mod templates;

/// Buffered streaming for smoother UX
#[ cfg( feature = "buffered_streaming" ) ]
pub mod buffered_streaming;

// Re-export key types at the top level for easier access
pub use models::*;

// Re-export compression types when feature is enabled
#[ cfg( feature = "compression" ) ]
pub use internal::http::compression::{ CompressionConfig, CompressionAlgorithm };

// Re-export cost quota types when feature is enabled
#[ cfg( feature = "enterprise_quota" ) ]
pub use enterprise::
{
  CostQuotaManager,
  CostQuotaConfig,
  CostQuotaExceededError,
  ModelPricing,
  CostUsageMetrics,
};

// Re-export diagnostic types when feature is enabled
#[ cfg( feature = "diagnostics_curl" ) ]
pub use diagnostics::{ InlineData, CurlOptions };

// Re-export streaming control types when feature is enabled
#[ cfg( feature = "streaming_control" ) ]
pub use models::streaming_control::
{
  StreamState,
  StreamControlConfig,
  StreamControlConfigBuilder,
  StreamMetrics,
  StreamMetricsSnapshot,
  BufferStrategy,
  MetricsLevel,
  ControllableStream,
  ControllableStreamBuilder,
};
