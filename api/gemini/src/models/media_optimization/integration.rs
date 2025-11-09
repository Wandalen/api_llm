//! Multimodal integration API for high-level media operations

use super::*;
use std::sync::Arc;
use std::path::Path;
use bytes::Bytes;
use futures_util::Stream;

/// Optimized Media API providing high-performance file operations
#[ derive( Debug ) ]
pub struct OptimizedMediaApi< 'a >
{
  /// Reference to the Gemini client
  client : &'a crate::client::Client,
  /// Processing configuration
  config : MediaProcessingConfig,
  /// Processing pipeline instance
  pipeline : Arc< MediaProcessingPipeline >,
}

impl< 'a > OptimizedMediaApi< 'a >
{
  /// Create a new optimized media API instance with default configuration
  pub fn new( client : &'a crate::client::Client ) -> Self
  {
    let config = MediaProcessingConfig::default();
    let pipeline = Arc::new( MediaProcessingPipeline::new( config.clone() ) );

    Self {
      client,
      config,
      pipeline,
    }
  }

  /// Create a new optimized media API instance with custom configuration
  pub fn with_config( client : &'a crate::client::Client, config : MediaProcessingConfig ) -> Self
  {
    let pipeline = Arc::new( MediaProcessingPipeline::new( config.clone() ) );

    Self {
      client,
      config,
      pipeline,
    }
  }

  /// Upload a file with advanced optimization
  pub async fn upload_optimized< P: AsRef< Path > >( &self, file_path : P ) -> Result< ProcessedMediaResult, crate::error::Error >
  {
    self.pipeline.process_upload( file_path.as_ref() ).await
  }

  /// Download and process a file with optimization
  pub async fn download_optimized( &self, file_id : &str, destination : &Path ) -> Result< ProcessedMediaResult, crate::error::Error >
  {
    self.pipeline.process_download( file_id, destination ).await
  }

  /// Get processing metrics and statistics
  pub fn get_metrics( &self ) -> MediaProcessingMetricsReport
  {
    self.pipeline.get_metrics()
  }

  /// Get cache statistics
  pub fn get_cache_stats( &self ) -> MediaCacheStatsReport
  {
    self.pipeline.get_cache_stats()
  }

  /// Clear processing cache
  pub async fn clear_cache( &self )
  {
    self.pipeline.clear_cache().await
  }

  /// Process data in streaming chunks for large files
  pub async fn process_stream< S >( &self, stream : S, metadata : ProcessedMediaMetadata ) -> Result< ProcessedMediaResult, crate::error::Error >
  where
    S: Stream< Item = Result< Bytes, crate::error::Error > > + Send + Unpin,
  {
    self.pipeline.process_stream( stream, metadata ).await
  }

  /// Generate thumbnail for uploaded media
  pub async fn generate_thumbnail( &self, file_data : &Bytes, mime_type : &str ) -> Result< Bytes, crate::error::Error >
  {
    if let Some( ref thumbnail_config ) = self.config.thumbnail_config
    {
      let generator = ThumbnailGenerator::new( thumbnail_config.clone() );
      generator.generate_thumbnail( file_data, mime_type ).await
    } else {
      Err( crate::error::Error::ApiError( "Thumbnail generation not configured".to_string() ) )
    }
  }

  /// Get the underlying client reference for direct API access
  pub fn client( &self ) -> &'a crate::client::Client
  {
    self.client
  }

  /// Get current configuration
  pub fn config( &self ) -> &MediaProcessingConfig
  {
    &self.config
  }
}
