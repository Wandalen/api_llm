//! Structures related to Audit Logs API endpoints.

/// Define a private namespace for all its items.
mod private
{
  // Use full paths from crate root for components
  // No sibling imports needed here currently
  // Serde imports
  use serde::Deserialize;

  /// Represents the user associated with an audit log event actor.
  ///
  /// # Used By
  /// - `AuditLogActorSession`
  /// - `AuditLogActorApiKey`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogActorUser
  {
    /// The user ID.
    pub id : String,
    /// The user email.
    pub email : String,
  }

  /// Represents details about the IP address associated with an audit log event.
  ///
  /// # Used By
  /// - `AuditLogActorSession`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct IpAddressDetails
  {
    /// ISO country code.
    pub country : Option< String >,
    /// City name.
    pub city : Option< String >,
    /// Region name.
    pub region : Option< String >,
    /// Region code.
    pub region_code : Option< String >,
    /// Autonomous System Number (ASN).
    pub asn : Option< String >,
    /// Latitude coordinate.
    pub latitude : Option< String >,
    /// Longitude coordinate.
    pub longitude : Option< String >,
  }

  /// Represents the session context for an audit log event actor.
  ///
  /// # Used By
  /// - `AuditLogActor`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogActorSession
  {
    /// The user associated with the session.
    pub user : AuditLogActorUser,
    /// The IP address from which the action was performed.
    pub ip_address : String,
    /// The user agent string of the client.
    pub user_agent : Option< String >,
    /// JA3 TLS fingerprint.
    pub ja3 : Option< String >,
    /// JA4 TLS fingerprint.
    pub ja4 : Option< String >,
    /// Details derived from the IP address.
    pub ip_address_details : Option< IpAddressDetails >,
  }

  /// Represents a service account associated with an audit log event actor (via API key).
  ///
  /// # Used By
  /// - `AuditLogActorApiKey`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogActorServiceAccount
  {
    /// The service account ID.
    pub id : String,
  }

  /// Represents the API key used by an audit log event actor.
  ///
  /// # Used By
  /// - `AuditLogActor`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogActorApiKey
  {
    /// The tracking ID of the API key.
    pub id : String,
    /// The type of API key ("user" or "`service_account`").
    pub r#type : String,
    /// Details if the key belongs to a user.
    pub user : Option< AuditLogActorUser >,
    /// Details if the key belongs to a service account.
    pub service_account : Option< AuditLogActorServiceAccount >,
  }

  /// Represents the actor who performed the audit logged action.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogActor
  {
    /// The type of actor ("session" or "`api_key`").
    pub r#type : String,
    /// Session details if the actor type is "session".
    pub session : Option< AuditLogActorSession >,
    /// API key details if the actor type is "`api_key`".
    pub api_key : Option< AuditLogActorApiKey >,
  }

  /// Represents the project context for an audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogProject
  {
    /// The project ID.
    pub id : String,
    /// The project title.
    pub name : String,
  }

  /// Data specific to an `api_key.created` event.
  ///
  /// # Used By
  /// - `AuditLog` (as `api_key_created`)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogApiKeyCreatedData
  {
    /// A list of scopes allowed for the API key.
    pub scopes : Option< Vec< String > >,
  }

  /// Details for an `api_key.created` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogApiKeyCreated
  {
    /// The tracking ID of the created API key.
    pub id : String,
    /// The payload used to create the API key.
    pub data : AuditLogApiKeyCreatedData,
  }

  /// Changes requested during an `api_key.updated` event.
  ///
  /// # Used By
  /// - `AuditLogApiKeyUpdated`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogApiKeyUpdatedChanges
  {
    /// The updated list of scopes allowed for the API key.
    pub scopes : Option< Vec< String > >,
  }

  /// Details for an `api_key.updated` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogApiKeyUpdated
  {
    /// The tracking ID of the updated API key.
    pub id : String,
    /// The payload containing the requested changes.
    pub changes_requested : AuditLogApiKeyUpdatedChanges,
  }

  /// Details for an `api_key.deleted` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogApiKeyDeleted
  {
    /// The tracking ID of the deleted API key.
    pub id : String,
  }

  /// Data specific to an `invite.sent` event.
  ///
  /// # Used By
  /// - `AuditLogInviteSent`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogInviteSentData
  {
    /// The email invited to the organization.
    pub email : String,
    /// The role the email was invited to be ("owner" or "member").
    pub role : String,
  }

  /// Details for an `invite.sent` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogInviteSent
  {
    /// The ID of the invite.
    pub id : String,
    /// The payload used to create the invite.
    pub data : AuditLogInviteSentData,
  }

