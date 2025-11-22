//! Batch Mode API data structures for async job-based processing.
//!
//! Batch Mode provides 50% cost discount for non-time-sensitive requests
//! with a 24-hour Service Level Objective (SLO).
//!
//! Reference : quickstarts/Batch_mode.ipynb

use serde::{ Deserialize, Serialize };
use std::time::SystemTime;

/// State of a batch job.
#[ derive( Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize ) ]
#[ serde( rename_all = "SCREAMING_SNAKE_CASE" ) ]
pub enum BatchJobState
{
  /// Job is pending execution
  Pending,
  /// Job is currently running
  Running,
  /// Job completed successfully
  Succeeded,
  /// Job failed
  Failed,
  /// Job was cancelled
  Cancelled,
  /// Job completed with some failures
  PartiallyCompleted,
}

/// Batch job information returned when creating a job.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct BatchJob
{
  /// Unique identifier for this batch job
  pub job_id : String,

  /// Current state of the job
  pub state : BatchJobState,

  /// Model used for this batch
  pub model : String,

  /// Number of requests in this batch
  pub request_count : usize,

  /// When the job was created
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub create_time : Option< SystemTime >,

  /// When the job results will expire
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub expiration_time : Option< SystemTime >,

  /// Error message if job failed
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub error : Option< String >,
}

/// Status information for a batch job.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct BatchJobStatus
{
  /// Job identifier
  pub job_id : String,

  /// Current job state
  pub state : BatchJobState,

  /// Number of completed requests
  #[ serde( default ) ]
  pub completed_count : usize,

  /// Number of failed requests
  #[ serde( default ) ]
  pub failed_count : usize,

  /// When the status was last updated
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub update_time : Option< SystemTime >,

  /// Error details if job failed
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub error : Option< String >,
}

/// Billing metadata for batch operations showing cost discount.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct BatchBillingMetadata
{
  /// Discount percentage (should be 50 for batch mode)
  pub discount_percentage : u32,

  /// Standard cost without discount
  pub standard_cost : f64,

  /// Discounted cost (50% off)
  pub discounted_cost : f64,

  /// Total tokens processed
  #[ serde( default ) ]
  pub total_tokens : i32,
}

/// Results from a completed batch job.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct BatchJobResults
{
  /// Job identifier
  pub job_id : String,

  /// Final job state
  pub state : BatchJobState,

  /// Individual responses for each request
  pub responses : Vec< super::GenerateContentResponse >,

  /// Billing information showing 50% discount
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub billing_metadata : Option< BatchBillingMetadata >,

  /// When results were retrieved
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub retrieve_time : Option< SystemTime >,
}

/// Results from a batch embedding job.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct BatchEmbeddingResults
{
  /// Job identifier
  pub job_id : String,

  /// Final job state
  pub state : BatchJobState,

  /// Individual embeddings for each text
  pub embeddings : Vec< super::ContentEmbedding >,

  /// Billing information
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub billing_metadata : Option< BatchBillingMetadata >,
}

/// List of batch jobs with pagination.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct BatchJobList
{
  /// List of jobs
  pub jobs : Vec< BatchJob >,

  /// Token for next page if more results available
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub next_page_token : Option< String >,
}

/// Request to create a batch job with inline requests.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct CreateBatchJobRequest
{
  /// Model to use for all requests
  pub model : String,

  /// Individual requests to process
  pub requests : Vec< super::GenerateContentRequest >,
}

/// Request to create a batch embedding job.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct CreateBatchEmbeddingRequest
{
  /// Model to use for embeddings
  pub model : String,

  /// Texts to embed
  pub texts : Vec< String >,
}
