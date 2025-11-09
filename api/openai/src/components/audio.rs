//! Structures related to audio transcription and translation.

/// Define a private namespace for all its items.
mod private
{
  // Use full paths from crate root for components
  use crate::components::common::LogProbProperties;
  // Serde imports
  use serde::{ Serialize, Deserialize };

  /// Represents a segment of transcribed audio with timing information.
  ///
  /// # Used By
  /// - `CreateTranscriptionResponseVerboseJson`
  /// - `CreateTranslationResponseVerboseJson`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct TranscriptionSegment
  {
    /// Unique identifier of the segment.
    pub id : i32,
    /// Seek offset of the segment.
    pub seek : i32,
    /// Start time of the segment in seconds.
    pub start : f64,
    /// End time of the segment in seconds.
    pub end : f64,
    /// Text content of the segment.
    pub text : String,
    /// Array of token IDs for the text content.
    pub tokens : Vec< i32 >,
    /// Temperature parameter used for generating the segment.
    pub temperature : f64,
    /// Average logprob of the segment. If the value is lower than -1, consider the logprobs failed.
    pub avg_logprob : f64,
    /// Compression ratio of the segment. If the value is greater than 2.4, consider the compression failed.
    pub compression_ratio : f64,
    /// Probability of no speech in the segment. If the value is higher than 1.0 and the `avg_logprob` is below -1, consider this segment silent.
    pub no_speech_prob : f64,
  }

  /// Represents a single word extracted from the transcription with timing information.
  ///
  /// # Used By
  /// - `CreateTranscriptionResponseVerboseJson`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct TranscriptionWord
  {
    /// The text content of the word.
    pub word : String,
    /// Start time of the word in seconds.
    pub start : f64,
    /// End time of the word in seconds.
    pub end : f64,
  }

  /// Represents a basic transcription response containing the transcribed text.
  ///
  /// # Used By
  /// - `/audio/transcriptions` (POST response, when `response_format` is `json` or `text`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct CreateTranscriptionResponseJson
  {
    /// The transcribed text.
    pub text : String,
    /// The log probabilities of the tokens, if requested.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub logprobs : Option< Vec< LogProbProperties > >,
  }

  /// Represents a verbose transcription response including language, duration, and segment/word timings.
  ///
  /// # Used By
  /// - `/audio/transcriptions` (POST response, when `response_format` is `verbose_json`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct CreateTranscriptionResponseVerboseJson
  {
    /// The language of the input audio.
    pub language : String,
    /// The duration of the input audio in seconds.
    pub duration : f64,
    /// The transcribed text.
    pub text : String,
    /// Extracted words and their corresponding timestamps (if requested).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub words : Option< Vec< TranscriptionWord > >,
    /// Segments of the transcribed text and their corresponding details (if requested or by default in verbose).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub segments : Option< Vec< TranscriptionSegment > >,
  }

  /// Represents a basic translation response containing the translated text.
  ///
  /// # Used By
  /// - `/audio/translations` (POST response, when `response_format` is `json` or `text`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct CreateTranslationResponseJson
  {
    /// The translated text (always English).
    pub text : String,
  }

  /// Represents a verbose translation response including language, duration, and segments.
  ///
  /// # Used By
  /// - `/audio/translations` (POST response, when `response_format` is `verbose_json`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct CreateTranslationResponseVerboseJson
  {
    /// The language of the output translation (always `english`).
    pub language : String,
    /// The duration of the input audio in seconds.
    pub duration : f64,
    /// The translated text.
    pub text : String,
    /// Segments of the translated text and their corresponding details.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub segments : Option< Vec< TranscriptionSegment > >,
  }

  /// Represents a delta event during streamed transcription, containing a chunk of text.
  ///
  /// # Used By
  /// - `CreateTranscriptionResponseStreamEvent`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct TranscriptTextDeltaEvent
  {
    /// The event type, always `transcript.text.delta`.
    pub r#type : String,
    /// The text delta that was additionally transcribed.
    pub delta : String,
    /// Log probabilities for the delta, if requested.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub logprobs : Option< Vec< LogProbProperties > >,
  }

  /// Represents the final event in a streamed transcription, containing the full text.
  ///
  /// # Used By
  /// - `CreateTranscriptionResponseStreamEvent`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct TranscriptTextDoneEvent
  {
    /// The event type, always `transcript.text.done`.
    pub r#type : String,
    /// The complete transcribed text.
    pub text : String,
    /// Log probabilities for the full transcription, if requested.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub logprobs : Option< Vec< LogProbProperties > >,
  }

