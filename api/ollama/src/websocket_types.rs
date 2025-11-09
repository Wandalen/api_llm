// WebSocket type definitions.
//
// Note : This file is included via include!() in websocket.rs
// All imports are done in the parent module.

// =====================================
// WebSocket Streaming Types
// =====================================

/// WebSocket connection state
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum WebSocketState
{
  /// WebSocket is disconnected
  Disconnected,
  /// WebSocket is in the process of connecting
  Connecting,
  /// WebSocket is connected and ready for communication
  Connected,
  /// WebSocket is reconnecting after a disconnection
  Reconnecting,
  /// WebSocket is in an error state
  Error,
}

/// WebSocket-specific error types
#[ derive( Debug ) ]
pub enum WebSocketError
{
  /// Connection failed
  ConnectionFailed( String ),
  /// Protocol error
  ProtocolError( String ),
  /// Authentication failed
  AuthenticationFailed( String ),
  /// Compression error
  CompressionError( String ),
  /// Message queue overflow
  QueueOverflow,
  /// Heartbeat timeout
  HeartbeatTimeout,
  /// Invalid message format
  InvalidMessage( String ),
  /// Connection pool exhausted
  PoolExhausted,
  /// Streaming error with message and optional error code
  StreamingError {
    /// Error message
    message : String,
    /// Optional error code
    code : Option< u16 >
  },
  /// Generic WebSocket error
  Generic( String ),
}

/// WebSocket connection metrics
#[ derive( Debug, Clone ) ]
pub struct WebSocketMetrics
{
  /// Total messages sent
  pub messages_sent : u64,
  /// Total messages received
  pub messages_received : u64,
  /// Connection uptime in seconds
  pub uptime_seconds : u64,
  /// Number of reconnection attempts
  pub reconnection_attempts : u32,
  /// Average message latency in milliseconds
  pub average_latency_ms : u32,
  /// Current queue size
  pub queue_size : usize,
  /// Number of compression errors
  pub compression_errors : u32,
  /// Number of heartbeat failures
  pub heartbeat_failures : u32,
  /// Total bytes sent
  pub bytes_sent : u64,
  /// Total bytes received
  pub bytes_received : u64,
  /// Number of heartbeats sent
  pub heartbeat_count : u64,
  /// Number of reconnection attempts made
  pub reconnect_count : u32,
  /// Connection uptime as Duration
  pub uptime : core::time::Duration,
  /// Compression ratio (0.0 to 1.0)
  pub compression_ratio : f64,
  /// Creation timestamp for uptime calculation
  pub created_at : std::time::Instant,
}

/// WebSocket configuration
#[ derive( Debug, Clone ) ]
pub struct WebSocketConfig
{
  /// WebSocket server URL
  pub url : String,
  /// Connection timeout
  pub timeout : Duration,
  /// Maximum reconnection attempts
  pub max_reconnection_attempts : u32,
  /// Reconnection delay
  pub reconnection_delay : Duration,
  /// Enable compression
  pub enable_compression : bool,
  /// Heartbeat interval
  pub heartbeat_interval : Duration,
  /// Maximum queue size
  pub max_queue_size : usize,
  /// Authentication token
  pub auth_token : Option< String >,
  /// Enable automatic reconnection
  pub enable_auto_reconnect : bool,
  /// Connection pool size
  pub pool_size : usize,
  /// HTTP fallback URL
  pub http_fallback_url : Option< String >,
}

/// Message queue entry for reliable delivery
#[ derive( Debug, Clone ) ]
pub struct QueuedMessage
{
  /// Unique message ID
  pub id : String,
  /// Message content
  pub content : String,
  /// Message priority (higher = more important)
  pub priority : u32,
  /// Timestamp when message was queued
  pub timestamp : Instant,
  /// Number of retry attempts
  pub retry_count : u32,
  /// Maximum retry attempts for this message
  pub max_retries : u32,
}

/// Message queue for reliable delivery
#[ derive( Debug ) ]
pub struct MessageQueue
{
  /// Queue storage (priority queue)
  pub queue : Arc< Mutex< Vec< QueuedMessage > > >,
  /// Maximum queue size
  pub max_size : usize,
  /// Queue metrics
  pub metrics : Arc< RwLock< WebSocketMetrics > >,
}

/// WebSocket connection pool entry
#[ derive( Debug ) ]
pub struct PooledConnection
{
  /// Connection ID
  pub id : String,
  /// Connection state
  pub state : Arc< RwLock< WebSocketState > >,
  /// Connection establishment time
  pub established_at : Instant,
  /// Last activity timestamp
  pub last_activity : Arc< RwLock< Instant > >,
  /// Number of active streams
  pub active_streams : Arc< RwLock< u32 > >,
  /// Connection-specific metrics
  pub metrics : Arc< RwLock< WebSocketMetrics > >,
}

/// WebSocket connection pool for connection reuse
#[ derive( Debug ) ]
pub struct ConnectionPool
{
  /// Pool of connections
  pub connections : Arc< RwLock< Vec< PooledConnection > > >,
  /// Maximum pool size
  pub max_size : usize,
  /// Connection timeout before cleanup
  pub connection_timeout : Duration,
  /// Pool metrics
  pub metrics : Arc< RwLock< WebSocketMetrics > >,
}

