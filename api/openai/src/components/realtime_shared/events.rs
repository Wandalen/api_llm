//! Event enums and structures for Realtime API client and server communication.

/// Define a private namespace for event-related items.
mod private
{
  // Use full paths from crate root for components
  use crate::components::realtime_shared::session::RealtimeSessionCreateRequest;
  use crate::components::realtime_shared::transcription::RealtimeTranscriptionSessionCreateRequest;
  use crate::components::realtime_shared::conversation::RealtimeConversationItem;
  use crate::components::realtime_shared::response::RealtimeResponseCreateParams;
  use crate::components::realtime_shared::events_server::
  {
    RealtimeServerEventConversationCreated,
    RealtimeServerEventConversationItemCreated,
    RealtimeServerEventConversationItemDeleted,
    RealtimeServerEventConversationItemInputAudioTranscriptionCompleted,
    RealtimeServerEventConversationItemInputAudioTranscriptionDelta,
    RealtimeServerEventConversationItemInputAudioTranscriptionFailed,
    RealtimeServerEventConversationItemRetrieved,
    RealtimeServerEventConversationItemTruncated,
    RealtimeServerEventError,
    RealtimeServerEventInputAudioBufferCleared,
    RealtimeServerEventInputAudioBufferCommitted,
    RealtimeServerEventInputAudioBufferSpeechStarted,
    RealtimeServerEventInputAudioBufferSpeechStopped,
    RealtimeServerEventRateLimitsUpdated,
    RealtimeServerEventResponseAudioDelta,
    RealtimeServerEventResponseAudioDone,
    RealtimeServerEventResponseAudioTranscriptDelta,
    RealtimeServerEventResponseAudioTranscriptDone,
    RealtimeServerEventResponseContentPartAdded,
    RealtimeServerEventResponseContentPartDone,
    RealtimeServerEventResponseCreated,
    RealtimeServerEventResponseDone,
    RealtimeServerEventResponseFunctionCallArgumentsDelta,
    RealtimeServerEventResponseFunctionCallArgumentsDone,
    RealtimeServerEventResponseOutputItemAdded,
    RealtimeServerEventResponseOutputItemDone,
    RealtimeServerEventResponseTextDelta,
    RealtimeServerEventResponseTextDone,
    RealtimeServerEventSessionCreated,
    RealtimeServerEventSessionUpdated,
    RealtimeServerEventTranscriptionSessionCreated,
    RealtimeServerEventTranscriptionSessionUpdated,
  };

  // Serde imports
  use serde::{ Serialize, Deserialize };

  // --- Client Events ---

  /// Client event to create a new conversation item.
  ///
  /// # Used By
  /// - `RealtimeClientEvent`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, former::Former ) ] // Added Serialize
  pub struct RealtimeClientEventConversationItemCreate
  {
    /// Optional client-generated ID for the event.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub event_id : Option< String >,
    /// ID of the preceding item or "root". Appends if null.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub previous_item_id : Option< String >,
    /// The item to create.
    pub item : RealtimeConversationItem,
  }

  /// Client event to delete a conversation item.
  ///
  /// # Used By
  /// - `RealtimeClientEvent`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, former::Former ) ] // Added Serialize
  pub struct RealtimeClientEventConversationItemDelete
  {
    /// Optional client-generated ID for the event.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub event_id : Option< String >,
    /// The ID of the item to delete.
    pub item_id : String,
  }

  /// Client event to retrieve a specific conversation item.
  ///
  /// # Used By
  /// - `RealtimeClientEvent`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, former::Former ) ] // Added Serialize
  pub struct RealtimeClientEventConversationItemRetrieve
  {
    /// Optional client-generated ID for the event.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub event_id : Option< String >,
    /// The ID of the item to retrieve.
    pub item_id : String,
  }

  /// Client event to truncate a previous assistant audio message.
  ///
  /// # Used By
  /// - `RealtimeClientEvent`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, former::Former ) ] // Added Serialize
  pub struct RealtimeClientEventConversationItemTruncate
  {
    /// Optional client-generated ID for the event.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub event_id : Option< String >,
    /// The ID of the assistant message item to truncate.
    pub item_id : String,
    /// The index of the content part to truncate (should be 0).
    pub content_index : i32,
    /// Inclusive duration up to which audio is truncated (ms).
    pub audio_end_ms : i32,
  }

