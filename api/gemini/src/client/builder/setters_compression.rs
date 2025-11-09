//! Compression feature configuration setters for ClientBuilder.

use super::ClientBuilder;

impl ClientBuilder
{
  /// Enable request/response compression with the given configuration.
  ///
  /// Compression reduces bandwidth usage and can improve performance on slow connections.
  /// Supports Gzip, Deflate, and Brotli algorithms.
  ///
  /// # Arguments
  ///
  /// * `config` - Compression configuration specifying algorithm, level, and thresholds
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use api_gemini::client::Client;
  /// use api_gemini::internal::http::compression::{ CompressionConfig, CompressionAlgorithm };
  ///
  /// # #[ tokio::main ]
  /// # async fn main() -> Result< (), Box< dyn std::error::Error > > {
  /// let compression = CompressionConfig::new()
  ///   .algorithm( CompressionAlgorithm::Gzip )
  ///   .level( 6 )
  ///   .min_size( 1024 );
  ///
  /// let client = Client::builder()
  ///   .api_key( "your-api-key".to_string() )
  ///   .enable_compression( compression )
  ///   .build()?;
  /// # Ok( () )
  /// # }
  /// ```
  #[ must_use ]
  #[ inline ]
  pub fn enable_compression( mut self, config : crate::internal::http::compression::CompressionConfig ) -> Self
  {
    self.compression_config = Some( config );
    self
  }

  /// Disable compression (default).
  #[ must_use ]
  #[ inline ]
  pub fn disable_compression( mut self ) -> Self
  {
    self.compression_config = None;
    self
  }
}
