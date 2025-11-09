//! Advanced Media API Optimization for high-performance file processing and management.
//!
//! This module provides comprehensive media processing optimization including:
//! - Intelligent caching and storage strategies
//! - Memory-efficient large file handling with streaming
//! - Advanced compression and optimization algorithms
//! - Real-time performance monitoring and metrics
//! - Configurable processing pipelines
//! - Concurrent upload/download optimization

use core::time::Duration;
use bytes::Bytes;

mod upload;
mod metadata;
mod integration;

/// Advanced media processing configuration
#[ derive( Debug, Clone ) ]
pub struct MediaProcessingConfig
{
  /// Maximum file size for in-memory processing (bytes)
  pub max_memory_file_size : usize,
  /// Chunk size for streaming large files (bytes)
  pub streaming_chunk_size : usize,
  /// Maximum concurrent upload/download operations
  pub max_concurrent_operations : usize,
  /// Enable compression for uploads
  pub enable_compression : bool,
  /// Compression quality (0-100)
  pub compression_quality : u8,
  /// Cache TTL for processed media
  pub cache_ttl_seconds : u64,
  /// Maximum cache size (bytes)
  pub max_cache_size_bytes : usize,
  /// Enable metrics collection
  pub enable_metrics : bool,
  /// Retry configuration for failed operations
  pub retry_config : MediaRetryConfig,
  /// Thumbnail generation settings
  pub thumbnail_config : Option< ThumbnailConfig >,
}

impl Default for MediaProcessingConfig
{
  #[ inline ]
  fn default() -> Self
  {
    Self {
      max_memory_file_size : 10 * 1024 * 1024, // 10MB
      streaming_chunk_size : 64 * 1024, // 64KB
      max_concurrent_operations : 10,
      enable_compression : true,
      compression_quality : 85,
      cache_ttl_seconds : 3600, // 1 hour
      max_cache_size_bytes : 100 * 1024 * 1024, // 100MB
      enable_metrics : true,
      retry_config : MediaRetryConfig::default(),
      thumbnail_config : Some( ThumbnailConfig::default() ),
    }
  }
}

/// Retry configuration for media operations
#[ derive( Debug, Clone ) ]
pub struct MediaRetryConfig
{
  /// Maximum number of retry attempts
  pub max_attempts : u32,
  /// Initial delay between retries
  pub initial_delay : Duration,
  /// Maximum delay between retries
  pub max_delay : Duration,
  /// Backoff multiplier
  pub backoff_multiplier : f64,
  /// Enable jitter in retry delays
  pub enable_jitter : bool,
}

impl Default for MediaRetryConfig
{
  #[ inline ]
  fn default() -> Self
  {
    Self {
      max_attempts : 3,
      initial_delay : Duration::from_millis( 100 ),
      max_delay : Duration::from_secs( 10 ),
      backoff_multiplier : 2.0,
      enable_jitter : true,
    }
  }
}

/// Thumbnail generation configuration
#[ derive( Debug, Clone ) ]
pub struct ThumbnailConfig
{
  /// Enable automatic thumbnail generation
  pub enabled : bool,
  /// Thumbnail width in pixels
  pub width : u32,
  /// Thumbnail height in pixels
  pub height : u32,
  /// Thumbnail quality (0-100)
  pub quality : u8,
  /// Output format for thumbnails
  pub format : ThumbnailFormat,
}

impl Default for ThumbnailConfig
{
  #[ inline ]
  fn default() -> Self
  {
    Self {
      enabled : true,
      width : 256,
      height : 256,
      quality : 80,
      format : ThumbnailFormat::WebP,
    }
  }
}

/// Supported thumbnail formats
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub enum ThumbnailFormat
{
  /// JPEG format
  Jpeg,
  /// PNG format
  Png,
  /// WebP format (default)
  WebP,
}

/// Cache statistics report
#[ derive( Debug, Clone ) ]
pub struct MediaCacheStatsReport
{
  /// Number of cache hits
  pub hits : u64,
  /// Number of cache misses
  pub misses : u64,
  /// Hit rate as percentage
  pub hit_rate : f64,
  /// Number of evictions performed
  pub evictions : u64,
  /// Total cache size in bytes
  pub total_size_bytes : usize,
  /// Total compressed bytes stored
  pub total_compressed_bytes : u64,
  /// Average compression time per byte in microseconds
  pub avg_compression_time_us : u64,
}

/// Result of media processing
#[ derive( Debug, Clone ) ]
pub struct ProcessedMediaResult
{
  /// Processed media data
  pub processed_data : Bytes,
  /// Processing metadata
  pub metadata : ProcessedMediaMetadata,
  /// Whether result came from cache
  pub cache_hit : bool,
}

/// Metadata about processed media
#[ derive( Debug, Clone ) ]
pub struct ProcessedMediaMetadata
{
  /// Original file size in bytes
  pub original_size : usize,
  /// Processed file size in bytes
  pub processed_size : usize,
  /// MIME type
  pub mime_type : String,
  /// Whether compression was applied
  pub is_compressed : bool,
  /// Compression ratio achieved
  pub compression_ratio : f64,
  /// Processing time in milliseconds
  pub processing_time_ms : u64,
  /// Generated thumbnail data
  pub thumbnail_data : Option< Bytes >,
}

/// Media processing metrics report
#[ derive( Debug, Clone ) ]
pub struct MediaProcessingMetricsReport
{
  /// Total files processed
  pub files_processed : u64,
  /// Total bytes processed
  pub bytes_processed : u64,
  /// Average processing time per file in milliseconds
  pub avg_processing_time_ms : u64,
  /// Number of failed operations
  pub failed_operations : u64,
  /// Number of retries performed
  pub retries_performed : u64,
  /// Memory usage high watermark in bytes
  pub memory_high_watermark_bytes : usize,
  /// Cache statistics
  pub cache_stats : MediaCacheStatsReport,
}

// Re-export all public types for external use
// Using simple pub use since we're not using the private module pattern here
pub use self::{
  upload ::{ MediaProcessingPipeline, MediaProcessingMetrics },
  metadata ::{ MediaCache, MediaCacheStats, CachedMediaMetadata, ThumbnailGenerator },
  integration ::OptimizedMediaApi,
};
