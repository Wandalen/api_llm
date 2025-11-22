//! HTTP Compression Support
//!
//! Provides gzip compression for request bodies and decompression for response bodies
//! to reduce bandwidth usage and improve performance for large prompts.
//!
//! # Features
//!
//! - Request body compression with gzip
//! - Response body decompression (gzip/deflate)
//! - Automatic content-encoding headers
//! - Configurable compression levels
//!
//! # Benefits
//!
//! - ~60-80% size reduction for text content
//! - Lower latency on slow connections
//! - Cost savings for high-volume deployments
//! - Reduced bandwidth usage

#[ cfg( feature = "compression" ) ]
mod private
{
  use std::io::{ Read, Write };
  use flate2::Compression;
  use flate2::read::GzDecoder;
  use flate2::write::GzEncoder;

  /// Compression configuration
  #[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
  pub struct CompressionConfig
  {
    /// Compression level (0-9, where 0=none, 6=default, 9=best)
    pub level : u32,
    /// Minimum size in bytes before compression is applied
    pub min_size : usize,
  }

  impl CompressionConfig
  {
    /// Create new compression config with default settings
    ///
    /// Default : level=6 (balanced), `min_size=1024` (1KB)
    #[ inline ]
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        level : 6,
        min_size : 1024,
      }
    }

    /// Set compression level (0-9)
    ///
    /// - 0: No compression
    /// - 1: Fastest compression
    /// - 6: Default balanced compression
    /// - 9: Best compression
    #[ inline ]
    #[ must_use ]
    pub fn with_level( mut self, level : u32 ) -> Self
    {
      self.level = level.min( 9 );
      self
    }

    /// Set minimum size threshold for compression
    ///
    /// Bodies smaller than this size won't be compressed.
    /// Default : 1024 bytes (1KB)
    #[ inline ]
    #[ must_use ]
    pub fn with_min_size( mut self, min_size : usize ) -> Self
    {
      self.min_size = min_size;
      self
    }
  }

  impl Default for CompressionConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self::new()
    }
  }

  /// Compress data using gzip
  ///
  /// # Arguments
  ///
  /// * `data` - The data to compress
  /// * `config` - Compression configuration
  ///
  /// # Returns
  ///
  /// Compressed data, or original data if compression would increase size
  ///
  /// # Errors
  ///
  /// Returns an error if compression fails
  #[ inline ]
  pub fn compress( data : &[ u8 ], config : &CompressionConfig ) -> Result< Vec< u8 >, std::io::Error >
  {
    // Skip compression if data is too small
    if data.len() < config.min_size
    {
      return Ok( data.to_vec() );
    }

    let mut encoder = GzEncoder::new( Vec::new(), Compression::new( config.level ) );
    encoder.write_all( data )?;
    let compressed = encoder.finish()?;

    // Only use compressed version if it's actually smaller
    if compressed.len() < data.len()
    {
      Ok( compressed )
    }
    else
    {
      Ok( data.to_vec() )
    }
  }

  /// Decompress gzip-compressed data
  ///
  /// # Arguments
  ///
  /// * `data` - The compressed data
  ///
  /// # Returns
  ///
  /// Decompressed data
  ///
  /// # Errors
  ///
  /// Returns an error if decompression fails or data is corrupted
  #[ inline ]
  pub fn decompress( data : &[ u8 ] ) -> Result< Vec< u8 >, std::io::Error >
  {
    let mut decoder = GzDecoder::new( data );
    let mut decompressed = Vec::new();
    decoder.read_to_end( &mut decompressed )?;
    Ok( decompressed )
  }

  /// Check if data appears to be gzip-compressed
  ///
  /// Checks for gzip magic number (0x1f, 0x8b)
  #[ inline ]
  #[ must_use ]
  pub fn is_gzip( data : &[ u8 ] ) -> bool
  {
    data.len() >= 2 && data[ 0 ] == 0x1f && data[ 1 ] == 0x8b
  }

  /// Add compression headers to request
  ///
  /// Adds:
  /// - `Content-Encoding : gzip` if data is compressed
  /// - `Accept-Encoding : gzip, deflate` to accept compressed responses
  #[ inline ]
  pub fn add_compression_headers
  (
    headers : &mut reqwest::header::HeaderMap,
    is_compressed : bool,
  )
  {
    // Always accept compressed responses
    headers.insert
    (
      reqwest::header::ACCEPT_ENCODING,
      reqwest::header::HeaderValue::from_static( "gzip, deflate" ),
    );

    // Add content-encoding if we compressed the request
    if is_compressed
    {
      headers.insert
      (
        reqwest::header::CONTENT_ENCODING,
        reqwest::header::HeaderValue::from_static( "gzip" ),
      );
    }
  }
}

#[ cfg( feature = "compression" ) ]
crate::mod_interface!
{
  exposed use
  {
    CompressionConfig,
    compress,
    decompress,
    is_gzip,
    add_compression_headers,
  };
}
