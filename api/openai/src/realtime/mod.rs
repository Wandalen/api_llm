// src/api/realtime/mod.rs
//! This module defines the `Realtime` API client, which provides methods
//! for interacting with the `OpenAI` Realtime API.
//!
//! For more details, refer to the [`OpenAI` Realtime API documentation](https://platform.openai.com/docs/api-reference/realtime).

/// Define a private namespace for all its items.
mod private
{
  // Use crate root for base access
  use crate::
  {
    client ::Client,
    error ::{ OpenAIError, Result },
    environment ::{ OpenaiEnvironment, EnvironmentInterface },
  };
  use crate::components::realtime_shared:: // Corrected import path
  {
    RealtimeClientEvent,
    RealtimeServerEvent,
    RealtimeSession,
    RealtimeSessionCreateRequest,
    RealtimeSessionCreateResponse,
    // RealtimeSessionUpdateRequest, // Not available
    RealtimeTranscriptionSessionCreateRequest,
    RealtimeTranscriptionSessionCreateResponse,
    // RealtimeTranscriptionSessionUpdateRequest, // Not available
  };

  // External crates
  use tokio::sync::mpsc;
  use tokio::
  {
    net ::TcpStream,
    sync ::{ Mutex }, // Mutex is needed here
  };
  use tokio_tungstenite::{ MaybeTlsStream, WebSocketStream };
  use futures_util::
  {
    StreamExt as _, // Renamed to avoid conflict
    SinkExt as _, // Renamed to avoid conflict
  };
  use serde_json;
  use std::sync::Arc;

