//! Structures related to the Uploads API for handling large file uploads in parts.

/// Define a private namespace for all its items.
mod private
{
  // Use full paths from crate root for components
  use crate::components::files::FileObject;
  // Serde imports
  use serde::{ Serialize, Deserialize }; // Added Serialize

  /// Represents an intermediate Upload object that you can add Parts to.
  /// Once completed, it results in a standard File object.
  ///
  /// # Used By
  /// - `/uploads` (POST response)
  /// - `/uploads/{upload_id}/cancel` (POST response)
  /// - `/uploads/{upload_id}/complete` (POST response)
  /// - `UploadPart`
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  #[ non_exhaustive ]
  pub struct Upload
  {
    /// The intended number of bytes to be uploaded.
    pub bytes : i64,
    /// The Unix timestamp (in seconds) for when the Upload was created.
    pub created_at : i64,
    /// The Unix timestamp (in seconds) for when the Upload will expire.
    pub expires_at : i64,
    /// The ready File object after the Upload is completed. Only present when status is `completed`.
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub file : Option< FileObject >,
    /// The name of the file to be uploaded.
    pub filename : String,
    /// The Upload unique identifier, which can be referenced in API endpoints.
    pub id : String,
    /// The object type, which is always "upload".
    pub object : String,
    /// The intended purpose of the file (e.g., `assistants`, `batch`, `fine-tune`).
    pub purpose : String,
    /// The status of the Upload (`pending`, `completed`, `cancelled`, `expired`).
    pub status : String,
  }

  /// Represents a chunk of bytes (a Part) added to an Upload object.
  ///
  /// # Used By
  /// - `/uploads/{upload_id}/parts` (POST response)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  #[ non_exhaustive ]
  pub struct UploadPart
  {
    /// The Unix timestamp (in seconds) for when the Part was created.
    pub created_at : i64,
    /// The upload Part unique identifier.
    pub id : String,
    /// The object type, which is always `upload.part`.
    pub object : String,
    /// The ID of the Upload object that this Part was added to.
    pub upload_id : String,
  }
} // end mod private

crate ::mod_interface!
{
  exposed use
  {
    Upload,
    UploadPart
  };
}