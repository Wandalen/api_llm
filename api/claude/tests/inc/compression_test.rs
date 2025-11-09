//! HTTP Compression Tests
//!
//! Unit tests for the compression feature that provides gzip compression
//! for request/response bodies.

#[ allow( unused_imports ) ]
use super::*;

#[ cfg( feature = "compression" ) ]
mod compression_tests
{
  use super::*;
  use the_module::{ CompressionConfig, compress, decompress, is_gzip };

  #[ test ]
  fn test_compression_config_defaults()
  {
    let config = CompressionConfig::new();
    assert_eq!( config.level, 6 );
    assert_eq!( config.min_size, 1024 );
  }

  #[ test ]
  fn test_compression_config_builder()
  {
    let config = CompressionConfig::new()
      .with_level( 9 )
      .with_min_size( 2048 );

    assert_eq!( config.level, 9 );
    assert_eq!( config.min_size, 2048 );
  }

  #[ test ]
  fn test_compression_level_clamping()
  {
    // Test that level is clamped to max 9
    let config = CompressionConfig::new().with_level( 15 );
    assert_eq!( config.level, 9 );
  }

  #[ test ]
  fn test_compress_small_data()
  {
    // Data smaller than min_size should not be compressed
    let config = CompressionConfig::new().with_min_size( 1000 );
    let data = b"Hello, world!";
    let result = compress( data, &config ).unwrap();

    // Should return original data (too small)
    assert_eq!( result, data );
  }

  #[ test ]
  fn test_compress_large_data()
  {
    // Repeated text compresses very well
    let config = CompressionConfig::new().with_min_size( 100 );
    let data = "Hello, world! ".repeat( 1000 );
    let result = compress( data.as_bytes(), &config ).unwrap();

    // Should be compressed (much smaller)
    assert!( result.len() < data.len() );
    // Should have gzip magic number
    assert!( is_gzip( &result ) );
  }

  #[ test ]
  fn test_compress_decompress_roundtrip()
  {
    let config = CompressionConfig::new().with_min_size( 100 );
    let original = "This is a test message that will be compressed. ".repeat( 100 );

    // Compress
    let compressed = compress( original.as_bytes(), &config ).unwrap();
    assert!( compressed.len() < original.len() );

    // Decompress
    let decompressed = decompress( &compressed ).unwrap();
    assert_eq!( decompressed, original.as_bytes() );
  }

  #[ test ]
  fn test_is_gzip_detection()
  {
    // Valid gzip data has magic number 0x1f 0x8b
    let gzip_data = vec![ 0x1f, 0x8b, 0x00, 0x00, 0x00 ];
    assert!( is_gzip( &gzip_data ) );

    // Non-gzip data
    let plain_data = b"Hello, world!";
    assert!( !is_gzip( plain_data ) );

    // Too short
    let short_data = vec![ 0x1f ];
    assert!( !is_gzip( &short_data ) );
  }

  #[ test ]
  fn test_compress_incompressible_data()
  {
    // Random data doesnt compress well
    let config = CompressionConfig::new().with_min_size( 100 );
    let data : Vec< u8 > = ( 0_u8..200 ).collect();
    let result = compress( &data, &config ).unwrap();

    // Should return original if compressed version is larger
    // This test might vary based on exact data, but generally
    // the function should prefer original if compression doesnt help
    assert!( result.len() <= data.len() + 50 ); // Allow small overhead
  }

  #[ test ]
  fn test_different_compression_levels()
  {
    let data = "Compress me! ".repeat( 500 );

    // Level 1 (fastest)
    let config1 = CompressionConfig::new().with_level( 1 ).with_min_size( 100 );
    let result1 = compress( data.as_bytes(), &config1 ).unwrap();

    // Level 9 (best)
    let config9 = CompressionConfig::new().with_level( 9 ).with_min_size( 100 );
    let result9 = compress( data.as_bytes(), &config9 ).unwrap();

    // Both should compress
    assert!( result1.len() < data.len() );
    assert!( result9.len() < data.len() );

    // Level 9 should generally compress better (though not guaranteed for all data)
    // At minimum both should decompress correctly
    let decompressed1 = decompress( &result1 ).unwrap();
    let decompressed9 = decompress( &result9 ).unwrap();

    assert_eq!( decompressed1, data.as_bytes() );
    assert_eq!( decompressed9, data.as_bytes() );
  }

  #[ test ]
  fn test_add_compression_headers()
  {
    use the_module::add_compression_headers;
    use reqwest::header::{ HeaderMap, ACCEPT_ENCODING, CONTENT_ENCODING };

    let mut headers = HeaderMap::new();

    // Add headers without compression
    add_compression_headers( &mut headers, false );

    assert!( headers.contains_key( ACCEPT_ENCODING ) );
    assert!( !headers.contains_key( CONTENT_ENCODING ) );

    let accept = headers.get( ACCEPT_ENCODING ).unwrap().to_str().unwrap();
    assert!( accept.contains( "gzip" ) );

    // Add headers with compression
    let mut headers2 = HeaderMap::new();
    add_compression_headers( &mut headers2, true );

    assert!( headers2.contains_key( ACCEPT_ENCODING ) );
    assert!( headers2.contains_key( CONTENT_ENCODING ) );

    let encoding = headers2.get( CONTENT_ENCODING ).unwrap().to_str().unwrap();
    assert_eq!( encoding, "gzip" );
  }

  #[ test ]
  fn test_compression_config_default()
  {
    let config = CompressionConfig::default();
    assert_eq!( config.level, 6 );
    assert_eq!( config.min_size, 1024 );
  }
}

#[ cfg( not( feature = "compression" ) ) ]
mod compression_feature_disabled
{
  #[ test ]
  fn test_compression_feature_disabled()
  {
    // When compression feature is disabled, this test verifies
    // that compilation succeeds without the feature
  }
}
