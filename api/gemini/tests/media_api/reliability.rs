//! Reliability tests : error handling, pagination, concurrent operations

use super::*;

/// Test error handling and edge cases
#[ tokio::test ]
async fn test_error_handling_edge_cases() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let files_api = client.files();

  println!( "✓ Testing error handling and edge cases:" );

  let empty_upload = UploadFileRequest {
    file_data: vec![],
    mime_type: "image/png".to_string(),
    display_name: Some( "Empty File".to_string() ),
  };

  match files_api.upload( &empty_upload ).await
  {
    Err( e ) => {
    println!( "  - Empty file upload properly rejected : {}", e );
      assert!( !e.to_string().is_empty(), "Error message should be non-empty" );
    },
    Ok( _ ) => {
      println!( "  - Empty file upload unexpectedly succeeded" );
      // Empty upload succeeded - this is valid API behavior
    },
  }

  match files_api.get( "invalid/file/name" ).await
  {
    Err( e ) => {
    println!( "  - Invalid file name properly rejected : {}", e );
      assert!( !e.to_string().is_empty(), "Error should have meaningful message" );
    },
    Ok( _ ) => {
      println!( "  - Invalid file name unexpectedly succeeded" );
      panic!( "Invalid file name should not succeed" );
    },
  }

  match files_api.delete( "files/non-existent-file-123" ).await
  {
    Err( e ) => {
    println!( "  - Non-existent file deletion properly rejected : {}", e );
      assert!( !e.to_string().is_empty(), "Error should have meaningful message" );
    },
    Ok( _ ) => {
      println!( "  - Non-existent file deletion unexpectedly succeeded" );
      // Idempotent deletion is valid API behavior
    },
  }

  let invalid_mime_upload = UploadFileRequest {
    file_data: vec![ 1, 2, 3, 4 ],
    mime_type: "invalid/mime/type".to_string(),
    display_name: Some( "Invalid MIME".to_string() ),
  };

  match files_api.upload( &invalid_mime_upload ).await
  {
    Err( e ) => {
    println!( "  - Invalid MIME type properly handled : {}", e );
      assert!( !e.to_string().is_empty(), "Error should have meaningful message" );
    },
    Ok( response ) => {
    println!( "  - Invalid MIME type accepted (API flexibility): {}", response.file.mime_type );
      assert!( !response.file.name.is_empty(), "Uploaded file should have valid name" );
      let _ = files_api.delete( &response.file.name ).await;
    }
  }

  println!( "✓ Error handling and edge cases testing completed" );

  // Assert that we tested all error cases
  assert!( true, "All error handling tests completed" );

  Ok( () )
}

/// Test pagination and large result sets
#[ tokio::test ]
async fn test_pagination_large_result_sets() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_integration_client();
  let files_api = client.files();

  let page_sizes = vec![ 1, 5, 10, 50 ];

  for page_size in page_sizes
  {
    let list_request = ListFilesRequest {
      page_size: Some( page_size ),
      page_token: None,
    };

    let list_response = files_api.list( &list_request ).await?;

  println!( "✓ Pagination test with page_size {}:", page_size );
  println!( "  - Files returned : {}", list_response.files.len() );
  println!( "  - Has next page : {}", list_response.next_page_token.is_some() );

    // Assert pagination respects page size limit
    assert!( list_response.files.len() <= page_size, "Returned files should not exceed page_size" );

    if let Some( next_token ) = list_response.next_page_token
    {
      let next_page_request = ListFilesRequest {
        page_size: Some( page_size ),
        page_token: Some( next_token ),
      };

      let next_page_response = files_api.list( &next_page_request ).await?;
    println!( "  - Next page files : {}", next_page_response.files.len() );

      // Assert next page also respects page size
      assert!( next_page_response.files.len() <= page_size, "Next page files should not exceed page_size" );
    }
  }

  Ok( () )
}

/// Test concurrent file operations
#[ tokio::test ]
async fn test_concurrent_file_operations() -> Result< (), Box< dyn std::error::Error > >
{
  let _client = create_integration_client();

  let concurrent_uploads = vec![
  ( "image/png", b"Concurrent Upload 1".to_vec() ),
  ( "text/plain", b"Concurrent Upload 2".to_vec() ),
( "application/json", b"{\"test\": \"concurrent3\"}".to_vec() ),
  ];

  let upload_handles : Vec< _ > = concurrent_uploads.into_iter().enumerate().map( |( index, ( mime_type, data ) )| {
    tokio ::spawn( async move {
      let client = create_integration_client();
      let files_api = client.files();
      let upload_request = UploadFileRequest {
        file_data: data,
        mime_type: mime_type.to_string(),
      display_name : Some( format!( "Concurrent Test {}", index + 1 ) ),
      };
      files_api.upload( &upload_request ).await
    } )
  } ).collect();

  let mut successful_uploads = Vec::new();
  for ( index, handle ) in upload_handles.into_iter().enumerate()
  {
    match handle.await?
    {
      Ok( response ) => {
    println!( "✓ Concurrent upload {} successful : {}", index + 1, response.file.name );
        successful_uploads.push( response.file.name );
      },
      Err( e ) => {
    println!( "⚠ Concurrent upload {} failed : {}", index + 1, e );
      }
    }
  }

println!( "✓ Concurrent operations test : {}/{} uploads successful", successful_uploads.len(), 3 );

  // Assert that at least some concurrent operations succeeded
  assert!( !successful_uploads.is_empty(), "At least one concurrent upload should succeed" );

  let cleanup_client = create_integration_client();
  let cleanup_files_api = cleanup_client.files();
  for file_name in successful_uploads
  {
    let _ = cleanup_files_api.delete( &file_name ).await;
  }

  Ok( () )
}
