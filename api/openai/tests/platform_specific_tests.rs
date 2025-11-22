//! Platform-specific features tests

use api_openai::
{
  platform_specific ::
  {
    SearchGroundingConfig, SearchEngine, GroundedResponse, SearchSource, SearchMetadata,
    CodeExecutionConfig, CodeRuntime, SecurityLevel, CodeExecutionResult,
    WebBrowsingConfig, BrowsingResult, BrowsingMetadata,
    ToolParameters, ParameterDefinition, ToolResult, CustomTool,
    ImageGenerationConfig, ImageModel, ImageSize, ImageQuality, ImageStyle, ImageResponseFormat,
    ImageResult, ImageMetadata,
    ApiConnectorConfig, ApiAuthentication, RateLimitConfig, RetryConfig, ApiConnector,
    PlatformSpecificClient,
  },
  environment ::{ OpenaiEnvironmentImpl, OpenAIRecommended },
  secret ::Secret,
  Client,
  error ::Result,
};
use std::
{
  collections ::HashMap,
};
use core::time::Duration;
use serde_json::json;

#[ tokio::test ]
async fn test_search_grounding_config_creation()
{
  let config = SearchGroundingConfig
  {
    search_engine : SearchEngine::Google,
    max_results : 5,
    snippet_length : 150,
    enable_safe_search : true,
    language_preference : Some( "en".to_string() ),
  };

  assert!( matches!( config.search_engine, SearchEngine::Google ) );
  assert_eq!( config.max_results, 5 );
  assert_eq!( config.snippet_length, 150 );
  assert!( config.enable_safe_search );
  assert_eq!( config.language_preference, Some( "en".to_string() ) );
}

#[ test ]
fn test_search_grounding_config_default()
{
  let config = SearchGroundingConfig::default();

  assert!( matches!( config.search_engine, SearchEngine::Google ) );
  assert_eq!( config.max_results, 10 );
  assert_eq!( config.snippet_length, 200 );
  assert!( config.enable_safe_search );
  assert_eq!( config.language_preference, Some( "en".to_string() ) );
}

#[ test ]
fn test_search_engine_variants()
{
  let google = SearchEngine::Google;
  let bing = SearchEngine::Bing;
  let custom = SearchEngine::Custom
  {
    endpoint : "https://api.example.com/search".to_string(),
    api_key : "test-key-12345".to_string(),
  };

  assert!( matches!( google, SearchEngine::Google ) );
  assert!( matches!( bing, SearchEngine::Bing ) );

  if let SearchEngine::Custom { endpoint, api_key } = custom
  {
    assert_eq!( endpoint, "https://api.example.com/search" );
    assert_eq!( api_key, "test-key-12345" );
  }
  else
  {
    panic!( "Expected Custom search engine variant" );
  }
}

#[ test ]
fn test_grounded_response_structure()
{
  let sources = vec![
    SearchSource
    {
      url : "https://example.com/1".to_string(),
      title : "Example 1".to_string(),
      snippet : "This is a test snippet".to_string(),
      relevance_score : 0.95,
    },
    SearchSource
    {
      url : "https://example.com/2".to_string(),
      title : "Example 2".to_string(),
      snippet : "Another test snippet".to_string(),
      relevance_score : 0.88,
    },
  ];

  let metadata = SearchMetadata
  {
    query : "test query".to_string(),
    total_results : 100,
    search_time_ms : 250,
    engine_used : "Google".to_string(),
  };

  let response = GroundedResponse
  {
    response : "Based on search results...".to_string(),
    sources,
    confidence_score : 0.92,
    search_metadata : metadata,
  };

  assert!( response.response.contains( "Based on" ) );
  assert_eq!( response.sources.len(), 2 );
  assert!( ( response.confidence_score - 0.92 ).abs() < f64::EPSILON );
  assert_eq!( response.search_metadata.total_results, 100 );
}

#[ test ]
fn test_code_execution_config_creation()
{
  let config = CodeExecutionConfig
  {
    runtime : CodeRuntime::Python,
    timeout : Duration::from_secs( 60 ),
    memory_limit : 256 * 1024 * 1024, // 256MB
    allowed_imports : vec![ "numpy".to_string(), "pandas".to_string() ],
    security_level : SecurityLevel::Sandbox,
  };

  assert!( matches!( config.runtime, CodeRuntime::Python ) );
  assert_eq!( config.timeout, Duration::from_secs( 60 ) );
  assert_eq!( config.memory_limit, 256 * 1024 * 1024 );
  assert_eq!( config.allowed_imports.len(), 2 );
  assert!( matches!( config.security_level, SecurityLevel::Sandbox ) );
}

