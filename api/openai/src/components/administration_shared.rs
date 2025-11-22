//! Structures shared across the Administration API endpoints (Users, Projects, Invites, API Keys, Rate Limits).

/// Define a private namespace for all its items.
mod private
{
  // Use full paths from crate root for components
  // Removed unused : use crate::components::common::Metadata;
  // Serde imports
  use serde::{ Serialize, Deserialize };

  /// Represents the owner of an Admin API Key (User or Service Account).
  ///
  /// # Used By
  /// - `AdminApiKey`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
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
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
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
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub value : Option< String >,
    /// Creation timestamp.
    pub created_at : i64,
    /// Details of the key owner.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub owner : Option< AdminApiKeyOwner >,
  }

  /// Response containing a list of Admin API Keys.
  ///
  /// # Used By
  /// - `/organization/admin_api_keys` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
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
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
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
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
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
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub accepted_at : Option< i64 >,
    /// Projects granted membership upon acceptance.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub projects : Option< Vec< InviteProject > >,
  }

  /// Response containing a list of organization invites.
  ///
  /// # Used By
  /// - `/organization/invites` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
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
  /// - `ProjectUser`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
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
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
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
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
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
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub archived_at : Option< i64 >,
    /// Project status ("active" or "archived").
    pub status : String,
  }

  /// Response containing a list of organization projects.
  ///
  /// # Used By
  /// - `/organization/projects` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
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
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
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
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ]
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
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
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
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
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
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
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
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub max_images_per_1_minute : Option< i32 >,
    /// Maximum audio megabytes per minute (relevant models only).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub max_audio_megabytes_per_1_minute : Option< i32 >,
    /// Maximum requests per day (relevant models only).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub max_requests_per_1_day : Option< i32 >,
    /// Maximum batch input tokens per day (relevant models only).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub batch_1_day_max_input_tokens : Option< i32 >,
  }

  /// Response containing a list of project rate limits.
  ///
  /// # Used By
  /// - `/organization/projects/{project_id}/rate_limits` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
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
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
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
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
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
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
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
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ] // Only Deserialize needed
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
  };
}