//! WebSocket configuration implementation.

#[ cfg( feature = "websocket_streaming" ) ]
mod private
{
  use core::time::Duration;
  use crate::websocket::WebSocketConfig;

  impl WebSocketConfig
  {
    /// Create a new WebSocket configuration with default values
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        url : "ws://localhost:11434/api/ws".to_string(),
        timeout : Duration::from_secs( 30 ),
        max_reconnection_attempts : 5,
        reconnection_delay : Duration::from_secs( 2 ),
        enable_compression : true,
        heartbeat_interval : Duration::from_secs( 30 ),
        max_queue_size : 1000,
        auth_token : None,
        enable_auto_reconnect : true,
        pool_size : 5,
        http_fallback_url : Some( "http://localhost:11434/api/chat".to_string() ),
      }
    }

    /// Set the WebSocket URL
    #[ inline ]
    #[ must_use ]
    pub fn with_url< S: Into< String > >( mut self, url : S ) -> Self
    {
      self.url = url.into();
      self
    }

    /// Set the connection timeout
    #[ inline ]
    #[ must_use ]
    pub fn with_timeout( mut self, timeout : Duration ) -> Self
    {
      self.timeout = timeout;
      self
    }

    /// Set the maximum reconnection attempts
    #[ inline ]
    #[ must_use ]
    pub fn with_max_reconnection_attempts( mut self, attempts : u32 ) -> Self
    {
      self.max_reconnection_attempts = attempts;
      self
    }

    /// Set the reconnection delay
    #[ inline ]
    #[ must_use ]
    pub fn with_reconnection_delay( mut self, delay : Duration ) -> Self
    {
      self.reconnection_delay = delay;
      self
    }

    /// Enable or disable compression
    #[ inline ]
    #[ must_use ]
    pub fn with_compression( mut self, enable : bool ) -> Self
    {
      self.enable_compression = enable;
      self
    }

    /// Set the heartbeat interval
    #[ inline ]
    #[ must_use ]
    pub fn with_heartbeat_interval( mut self, interval : Duration ) -> Self
    {
      self.heartbeat_interval = interval;
      self
    }

    /// Set the maximum queue size
    #[ inline ]
    #[ must_use ]
    pub fn with_max_queue_size( mut self, size : usize ) -> Self
    {
      self.max_queue_size = size;
      self
    }

    /// Set the authentication token
    #[ inline ]
    #[ must_use ]
    pub fn with_auth_token< S: Into< String > >( mut self, token : S ) -> Self
    {
      self.auth_token = Some( token.into() );
      self
    }

    /// Enable or disable auto-reconnect
    #[ inline ]
    #[ must_use ]
    pub fn with_auto_reconnect( mut self, enable : bool ) -> Self
    {
      self.enable_auto_reconnect = enable;
      self
    }

    /// Set the connection pool size
    #[ inline ]
    #[ must_use ]
    pub fn with_pool_size( mut self, size : usize ) -> Self
    {
      self.pool_size = size;
      self
    }

    /// Set the HTTP fallback URL
    #[ inline ]
    #[ must_use ]
    pub fn with_http_fallback< S: Into< String > >( mut self, url : S ) -> Self
    {
      self.http_fallback_url = Some( url.into() );
      self
    }

    /// Disable HTTP fallback
    #[ inline ]
    #[ must_use ]
    pub fn without_http_fallback( mut self ) -> Self
    {
      self.http_fallback_url = None;
      self
    }

    /// Get the WebSocket URL
    #[ inline ]
    #[ must_use ]
    pub fn url( &self ) -> &str
    {
      &self.url
    }

    /// Get the connection timeout
    #[ inline ]
    #[ must_use ]
    pub fn timeout( &self ) -> Duration
    {
      self.timeout
    }

    /// Get the maximum reconnection attempts
    #[ inline ]
    #[ must_use ]
    pub fn max_reconnection_attempts( &self ) -> u32
    {
      self.max_reconnection_attempts
    }

    /// Get the reconnection delay
    #[ inline ]
    #[ must_use ]
    pub fn reconnection_delay( &self ) -> Duration
    {
      self.reconnection_delay
    }

    /// Check if compression is enabled
    #[ inline ]
    #[ must_use ]
    pub fn is_compression_enabled( &self ) -> bool
    {
      self.enable_compression
    }

    /// Get the heartbeat interval
    #[ inline ]
    #[ must_use ]
    pub fn heartbeat_interval( &self ) -> Duration
    {
      self.heartbeat_interval
    }

    /// Get the maximum queue size
    #[ inline ]
    #[ must_use ]
    pub fn max_queue_size( &self ) -> usize
    {
      self.max_queue_size
    }

    /// Get the authentication token
    #[ inline ]
    #[ must_use ]
    pub fn auth_token( &self ) -> Option< &str >
    {
      self.auth_token.as_deref()
    }

    /// Check if auto-reconnect is enabled
    #[ inline ]
    #[ must_use ]
    pub fn is_auto_reconnect_enabled( &self ) -> bool
    {
      self.enable_auto_reconnect
    }

    /// Get the connection pool size
    #[ inline ]
    #[ must_use ]
    pub fn pool_size( &self ) -> usize
    {
      self.pool_size
    }

    /// Get the HTTP fallback URL
    #[ inline ]
    #[ must_use ]
    pub fn http_fallback_url( &self ) -> Option< &str >
    {
      self.http_fallback_url.as_deref()
    }

    /// Validate the configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid
    #[ inline ]
    pub fn validate( &self ) -> error_tools::untyped::Result< () >
    {
      if self.url.is_empty()
      {
        return Err( error_tools::untyped::format_err!( "WebSocket URL cannot be empty" ) );
      }

      if self.max_queue_size == 0
      {
        return Err( error_tools::untyped::format_err!( "Max queue size must be greater than 0" ) );
      }

      if self.pool_size == 0
      {
        return Err( error_tools::untyped::format_err!( "Pool size must be greater than 0" ) );
      }

      Ok( () )
    }
  }
}

#[ cfg( feature = "websocket_streaming" ) ]
crate ::mod_interface!
{
  exposed use {};
}
