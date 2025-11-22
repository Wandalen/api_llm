//! Structured logging support for Ollama API client.
//!
//! Provides tracing integration for observability, debugging, and performance monitoring.

#[ cfg( feature = "structured_logging" ) ]
mod private
{
  use error_tools::untyped::Result;

  /// Logging configuration for structured events
  #[ derive( Debug, Clone ) ]
  pub struct LoggingConfig
  {
    /// Minimum log level (trace, debug, info, warn, error)
    pub level : String,
    /// Output format for logs
    pub format : LogFormat,
    /// Whether to emit correlation IDs for request tracking
    pub emit_correlation_ids : bool,
    /// Whether to include span timing information
    pub include_timing : bool,
    /// Whether to log request/response bodies (may contain sensitive data)
    pub log_bodies : bool,
  }

  /// Log output format options
  #[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
  pub enum LogFormat
  {
    /// JSON format for structured log aggregation
    Json,
    /// Pretty-printed format for human readability
    Pretty,
    /// Compact format for minimal output
    Compact,
  }

  /// Request context for correlation tracking
  #[ derive( Debug, Clone ) ]
  pub struct RequestContext
  {
    /// Unique request identifier
    pub request_id : String,
    /// Model being used
    pub model : Option< String >,
    /// Operation type (chat, generate, embeddings, etc.)
    pub operation : String,
    /// Start timestamp
    pub started_at : std::time::Instant,
  }

  impl LoggingConfig
  {
    /// Create new logging configuration with defaults
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        level : "info".to_string(),
        format : LogFormat::Pretty,
        emit_correlation_ids : true,
        include_timing : true,
        log_bodies : false,
      }
    }

    /// Set log level
    #[ inline ]
    #[ must_use ]
    pub fn with_level( mut self, level : &str ) -> Self
    {
      self.level = level.to_string();
      self
    }

    /// Set log format
    #[ inline ]
    #[ must_use ]
    pub fn with_format( mut self, format : LogFormat ) -> Self
    {
      self.format = format;
      self
    }

    /// Enable or disable correlation IDs
    #[ inline ]
    #[ must_use ]
    pub fn with_correlation_ids( mut self, enable : bool ) -> Self
    {
      self.emit_correlation_ids = enable;
      self
    }

    /// Enable or disable span timing
    #[ inline ]
    #[ must_use ]
    pub fn with_timing( mut self, enable : bool ) -> Self
    {
      self.include_timing = enable;
      self
    }

    /// Enable or disable body logging (use with caution - may log sensitive data)
    #[ inline ]
    #[ must_use ]
    pub fn with_body_logging( mut self, enable : bool ) -> Self
    {
      self.log_bodies = enable;
      self
    }

    /// Initialize tracing subscriber with this configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the subscriber cannot be initialized
    #[ inline ]
    pub fn init( &self ) -> Result< () >
    {
      // Note : This is a basic initialization
      // In production, you would use tracing_subscriber::registry()
      // with layers for different output formats

      // For now, this is a placeholder that can be enhanced with actual tracing setup
      // when the feature is enabled and tracing is available

      Ok( () )
    }
  }

  impl Default for LoggingConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  impl RequestContext
  {
    /// Create new request context
    #[ inline ]
    #[ must_use ]
    pub fn new( operation : &str ) -> Self
    {
      Self
      {
        request_id : Self::generate_request_id(),
        model : None,
        operation : operation.to_string(),
        started_at : std::time::Instant::now(),
      }
    }

    /// Create request context with model
    #[ inline ]
    #[ must_use ]
    pub fn with_model( mut self, model : &str ) -> Self
    {
      self.model = Some( model.to_string() );
      self
    }

    /// Generate unique request ID
    #[ inline ]
    fn generate_request_id() -> String
    {
      use std::time::SystemTime;

      let timestamp = SystemTime::now()
        .duration_since( SystemTime::UNIX_EPOCH )
        .unwrap_or_default()
        .as_millis();

      format!( "req_{}", timestamp )
    }

    /// Get elapsed time since request start
    #[ inline ]
    #[ must_use ]
    pub fn elapsed( &self ) -> std::time::Duration
    {
      self.started_at.elapsed()
    }

    /// Get elapsed time in milliseconds
    #[ inline ]
    #[ must_use ]
    pub fn elapsed_ms( &self ) -> u128
    {
      self.elapsed().as_millis()
    }
  }

  /// Logging instrumentation helpers
  pub mod instrument
  {
    use super::RequestContext;

    /// Log request start
    #[ inline ]
    pub fn log_request_start( ctx : &RequestContext )
    {
      #[ cfg( feature = "structured_logging" ) ]
      {
        tracing ::info!(
          request_id = %ctx.request_id,
          operation = %ctx.operation,
          model = ?ctx.model,
          "Starting API request"
        );
      }

      #[ cfg( not( feature = "structured_logging" ) ) ]
      {
        let _ = ctx; // Suppress unused warning
      }
    }

    /// Log request completion
    #[ inline ]
    pub fn log_request_complete( ctx : &RequestContext )
    {
      #[ cfg( feature = "structured_logging" ) ]
      {
        tracing ::info!(
          request_id = %ctx.request_id,
          operation = %ctx.operation,
          duration_ms = ctx.elapsed_ms(),
          "API request completed"
        );
      }

      #[ cfg( not( feature = "structured_logging" ) ) ]
      {
        let _ = ctx; // Suppress unused warning
      }
    }

    /// Log request error
    #[ inline ]
    pub fn log_request_error( ctx : &RequestContext, error : &str )
    {
      #[ cfg( feature = "structured_logging" ) ]
      {
        tracing ::error!(
          request_id = %ctx.request_id,
          operation = %ctx.operation,
          duration_ms = ctx.elapsed_ms(),
          error = %error,
          "API request failed"
        );
      }

      #[ cfg( not( feature = "structured_logging" ) ) ]
      {
        let _ = ( ctx, error ); // Suppress unused warning
      }
    }

    /// Log streaming chunk received
    #[ inline ]
    pub fn log_stream_chunk( ctx : &RequestContext, chunk_num : usize )
    {
      #[ cfg( feature = "structured_logging" ) ]
      {
        tracing ::debug!(
          request_id = %ctx.request_id,
          chunk = chunk_num,
          "Received streaming chunk"
        );
      }

      #[ cfg( not( feature = "structured_logging" ) ) ]
      {
        let _ = ( ctx, chunk_num ); // Suppress unused warning
      }
    }
  }
}

#[ cfg( feature = "structured_logging" ) ]
crate ::mod_interface!
{
  exposed use private::LoggingConfig;
  exposed use private::LogFormat;
  exposed use private::RequestContext;
  exposed use private::instrument;
}
