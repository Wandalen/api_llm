//! Audio Processing Tests
//!
//! Comprehensive test suite for audio processing capabilities in the Ollama API client.
//! Tests speech-to-text, text-to-speech, audio streaming, and real-time voice chat.
//!
//! Note : These tests focus on API structure and error handling rather than actual
//! audio processing, since the feature is not yet implemented in the Ollama API.

#[ cfg( feature = "audio_processing" ) ]
#[ allow( clippy::std_instead_of_core ) ] // std required for time operations
mod audio_processing_tests
{
  use api_ollama::{
    AudioFormat, SpeechToTextRequest,
    TextToSpeechRequest, TextToSpeechResponse, AudioStreamRequest,
    AudioStreamChunk, VoiceChatRequest, VoiceChatResponse,
    AudioProcessingConfig
  };
  use std::time::{ Duration, Instant };

  /// Sample audio data for testing
  const SAMPLE_AUDIO_WAV: &[ u8 ] = b"RIFF\x24\x08\x00\x00WAVE";
  const SAMPLE_AUDIO_MP3: &[ u8 ] = b"\xFF\xFB\x90\x00\x00\x00\x00\x00\x00\x00\x00\x00";

  /// Test speech-to-text request structure and validation
  #[ tokio::test ]
  async fn test_speech_to_text_request_structure()
  {
    // Test creating a valid speech-to-text request
    let speech_request = SpeechToTextRequest
    {
      model : "whisper".to_string(),
      audio_data : SAMPLE_AUDIO_WAV.to_vec(),
      format : AudioFormat::Wav,
      language : Some( "en".to_string() ),
      options : None,
    };

    // Test request structure
    assert_eq!( speech_request.model, "whisper" );
    assert_eq!( speech_request.audio_data.len(), SAMPLE_AUDIO_WAV.len() );
    assert_eq!( speech_request.format, AudioFormat::Wav );
    assert_eq!( speech_request.language, Some( "en".to_string() ) );
    assert!( speech_request.options.is_none() );

    println!( "✓ Speech-to-text request structure validation successful" );

    // Test serialization
    let serialized = serde_json::to_string( &speech_request );
    assert!( serialized.is_ok() );
    println!( "✓ Speech-to-text request serialization successful" );
  }

  /// Test text-to-speech request structure and validation
  #[ tokio::test ]
  async fn test_text_to_speech_request_structure()
  {
    // Create text-to-speech request
    let tts_request = TextToSpeechRequest
    {
      model : "tts-1".to_string(),
      text : "Hello, this is a test of text-to-speech functionality.".to_string(),
      voice : Some( "alloy".to_string() ),
      format : AudioFormat::Mp3,
      speed : Some( 1.0 ),
      options : None,
    };

    // Test request structure validation
    assert_eq!( tts_request.model, "tts-1" );
    assert!( !tts_request.text.is_empty() );
    assert_eq!( tts_request.voice, Some( "alloy".to_string() ) );
    assert_eq!( tts_request.format, AudioFormat::Mp3 );
    assert_eq!( tts_request.speed, Some( 1.0 ) );

    println!( "✓ Text-to-speech request structure validation successful" );

    // Test serialization
    let serialized = serde_json::to_string( &tts_request );
    assert!( serialized.is_ok() );
    println!( "✓ Text-to-speech request serialization successful" );

    // Test response structure
    let response = TextToSpeechResponse
    {
      audio_data : SAMPLE_AUDIO_MP3.to_vec(),
      format : AudioFormat::Mp3,
      duration : Some( 5.0 ),
      sample_rate : Some( 44100 ),
      metadata : None,
    };

    assert!( !response.audio_data.is_empty() );
    assert_eq!( response.format, AudioFormat::Mp3 );
    assert_eq!( response.duration, Some( 5.0 ) );

    println!( "✓ Text-to-speech response structure validation successful" );
  }

