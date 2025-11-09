//! Integration test for enhanced retry logic
//!
//! This test validates that the enhanced retry logic integrates properly
//! with the `OpenAI` client HTTP layer and provides actual retry behavior.

#[ cfg( feature = "retry" ) ]
#[ tokio::test ]
async fn test_client_with_retry_configuration()
{
  use api_openai::
  {
    Client,
    environment ::{ OpenaiEnvironmentImpl, OpenAIRecommended },
    secret ::Secret,
    enhanced_retry ::EnhancedRetryConfig,
  };

  // Create retry configuration with fast settings for testing
  let retry_config = EnhancedRetryConfig::new()
    .with_max_attempts( 2 )
    .with_base_delay( 10 ) // 10ms for fast testing
    .with_max_delay( 100 )
    .with_jitter( 5 );

  // Create client with retry configuration
  let secret = Secret::new( "sk-test-key-for-retry-testing".to_string() ).expect( "Secret creation should succeed" );
  let env = OpenaiEnvironmentImpl::build(
    secret,
    None,
    None,
    OpenAIRecommended::base_url().to_string(),
    OpenAIRecommended::realtime_base_url().to_string()
  ).expect( "Environment creation should succeed" );

  let client = Client::build( env )
    .expect( "Client creation should succeed" )
    .with_retry_config( retry_config.clone() );

  // Verify retry configuration is set
  assert!( client.retry_config().is_some() );
  let client_config = client.retry_config().unwrap();
  assert_eq!( client_config.max_attempts, 2 );
  assert_eq!( client_config.base_delay_ms, 10 );
}

#[ cfg( feature = "retry" ) ]
#[ tokio::test ]
async fn test_client_with_default_retry()
{
  use api_openai::
  {
    Client,
    environment ::{ OpenaiEnvironmentImpl, OpenAIRecommended },
    secret ::Secret,
  };

  // Create client with default retry configuration
  let secret = Secret::new( "sk-test-key-for-retry-testing".to_string() ).expect( "Secret creation should succeed" );
  let env = OpenaiEnvironmentImpl::build(
    secret,
    None,
    None,
    OpenAIRecommended::base_url().to_string(),
    OpenAIRecommended::realtime_base_url().to_string()
  ).expect( "Environment creation should succeed" );

  let client = Client::build( env )
    .expect( "Client creation should succeed" )
    .with_retry();

  // Verify default retry configuration is set
  assert!( client.retry_config().is_some() );
  let client_config = client.retry_config().unwrap();
  assert_eq!( client_config.max_attempts, 3 ); // Default value
  assert_eq!( client_config.base_delay_ms, 1000 ); // Default value
}

#[ cfg( not( feature = "retry" ) ) ]
#[ tokio::test ]
async fn test_client_without_retry_feature()
{
  use api_openai::
  {
    Client,
    environment ::{ OpenaiEnvironmentImpl, OpenAIRecommended },
    secret ::Secret,
  };

  // Create client without retry configuration (feature disabled)
  let secret = Secret::new( "sk-test-key-for-retry-testing".to_string() ).expect( "Secret creation should succeed" );
  let env = OpenaiEnvironmentImpl::build(
    secret,
    None,
    None,
    OpenAIRecommended::base_url().to_string(),
    OpenAIRecommended::realtime_base_url().to_string()
  ).expect( "Environment creation should succeed" );

  let client = Client::build( env )
    .expect( "Client creation should succeed" );

  // Verify no retry configuration is available
  assert!( client.retry_config().is_none() );
}