#[ test ]
fn test_code_execution_config_default()
{
  let config = CodeExecutionConfig::default();

  assert!( matches!( config.runtime, CodeRuntime::Python ) );
  assert_eq!( config.timeout, Duration::from_secs( 30 ) );
  assert_eq!( config.memory_limit, 128 * 1024 * 1024 );
  assert!( !config.allowed_imports.is_empty() );
  assert!( matches!( config.security_level, SecurityLevel::Sandbox ) );
}

#[ test ]
fn test_code_runtime_variants()
{
  let python = CodeRuntime::Python;
  let javascript = CodeRuntime::JavaScript;
  let rust = CodeRuntime::Rust;
  let go = CodeRuntime::Go;
  let custom = CodeRuntime::Custom
  {
    name : "nodejs".to_string(),
    image : "node:18-alpine".to_string(),
  };

  assert!( matches!( python, CodeRuntime::Python ) );
  assert!( matches!( javascript, CodeRuntime::JavaScript ) );
  assert!( matches!( rust, CodeRuntime::Rust ) );
  assert!( matches!( go, CodeRuntime::Go ) );

  if let CodeRuntime::Custom { name, image } = custom
  {
    assert_eq!( name, "nodejs" );
    assert_eq!( image, "node:18-alpine" );
  }
}

#[ test ]
fn test_security_level_variants()
{
  let sandbox = SecurityLevel::Sandbox;
  let restricted = SecurityLevel::Restricted;
  let trusted = SecurityLevel::Trusted;

  assert!( matches!( sandbox, SecurityLevel::Sandbox ) );
  assert!( matches!( restricted, SecurityLevel::Restricted ) );
  assert!( matches!( trusted, SecurityLevel::Trusted ) );
}

#[ test ]
fn test_code_execution_result_structure()
{
  let result = CodeExecutionResult
  {
    output : "Hello, World!".to_string(),
    error : None,
    execution_time : Duration::from_millis( 150 ),
    memory_used : 1024 * 1024, // 1MB
    return_code : 0,
  };

  assert_eq!( result.output, "Hello, World!" );
  assert!( result.error.is_none() );
  assert_eq!( result.execution_time, Duration::from_millis( 150 ) );
  assert_eq!( result.memory_used, 1024 * 1024 );
  assert_eq!( result.return_code, 0 );

  // Test with error
  let error_result = CodeExecutionResult
  {
    output : String::new(),
    error : Some( "SyntaxError : invalid syntax".to_string() ),
    execution_time : Duration::from_millis( 50 ),
    memory_used : 512 * 1024,
    return_code : 1,
  };

  assert!( error_result.error.is_some() );
  assert_eq!( error_result.return_code, 1 );
}

#[ test ]
fn test_web_browsing_config_creation()
{
  let config = WebBrowsingConfig
  {
    user_agent : "Custom Agent/1.0".to_string(),
    max_page_size : 5 * 1024 * 1024, // 5MB
    follow_redirects : false,
    javascript_enabled : true,
    screenshot_enabled : true,
    timeout : Duration::from_secs( 45 ),
  };

  assert_eq!( config.user_agent, "Custom Agent/1.0" );
  assert_eq!( config.max_page_size, 5 * 1024 * 1024 );
  assert!( !config.follow_redirects );
  assert!( config.javascript_enabled );
  assert!( config.screenshot_enabled );
  assert_eq!( config.timeout, Duration::from_secs( 45 ) );
}

#[ test ]
fn test_web_browsing_config_default()
{
  let config = WebBrowsingConfig::default();

  assert!( config.user_agent.contains( "OpenAI-Client" ) );
  assert_eq!( config.max_page_size, 10 * 1024 * 1024 );
  assert!( config.follow_redirects );
  assert!( !config.javascript_enabled ); // Disabled by default for security
  assert!( !config.screenshot_enabled );
  assert_eq!( config.timeout, Duration::from_secs( 30 ) );
}

