//! Structures shared across the Batch API.

/// Define a private namespace for all its items.
mod private
{
  // Use full paths from crate root for components
  use crate::components::common::Metadata;
  // Serde imports
  use serde::{ Serialize, Deserialize };
  use serde_json::Value; // Needed for BatchRequestOutput body

  // Note : Many structs here are duplicates from administration_shared.rs
  // In a real implementation, these would likely be defined in a common place
  // (like common.rs or a dedicated admin.rs) and reused.
  // For this exercise, I'm keeping them separate as generated.

  /// Represents the owner of an Admin API Key (User or Service Account).
  ///
  /// # Used By
  /// - `AdminApiKey`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AdminApiKeyOwner
  {
    /// Type of owner ("user" or "`service_account`").
    pub r#type : String,
    /// ID of the owner.
    pub id : String,
    /// Name of the owner.
    pub name : String,
    /// Creation timestamp.
    pub created_at : i64,
    /// Role of the owner within the organization.
    pub role : String,
  }

  /// Represents an Admin API Key.
  ///
  /// # Used By
  /// - `ApiKeyList`
  /// - `/organization/admin_api_keys` (POST response)
  /// - `/organization/admin_api_keys/{key_id}` (GET response)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AdminApiKey
  {
    /// Object type, always "`organization.admin_api_key`".
    pub object : String,
    /// Key ID.
    pub id : String,
    /// User-defined name for the key.
    pub name : String,
    /// Redacted value (e.g., "sk-admin...def").
    pub redacted_value : String,
    /// The full, unredacted key value (only present on creation).
    pub value : Option< String >,
    /// Creation timestamp.
    pub created_at : i64,
    /// Details of the key owner.
    pub owner : Option< AdminApiKeyOwner >,
  }

  /// Response containing a list of Admin API Keys.
  ///
  /// # Used By
  /// - `/organization/admin_api_keys` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ApiKeyList
  {
    /// Object type, always "list".
    pub object : String,
    /// List of Admin API Key objects.
    pub data : Vec< AdminApiKey >,
    /// Indicates if more keys are available for pagination.
    pub has_more : bool,
    /// ID of the first key in the list.
    pub first_id : Option< String >,
    /// ID of the last key in the list.
    pub last_id : Option< String >,
  }

  /// Represents project membership granted upon invite acceptance.
  ///
  /// # Used By
  /// - `Invite`
  /// - `InviteRequest` (within `requests/administration.rs` - *assuming*)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct InviteProject
  {
    /// Project ID.
    pub id : String,
    /// Project membership role ("member" or "owner").
    pub role : String,
  }

  /// Represents an invitation to join an organization.
  ///
  /// # Used By
  /// - `InviteListResponse`
  /// - `/organization/invites` (POST response)
  /// - `/organization/invites/{invite_id}` (GET response)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct Invite
  {
    /// Object type, always "organization.invite".
    pub object : String,
    /// Invite ID.
    pub id : String,
    /// Email address invited.
    pub email : String,
    /// Organization role assigned upon acceptance ("owner" or "reader").
    pub role : String,
    /// Invite status ("accepted", "expired", or "pending").
    pub status : String,
    /// Invitation timestamp.
    pub invited_at : i64,
    /// Expiration timestamp.
    pub expires_at : i64,
    /// Acceptance timestamp (null if not accepted).
    pub accepted_at : Option< i64 >,
    /// Projects granted membership upon acceptance.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub projects : Option< Vec< InviteProject > >,
  }

  /// Response containing a list of organization invites.
  ///
  /// # Used By
  /// - `/organization/invites` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct InviteListResponse
  {
    /// Object type, always "list".
    pub object : String,
    /// List of Invite objects.
    pub data : Vec< Invite >,
    /// ID of the first invite in the list.
    pub first_id : Option< String >,
    /// ID of the last invite in the list.
    pub last_id : Option< String >,
    /// Indicates if more invites are available.
    pub has_more : bool,
  }

