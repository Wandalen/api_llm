// Type definitions for enhanced error handling
//
// This file contains struct definitions used by enhanced.rs.
// It is included directly via `include!()` macro.

/// Enhanced error with context and classification
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct EnhancedAnthropicError
{
  /// Error type classification
  error_type : ErrorType,
  /// Error message
  message : String,
  /// Error context information
  context : Option< ErrorContext >,
  /// Error classification
  class : ErrorClass,
  /// Error severity
  severity : ErrorSeverity,
  /// Whether error is transient
  is_transient : bool,
  /// Stack trace information
  stack_trace : Vec< String >,
  /// Request correlation ID
  correlation_id : Option< String >,
  /// Request ID
  request_id : Option< String >,
}

/// Error context information
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ErrorContext
{
  /// Operation name
  operation : String,
  /// Request ID
  request_id : String,
  /// Additional context data
  context_data : std::collections::HashMap< String, String >,
  /// Timestamp
  timestamp : chrono::DateTime< chrono::Utc >,
}

/// Timeout error details
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct TimeoutError
{
  /// Timeout type
  timeout_type : TimeoutType,
  /// Timeout duration
  duration : Duration,
  /// Error message
  message : String,
}

/// Network error details
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct NetworkError
{
  /// Network error type
  error_type : NetworkErrorType,
  /// Error message
  message : String,
  /// Additional error details
  details : Option< String >,
}

/// Error parser for API responses
#[ derive( Debug ) ]
pub struct ErrorParser;

/// Error mapper for HTTP status codes
#[ derive( Debug ) ]
pub struct ErrorMapper;

/// Error classifier for categorization
#[ derive( Debug ) ]
pub struct ErrorClassifier;

/// Error recovery strategy analyzer
#[ derive( Debug ) ]
pub struct ErrorRecovery;

/// Recovery strategy information
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct RecoveryStrategy
{
  /// Whether to retry
  should_retry : bool,
  /// Maximum retry attempts
  max_retries : u32,
  /// Backoff strategy
  backoff_strategy : BackoffStrategy,
  /// Base delay
  base_delay : Duration,
  /// Whether user action is required
  requires_user_action : bool,
  /// Suggested action for user
  suggested_action : String,
}

/// Actionable error with remediation steps
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ActionableError
{
  /// Original error message
  message : String,
  /// Remediation steps
  remediation_steps : Vec< String >,
  /// Documentation links
  documentation_links : Vec< String >,
  /// Estimated fix time
  estimated_fix_time : Option< Duration >,
}

/// Batch error aggregation
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct BatchError
{
  /// Individual errors
  errors : Vec< EnhancedAnthropicError >,
  /// Error summary
  summary : String,
  /// Whether any requests succeeded
  has_partial_success : bool,
}

/// Timeout error classifier
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct TimeoutClassification
{
  /// Timeout type
  timeout_type : TimeoutType,
  /// Whether network related
  is_network_related : bool,
  /// Whether server processing related
  is_server_processing_related : bool,
  /// Suggested timeout increase
  suggested_timeout_increase : Duration,
  /// Whether supports streaming fallback
  supports_streaming_fallback : bool,
}

/// Network error classifier
#[ derive( Debug ) ]
pub struct NetworkErrorClassifier;

/// Network error classification
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ allow( clippy::struct_excessive_bools ) ]
pub struct NetworkErrorClassification
{
  /// Error type
  error_type : NetworkErrorType,
  /// Whether infrastructure related
  is_infrastructure_related : bool,
  /// Whether security related
  is_security_related : bool,
  /// Whether service availability related
  is_service_availability_related : bool,
  /// Suggested actions
  suggested_actions : Vec< String >,
  /// Whether supports retry with backoff
  supports_retry_with_backoff : bool,
}

/// Backoff calculator for rate limiting
#[ derive( Debug ) ]
pub struct BackoffCalculator;

/// Backoff strategy details
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct BackoffStrategyDetails
{
  /// Initial delay
  initial_delay : Duration,
  /// Backoff type
  backoff_type : BackoffType,
  /// Maximum retries
  max_retries : u32,
  /// Whether jitter is enabled
  jitter_enabled : bool,
  /// Suggested batch size reduction
  suggested_batch_size_reduction : Option< f32 >,
}

/// Credential hint generator
#[ derive( Debug ) ]
pub struct CredentialHintGenerator;

/// Credential hints for authentication errors
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct CredentialHints
{
  /// Remediation steps
  remediation_steps : Vec< String >,
  /// Whether has format example
  has_format_example : bool,
  /// Documentation link
  documentation_link : Option< String >,
  /// Whether requires renewal
  requires_renewal : bool,
  /// Renewal instructions
  renewal_instructions : Option< String >,
  /// Estimated resolution time
  estimated_resolution_time : Option< Duration >,
  /// Whether permission upgrade required
  permission_upgrade_required : bool,
  /// Required permissions
  required_permissions : Option< Vec< String > >,
  /// Upgrade instructions
  upgrade_instructions : Option< String >,
}

/// Error serializer
#[ derive( Debug ) ]
pub struct ErrorSerializer;

/// Error logger with structured data
#[ derive( Debug ) ]
pub struct ErrorLogger;

/// Log entry with structured data
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct LogEntry
{
  /// Log message
  message : String,
  /// Log severity
  severity : LogSeverity,
  /// Structured data
  structured_data : std::collections::HashMap< String, serde_json::Value >,
  /// Timestamp
  timestamp : chrono::DateTime< chrono::Utc >,
}

/// Error metrics reporter
#[ derive( Debug ) ]
pub struct ErrorMetrics
{
  /// Error counts by type
  error_counts : std::collections::HashMap< String, u64 >,
  /// Alert thresholds
  alert_thresholds : std::collections::HashMap< String, u64 >,
}

/// Correlation tracker for cross-request errors
#[ derive( Debug ) ]
pub struct CorrelationTracker;

/// Correlation summary
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct CorrelationSummary
{
  /// Total requests
  total_requests : u32,
  /// Failed requests
  failed_requests : u32,
  /// Error patterns
  error_patterns : Vec< String >,
}

/// Error localizer for multi-language support
#[ derive( Debug ) ]
pub struct ErrorLocalizer;

/// Localized error message
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct LocalizedError
{
  /// Localized message
  message : String,
  /// Locale
  locale : String,
}

/// Custom error type for domain-specific errors
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct CustomError
{
  /// Error name
  name : String,
  /// Error message
  message : String,
  /// Error severity
  severity : ErrorSeverity,
}

/// Error chain for cause relationships
#[ derive( Debug, Clone ) ]
pub struct ErrorChain
{
  /// Primary error
  primary : CustomError,
  /// Causing errors
  causes : Vec< AnthropicError >,
  /// Context
  context : String,
}

/// Chained error result
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ChainedError
{
  /// Chain length
  chain_length : u32,
  /// Root cause
  root_cause : String,
  /// Immediate cause
  immediate_cause : String,
  /// Context
  context : String,
}

/// Request context for operations
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct RequestContext
{
  /// Correlation ID
  correlation_id : String,
  /// Request sequence
  request_sequence : u32,
}
