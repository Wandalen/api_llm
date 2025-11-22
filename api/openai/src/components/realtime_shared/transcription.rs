//! Transcription-specific session structures for the Realtime API.

/// Define a private namespace for transcription-related items.
mod private
{
  // Use full paths from crate root for components
  use crate::components::realtime_shared::session::
  {
    RealtimeSessionInputAudioTranscription,
    RealtimeSessionTurnDetection,
    RealtimeSessionInputAudioNoiseReduction,
    RealtimeClientSecret,
  };

  // Serde imports
  use serde::{ Serialize, Deserialize };

  /// Response object returned when creating a Realtime transcription session via REST API.
  ///
  /// # Used By
  /// - `/realtime/transcription_sessions` (POST)
  /// - `RealtimeServerEventTranscriptionSessionUpdated`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct RealtimeTranscriptionSessionCreateResponse
  {
    /// Ephemeral key for client authentication (only present on creation via REST).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub client_secret : Option< RealtimeClientSecret >,
    /// Unique identifier for the session.
    pub id : String,
    /// The object type, always "`realtime.transcription_session`".
    pub object : String,
    /// The set of modalities supported (always [`"audio"`, `"text"`] for transcription).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub modalities : Option< Vec< String > >,
    /// The format of input audio.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_format : Option< String >,
    /// Configuration for input audio transcription.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_transcription : Option< RealtimeSessionInputAudioTranscription >,
    /// Configuration for turn detection.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub turn_detection : Option< RealtimeSessionTurnDetection >,
    /// Configuration for input audio noise reduction.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_noise_reduction : Option< RealtimeSessionInputAudioNoiseReduction >,
    /// Items to include in the transcription response (e.g., logprobs).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub include : Option< Vec< String > >,
  }

  /// Represents the request body for creating or updating a Realtime transcription session.
  ///
  /// # Used By
  /// - `/realtime/transcription_sessions` (POST)
  /// - `RealtimeClientEventTranscriptionSessionUpdate`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq, Default, former::Former ) ] // Added Serialize, Default
  pub struct RealtimeTranscriptionSessionCreateRequest
  {
    /// Supported modalities (always [`"audio"`, `"text"`] for transcription).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub modalities : Option< Vec< String > >,
    /// Format of input audio.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_format : Option< String >,
    /// Input audio transcription configuration.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_transcription : Option< RealtimeSessionInputAudioTranscription >,
    /// Turn detection configuration.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub turn_detection : Option< RealtimeSessionTurnDetection >,
    /// Input audio noise reduction configuration.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub input_audio_noise_reduction : Option< RealtimeSessionInputAudioNoiseReduction >,
    /// Items to include in the transcription response (e.g., logprobs).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub include : Option< Vec< String > >,
  }

} // end mod private

crate ::mod_interface!
{
  exposed use
  {
    RealtimeTranscriptionSessionCreateResponse,
    RealtimeTranscriptionSessionCreateRequest,
  };
}