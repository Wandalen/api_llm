//! Session configuration and management structures for the Realtime API.

/// Define a private namespace for session-related items.
mod private
{
  // Use full paths from crate root for components
  use crate::components::common::VoiceIdsShared;
  use crate::components::tools::Tool;

  // Serde imports
  use serde::{ Serialize, Deserialize };
  use serde_json::Value; // Keep if needed for Value type

  /// Configuration for input audio transcription in a Realtime session.
  ///
  /// # Used By
  /// - `RealtimeSession`
  /// - `RealtimeSessionCreateResponse`
  /// - `RealtimeSessionCreateRequest`
  /// - `RealtimeTranscriptionSessionCreateResponse`
  /// - `RealtimeTranscriptionSessionCreateRequest`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, former::Former ) ]
  pub struct RealtimeSessionInputAudioTranscription
  {
    /// The model to use for transcription (e.g., "whisper-1").
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub model : Option< String >,
    /// The language of the input audio (ISO-639-1 format).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub language : Option< String >,
    /// An optional text prompt to guide the transcription model.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub prompt : Option< String >,
  }

  /// Configuration for turn detection (VAD) in a Realtime session.
  ///
  /// # Used By
  /// - `RealtimeSession`
  /// - `RealtimeSessionCreateResponse`
  /// - `RealtimeSessionCreateRequest`
  /// - `RealtimeTranscriptionSessionCreateResponse`
  /// - `RealtimeTranscriptionSessionCreateRequest`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, former::Former ) ]
  pub struct RealtimeSessionTurnDetection
  {
    /// Type of turn detection (`server_vad` or `semantic_vad`). Defaults to `server_vad`.
    #[ serde( default = "default_turn_detection_type" ) ]
    pub r#type : String,
    /// Eagerness for `semantic_vad` (`low`, `medium`, `high`, `auto`). Defaults to `auto`.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub eagerness : Option< String >,
    /// Activation threshold for `server_vad` (0.0 to 1.0). Defaults to 0.5.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub threshold : Option< f64 >,
    /// Audio padding before VAD start for `server_vad` (ms). Defaults to 300ms.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub prefix_padding_ms : Option< i32 >,
    /// Silence duration to detect speech stop for `server_vad` (ms). Defaults to 500ms.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub silence_duration_ms : Option< i32 >,
    /// Whether to automatically create a response on VAD stop. Defaults to true.
    #[ serde( default = "default_true", skip_serializing_if = "is_true" ) ]
    pub create_response : Option< bool >,
    /// Whether to automatically interrupt ongoing response on VAD start. Defaults to true. Not applicable for transcription sessions.
    #[ serde( default = "default_true", skip_serializing_if = "is_true" ) ]
    pub interrupt_response : Option< bool >,
  }

  // Helper functions for default values in RealtimeSessionTurnDetection
  fn default_turn_detection_type() -> String { "server_vad".to_string() }
  #[ allow(clippy::unnecessary_wraps) ]
  fn default_true() -> Option< bool > { Some(true) }
  #[ allow(clippy::ref_option, clippy::trivially_copy_pass_by_ref) ]
  fn is_true(val : &Option< bool >) -> bool { val == &Some(true) }

  /// Configuration for input audio noise reduction.
  ///
  /// # Used By
  /// - `RealtimeSession`
  /// - `RealtimeSessionCreateResponse`
  /// - `RealtimeSessionCreateRequest`
  /// - `RealtimeTranscriptionSessionCreateResponse`
  /// - `RealtimeTranscriptionSessionCreateRequest`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, former::Former ) ]
  pub struct RealtimeSessionInputAudioNoiseReduction
  {
    /// Type of noise reduction (`near_field` or `far_field`).
    pub r#type : String,
  }

  /// Represents the configuration of an active Realtime session.
  ///
  /// # Used By
  /// - `RealtimeServerEventSessionCreated`
  /// - `RealtimeServerEventSessionUpdated`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct RealtimeSession
  {
    /// Unique identifier for the session.
    pub id : String,
    /// The set of modalities the model can respond with.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub modalities : Option< Vec< String > >,
    /// The Realtime model used for this session.
    pub model : String,
    /// The default system instructions for the session.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub instructions : Option< String >,
    /// The voice the model uses to respond.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub voice : Option< VoiceIdsShared >,
    /// The format of input audio.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_format : Option< String >,
    /// The format of output audio.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub output_audio_format : Option< String >,
    /// Configuration for input audio transcription.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_transcription : Option< RealtimeSessionInputAudioTranscription >,
    /// Configuration for turn detection.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub turn_detection : Option< RealtimeSessionTurnDetection >,
    /// Configuration for input audio noise reduction.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_noise_reduction : Option< RealtimeSessionInputAudioNoiseReduction >,
    /// Tools (functions) available to the model.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tools : Option< Vec< Tool > >,
    /// How the model chooses tools.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tool_choice : Option< String >,
    /// Sampling temperature for the model.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub temperature : Option< f64 >,
    /// Maximum number of output tokens for a single assistant response.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub max_response_output_tokens : Option< Value >, // Can be integer or "inf"
  }