  /// Represents a user within an organization.
  ///
  /// # Used By
  /// - `UserListResponse`
  /// - `/organization/users/{user_id}` (GET, POST response)
  /// - `AuditLogActorUser` (within `audit_logs_shared.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct User
  {
    /// Object type, always "organization.user".
    pub object : String,
    /// User ID.
    pub id : String,
    /// User's full name.
    pub name : String,
    /// User's email address.
    pub email : String,
    /// User's organization role ("owner" or "reader").
    pub role : String,
    /// Timestamp when the user was added.
    pub added_at : i64,
  }

  /// Response containing a list of organization users.
  ///
  /// # Used By
  /// - `/organization/users` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct UserListResponse
  {
    /// Object type, always "list".
    pub object : String,
    /// List of User objects.
    pub data : Vec< User >,
    /// ID of the first user in the list.
    pub first_id : Option< String >,
    /// ID of the last user in the list.
    pub last_id : Option< String >,
    /// Indicates if more users are available.
    pub has_more : bool,
  }

  /// Represents an organization project.
  ///
  /// # Used By
  /// - `ProjectListResponse`
  /// - `/organization/projects` (POST response)
  /// - `/organization/projects/{project_id}` (GET, POST response)
  /// - `/organization/projects/{project_id}/archive` (POST response)
  /// - `AuditLogProject` (within `audit_logs_shared.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct Project
  {
    /// Project ID.
    pub id : String,
    /// Object type, always "organization.project".
    pub object : String,
    /// Project name (appears in reports).
    pub name : String,
    /// Creation timestamp.
    pub created_at : i64,
    /// Archive timestamp (null if active).
    pub archived_at : Option< i64 >,
    /// Project status ("active" or "archived").
    pub status : String,
  }

  /// Response containing a list of organization projects.
  ///
  /// # Used By
  /// - `/organization/projects` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ProjectListResponse
  {
    /// Object type, always "list".
    pub object : String,
    /// List of Project objects.
    pub data : Vec< Project >,
    /// ID of the first project in the list.
    pub first_id : Option< String >,
    /// ID of the last project in the list.
    pub last_id : Option< String >,
    /// Indicates if more projects are available.
    pub has_more : bool,
  }

  /// Represents a user's membership within a specific project.
  ///
  /// # Used By
  /// - `ProjectApiKeyOwner`
  /// - `ProjectUserListResponse`
  /// - `/organization/projects/{project_id}/users` (POST response)
  /// - `/organization/projects/{project_id}/users/{user_id}` (GET, POST response)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct ProjectUser
  {
    /// Object type, always "organization.project.user".
    pub object : String,
    /// User ID.
    pub id : String,
    /// User's name.
    pub name : String,
    /// User's email.
    pub email : String,
    /// User's role within the project ("owner" or "member").
    pub role : String,
    /// Timestamp when the user was added to the project.
    pub added_at : i64,
  }

  /// Represents a service account within a project.
  ///
  /// # Used By
  /// - `ProjectApiKeyOwner`
  /// - `ProjectServiceAccountListResponse`
  /// - `/organization/projects/{project_id}/service_accounts/{service_account_id}` (GET response)
  /// - `AuditLogActorServiceAccount` (within `audit_logs_shared.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct ProjectServiceAccount
  {
    /// Object type, always "`organization.project.service_account`".
    pub object : String,
    /// Service account ID.
    pub id : String,
    /// Service account name.
    pub name : String,
    /// Service account role ("owner" or "member").
    pub role : String,
    /// Creation timestamp.
    pub created_at : i64,
  }

  /// Represents the owner of a Project API Key (either a User or Service Account).
  ///
  /// # Used By
  /// - `ProjectApiKey`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ProjectApiKeyOwner
  {
    /// Type of owner ("user" or "`service_account`").
    pub r#type : String,
    /// User details if owner is a user.
    pub user : Option< ProjectUser >,
    /// Service account details if owner is a service account.
    pub service_account : Option< ProjectServiceAccount >,
  }

