//! Audio API Types
//!
//! Data structures for audio API requests and responses.

use serde::{ Deserialize, Serialize };

/// Audio input for audio processing tasks
///
/// Supports multiple input formats for flexibility.
#[ derive( Debug, Clone ) ]
pub enum AudioInput
{
  /// Raw audio bytes (WAV, MP3, FLAC, etc.)
  Bytes( Vec< u8 > ),

  /// Base64-encoded audio
  Base64( String ),

  /// URL to audio file
  Url( String ),
}

impl AudioInput
{
  /// Create audio input from raw bytes
  #[ inline ]
  #[ must_use ]
  pub fn from_bytes( bytes : Vec< u8 > ) -> Self
  {
  Self::Bytes( bytes )
  }

  /// Create audio input from base64 string
  #[ inline ]
  #[ must_use ]
  pub fn from_base64( data : impl Into< String > ) -> Self
  {
  Self::Base64( data.into() )
  }

  /// Create audio input from URL
  #[ inline ]
  #[ must_use ]
  pub fn from_url( url : impl Into< String > ) -> Self
  {
  Self::Url( url.into() )
  }

  /// Convert to base64 for API transmission
  #[ inline ]
  #[ must_use ]
  pub fn to_base64( &self ) -> String
  {
  match self
  {
      Self::Bytes( bytes ) => base64_encode( bytes ),
      Self::Base64( data ) => data.clone(),
      Self::Url( url ) => url.clone(), // URLs sent as-is
  }
  }
}

/// Encode bytes to base64
fn base64_encode( bytes : &[ u8 ] ) -> String
{
  use base64::{ Engine, engine::general_purpose };
  general_purpose::STANDARD.encode( bytes )
}

/// Automatic Speech Recognition (ASR) result
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct TranscriptionResult
{
  /// Transcribed text
  pub text : String,
}

/// Audio classification result
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct AudioClassificationResult
{
  /// Predicted label
  pub label : String,

  /// Confidence score (0.0 - 1.0)
  pub score : f64,
}

/// Text-to-Speech generation result
#[ derive( Debug, Clone ) ]
pub struct SpeechGenerationResult
{
  /// Generated audio data
  pub audio_data : Vec< u8 >,

  /// Sample rate (Hz)
  pub sample_rate : Option< u32 >,

  /// Audio format (e.g., "wav", "mp3")
  pub format : Option< String >,
}

/// Audio-to-audio transformation result
#[ derive( Debug, Clone ) ]
pub struct AudioTransformResult
{
  /// Transformed audio data
  pub audio_data : Vec< u8 >,

  /// Sample rate (Hz)
  pub sample_rate : Option< u32 >,

  /// Audio format
  pub format : Option< String >,
}

#[ cfg( test ) ]
#[ allow( clippy::float_cmp ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn test_audio_input_from_bytes()
  {
  let bytes = vec![ 0x52, 0x49, 0x46, 0x46 ]; // RIFF header (WAV)
  let input = AudioInput::from_bytes( bytes.clone() );

  match input
  {
      AudioInput::Bytes( b ) => assert_eq!( b, bytes ),
      _ => panic!( "Wrong variant" ),
  }
  }

  #[ test ]
  fn test_audio_input_from_base64()
  {
  let data = "UklGRg=="; // Base64 for "RIFF"
  let input = AudioInput::from_base64( data );

  match input
  {
      AudioInput::Base64( d ) => assert_eq!( d, data ),
      _ => panic!( "Wrong variant" ),
  }
  }

  #[ test ]
  fn test_audio_input_from_url()
  {
  let url = "https://example.com/audio.wav";
  let input = AudioInput::from_url( url );

  match input
  {
      AudioInput::Url( u ) => assert_eq!( u, url ),
      _ => panic!( "Wrong variant" ),
  }
  }

  #[ test ]
  fn test_audio_input_to_base64_from_bytes()
  {
  let bytes = vec![ 0x52, 0x49, 0x46, 0x46 ]; // RIFF
  let input = AudioInput::from_bytes( bytes );

  let base64 = input.to_base64();
  assert_eq!( base64, "UklGRg==" );
  }

  #[ test ]
  fn test_transcription_result_creation()
  {
  let result = TranscriptionResult
  {
      text : "Hello world".to_string(),
  };

  assert_eq!( result.text, "Hello world" );
  }

  #[ test ]
  fn test_audio_classification_result_creation()
  {
  let result = AudioClassificationResult
  {
      label : "speech".to_string(),
      score : 0.95,
  };

  assert_eq!( result.label, "speech" );
  assert_eq!( result.score, 0.95 );
  }

  #[ test ]
  fn test_speech_generation_result_creation()
  {
  let result = SpeechGenerationResult
  {
      audio_data : vec![ 1, 2, 3, 4 ],
      sample_rate : Some( 22050 ),
      format : Some( "wav".to_string() ),
  };

  assert_eq!( result.audio_data.len(), 4 );
  assert_eq!( result.sample_rate, Some( 22050 ) );
  }

  #[ test ]
  fn test_audio_transform_result_creation()
  {
  let result = AudioTransformResult
  {
      audio_data : vec![ 5, 6, 7, 8 ],
      sample_rate : Some( 16000 ),
      format : Some( "mp3".to_string() ),
  };

  assert_eq!( result.audio_data.len(), 4 );
  assert_eq!( result.format, Some( "mp3".to_string() ) );
  }
}
