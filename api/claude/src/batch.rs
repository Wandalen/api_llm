//! Batch Messages API
//!
//! Provides asynchronous batch processing for up to 100,000 message requests.
//! Batch processing allows efficient handling of large-scale operations with
//! server-side request management and result retrieval.

#[ cfg( feature = "batch-processing" ) ]
mod private
{
  use serde::{ Serialize, Deserialize };

  /// Request item for batch processing
  ///
  /// Each batch request item contains a custom ID for result matching
  /// and the message request parameters.
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct BatchRequestItem
  {
    /// User-defined ID for matching results (max 256 characters)
    pub custom_id : String,
    /// Message request parameters
    pub params : crate::CreateMessageRequest,
  }

  impl BatchRequestItem
  {
    /// Create new batch request item
    #[ must_use ]
    pub fn new( custom_id : String, params : crate::CreateMessageRequest ) -> Self
    {
      Self { custom_id, params }
    }

    /// Validate batch request item
    ///
    /// # Errors
    ///
    /// Returns error if `custom_id` exceeds 256 characters or params are invalid
    pub fn validate( &self ) -> crate::AnthropicResult< () >
    {
      if self.custom_id.len() > 256
      {
        return Err( crate::AnthropicError::InvalidArgument(
          format!( "custom_id length {} exceeds maximum of 256 characters", self.custom_id.len() )
        ) );
      }

      if self.custom_id.is_empty()
      {
        return Err( crate::AnthropicError::InvalidArgument(
          "custom_id cannot be empty".to_string()
        ) );
      }

      // Delegate to CreateMessageRequest validation if available
      #[ cfg( feature = "error-handling" ) ]
      {
        self.params.validate()?;
      }

      Ok( () )
    }
  }

  /// Batch creation request
  ///
  /// Contains an array of message requests to process asynchronously.
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct CreateBatchRequest
  {
    /// Array of batch request items (up to 100,000 items, 256 MB max)
    pub requests : Vec< BatchRequestItem >,
  }

  impl CreateBatchRequest
  {
    /// Create new batch request
    #[ must_use ]
    pub fn new( requests : Vec< BatchRequestItem > ) -> Self
    {
      Self { requests }
    }

    /// Validate batch request
    ///
    /// # Errors
    ///
    /// Returns error if request count exceeds limits or items are invalid
    pub fn validate( &self ) -> crate::AnthropicResult< () >
    {
      const MAX_REQUESTS : usize = 100_000;

      if self.requests.is_empty()
      {
        return Err( crate::AnthropicError::InvalidArgument(
          "Batch request cannot be empty".to_string()
        ) );
      }

      if self.requests.len() > MAX_REQUESTS
      {
        return Err( crate::AnthropicError::InvalidArgument(
          format!( "Batch contains {} requests, exceeding maximum of {}", self.requests.len(), MAX_REQUESTS )
        ) );
      }

      // Validate each request item
      for ( idx, item ) in self.requests.iter().enumerate()
      {
        item.validate().map_err( | e |
          crate::AnthropicError::InvalidArgument(
            format!( "Batch request item {idx} invalid : {e}" )
          )
        )?;
      }

      Ok( () )
    }
  }

  /// Request counts for batch status
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq, Eq ) ]
  pub struct RequestCounts
  {
    /// Total number of requests in batch
    pub processing : u32,
    /// Number of successfully completed requests
    pub succeeded : u32,
    /// Number of requests with errors
    pub errored : u32,
    /// Number of canceled requests
    pub canceled : u32,
    /// Number of expired requests
    pub expired : u32,
  }

  /// Batch processing status
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq, Eq ) ]
  #[ serde( rename_all = "snake_case" ) ]
  pub enum BatchProcessingStatus
  {
    /// Batch is currently being processed
    InProgress,
    /// Batch processing has completed (success or failure)
    Ended,
    /// Batch processing was canceled
    Canceling,
    /// Batch has expired
    Expired,
  }

  /// Batch response from API
  ///
  /// Returned when creating or retrieving batch status.
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq, Eq ) ]
  pub struct BatchResponse
  {
    /// Unique batch identifier
    pub id : String,
    /// Object type (always `"message_batch"`)
    pub r#type : String,
    /// Current processing status
    pub processing_status : BatchProcessingStatus,
    /// Request count statistics
    pub request_counts : RequestCounts,
    /// When batch processing ended (if completed)
    pub ended_at : Option< String >,
    /// When batch was created
    pub created_at : String,
    /// When batch will expire
    pub expires_at : String,
    /// URL to download results (available when ended)
    pub results_url : Option< String >,
  }

  impl BatchResponse
  {
    /// Check if batch processing is complete
    #[ must_use ]
    pub fn is_completed( &self ) -> bool
    {
      matches!( self.processing_status, BatchProcessingStatus::Ended )
    }

    /// Check if results are available for download
    #[ must_use ]
    pub fn has_results( &self ) -> bool
    {
      self.results_url.is_some()
    }

    /// Get total request count
    #[ must_use ]
    pub fn total_requests( &self ) -> u32
    {
      self.request_counts.processing
        + self.request_counts.succeeded
        + self.request_counts.errored
        + self.request_counts.canceled
        + self.request_counts.expired
    }
  }

  /// Individual batch result from JSONL file
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct BatchResult
  {
    /// Custom ID from original request
    pub custom_id : String,
    /// Result type ("succeeded" or "errored")
    pub result_type : String,
    /// Message response (if succeeded)
    pub message : Option< crate::CreateMessageResponse >,
    /// Error information (if errored)
    pub error : Option< BatchResultError >,
  }

  /// Error information in batch result
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq, Eq ) ]
  pub struct BatchResultError
  {
    /// Error type
    pub r#type : String,
    /// Error message
    pub message : String,
  }

  /// Batch list response
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq, Eq ) ]
  pub struct BatchListResponse
  {
    /// List of batches
    pub data : Vec< BatchResponse >,
    /// Whether there are more results
    pub has_more : bool,
    /// ID of first item in list
    pub first_id : Option< String >,
    /// ID of last item in list
    pub last_id : Option< String >,
  }
}

#[ cfg( feature = "batch-processing" ) ]
crate::mod_interface!
{
  exposed use
  {
    BatchRequestItem,
    CreateBatchRequest,
    RequestCounts,
    BatchProcessingStatus,
    BatchResponse,
    BatchResult,
    BatchResultError,
    BatchListResponse,
  };
}

#[ cfg( not( feature = "batch-processing" ) ) ]
crate::mod_interface!
{
  // Empty - types not available without feature
}
