//! HTTP Compression for Request/Response Optimization
//!
//! Provides compression support to reduce bandwidth usage.

#[ cfg( feature = "enabled" ) ]
mod private
{
  use error_tools::untyped::Result;

  /// Compression configuration
  #[ derive( Debug, Clone ) ]
  pub struct CompressionConfig
  {
    /// Whether compression is enabled
    pub enabled : bool,
    /// Compression level (1-9 for gzip/deflate)
    pub level : u32,
    /// Minimum size in bytes to compress
    pub min_size : usize,
    /// Compression algorithm
    pub algorithm : CompressionAlgorithm,
  }

  /// Compression algorithms
  #[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
  pub enum CompressionAlgorithm
  {
    /// Gzip compression
    Gzip,
    /// Deflate compression
    Deflate,
  }

  impl CompressionAlgorithm
  {
    /// Get the content-encoding header value
    #[ must_use ]
    pub fn content_encoding( self ) -> &'static str
    {
      match self
      {
        Self::Gzip => "gzip",
        Self::Deflate => "deflate",
      }
    }
  }

  impl Default for CompressionConfig
  {
    fn default() -> Self
    {
      Self
      {
        enabled : true,
        level : 6,
        min_size : 1024,
        algorithm : CompressionAlgorithm::Gzip,
      }
    }
  }

  impl CompressionConfig
  {
    /// Create new compression configuration
    #[ must_use ]
    pub fn new() -> Self
    {
      Self::default()
    }

    /// Set compression level (1-9)
    #[ must_use ]
    pub fn with_level( mut self, level : u32 ) -> Self
    {
      self.level = level.clamp( 1, 9 );
      self
    }

    /// Set minimum size for compression
    #[ must_use ]
    pub fn with_min_size( mut self, size : usize ) -> Self
    {
      self.min_size = size;
      self
    }

    /// Set compression algorithm
    #[ must_use ]
    pub fn with_algorithm( mut self, algorithm : CompressionAlgorithm ) -> Self
    {
      self.algorithm = algorithm;
      self
    }

    /// Enable compression
    #[ must_use ]
    pub fn enable( mut self ) -> Self
    {
      self.enabled = true;
      self
    }

    /// Disable compression
    #[ must_use ]
    pub fn disable( mut self ) -> Self
    {
      self.enabled = false;
      self
    }

    /// Compress data if it meets size threshold
    ///
    /// # Errors
    ///
    /// Returns error if compression fails
    pub fn compress( &self, data : &[ u8 ] ) -> Result< Vec< u8 > >
    {
      if !self.enabled || data.len() < self.min_size
      {
        return Ok( data.to_vec() );
      }

      match self.algorithm
      {
        CompressionAlgorithm::Gzip => {
          use std::io::Write;
          let mut encoder = flate2::write::GzEncoder::new(
            Vec::new(),
            flate2::Compression::new( self.level )
          );
          encoder.write_all( data )?;
          Ok( encoder.finish()? )
        },
        CompressionAlgorithm::Deflate => {
          use std::io::Write;
          let mut encoder = flate2::write::DeflateEncoder::new(
            Vec::new(),
            flate2::Compression::new( self.level )
          );
          encoder.write_all( data )?;
          Ok( encoder.finish()? )
        },
      }
    }

    /// Decompress data
    ///
    /// # Errors
    ///
    /// Returns error if decompression fails
    pub fn decompress( &self, data : &[ u8 ] ) -> Result< Vec< u8 > >
    {
      if !self.enabled
      {
        return Ok( data.to_vec() );
      }

      match self.algorithm
      {
        CompressionAlgorithm::Gzip => {
          use std::io::Read;
          let mut decoder = flate2::read::GzDecoder::new( data );
          let mut decompressed = Vec::new();
          decoder.read_to_end( &mut decompressed )?;
          Ok( decompressed )
        },
        CompressionAlgorithm::Deflate => {
          use std::io::Read;
          let mut decoder = flate2::read::DeflateDecoder::new( data );
          let mut decompressed = Vec::new();
          decoder.read_to_end( &mut decompressed )?;
          Ok( decompressed )
        },
      }
    }
  }

  #[ cfg( test ) ]
  mod tests
  {
    use super::*;

    #[ test ]
    fn test_compression_config_default()
    {
      let config = CompressionConfig::default();
      assert!( config.enabled );
      assert_eq!( config.level, 6 );
      assert_eq!( config.min_size, 1024 );
    }

    #[ test ]
    fn test_compression_config_builder()
    {
      let config = CompressionConfig::new()
        .with_level( 9 )
        .with_min_size( 512 )
        .with_algorithm( CompressionAlgorithm::Deflate );

      assert_eq!( config.level, 9 );
      assert_eq!( config.min_size, 512 );
      assert_eq!( config.algorithm, CompressionAlgorithm::Deflate );
    }

    #[ test ]
    fn test_gzip_compression()
    {
      let config = CompressionConfig::new()
        .with_algorithm( CompressionAlgorithm::Gzip )
        .with_min_size( 10 );

      let data = "Hello, World! This is a test of compression. ".repeat( 20 );
      let compressed = config.compress( data.as_bytes() ).unwrap();
      let decompressed = config.decompress( &compressed ).unwrap();

      assert_eq!( data.as_bytes(), decompressed.as_slice() );
      assert!( compressed.len() < data.len() );
    }

    #[ test ]
    fn test_deflate_compression()
    {
      let config = CompressionConfig::new()
        .with_algorithm( CompressionAlgorithm::Deflate )
        .with_min_size( 10 );

      let data = "Hello, World! This is a test of compression. ".repeat( 20 );
      let compressed = config.compress( data.as_bytes() ).unwrap();
      let decompressed = config.decompress( &compressed ).unwrap();

      assert_eq!( data.as_bytes(), decompressed.as_slice() );
      assert!( compressed.len() < data.len() );
    }

    #[ test ]
    fn test_min_size_threshold()
    {
      let config = CompressionConfig::new()
        .with_min_size( 100 );

      let small_data = b"Small";
      let result = config.compress( small_data ).unwrap();

      // Should not compress below threshold
      assert_eq!( result, small_data );
    }

    #[ test ]
    fn test_compression_disabled()
    {
      let config = CompressionConfig::new()
        .disable();

      let data = b"Hello, World!";
      let result = config.compress( data ).unwrap();

      assert_eq!( result, data );
    }

    #[ test ]
    fn test_algorithm_content_encoding()
    {
      assert_eq!( CompressionAlgorithm::Gzip.content_encoding(), "gzip" );
      assert_eq!( CompressionAlgorithm::Deflate.content_encoding(), "deflate" );
    }
  }
}

#[ cfg( feature = "enabled" ) ]
crate::mod_interface!
{
  exposed use
  {
    CompressionConfig,
    CompressionAlgorithm,
  };
}
