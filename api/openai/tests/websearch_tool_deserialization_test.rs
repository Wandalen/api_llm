//! Test for `WebSearchTool` JSON deserialization issue
//!
//! This test reproduces the issue where `WebSearchTool` fails to deserialize
//! because it's defined as a unit struct but the API returns a complex object.

use api_openai::components::tools::Tool;

#[ test ]
fn test_websearch_tool_deserialization_failure()
{
  // This is the JSON structure returned by the OpenAI API for web_search_preview tools
  let api_response_json = r#"{
    "type": "web_search_preview",
    "search_context_size": "medium",
    "user_location": {
      "type": "approximate",
      "city": null,
      "country": "US",
      "region": null,
      "timezone": null
    }
  }"#;

  // This should fail because WebSearchTool is currently defined as a unit struct
  // but the API returns a complex object with fields
  let result : Result< Tool, _ > = serde_json::from_str( api_response_json );

  match result
  {
    Ok( tool ) =>
    {
      println!( "✅ WebSearchTool deserialization succeeded : {tool:?}" );

      // Verify it's the correct tool type
      if let Tool::WebSearch( web_search_tool ) = tool
      {
        println!( "✅ Successfully parsed WebSearchTool with proper structure" );
        println!( "   - search_context_size : {:?}", web_search_tool.search_context_size );
        println!( "   - user_location : {:?}", web_search_tool.user_location );

        // Verify we captured the expected fields
        assert_eq!( web_search_tool.search_context_size, Some( "medium".to_string() ) );
        assert!( web_search_tool.user_location.is_some() );

        if let Some( location ) = &web_search_tool.user_location
        {
          assert_eq!( location.r#type, Some( "approximate".to_string() ) );
          assert_eq!( location.country, Some( "US".to_string() ) );
          assert_eq!( location.city, None );
        }
      }
      else
      {
        panic!( "❌ Parsed tool is not WebSearch variant" );
      }
    },
    Err( e ) =>
    {
      println!( "❌ ISSUE: WebSearchTool deserialization failed : {e}" );

      // Check if it's the specific unit struct issue
      let error_msg = e.to_string();
      if error_msg.contains( "invalid type : map, expected unit struct" )
      {
        println!( "🔍 CONFIRMED: Issue is unit struct vs complex object mismatch" );
        println!( "🔧 FIX NEEDED: WebSearchTool needs to be changed from unit struct to struct with fields" );
        panic!( "WebSearchTool deserialization fails - needs structure fix" );
      }
      else
      {
        panic!( "❌ Unexpected deserialization error : {error_msg}" );
      }
    }
  }
}

#[ test ]
fn test_websearch_tool_unit_struct_behavior()
{
  // Test the current unit struct behavior
  let unit_struct_json = r#"{"type": "web_search_preview"}"#;

  // This might work with just the type field
  let result : Result< Tool, _ > = serde_json::from_str( unit_struct_json );

  match result
  {
    Ok( tool ) =>
    {
      println!( "✅ Unit struct WebSearchTool works with minimal JSON: {tool:?}" );
    },
    Err( e ) =>
    {
      println!( "❌ Even unit struct fails : {e}" );
    }
  }
}