#[ test ]
fn test_browsing_result_structure()
{
  let metadata = BrowsingMetadata
  {
    status_code : 200,
    content_type : "text/html".to_string(),
    content_length : 12345,
    load_time_ms : 500,
    redirect_count : 1,
  };

  let result = BrowsingResult
  {
    url : "https://example.com/final".to_string(),
    title : "Example Page".to_string(),
    content : "Page content goes here...".to_string(),
    links : vec![ "https://example.com/link1".to_string(), "https://example.com/link2".to_string() ],
    images : vec![ "https://example.com/image1.jpg".to_string() ],
    screenshot : Some( vec![ 1, 2, 3, 4, 5 ] ), // Mock screenshot data
    metadata,
  };

  assert_eq!( result.url, "https://example.com/final" );
  assert_eq!( result.title, "Example Page" );
  assert!( result.content.contains( "Page content" ) );
  assert_eq!( result.links.len(), 2 );
  assert_eq!( result.images.len(), 1 );
  assert!( result.screenshot.is_some() );
  assert_eq!( result.metadata.status_code, 200 );
}

#[ test ]
fn test_tool_parameters_creation()
{
  let mut properties = HashMap::new();
  properties.insert( "input".to_string(), ParameterDefinition
  {
    param_type : "string".to_string(),
    description : "Input text to process".to_string(),
    required : true,
    default : None,
  });
  properties.insert( "format".to_string(), ParameterDefinition
  {
    param_type : "string".to_string(),
    description : "Output format".to_string(),
    required : false,
    default : Some( json!( "text" ) ),
  });

  let params = ToolParameters
  {
    required : vec![ "input".to_string() ],
    properties,
  };

  assert_eq!( params.required.len(), 1 );
  assert_eq!( params.properties.len(), 2 );
  assert!( params.properties.contains_key( "input" ) );
  assert!( params.properties.contains_key( "format" ) );

  let format_param = &params.properties[ "format" ];
  assert!( !format_param.required );
  assert!( format_param.default.is_some() );
}

#[ test ]
fn test_tool_result_structure()
{
  let success_result = ToolResult
  {
    output : json!( { "result": "processed successfully" } ),
    success : true,
    error_message : None,
  };

  assert!( success_result.success );
  assert!( success_result.error_message.is_none() );
  assert!( success_result.output.is_object() );

  let error_result = ToolResult
  {
    output : json!( null ),
    success : false,
    error_message : Some( "Processing failed".to_string() ),
  };

  assert!( !error_result.success );
  assert!( error_result.error_message.is_some() );
}

// Mock implementation of CustomTool for testing
struct MockTool
{
  name : String,
  description : String,
}

impl MockTool
{
  fn new( name : &str, description : &str ) -> Self
  {
    Self
    {
      name : name.to_string(),
      description : description.to_string(),
    }
  }
}

#[ async_trait::async_trait ]
impl CustomTool for MockTool
{
  fn name( &self ) -> &str
  {
    &self.name
  }

  fn description( &self ) -> &str
  {
    &self.description
  }

  fn parameters( &self ) -> ToolParameters
  {
    let mut properties = HashMap::new();
    properties.insert( "text".to_string(), ParameterDefinition
    {
      param_type : "string".to_string(),
      description : "Text to process".to_string(),
      required : true,
      default : None,
    });

    ToolParameters
    {
      required : vec![ "text".to_string() ],
      properties,
    }
  }

  async fn execute( &self, parameters : serde_json::Value ) -> Result< ToolResult >
  {
    if let Some( text ) = parameters.get( "text" ).and_then( | v | v.as_str() )
    {
      Ok( ToolResult
      {
        output : json!( { "processed": text.to_uppercase() } ),
        success : true,
        error_message : None,
      })
    }
    else
    {
      Ok( ToolResult
      {
        output : json!( null ),
        success : false,
        error_message : Some( "Missing required parameter : text".to_string() ),
      })
    }
  }
}

#[ tokio::test ]
async fn test_custom_tool_implementation()
{
  let tool = MockTool::new( "text_processor", "Processes text input" );

  assert_eq!( tool.name(), "text_processor" );
  assert_eq!( tool.description(), "Processes text input" );

  let params = tool.parameters();
  assert_eq!( params.required.len(), 1 );
  assert!( params.properties.contains_key( "text" ) );

  // Test successful execution
  let input = json!( { "text": "hello world" } );
  let result = tool.execute( input ).await.unwrap();
  assert!( result.success );
  assert_eq!( result.output[ "processed" ], "HELLO WORLD" );

  // Test error case
  let invalid_input = json!( { "invalid": "data" } );
  let error_result = tool.execute( invalid_input ).await.unwrap();
  assert!( !error_result.success );
  assert!( error_result.error_message.is_some() );
}

