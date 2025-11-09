//! Image Generation and Manipulation
//!
//! Types and configurations for AI image generation.

use serde::{ Serialize, Deserialize };

/// Configuration for image generation operations.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ImageGenerationConfig
{
  /// Model to use for generation
  pub model : ImageModel,
  /// Image size
  pub size : ImageSize,
  /// Image quality
  pub quality : ImageQuality,
  /// Image style
  pub style : ImageStyle,
  /// Response format
  pub response_format : ImageResponseFormat,
}

impl Default for ImageGenerationConfig
{
  #[ inline ]
  fn default() -> Self
  {
    Self
    {
      model : ImageModel::DallE3,
      size : ImageSize::Square1024,
      quality : ImageQuality::Standard,
      style : ImageStyle::Vivid,
      response_format : ImageResponseFormat::Url,
    }
  }
}

/// Available image generation models.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub enum ImageModel
{
  /// DALL-E 2 model
  DallE2,
  /// DALL-E 3 model
  DallE3,
  /// Custom model
  Custom( String ),
}

/// Image size options.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub enum ImageSize
{
  /// 256x256 pixels
  Square256,
  /// 512x512 pixels
  Square512,
  /// 1024x1024 pixels
  Square1024,
  /// 1792x1024 pixels
  Wide1792x1024,
  /// 1024x1792 pixels
  Tall1024x1792,
}

/// Image quality options.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub enum ImageQuality
{
  /// Standard quality
  Standard,
  /// High definition quality
  HD,
}

/// Image style options.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub enum ImageStyle
{
  /// Vivid style
  Vivid,
  /// Natural style
  Natural,
}

/// Image response format.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub enum ImageResponseFormat
{
  /// Return URL to image
  Url,
  /// Return base64-encoded image data
  Base64,
}

/// Result of image generation/manipulation.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ImageResult
{
  /// Image URL or base64 data
  pub url : Option< String >,
  /// Base64-encoded image data
  pub b64_json : Option< String >,
  /// Image metadata
  pub metadata : ImageMetadata,
}

/// Metadata about generated image.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ImageMetadata
{
  /// Image width in pixels
  pub width : u32,
  /// Image height in pixels
  pub height : u32,
  /// Image format (PNG, JPEG, etc.)
  pub format : String,
  /// Generation time in milliseconds
  pub generation_time_ms : u64,
}