  /// The client for the `OpenAI` Realtime API.
  #[ derive( Debug, Clone ) ]
  pub struct Realtime< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client Client< E >,
  }

  impl< 'client, E > Realtime< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Creates a new `Realtime` client.
    ///
    /// # Arguments
    /// - `client`: The core `OpenAI` `Client` to use for requests.
    pub(crate) fn new( client : &'client Client< E > ) -> Self
    {
      Self { client }
    }

    /// Creates a new Realtime session.
    ///
    /// # Arguments
    /// - `request`: The request body for creating a Realtime session.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn create_session( &self, request : RealtimeSessionCreateRequest ) -> Result< RealtimeSessionCreateResponse >
    {
      self.client.post( "realtime/sessions", &request ).await
    }

    /// Retrieves a Realtime session.
    ///
    /// # Arguments
    /// - `session_id`: The ID of the Realtime session to retrieve.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn retrieve_session( &self, session_id : &str ) -> Result< RealtimeSession >
    {
      let path = format!( "/realtime/sessions/{session_id}" );
      self.client.get( &path ).await
    }

    /// Updates a Realtime session.
    ///
    /// # Arguments
    /// - `session_id`: The ID of the Realtime session to update.
    /// - `request`: The request body for updating the Realtime session.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn update_session( &self, session_id : &str, request : serde_json::Value ) -> Result< RealtimeSession >
    {
      let path = format!( "/realtime/sessions/{session_id}" );
      self.client.post( &path, &request ).await
    }

    /// Deletes a Realtime session.
    ///
    /// # Arguments
    /// - `session_id`: The ID of the Realtime session to delete.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn delete_session( &self, session_id : &str ) -> Result< serde_json::Value >
    {
      let path = format!( "/realtime/sessions/{session_id}" );
      self.client.delete( &path ).await
    }

    /// Creates a new Realtime transcription session.
    ///
    /// # Arguments
    /// - `request`: The request body for creating a Realtime transcription session.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn create_transcription_session( &self, request : RealtimeTranscriptionSessionCreateRequest ) -> Result< RealtimeTranscriptionSessionCreateResponse >
    {
      self.client.post( "realtime/transcription_sessions", &request ).await
    }

    /// Updates a Realtime transcription session.
    ///
    /// # Arguments
    /// - `session_id`: The ID of the Realtime transcription session to update.
    /// - `request`: The request body for updating the Realtime transcription session.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn update_transcription_session( &self, session_id : &str, request : serde_json::Value ) -> Result< RealtimeTranscriptionSessionCreateResponse >
    {
      let path = format!( "/realtime/transcription_sessions/{session_id}" );
      self.client.post( &path, &request ).await
    }

    /// Deletes a Realtime transcription session.
    ///
    /// # Arguments
    /// - `session_id`: The ID of the Realtime transcription session to delete.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn delete_transcription_session( &self, session_id : &str ) -> Result< serde_json::Value >
    {
      let path = format!( "/realtime/transcription_sessions/{session_id}" );
      self.client.delete( &path ).await
    }

    /// Establishes a WebSocket connection for Realtime API events.
    ///
    /// # Arguments
    /// - `session_id`: The ID of the Realtime session to connect to.
    ///
    /// # Errors
    /// Returns `OpenAIError::Ws` if the WebSocket connection fails.
    #[ inline ]
    pub async fn connect_ws( &self, session_id : &str ) -> Result< WsSession >
    {
      let url = self.client.environment.join_realtime_base_url( &format!( "sessions/{session_id}/events" ) )?;
      WsSession::connect( url.as_str() ).await
    }
  }

  /// Represents a message handled by the WebSocket session.
  #[ derive( Debug ) ]
  pub enum HandlerMessage
  {
    /// A message received from the WebSocket.
    Message( String ),
    /// An error occurred during WebSocket communication.
    Error( OpenAIError ),
    /// The WebSocket connection was closed.
    Closed,
  }

  /// A WebSocket session client for the `OpenAI` Realtime API.
  ///
  /// Manages the WebSocket connection, sending and receiving messages,
  /// and handling events.
  #[ derive( Debug, Clone ) ]
  pub struct WsSession
  {
    /// Receiver for messages from the WebSocket.
    pub rx : Arc< tokio::sync::Mutex< tokio::sync::mpsc::Receiver< HandlerMessage > > >, // Changed to Arc< Mutex< Receiver > >
    /// Sender for messages to the WebSocket.
    pub tx : Arc< tokio::sync::mpsc::Sender< HandlerMessage > >,
    /// The WebSocket stream.
    pub ws_stream : Arc< tokio::sync::Mutex< WebSocketStream< MaybeTlsStream< TcpStream > > > >,
  }

  impl WsSession
  {
    /// Creates a new `WsSession` and connects to the specified URL.
    ///
    /// # Arguments
    /// - `url`: The WebSocket URL to connect to.
    ///
    /// # Errors
    /// Returns `OpenAIError::Ws` if the connection fails.
    #[ inline ]
    pub async fn connect( url : &str ) -> Result< Self >
    {
      let ( ws_stream, _ ) = tokio_tungstenite::connect_async( url )
      .await
      .map_err( | e | OpenAIError::Ws( e.to_string() ) )?; // Convert error to String

      let ws_stream_arc = Arc::new( tokio::sync::Mutex::new( ws_stream ) );
      let ( tx, rx ) = mpsc::channel( 100 );
      let rx_arc = Arc::new( Mutex::new( rx ) ); // Wrap rx in Arc< Mutex >
      let tx_arc = Arc::new( tx ); // Wrap tx in Arc

      let ws_stream_locked = Arc::< _ >::clone( &ws_stream_arc );
      let tx_clone = Arc::< _ >::clone( &tx_arc ); // Clone the Arc< Sender >

      tokio ::spawn( async move
      {
        let mut ws_stream_locked = ws_stream_locked.lock().await;
        loop
        {
          tokio ::select!
          {
            // Receive messages from the WebSocket
            msg = ws_stream_locked.next() =>
            {
              match msg
              {
                Some( Ok( msg ) ) =>
                {
                  if msg.is_text()
                  {
                    let message = msg.to_string();
                    let _ = tx_clone.send( HandlerMessage::Message( message ) ).await.ok();
                  }
                },
                Some( Err( error ) ) =>
                {
                  let _ = tx_clone.send( HandlerMessage::Error( OpenAIError::Ws( error.to_string() ) ) ).await.ok(); // Convert error to String
                  break;
                },
                None =>
                {
                  let _ = tx_clone.send( HandlerMessage::Closed ).await.ok();
                  break;
                },
              }
            },
            // Handle messages to send (if any, though this is primarily a receiver)
            // This branch is mostly for demonstration or future expansion
            _unit = tokio::time::sleep( tokio::time::Duration::from_secs( 1 ) ) =>
            {
              // Periodically check or send keep-alive if needed
            }
          }
        }
      });

      Ok( Self
      {
        ws_stream : ws_stream_arc,
        tx : tx_arc, // Assign the Arc< Sender >
        rx : rx_arc,
      })
    }

    /// Sends a client event message over the WebSocket.
    ///
    /// # Arguments
    /// - `event`: The `RealtimeClientEvent` to send.
    ///
    /// # Errors
    /// Returns `OpenAIError::Internal` if serialization or sending fails.
    #[ inline ]
    pub async fn send_event( &self, event : RealtimeClientEvent ) -> Result< () >
    {
      let message = serde_json::to_string( &event )
      .map_err( | e | OpenAIError::Internal( format!( "Serialization error : {e}" ) ) )?;
      let mut ws_stream_locked = self.ws_stream.lock().await;
      ws_stream_locked.send( tokio_tungstenite::tungstenite::Message::Text( message.into() ) ) // Convert String to Utf8Bytes
      .await
      .map_err( | e | OpenAIError::Ws( e.to_string() ) )?; // Convert error to String
      Ok( () )
    }

    /// Receives a server event message from the WebSocket.
    ///
    /// # Errors
    /// Returns `OpenAIError::Internal` if deserialization fails or if the channel is closed.
    #[ inline ]
    pub async fn recv_event( &self ) -> Result< RealtimeServerEvent >
    {
      match self.rx.lock().await.recv().await // Re-added .lock().await
      {
        Some( HandlerMessage::Message( message ) ) =>
        {
          serde_json ::from_str( &message )
          .map_err( | e | OpenAIError::Internal( format!( "Deserialization error : {e}" ) ).into() )
        },
        Some( HandlerMessage::Error( error ) ) => Err( error.into() ),
        Some( HandlerMessage::Closed ) | None => Err( OpenAIError::Ws( tokio_tungstenite::tungstenite::Error::ConnectionClosed.to_string() ).into() ), // Convert error to String
      }
    }
  }
} // end mod private

crate ::mod_interface!
{
  // Expose all structs defined in this module
  exposed use
  {
    Realtime,
    WsSession,
    HandlerMessage,
  };
}