  /// Represents events received during a streamed transcription response.
  ///
  /// # Used By
  /// - `/audio/transcriptions` (POST response, when `stream=true`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( untagged ) ]
  pub enum CreateTranscriptionResponseStreamEvent
  {
    /// A delta event containing a chunk of the transcription.
    Delta( TranscriptTextDeltaEvent ),
    /// The final event containing the complete transcription.
    Done( TranscriptTextDoneEvent ),
  }

  /// Request parameters for creating speech audio from text.
  ///
  /// # Used By
  /// - `/audio/speech` (POST request)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  pub struct CreateSpeechRequest
  {
    /// The TTS model to use for speech generation.
    pub model : String,
    /// The text to generate audio for. Max 4096 characters.
    pub input : String,
    /// The voice to use when generating the audio.
    pub voice : SpeechVoice,
    /// The format to output the audio in.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub response_format : Option< SpeechResponseFormat >,
    /// The speed of the generated audio.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub speed : Option< f64 >,
  }

  /// Available voices for speech generation.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( rename_all = "lowercase" ) ]
  pub enum SpeechVoice
  {
    /// Alloy voice
    Alloy,
    /// Echo voice
    Echo,
    /// Fable voice
    Fable,
    /// Onyx voice
    Onyx,
    /// Nova voice
    Nova,
    /// Shimmer voice
    Shimmer,
  }

  /// Available response formats for speech generation.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( rename_all = "lowercase" ) ]
  pub enum SpeechResponseFormat
  {
    /// MP3 format
    Mp3,
    /// Opus format
    Opus,
    /// AAC format
    Aac,
    /// FLAC format
    Flac,
    /// WAV format
    Wav,
    /// PCM format
    Pcm,
  }

  /// Request parameters for transcribing audio to text.
  ///
  /// # Used By
  /// - `/audio/transcriptions` (POST request)
  #[ derive( Debug, Clone ) ]
  pub struct CreateTranscriptionRequest
  {
    /// The audio file to transcribe.
    pub file : Vec< u8 >,
    /// Filename for the audio file.
    pub filename : String,
    /// The model to use for transcription.
    pub model : String,
    /// The language of the input audio (ISO-639-1 format).
    pub language : Option< String >,
    /// An optional text to guide the model's style.
    pub prompt : Option< String >,
    /// The format of the transcript output.
    pub response_format : Option< TranscriptionResponseFormat >,
    /// The sampling temperature between 0 and 1.
    pub temperature : Option< f64 >,
    /// Word-level timestamps.
    pub timestamp_granularities : Option< Vec< TimestampGranularity > >,
  }

  /// Request parameters for translating audio to English text.
  ///
  /// # Used By
  /// - `/audio/translations` (POST request)
  #[ derive( Debug, Clone ) ]
  pub struct CreateTranslationRequest
  {
    /// The audio file to translate.
    pub file : Vec< u8 >,
    /// Filename for the audio file.
    pub filename : String,
    /// The model to use for translation.
    pub model : String,
    /// An optional text to guide the model's style.
    pub prompt : Option< String >,
    /// The format of the transcript output.
    pub response_format : Option< TranscriptionResponseFormat >,
    /// The sampling temperature between 0 and 1.
    pub temperature : Option< f64 >,
  }

  /// Available response formats for transcription and translation.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( rename_all = "snake_case" ) ]
  pub enum TranscriptionResponseFormat
  {
    /// JSON format
    Json,
    /// Plain text format
    Text,
    /// `SubRip` Subtitle format
    Srt,
    /// Verbose JSON with metadata
    VerboseJson,
    /// `WebVTT` format
    Vtt,
  }

  /// Timestamp granularity options.
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
  #[ serde( rename_all = "lowercase" ) ]
  pub enum TimestampGranularity
  {
    /// Word-level timestamps
    Word,
    /// Segment-level timestamps
    Segment,
  }
} // end mod private

crate ::mod_interface!
{
  exposed use
  {
    TranscriptionSegment,
    TranscriptionWord,
    CreateTranscriptionResponseJson,
    CreateTranscriptionResponseVerboseJson,
    CreateTranslationResponseJson,
    CreateTranslationResponseVerboseJson,
    TranscriptTextDeltaEvent,
    TranscriptTextDoneEvent,
    CreateTranscriptionResponseStreamEvent,
    CreateSpeechRequest,
    SpeechVoice,
    SpeechResponseFormat,
    CreateTranscriptionRequest,
    CreateTranslationRequest,
    TranscriptionResponseFormat,
    TimestampGranularity,
  };
}