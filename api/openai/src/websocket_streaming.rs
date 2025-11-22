//! WebSocket Streaming Module
//!
//! This module provides stateless WebSocket streaming utilities for `OpenAI` API communication.
//! Following the "Thin Client, Rich API" principle, this module offers WebSocket management
//! patterns and connection tools without automatic behaviors or persistent state management.

#![ allow( clippy::missing_inline_in_public_items ) ]

mod private
{
  use std::
  {
    collections ::{ HashMap, VecDeque },
    sync ::{ Arc, Mutex },
    time ::Instant,
  };
  use core::time::Duration;
  use serde::{ Deserialize, Serialize };
  use tokio::sync::{ mpsc, watch };

  /// WebSocket connection state
  #[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
  pub enum WebSocketState
  {
    /// Connection is being established
    Connecting,
    /// Connection is established and ready
    Connected,
    /// Connection is closing gracefully
    Closing,
    /// Connection is closed
    Closed,
    /// Connection failed with error
    Failed( String ),
  }

  /// WebSocket message type
  #[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
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
    Close( Option< String > ),
  }

  impl WebSocketMessage
  {
    /// Get message as text if it's a text message
    #[ must_use ]
    pub fn as_text( &self ) -> Option< &str >
    {
      match self
      {
        WebSocketMessage::Text( text ) => Some( text ),
        _ => None,
      }
    }

    /// Get message as binary if it's a binary message
    #[ must_use ]
    pub fn as_binary( &self ) -> Option< &[ u8 ] >
    {
      match self
      {
        WebSocketMessage::Binary( data ) => Some( data ),
        _ => None,
      }
    }

    /// Check if message is a control message (ping, pong, close)
    #[ must_use ]
    pub fn is_control( &self ) -> bool
    {
      matches!( self, WebSocketMessage::Ping( _ ) | WebSocketMessage::Pong( _ ) | WebSocketMessage::Close( _ ) )
    }

    /// Get size of the message in bytes
    #[ must_use ]
    pub fn size( &self ) -> usize
    {
      match self
      {
        WebSocketMessage::Text( text ) => text.len(),
        WebSocketMessage::Binary( data ) | WebSocketMessage::Ping( data ) | WebSocketMessage::Pong( data ) => data.len(),
        WebSocketMessage::Close( reason ) => reason.as_ref().map_or( 0, String::len ),
      }
    }
  }

  /// WebSocket connection configuration
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct WebSocketConfig
  {
    /// Connection timeout in milliseconds
    pub connect_timeout_ms : u64,
    /// Maximum message size in bytes
    pub max_message_size : usize,
    /// Heartbeat interval in milliseconds
    pub heartbeat_interval_ms : u64,
    /// Maximum queue size for outbound messages
    pub max_queue_size : usize,
    /// Enable compression
    pub enable_compression : bool,
    /// Reconnection attempts
    pub max_reconnect_attempts : u32,
    /// Reconnection delay in milliseconds
    pub reconnect_delay_ms : u64,
  }

  impl Default for WebSocketConfig
  {
    fn default() -> Self
    {
      Self
      {
        connect_timeout_ms : 30000,
        max_message_size : 16 * 1024 * 1024, // 16MB
        heartbeat_interval_ms : 30000,
        max_queue_size : 1000,
        enable_compression : true,
        max_reconnect_attempts : 3,
        reconnect_delay_ms : 1000,
      }
    }
  }

  /// WebSocket connection handle
  #[ derive( Debug ) ]
  pub struct WebSocketConnection
  {
    /// Connection identifier
    pub id : String,
    /// Target URL
    pub url : String,
    /// Current connection state
    pub state : WebSocketState,
    /// Configuration
    pub config : WebSocketConfig,
    /// Connection timestamp
    pub connected_at : Option< Instant >,
    /// Last activity timestamp
    pub last_activity : Instant,
    /// Message queue for outbound messages
    message_queue : Arc< Mutex< VecDeque< WebSocketMessage > > >,
  }

  impl WebSocketConnection
  {
    /// Create a new WebSocket connection handle
    #[ must_use ]
    pub fn new( id : String, url : String, config : WebSocketConfig ) -> Self
    {
      Self
      {
        id,
        url,
        state : WebSocketState::Connecting,
        config,
        connected_at : None,
        last_activity : Instant::now(),
        message_queue : Arc::new( Mutex::new( VecDeque::new() ) ),
      }
    }

    /// Update connection state
    pub fn update_state( &mut self, state : WebSocketState )
    {
      self.state = state;
      self.last_activity = Instant::now();

      if matches!( self.state, WebSocketState::Connected )
      {
        self.connected_at = Some( Instant::now() );
      }
    }

    /// Get connection duration if connected
    #[ must_use ]
    pub fn connection_duration( &self ) -> Option< Duration >
    {
      self.connected_at.map( | connected | connected.elapsed() )
    }

    /// Get time since last activity
    #[ must_use ]
    pub fn idle_duration( &self ) -> Duration
    {
      self.last_activity.elapsed()
    }

    /// Check if connection is active
    #[ must_use ]
    pub fn is_active( &self ) -> bool
    {
      matches!( self.state, WebSocketState::Connected )
    }

    /// Queue a message for sending
    ///
    /// # Errors
    ///
    /// Returns an error if the message queue is full or if the message exceeds the maximum size limit.
    ///
    /// # Panics
    ///
    /// Panics if the message queue mutex is poisoned.
    pub fn queue_message( &self, message : WebSocketMessage ) -> Result< (), String >
    {
      let mut queue = self.message_queue.lock().unwrap();

      if queue.len() >= self.config.max_queue_size
      {
        return Err( "Message queue is full".to_string() );
      }

      if message.size() > self.config.max_message_size
      {
        return Err( "Message exceeds maximum size".to_string() );
      }

      queue.push_back( message );
      Ok( () )
    }

    /// Dequeue next message for sending
    ///
    /// # Panics
    ///
    /// Panics if the message queue mutex is poisoned.
    #[ must_use ]
    #[ inline ]
    pub fn dequeue_message( &self ) -> Option< WebSocketMessage >
    {
      let mut queue = self.message_queue.lock().unwrap();
      queue.pop_front()
    }

    /// Get queue size
    ///
    /// # Panics
    ///
    /// Panics if the message queue mutex is poisoned.
    #[ must_use ]
    #[ inline ]
    pub fn queue_size( &self ) -> usize
    {
      self.message_queue.lock().unwrap().len()
    }

    /// Clear message queue
    ///
    /// # Panics
    ///
    /// Panics if the message queue mutex is poisoned.
    #[ inline ]
    pub fn clear_queue( &self )
    {
      let mut queue = self.message_queue.lock().unwrap();
      queue.clear();
    }
  }

  /// WebSocket connection pool for managing multiple connections
  #[ derive( Debug ) ]
  pub struct WebSocketPool
  {
    /// Active connections
    connections : HashMap<  String, WebSocketConnection  >,
    /// Pool configuration
    config : WebSocketPoolConfig,
  }

  /// Configuration for WebSocket connection pool
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct WebSocketPoolConfig
  {
    /// Maximum number of concurrent connections
    pub max_connections : usize,
    /// Connection idle timeout in milliseconds
    pub idle_timeout_ms : u64,
    /// Pool cleanup interval in milliseconds
    pub cleanup_interval_ms : u64,
  }

  impl Default for WebSocketPoolConfig
  {
    fn default() -> Self
    {
      Self
      {
        max_connections : 100,
        idle_timeout_ms : 300_000, // 5 minutes
        cleanup_interval_ms : 60000, // 1 minute
      }
    }
  }

  impl WebSocketPool
  {
    /// Create a new WebSocket connection pool
    #[ must_use ]
    #[ inline ]
    pub fn new( config : WebSocketPoolConfig ) -> Self
    {
      Self
      {
        connections : HashMap::new(),
        config,
      }
    }

    /// Add a connection to the pool
    ///
    /// # Errors
    ///
    /// Returns an error if the connection pool is full.
    #[ inline ]
    pub fn add_connection( &mut self, connection : WebSocketConnection ) -> Result< (), String >
    {
      if self.connections.len() >= self.config.max_connections
      {
        return Err( "Connection pool is full".to_string() );
      }

      self.connections.insert( connection.id.clone(), connection );
      Ok( () )
    }

    /// Get a connection from the pool
    #[ must_use ]
    #[ inline ]
    pub fn get_connection( &self, id : &str ) -> Option< &WebSocketConnection >
    {
      self.connections.get( id )
    }

    /// Get a mutable connection from the pool
    pub fn get_connection_mut( &mut self, id : &str ) -> Option< &mut WebSocketConnection >
    {
      self.connections.get_mut( id )
    }

    /// Remove a connection from the pool
    pub fn remove_connection( &mut self, id : &str ) -> Option< WebSocketConnection >
    {
      self.connections.remove( id )
    }

    /// Get all connection IDs
    #[ must_use ]
    #[ inline ]
    pub fn connection_ids( &self ) -> Vec< String >
    {
      self.connections.keys().cloned().collect()
    }

    /// Get active connection count
    #[ must_use ]
    pub fn active_connection_count( &self ) -> usize
    {
      self.connections.values().filter( | conn | conn.is_active() ).count()
    }

    /// Clean up idle connections
    pub fn cleanup_idle_connections( &mut self ) -> Vec< String >
    {
      let idle_timeout = Duration::from_millis( self.config.idle_timeout_ms );
      let mut removed = Vec::new();

      self.connections.retain( | id, conn |
      {
        if conn.idle_duration() > idle_timeout
        {
          removed.push( id.clone() );
          false
        }
        else
        {
          true
        }
      });

      removed
    }
  }

  /// WebSocket event types
  #[ derive( Debug, Clone ) ]
  pub enum WebSocketEvent
  {
    /// Connection established
    Connected
    {
      /// Connection ID
      connection_id : String,
    },
    /// Connection closed
    Disconnected
    {
      /// Connection ID
      connection_id : String,
      /// Reason for disconnection
      reason : Option< String >,
    },
    /// Message received
    MessageReceived
    {
      /// Connection ID
      connection_id : String,
      /// Received message
      message : WebSocketMessage,
    },
    /// Message sent
    MessageSent
    {
      /// Connection ID
      connection_id : String,
      /// Sent message
      message : WebSocketMessage,
    },
    /// Connection error occurred
    Error
    {
      /// Connection ID
      connection_id : String,
      /// Error message
      error : String,
    },
  }

  /// WebSocket streaming utilities
  #[ derive( Debug ) ]
  pub struct WebSocketStreamer;

  impl WebSocketStreamer
  {
    /// Create a connection event notifier
    #[ must_use ]
    pub fn create_event_notifier() -> ( WebSocketEventSender, WebSocketEventReceiver )
    {
      let ( tx, rx ) = mpsc::unbounded_channel();
      ( WebSocketEventSender { sender : tx }, WebSocketEventReceiver { receiver : rx } )
    }

    /// Create a message channel for real-time communication
    #[ must_use ]
    pub fn create_message_channel() -> ( WebSocketMessageSender, WebSocketMessageReceiver )
    {
      let ( tx, rx ) = mpsc::unbounded_channel();
      ( WebSocketMessageSender { sender : tx }, WebSocketMessageReceiver { receiver : rx } )
    }

    /// Create a connection state watcher
    #[ must_use ]
    pub fn create_state_watcher( initial_state : WebSocketState ) -> ( watch::Sender< WebSocketState >, watch::Receiver< WebSocketState > )
    {
      watch ::channel( initial_state )
    }

    /// Validate WebSocket configuration
    ///
    /// # Errors
    ///
    /// Returns an error if any configuration parameters are invalid (e.g., zero timeouts, invalid limits).
    pub fn validate_config( config : &WebSocketConfig ) -> Result< (), String >
    {
      if config.connect_timeout_ms == 0
      {
        return Err( "connect_timeout_ms must be greater than 0".to_string() );
      }

      if config.max_message_size == 0
      {
        return Err( "max_message_size must be greater than 0".to_string() );
      }

      if config.heartbeat_interval_ms == 0
      {
        return Err( "heartbeat_interval_ms must be greater than 0".to_string() );
      }

      if config.max_queue_size == 0
      {
        return Err( "max_queue_size must be greater than 0".to_string() );
      }

      Ok( () )
    }

    /// Create a heartbeat timer
    #[ must_use ]
    pub fn create_heartbeat_timer( interval : Duration ) -> mpsc::UnboundedReceiver< Instant >
    {
      let ( tx, rx ) = mpsc::unbounded_channel();

      tokio ::spawn( async move
      {
        let mut ticker = tokio::time::interval( interval );
        loop
        {
          ticker.tick().await;
          if tx.send( Instant::now() ).is_err()
          {
            break;
          }
        }
      });

      rx
    }

    /// Calculate reconnection delay with exponential backoff
    #[ must_use ]
    pub fn calculate_reconnect_delay( attempt : u32, base_delay_ms : u64, max_delay_ms : u64 ) -> Duration
    {
      let base_delay = Duration::from_millis( base_delay_ms );
      let max_delay = Duration::from_millis( max_delay_ms );

      // Exponential backoff : base_delay * 2^attempt
      let multiplier = 2_u64.saturating_pow( attempt );
      let calculated_delay = base_delay.saturating_mul( u32::try_from( multiplier ).unwrap_or( u32::MAX ) );

      core ::cmp::min( calculated_delay, max_delay )
    }

    /// Process message queue for a connection
    #[ must_use ]
    pub fn process_message_queue( connection : &WebSocketConnection, max_messages : usize ) -> Vec< WebSocketMessage >
    {
      let mut messages = Vec::new();
      for _ in 0..max_messages
      {
        if let Some( message ) = connection.dequeue_message()
        {
          messages.push( message );
        }
        else
        {
          break;
        }
      }
      messages
    }

    /// Create connection statistics
    #[ must_use ]
    pub fn connection_statistics( connection : &WebSocketConnection ) -> WebSocketConnectionStats
    {
      WebSocketConnectionStats
      {
        connection_id : connection.id.clone(),
        state : connection.state.clone(),
        connected_duration : connection.connection_duration(),
        idle_duration : connection.idle_duration(),
        queue_size : connection.queue_size(),
        last_activity : connection.last_activity,
      }
    }
  }

  /// WebSocket connection statistics
  #[ derive( Debug, Clone ) ]
  pub struct WebSocketConnectionStats
  {
    /// Connection identifier
    pub connection_id : String,
    /// Current connection state
    pub state : WebSocketState,
    /// Duration since connection established
    pub connected_duration : Option< Duration >,
    /// Duration since last activity
    pub idle_duration : Duration,
    /// Current message queue size
    pub queue_size : usize,
    /// Last activity timestamp
    pub last_activity : Instant,
  }

  /// Sender for WebSocket events
  #[ derive( Debug, Clone ) ]
  pub struct WebSocketEventSender
  {
    sender : mpsc::UnboundedSender< WebSocketEvent >,
  }

  impl WebSocketEventSender
  {
    /// Send a WebSocket event
    ///
    /// # Errors
    ///
    /// Returns an error if the event cannot be sent through the WebSocket event channel.
    pub fn send_event( &self, event : WebSocketEvent ) -> Result< (), &'static str >
    {
      self.sender.send( event ).map_err( | _ | "Failed to send WebSocket event" )
    }

    /// Send connection established event
    ///
    /// # Errors
    ///
    /// Returns an error if the connection event cannot be sent through the WebSocket event channel.
    pub fn send_connected( &self, connection_id : String ) -> Result< (), &'static str >
    {
      self.send_event( WebSocketEvent::Connected { connection_id } )
    }

    /// Send disconnection event
    ///
    /// # Errors
    ///
    /// Returns an error if the disconnection event cannot be sent through the WebSocket event channel.
    pub fn send_disconnected( &self, connection_id : String, reason : Option< String > ) -> Result< (), &'static str >
    {
      self.send_event( WebSocketEvent::Disconnected { connection_id, reason } )
    }

    /// Send message received event
    ///
    /// # Errors
    ///
    /// Returns an error if the message received event cannot be sent through the WebSocket event channel.
    pub fn send_message_received( &self, connection_id : String, message : WebSocketMessage ) -> Result< (), &'static str >
    {
      self.send_event( WebSocketEvent::MessageReceived { connection_id, message } )
    }

    /// Send error event
    ///
    /// # Errors
    ///
    /// Returns an error if the error event cannot be sent through the WebSocket event channel.
    pub fn send_error( &self, connection_id : String, error : String ) -> Result< (), &'static str >
    {
      self.send_event( WebSocketEvent::Error { connection_id, error } )
    }
  }

  /// Receiver for WebSocket events
  #[ derive( Debug ) ]
  pub struct WebSocketEventReceiver
  {
    receiver : mpsc::UnboundedReceiver< WebSocketEvent >,
  }

  impl WebSocketEventReceiver
  {
    /// Try to receive a WebSocket event (non-blocking)
    pub fn try_recv( &mut self ) -> Option< WebSocketEvent >
    {
      self.receiver.try_recv().ok()
    }

    /// Receive next WebSocket event (blocking)
    pub async fn recv( &mut self ) -> Option< WebSocketEvent >
    {
      self.receiver.recv().await
    }
  }

  /// Sender for WebSocket messages
  #[ derive( Debug, Clone ) ]
  pub struct WebSocketMessageSender
  {
    sender : mpsc::UnboundedSender< WebSocketMessage >,
  }

  impl WebSocketMessageSender
  {
    /// Send a WebSocket message
    ///
    /// # Errors
    ///
    /// Returns an error if the message cannot be sent through the WebSocket channel.
    pub fn send_message( &self, message : WebSocketMessage ) -> Result< (), &'static str >
    {
      self.sender.send( message ).map_err( | _ | "Failed to send WebSocket message" )
    }

    /// Send text message
    ///
    /// # Errors
    ///
    /// Returns an error if the message cannot be sent through the WebSocket channel.
    pub fn send_text( &self, text : String ) -> Result< (), &'static str >
    {
      self.send_message( WebSocketMessage::Text( text ) )
    }

    /// Send binary message
    ///
    /// # Errors
    ///
    /// Returns an error if the message cannot be sent through the WebSocket channel.
    pub fn send_binary( &self, data : Vec< u8 > ) -> Result< (), &'static str >
    {
      self.send_message( WebSocketMessage::Binary( data ) )
    }

    /// Send ping message
    ///
    /// # Errors
    ///
    /// Returns an error if the ping message cannot be sent through the WebSocket channel.
    pub fn send_ping( &self, data : Vec< u8 > ) -> Result< (), &'static str >
    {
      self.send_message( WebSocketMessage::Ping( data ) )
    }
  }

  /// Receiver for WebSocket messages
  #[ derive( Debug ) ]
  pub struct WebSocketMessageReceiver
  {
    receiver : mpsc::UnboundedReceiver< WebSocketMessage >,
  }

  impl WebSocketMessageReceiver
  {
    /// Try to receive a WebSocket message (non-blocking)
    pub fn try_recv( &mut self ) -> Option< WebSocketMessage >
    {
      self.receiver.try_recv().ok()
    }

    /// Receive next WebSocket message (blocking)
    pub async fn recv( &mut self ) -> Option< WebSocketMessage >
    {
      self.receiver.recv().await
    }
  }
}

crate ::mod_interface!
{
  exposed use private::WebSocketState;
  exposed use private::WebSocketMessage;
  exposed use private::WebSocketConfig;
  exposed use private::WebSocketConnection;
  exposed use private::WebSocketPool;
  exposed use private::WebSocketPoolConfig;
  exposed use private::WebSocketEvent;
  exposed use private::WebSocketStreamer;
  exposed use private::WebSocketConnectionStats;
  exposed use private::WebSocketEventSender;
  exposed use private::WebSocketEventReceiver;
  exposed use private::WebSocketMessageSender;
  exposed use private::WebSocketMessageReceiver;
}