  /// Represents an ephemeral client secret for authenticating Realtime API connections.
  ///
  /// # Used By
  /// - `RealtimeSessionCreateResponse`
  /// - `RealtimeTranscriptionSessionCreateResponse`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct RealtimeClientSecret
  {
    /// Ephemeral key usable in client environments.
    pub value : String,
    /// Timestamp for when the token expires (Unix seconds).
    pub expires_at : i64,
  }

  /// Response object returned when creating a Realtime session via REST API.
  ///
  /// # Used By
  /// - `/realtime/sessions` (POST)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct RealtimeSessionCreateResponse
  {
    /// Ephemeral key for client authentication.
    pub client_secret : RealtimeClientSecret,
    /// Unique identifier for the session.
    pub id : String,
    /// The object type, always "realtime.session".
    pub object : String,
    /// The set of modalities the model can respond with.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub modalities : Option< Vec< String > >,
    /// The Realtime model used for this session.
    pub model : String,
    /// The default system instructions for the session.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub instructions : Option< String >,
    /// The voice the model uses to respond.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub voice : Option< VoiceIdsShared >,
    /// The format of input audio.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_format : Option< String >,
    /// The format of output audio.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub output_audio_format : Option< String >,
    /// Configuration for input audio transcription.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_transcription : Option< RealtimeSessionInputAudioTranscription >,
    /// Configuration for turn detection.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub turn_detection : Option< RealtimeSessionTurnDetection >,
    /// Configuration for input audio noise reduction.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_noise_reduction : Option< RealtimeSessionInputAudioNoiseReduction >,
    /// Tools (functions) available to the model.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tools : Option< Vec< Tool > >,
    /// How the model chooses tools.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tool_choice : Option< String >,
    /// Sampling temperature for the model.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub temperature : Option< f64 >,
    /// Maximum number of output tokens for a single assistant response.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub max_response_output_tokens : Option< Value >, // Can be integer or "inf"
  }

  /// Represents the request body for creating or updating a Realtime session via REST or WebSocket.
  ///
  /// # Used By
  /// - `/realtime/sessions` (POST)
  /// - `RealtimeClientEventSessionUpdate`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Default, former::Former ) ] // Added Serialize, Default
  pub struct RealtimeSessionCreateRequest
  {
    /// The set of modalities the model can respond with.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub modalities : Option< Vec< String > >,
    /// The Realtime model ID.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub model : Option< String >,
    /// Default system instructions.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub instructions : Option< String >,
    /// Default voice for audio output.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub voice : Option< VoiceIdsShared >,
    /// Format of input audio.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_format : Option< String >,
    /// Format of output audio.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub output_audio_format : Option< String >,
    /// Input audio transcription configuration.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_transcription : Option< RealtimeSessionInputAudioTranscription >,
    /// Turn detection configuration.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub turn_detection : Option< RealtimeSessionTurnDetection >,
    /// Input audio noise reduction configuration.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_noise_reduction : Option< RealtimeSessionInputAudioNoiseReduction >,
    /// Default tools available to the model.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tools : Option< Vec< Tool > >,
    /// Default tool choice strategy.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tool_choice : Option< String >, // Should ideally be ToolChoice enum
    /// Default sampling temperature.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub temperature : Option< f64 >,
    /// Default maximum output tokens.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub max_response_output_tokens : Option< Value >, // Can be integer or "inf"
  }

} // end mod private

crate ::mod_interface!
{
  exposed use
  {
    RealtimeSessionInputAudioTranscription,
    RealtimeSessionTurnDetection,
    RealtimeSessionInputAudioNoiseReduction,
    RealtimeSession,
    RealtimeClientSecret,
    RealtimeSessionCreateResponse,
    RealtimeSessionCreateRequest,
  };
}