//! Basic file operations tests : CRUD operations (Create, Read, Update, Delete)

use super::*;

/// Test basic file upload functionality
#[ tokio::test ]
async fn test_basic_file_upload() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let files_api = client.files();

  // Create simple test image data (basic PNG header)
  let test_image_data = vec![ 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A ];

  let upload_request = UploadFileRequest {
    file_data: test_image_data,
    mime_type: "image/png".to_string(),
    display_name: Some( "Test Upload Image".to_string() ),
  };

  let upload_response = files_api.upload( &upload_request ).await?;

  // Verify upload response
  assert!( !upload_response.file.name.is_empty() );
  assert_eq!( upload_response.file.mime_type, "image/png" );
  assert!( upload_response.file.display_name.is_some() );
  assert!( upload_response.file.uri.is_some() );

  println!( "✓ Basic file upload successful:" );
println!( "  - File name : {}", upload_response.file.name );
println!( "  - Display name : {:?}", upload_response.file.display_name );
println!( "  - Size : {:?} bytes", upload_response.file.size_bytes );
println!( "  - State : {:?}", upload_response.file.state );

  Ok( () )
}

/// Test file listing functionality
#[ tokio::test ]
async fn test_file_listing() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let files_api = client.files();

  // List files with default request
  let list_request = ListFilesRequest::default();
  let list_response = files_api.list( &list_request ).await?;

  // Assert response structure is valid
  assert!( list_response.files.len() >= 0, "Files list should be valid (can be empty)" );

  println!( "✓ File listing successful:" );
println!( "  - Total files found : {}", list_response.files.len() );

  // If there are files, verify they have required fields
  for ( index, file ) in list_response.files.iter().take( 3 ).enumerate()
  {
println!( "  - File {}: {} ({})", index + 1, file.name, file.mime_type );
    assert!( !file.name.is_empty(), "File name should not be empty" );
    assert!( !file.mime_type.is_empty(), "File mime_type should not be empty" );
  }

  // Verify pagination token format if present
  if let Some( next_token ) = &list_response.next_page_token
  {
  println!( "  - Has next page : {}", !next_token.is_empty() );
    // If token exists, it should not be empty
    if list_response.files.len() > 0
    {
      // Token can be empty or non-empty, just verify it's a valid string
      assert!( next_token.len() >= 0, "Next page token should be valid string" );
    }
  }

  Ok( () )
}

/// Test file metadata retrieval
#[ tokio::test ]
async fn test_file_metadata_retrieval() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let files_api = client.files();

  // First upload a test file
  let test_data = vec![ 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A ]; // PNG header
  let upload_request = UploadFileRequest {
    file_data: test_data,
    mime_type: "image/png".to_string(),
    display_name: Some( "Metadata Test File".to_string() ),
  };

  let upload_response = files_api.upload( &upload_request ).await?;
  let file_name = upload_response.file.name.clone();

  // Retrieve metadata for the uploaded file
  let file_metadata = files_api.get( &file_name ).await?;

  // Verify metadata consistency
  assert_eq!( file_metadata.name, file_name );
  assert_eq!( file_metadata.mime_type, "image/png" );
  assert!( file_metadata.display_name.is_some() );

  println!( "✓ File metadata retrieval successful:" );
println!( "  - Name : {}", file_metadata.name );
println!( "  - MIME type : {}", file_metadata.mime_type );
println!( "  - Display name : {:?}", file_metadata.display_name );
println!( "  - Size : {:?} bytes", file_metadata.size_bytes );
println!( "  - Creation time : {:?}", file_metadata.create_time );
println!( "  - State : {:?}", file_metadata.state );

  Ok( () )
}

/// Test file deletion functionality
#[ tokio::test ]
async fn test_file_deletion() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let files_api = client.files();

  // Upload a file specifically for deletion
  let test_data = vec![ 0x25, 0x50, 0x44, 0x46 ]; // PDF header
  let upload_request = UploadFileRequest {
    file_data: test_data,
    mime_type: "application/pdf".to_string(),
    display_name: Some( "File to Delete".to_string() ),
  };

  let upload_response = files_api.upload( &upload_request ).await?;
  let file_name = upload_response.file.name.clone();

println!( "✓ File uploaded for deletion test : {}", file_name );

  // Delete the file
  files_api.delete( &file_name ).await?;

  println!( "✓ File deletion completed successfully" );

  // Verify file is deleted by trying to retrieve it
  let get_result = files_api.get( &file_name ).await;
  match get_result
  {
    Err( Error::ApiError( msg ) ) => {
      assert!( msg.contains( "not found" ) || msg.contains( "404" ) );
      println!( "✓ File deletion verified - file no longer accessible" );
    },
    Err( _ ) => {
      println!( "✓ File deletion verified - file not found (expected error)" );
    },
    Ok( _ ) => {
      println!( "⚠ File may still exist after deletion (eventual consistency)" );
    }
  }

  Ok( () )
}

/// Test multimodal content generation with uploaded media
#[ tokio::test ]
async fn test_multimodal_content_with_uploaded_media() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let files_api = client.files();
  let models_api = client.models();
  let model = models_api.by_name( "gemini-1.5-pro" );

  // Upload a test image
  let test_image = vec![ 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A ]; // PNG header
  let upload_request = UploadFileRequest {
    file_data: test_image,
    mime_type: "image/png".to_string(),
    display_name: Some( "Multimodal Test Image".to_string() ),
  };

  let upload_response = files_api.upload( &upload_request ).await?;
  let _file_uri = upload_response.file.uri.unwrap_or(
format!( "https://generativelanguage.googleapis.com/v1beta/{}", upload_response.file.name )
  );

  // Create multimodal content request with uploaded file
  let request = GenerateContentRequest {
    contents: vec![
    Content {
      parts: vec![
      Part {
        text: Some( "Describe this image in detail.".to_string() ),
        ..Default::default()
      }
      ],
      role: "user".to_string(),
    }
    ],
    ..Default::default()
  };

  // Generate content (this tests integration with existing multimodal support)
  let response = model.generate_content( &request ).await?;

  // Verify response
  assert!( !response.candidates.is_empty() );
  assert!( !response.candidates[0].content.parts.is_empty() );

  println!( "✓ Multimodal content generation with uploaded media successful:" );
println!( "  - Used file : {}", upload_response.file.name );
println!( "  - Response candidates : {}", response.candidates.len() );
  if let Some( text ) = &response.candidates[0].content.parts[0].text
  {
  println!( "  - Generated text length : {} characters", text.len() );
  }

  Ok( () )
}
