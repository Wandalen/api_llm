// src/components/query.rs
//! Defines common query parameters used for listing resources (pagination, sorting).

/// Define a private namespace for all its items.
mod private
{
  // Serde and Former imports
  use serde::Serialize;
  use former::Former;

  /// Represents common query parameters for list endpoints supporting pagination and ordering.
  ///
  /// Fields correspond to standard parameters like `limit`, `order`, `after`, and `before`.
  /// Use the `Former` derive to construct instances easily.
  ///
  /// # Used By (as query parameter)
  /// - `/assistants` (GET)
  /// - `/threads/{thread_id}/messages` (GET)
  /// - `/threads/{thread_id}/runs` (GET)
  /// - `/threads/{thread_id}/runs/{run_id}/steps` (GET)
  /// - `/vector_stores` (GET)
  /// - `/vector_stores/{vector_store_id}/files` (GET)
  /// - `/vector_stores/{vector_store_id}/file_batches/{batch_id}/files` (GET)
  /// - `/files` (GET)
  /// - `/batches` (GET)
  /// - `/chat/completions` (GET)
  /// - `/fine_tuning/jobs` (GET)
  /// - `/fine_tuning/jobs/{fine_tuning_job_id}/events` (GET)
  /// - `/fine_tuning/jobs/{fine_tuning_job_id}/checkpoints` (GET)
  /// - `/fine_tuning/checkpoints/{permission_id}/permissions` (GET)
  /// - `/organization/admin_api_keys` (GET)
  /// - `/organization/audit_logs` (GET)
  /// - `/organization/invites` (GET)
  /// - `/organization/projects` (GET)
  /// - `/organization/projects/{project_id}/api_keys` (GET)
  /// - `/organization/projects/{project_id}/rate_limits` (GET)
  /// - `/organization/projects/{project_id}/service_accounts` (GET)
  /// - `/organization/projects/{project_id}/users` (GET)
  /// - `/organization/users` (GET)
  /// - `/responses/{response_id}/input_items` (GET) // Added this endpoint
  #[ derive( Debug, Serialize, Clone, PartialEq, Former ) ] // REMOVED Default
  pub struct ListQuery
  {
    /// A limit on the number of objects to be returned. Limit can range between 1 and 100, with a default depending on the endpoint (often 20).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub limit : Option< u32 >, // Using u32 as limit is typically non-negative

    /// Sort order by the `created_at` timestamp of the objects. `asc` for ascending order and `desc` for descending order. Default varies by endpoint (often `desc`).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub order : Option< String >, // Could be an enum Order { Asc, Desc }

    /// A cursor for use in pagination. `after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `after=obj_foo` in order to fetch the next page of the list.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub after : Option< String >,

    /// A cursor for use in pagination. `before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_foo`, your subsequent call can include `before=obj_foo` in order to fetch the previous page of the list.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub before : Option< String >,
  }

} // end mod private

crate ::mod_interface!
{
  exposed use ListQuery;
}