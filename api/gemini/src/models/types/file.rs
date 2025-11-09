//! File management types for the Gemini API.

use serde::{ Deserialize, Serialize };

/// File metadata for uploads and management.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct FileMetadata
{
  /// The name of the file.
  pub name : String,

  /// The display name of the file.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub display_name : Option< String >,

  /// The MIME type of the file.
  pub mime_type : String,

  /// The size of the file in bytes.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub size_bytes : Option< i64 >,

  /// The creation time of the file.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub create_time : Option< String >,

  /// The last update time of the file.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub update_time : Option< String >,

  /// The expiration time of the file.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub expiration_time : Option< String >,

  /// The SHA256 hash of the file.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub sha256_hash : Option< String >,

  /// The URI of the file.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub uri : Option< String >,

  /// The state of the file processing.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub state : Option< String >,

  /// Error information if the file processing failed.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub error : Option< serde_json::Value >,

  /// Video metadata if the file is a video.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub video_metadata : Option< VideoMetadata >,
}

/// Video metadata for video files.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct VideoMetadata
{
  /// The duration of the video.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub video_duration : Option< String >,
}

/// Request for uploading a file.
#[ derive( Debug, Clone ) ]
pub struct UploadFileRequest
{
  /// The file data as bytes.
  pub file_data : Vec< u8 >,

  /// The MIME type of the file.
  pub mime_type : String,

  /// The display name for the file.
  pub display_name : Option< String >,
}

/// Response from uploading a file.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct UploadFileResponse
{
  /// The uploaded file metadata.
  pub file : FileMetadata,
}

/// Request for listing files.
#[ derive( Debug, Clone, Default ) ]
pub struct ListFilesRequest
{
  /// The maximum number of files to return.
  pub page_size : Option< i32 >,

  /// The page token for pagination.
  pub page_token : Option< String >,
}

/// Response from listing files.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
#[ serde( rename_all = "camelCase" ) ]
pub struct ListFilesResponse
{
  /// The list of files.
  pub files : Vec< FileMetadata >,

  /// The next page token for pagination.
  #[ serde( skip_serializing_if = "Option::is_none" ) ]
  pub next_page_token : Option< String >,
}

/// Request for deleting a file.
#[ derive( Debug, Clone ) ]
pub struct DeleteFileRequest
{
  /// The name of the file to delete.
  pub name : String,
}