#[ test ]
fn test_image_generation_config_creation()
{
  let config = ImageGenerationConfig
  {
    model : ImageModel::DallE3,
    size : ImageSize::Wide1792x1024,
    quality : ImageQuality::HD,
    style : ImageStyle::Natural,
    response_format : ImageResponseFormat::Base64,
  };

  assert!( matches!( config.model, ImageModel::DallE3 ) );
  assert!( matches!( config.size, ImageSize::Wide1792x1024 ) );
  assert!( matches!( config.quality, ImageQuality::HD ) );
  assert!( matches!( config.style, ImageStyle::Natural ) );
  assert!( matches!( config.response_format, ImageResponseFormat::Base64 ) );
}

#[ test ]
fn test_image_generation_config_default()
{
  let config = ImageGenerationConfig::default();

  assert!( matches!( config.model, ImageModel::DallE3 ) );
  assert!( matches!( config.size, ImageSize::Square1024 ) );
  assert!( matches!( config.quality, ImageQuality::Standard ) );
  assert!( matches!( config.style, ImageStyle::Vivid ) );
  assert!( matches!( config.response_format, ImageResponseFormat::Url ) );
}

#[ test ]
fn test_image_model_variants()
{
  let dalle2 = ImageModel::DallE2;
  let dalle3 = ImageModel::DallE3;
  let custom = ImageModel::Custom( "midjourney-v6".to_string() );

  assert!( matches!( dalle2, ImageModel::DallE2 ) );
  assert!( matches!( dalle3, ImageModel::DallE3 ) );

  if let ImageModel::Custom( name ) = custom
  {
    assert_eq!( name, "midjourney-v6" );
  }
}

#[ test ]
fn test_image_size_variants()
{
  let sizes = [
    ImageSize::Square256,
    ImageSize::Square512,
    ImageSize::Square1024,
    ImageSize::Wide1792x1024,
    ImageSize::Tall1024x1792,
  ];

  assert_eq!( sizes.len(), 5 );

  // Test that all variants are different
  for ( i, size1 ) in sizes.iter().enumerate()
  {
    for ( j, size2 ) in sizes.iter().enumerate()
    {
      if i != j
      {
        assert!( core::mem::discriminant( size1 ) != core::mem::discriminant( size2 ) );
      }
    }
  }
}

#[ test ]
fn test_image_result_structure()
{
  let metadata = ImageMetadata
  {
    width : 1024,
    height : 1024,
    format : "PNG".to_string(),
    generation_time_ms : 5000,
  };

  let url_result = ImageResult
  {
    url : Some( "https://example.com/image.png".to_string() ),
    b64_json : None,
    metadata : metadata.clone(),
  };

  let base64_result = ImageResult
  {
    url : None,
    b64_json : Some( "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChAGAWaM8jwAAAABJRU5ErkJggg==".to_string() ),
    metadata,
  };

  assert!( url_result.url.is_some() );
  assert!( url_result.b64_json.is_none() );
  assert_eq!( url_result.metadata.width, 1024 );

  assert!( base64_result.url.is_none() );
  assert!( base64_result.b64_json.is_some() );
  assert_eq!( base64_result.metadata.format, "PNG" );
}

#[ test ]
fn test_api_connector_config_creation()
{
  let auth = ApiAuthentication::ApiKey
  {
    header : "Authorization".to_string(),
    key : "Bearer test-token".to_string(),
  };

  let rate_limits = RateLimitConfig
  {
    requests_per_minute : 100,
    requests_per_hour : 1000,
    burst_limit : 10,
  };

  let retry_config = RetryConfig
  {
    max_retries : 3,
    base_delay : Duration::from_millis( 100 ),
    max_delay : Duration::from_secs( 10 ),
    backoff_factor : 2.0,
  };

  let config = ApiConnectorConfig
  {
    base_url : "https://api.example.com".to_string(),
    authentication : auth,
    rate_limits : Some( rate_limits ),
    retry_config : Some( retry_config ),
  };

  assert_eq!( config.base_url, "https://api.example.com" );
  assert!( matches!( config.authentication, ApiAuthentication::ApiKey { .. } ) );
  assert!( config.rate_limits.is_some() );
  assert!( config.retry_config.is_some() );
}

