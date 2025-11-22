// src/api/fine_tuning.rs
//! This module defines the `FineTuning` API client, which provides methods
//! for interacting with the `OpenAI` Fine-tuning API.
//!
//! For more details, refer to the [OpenAI Fine-tuning API documentation](https://platform.openai.com/docs/api-reference/fine-tuning).

/// Define a private namespace for all its items.
mod private
{
  // Use crate root for base access
  use crate::
  {
    client ::Client,
    error ::Result,
    environment ::{ OpenaiEnvironment, EnvironmentInterface },
  };
  use crate::components::fine_tuning_shared::
  {
    FineTuningJob,
    FineTuningJobEvent,
    ListFineTuningJobEventsResponse,
    ListPaginatedFineTuningJobsResponse,
    // FineTuningJobRequest - doesn't exist, need to create or use FineTuningJob
  };
  use crate::components::common::ListQuery;

  // External crates

  use serde_json;
  use tokio::sync::mpsc;

  /// The client for the `OpenAI` Fine-tuning API.
  #[ derive( Debug, Clone ) ]
  pub struct FineTuning< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client Client< E >,
  }

  impl< 'client, E > FineTuning< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Creates a new `FineTuning` client.
    ///
    /// # Arguments
    /// - `client`: The core `OpenAI` `Client` to use for requests.
    #[ inline ]
    pub(crate) fn new( client : &'client Client< E > ) -> Self
    {
      Self { client }
    }

    /// Creates a fine-tuning job.
    ///
    /// # Arguments
    /// - `request`: The request body for creating a fine-tuning job.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn create_job( &self, request : FineTuningJob ) -> Result< FineTuningJob >
    {
      self.client.post( "fine_tuning/jobs", &request ).await
    }

    /// Lists fine-tuning jobs.
    ///
    /// # Arguments
    /// - `query`: Optional query parameters for listing jobs.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn list_jobs( &self, query : Option< ListQuery > ) -> Result< ListPaginatedFineTuningJobsResponse >
    {
      let path = "/fine_tuning/jobs";
      if let Some( q ) = query
      {
        self.client.get_with_query( path, &q ).await
      }
      else
      {
        self.client.get( path ).await
      }
    }

    /// Retrieves a fine-tuning job.
    ///
    /// # Arguments
    /// - `job_id`: The ID of the fine-tuning job to retrieve.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn retrieve_job( &self, job_id : &str ) -> Result< FineTuningJob >
    {
      let path = format!( "/fine_tuning/jobs/{job_id}" );
      self.client.get( &path ).await
    }

    /// Cancels a fine-tuning job.
    ///
    /// # Arguments
    /// - `job_id`: The ID of the fine-tuning job to cancel.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn cancel_job( &self, job_id : &str ) -> Result< FineTuningJob >
    {
      let path = format!( "/fine_tuning/jobs/{job_id}/cancel" );
      self.client.post( &path, &serde_json::json!({}) ).await
    }

    /// Lists events for a fine-tuning job.
    ///
    /// # Arguments
    /// - `job_id`: The ID of the fine-tuning job.
    /// - `query`: Optional query parameters for listing events.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn list_job_events( &self, job_id : &str, query : Option< ListQuery > ) -> Result< ListFineTuningJobEventsResponse >
    {
      let path = format!( "/fine_tuning/jobs/{job_id}/events" );
      if let Some( q ) = query
      {
        self.client.get_with_query( &path, &q ).await
      }
      else
      {
        self.client.get( &path ).await
      }
    }

    /// Streams events for a fine-tuning job.
    ///
    /// # Arguments
    /// - `job_id`: The ID of the fine-tuning job.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn stream_job_events( &self, job_id : &str ) -> Result< mpsc::Receiver< Result< FineTuningJobEvent > > >
    {
      let path = format!( "/fine_tuning/jobs/{job_id}/events" );
      self.client.post_stream( &path, &serde_json::json!({ "stream": true }) ).await
    }
  }
} // end mod private

crate ::mod_interface!
{
  // Expose all structs defined in this module
  exposed use
  {
    FineTuning,
  };
}