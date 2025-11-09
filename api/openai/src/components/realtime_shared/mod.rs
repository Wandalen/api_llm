//! Structures shared across the Realtime API for session management and event handling.

// Declare submodules
pub mod session;
pub mod transcription;
pub mod conversation;
pub mod response;
pub mod events;
pub mod events_server;

// Re-export session types
pub use session::
{
  RealtimeSessionInputAudioTranscription,
  RealtimeSessionTurnDetection,
  RealtimeSessionInputAudioNoiseReduction,
  RealtimeSession,
  RealtimeClientSecret,
  RealtimeSessionCreateResponse,
  RealtimeSessionCreateRequest,
};

// Re-export transcription types
pub use transcription::
{
  RealtimeTranscriptionSessionCreateResponse,
  RealtimeTranscriptionSessionCreateRequest,
};

// Re-export conversation types
pub use conversation::
{
  RealtimeConversationItem,
  RealtimeConversationItemContent,
  RealtimeConversationItemWithReference,
  RealtimeConversationInfo,
};

// Re-export response types
pub use response::
{
  RealtimeResponseStatusDetails,
  RealtimeResponseInputTokenDetails,
  RealtimeResponseOutputTokenDetails,
  RealtimeResponseUsage,
  RealtimeResponse,
  RealtimeResponseCreateParams,
};

// Re-export client event types
pub use events::
{
  RealtimeClientEventConversationItemCreate,
  RealtimeClientEventConversationItemDelete,
  RealtimeClientEventConversationItemRetrieve,
  RealtimeClientEventConversationItemTruncate,
  RealtimeClientEventInputAudioBufferAppend,
  RealtimeClientEventInputAudioBufferClear,
  RealtimeClientEventInputAudioBufferCommit,
  RealtimeClientEventResponseCancel,
  RealtimeClientEventResponseCreate,
  RealtimeClientEventSessionUpdate,
  RealtimeClientEventTranscriptionSessionUpdate,
  RealtimeClientEvent,
  RealtimeServerEvent,
};

// Re-export server event types
pub use events_server::
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
  RealtimeErrorDetails,
  RealtimeServerEventInputAudioBufferCleared,
  RealtimeServerEventInputAudioBufferCommitted,
  RealtimeServerEventInputAudioBufferSpeechStarted,
  RealtimeServerEventInputAudioBufferSpeechStopped,
  RealtimeServerEventRateLimitsUpdated,
  RateLimitInfo,
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