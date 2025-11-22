//! Audio processing functionality for the Ollama API client
//!
//! This module provides comprehensive audio processing capabilities including:
//! - Speech-to-text conversion using Ollama models
//! - Text-to-speech output generation
//! - Audio streaming with format validation
//! - Real-time voice chat integration
//!
//! All functionality follows the "Thin Client, Rich API" governing principle,
//! providing explicit control with transparent API mapping to Ollama endpoints.

use serde::{ Serialize, Deserialize };
use core::time::Duration;

/// Audio format enumeration supporting common audio file types
#[ derive( Debug, Clone, PartialEq, Serialize, Deserialize ) ]
pub enum AudioFormat
{
  /// WAV (Waveform Audio File Format) - uncompressed audio
  Wav,
  /// MP3 (MPEG Audio Layer 3) - compressed audio
  Mp3,
  /// FLAC (Free Lossless Audio Codec) - lossless compression
  Flac,
  /// OGG (Ogg Vorbis) - open-source compressed audio
  Ogg,
}

impl AudioFormat
{
  /// Detect audio format from raw byte data by examining file headers
  ///
  /// # Arguments
  /// * `data` - Raw audio file bytes to analyze
  ///
  /// # Returns
  /// * `Some(AudioFormat)` if format is recognized
  /// * `None` if format cannot be determined
  #[ inline ]
  #[ must_use ]
  pub fn detect_format( data : &[ u8 ] ) -> Option< Self >
  {
    if data.len() < 12
    {
      return None;
    }

    // Check for WAV format : "RIFF" header + "WAVE" identifier
    if data.starts_with( b"RIFF" ) && &data[ 8..12 ] == b"WAVE"
    {
      Some( AudioFormat::Wav )
    }
    // Check for MP3 format : Frame sync patterns
    else if data.starts_with( b"\xFF\xFB" ) || data.starts_with( b"\xFF\xFA" )
    {
      Some( AudioFormat::Mp3 )
    }
    // Check for FLAC format : "fLaC" signature
    else if data.starts_with( b"fLaC" )
    {
      Some( AudioFormat::Flac )
    }
    // Check for OGG format : "OggS" signature
    else if data.starts_with( b"OggS" )
    {
      Some( AudioFormat::Ogg )
    }
    else
    {
      None
    }
  }

  /// Get the MIME type for this audio format
  #[ inline ]
  #[ must_use ]
  pub fn mime_type( &self ) -> &'static str
  {
    match self
    {
      AudioFormat::Wav => "audio/wav",
      AudioFormat::Mp3 => "audio/mpeg",
      AudioFormat::Flac => "audio/flac",
      AudioFormat::Ogg => "audio/ogg",
    }
  }

  /// Get the file extension for this audio format
  #[ inline ]
  #[ must_use ]
  pub fn file_extension( &self ) -> &'static str
  {
    match self
    {
      AudioFormat::Wav => "wav",
      AudioFormat::Mp3 => "mp3",
      AudioFormat::Flac => "flac",
      AudioFormat::Ogg => "ogg",
    }
  }
}

/// Request structure for speech-to-text conversion
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct SpeechToTextRequest
{
  /// Model name to use for speech recognition (e.g., "whisper")
  pub model : String,
  /// Raw audio data bytes
  pub audio_data : Vec< u8 >,
  /// Audio format specification
  pub format : AudioFormat,
  /// Optional language hint for recognition (ISO 639-1 code)
  pub language : Option< String >,
  /// Additional model-specific options
  pub options : Option< serde_json::Value >,
}

/// Response structure for speech-to-text conversion
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct SpeechToTextResponse
{
  /// Transcribed text from the audio
  pub text : String,
  /// Confidence score for the transcription (0.0 to 1.0)
  pub confidence : Option< f64 >,
  /// Detected or specified language of the audio
  pub language : Option< String >,
  /// Processing duration in seconds
  pub duration : Option< f64 >,
  /// Additional metadata from the model
  pub metadata : Option< serde_json::Value >,
}

/// Request structure for text-to-speech generation
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct TextToSpeechRequest
{
  /// Model name to use for speech synthesis (e.g., "tts-1")
  pub model : String,
  /// Text to convert to speech
  pub text : String,
  /// Voice selection (model-dependent)
  pub voice : Option< String >,
  /// Output audio format
  pub format : AudioFormat,
  /// Speech speed multiplier (1.0 = normal)
  pub speed : Option< f64 >,
  /// Additional synthesis options
  pub options : Option< serde_json::Value >,
}

/// Response structure for text-to-speech generation
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct TextToSpeechResponse
{
  /// Generated audio data bytes
  pub audio_data : Vec< u8 >,
  /// Audio format of the output
  pub format : AudioFormat,
  /// Duration of the generated audio in seconds
  pub duration : Option< f64 >,
  /// Sample rate of the generated audio
  pub sample_rate : Option< u32 >,
  /// Additional metadata from synthesis
  pub metadata : Option< serde_json::Value >,
}

/// Request structure for audio streaming operations
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct AudioStreamRequest
{
  /// Model name for audio processing
  pub model : String,
  /// Audio data to stream
  pub audio_data : Vec< u8 >,
  /// Input audio format
  pub format : AudioFormat,
  /// Enable streaming mode
  pub stream : bool,
  /// Chunk size for streaming (bytes)
  pub chunk_size : Option< usize >,
  /// Stream timeout in seconds
  pub timeout : Option< u64 >,
}

