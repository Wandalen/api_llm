//! Vision and Multimodal API Implementation
//!
//! Provides access to `HuggingFace`'s computer vision models for image analysis.
//!
//! ## Features
//!
//! - **Image Classification**: Classify images into categories
//! - **Object Detection**: Detect and locate objects in images
//! - **Image-to-Text**: Generate captions and descriptions for images
//!
//! ## Usage
//!
//! ```no_run
//! # use api_huggingface::{Client, environment::HuggingFaceEnvironmentImpl, secret::Secret};
//! # use api_huggingface::vision::ImageInput;
//! # use std::fs;
//! # async fn example() -> Result< (), Box< dyn std::error::Error > > {
//! # let api_key = Secret::new("test".to_string());
//! # let env = HuggingFaceEnvironmentImpl::build(api_key, None)?;
//! # let client = Client::build(env)?;
//! # let vision = client.vision();
//! // Load image
//! let image_data = fs::read( "cat.jpg" )?;
//! let input = ImageInput::from_bytes( image_data );
//!
//! // Classify image
//! let result = vision.classify_image( input, "google/vit-base-patch16-224" ).await?;
//! println!( "Classification : {:?}", result );
//! # Ok(())
//! # }
//! ```

pub mod types;
pub mod classification;
pub mod detection;
pub mod captioning;

pub use types::*;

use crate::Client;

/// Vision API interface
///
/// Provides methods for computer vision tasks using `HuggingFace` models.
#[ derive( Debug, Clone ) ]
pub struct Vision< E >
where
  E : Clone,
{
  pub( crate ) client : Client< E >,
}

impl< E > Vision< E >
where
  E : Clone,
{
  /// Create a new Vision API group
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