  /// Details for an `invite.accepted` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogInviteAccepted
  {
    /// The ID of the accepted invite.
    pub id : String,
  }

  /// Details for an `invite.deleted` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogInviteDeleted
  {
    /// The ID of the deleted invite.
    pub id : String,
  }

  /// Details for a `login.failed` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogLoginFailed
  {
    /// The error code of the failure.
    pub error_code : String,
    /// The error message of the failure.
    pub error_message : String,
  }

  /// Details for a `logout.failed` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogLogoutFailed
  {
    /// The error code of the failure.
    pub error_code : String,
    /// The error message of the failure.
    pub error_message : String,
  }

  /// Settings changed during an `organization.updated` event.
  ///
  /// # Used By
  /// - `AuditLogOrganizationUpdatedChanges`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogOrganizationUpdatedSettings
  {
    /// Visibility of the threads page ("`ANY_ROLE`", "OWNERS", or "NONE").
    pub threads_ui_visibility : Option< String >,
    /// Visibility of the usage dashboard ("`ANY_ROLE`" or "OWNERS").
    pub usage_dashboard_visibility : Option< String >,
  }

  /// Changes requested during an `organization.updated` event.
  ///
  /// # Used By
  /// - `AuditLogOrganizationUpdated`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogOrganizationUpdatedChanges
  {
    /// The updated organization title.
    pub title : Option< String >,
    /// The updated organization description.
    pub description : Option< String >,
    /// The updated organization name.
    pub name : Option< String >,
    /// The updated organization settings.
    pub settings : Option< AuditLogOrganizationUpdatedSettings >,
  }

  /// Details for an `organization.updated` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogOrganizationUpdated
  {
    /// The organization ID.
    pub id : String,
    /// The payload containing the requested changes.
    pub changes_requested : AuditLogOrganizationUpdatedChanges,
  }

  /// Data specific to a `project.created` event.
  ///
  /// # Used By
  /// - `AuditLogProjectCreated`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogProjectCreatedData
  {
    /// The project name.
    pub name : String,
    /// The title of the project as seen on the dashboard.
    pub title : String,
  }

  /// Details for a `project.created` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogProjectCreated
  {
    /// The project ID.
    pub id : String,
    /// The payload used to create the project.
    pub data : AuditLogProjectCreatedData,
  }

  /// Changes requested during a `project.updated` event.
  ///
  /// # Used By
  /// - `AuditLogProjectUpdated`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogProjectUpdatedChanges
  {
    /// The updated title of the project.
    pub title : String,
  }

  /// Details for a `project.updated` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogProjectUpdated
  {
    /// The project ID.
    pub id : String,
    /// The payload containing the requested changes.
    pub changes_requested : AuditLogProjectUpdatedChanges,
  }

  /// Details for a `project.archived` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogProjectArchived
  {
    /// The project ID.
    pub id : String,
  }

  /// Changes requested during a `rate_limit.updated` event.
  ///
  /// # Used By
  /// - `AuditLogRateLimitUpdated`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogRateLimitUpdatedChanges
  {
    /// The updated maximum requests per minute.
    pub max_requests_per_1_minute : Option< i32 >,
    /// The updated maximum tokens per minute.
    pub max_tokens_per_1_minute : Option< i32 >,
    /// The updated maximum images per minute.
    pub max_images_per_1_minute : Option< i32 >,
    /// The updated maximum audio megabytes per minute.
    pub max_audio_megabytes_per_1_minute : Option< i32 >,
    /// The updated maximum requests per day.
    pub max_requests_per_1_day : Option< i32 >,
    /// The updated maximum batch input tokens per day.
    pub batch_1_day_max_input_tokens : Option< i32 >,
  }

  /// Details for a `rate_limit.updated` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogRateLimitUpdated
  {
    /// The rate limit ID.
    pub id : String,
    /// The payload containing the requested changes.
    pub changes_requested : AuditLogRateLimitUpdatedChanges,
  }

  /// Details for a `rate_limit.deleted` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogRateLimitDeleted
  {
    /// The rate limit ID.
    pub id : String,
  }

  /// Data specific to a `service_account.created` event.
  ///
  /// # Used By
  /// - `AuditLogServiceAccountCreated`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogServiceAccountCreatedData
  {
    /// The role of the service account ("owner" or "member").
    pub role : String,
  }

  /// Details for a `service_account.created` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogServiceAccountCreated
  {
    /// The service account ID.
    pub id : String,
    /// The payload used to create the service account.
    pub data : AuditLogServiceAccountCreatedData,
  }

