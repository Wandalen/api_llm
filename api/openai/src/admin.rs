//! Administrative APIs Module
//!
//! This module provides comprehensive administrative functionality for `OpenAI` organizations,
//! including user management, project management, and organizational operations.
//! Following the "Thin Client, Rich API" principle, this module offers direct access
//! to `OpenAI`'s administrative endpoints without automatic behaviors.

use mod_interface::mod_interface;

mod private
{
  use crate::{
    client ::Client,
    environment ::{ EnvironmentInterface, OpenaiEnvironment },
    error ::Result,
  };
  use serde::{ Deserialize, Serialize };

  /// Organization entity
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct Organization
  {
    /// Unique identifier for the organization
    pub id : String,
    /// Organization name
    pub name : String,
    /// Optional organization description
    pub description : Option< String >,
    /// Whether this is a personal organization
    pub personal : bool,
    /// Organization settings
    pub settings : OrganizationSettings,
    /// Unix timestamp when created
    pub created_at : u64,
    /// Unix timestamp when last updated
    pub updated_at : u64,
  }

  /// Organization settings
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct OrganizationSettings
  {
    /// Whether organization has legacy account
    pub has_legacy_account : bool,
    /// Maximum number of users allowed
    pub max_users : Option< u32 >,
    /// API usage limits
    pub usage_limits : Option< UsageLimits >,
  }

  /// Usage limits for organization
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct UsageLimits
  {
    /// Maximum monthly spend in cents
    pub max_monthly_spend : Option< u64 >,
    /// Maximum daily requests
    pub max_daily_requests : Option< u64 >,
  }

  /// User entity within organization
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct User
  {
    /// Object type identifier
    pub object : String,
    /// Unique user identifier
    pub id : String,
    /// User display name
    pub name : String,
    /// User email address
    pub email : String,
    /// User role in organization
    pub role : UserRole,
    /// Unix timestamp when user was added
    pub added_at : u64,
  }

  /// User roles within organization
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub enum UserRole
  {
    /// Organization owner (full permissions)
    #[ serde( rename = "owner" ) ]
    Owner,
    /// Organization admin (manage users and projects)
    #[ serde( rename = "admin" ) ]
    Admin,
    /// Organization member (create and manage own projects)
    #[ serde( rename = "member" ) ]
    Member,
    /// Organization reader (read-only access)
    #[ serde( rename = "reader" ) ]
    Reader,
  }

  /// Project entity
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct Project
  {
    /// Unique project identifier
    pub id : String,
    /// Object type identifier
    pub object : String,
    /// Project name
    pub name : String,
    /// Unix timestamp when created
    pub created_at : u64,
    /// Unix timestamp when archived (if archived)
    pub archived_at : Option< u64 >,
    /// Current project status
    pub status : ProjectStatus,
  }

  /// Project status enumeration
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub enum ProjectStatus
  {
    /// Project is active
    #[ serde( rename = "active" ) ]
    Active,
    /// Project is archived
    #[ serde( rename = "archived" ) ]
    Archived,
  }

  /// Invite entity for user invitations
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct Invite
  {
    /// Object type identifier
    pub object : String,
    /// Unique invite identifier
    pub id : String,
    /// Email address of invitee
    pub email : String,
    /// Role to be assigned upon acceptance
    pub role : UserRole,
    /// Invite status
    pub status : InviteStatus,
    /// Unix timestamp when invite was sent
    pub invited_at : u64,
    /// Unix timestamp when invite expires
    pub expires_at : u64,
  }

  /// Invite status enumeration
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub enum InviteStatus
  {
    /// Invite is pending acceptance
    #[ serde( rename = "pending" ) ]
    Pending,
    /// Invite has been accepted
    #[ serde( rename = "accepted" ) ]
    Accepted,
    /// Invite has expired
    #[ serde( rename = "expired" ) ]
    Expired,
  }

  /// Request to update organization
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct OrganizationUpdate
  {
    /// New organization name
    pub name : Option< String >,
    /// New organization description
    pub description : Option< String >,
    /// Updated settings
    pub settings : Option< OrganizationSettings >,
  }

  /// Request to create a new project
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct CreateProjectRequest
  {
    /// Project name
    pub name : String,
  }

  /// Request to update project
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ProjectUpdate
  {
    /// New project name
    pub name : Option< String >,
  }

  /// Generic delete response
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct DeleteResponse
  {
    /// ID of deleted entity
    pub id : String,
    /// Object type identifier
    pub object : String,
    /// Whether deletion was successful
    pub deleted : bool,
  }

  /// List response wrapper
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ListResponse< T >
  {
    /// Object type identifier
    pub object : String,
    /// List of items
    pub data : Vec< T >,
    /// Whether there are more items available
    pub has_more : bool,
    /// First item ID for pagination
    pub first_id : Option< String >,
    /// Last item ID for pagination
    pub last_id : Option< String >,
  }