  /// Represents an API key associated with a project.
  ///
  /// # Used By
  /// - `ProjectApiKeyListResponse`
  /// - `/organization/projects/{project_id}/api_keys/{key_id}` (GET response)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ProjectApiKey
  {
    /// Object type, always "`organization.project.api_key`".
    pub object : String,
    /// Redacted value of the API key (e.g., "sk-abc...def").
    pub redacted_value : String,
    /// Name of the API key.
    pub name : String,
    /// Creation timestamp.
    pub created_at : i64,
    /// Key ID.
    pub id : String,
    /// Details of the key owner.
    pub owner : ProjectApiKeyOwner,
  }

  /// Response containing a list of project API keys.
  ///
  /// # Used By
  /// - `/organization/projects/{project_id}/api_keys` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ProjectApiKeyListResponse
  {
    /// Object type, always "list".
    pub object : String,
    /// List of Project API Key objects.
    pub data : Vec< ProjectApiKey >,
    /// ID of the first key in the list.
    pub first_id : Option< String >,
    /// ID of the last key in the list.
    pub last_id : Option< String >,
    /// Indicates if more keys are available.
    pub has_more : bool,
  }

  /// Represents rate limit configuration for a specific model within a project.
  ///
  /// # Used By
  /// - `ProjectRateLimitListResponse`
  /// - `/organization/projects/{project_id}/rate_limits/{rate_limit_id}` (POST response)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct ProjectRateLimit
  {
    /// Object type, always "`project.rate_limit`".
    pub object : String,
    /// Rate limit ID.
    pub id : String,
    /// Model identifier this limit applies to.
    pub model : String,
    /// Maximum requests per minute.
    pub max_requests_per_1_minute : i32,
    /// Maximum tokens per minute.
    pub max_tokens_per_1_minute : i32,
    /// Maximum images per minute (relevant models only).
    pub max_images_per_1_minute : Option< i32 >,
    /// Maximum audio megabytes per minute (relevant models only).
    pub max_audio_megabytes_per_1_minute : Option< i32 >,
    /// Maximum requests per day (relevant models only).
    pub max_requests_per_1_day : Option< i32 >,
    /// Maximum batch input tokens per day (relevant models only).
    pub batch_1_day_max_input_tokens : Option< i32 >,
  }

  /// Response containing a list of project rate limits.
  ///
  /// # Used By
  /// - `/organization/projects/{project_id}/rate_limits` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ProjectRateLimitListResponse
  {
    /// Object type, always "list".
    pub object : String,
    /// List of Project Rate Limit objects.
    pub data : Vec< ProjectRateLimit >,
    /// ID of the first rate limit in the list.
    pub first_id : Option< String >,
    /// ID of the last rate limit in the list.
    pub last_id : Option< String >,
    /// Indicates if more rate limits are available.
    pub has_more : bool,
  }

  /// Response containing a list of project service accounts.
  ///
  /// # Used By
  /// - `/organization/projects/{project_id}/service_accounts` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ProjectServiceAccountListResponse
  {
    /// Object type, always "list".
    pub object : String,
    /// List of Project Service Account objects.
    pub data : Vec< ProjectServiceAccount >,
    /// ID of the first service account in the list.
    pub first_id : Option< String >,
    /// ID of the last service account in the list.
    pub last_id : Option< String >,
    /// Indicates if more service accounts are available.
    pub has_more : bool,
  }

  /// Represents the API key generated for a new service account (only present on creation).
  ///
  /// # Used By
  /// - `ProjectServiceAccountCreateResponse`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ProjectServiceAccountApiKey
  {
    /// Object type, always "`organization.project.service_account.api_key`".
    pub object : String,
    /// The unredacted API key value.
    pub value : String,
    /// Name of the API key.
    pub name : String,
    /// Creation timestamp.
    pub created_at : i64,
    /// Key ID.
    pub id : String,
  }