  /// Changes requested during a `service_account.updated` event.
  ///
  /// # Used By
  /// - `AuditLogServiceAccountUpdated`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogServiceAccountUpdatedChanges
  {
    /// The updated role of the service account ("owner" or "member").
    pub role : String,
  }

  /// Details for a `service_account.updated` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogServiceAccountUpdated
  {
    /// The service account ID.
    pub id : String,
    /// The payload containing the requested changes.
    pub changes_requested : AuditLogServiceAccountUpdatedChanges,
  }

  /// Details for a `service_account.deleted` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogServiceAccountDeleted
  {
    /// The service account ID.
    pub id : String,
  }

  /// Data specific to a `user.added` event (user added to a project).
  ///
  /// # Used By
  /// - `AuditLogUserAdded`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogUserAddedData
  {
    /// The role assigned to the user in the project ("owner" or "member").
    pub role : String,
  }

  /// Details for a `user.added` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogUserAdded
  {
    /// The user ID.
    pub id : String,
    /// The payload used to add the user to the project.
    pub data : AuditLogUserAddedData,
  }

  /// Changes requested during a `user.updated` event (user role changed in a project).
  ///
  /// # Used By
  /// - `AuditLogUserUpdated`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogUserUpdatedChanges
  {
    /// The updated role of the user ("owner" or "member").
    pub role : String,
  }

  /// Details for a `user.updated` audit log event.
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogUserUpdated
  {
    /// The project ID where the user role was updated.
    pub id : String,
    /// The payload containing the requested changes.
    pub changes_requested : AuditLogUserUpdatedChanges,
  }

  /// Details for a `user.deleted` audit log event (user removed from a project).
  ///
  /// # Used By
  /// - `AuditLog`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogUserDeleted
  {
    /// The user ID.
    pub id : String,
  }

  /// Represents a single audit log entry.
  ///
  /// # Used By
  /// - `ListAuditLogsResponse`
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLog
  {
    /// The ID of this log entry.
    pub id : String,
    /// The type of event that occurred.
    pub r#type : String, // Enum : AuditLogEventType
    /// The Unix timestamp (in seconds) of the event.
    pub effective_at : i64,
    /// The project that the action was scoped to. Absent for actions not scoped to projects.
    pub project : Option< AuditLogProject >,
    /// The actor who performed the action.
    pub actor : AuditLogActor,
    /// Details specific to an `api_key.created` event.
    #[ serde( rename = "api_key.created", skip_serializing_if = "Option::is_none" ) ]
    pub api_key_created : Option< AuditLogApiKeyCreated >,
    /// Details specific to an `api_key.updated` event.
    #[ serde( rename = "api_key.updated", skip_serializing_if = "Option::is_none" ) ]
    pub api_key_updated : Option< AuditLogApiKeyUpdated >,
    /// Details specific to an `api_key.deleted` event.
    #[ serde( rename = "api_key.deleted", skip_serializing_if = "Option::is_none" ) ]
    pub api_key_deleted : Option< AuditLogApiKeyDeleted >,
    /// Details specific to an `invite.sent` event.
    #[ serde( rename = "invite.sent", skip_serializing_if = "Option::is_none" ) ]
    pub invite_sent : Option< AuditLogInviteSent >,
    /// Details specific to an `invite.accepted` event.
    #[ serde( rename = "invite.accepted", skip_serializing_if = "Option::is_none" ) ]
    pub invite_accepted : Option< AuditLogInviteAccepted >,
    /// Details specific to an `invite.deleted` event.
    #[ serde( rename = "invite.deleted", skip_serializing_if = "Option::is_none" ) ]
    pub invite_deleted : Option< AuditLogInviteDeleted >,
    /// Details specific to a `login.failed` event.
    #[ serde( rename = "login.failed", skip_serializing_if = "Option::is_none" ) ]
    pub login_failed : Option< AuditLogLoginFailed >,
    /// Details specific to a `logout.failed` event.
    #[ serde( rename = "logout.failed", skip_serializing_if = "Option::is_none" ) ]
    pub logout_failed : Option< AuditLogLogoutFailed >,
    /// Details specific to an `organization.updated` event.
    #[ serde( rename = "organization.updated", skip_serializing_if = "Option::is_none" ) ]
    pub organization_updated : Option< AuditLogOrganizationUpdated >,
    /// Details specific to a `project.created` event.
    #[ serde( rename = "project.created", skip_serializing_if = "Option::is_none" ) ]
    pub project_created : Option< AuditLogProjectCreated >,
    /// Details specific to a `project.updated` event.
    #[ serde( rename = "project.updated", skip_serializing_if = "Option::is_none" ) ]
    pub project_updated : Option< AuditLogProjectUpdated >,
    /// Details specific to a `project.archived` event.
    #[ serde( rename = "project.archived", skip_serializing_if = "Option::is_none" ) ]
    pub project_archived : Option< AuditLogProjectArchived >,
    /// Details specific to a `rate_limit.updated` event.
    #[ serde( rename = "rate_limit.updated", skip_serializing_if = "Option::is_none" ) ]
    pub rate_limit_updated : Option< AuditLogRateLimitUpdated >,
    /// Details specific to a `rate_limit.deleted` event.
    #[ serde( rename = "rate_limit.deleted", skip_serializing_if = "Option::is_none" ) ]
    pub rate_limit_deleted : Option< AuditLogRateLimitDeleted >,
    /// Details specific to a `service_account.created` event.
    #[ serde( rename = "service_account.created", skip_serializing_if = "Option::is_none" ) ]
    pub service_account_created : Option< AuditLogServiceAccountCreated >,
    /// Details specific to a `service_account.updated` event.
    #[ serde( rename = "service_account.updated", skip_serializing_if = "Option::is_none" ) ]
    pub service_account_updated : Option< AuditLogServiceAccountUpdated >,
    /// Details specific to a `service_account.deleted` event.
    #[ serde( rename = "service_account.deleted", skip_serializing_if = "Option::is_none" ) ]
    pub service_account_deleted : Option< AuditLogServiceAccountDeleted >,
    /// Details specific to a `user.added` event.
    #[ serde( rename = "user.added", skip_serializing_if = "Option::is_none" ) ]
    pub user_added : Option< AuditLogUserAdded >,
    /// Details specific to a `user.updated` event.
    #[ serde( rename = "user.updated", skip_serializing_if = "Option::is_none" ) ]
    pub user_updated : Option< AuditLogUserUpdated >,
    /// Details specific to a `user.deleted` event.
    #[ serde( rename = "user.deleted", skip_serializing_if = "Option::is_none" ) ]
    pub user_deleted : Option< AuditLogUserDeleted >,
  }

