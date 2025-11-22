//! Comprehensive tests for `HuggingFace` Vision and Multimodal API functionality

#![ allow( clippy::float_cmp ) ]
#![ allow( clippy::doc_markdown ) ]

use api_huggingface::
{
  Client,
  environment::HuggingFaceEnvironmentImpl,
  secret::Secret,
  vision::
  {
  ImageInput, ClassificationResult, DetectionResult, BoundingBox, CaptionResult,
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

/// Test vision API group creation
#[ tokio::test ]
async fn test_vision_api_creation()
{
  // Setup
  let client = create_test_client().expect( "Client creation should succeed" );

  // Execution
  let vision = client.vision();

  // Verification
  assert!( core::mem::size_of_val( &vision ) > 0, "Vision API group should be created" );
}

// =============================================================================
// ImageInput Tests
// =============================================================================

/// Test `ImageInput` construction from bytes
#[ test ]
fn test_image_input_from_bytes()
{
  // Setup
  let image_data = vec![ 0x89, 0x50, 0x4E, 0x47 ]; // PNG header

  // Execution
  let input = ImageInput::from_bytes( image_data.clone() );

  // Verification
  match input
  {
  ImageInput::Bytes( data ) => assert_eq!( data, image_data ),
  _ => panic!( "Expected Bytes variant" ),
  }
}

/// Test ImageInput construction from base64
#[ test ]
fn test_image_input_from_base64()
{
  // Setup
  let base64_data = "aVZCT1J3MEtHZ29BQUFBTlNVaEVVZ0FBQUFB";

  // Execution
  let input = ImageInput::from_base64( base64_data );

  // Verification
  match input
  {
  ImageInput::Base64( data ) => assert_eq!( data, base64_data ),
  _ => panic!( "Expected Base64 variant" ),
  }
}

/// Test ImageInput construction from URL
#[ test ]
fn test_image_input_from_url()
{
  // Setup
  let url = "https://example.com/image.jpg";

  // Execution
  let input = ImageInput::from_url( url );

  // Verification
  match input
  {
  ImageInput::Url( data ) => assert_eq!( data, url ),
  _ => panic!( "Expected Url variant" ),
  }
}

/// Test ImageInput base64 conversion from bytes
#[ test ]
fn test_image_input_to_base64_from_bytes()
{
  // Setup
  let image_data = vec![ 0x48, 0x65, 0x6C, 0x6C, 0x6F ]; // "Hello" in bytes
  let input = ImageInput::from_bytes( image_data );

  // Execution
  let base64 = input.to_base64();

  // Verification
  assert!( !base64.is_empty(), "Base64 encoding should not be empty" );
  assert_eq!( base64, "SGVsbG8=" ); // Standard base64 encoding of "Hello"
}

/// Test ImageInput base64 conversion from existing base64
#[ test ]
fn test_image_input_to_base64_from_base64()
{
  // Setup
  let original_base64 = "SGVsbG8=";
  let input = ImageInput::from_base64( original_base64 );

  // Execution
  let base64 = input.to_base64();

  // Verification
  assert_eq!( base64, original_base64, "Base64 should be returned unchanged" );
}

/// Test ImageInput base64 conversion from URL
#[ test ]
fn test_image_input_to_base64_from_url()
{
  // Setup
  let url = "https://example.com/image.jpg";
  let input = ImageInput::from_url( url );

  // Execution
  let base64 = input.to_base64();

  // Verification
  assert_eq!( base64, url, "URL should be returned unchanged" );
}

// =============================================================================
// Data Structure Tests
// =============================================================================

/// Test ClassificationResult creation
#[ test ]
fn test_classification_result_creation()
{
  // Setup
  let label = "cat".to_string();
  let score = 0.95;

  // Execution
  let result = ClassificationResult { label : label.clone(), score };

  // Verification
  assert_eq!( result.label, label );
  assert_eq!( result.score, score );
}

/// Test ClassificationResult deserialization
#[ test ]
fn test_classification_result_deserialization()
{
  // Setup
  let json = r#"{"label": "dog", "score": 0.87}"#;

  // Execution
  let result : ClassificationResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( result.label, "dog" );
  assert_eq!( result.score, 0.87 );
}

/// Test BoundingBox creation
#[ test ]
fn test_bounding_box_creation()
{
  // Setup
  let bbox = BoundingBox
  {
  xmin : 10.0,
  ymin : 20.0,
  xmax : 100.0,
  ymax : 200.0,
  };

  // Verification
  assert_eq!( bbox.xmin, 10.0 );
  assert_eq!( bbox.ymin, 20.0 );
  assert_eq!( bbox.xmax, 100.0 );
  assert_eq!( bbox.ymax, 200.0 );
}

/// Test BoundingBox deserialization
#[ test ]
fn test_bounding_box_deserialization()
{
  // Setup
  let json = r#"{"xmin": 15.5, "ymin": 25.5, "xmax": 115.5, "ymax": 215.5}"#;

  // Execution
  let bbox : BoundingBox = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( bbox.xmin, 15.5 );
  assert_eq!( bbox.ymin, 25.5 );
  assert_eq!( bbox.xmax, 115.5 );
  assert_eq!( bbox.ymax, 215.5 );
}

/// Test DetectionResult creation
#[ test ]
fn test_detection_result_creation()
{
  // Setup
  let label = "person".to_string();
  let score = 0.92;
  let box_coords = BoundingBox
  {
  xmin : 50.0,
  ymin : 60.0,
  xmax : 150.0,
  ymax : 250.0,
  };

  // Execution
  let result = DetectionResult
  {
  label : label.clone(),
  score,
  box_coords : box_coords.clone(),
  };

  // Verification
  assert_eq!( result.label, label );
  assert_eq!( result.score, score );
  assert_eq!( result.box_coords.xmin, box_coords.xmin );
}

/// Test DetectionResult deserialization
#[ test ]
fn test_detection_result_deserialization()
{
  // Setup
  let json = r#"{
  "label": "car",
  "score": 0.88,
  "box": {
      "xmin": 100.0,
      "ymin": 150.0,
      "xmax": 300.0,
      "ymax": 400.0
  }
  }"#;

  // Execution
  let result : DetectionResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( result.label, "car" );
  assert_eq!( result.score, 0.88 );
  assert_eq!( result.box_coords.xmin, 100.0 );
  assert_eq!( result.box_coords.ymin, 150.0 );
  assert_eq!( result.box_coords.xmax, 300.0 );
  assert_eq!( result.box_coords.ymax, 400.0 );
}

/// Test CaptionResult creation
#[ test ]
fn test_caption_result_creation()
{
  // Setup
  let text = "A beautiful sunset over the ocean".to_string();

  // Execution
  let result = CaptionResult { generated_text : text.clone() };

  // Verification
  assert_eq!( result.generated_text, text );
}

/// Test CaptionResult deserialization
#[ test ]
fn test_caption_result_deserialization()
{
  // Setup
  let json = r#"{"generated_text": "A cat sitting on a mat"}"#;

  // Execution
  let result : CaptionResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( result.generated_text, "A cat sitting on a mat" );
}

// =============================================================================
// Base64 Encoding Tests
// =============================================================================

/// Test base64 encoding of common image types
#[ test ]
fn test_base64_encoding_png_header()
{
  // Setup - PNG file signature
  let png_header = vec![ 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A ];
  let input = ImageInput::from_bytes( png_header );

  // Execution
  let base64 = input.to_base64();

  // Verification
  assert!( base64.starts_with( "iVBOR" ), "PNG header should encode to base64 starting with 'iVBOR'" );
}

/// Test base64 encoding of JPEG header
#[ test ]
fn test_base64_encoding_jpeg_header()
{
  // Setup - JPEG file signature
  let jpeg_header = vec![ 0xFF, 0xD8, 0xFF, 0xE0 ];
  let input = ImageInput::from_bytes( jpeg_header );

  // Execution
  let base64 = input.to_base64();

  // Verification
  assert!( base64.starts_with( "/9j/" ), "JPEG header should encode to base64 starting with '/9j/'" );
}

/// Test base64 encoding round-trip
#[ test ]
fn test_base64_roundtrip()
{
  // Setup
  let original_data = b"Test image data for encoding";
  let input = ImageInput::from_bytes( original_data.to_vec() );

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

/// Test classification response with single result
#[ test ]
fn test_classification_single_response_format()
{
  // Setup
  let json = r#"[
  {"label": "tabby cat", "score": 0.95},
  {"label": "persian cat", "score": 0.03},
  {"label": "siamese cat", "score": 0.02}
  ]"#;

  // Execution
  let results : Vec< ClassificationResult > = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( results.len(), 3 );
  assert_eq!( results[ 0 ].label, "tabby cat" );
  assert_eq!( results[ 0 ].score, 0.95 );
  assert_eq!( results[ 1 ].label, "persian cat" );
  assert_eq!( results[ 2 ].label, "siamese cat" );
}

/// Test classification response with batch results
#[ test ]
fn test_classification_batch_response_format()
{
  // Setup
  let json = r#"[
  [
      {"label": "cat", "score": 0.9},
      {"label": "dog", "score": 0.1}
  ],
  [
      {"label": "bird", "score": 0.8},
      {"label": "fish", "score": 0.2}
  ]
  ]"#;

  // Execution
  let batches : Vec< Vec< ClassificationResult > > = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( batches.len(), 2 );
  assert_eq!( batches[ 0 ].len(), 2 );
  assert_eq!( batches[ 1 ].len(), 2 );
  assert_eq!( batches[ 0 ][ 0 ].label, "cat" );
  assert_eq!( batches[ 1 ][ 0 ].label, "bird" );
}

