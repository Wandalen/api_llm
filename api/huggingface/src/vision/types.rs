//! Vision API Types
//!
//! Data structures for vision API requests and responses.

use serde::{ Deserialize, Serialize };

/// Image input for vision tasks
///
/// Supports multiple input formats for flexibility.
#[ derive( Debug, Clone ) ]
pub enum ImageInput
{
  /// Raw image bytes
  Bytes( Vec< u8 > ),

  /// Base64-encoded image
  Base64( String ),

  /// URL to image
  Url( String ),
}

impl ImageInput
{
  /// Create image input from raw bytes
  #[ inline ]
  #[ must_use ]
  pub fn from_bytes( bytes : Vec< u8 > ) -> Self
  {
  Self::Bytes( bytes )
  }

  /// Create image input from base64 string
  #[ inline ]
  #[ must_use ]
  pub fn from_base64( data : impl Into< String > ) -> Self
  {
  Self::Base64( data.into() )
  }

  /// Create image input from URL
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

/// Image classification result
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ClassificationResult
{
  /// Predicted label
  pub label : String,

  /// Confidence score (0.0 - 1.0)
  pub score : f64,
}

/// Object detection bounding box
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct BoundingBox
{
  /// X coordinate of top-left corner
  pub xmin : f64,

  /// Y coordinate of top-left corner
  pub ymin : f64,

  /// X coordinate of bottom-right corner
  pub xmax : f64,

  /// Y coordinate of bottom-right corner
  pub ymax : f64,
}

/// Object detection result
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct DetectionResult
{
  /// Detected object label
  pub label : String,

  /// Confidence score (0.0 - 1.0)
  pub score : f64,

  /// Bounding box coordinates
  #[ serde( rename = "box" ) ]
  pub box_coords : BoundingBox,
}

/// Image captioning result
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct CaptionResult
{
  /// Generated caption text
  pub generated_text : String,
}

#[ cfg( test ) ]
#[ allow( clippy::float_cmp ) ]
mod tests
{
  use super::*;

  #[ test ]
  fn test_image_input_from_bytes()
  {
  let bytes = vec![ 1, 2, 3, 4 ];
  let input = ImageInput::from_bytes( bytes.clone() );

  match input
  {
      ImageInput::Bytes( b ) => assert_eq!( b, bytes ),
      _ => panic!( "Wrong variant" ),
  }
  }

  #[ test ]
  fn test_image_input_from_base64()
  {
  let data = "SGVsbG8=";
  let input = ImageInput::from_base64( data );

  match input
  {
      ImageInput::Base64( d ) => assert_eq!( d, data ),
      _ => panic!( "Wrong variant" ),
  }
  }

  #[ test ]
  fn test_image_input_from_url()
  {
  let url = "https://example.com/image.jpg";
  let input = ImageInput::from_url( url );

  match input
  {
      ImageInput::Url( u ) => assert_eq!( u, url ),
      _ => panic!( "Wrong variant" ),
  }
  }

  #[ test ]
  fn test_base64_encoding()
  {
  let bytes = vec![ 72, 101, 108, 108, 111 ]; // "Hello"
  let input = ImageInput::from_bytes( bytes );

  let encoded = input.to_base64();
  assert_eq!( encoded, "SGVsbG8=" );
  }

  #[ test ]
  fn test_classification_result_creation()
  {
  let result = ClassificationResult
  {
      label : "cat".to_string(),
      score : 0.95,
  };

  assert_eq!( result.label, "cat" );
  assert!( ( result.score - 0.95 ).abs() < 0.01 );
  }

  #[ test ]
  fn test_bounding_box_creation()
  {
  let bbox = BoundingBox
  {
      xmin : 10.0,
      ymin : 20.0,
      xmax : 100.0,
      ymax : 200.0,
  };

  assert_eq!( bbox.xmin, 10.0 );
  assert_eq!( bbox.ymax, 200.0 );
  }

  #[ test ]
  fn test_detection_result_creation()
  {
  let result = DetectionResult
  {
      label : "dog".to_string(),
      score : 0.88,
      box_coords : BoundingBox
      {
  xmin : 0.0,
  ymin : 0.0,
  xmax : 50.0,
  ymax : 50.0,
      },
  };

  assert_eq!( result.label, "dog" );
  assert_eq!( result.box_coords.xmax, 50.0 );
  }

  #[ test ]
  fn test_caption_result_creation()
  {
  let result = CaptionResult
  {
      generated_text : "A cat sitting on a mat".to_string(),
  };

  assert!( result.generated_text.contains( "cat" ) );
  }
}