#[ test ]
fn test_api_authentication_variants()
{
  let none = ApiAuthentication::None;
  let api_key = ApiAuthentication::ApiKey
  {
    header : "X-API-Key".to_string(),
    key : "secret-key".to_string(),
  };
  let bearer = ApiAuthentication::Bearer
  {
    token : "bearer-token".to_string(),
  };
  let oauth2 = ApiAuthentication::OAuth2
  {
    client_id : "client-123".to_string(),
    client_secret : "secret-456".to_string(),
  };

  let mut headers = HashMap::new();
  headers.insert( "Custom-Auth".to_string(), "custom-value".to_string() );
  let custom = ApiAuthentication::Custom { headers };

  assert!( matches!( none, ApiAuthentication::None ) );
  assert!( matches!( api_key, ApiAuthentication::ApiKey { .. } ) );
  assert!( matches!( bearer, ApiAuthentication::Bearer { .. } ) );
  assert!( matches!( oauth2, ApiAuthentication::OAuth2 { .. } ) );
  assert!( matches!( custom, ApiAuthentication::Custom { .. } ) );
}

// Mock implementation of ApiConnector for testing
struct MockApiConnector
{
  name : String,
  config : ApiConnectorConfig,
}

impl MockApiConnector
{
  fn new( name : &str, base_url : &str ) -> Self
  {
    Self
    {
      name : name.to_string(),
      config : ApiConnectorConfig
      {
        base_url : base_url.to_string(),
        authentication : ApiAuthentication::None,
        rate_limits : None,
        retry_config : None,
      },
    }
  }
}

#[ async_trait::async_trait ]
impl ApiConnector for MockApiConnector
{
  fn name( &self ) -> &str
  {
    &self.name
  }

  fn config( &self ) -> &ApiConnectorConfig
  {
    &self.config
  }

  async fn make_request(
    &self,
    method : &str,
    endpoint : &str,
    body : Option< serde_json::Value >
  ) -> Result< serde_json::Value >
  {
    // Mock implementation that returns a success response
    Ok( json!({
      "mock_response": true,
      "method": method,
      "endpoint": endpoint,
      "body_provided": body.is_some()
    }))
  }
}

#[ tokio::test ]
async fn test_api_connector_implementation()
{
  let connector = MockApiConnector::new( "test-api", "https://api.test.com" );

  assert_eq!( connector.name(), "test-api" );
  assert_eq!( connector.config().base_url, "https://api.test.com" );

  let response = connector.make_request( "GET", "/test", None ).await.unwrap();
  assert_eq!( response[ "mock_response" ], true );
  assert_eq!( response[ "method" ], "GET" );
  assert_eq!( response[ "endpoint" ], "/test" );
  assert_eq!( response[ "body_provided" ], false );

  let body = json!( { "test": "data" } );
  let response_with_body = connector.make_request( "POST", "/create", Some( body ) ).await.unwrap();
  assert_eq!( response_with_body[ "body_provided" ], true );
}

#[ tokio::test ]
async fn test_platform_specific_client_trait_not_implemented()
{
  // Test that the default implementations return NotSupported errors
  let secret = Secret::new_unchecked( "sk-test_platform_specific_1234567890abcdef".to_string() );
  let environment = OpenaiEnvironmentImpl::build(
    secret,
    None,
    None,
    OpenAIRecommended::base_url().to_string(),
    OpenAIRecommended::realtime_base_url().to_string(),
  ).expect( "Environment creation should work" );

  let client = Client::build( environment ).expect( "Client creation should work" );

  // Test search grounding - should return NotSupported
  let search_config = SearchGroundingConfig::default();
  let search_result = client.search_and_ground( "test query", None, search_config ).await;
  assert!( search_result.is_err() );

  if let Err( e ) = search_result
  {
    assert!( e.to_string().contains( "not yet implemented" ) );
  }

  // Test code execution - should return NotSupported
  let code_config = CodeExecutionConfig::default();
  let code_result = client.execute_code( "print('hello')", code_config ).await;
  assert!( code_result.is_err() );

  if let Err( e ) = code_result
  {
    assert!( e.to_string().contains( "not yet implemented" ) );
  }

  // Test web browsing - should return NotSupported
  let browse_config = WebBrowsingConfig::default();
  let browse_result = client.browse_url( "https://example.com", browse_config ).await;
  assert!( browse_result.is_err() );

  if let Err( e ) = browse_result
  {
    assert!( e.to_string().contains( "not yet implemented" ) );
  }

  // Test image generation - should return NotSupported
  let image_config = ImageGenerationConfig::default();
  let image_result = client.generate_image( "a beautiful sunset", image_config ).await;
  assert!( image_result.is_err() );

  if let Err( e ) = image_result
  {
    assert!( e.to_string().contains( "not yet implemented" ) );
  }

  // Test tool methods - should return NotSupported or empty list
  let tools = client.list_registered_tools();
  assert!( tools.is_empty() );
}