/// Test detection response format
#[ test ]
fn test_detection_response_format()
{
  // Setup
  let json = r#"[
  {
      "label": "person",
      "score": 0.95,
      "box": {"xmin": 50, "ymin": 100, "xmax": 200, "ymax": 400}
  },
  {
      "label": "car",
      "score": 0.87,
      "box": {"xmin": 300, "ymin": 200, "xmax": 500, "ymax": 350}
  }
  ]"#;

  // Execution
  let results : Vec< DetectionResult > = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( results.len(), 2 );
  assert_eq!( results[ 0 ].label, "person" );
  assert_eq!( results[ 0 ].score, 0.95 );
  assert_eq!( results[ 0 ].box_coords.xmin, 50.0 );
  assert_eq!( results[ 1 ].label, "car" );
  assert_eq!( results[ 1 ].score, 0.87 );
}

/// Test caption response single format
#[ test ]
fn test_caption_single_response_format()
{
  // Setup
  let json = r#"{"generated_text": "A beautiful landscape"}"#;

  // Execution
  let result : CaptionResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( result.generated_text, "A beautiful landscape" );
}

/// Test caption response multiple format
#[ test ]
fn test_caption_multiple_response_format()
{
  // Setup
  let json = r#"[
  {"generated_text": "First caption"},
  {"generated_text": "Second caption"}
  ]"#;

  // Execution
  let results : Vec< CaptionResult > = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( results.len(), 2 );
  assert_eq!( results[ 0 ].generated_text, "First caption" );
  assert_eq!( results[ 1 ].generated_text, "Second caption" );
}

