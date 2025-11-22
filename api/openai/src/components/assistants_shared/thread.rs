//! Thread-related types and structures for the Assistants API.

/// Define a private namespace for thread-related items.
mod private
{
  use crate::components::common::Metadata;
  use crate::components::assistants_shared::assistant::ToolResources;

  // Add serde imports
  use serde::{ Serialize, Deserialize };

  /// Represents a thread that contains messages.
  ///
  /// # Used By
  /// - `/threads` (POST response)
  /// - `/threads/{thread_id}` (GET, POST response)
  /// - `AssistantStreamEvent::ThreadCreated`
  /// - `MessageObject`
  /// - `RunObject`
  /// - `RunStepObject`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct ThreadObject
  {
    /// The identifier, which can be referenced in API endpoints.
    pub id : String,
    /// The object type, which is always `thread`.
    pub object : String,
    /// The Unix timestamp (in seconds) for when the thread was created.
    pub created_at : i64,
    /// A set of resources made available to the assistant's tools in this thread.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub tool_resources : Option< ToolResources >,
    /// Set of 16 key-value pairs attached to the object.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub metadata : Option< Metadata >,
  }

  /// Response containing a list of threads. (Note : `OpenAI` spec doesn't define a list threads endpoint, but this structure might be useful internally or for future expansion).
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ListThreadsResponse
  {
    /// The object type, always "list".
    pub object : String,
    /// A list of thread objects.
    pub data : Vec< ThreadObject >,
    /// The ID of the first thread in the list.
    pub first_id : String,
    /// The ID of the last thread in the list.
    pub last_id : String,
    /// Indicates whether there are more threads available.
    pub has_more : bool,
  }
}

crate ::mod_interface!
{
  exposed use private::ThreadObject;
  exposed use private::ListThreadsResponse;
}