  /// Administrative API client
  #[ derive( Debug ) ]
  pub struct Admin< 'client, E >
  where
    E: OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client Client< E >,
  }

  impl< 'client, E > Admin< 'client, E >
  where
    E: OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Create new Admin instance
    #[ inline ]
    pub fn new( client : &'client Client< E > ) -> Self
    {
      Self { client }
    }

    // ================================
    // Organizations API
    // ================================

    /// List all organizations accessible to the authenticated user
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or if the response cannot be parsed.
    #[ inline ]
    pub async fn list_organizations( &self ) -> Result< Vec< Organization > >
    {
      let response : ListResponse< Organization > = self.client.get( "organizations" ).await?;
      Ok( response.data )
    }

    /// Retrieve details of a specific organization
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, the organization is not found,
    /// or if the response cannot be parsed.
    #[ inline ]
    pub async fn get_organization( &self, org_id : &str ) -> Result< Organization >
    {
      let path = format!( "/organizations/{org_id}" );
      let organization : Organization = self.client.get( &path ).await?;
      Ok( organization )
    }

    /// Update organization details
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, the organization is not found,
    /// insufficient permissions, or if the response cannot be parsed.
    #[ inline ]
    pub async fn update_organization(
      &self,
      org_id : &str,
      update : OrganizationUpdate
    ) -> Result< Organization >
    {
      let path = format!( "/organizations/{org_id}" );
      let organization : Organization = self.client.post( &path, &update ).await?;
      Ok( organization )
    }

    /// Delete an organization
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, the organization is not found,
    /// insufficient permissions, or if the response cannot be parsed.
    #[ inline ]
    pub async fn delete_organization( &self, org_id : &str ) -> Result< DeleteResponse >
    {
      let path = format!( "/organizations/{org_id}" );
      let response : DeleteResponse = self.client.delete( &path ).await?;
      Ok( response )
    }

    // ================================
    // Users API
    // ================================

    /// List all users in an organization
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, the organization is not found,
    /// insufficient permissions, or if the response cannot be parsed.
    #[ inline ]
    pub async fn list_users( &self, org_id : &str ) -> Result< Vec< User > >
    {
      let path = format!( "/organizations/{org_id}/users" );
      let response : ListResponse< User > = self.client.get( &path ).await?;
      Ok( response.data )
    }

    /// Retrieve details of a specific user
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, the user is not found,
    /// insufficient permissions, or if the response cannot be parsed.
    #[ inline ]
    pub async fn get_user( &self, user_id : &str ) -> Result< User >
    {
      let path = format!( "/organization/users/{user_id}" );
      let user : User = self.client.get( &path ).await?;
      Ok( user )
    }

    /// Update user role in organization
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, the user is not found,
    /// insufficient permissions, invalid role, or if the response cannot be parsed.
    #[ inline ]
    pub async fn update_user( &self, user_id : &str, role : UserRole ) -> Result< User >
    {
      let path = format!( "/organization/users/{user_id}" );
      let update_data = serde_json::json!( { "role": role } );
      let user : User = self.client.post( &path, &update_data ).await?;
      Ok( user )
    }

    /// Remove user from organization
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, the user is not found,
    /// insufficient permissions, or if the response cannot be parsed.
    #[ inline ]
    pub async fn delete_user( &self, user_id : &str ) -> Result< DeleteResponse >
    {
      let path = format!( "/organization/users/{user_id}" );
      let response : DeleteResponse = self.client.delete( &path ).await?;
      Ok( response )
    }

    // ================================
    // Projects API
    // ================================

    /// List projects (optionally filtered by organization)
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, the organization is not found (when specified),
    /// insufficient permissions, or if the response cannot be parsed.
    #[ inline ]
    pub async fn list_projects( &self, org_id : Option< &str > ) -> Result< Vec< Project > >
    {
      let path = if let Some( org_id ) = org_id
      {
        format!( "/organizations/{org_id}/projects" )
      }
      else
      {
        "/organization/projects".to_string()
      };

      let response : ListResponse< Project > = self.client.get( &path ).await?;
      Ok( response.data )
    }

    /// Create a new project
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, invalid project name,
    /// insufficient permissions, or if the response cannot be parsed.
    #[ inline ]
    pub async fn create_project( &self, request : CreateProjectRequest ) -> Result< Project >
    {
      let project : Project = self.client.post( "organization/projects", &request ).await?;
      Ok( project )
    }

    /// Retrieve details of a specific project
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, the project is not found,
    /// insufficient permissions, or if the response cannot be parsed.
    #[ inline ]
    pub async fn get_project( &self, project_id : &str ) -> Result< Project >
    {
      let path = format!( "/organization/projects/{project_id}" );
      let project : Project = self.client.get( &path ).await?;
      Ok( project )
    }

    /// Update project details
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, the project is not found,
    /// invalid update parameters, insufficient permissions, or if the response cannot be parsed.
    #[ inline ]
    pub async fn update_project(
      &self,
      project_id : &str,
      update : ProjectUpdate
    ) -> Result< Project >
    {
      let path = format!( "/organization/projects/{project_id}" );
      let project : Project = self.client.post( &path, &update ).await?;
      Ok( project )
    }

    /// Archive a project (soft delete)
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, the project is not found,
    /// project is already archived, insufficient permissions, or if the response cannot be parsed.
    #[ inline ]
    pub async fn archive_project( &self, project_id : &str ) -> Result< Project >
    {
      let path = format!( "/organization/projects/{project_id}/archive" );
      let project : Project = self.client.post( &path, &serde_json::json!( {} ) ).await?;
      Ok( project )
    }

    // ================================
    // Invites API
    // ================================

    /// Send an invitation to join the organization
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, invalid email address,
    /// invalid role, insufficient permissions, or if the response cannot be parsed.
    #[ inline ]
    pub async fn send_invite( &self, email : &str, role : UserRole ) -> Result< Invite >
    {
      let invite_data = serde_json::json!({
        "email": email,
        "role": role
      });

      let invite : Invite = self.client.post( "organization/invites", &invite_data ).await?;
      Ok( invite )
    }

    /// List all pending invites for the organization
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, insufficient permissions,
    /// or if the response cannot be parsed.
    #[ inline ]
    pub async fn list_invites( &self ) -> Result< Vec< Invite > >
    {
      let response : ListResponse< Invite > = self.client.get( "organization/invites" ).await?;
      Ok( response.data )
    }

    /// Cancel/delete an invitation
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, the invite is not found,
    /// invite has already expired, insufficient permissions, or if the response cannot be parsed.
    #[ inline ]
    pub async fn delete_invite( &self, invite_id : &str ) -> Result< DeleteResponse >
    {
      let path = format!( "/organization/invites/{invite_id}" );
      let response : DeleteResponse = self.client.delete( &path ).await?;
      Ok( response )
    }

  }

  // ================================
  // Utility Functions
  // ================================

  /// Check if a user has sufficient permissions for an operation
  #[ must_use ]
  #[ inline ]
  pub fn validate_permission( user_role : &UserRole, required_role : &UserRole ) -> bool
  {
    use UserRole::{Owner, Admin, Member, Reader};
    matches!(
      ( user_role, required_role ),
      ( Owner, _ ) |                         // Owner can do everything
      ( Admin, Admin | Member | Reader ) |   // Admin can manage admin, member, reader
      ( Member, Member | Reader ) |          // Member can manage member, reader
      ( Reader, Reader )                     // Reader can only manage reader
    )
  }

  /// Get role hierarchy level for comparison
  #[ must_use ]
  #[ inline ]
  pub fn role_level( role : &UserRole ) -> u8
  {
    match role
    {
      UserRole::Owner => 4,
      UserRole::Admin => 3,
      UserRole::Member => 2,
      UserRole::Reader => 1,
    }
  }

  // ================================
  // Tests
  // ================================

  #[ cfg( test ) ]
  mod tests
  {
    use super::*;

    #[ test ]
    fn test_organization_serialization()
    {
      let org = Organization
      {
        id : "org-123".to_string(),
        name : "Test Org".to_string(),
        description : Some( "A test organization".to_string() ),
        personal : false,
        settings : OrganizationSettings
        {
          has_legacy_account : false,
          max_users : Some( 10 ),
          usage_limits : Some( UsageLimits
          {
            max_monthly_spend : Some( 10000 ),
            max_daily_requests : Some( 1000 ),
          } ),
        },
        created_at : 1_234_567_890,
        updated_at : 1_234_567_890,
      };

      let json = serde_json::to_string( &org ).unwrap();
      let deserialized : Organization = serde_json::from_str( &json ).unwrap();
      assert_eq!( org, deserialized );
    }

    #[ test ]
    fn test_user_role_serialization()
    {
      let role = UserRole::Admin;
      let json = serde_json::to_string( &role ).unwrap();
      assert_eq!( json, "\"admin\"" );

      let deserialized : UserRole = serde_json::from_str( &json ).unwrap();
      assert_eq!( role, deserialized );
    }

    #[ test ]
    fn test_project_status_serialization()
    {
      let status = ProjectStatus::Active;
      let json = serde_json::to_string( &status ).unwrap();
      assert_eq!( json, "\"active\"" );

      let deserialized : ProjectStatus = serde_json::from_str( &json ).unwrap();
      assert_eq!( status, deserialized );
    }

    #[ test ]
    fn test_permission_validation()
    {
      // Owner can do everything
      assert!( validate_permission( &UserRole::Owner, &UserRole::Reader ) );
      assert!( validate_permission( &UserRole::Owner, &UserRole::Admin ) );

      // Admin can manage members and readers
      assert!( validate_permission( &UserRole::Admin, &UserRole::Member ) );
      assert!( validate_permission( &UserRole::Admin, &UserRole::Reader ) );
      assert!( !validate_permission( &UserRole::Admin, &UserRole::Owner ) );

      // Member can only access member-level resources
      assert!( validate_permission( &UserRole::Member, &UserRole::Reader ) );
      assert!( !validate_permission( &UserRole::Member, &UserRole::Admin ) );

      // Reader has minimal permissions
      assert!( validate_permission( &UserRole::Reader, &UserRole::Reader ) );
      assert!( !validate_permission( &UserRole::Reader, &UserRole::Member ) );
    }

    #[ test ]
    fn test_role_hierarchy()
    {
      assert_eq!( role_level( &UserRole::Owner ), 4 );
      assert_eq!( role_level( &UserRole::Admin ), 3 );
      assert_eq!( role_level( &UserRole::Member ), 2 );
      assert_eq!( role_level( &UserRole::Reader ), 1 );

      // Verify hierarchy ordering
      assert!( role_level( &UserRole::Owner ) > role_level( &UserRole::Admin ) );
      assert!( role_level( &UserRole::Admin ) > role_level( &UserRole::Member ) );
      assert!( role_level( &UserRole::Member ) > role_level( &UserRole::Reader ) );
    }

    #[ test ]
    fn test_delete_response_serialization()
    {
      let response = DeleteResponse
      {
        id : "org-123".to_string(),
        object : "organization".to_string(),
        deleted : true,
      };

      let json = serde_json::to_string( &response ).unwrap();
      let deserialized : DeleteResponse = serde_json::from_str( &json ).unwrap();
      assert_eq!( response, deserialized );
    }

    #[ test ]
    fn test_invite_status_serialization()
    {
      let status = InviteStatus::Pending;
      let json = serde_json::to_string( &status ).unwrap();
      assert_eq!( json, "\"pending\"" );

      let deserialized : InviteStatus = serde_json::from_str( &json ).unwrap();
      assert_eq!( status, deserialized );
    }
  }
}