#[ test ]
fn test_rate_limit_config_structure()
{
  let config = RateLimitConfig
  {
    requests_per_minute : 60,
    requests_per_hour : 1000,
    burst_limit : 10,
  };

  assert_eq!( config.requests_per_minute, 60 );
  assert_eq!( config.requests_per_hour, 1000 );
  assert_eq!( config.burst_limit, 10 );
}

#[ test ]
fn test_retry_config_structure()
{
  let config = RetryConfig
  {
    max_retries : 5,
    base_delay : Duration::from_millis( 200 ),
    max_delay : Duration::from_secs( 30 ),
    backoff_factor : 1.5,
  };

  assert_eq!( config.max_retries, 5 );
  assert_eq!( config.base_delay, Duration::from_millis( 200 ) );
  assert_eq!( config.max_delay, Duration::from_secs( 30 ) );
  assert!( ( config.backoff_factor - 1.5 ).abs() < f64::EPSILON );
}

#[ test ]
fn test_serialization_deserialization()
{
  // Test SearchGroundingConfig serialization
  let search_config = SearchGroundingConfig::default();
  let serialized = serde_json::to_string( &search_config ).unwrap();
  let deserialized : SearchGroundingConfig = serde_json::from_str( &serialized ).unwrap();
  assert_eq!( deserialized.max_results, search_config.max_results );

  // Test CodeExecutionConfig serialization
  let code_config = CodeExecutionConfig::default();
  let serialized = serde_json::to_string( &code_config ).unwrap();
  let deserialized : CodeExecutionConfig = serde_json::from_str( &serialized ).unwrap();
  assert_eq!( deserialized.memory_limit, code_config.memory_limit );

  // Test ImageGenerationConfig serialization
  let image_config = ImageGenerationConfig::default();
  let serialized = serde_json::to_string( &image_config ).unwrap();
  let deserialized : ImageGenerationConfig = serde_json::from_str( &serialized ).unwrap();
  assert!( matches!( deserialized.model, ImageModel::DallE3 ) );
}

#[ test ]
fn test_duration_handling()
{
  let config = CodeExecutionConfig
  {
    timeout : Duration::from_secs( 120 ),
    ..Default::default()
  };

  assert_eq!( config.timeout, Duration::from_secs( 120 ) );
  assert!( config.timeout > Duration::from_secs( 60 ) );
  assert!( config.timeout < Duration::from_secs( 180 ) );
}

#[ test ]
fn test_memory_limit_calculations()
{
  let small_config = CodeExecutionConfig
  {
    memory_limit : 64 * 1024 * 1024, // 64MB
    ..Default::default()
  };

  let large_config = CodeExecutionConfig
  {
    memory_limit : 1024 * 1024 * 1024, // 1GB
    ..Default::default()
  };

  assert_eq!( small_config.memory_limit, 64 * 1024 * 1024 );
  assert_eq!( large_config.memory_limit, 1024 * 1024 * 1024 );
  assert!( large_config.memory_limit > small_config.memory_limit );
}

#[ test ]
fn test_browsing_metadata_http_codes()
{
  let success_metadata = BrowsingMetadata
  {
    status_code : 200,
    content_type : "text/html".to_string(),
    content_length : 5000,
    load_time_ms : 250,
    redirect_count : 0,
  };

  let redirect_metadata = BrowsingMetadata
  {
    status_code : 301,
    content_type : "text/html".to_string(),
    content_length : 1000,
    load_time_ms : 150,
    redirect_count : 2,
  };

  assert_eq!( success_metadata.status_code, 200 );
  assert_eq!( success_metadata.redirect_count, 0 );
  assert_eq!( redirect_metadata.status_code, 301 );
  assert_eq!( redirect_metadata.redirect_count, 2 );
}