  /// Client event to append audio bytes to the input buffer.
  ///
  /// # Used By
  /// - `RealtimeClientEvent`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, former::Former ) ] // Added Serialize
  pub struct RealtimeClientEventInputAudioBufferAppend
  {
    /// Optional client-generated ID for the event.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub event_id : Option< String >,
    /// Base64-encoded audio bytes in the configured format.
    pub audio : String,
  }

  /// Client event to clear the input audio buffer.
  ///
  /// # Used By
  /// - `RealtimeClientEvent`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, former::Former ) ] // Added Serialize
  pub struct RealtimeClientEventInputAudioBufferClear
  {
    /// Optional client-generated ID for the event.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub event_id : Option< String >,
  }

  /// Client event to commit the input audio buffer (creates a user message).
  /// Not needed in Server VAD mode.
  ///
  /// # Used By
  /// - `RealtimeClientEvent`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, former::Former ) ] // Added Serialize
  pub struct RealtimeClientEventInputAudioBufferCommit
  {
    /// Optional client-generated ID for the event.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub event_id : Option< String >,
  }

  /// Client event to cancel an in-progress response.
  ///
  /// # Used By
  /// - `RealtimeClientEvent`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, former::Former ) ] // Added Serialize
  pub struct RealtimeClientEventResponseCancel
  {
    /// Optional client-generated ID for the event.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub event_id : Option< String >,
    /// Optional ID of the specific response to cancel.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub response_id : Option< String >,
  }

  /// Client event to trigger model inference and create a response.
  /// Not needed in Server VAD mode with `create_response : true`.
  ///
  /// # Used By
  /// - `RealtimeClientEvent`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, former::Former ) ] // Added Serialize
  pub struct RealtimeClientEventResponseCreate
  {
    /// Optional client-generated ID for the event.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub event_id : Option< String >,
    /// Optional overrides for the response generation.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub response : Option< RealtimeResponseCreateParams >,
  }

  /// Client event to update the session's default configuration.
  ///
  /// # Used By
  /// - `RealtimeClientEvent`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, former::Former ) ] // Added Serialize
  pub struct RealtimeClientEventSessionUpdate
  {
    /// Optional client-generated ID for the event.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub event_id : Option< String >,
    /// The session configuration fields to update.
    pub session : RealtimeSessionCreateRequest, // Reusing request struct for update payload
  }

  /// Client event to update a transcription session's configuration.
  ///
  /// # Used By
  /// - `RealtimeClientEvent`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, former::Former ) ] // Added Serialize
  pub struct RealtimeClientEventTranscriptionSessionUpdate
  {
    /// Optional client-generated ID for the event.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub event_id : Option< String >,
    /// The transcription session configuration fields to update.
    pub session : RealtimeTranscriptionSessionCreateRequest,
  }

  /// Represents the different types of events sent by the client in a Realtime session.
  ///
  /// # Used By
  /// - Realtime WebSocket client implementations.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  #[ serde( tag = "type" ) ] // Use the 'type' field to determine the variant
  pub enum RealtimeClientEvent
  {
    /// Add a new item to the conversation context.
    #[ serde( rename = "conversation.item.create" ) ]
    ConversationItemCreate( RealtimeClientEventConversationItemCreate ),
    /// Delete an item from the conversation context.
    #[ serde( rename = "conversation.item.delete" ) ]
    ConversationItemDelete( RealtimeClientEventConversationItemDelete ),
    /// Retrieve the server's representation of a conversation item.
    #[ serde( rename = "conversation.item.retrieve" ) ]
    ConversationItemRetrieve( RealtimeClientEventConversationItemRetrieve ),
    /// Truncate a previous assistant audio message.
    #[ serde( rename = "conversation.item.truncate" ) ]
    ConversationItemTruncate( RealtimeClientEventConversationItemTruncate ),
    /// Append audio bytes to the input buffer.
    #[ serde( rename = "input_audio_buffer.append" ) ]
    InputAudioBufferAppend( RealtimeClientEventInputAudioBufferAppend ),
    /// Clear the input audio buffer.
    #[ serde( rename = "input_audio_buffer.clear" ) ]
    InputAudioBufferClear( RealtimeClientEventInputAudioBufferClear ),
    /// Commit the input audio buffer (creates a user message).
    #[ serde( rename = "input_audio_buffer.commit" ) ]
    InputAudioBufferCommit( RealtimeClientEventInputAudioBufferCommit ),
    /// Cancel an in-progress response.
    #[ serde( rename = "response.cancel" ) ]
    ResponseCancel( RealtimeClientEventResponseCancel ),
    /// Trigger the model to generate a response.
    #[ serde( rename = "response.create" ) ]
    ResponseCreate( RealtimeClientEventResponseCreate ),
    /// Update the session's default configuration.
    #[ serde( rename = "session.update" ) ]
    SessionUpdate( RealtimeClientEventSessionUpdate ),
    /// Update a transcription session's configuration.
    #[ serde( rename = "transcription_session.update" ) ]
    TranscriptionSessionUpdate( RealtimeClientEventTranscriptionSessionUpdate ),
  }


