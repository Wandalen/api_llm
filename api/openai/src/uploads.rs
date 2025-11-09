//! Uploads Module
//!
//! This module provides comprehensive file upload and management functionality for the `OpenAI` API.
//! Following the "Thin Client, Rich API" principle, this module offers file operations
//! patterns and upload tools without automatic behaviors or persistent state management.

use mod_interface::mod_interface;

mod private
{
  use crate::
  {
    client ::Client,
    environment ::{ EnvironmentInterface, OpenaiEnvironment },
    error ::{ OpenAIError, Result },
  };
  use std::
  {
    path ::Path,
    fs ::File,
    io ::Read,
  };
  use serde::{ Deserialize, Serialize };
  use reqwest::multipart::{ Form, Part };

  /// File object returned by the `OpenAI` Files API
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct FileObject
  {
    /// Unique identifier for the file
    pub id : String,
    /// Object type, always "file"
    pub object : String,
    /// Size of the file in bytes
    pub bytes : u64,
    /// Unix timestamp when the file was created
    pub created_at : u64,
    /// Name of the file
    pub filename : String,
    /// Purpose of the file (e.g., "fine-tune", "assistants")
    pub purpose : String,
    /// Current status of the file
    pub status : FileStatus,
    /// Additional details about the file status
    pub status_details : Option< String >,
  }

  /// Status of a file in the `OpenAI` system
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub enum FileStatus
  {
    /// File has been uploaded successfully
    #[ serde( rename = "uploaded" ) ]
    Uploaded,
    /// File has been processed and is ready for use
    #[ serde( rename = "processed" ) ]
    Processed,
    /// File processing encountered an error
    #[ serde( rename = "error" ) ]
    Error,
  }

  /// Response when deleting a file
  #[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
  pub struct DeleteFileResponse
  {
    /// ID of the deleted file
    pub id : String,
    /// Object type, always "file"
    pub object : String,
    /// Whether the file was successfully deleted
    pub deleted : bool,
  }

  /// List files response
  #[ derive( Debug, Clone, Serialize, Deserialize ) ]
  pub struct ListFilesResponse
  {
    /// Object type, always "list"
    pub object : String,
    /// List of file objects
    pub data : Vec< FileObject >,
    /// Whether there are more files available
    pub has_more : bool,
  }

  /// File upload configuration
  #[ derive( Debug, Clone ) ]
  pub struct UploadConfig
  {
    /// Maximum file size in bytes (default : 512MB)
    pub max_file_size : u64,
    /// Allowed file extensions
    pub allowed_extensions : Vec< String >,
    /// Whether to validate file content type
    pub validate_content_type : bool,
  }

  impl Default for UploadConfig
  {
    #[ inline ]
    fn default() -> Self
    {
      Self
      {
        max_file_size : 512 * 1024 * 1024, // 512MB
        allowed_extensions : vec![
          ".txt".to_string(),
          ".json".to_string(),
          ".jsonl".to_string(),
          ".csv".to_string(),
          ".pdf".to_string(),
          ".doc".to_string(),
          ".docx".to_string(),
          ".png".to_string(),
          ".jpg".to_string(),
          ".jpeg".to_string(),
          ".gif".to_string(),
          ".webp".to_string(),
        ],
        validate_content_type : true,
      }
    }
  }

