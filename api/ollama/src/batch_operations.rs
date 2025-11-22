//! Batch Operations functionality for the Ollama API client
//!
//! This module provides comprehensive batch operations capabilities including:
//! - Multiple requests in single API calls for improved throughput
//! - Bulk processing with intelligent concurrency management
//! - Performance optimization through request batching
//! - Error handling for partial batch failures
//!
//! All functionality follows the "Thin Client, Rich API" governing principle,
//! providing explicit control with transparent batch operation management.

use serde::Serialize;
use core::time::Duration;
use crate::{ ChatRequest, GenerateRequest };

/// Request structure for batch chat operations
#[ derive( Debug, Clone, Serialize ) ]
pub struct BatchChatRequest
{
  /// List of chat requests to process in batch
  pub requests : Vec< ChatRequest >,
  /// Batch operation configuration
  pub batch_config : Option< BatchOperationConfig >,
  /// Enable parallel execution of requests
  pub parallel_execution : bool,
  /// Stop processing on first error
  pub fail_fast : bool,
  /// Overall timeout for the entire batch
  pub timeout : Option< Duration >,
}

/// Response structure for batch chat operations
#[ derive( Debug, Clone, Serialize ) ]
pub struct BatchChatResponse
{
  /// Results for each request in the batch
  pub results : Vec< BatchResult >,
  /// Total number of requests processed
  pub total_requests : usize,
  /// Number of successful requests
  pub successful_requests : usize,
  /// Number of failed requests
  pub failed_requests : usize,
  /// Total execution time in milliseconds
  pub execution_time_ms : u64,
  /// Throughput in requests per second
  pub throughput_requests_per_second : f64,
  /// List of optimizations applied during batch processing
  pub batch_optimizations : Option< Vec< String > >,
  /// Error messages from batch processing
  pub errors : Vec< String >,
}

/// Request structure for batch generation operations
#[ derive( Debug, Clone, Serialize ) ]
pub struct BatchGenerateRequest
{
  /// List of generation requests to process in batch
  pub requests : Vec< GenerateRequest >,
  /// Batch operation configuration
  pub batch_config : Option< BatchOperationConfig >,
  /// Enable parallel execution of requests
  pub parallel_execution : bool,
  /// Stop processing on first error
  pub fail_fast : bool,
  /// Overall timeout for the entire batch
  pub timeout : Option< Duration >,
}

/// Response structure for batch generation operations
#[ derive( Debug, Clone, Serialize ) ]
pub struct BatchGenerateResponse
{
  /// Results for each request in the batch
  pub results : Vec< BatchResult >,
  /// Total number of requests processed
  pub total_requests : usize,
  /// Number of successful requests
  pub successful_requests : usize,
  /// Number of failed requests
  pub failed_requests : usize,
  /// Total execution time in milliseconds
  pub execution_time_ms : u64,
  /// Throughput in requests per second
  pub throughput_requests_per_second : f64,
  /// List of optimizations applied during batch processing
  pub batch_optimizations : Option< Vec< String > >,
  /// Error messages from batch processing
  pub errors : Vec< String >,
}

/// Configuration for batch operations
#[ derive( Debug, Clone, Serialize ) ]
pub struct BatchOperationConfig
{
  /// Maximum number of requests in a single batch
  max_batch_size : usize,
  /// Maximum number of concurrent requests
  concurrent_limit : usize,
  /// Whether to retry failed requests
  retry_failed : bool,
  /// Whether to preserve request order in results
  preserve_order : bool,
  /// Timeout per individual request
  timeout_per_request : Option< Duration >,
  /// Chunk size for processing large batches
  chunk_size : usize,
}

/// Result of a single request within a batch
#[ derive( Debug, Clone, Serialize ) ]
pub enum BatchResult
{
  /// Successful request result
  Success( serde_json::Value ),
  /// Failed request with error details
  Error( BatchError ),
}

/// Error information for a failed request in a batch
#[ derive( Debug, Clone, Serialize ) ]
pub struct BatchError
{
  /// Index of the failed request in the original batch
  pub request_index : usize,
  /// Error code categorizing the failure
  pub error_code : String,
  /// Human-readable error message
  pub error_message : String,
  /// Whether this error is recoverable through retry
  pub recoverable : bool,
}

impl BatchChatRequest
{
  /// Create a new batch chat request
  #[ inline ]
  #[ must_use ]
  pub fn new( requests : Vec< ChatRequest > ) -> Self
  {
    Self
    {
      requests,
      batch_config : None,
      parallel_execution : true,
      fail_fast : false,
      timeout : None,
    }
  }

  /// Set batch configuration
  #[ inline ]
  #[ must_use ]
  pub fn with_config( mut self, config : BatchOperationConfig ) -> Self
  {
    self.batch_config = Some( config );
    self
  }

