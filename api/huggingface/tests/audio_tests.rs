//! Comprehensive tests for `HuggingFace` Audio Processing API functionality

#![ allow( clippy::doc_markdown ) ]
#![ allow( clippy::float_cmp ) ]
#![ allow( clippy::single_char_pattern ) ]

use api_huggingface::
{
  Client,
  environment::HuggingFaceEnvironmentImpl,
  secret::Secret,
  audio::
  {
  AudioInput, TranscriptionResult, AudioClassificationResult,
  SpeechGenerationResult, AudioTransformResult,
  },
  error::Result,
};
use base64::Engine;

/// Helper function to create a test client
fn create_test_client() -> Result< Client< HuggingFaceEnvironmentImpl > >
{
  let api_key = Secret::new( "test-api-key".to_string() );
  let env = HuggingFaceEnvironmentImpl::build( api_key, None )?;
  Client::build( env )
}

/// Test audio API group creation
#[ tokio::test ]
async fn test_audio_api_creation()
{
  // Setup
  let client = create_test_client().expect( "Client creation should succeed" );

  // Execution
  let audio = client.audio();

  // Verification
  assert!( core::mem::size_of_val( &audio ) > 0, "Audio API group should be created" );
}

// =============================================================================
// AudioInput Tests
// =============================================================================

/// Test AudioInput construction from bytes
#[ test ]
fn test_audio_input_from_bytes()
{
  // Setup
  let audio_data = vec![ 0x52, 0x49, 0x46, 0x46 ]; // RIFF header (WAV)

  // Execution
  let input = AudioInput::from_bytes( audio_data.clone() );

  // Verification
  match input
  {
  AudioInput::Bytes( data ) => assert_eq!( data, audio_data ),
  _ => panic!( "Expected Bytes variant" ),
  }
}

/// Test AudioInput construction from base64
#[ test ]
fn test_audio_input_from_base64()
{
  // Setup
  let base64_data = "UklGRg==";

  // Execution
  let input = AudioInput::from_base64( base64_data );

  // Verification
  match input
  {
  AudioInput::Base64( data ) => assert_eq!( data, base64_data ),
  _ => panic!( "Expected Base64 variant" ),
  }
}

/// Test AudioInput construction from URL
#[ test ]
fn test_audio_input_from_url()
{
  // Setup
  let url = "https://example.com/audio.wav";

  // Execution
  let input = AudioInput::from_url( url );

  // Verification
  match input
  {
  AudioInput::Url( data ) => assert_eq!( data, url ),
  _ => panic!( "Expected Url variant" ),
  }
}

/// Test AudioInput base64 conversion from bytes
#[ test ]
fn test_audio_input_to_base64_from_bytes()
{
  // Setup
  let audio_data = vec![ 0x52, 0x49, 0x46, 0x46 ]; // RIFF
  let input = AudioInput::from_bytes( audio_data );

  // Execution
  let base64 = input.to_base64();

  // Verification
  assert!( !base64.is_empty(), "Base64 encoding should not be empty" );
  assert_eq!( base64, "UklGRg==" ); // Standard base64 encoding of RIFF
}

/// Test AudioInput base64 conversion from existing base64
#[ test ]
fn test_audio_input_to_base64_from_base64()
{
  // Setup
  let original_base64 = "UklGRg==";
  let input = AudioInput::from_base64( original_base64 );

  // Execution
  let base64 = input.to_base64();

  // Verification
  assert_eq!( base64, original_base64, "Base64 should be returned unchanged" );
}

/// Test AudioInput base64 conversion from URL
#[ test ]
fn test_audio_input_to_base64_from_url()
{
  // Setup
  let url = "https://example.com/audio.wav";
  let input = AudioInput::from_url( url );

  // Execution
  let base64 = input.to_base64();

  // Verification
  assert_eq!( base64, url, "URL should be returned unchanged" );
}

// =============================================================================
// Data Structure Tests
// =============================================================================

/// Test TranscriptionResult creation
#[ test ]
fn test_transcription_result_creation()
{
  // Setup
  let text = "Hello world".to_string();

  // Execution
  let result = TranscriptionResult { text : text.clone() };

  // Verification
  assert_eq!( result.text, text );
}