  /// Uploads API implementation
  #[ derive( Debug ) ]
  pub struct Uploads< 'client, E >
  where
    E: OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    client : &'client Client< E >,
    config : UploadConfig,
  }

  impl< 'client, E > Uploads< 'client, E >
  where
    E: OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
  {
    /// Create a new Uploads instance
    #[ inline ]
    pub fn new( client : &'client Client< E > ) -> Self
    {
      Self
      {
        client,
        config : UploadConfig::default(),
      }
    }

    /// Create a new Uploads instance with custom configuration
    #[ inline ]
    pub fn with_config( client : &'client Client< E >, config : UploadConfig ) -> Self
    {
      Self
      {
        client,
        config,
      }
    }

    /// Upload a file to `OpenAI`
    ///
    /// # Errors
    ///
    /// Returns an error if the file does not exist, file size exceeds limits,
    /// API request fails, or if the response cannot be parsed.
    #[ inline ]
    pub async fn upload_file< P: AsRef< Path > >(
      &self,
      file_path : P,
      purpose : &str
    ) -> Result< FileObject >
    {
      let path = file_path.as_ref();

      // Validate file exists
      if !path.exists()
      {
        return Err( OpenAIError::Internal( format!( "File not found : {}", path.display() ) ).into() );
      }

      // Validate file size
      let metadata = std::fs::metadata( path )
        .map_err( | e | OpenAIError::Internal( format!( "Failed to read file metadata : {e}" ) ) )?;

      if metadata.len() > self.config.max_file_size
      {
        let file_size = metadata.len();
        let max_size = self.config.max_file_size;
        return Err( OpenAIError::Internal( format!(
          "File too large : {file_size} bytes (max : {max_size} bytes)"
        ) ).into() );
      }

      // Validate file extension
      if let Some( extension ) = path.extension()
      {
        let ext = format!( ".{}", extension.to_string_lossy().to_lowercase() );
        if !self.config.allowed_extensions.contains( &ext )
        {
          return Err( OpenAIError::Internal( format!(
            "Unsupported file extension : {ext}"
          ) ).into() );
        }
      }

      // Read file content
      let mut file = File::open( path )
        .map_err( | e | OpenAIError::Internal( format!( "Failed to open file : {e}" ) ) )?;

      let mut content = Vec::new();
      file.read_to_end( &mut content )
        .map_err( | e | OpenAIError::Internal( format!( "Failed to read file content : {e}" ) ) )?;

      // Create multipart form
      let filename = path.file_name()
        .and_then( | name | name.to_str() )
        .unwrap_or( "upload" )
        .to_string();

      let file_part = Part::bytes( content )
        .file_name( filename )
        .mime_str( "application/octet-stream" )
        .map_err( | e | OpenAIError::Internal( format!( "Failed to create file part : {e}" ) ) )?;

      let form = Form::new()
        .part( "file", file_part )
        .text( "purpose", purpose.to_string() );

      // Make request using client's multipart method
      let file_object : FileObject = self.client.post_multipart( "files", form ).await?;

      Ok( file_object )
    }

    /// List uploaded files
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails or if the response cannot be parsed.
    #[ inline ]
    pub async fn list_files( &self, purpose : Option< &str > ) -> Result< Vec< FileObject > >
    {
      let path = if let Some( purpose_filter ) = purpose
      {
        format!( "/files?purpose={purpose_filter}" )
      }
      else
      {
        "/files".to_string()
      };

      let list_response : ListFilesResponse = self.client.get( &path ).await?;

      Ok( list_response.data )
    }

    /// Retrieve information about a specific file
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, the file is not found,
    /// or if the response cannot be parsed.
    #[ inline ]
    pub async fn get_file( &self, file_id : &str ) -> Result< FileObject >
    {
      let path = format!( "/files/{file_id}" );
      let file_object : FileObject = self.client.get( &path ).await?;
      Ok( file_object )
    }

    /// Delete a file
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, the file is not found,
    /// or if the response cannot be parsed.
    #[ inline ]
    pub async fn delete_file( &self, file_id : &str ) -> Result< DeleteFileResponse >
    {
      let path = format!( "/files/{file_id}" );
      let delete_response : DeleteFileResponse = self.client.delete( &path ).await?;
      Ok( delete_response )
    }

    /// Download file content
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails, the file is not found,
    /// or if the response cannot be parsed.
    #[ inline ]
    pub async fn download_file( &self, file_id : &str ) -> Result< Vec< u8 > >
    {
      let path = format!( "/files/{file_id}/content" );
      self.client.get_bytes( &path ).await
    }
  }

  #[ cfg( test ) ]
  mod tests
  {
    use super::*;

    #[ test ]
    fn test_file_object_serialization()
    {
      let file_obj = FileObject
      {
        id : "file-123".to_string(),
        object : "file".to_string(),
        bytes : 1024,
        created_at : 1_234_567_890,
        filename : "test.txt".to_string(),
        purpose : "fine-tune".to_string(),
        status : FileStatus::Uploaded,
        status_details : None,
      };

      let json = serde_json::to_string( &file_obj ).unwrap();
      let deserialized : FileObject = serde_json::from_str( &json ).unwrap();

      assert_eq!( file_obj, deserialized );
    }

    #[ test ]
    fn test_file_status_serialization()
    {
      let status = FileStatus::Processed;
      let json = serde_json::to_string( &status ).unwrap();
      assert_eq!( json, "\"processed\"" );

      let deserialized : FileStatus = serde_json::from_str( &json ).unwrap();
      assert_eq!( status, deserialized );
    }

    #[ test ]
    fn test_delete_file_response_serialization()
    {
      let response = DeleteFileResponse
      {
        id : "file-123".to_string(),
        object : "file".to_string(),
        deleted : true,
      };

      let json = serde_json::to_string( &response ).unwrap();
      let deserialized : DeleteFileResponse = serde_json::from_str( &json ).unwrap();

      assert_eq!( response, deserialized );
    }

    #[ test ]
    fn test_upload_config_default()
    {
      let config = UploadConfig::default();

      assert_eq!( config.max_file_size, 512 * 1024 * 1024 );
      assert!( config.allowed_extensions.contains( &".txt".to_string() ) );
      assert!( config.allowed_extensions.contains( &".json".to_string() ) );
      assert!( config.validate_content_type );
    }
  }
}

mod_interface!
{
  orphan use private::
  {
    FileObject,
    FileStatus,
    DeleteFileResponse,
    ListFilesResponse,
    UploadConfig,
    Uploads,
  };
}

use crate::
{
  client ::Client,
  environment ::{ EnvironmentInterface, OpenaiEnvironment },
  error ::Result,
};
use std::path::Path;

impl< E > Client< E >
where
  E: OpenaiEnvironment + EnvironmentInterface + Send + Sync + 'static,
{
  /// Access the uploads API
  #[ inline ]
  pub fn uploads( &self ) -> Uploads< '_, E >
  {
    Uploads::new( self )
  }

  /// Upload a file with default configuration
  ///
  /// # Errors
  ///
  /// Returns `OpenAIError` if the file upload fails.
  #[ inline ]
  pub async fn upload_file< P: AsRef< Path > >(
    &self,
    file_path : P,
    purpose : &str
  ) -> Result< FileObject >
  {
    self.uploads().upload_file( file_path, purpose ).await
  }

  /// List uploaded files
  ///
  /// # Errors
  ///
  /// Returns `OpenAIError` if the request fails.
  #[ inline ]
  pub async fn list_files( &self, purpose : Option< &str > ) -> Result< Vec< FileObject > >
  {
    self.uploads().list_files( purpose ).await
  }

  /// Get file information
  ///
  /// # Errors
  ///
  /// Returns `OpenAIError` if the request fails.
  #[ inline ]
  pub async fn get_file( &self, file_id : &str ) -> Result< FileObject >
  {
    self.uploads().get_file( file_id ).await
  }

  /// Delete a file
  ///
  /// # Errors
  ///
  /// Returns `OpenAIError` if the request fails.
  #[ inline ]
  pub async fn delete_file( &self, file_id : &str ) -> Result< DeleteFileResponse >
  {
    self.uploads().delete_file( file_id ).await
  }

  /// Download file content
  ///
  /// # Errors
  ///
  /// Returns `OpenAIError` if the request fails.
  #[ inline ]
  pub async fn download_file( &self, file_id : &str ) -> Result< Vec< u8 > >
  {
    self.uploads().download_file( file_id ).await
  }
}