// src/api/moderations.rs
//! This module defines the `Moderations` API client, which provides methods
//! for interacting with the `OpenAI` Moderation API.
//!
//! For more details, refer to the [`OpenAI` Moderation API documentation](https://platform.openai.com/docs/api-reference/moderations).

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
  use crate::components::moderations::
  {
    CreateModerationResponse,
  };

  // External crates


  /// The client for the `OpenAI` Moderation API.
  #[ derive( Debug, Clone ) ]
  pub struct Moderations< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client Client< E >,
  }

  impl< 'client, E > Moderations< 'client, E >
  where
    E : OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Creates a new `Moderations` client.
    ///
    /// # Arguments
    /// - `client`: The core `OpenAI` `Client` to use for requests.
    #[ inline ]
    pub(crate) fn new( client : &'client Client< E > ) -> Self
    {
      Self { client }
    }

    /// Classifies if text violates `OpenAI`'s content policy.
    ///
    /// # Arguments
    /// - `request`: The request body for moderation.
    ///
    /// # Errors
    /// Returns `OpenAIError` if the request fails.
    #[ inline ]
    pub async fn create( &self, request : serde_json::Value ) -> Result< CreateModerationResponse >
    {
      self.client.post( "moderations", &request ).await
    }
  }
} // end mod private

crate ::mod_interface!
{
  // Expose all structs defined in this module
  exposed use
  {
    Moderations,
  };
}