  /// Test audio streaming request structure and validation
  #[ tokio::test ]
  async fn test_audio_streaming_request_structure()
  {
    // Test WAV format streaming request
    let wav_request = AudioStreamRequest
    {
      model : "whisper".to_string(),
      audio_data : SAMPLE_AUDIO_WAV.to_vec(),
      format : AudioFormat::Wav,
      stream : true,
      chunk_size : Some( 1024 ),
      timeout : Some( 30 ),
    };

    // Test request structure validation
    assert_eq!( wav_request.model, "whisper" );
    assert_eq!( wav_request.audio_data.len(), SAMPLE_AUDIO_WAV.len() );
    assert_eq!( wav_request.format, AudioFormat::Wav );
    assert!( wav_request.stream );
    assert_eq!( wav_request.chunk_size, Some( 1024 ) );

    println!( "✓ WAV audio streaming request structure validation successful" );

    // Test MP3 format streaming request
    let mp3_request = AudioStreamRequest
    {
      model : "whisper".to_string(),
      audio_data : SAMPLE_AUDIO_MP3.to_vec(),
      format : AudioFormat::Mp3,
      stream : true,
      chunk_size : Some( 512 ),
      timeout : Some( 30 ),
    };

    assert_eq!( mp3_request.format, AudioFormat::Mp3 );
    assert_eq!( mp3_request.chunk_size, Some( 512 ) );

    println!( "✓ MP3 audio streaming request structure validation successful" );

    // Test audio stream chunk structure
    let chunk = AudioStreamChunk
    {
      data : SAMPLE_AUDIO_WAV.to_vec(),
      timestamp : 1.5,
      is_final : false,
      sequence : Some( 1 ),
    };

    assert!( !chunk.data.is_empty() );
    assert!( (chunk.timestamp - 1.5).abs() < f64::EPSILON );
    assert!( !chunk.is_final );

    println!( "✓ Audio stream chunk structure validation successful" );
  }

  /// Test voice chat request and response structure validation
  #[ tokio::test ]
  async fn test_voice_chat_structure_validation()
  {
    // Create voice chat session request
    let chat_request = VoiceChatRequest
    {
      model : "llama3.2".to_string(),
      audio_input : SAMPLE_AUDIO_WAV.to_vec(),
      input_format : AudioFormat::Wav,
      output_format : AudioFormat::Mp3,
      conversation_id : Some( "test-chat-001".to_string() ),
      real_time : true,
      voice : Some( "alloy".to_string() ),
      context : None,
    };

    // Test request structure validation
    assert_eq!( chat_request.model, "llama3.2" );
    assert_eq!( chat_request.audio_input.len(), SAMPLE_AUDIO_WAV.len() );
    assert_eq!( chat_request.input_format, AudioFormat::Wav );
    assert_eq!( chat_request.output_format, AudioFormat::Mp3 );
    assert_eq!( chat_request.conversation_id, Some( "test-chat-001".to_string() ) );
    assert!( chat_request.real_time );

    println!( "✓ Voice chat request structure validation successful" );

    // Test voice chat response structure
    let response = VoiceChatResponse
    {
      audio_response : SAMPLE_AUDIO_MP3.to_vec(),
      text_response : Some( "Hello! How can I help you today?".to_string() ),
      output_format : AudioFormat::Mp3,
      conversation_id : "test-chat-001".to_string(),
      latency_ms : Some( 150 ),
      metadata : None,
    };

    assert!( !response.audio_response.is_empty() );
    assert!( response.text_response.is_some() );
    assert_eq!( response.output_format, AudioFormat::Mp3 );
    assert_eq!( response.conversation_id, "test-chat-001" );

    println!( "✓ Voice chat response structure validation successful" );
  }

  /// Test audio format validation and error handling
  #[ tokio::test ]
  async fn test_audio_format_validation()
  {
    // Test with invalid audio data
    let invalid_request = SpeechToTextRequest
    {
      model : "whisper".to_string(),
      audio_data : b"INVALID_AUDIO_DATA".to_vec(),
      format : AudioFormat::Wav,
      language : None,
      options : None,
    };

    // Test structure creation with invalid data
    assert_eq!( invalid_request.model, "whisper" );
    assert_eq!( invalid_request.audio_data, b"INVALID_AUDIO_DATA" );
    assert_eq!( invalid_request.format, AudioFormat::Wav );
    assert!( invalid_request.language.is_none() );
    assert!( invalid_request.options.is_none() );

    println!( "✓ Invalid audio data request structure validation successful" );

    // Test format detection
    let wav_detected = AudioFormat::detect_format( SAMPLE_AUDIO_WAV );
    assert_eq!( wav_detected, Some( AudioFormat::Wav ) );

    let mp3_detected = AudioFormat::detect_format( SAMPLE_AUDIO_MP3 );
    assert_eq!( mp3_detected, Some( AudioFormat::Mp3 ) );

    let invalid_detected = AudioFormat::detect_format( b"INVALID_AUDIO_DATA" );
    assert_eq!( invalid_detected, None );

    println!( "✓ Audio format detection validation successful" );
  }