  /// Set parallel execution mode
  #[ inline ]
  #[ must_use ]
  pub fn with_parallel_execution( mut self, parallel : bool ) -> Self
  {
    self.parallel_execution = parallel;
    self
  }

  /// Set fail-fast behavior
  #[ inline ]
  #[ must_use ]
  pub fn with_fail_fast( mut self, fail_fast : bool ) -> Self
  {
    self.fail_fast = fail_fast;
    self
  }

  /// Set batch timeout
  #[ inline ]
  #[ must_use ]
  pub fn with_timeout( mut self, timeout : Duration ) -> Self
  {
    self.timeout = Some( timeout );
    self
  }

  /// Get the number of requests in this batch
  #[ inline ]
  #[ must_use ]
  pub fn request_count( &self ) -> usize
  {
    self.requests.len()
  }

  /// Check if the batch is empty
  #[ inline ]
  #[ must_use ]
  pub fn is_empty( &self ) -> bool
  {
    self.requests.is_empty()
  }
}

impl BatchGenerateRequest
{
  /// Create a new batch generation request
  #[ inline ]
  #[ must_use ]
  pub fn new( requests : Vec< GenerateRequest > ) -> Self
  {
    Self
    {
      requests,
      batch_config : None,
      parallel_execution : true,
      fail_fast : false,
      timeout : None,
    }
  }

  /// Set batch configuration
  #[ inline ]
  #[ must_use ]
  pub fn with_config( mut self, config : BatchOperationConfig ) -> Self
  {
    self.batch_config = Some( config );
    self
  }

  /// Set parallel execution mode
  #[ inline ]
  #[ must_use ]
  pub fn with_parallel_execution( mut self, parallel : bool ) -> Self
  {
    self.parallel_execution = parallel;
    self
  }

  /// Set fail-fast behavior
  #[ inline ]
  #[ must_use ]
  pub fn with_fail_fast( mut self, fail_fast : bool ) -> Self
  {
    self.fail_fast = fail_fast;
    self
  }

  /// Set batch timeout
  #[ inline ]
  #[ must_use ]
  pub fn with_timeout( mut self, timeout : Duration ) -> Self
  {
    self.timeout = Some( timeout );
    self
  }

  /// Get the number of requests in this batch
  #[ inline ]
  #[ must_use ]
  pub fn request_count( &self ) -> usize
  {
    self.requests.len()
  }

  /// Check if the batch is empty
  #[ inline ]
  #[ must_use ]
  pub fn is_empty( &self ) -> bool
  {
    self.requests.is_empty()
  }
}

impl BatchOperationConfig
{
  /// Create a new batch operation configuration with defaults
  #[ inline ]
  #[ must_use ]
  pub fn new() -> Self
  {
    Self
    {
      max_batch_size : 100,
      concurrent_limit : 5,
      retry_failed : false,
      preserve_order : true,
      timeout_per_request : None,
      chunk_size : 10,
    }
  }

  /// Set maximum batch size
  #[ inline ]
  #[ must_use ]
  pub fn with_max_batch_size( mut self, max_size : usize ) -> Self
  {
    self.max_batch_size = max_size.max( 1 ); // Ensure at least 1
    self
  }

  /// Set concurrent limit
  #[ inline ]
  #[ must_use ]
  pub fn with_concurrent_limit( mut self, limit : usize ) -> Self
  {
    self.concurrent_limit = limit.max( 1 ); // Ensure at least 1
    self
  }

  /// Set retry failed requests
  #[ inline ]
  #[ must_use ]
  pub fn with_retry_failed( mut self, retry : bool ) -> Self
  {
    self.retry_failed = retry;
    self
  }

  /// Set preserve order
  #[ inline ]
  #[ must_use ]
  pub fn with_preserve_order( mut self, preserve : bool ) -> Self
  {
    self.preserve_order = preserve;
    self
  }

  /// Set timeout per request
  #[ inline ]
  #[ must_use ]
  pub fn with_timeout_per_request( mut self, timeout : Duration ) -> Self
  {
    self.timeout_per_request = Some( timeout );
    self
  }

  /// Set chunk size for large batches
  #[ inline ]
  #[ must_use ]
  pub fn with_chunk_size( mut self, size : usize ) -> Self
  {
    self.chunk_size = size.max( 1 ); // Ensure at least 1
    self
  }

  /// Get maximum batch size
  #[ inline ]
  #[ must_use ]
  pub fn max_batch_size( &self ) -> usize
  {
    self.max_batch_size
  }

  /// Get concurrent limit
  #[ inline ]
  #[ must_use ]
  pub fn concurrent_limit( &self ) -> usize
  {
    self.concurrent_limit
  }

  /// Get retry failed setting
  #[ inline ]
  #[ must_use ]
  pub fn retry_failed( &self ) -> bool
  {
    self.retry_failed
  }