/// Individual chunk in an audio stream
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct AudioStreamChunk
{
  /// Audio data for this chunk
  pub data : Vec< u8 >,
  /// Timestamp within the overall audio stream
  pub timestamp : f64,
  /// Whether this is the final chunk in the stream
  pub is_final : bool,
  /// Sequence number for ordering
  pub sequence : Option< u64 >,
}

/// Request structure for voice chat functionality
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct VoiceChatRequest
{
  /// Model name for conversation processing
  pub model : String,
  /// Input audio from the user
  pub audio_input : Vec< u8 >,
  /// Format of the input audio
  pub input_format : AudioFormat,
  /// Desired format for the response audio
  pub output_format : AudioFormat,
  /// Conversation session identifier
  pub conversation_id : Option< String >,
  /// Enable real-time processing
  pub real_time : bool,
  /// Voice selection for response
  pub voice : Option< String >,
  /// Additional chat context
  pub context : Option< serde_json::Value >,
}

/// Response structure for voice chat functionality
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct VoiceChatResponse
{
  /// Audio response from the model
  pub audio_response : Vec< u8 >,
  /// Text version of the response (optional)
  pub text_response : Option< String >,
  /// Format of the output audio
  pub output_format : AudioFormat,
  /// Conversation session identifier
  pub conversation_id : String,
  /// Processing latency in milliseconds
  pub latency_ms : Option< u64 >,
  /// Additional response metadata
  pub metadata : Option< serde_json::Value >,
}

/// Configuration for audio processing operations
#[ derive( Debug, Clone ) ]
pub struct AudioProcessingConfig
{
  /// Audio sample rate in Hz
  sample_rate : u32,
  /// Bit depth for audio processing
  bit_depth : u16,
  /// Number of audio channels
  channels : u8,
  /// Buffer size for processing
  buffer_size : usize,
  /// Default timeout for audio operations
  timeout : Duration,
}

impl AudioProcessingConfig
{
  /// Create a new audio processing configuration with defaults
  pub fn new() -> Self
  {
    Self
    {
      sample_rate : 44100,
      bit_depth : 16,
      channels : 1,
      buffer_size : 512,
      timeout : Duration::from_secs( 30 ),
    }
  }

  /// Set the sample rate
  #[ inline ]
  #[ must_use ]
  pub fn with_sample_rate( mut self, rate : u32 ) -> Self
  {
    self.sample_rate = rate;
    self
  }

  /// Set the bit depth
  #[ inline ]
  #[ must_use ]
  pub fn with_bit_depth( mut self, depth : u16 ) -> Self
  {
    self.bit_depth = depth;
    self
  }

  /// Set the number of channels
  #[ inline ]
  #[ must_use ]
  pub fn with_channels( mut self, channels : u8 ) -> Self
  {
    self.channels = channels;
    self
  }

  /// Set the buffer size
  #[ inline ]
  #[ must_use ]
  pub fn with_buffer_size( mut self, size : usize ) -> Self
  {
    self.buffer_size = size;
    self
  }

  /// Set the timeout duration
  #[ inline ]
  #[ must_use ]
  pub fn with_timeout( mut self, timeout : Duration ) -> Self
  {
    self.timeout = timeout;
    self
  }

  /// Get the sample rate
  #[ inline ]
  #[ must_use ]
  pub fn sample_rate( &self ) -> u32 { self.sample_rate }

  /// Get the bit depth
  #[ inline ]
  #[ must_use ]
  pub fn bit_depth( &self ) -> u16 { self.bit_depth }

  /// Get the number of channels
  #[ inline ]
  #[ must_use ]
  pub fn channels( &self ) -> u8 { self.channels }

  /// Get the buffer size
  #[ inline ]
  #[ must_use ]
  pub fn buffer_size( &self ) -> usize { self.buffer_size }

  /// Get the timeout duration
  #[ inline ]
  #[ must_use ]
  pub fn timeout( &self ) -> Duration { self.timeout }
}

impl Default for AudioProcessingConfig
{
  fn default() -> Self
  {
    Self::new()
  }
}

/// Stream receiver for audio processing operations
#[ derive( Debug ) ]
pub struct AudioStreamReceiver
{
  // Internal stream handling - implementation depends on the streaming backend
  // This would typically wrap a tokio::sync::mpsc::Receiver or similar
  inner : tokio::sync::mpsc::Receiver< Result< AudioStreamChunk, String > >,
}

impl AudioStreamReceiver
{
  /// Create a new audio stream receiver
  #[ allow( dead_code ) ]
  pub( crate ) fn new( receiver : tokio::sync::mpsc::Receiver< Result< AudioStreamChunk, String > > ) -> Self
  {
    Self { inner : receiver }
  }

  /// Receive the next chunk from the audio stream
  ///
  /// # Returns
  /// * `Some(Ok(chunk))` - Successfully received audio chunk
  /// * `Some(Err(error))` - Error occurred during streaming
  /// * `None` - Stream has ended
  pub async fn recv( &mut self ) -> Option< Result< AudioStreamChunk, String > >
  {
    self.inner.recv().await
  }

  /// Close the stream receiver
  pub fn close( &mut self )
  {
    self.inner.close();
  }
}