  /// Represents the different types of events sent by the server in a Realtime session.
  ///
  /// # Used By
  /// - Realtime WebSocket client implementations (for deserializing incoming events).
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  #[ serde( tag = "type" ) ] // Use the 'type' field to determine the variant
  pub enum RealtimeServerEvent
  {
    /// Conversation created event.
    #[ serde( rename = "conversation.created" ) ]
    ConversationCreated( RealtimeServerEventConversationCreated ),
    /// Conversation item created event.
    #[ serde( rename = "conversation.item.created" ) ]
    ConversationItemCreated( RealtimeServerEventConversationItemCreated ),
    /// Conversation item deleted event.
    #[ serde( rename = "conversation.item.deleted" ) ]
    ConversationItemDeleted( RealtimeServerEventConversationItemDeleted ),
    /// Input audio transcription completed event.
    #[ serde( rename = "conversation.item.input_audio_transcription.completed" ) ]
    ConversationItemInputAudioTranscriptionCompleted( RealtimeServerEventConversationItemInputAudioTranscriptionCompleted ),
    /// Input audio transcription delta event.
    #[ serde( rename = "conversation.item.input_audio_transcription.delta" ) ]
    ConversationItemInputAudioTranscriptionDelta( RealtimeServerEventConversationItemInputAudioTranscriptionDelta ),
    /// Input audio transcription failed event.
    #[ serde( rename = "conversation.item.input_audio_transcription.failed" ) ]
    ConversationItemInputAudioTranscriptionFailed( RealtimeServerEventConversationItemInputAudioTranscriptionFailed ),
    /// Conversation item retrieved event.
    #[ serde( rename = "conversation.item.retrieved" ) ]
    ConversationItemRetrieved( RealtimeServerEventConversationItemRetrieved ),
    /// Conversation item truncated event.
    #[ serde( rename = "conversation.item.truncated" ) ]
    ConversationItemTruncated( RealtimeServerEventConversationItemTruncated ),
    /// Error event.
    #[ serde( rename = "error" ) ]
    Error( RealtimeServerEventError ),
    /// Input audio buffer cleared event.
    #[ serde( rename = "input_audio_buffer.cleared" ) ]
    InputAudioBufferCleared( RealtimeServerEventInputAudioBufferCleared ),
    /// Input audio buffer committed event.
    #[ serde( rename = "input_audio_buffer.committed" ) ]
    InputAudioBufferCommitted( RealtimeServerEventInputAudioBufferCommitted ),
    /// Input audio buffer speech started event.
    #[ serde( rename = "input_audio_buffer.speech_started" ) ]
    InputAudioBufferSpeechStarted( RealtimeServerEventInputAudioBufferSpeechStarted ),
    /// Input audio buffer speech stopped event.
    #[ serde( rename = "input_audio_buffer.speech_stopped" ) ]
    InputAudioBufferSpeechStopped( RealtimeServerEventInputAudioBufferSpeechStopped ),
    /// Rate limits updated event.
    #[ serde( rename = "rate_limits.updated" ) ]
    RateLimitsUpdated( RealtimeServerEventRateLimitsUpdated ),
    /// Response audio delta event.
    #[ serde( rename = "response.audio.delta" ) ]
    ResponseAudioDelta( RealtimeServerEventResponseAudioDelta ),
    /// Response audio done event.
    #[ serde( rename = "response.audio.done" ) ]
    ResponseAudioDone( RealtimeServerEventResponseAudioDone ),
    /// Response audio transcript delta event.
    #[ serde( rename = "response.audio_transcript.delta" ) ]
    ResponseAudioTranscriptDelta( RealtimeServerEventResponseAudioTranscriptDelta ),
    /// Response audio transcript done event.
    #[ serde( rename = "response.audio_transcript.done" ) ]
    ResponseAudioTranscriptDone( RealtimeServerEventResponseAudioTranscriptDone ),
    /// Response content part added event.
    #[ serde( rename = "response.content_part.added" ) ]
    ResponseContentPartAdded( RealtimeServerEventResponseContentPartAdded ),
    /// Response content part done event.
    #[ serde( rename = "response.content_part.done" ) ]
    ResponseContentPartDone( RealtimeServerEventResponseContentPartDone ),
    /// Response created event.
    #[ serde( rename = "response.created" ) ]
    ResponseCreated( RealtimeServerEventResponseCreated ),
    /// Response done event.
    #[ serde( rename = "response.done" ) ]
    ResponseDone( RealtimeServerEventResponseDone ),
    /// Response function call arguments delta event.
    #[ serde( rename = "response.function_call_arguments.delta" ) ]
    ResponseFunctionCallArgumentsDelta( RealtimeServerEventResponseFunctionCallArgumentsDelta ),
    /// Response function call arguments done event.
    #[ serde( rename = "response.function_call_arguments.done" ) ]
    ResponseFunctionCallArgumentsDone( RealtimeServerEventResponseFunctionCallArgumentsDone ),
    /// Response output item added event.
    #[ serde( rename = "response.output_item.added" ) ]
    ResponseOutputItemAdded( RealtimeServerEventResponseOutputItemAdded ),
    /// Response output item done event.
    #[ serde( rename = "response.output_item.done" ) ]
    ResponseOutputItemDone( RealtimeServerEventResponseOutputItemDone ),
    /// Response text delta event.
    #[ serde( rename = "response.text.delta" ) ]
    ResponseTextDelta( RealtimeServerEventResponseTextDelta ),
    /// Response text done event.
    #[ serde( rename = "response.text.done" ) ]
    ResponseTextDone( RealtimeServerEventResponseTextDone ),
    /// Session created event.
    #[ serde( rename = "session.created" ) ]
    SessionCreated( RealtimeServerEventSessionCreated ),
    /// Session updated event.
    #[ serde( rename = "session.updated" ) ]
    SessionUpdated( RealtimeServerEventSessionUpdated ),
    /// Transcription session created event.
    #[ serde( rename = "transcription_session.created" ) ]
    TranscriptionSessionCreated( RealtimeServerEventTranscriptionSessionCreated ),
    /// Transcription session updated event.
    #[ serde( rename = "transcription_session.updated" ) ]
    TranscriptionSessionUpdated( RealtimeServerEventTranscriptionSessionUpdated ),
  }

} // end mod private

crate ::mod_interface!
{
  exposed use private::RealtimeClientEventConversationItemCreate;
  exposed use private::RealtimeClientEventConversationItemDelete;
  exposed use private::RealtimeClientEventConversationItemRetrieve;
  exposed use private::RealtimeClientEventConversationItemTruncate;
  exposed use private::RealtimeClientEventInputAudioBufferAppend;
  exposed use private::RealtimeClientEventInputAudioBufferClear;
  exposed use private::RealtimeClientEventInputAudioBufferCommit;
  exposed use private::RealtimeClientEventResponseCancel;
  exposed use private::RealtimeClientEventResponseCreate;
  exposed use private::RealtimeClientEventSessionUpdate;
  exposed use private::RealtimeClientEventTranscriptionSessionUpdate;
  exposed use private::RealtimeClientEvent;
  exposed use private::RealtimeServerEvent;
}