/// Test TranscriptionResult deserialization
#[ test ]
fn test_transcription_result_deserialization()
{
  // Setup
  let json = r#"{"text": "Test transcription"}"#;

  // Execution
  let result : TranscriptionResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( result.text, "Test transcription" );
}

/// Test AudioClassificationResult creation
#[ test ]
fn test_audio_classification_result_creation()
{
  // Setup
  let label = "speech".to_string();
  let score = 0.95;

  // Execution
  let result = AudioClassificationResult
  {
  label : label.clone(),
  score,
  };

  // Verification
  assert_eq!( result.label, label );
  assert_eq!( result.score, score );
}

/// Test AudioClassificationResult deserialization
#[ test ]
fn test_audio_classification_result_deserialization()
{
  // Setup
  let json = r#"{"label": "music", "score": 0.87}"#;

  // Execution
  let result : AudioClassificationResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( result.label, "music" );
  assert_eq!( result.score, 0.87 );
}

/// Test SpeechGenerationResult creation
#[ test ]
fn test_speech_generation_result_creation()
{
  // Setup
  let audio_data = vec![ 1, 2, 3, 4, 5 ];
  let sample_rate = Some( 22050 );
  let format = Some( "wav".to_string() );

  // Execution
  let result = SpeechGenerationResult
  {
  audio_data : audio_data.clone(),
  sample_rate,
  format : format.clone(),
  };

  // Verification
  assert_eq!( result.audio_data, audio_data );
  assert_eq!( result.sample_rate, sample_rate );
  assert_eq!( result.format, format );
}

/// Test AudioTransformResult creation
#[ test ]
fn test_audio_transform_result_creation()
{
  // Setup
  let audio_data = vec![ 6, 7, 8, 9, 10 ];
  let sample_rate = Some( 16000 );
  let format = Some( "mp3".to_string() );

  // Execution
  let result = AudioTransformResult
  {
  audio_data : audio_data.clone(),
  sample_rate,
  format : format.clone(),
  };

  // Verification
  assert_eq!( result.audio_data, audio_data );
  assert_eq!( result.sample_rate, sample_rate );
  assert_eq!( result.format, format );
}

// =============================================================================
// Base64 Encoding Tests
// =============================================================================

/// Test base64 encoding of WAV header
#[ test ]
fn test_base64_encoding_wav_header()
{
  // Setup - WAV file signature
  let wav_header = vec![ 0x52, 0x49, 0x46, 0x46 ]; // RIFF
  let input = AudioInput::from_bytes( wav_header );

  // Execution
  let base64 = input.to_base64();

  // Verification
  assert!( base64.starts_with( "UklG" ), "WAV header should encode to base64 starting with 'UklG'" );
}

/// Test base64 encoding of MP3 header
#[ test ]
fn test_base64_encoding_mp3_header()
{
  // Setup - MP3 file signature
  let mp3_header = vec![ 0xFF, 0xFB ]; // MP3 sync word
  let input = AudioInput::from_bytes( mp3_header );

  // Execution
  let base64 = input.to_base64();

  // Verification
  assert!( base64.starts_with( "//" ), "MP3 header should encode to base64 starting with '//'" );
}

/// Test base64 encoding round-trip
#[ test ]
fn test_base64_roundtrip()
{
  // Setup
  let original_data = b"Test audio data for encoding";
  let input = AudioInput::from_bytes( original_data.to_vec() );

  // Execution - encode
  let encoded = input.to_base64();

  // Decode back
  let decoded = base64::prelude::BASE64_STANDARD.decode( &encoded )
  .expect( "Base64 decode should succeed" );

  // Verification
  assert_eq!( decoded, original_data, "Roundtrip encoding should preserve data" );
}

// =============================================================================
// Response Format Tests
// =============================================================================

/// Test transcription response format
#[ test ]
fn test_transcription_response_format()
{
  // Setup
  let json = r#"{"text": "This is a test transcription"}"#;

  // Execution
  let result : TranscriptionResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( result.text, "This is a test transcription" );
}

/// Test audio classification response format
#[ test ]
fn test_audio_classification_response_format()
{
  // Setup
  let json = r#"[
  {"label": "speech", "score": 0.95},
  {"label": "music", "score": 0.03},
  {"label": "noise", "score": 0.02}
  ]"#;

  // Execution
  let results : Vec< AudioClassificationResult > = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( results.len(), 3 );
  assert_eq!( results[ 0 ].label, "speech" );
  assert_eq!( results[ 0 ].score, 0.95 );
  assert_eq!( results[ 1 ].label, "music" );
  assert_eq!( results[ 2 ].label, "noise" );
}

