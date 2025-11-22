//! Audio Processing API Implementation
//!
//! Provides access to `HuggingFace`'s audio processing models.
//!
//! ## Features
//!
//! - **Automatic Speech Recognition (ASR)**: Convert speech to text
//! - **Text-to-Speech (TTS)**: Generate speech from text
//! - **Audio Classification**: Classify audio into categories
//! - **Audio-to-Audio**: Transform audio (noise reduction, enhancement, etc.)
//!
//! ## Usage
//!
//! ```no_run
//! # use api_huggingface::{Client, environment::HuggingFaceEnvironmentImpl, secret::Secret};
//! # use api_huggingface::audio::AudioInput;
//! # use std::fs;
//! # async fn example() -> Result< (), Box< dyn std::error::Error > > {
//! # let api_key = Secret::new("test".to_string());
//! # let env = HuggingFaceEnvironmentImpl::build(api_key, None)?;
//! # let client = Client::build(env)?;
//! # let audio = client.audio();
//! // Load audio file
//! let audio_data = fs::read( "speech.wav" )?;
//! let input = AudioInput::from_bytes( audio_data );
//!
//! // Transcribe speech
//! let result = audio.transcribe( input, "openai/whisper-base" ).await?;
//! println!( "Transcription : {}", result );
//! # Ok(())
//! # }
//! ```

pub mod types;
pub mod asr;
pub mod tts;
pub mod classification;
pub mod audio_to_audio;

pub use types::*;

use crate::Client;

/// Audio API interface
///
/// Provides methods for audio processing tasks using `HuggingFace` models.
#[ derive( Debug, Clone ) ]
pub struct Audio< E >
where
  E : Clone,
{
  pub( crate ) client : Client< E >,
}

impl< E > Audio< E >
where
  E : Clone,
{
  /// Create a new Audio API group
  #[ inline ]
  #[ must_use ]
  pub fn new( client : &Client< E > ) -> Self
  {
  Self
  {
      client : (*client).clone(),
  }
  }
}