  /// Get preserve order setting
  #[ inline ]
  #[ must_use ]
  pub fn preserve_order( &self ) -> bool
  {
    self.preserve_order
  }

  /// Get timeout per request
  #[ inline ]
  #[ must_use ]
  pub fn timeout_per_request( &self ) -> Option< Duration >
  {
    self.timeout_per_request
  }

  /// Get chunk size
  #[ inline ]
  #[ must_use ]
  pub fn chunk_size( &self ) -> usize
  {
    self.chunk_size
  }
}

impl Default for BatchOperationConfig
{
  #[ inline ]
  fn default() -> Self
  {
    Self::new()
  }
}

impl BatchResult
{
  /// Check if the result is successful
  #[ inline ]
  #[ must_use ]
  pub fn is_success( &self ) -> bool
  {
    matches!( self, BatchResult::Success( _ ) )
  }

  /// Check if the result is an error
  #[ inline ]
  #[ must_use ]
  pub fn is_error( &self ) -> bool
  {
    matches!( self, BatchResult::Error( _ ) )
  }

  /// Get the success value if this result is successful
  #[ inline ]
  #[ must_use ]
  pub fn success_value( &self ) -> Option< &serde_json::Value >
  {
    match self
    {
      BatchResult::Success( value ) => Some( value ),
      BatchResult::Error( _ ) => None,
    }
  }

  /// Get the error if this result is an error
  #[ inline ]
  #[ must_use ]
  pub fn error_value( &self ) -> Option< &BatchError >
  {
    match self
    {
      BatchResult::Success( _ ) => None,
      BatchResult::Error( error ) => Some( error ),
    }
  }
}

impl BatchError
{
  /// Create a new batch error
  #[ inline ]
  #[ must_use ]
  pub fn new( request_index : usize, error_code : String, error_message : String, recoverable : bool ) -> Self
  {
    Self
    {
      request_index,
      error_code,
      error_message,
      recoverable,
    }
  }

  /// Create a recoverable error
  #[ inline ]
  #[ must_use ]
  pub fn recoverable( request_index : usize, error_code : String, error_message : String ) -> Self
  {
    Self::new( request_index, error_code, error_message, true )
  }

  /// Create a non-recoverable error
  #[ inline ]
  #[ must_use ]
  pub fn non_recoverable( request_index : usize, error_code : String, error_message : String ) -> Self
  {
    Self::new( request_index, error_code, error_message, false )
  }

  /// Check if this error is recoverable
  #[ inline ]
  #[ must_use ]
  pub fn is_recoverable( &self ) -> bool
  {
    self.recoverable
  }
}

impl BatchChatResponse
{
  /// Calculate success rate as percentage
  #[ inline ]
  #[ must_use ]
  pub fn success_rate( &self ) -> f64
  {
    if self.total_requests == 0
    {
      return 0.0;
    }
    ( self.successful_requests as f64 / self.total_requests as f64 ) * 100.0
  }

  /// Check if all requests were successful
  #[ inline ]
  #[ must_use ]
  pub fn all_successful( &self ) -> bool
  {
    self.failed_requests == 0
  }

  /// Check if any requests failed
  #[ inline ]
  #[ must_use ]
  pub fn has_failures( &self ) -> bool
  {
    self.failed_requests > 0
  }

  /// Get successful results only
  #[ inline ]
  #[ must_use ]
  pub fn successful_results( &self ) -> Vec< &serde_json::Value >
  {
    self.results.iter()
      .filter_map( | result | result.success_value() )
      .collect()
  }

  /// Get error results only
  #[ inline ]
  #[ must_use ]
  pub fn error_results( &self ) -> Vec< &BatchError >
  {
    self.results.iter()
      .filter_map( | result | result.error_value() )
      .collect()
  }
}

impl BatchGenerateResponse
{
  /// Calculate success rate as percentage
  #[ inline ]
  #[ must_use ]
  pub fn success_rate( &self ) -> f64
  {
    if self.total_requests == 0
    {
      return 0.0;
    }
    ( self.successful_requests as f64 / self.total_requests as f64 ) * 100.0
  }

  /// Check if all requests were successful
  #[ inline ]
  #[ must_use ]
  pub fn all_successful( &self ) -> bool
  {
    self.failed_requests == 0
  }

  /// Check if any requests failed
  #[ inline ]
  #[ must_use ]
  pub fn has_failures( &self ) -> bool
  {
    self.failed_requests > 0
  }

  /// Get successful results only
  #[ inline ]
  #[ must_use ]
  pub fn successful_results( &self ) -> Vec< &serde_json::Value >
  {
    self.results.iter()
      .filter_map( | result | result.success_value() )
      .collect()
  }

  /// Get error results only
  #[ inline ]
  #[ must_use ]
  pub fn error_results( &self ) -> Vec< &BatchError >
  {
    self.results.iter()
      .filter_map( | result | result.error_value() )
      .collect()
  }
}