mod_interface!
{
  orphan use private::
  {
    Organization,
    OrganizationSettings,
    UsageLimits,
    User,
    UserRole,
    Project,
    ProjectStatus,
    Invite,
    InviteStatus,
    OrganizationUpdate,
    CreateProjectRequest,
    ProjectUpdate,
    DeleteResponse,
    ListResponse,
    Admin,
    validate_permission,
    role_level,
  };
}

use crate::
{
  client ::Client,
  environment ::{ EnvironmentInterface, OpenaiEnvironment },
  error ::Result,
};

impl< E > Client< E >
where
  E: OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
{
  /// Access the administrative API
  #[ inline ]
  pub fn admin( &self ) -> Admin< '_, E >
  {
    Admin::new( self )
  }

  /// List organizations (convenience method)
  ///
  /// # Errors
  ///
  /// Returns an error if the API request fails or if the response cannot be parsed.
  #[ inline ]
  pub async fn list_organizations( &self ) -> Result< Vec< Organization > >
  {
    self.admin().list_organizations().await
  }

  /// Get organization details (convenience method)
  ///
  /// # Errors
  ///
  /// Returns an error if the API request fails, the organization is not found,
  /// or if the response cannot be parsed.
  #[ inline ]
  pub async fn get_organization( &self, org_id : &str ) -> Result< Organization >
  {
    self.admin().get_organization( org_id ).await
  }

  /// List users in organization (convenience method)
  ///
  /// # Errors
  ///
  /// Returns an error if the API request fails, the organization is not found,
  /// insufficient permissions, or if the response cannot be parsed.
  #[ inline ]
  pub async fn list_users( &self, org_id : &str ) -> Result< Vec< User > >
  {
    self.admin().list_users( org_id ).await
  }

  /// List projects (convenience method)
  ///
  /// # Errors
  ///
  /// Returns an error if the API request fails, the organization is not found (when specified),
  /// insufficient permissions, or if the response cannot be parsed.
  #[ inline ]
  pub async fn list_projects( &self, org_id : Option< &str > ) -> Result< Vec< Project > >
  {
    self.admin().list_projects( org_id ).await
  }

  /// Create project (convenience method)
  ///
  /// # Errors
  ///
  /// Returns an error if the API request fails, invalid project name,
  /// insufficient permissions, or if the response cannot be parsed.
  #[ inline ]
  pub async fn create_project( &self, name : String ) -> Result< Project >
  {
    let request = CreateProjectRequest { name };
    self.admin().create_project( request ).await
  }
}