  /// Test audio processing request serialization performance
  #[ tokio::test ]
  async fn test_audio_processing_serialization_performance()
  {
    // Test speech-to-text request serialization performance
    let speech_request = SpeechToTextRequest
    {
      model : "whisper".to_string(),
      audio_data : SAMPLE_AUDIO_WAV.to_vec(),
      format : AudioFormat::Wav,
      language : Some( "en".to_string() ),
      options : None,
    };

    let start_time = Instant::now();
    let serialized = serde_json::to_string( &speech_request );
    let speech_latency = start_time.elapsed();

    assert!( serialized.is_ok() );
    assert!( speech_latency < Duration::from_millis( 10 ) ); // Should be very fast

    // Test text-to-speech request serialization performance
    let tts_request = TextToSpeechRequest
    {
      model : "tts-1".to_string(),
      text : "Performance test".to_string(),
      voice : Some( "alloy".to_string() ),
      format : AudioFormat::Mp3,
      speed : Some( 1.0 ),
      options : None,
    };

    let start_time = Instant::now();
    let tts_serialized = serde_json::to_string( &tts_request );
    let tts_latency = start_time.elapsed();

    assert!( tts_serialized.is_ok() );
    assert!( tts_latency < Duration::from_millis( 10 ) );

    println!( "✓ Audio processing serialization performance:" );
    println!( "  Speech-to-text serialization latency : {speech_latency:?}" );
    println!( "  Text-to-speech serialization latency : {tts_latency:?}" );
  }

  /// Test comprehensive audio format support
  #[ tokio::test ]
  async fn test_comprehensive_audio_format_support()
  {
    // Test all supported audio formats
    let formats = vec![ AudioFormat::Wav, AudioFormat::Mp3, AudioFormat::Flac, AudioFormat::Ogg ];

    for format in formats
    {
      // Test format in request structures
      let speech_request = SpeechToTextRequest
      {
        model : "whisper".to_string(),
        audio_data : SAMPLE_AUDIO_WAV.to_vec(),
        format : format.clone(),
        language : Some( "en".to_string() ),
        options : None,
      };

      assert_eq!( speech_request.format, format );

      let tts_request = TextToSpeechRequest
      {
        model : "tts-1".to_string(),
        text : "Test".to_string(),
        voice : Some( "alloy".to_string() ),
        format : format.clone(),
        speed : Some( 1.0 ),
        options : None,
      };

      assert_eq!( tts_request.format, format );
    }

    println!( "✓ Comprehensive audio format support validation successful" );
  }

  /// Test audio processing configuration structure
  #[ tokio::test ]
  async fn test_audio_processing_configuration_structure()
  {
    // Test audio processing configuration builder pattern
    let audio_config = AudioProcessingConfig::new()
      .with_sample_rate( 44100 )
      .with_bit_depth( 16 )
      .with_channels( 2 )
      .with_buffer_size( 1024 );

    // Test configuration values
    assert_eq!( audio_config.sample_rate(), 44100 );
    assert_eq!( audio_config.bit_depth(), 16 );
    assert_eq!( audio_config.channels(), 2 );
    assert_eq!( audio_config.buffer_size(), 1024 );

    println!( "✓ Audio processing configuration structure validation successful" );

    // Test default configuration
    let default_config = AudioProcessingConfig::new();
    assert_eq!( default_config.sample_rate(), 44100 );
    assert_eq!( default_config.bit_depth(), 16 );
    assert_eq!( default_config.channels(), 1 );
    assert_eq!( default_config.buffer_size(), 512 );

    println!( "✓ Audio processing default configuration validation successful" );
  }
}
