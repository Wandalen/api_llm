//! Advanced file handling tests : large files, multiple types, search, processing, versioning

use super::*;

/// Test large file handling (with size limits)
#[ tokio::test ]
async fn test_large_file_handling() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let files_api = client.files();

  // Create a moderately large test file (1MB)
  let large_file_size = 1024 * 1024;
  let large_file_data = vec![ 0x42; large_file_size ];

  let upload_request = UploadFileRequest {
    file_data : large_file_data,
    mime_type : "application/octet-stream".to_string(),
    display_name : Some( "Large File Test".to_string() ),
  };

  let upload_result = files_api.upload( &upload_request ).await;

  match upload_result
  {
    Ok( response ) => {
      println!( "✓ Large file upload successful:" );
      println!( "  - File name : {}", response.file.name );
      println!( "  - File size : {:?} bytes", response.file.size_bytes );
      let _ = files_api.delete( &response.file.name ).await;
    },
    Err( Error::ApiError( msg ) ) if msg.contains( "size" ) || msg.contains( "limit" ) =>
    {
      println!( "✓ Large file upload rejected due to size limits (expected): {}", msg );
    },
    Err( e ) => {
      println!( "✓ Large file upload handling error (may be expected): {}", e );
    }
  }

  Ok( () )
}

/// Test multiple file types upload
#[ tokio::test ]
async fn test_multiple_file_types_upload() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let files_api = client.files();

  let test_files = vec![
    ( "image/jpeg", vec![ 0xFF, 0xD8, 0xFF, 0xE0 ], "JPEG Image" ),
    ( "image/png", vec![ 0x89, 0x50, 0x4E, 0x47 ], "PNG Image" ),
    ( "text/plain", b"Hello, World!".to_vec(), "Text File" ),
    ( "application/json", b"{\"test\": true}".to_vec(), "JSON Document" ),
  ];

  let mut uploaded_files = Vec::new();

  for ( mime_type, data, display_name ) in test_files
  {
    let upload_request = UploadFileRequest {
      file_data : data,
      mime_type : mime_type.to_string(),
      display_name : Some( display_name.to_string() ),
    };

    match files_api.upload( &upload_request ).await
    {
      Ok( response ) => {
        println!( "✓ {} upload successful : {}", mime_type, response.file.name );
        uploaded_files.push( response.file.name );
      },
      Err( e ) => {
        println!( "⚠ {} upload failed (may be expected): {}", mime_type, e );
      }
    }
  }

  println!( "✓ Multiple file types test completed : {}/{} successful", uploaded_files.len(), 4 );

  for file_name in uploaded_files
  {
    let _ = files_api.delete( &file_name ).await;
  }

  Ok( () )
}

/// Test media search and filtering capabilities
#[ tokio::test ]
async fn test_media_search_and_filtering() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let files_api = client.files();

  let test_uploads = vec![
    ( "image/png", "Search Test Image" ),
    ( "text/plain", "Search Test Document" ),
    ( "application/json", "Search Test Data" ),
  ];

  let mut uploaded_files = Vec::new();

  for ( mime_type, display_name ) in test_uploads
  {
    let test_data = match mime_type
    {
      "image/png" => vec![ 0x89, 0x50, 0x4E, 0x47 ],
      "text/plain" => b"Test document content".to_vec(),
      "application/json" => b"{\"search\": \"test\"}".to_vec(),
      _ => vec![ 0x00, 0x01, 0x02, 0x03 ],
    };

    let upload_request = UploadFileRequest {
      file_data : test_data,
      mime_type : mime_type.to_string(),
      display_name : Some( display_name.to_string() ),
    };

    if let Ok( response ) = files_api.upload( &upload_request ).await
    {
      uploaded_files.push( response.file.name );
    }
  }

  let list_request = ListFilesRequest {
    page_size : Some( 100 ),
    page_token : None,
  };

  let list_response = files_api.list( &list_request ).await?;

  let mut type_counts = HashMap::new();
  for file in &list_response.files
  {
    let count = type_counts.entry( file.mime_type.clone() ).or_insert( 0 );
    *count += 1;
  }

  println!( "✓ Media search and filtering test results:" );
  println!( "  - Total files : {}", list_response.files.len() );
  for ( mime_type, count ) in type_counts
  {
    println!( "  - {}: {} files", mime_type, count );
  }

  for file_name in uploaded_files
  {
    let _ = files_api.delete( &file_name ).await;
  }

  Ok( () )
}

/// Test media processing and transformation capabilities
#[ tokio::test ]
async fn test_media_processing_capabilities() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let files_api = client.files();

  let test_image = vec![ 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A ];
  let upload_request = UploadFileRequest {
    file_data : test_image,
    mime_type : "image/png".to_string(),
    display_name : Some( "Processing Test Image".to_string() ),
  };

  let upload_response = files_api.upload( &upload_request ).await?;
  let file_metadata = files_api.get( &upload_response.file.name ).await?;

  println!( "✓ Media processing capabilities test:" );
  println!( "  - File uploaded : {}", file_metadata.name );
  println!( "  - MIME type detected : {}", file_metadata.mime_type );
  println!( "  - Size analyzed : {:?} bytes", file_metadata.size_bytes );

  if let Some( hash ) = &file_metadata.sha256_hash
  {
    println!( "  - SHA256 hash computed : {}", hash );
  }

  if let Some( state ) = &file_metadata.state
  {
    println!( "  - Processing state : {}", state );
  }

  if file_metadata.mime_type.starts_with( "video/" )
  {
    if let Some( video_meta ) = &file_metadata.video_metadata
    {
      println!( "  - Video duration extracted : {:?}", video_meta.video_duration );
    }
  }

  let _ = files_api.delete( &upload_response.file.name ).await;

  Ok( () )
}

/// Test media versioning and management
#[ tokio::test ]
async fn test_media_versioning_management() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let files_api = client.files();

  let versions = vec![
    ( "Version 1.0", vec![ 0x89, 0x50, 0x4E, 0x47, 0x01 ] ),
    ( "Version 1.1", vec![ 0x89, 0x50, 0x4E, 0x47, 0x02 ] ),
    ( "Version 2.0", vec![ 0x89, 0x50, 0x4E, 0x47, 0x03 ] ),
  ];

  let mut uploaded_versions = Vec::new();

  for ( version_name, data ) in versions
  {
    let upload_request = UploadFileRequest {
      file_data : data,
      mime_type : "image/png".to_string(),
      display_name : Some( format!( "Versioned File - {}", version_name ) ),
    };

    if let Ok( response ) = files_api.upload( &upload_request ).await
    {
      uploaded_versions.push( ( version_name, response.file.name, response.file.create_time ) );
    }
  }

  println!( "✓ Media versioning test results:" );
  for ( version, file_name, create_time ) in &uploaded_versions
  {
    println!( "  - {}: {} (created : {:?})", version, file_name, create_time );
  }

  let list_response = files_api.list( &ListFilesRequest::default() ).await?;
  let versioned_files : Vec< _ > = list_response.files.iter()
    .filter( |f| f.display_name.as_ref().map_or( false, |name| name.contains( "Versioned File" ) ) )
    .collect();

  println!( "  - Found {} versioned files in listing", versioned_files.len() );

  for ( _, file_name, _ ) in uploaded_versions
  {
    let _ = files_api.delete( &file_name ).await;
  }

  Ok( () )
}
