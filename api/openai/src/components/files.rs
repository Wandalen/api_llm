//! Structures related to file objects used across the API.

/// Define a private namespace for all its items.
mod private
{
  // Serde imports
  use serde::{ Serialize, Deserialize }; // Added Serialize

  /// Represents a document that has been uploaded to `OpenAI`.
  /// Files are used across several endpoints like Assistants, Fine-tuning, and Batch API.
  ///
  /// # Used By
  /// - `/files` (GET - in `ListFilesResponse`, POST response)
  /// - `/files/{file_id}` (GET)
  /// - `Upload` (within `uploads.rs` - as nested `file` field)
  /// - `DeleteFileResponse` (within `common.rs`)
  #[ derive( Debug, Serialize, Deserialize, Clone, PartialEq ) ] // Added Serialize
  pub struct FileObject
  {
    /// The file identifier, which can be referenced in the API endpoints.
    pub id : String,
    /// The size of the file, in bytes.
    pub bytes : i64,
    /// The Unix timestamp (in seconds) for when the file was created.
    pub created_at : i64,
    /// The Unix timestamp (in seconds) for when the file will expire (used for temporary files like uploads).
    #[ serde( skip_serializing_if = "Option::is_none" ) ]
    pub expires_at : Option< i64 >,
    /// The name of the file.
    pub filename : String,
    /// The object type, which is always `file`.
    pub object : String,
    /// The intended purpose of the file (e.g., `assistants`, `fine-tune`, `vision`).
    pub purpose : String,
    /// Deprecated. The current status of the file (`uploaded`, `processed`, or `error`).
    #[ deprecated( note = "File status is deprecated." ) ]
    #[ serde( skip_serializing_if = "Option::is_none" ) ] // Skip if None
    pub status : Option< String >,
    /// Deprecated. For details on fine-tuning validation errors, see the `error` field on `fine_tuning.job`.
    #[ deprecated( note = "File status details are deprecated." ) ]
    #[ serde( skip_serializing_if = "Option::is_none" ) ] // Skip if None
    pub status_details : Option< String >,
  }

  /// Response containing a list of files.
  ///
  /// # Used By
  /// - `/files` (GET)
  #[ derive( Debug, Deserialize, Clone, PartialEq ) ]
  pub struct ListFilesResponse
  {
    /// The object type, always "list".
    pub object : String,
    /// A list of file objects.
    pub data : Vec< FileObject >,
    /// The ID of the first file in the list, used for pagination.
    pub first_id : Option< String >,
    /// The ID of the last file in the list, used for pagination.
    pub last_id : Option< String >,
    /// Indicates whether there are more files available.
    pub has_more : bool,
  }

  /// Request parameters for uploading a file.
  ///
  /// # Used By
  /// - `/files` (POST request)
  #[ derive( Debug, Clone ) ]
  pub struct CreateFileRequest
  {
    /// The file data to upload.
    pub file : Vec< u8 >,
    /// The filename for the uploaded file.
    pub filename : String,
    /// The intended purpose of the file.
    /// Use "assistants" for Assistants and Message files,
    /// "vision" for Vision capabilities,
    /// "batch" for Batch API,
    /// or "fine-tune" for fine-tuning.
    pub purpose : String,
  }
} // end mod private

crate ::mod_interface!
{
  exposed use
  {
    FileObject,
    ListFilesResponse,
    CreateFileRequest,
  };
}