  /// Response object returned when creating a project service account.
  ///
  /// # Used By
  /// - `/organization/projects/{project_id}/service_accounts` (POST)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ProjectServiceAccountCreateResponse
  {
    /// Object type, always "`organization.project.service_account`".
    pub object : String,
    /// Service account ID.
    pub id : String,
    /// Service account name.
    pub name : String,
    /// Service account role (always "member").
    pub role : String,
    /// Creation timestamp.
    pub created_at : i64,
    /// The generated API key for the service account.
    pub api_key : ProjectServiceAccountApiKey,
  }

  /// Response containing a list of users within a project.
  ///
  /// # Used By
  /// - `/organization/projects/{project_id}/users` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ProjectUserListResponse
  {
    /// Object type, always "list".
    pub object : String,
    /// List of Project User objects.
    pub data : Vec< ProjectUser >,
    /// ID of the first user in the list.
    pub first_id : Option< String >,
    /// ID of the last user in the list.
    pub last_id : Option< String >,
    /// Indicates if more users are available.
    pub has_more : bool,
  }

  // --- Batch API Specific ---

  /// Represents error details within a Batch object.
  ///
  /// # Used By
  /// - `Batch`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct BatchErrorData
  {
      /// An error code identifying the error type.
      pub code : Option< String >,
      /// A human-readable message providing more details about the error.
      pub message : Option< String >,
      /// The name of the parameter that caused the error, if applicable.
      pub param : Option< String >,
      /// The line number of the input file where the error occurred, if applicable.
      pub line : Option< i64 >,
  }

  /// Represents the list of errors for a Batch job.
  ///
  /// # Used By
  /// - `Batch`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct BatchErrors
  {
      /// The object type, which is always `list`.
      pub object : Option< String >,
      /// List of error data objects.
      pub data : Option< Vec< BatchErrorData > >,
  }

  /// Represents the request counts for different statuses within a batch job.
  ///
  /// # Used By
  /// - `Batch`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct BatchRequestCounts
  {
      /// Total number of requests in the batch.
      pub total : i64,
      /// Number of requests that have been completed successfully.
      pub completed : i64,
      /// Number of requests that have failed.
      pub failed : i64,
  }

  /// Represents a Batch job object.
  ///
  /// # Used By
  /// - `/batches` (POST response, GET - in `ListBatchesResponse`)
  /// - `/batches/{batch_id}` (GET)
  /// - `/batches/{batch_id}/cancel` (POST response)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct Batch
  {
      /// Batch job ID.
      pub id : String,
      /// The object type, always `batch`.
      pub object : String,
      /// The `OpenAI` API endpoint used by the batch.
      pub endpoint : String,
      /// Errors associated with the batch job.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub errors : Option< BatchErrors >,
      /// The ID of the input file for the batch.
      pub input_file_id : String,
      /// The time frame within which the batch should be processed (e.g., "24h").
      pub completion_window : String,
      /// The current status of the batch.
      pub status : String, // Enum : validating, failed, in_progress, finalizing, completed, expired, cancelling, cancelled
      /// The ID of the file containing the outputs of successfully executed requests.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub output_file_id : Option< String >,
      /// The ID of the file containing the outputs of requests with errors.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub error_file_id : Option< String >,
      /// Creation timestamp.
      pub created_at : i64,
      /// Timestamp when processing started.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub in_progress_at : Option< i64 >,
      /// Expiration timestamp.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub expires_at : Option< i64 >,
      /// Timestamp when finalization started.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub finalizing_at : Option< i64 >,
      /// Completion timestamp.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub completed_at : Option< i64 >,
      /// Failure timestamp.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub failed_at : Option< i64 >,
      /// Timestamp when the batch expired.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub expired_at : Option< i64 >,
      /// Timestamp when cancellation started.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub cancelling_at : Option< i64 >,
      /// Timestamp when the batch was cancelled.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub cancelled_at : Option< i64 >,
      /// Request counts for different statuses within the batch.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub request_counts : Option< BatchRequestCounts >,
      /// Metadata associated with the batch.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub metadata : Option< Metadata >,
  }