  /// Response containing a list of audit logs.
  ///
  /// # Used By
  /// - `/organization/audit_logs` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ListAuditLogsResponse
  {
    /// The object type, always "list".
    pub object : String,
    /// A list of audit log objects.
    pub data : Vec< AuditLog >,
    /// The ID of the first log in the list.
    pub first_id : Option< String >,
    /// The ID of the last log in the list.
    pub last_id : Option< String >,
    /// Indicates whether there are more logs available.
    pub has_more : bool,
  }

  /// Represents the type of an audit log event.
  ///
  /// # Used By
  /// - `AuditLog` (as `r#type`)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct AuditLogEventType
  {
    /// The event type string (e.g., "project.created", "`api_key.updated`").
    pub value : String,
  }
} // end mod private

crate ::mod_interface!
{
  exposed use
  {
    AuditLogActorUser,
    IpAddressDetails,
    AuditLogActorSession,
    AuditLogActorServiceAccount,
    AuditLogActorApiKey,
    AuditLogActor,
    AuditLogProject,
    AuditLogApiKeyCreatedData,
    AuditLogApiKeyCreated,
    AuditLogApiKeyUpdatedChanges,
    AuditLogApiKeyUpdated,
    AuditLogApiKeyDeleted,
    AuditLogInviteSentData,
    AuditLogInviteSent,
    AuditLogInviteAccepted,
    AuditLogInviteDeleted,
    AuditLogLoginFailed,
    AuditLogLogoutFailed,
    AuditLogOrganizationUpdatedSettings,
    AuditLogOrganizationUpdatedChanges,
    AuditLogOrganizationUpdated,
    AuditLogProjectCreatedData,
    AuditLogProjectCreated,
    AuditLogProjectUpdatedChanges,
    AuditLogProjectUpdated,
    AuditLogProjectArchived,
    AuditLogRateLimitUpdatedChanges,
    AuditLogRateLimitUpdated,
    AuditLogRateLimitDeleted,
    AuditLogServiceAccountCreatedData,
    AuditLogServiceAccountCreated,
    AuditLogServiceAccountUpdatedChanges,
    AuditLogServiceAccountUpdated,
    AuditLogServiceAccountDeleted,
    AuditLogUserAddedData,
    AuditLogUserAdded,
    AuditLogUserUpdatedChanges,
    AuditLogUserUpdated,
    AuditLogUserDeleted,
    AuditLog,
    ListAuditLogsResponse,
    AuditLogEventType
  };
}