// =============================================================================
// Edge Cases and Validation Tests
// =============================================================================

/// Test empty transcription text
#[ test ]
fn test_empty_transcription_text()
{
  // Setup
  let json = r#"{"text": ""}"#;

  // Execution
  let result : TranscriptionResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( result.text, "" );
}

/// Test very long transcription text
#[ test ]
fn test_long_transcription_text()
{
  // Setup
  let long_text = "word ".repeat( 500 );
  let json = format!( r#"{{"text": "{long_text}"}}"# );

  // Execution
  let result : TranscriptionResult = serde_json::from_str( &json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert!( result.text.len() > 500 );
  assert!( result.text.starts_with( "word " ) );
}

/// Test audio classification with zero score
#[ test ]
fn test_audio_classification_zero_score()
{
  // Setup
  let json = r#"{"label": "unknown", "score": 0.0}"#;

  // Execution
  let result : AudioClassificationResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( result.label, "unknown" );
  assert_eq!( result.score, 0.0 );
}

/// Test audio classification with perfect score
#[ test ]
fn test_audio_classification_perfect_score()
{
  // Setup
  let json = r#"{"label": "certain", "score": 1.0}"#;

  // Execution
  let result : AudioClassificationResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( result.label, "certain" );
  assert_eq!( result.score, 1.0 );
}

/// Test empty classification results
#[ test ]
fn test_empty_classification_results()
{
  // Setup
  let json = "[]";

  // Execution
  let results : Vec< AudioClassificationResult > = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( results.len(), 0, "Empty results should deserialize" );
}

/// Test speech generation result with no metadata
#[ test ]
fn test_speech_generation_result_no_metadata()
{
  // Setup
  let audio_data = vec![ 1, 2, 3 ];

  // Execution
  let result = SpeechGenerationResult
  {
  audio_data : audio_data.clone(),
  sample_rate : None,
  format : None,
  };

  // Verification
  assert_eq!( result.audio_data, audio_data );
  assert!( result.sample_rate.is_none() );
  assert!( result.format.is_none() );
}

/// Test audio transform result with large data
#[ test ]
fn test_audio_transform_result_large_data()
{
  // Setup
  let large_data = vec![ 0u8; 10000 ]; // 10KB of audio data

  // Execution
  let result = AudioTransformResult
  {
  audio_data : large_data.clone(),
  sample_rate : Some( 44100 ),
  format : Some( "wav".to_string() ),
  };

  // Verification
  assert_eq!( result.audio_data.len(), 10000 );
  assert_eq!( result.sample_rate, Some( 44100 ) );
}

/// Test transcription with special characters
#[ test ]
fn test_transcription_with_special_characters()
{
  // Setup
  let json = r#"{"text": "Hello, world! How are you? It's nice."}"#;

  // Execution
  let result : TranscriptionResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert!( result.text.contains( "," ) );
  assert!( result.text.contains( "!" ) );
  assert!( result.text.contains( "?" ) );
  assert!( result.text.contains( "'" ) );
}

/// Test transcription with unicode characters
#[ test ]
fn test_transcription_with_unicode()
{
  // Setup
  let json = r#"{"text": "Héllo wörld 你好"}"#;

  // Execution
  let result : TranscriptionResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert!( result.text.contains( "é" ) );
  assert!( result.text.contains( "ö" ) );
  assert!( result.text.contains( "你好" ) );
}

/// Test audio classification with multiple high scores
#[ test ]
fn test_audio_classification_multiple_high_scores()
{
  // Setup
  let json = r#"[
  {"label": "speech", "score": 0.48},
  {"label": "music", "score": 0.47},
  {"label": "ambient", "score": 0.05}
  ]"#;

  // Execution
  let results : Vec< AudioClassificationResult > = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( results.len(), 3 );
  assert!( results[ 0 ].score > 0.4 );
  assert!( results[ 1 ].score > 0.4 );
  assert!( results[ 2 ].score < 0.1 );
}