  /// Response containing a list of Batch jobs.
  ///
  /// # Used By
  /// - `/batches` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
  pub struct ListBatchesResponse
  {
      /// List of Batch objects.
      pub data : Vec< Batch >,
      /// ID of the first batch in the list.
      pub first_id : Option< String >,
      /// ID of the last batch in the list.
      pub last_id : Option< String >,
      /// Indicates if more batches are available.
      pub has_more : bool,
      /// Object type, always "list".
      pub object : String,
  }

  /// Represents the input structure for a single request within a batch file (JSONL format).
  ///
  /// # Used By
  /// - Batch input files (.jsonl)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct BatchRequestInput
  {
      /// A developer-provided per-request id used to match outputs to inputs. Must be unique per batch.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub custom_id : Option< String >,
      /// The HTTP method (currently only "POST").
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub method : Option< String >,
      /// The `OpenAI` API relative URL (e.g., "/v1/chat/completions").
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub url : Option< String >,
      // Note : The 'body' field containing the actual request payload (e.g., CreateChatCompletionRequest)
      // is handled dynamically during batch file creation/parsing, typically using serde_json::Value.
      // It's not explicitly defined with a concrete type here to maintain flexibility.
  }

  /// Represents the response structure for a single request within a batch output file.
  ///
  /// # Used By
  /// - Batch output/error files (.jsonl)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
  pub struct BatchRequestOutputResponse
  {
      /// The HTTP status code of the response.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub status_code : Option< i32 >,
      /// The unique `OpenAI` request ID.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub request_id : Option< String >,
      /// The JSON body of the response. Type depends on the endpoint called.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub body : Option< Value >,
  }

  /// Represents error details for a failed request within a batch output file.
  ///
  /// # Used By
  /// - `BatchRequestOutput`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
  pub struct BatchRequestOutputError
  {
      /// A machine-readable error code.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub code : Option< String >,
      /// A human-readable error message.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub message : Option< String >,
  }

  /// Represents the output structure for a single request within a batch output/error file (JSONL format).
  ///
  /// # Used By
  /// - Batch output/error files (.jsonl)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
  pub struct BatchRequestOutput
  {
      /// The unique ID of the batch request.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub id : Option< String >,
      /// The developer-provided custom ID corresponding to the input request.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub custom_id : Option< String >,
      /// The response object if the request was successful.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub response : Option< BatchRequestOutputResponse >,
      /// The error object if the request failed.
      #[ serde( skip_serializing_if = "Option::is_none" ) ]
      pub error : Option< BatchRequestOutputError >,
  }

} // end mod private

crate ::mod_interface!
{
  // Note : Re-exporting admin types again here, ideally they'd be in a common admin module.
  // qqq : rid of duplications
  exposed use
  {
    AdminApiKeyOwner,
    AdminApiKey,
    ApiKeyList,
    InviteProject,
    Invite,
    InviteListResponse,
    User,
    UserListResponse,
    Project,
    ProjectListResponse,
    ProjectUser,
    ProjectServiceAccount,
    ProjectApiKeyOwner,
    ProjectApiKey,
    ProjectApiKeyListResponse,
    ProjectRateLimit,
    ProjectRateLimitListResponse,
    ProjectServiceAccountListResponse,
    ProjectServiceAccountApiKey,
    ProjectServiceAccountCreateResponse,
    ProjectUserListResponse,
    // Batch specific
    BatchErrorData,
    BatchErrors,
    BatchRequestCounts,
    Batch,
    ListBatchesResponse,
    BatchRequestInput,
    BatchRequestOutputResponse,
    BatchRequestOutputError,
    BatchRequestOutput,
  };
}