/// Authentication context for WebSocket connections
#[ derive( Debug, Clone ) ]
pub struct WebSocketAuth
{
  /// Authentication token
  pub token : Option< String >,
  /// Token expiry time
  pub expires_at : Option< Instant >,
  /// Refresh token
  pub refresh_token : Option< String >,
  /// Authentication scheme (Bearer, Basic, etc.)
  pub scheme : String,
}

/// WebSocket authentication method
#[ derive( Debug, Clone ) ]
pub enum WebSocketAuthMethod
{
  /// Bearer token authentication
  Bearer( String ),
  /// Basic authentication with username and password
  Basic {
    /// Username for authentication
    username : String,
    /// Password for authentication
    password : String
  },
  /// API key authentication
  ApiKey( String ),
  /// Custom authentication
  Custom( String ),
  /// Bearer token authentication (alias)
  BearerToken( String ),
  /// No authentication
  None,
}

/// Connection type for fallback scenarios
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum ConnectionType
{
  /// WebSocket connection
  WebSocket,
  /// HTTP fallback connection
  HttpFallback,
}

/// Authentication status
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum AuthStatus
{
  /// Not authenticated
  NotAuthenticated,
  /// Authenticated successfully
  Authenticated,
  /// Authentication failed
  Failed,
}

/// WebSocket error handling strategy
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum WebSocketErrorHandling
{
  /// Fail fast on any error
  FailFast,
  /// Resilient error handling with retries
  Resilient,
  /// Ignore non-critical errors
  Ignore,
}

/// Recovery status after connection issues
#[ derive( Debug, Clone ) ]
pub struct RecoveryStatus
{
  /// Number of errors encountered
  pub error_count : u32,
  /// Number of recovery attempts made
  pub recovery_attempts : u32,
  /// Whether recovery is complete
  pub is_recovered : bool,
}

/// WebSocket pool for managing multiple connections
#[ derive( Debug ) ]
pub struct WebSocketPool
{
  /// Pool configuration
  pub config : WebSocketPoolConfig,
  /// Connection pool
  pub pool : ConnectionPool,
  /// Number of active connections
  pub active_connections : std::sync::Arc< std::sync::Mutex< usize > >,
  /// Cached connections by URL
  pub connections : std::sync::Arc< std::sync::Mutex< std::collections::HashMap<  String, WebSocketConnection  > > >,
}

/// Pool statistics
#[ derive( Debug, Clone ) ]
pub struct PoolStatistics
{
  /// Number of active connections
  pub active_connections : usize,
  /// Number of idle connections
  pub idle_connections : usize,
  /// Total number of connections
  pub total_connections : usize,
  /// Length of the connection queue
  pub queue_length : usize,
}

/// WebSocket client implementation
#[ derive( Debug ) ]
pub struct WebSocketClient
{
  /// Configuration
  pub config : WebSocketConfig,
  /// Current connection state
  pub state : Arc< RwLock< WebSocketState > >,
  /// Message queue for reliable delivery
  pub message_queue : MessageQueue,
  /// Connection pool
  pub connection_pool : ConnectionPool,
  /// Authentication context
  pub auth : Option< WebSocketAuth >,
  /// Client metrics
  pub metrics : Arc< RwLock< WebSocketMetrics > >,
  /// HTTP client for fallback
  pub http_client : Option< reqwest::Client >,
}

/// WebSocket connection wrapper
#[ derive( Debug, Clone ) ]
pub struct WebSocketConnection
{
  /// Connection ID
  pub id : String,
  /// Connection state
  pub state : Arc< RwLock< WebSocketState > >,
  /// Associated client
  pub client_id : String,
  /// Connection establishment time
  pub established_at : Instant,
  /// Last message timestamp
  pub last_message_at : Arc< RwLock< Instant > >,
  /// Connection metrics
  pub metrics : Arc< RwLock< WebSocketMetrics > >,
  /// Connection type (WebSocket or HTTP fallback)
  pub connection_type : ConnectionType,
}

/// WebSocket message types
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub enum WebSocketMessage
{
  /// Text message
  Text( String ),
  /// Binary message
  Binary( Vec< u8 > ),
  /// Ping message
  Ping( Vec< u8 > ),
  /// Pong message
  Pong( Vec< u8 > ),
  /// Close message
  Close( Option< ( u16, String ) > ),
}

/// WebSocket pool configuration
#[ derive( Debug, Clone ) ]
pub struct WebSocketPoolConfig
{
  /// Maximum connections in pool
  pub max_connections : usize,
  /// Minimum connections to maintain
  pub min_connections : usize,
  /// Connection timeout
  pub connection_timeout : Duration,
  /// Enable connection multiplexing
  pub enable_multiplexing : bool,
}

/// Queue information for WebSocket connections
#[ derive( Debug, Clone ) ]
pub struct QueueInfo
{
  /// Current queue size
  pub size : usize,
  /// Maximum queue capacity
  pub capacity : usize,
  /// Number of pending messages
  pub pending_messages : usize,
}

/// WebSocket chat stream for real-time conversation
#[ derive( Debug ) ]
pub struct WebSocketChatStream
{
  /// Connection ID
  pub connection_id : String,
  /// Chat request
  pub request : ChatRequest,
  /// Stream state
  pub state : Arc< RwLock< WebSocketState > >,
}
