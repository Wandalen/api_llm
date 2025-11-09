//! Server event structures for Realtime API communication.
//!
//! This module contains all server-to-client event types emitted during
//! a Realtime API WebSocket session.

/// Define a private namespace for server event items.
mod private
{
  use crate::components::common::{ Error, LogProbProperties };
  use crate::components::realtime_shared::session::RealtimeSession;
  use crate::components::realtime_shared::transcription::RealtimeTranscriptionSessionCreateResponse;
  use crate::components::realtime_shared::conversation::
  {
    RealtimeConversationItem,
    RealtimeConversationInfo,
  };
  use crate::components::realtime_shared::response::RealtimeResponse;

  use serde::{ Serialize, Deserialize };
  use serde_json::Value;

  /// Server event indicating a conversation was created.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventConversationCreated
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The conversation resource info.
    pub conversation : RealtimeConversationInfo,
  }

  /// Server event indicating a conversation item was created.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventConversationItemCreated
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the preceding item in the Conversation context.
    pub previous_item_id : Option< String >,
    /// The created conversation item.
    pub item : RealtimeConversationItem,
  }

  /// Server event indicating a conversation item was deleted.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventConversationItemDeleted
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the item that was deleted.
    pub item_id : String,
  }

  /// Server event indicating input audio transcription completed successfully.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventConversationItemInputAudioTranscriptionCompleted
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the user message item containing the audio.
    pub item_id : String,
    /// The index of the content part containing the audio.
    pub content_index : i32,
    /// The transcribed text.
    pub transcript : String,
    /// The log probabilities of the transcription, if requested.
    pub logprobs : Option< Vec< LogProbProperties > >,
  }

  /// Server event indicating a delta in the input audio transcription.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventConversationItemInputAudioTranscriptionDelta
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the item being transcribed.
    pub item_id : String,
    /// The index of the content part being transcribed.
    pub content_index : Option< i32 >, // Made optional as per example
    /// The text delta.
    pub delta : String,
    /// The log probabilities of the transcription delta, if requested.
    pub logprobs : Option< Vec< LogProbProperties > >,
  }

  /// Server event indicating input audio transcription failed.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailed
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the user message item.
    pub item_id : String,
    /// The index of the content part containing the audio.
    pub content_index : i32,
    /// Details of the transcription error.
    pub error : Error,
  }

  /// Server event containing the retrieved conversation item.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventConversationItemRetrieved
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The retrieved conversation item.
    pub item : RealtimeConversationItem,
  }

  /// Server event confirming an assistant audio message was truncated.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventConversationItemTruncated
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the assistant message item that was truncated.
    pub item_id : String,
    /// The index of the content part that was truncated.
    pub content_index : i32,
    /// The duration up to which the audio was truncated (ms).
    pub audio_end_ms : i32,
  }

  /// Server event indicating an error occurred.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventError
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// Details of the error.
    pub error : RealtimeErrorDetails,
  }

  /// Details of an error reported by the Realtime server.
  ///
  /// # Used By
  /// - `RealtimeServerEventError`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeErrorDetails
  {
    /// The type of error (e.g., "`invalid_request_error`").
    pub r#type : String,
    /// Error code, if any.
    pub code : Option< String >,
    /// A human-readable error message.
    pub message : String,
    /// Parameter related to the error, if any.
    pub param : Option< String >,
    /// The `event_id` of the client event that caused the error, if applicable.
    pub event_id : Option< String >,
  }

  /// Server event confirming the input audio buffer was cleared.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventInputAudioBufferCleared
  {
    /// The unique ID of the server event.
    pub event_id : String,
  }

  /// Server event confirming the input audio buffer was committed.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventInputAudioBufferCommitted
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the preceding item after which the new item will be inserted.
    pub previous_item_id : Option< String >,
    /// The ID of the user message item that will be created.
    pub item_id : String,
  }

  /// Server event indicating speech start detected by VAD.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventInputAudioBufferSpeechStarted
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// Milliseconds from session start when speech was first detected.
    pub audio_start_ms : i32,
    /// The ID of the user message item that will be created when speech stops.
    pub item_id : String,
  }

  /// Server event indicating speech stop detected by VAD.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventInputAudioBufferSpeechStopped
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// Milliseconds since session start when speech stopped.
    pub audio_end_ms : i32,
    /// The ID of the user message item that will be created.
    pub item_id : String,
  }

  /// Server event providing updated rate limit information.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventRateLimitsUpdated
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// List of rate limit information.
    pub rate_limits : Vec< RateLimitInfo >,
  }

  /// Contains information about a specific rate limit.
  ///
  /// # Used By
  /// - `RealtimeServerEventRateLimitsUpdated`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RateLimitInfo
  {
    /// The name of the rate limit (`requests` or `tokens`).
    pub name : String,
    /// The maximum allowed value for the rate limit.
    pub limit : i32,
    /// The remaining value before the limit is reached.
    pub remaining : i32,
    /// Seconds until the rate limit resets.
    pub reset_seconds : f64,
  }

  /// Server event containing a delta of model-generated audio.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventResponseAudioDelta
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the response.
    pub response_id : String,
    /// The ID of the item containing the audio.
    pub item_id : String,
    /// The index of the output item in the response.
    pub output_index : i32,
    /// The index of the content part in the item's content array.
    pub content_index : i32,
    /// Base64-encoded audio data delta.
    pub delta : String,
  }

  /// Server event indicating the model-generated audio stream is done.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventResponseAudioDone
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the response.
    pub response_id : String,
    /// The ID of the item containing the audio.
    pub item_id : String,
    /// The index of the output item in the response.
    pub output_index : i32,
    /// The index of the content part in the item's content array.
    pub content_index : i32,
  }

  /// Server event containing a delta of the model-generated audio transcript.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventResponseAudioTranscriptDelta
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the response.
    pub response_id : String,
    /// The ID of the item containing the audio.
    pub item_id : String,
    /// The index of the output item in the response.
    pub output_index : i32,
    /// The index of the content part in the item's content array.
    pub content_index : i32,
    /// The transcript delta.
    pub delta : String,
  }

  /// Server event indicating the model-generated audio transcript is done.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventResponseAudioTranscriptDone
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the response.
    pub response_id : String,
    /// The ID of the item containing the audio.
    pub item_id : String,
    /// The index of the output item in the response.
    pub output_index : i32,
    /// The index of the content part in the item's content array.
    pub content_index : i32,
    /// The final transcript of the audio.
    pub transcript : String,
  }

  /// Server event indicating a new content part was added to an output item.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventResponseContentPartAdded
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the response.
    pub response_id : String,
    /// The ID of the item to which the content part was added.
    pub item_id : String,
    /// The index of the output item in the response.
    pub output_index : i32,
    /// The index of the content part in the item's content array.
    pub content_index : i32,
    /// The content part that was added (structure depends on content type).
    pub part : Value,
  }

  /// Server event indicating a content part is done streaming.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventResponseContentPartDone
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the response.
    pub response_id : String,
    /// The ID of the item.
    pub item_id : String,
    /// The index of the output item in the response.
    pub output_index : i32,
    /// The index of the content part in the item's content array.
    pub content_index : i32,
    /// The content part that is done (structure depends on content type).
    pub part : Value,
  }

  /// Server event indicating a new response was created.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventResponseCreated
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The created response object (initial state).
    pub response : RealtimeResponse,
  }

  /// Server event indicating a response is done streaming (completed, failed, etc.).
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventResponseDone
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The final response object (omits raw audio data).
    pub response : RealtimeResponse,
  }

  /// Server event containing a delta for function call arguments.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventResponseFunctionCallArgumentsDelta
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the response.
    pub response_id : String,
    /// The ID of the function call item.
    pub item_id : String,
    /// The index of the output item in the response.
    pub output_index : i32,
    /// The ID of the function call.
    pub call_id : String,
    /// The arguments delta as a JSON string.
    pub delta : String,
  }

  /// Server event indicating function call arguments are finalized.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventResponseFunctionCallArgumentsDone
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the response.
    pub response_id : String,
    /// The ID of the function call item.
    pub item_id : String,
    /// The index of the output item in the response.
    pub output_index : i32,
    /// The ID of the function call.
    pub call_id : String,
    /// The final arguments as a JSON string.
    pub arguments : String,
  }

  /// Server event indicating a new output item was added to the response.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventResponseOutputItemAdded
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the Response to which the item belongs.
    pub response_id : String,
    /// The index of the output item in the Response.
    pub output_index : i32,
    /// The output item that was added.
    pub item : RealtimeConversationItem,
  }

  /// Server event indicating an output item is done streaming.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventResponseOutputItemDone
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the Response to which the item belongs.
    pub response_id : String,
    /// The index of the output item in the Response.
    pub output_index : i32,
    /// The output item that was marked done.
    pub item : RealtimeConversationItem,
  }

  /// Server event containing a delta for text content.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventResponseTextDelta
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the response.
    pub response_id : String,
    /// The ID of the item containing the text.
    pub item_id : String,
    /// The index of the output item in the response.
    pub output_index : i32,
    /// The index of the content part in the item's content array.
    pub content_index : i32,
    /// The text delta.
    pub delta : String,
  }

  /// Server event indicating text content is finalized.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventResponseTextDone
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The ID of the response.
    pub response_id : String,
    /// The ID of the item containing the text.
    pub item_id : String,
    /// The index of the output item in the response.
    pub output_index : i32,
    /// The index of the content part in the item's content array.
    pub content_index : i32,
    /// The final text content.
    pub text : String,
  }

  /// Server event indicating the session was created (initial event).
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventSessionCreated
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The initial session configuration.
    pub session : RealtimeSession,
  }

  /// Server event confirming a session update.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventSessionUpdated
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The updated session configuration.
    pub session : RealtimeSession,
  }

  /// Response object returned when creating a Realtime transcription session via REST API.
  ///
  /// # Used By
  /// - `/realtime/transcription_sessions` (POST)
  /// - `RealtimeServerEventTranscriptionSessionUpdated`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventTranscriptionSessionCreated
  {
    /// Unique identifier for the session.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub id : Option< String >,
    /// The object type, always "`realtime.transcription_session`".
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub object : Option< String >,
    /// The set of modalities supported (always [`"audio"`, `"text"`] for transcription).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub modalities : Option< Vec< String > >,
    /// The format of input audio.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_format : Option< String >,
    /// Configuration for input audio transcription.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_transcription : Option< crate::components::realtime_shared::session::RealtimeSessionInputAudioTranscription >,
    /// Configuration for turn detection.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub turn_detection : Option< crate::components::realtime_shared::session::RealtimeSessionTurnDetection >,
    /// Configuration for input audio noise reduction.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_noise_reduction : Option< crate::components::realtime_shared::session::RealtimeSessionInputAudioNoiseReduction >,
    /// Items to include in the transcription response (e.g., logprobs).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub include : Option< Vec< String > >,
  }

  /// Server event confirming a transcription session update.
  ///
  /// # Used By
  /// - `RealtimeServerEvent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct RealtimeServerEventTranscriptionSessionUpdated
  {
    /// The unique ID of the server event.
    pub event_id : String,
    /// The updated transcription session configuration.
    pub session : RealtimeTranscriptionSessionCreateResponse,
  }

} // end mod private

crate ::mod_interface!
{
  exposed use
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
}