// =============================================================================
// Edge Cases and Validation Tests
// =============================================================================

/// Test empty classification results
#[ test ]
fn test_empty_classification_results()
{
  // Setup
  let json = "[]";

  // Execution
  let results : Vec< ClassificationResult > = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( results.len(), 0, "Empty results should deserialize" );
}

/// Test classification with zero score
#[ test ]
fn test_classification_zero_score()
{
  // Setup
  let json = r#"{"label": "unknown", "score": 0.0}"#;

  // Execution
  let result : ClassificationResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( result.label, "unknown" );
  assert_eq!( result.score, 0.0 );
}

/// Test classification with perfect score
#[ test ]
fn test_classification_perfect_score()
{
  // Setup
  let json = r#"{"label": "certain", "score": 1.0}"#;

  // Execution
  let result : ClassificationResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( result.label, "certain" );
  assert_eq!( result.score, 1.0 );
}

/// Test detection with negative coordinates
#[ test ]
fn test_detection_negative_coordinates()
{
  // Setup
  let json = r#"{
  "label": "object",
  "score": 0.5,
  "box": {"xmin": -10.0, "ymin": -20.0, "xmax": 50.0, "ymax": 60.0}
  }"#;

  // Execution
  let result : DetectionResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( result.box_coords.xmin, -10.0 );
  assert_eq!( result.box_coords.ymin, -20.0 );
}

/// Test caption with empty text
#[ test ]
fn test_caption_empty_text()
{
  // Setup
  let json = r#"{"generated_text": ""}"#;

  // Execution
  let result : CaptionResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( result.generated_text, "" );
}

/// Test caption with very long text
#[ test ]
fn test_caption_long_text()
{
  // Setup
  let long_text = "A ".repeat( 500 );
  let json = format!( r#"{{"generated_text": "{long_text}"}}"# );

  // Execution
  let result : CaptionResult = serde_json::from_str( &json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert!( result.generated_text.len() > 500 );
  assert!( result.generated_text.starts_with( "A " ) );
}

/// Test detection with fractional coordinates
#[ test ]
fn test_detection_fractional_coordinates()
{
  // Setup
  let json = r#"{
  "label": "precise",
  "score": 0.999,
  "box": {"xmin": 10.5, "ymin": 20.7, "xmax": 30.3, "ymax": 40.1}
  }"#;

  // Execution
  let result : DetectionResult = serde_json::from_str( json )
  .expect( "Deserialization should succeed" );

  // Verification
  assert_eq!( result.box_coords.xmin, 10.5 );
  assert_eq!( result.box_coords.ymin, 20.7 );
  assert_eq!( result.box_coords.xmax, 30.3 );
  assert_eq!( result.box_coords.ymax, 40.1 );
}
