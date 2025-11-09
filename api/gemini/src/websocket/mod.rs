//! WebSocket streaming implementation for Gemini API integration.
//!
//! This module provides optimized WebSocket streaming functionality that integrates with
//! the Gemini API endpoints for real-time bidirectional communication. It builds upon
//! the models in `websocket_streaming.rs` to provide complete WebSocket integration
//! with advanced optimizations including:
//!
//! - Connection pooling and intelligent reuse strategies
//! - High-performance message serialization with multiple format options
//! - Advanced metrics collection and performance monitoring
//! - Sophisticated error handling and automatic recovery mechanisms
//! - Resource-efficient connection management with auto-scaling
//! - Comprehensive configuration options for fine-tuning performance

mod protocol;
mod connection;
mod streaming;

pub use protocol::
{
  StreamDirection,
  StreamControl,
  WebSocketStreamMessage,
  StreamSessionState,
  SessionMetrics,
  EnhancedStreamingMetrics,
  PerformanceBenchmarks,
};

pub use connection::
{
  WebSocketConnectionManager,
  WebSocketStreamSession,
  StreamController,
};

pub use streaming::
{
  WebSocketStreamBuilder,
  WebSocketStreamingApi,
  EnhancedWebSocketStreamBuilder,
  EnhancedConnectionResult,
};
