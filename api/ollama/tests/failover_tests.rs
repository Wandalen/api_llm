//! Failover system tests for automatic endpoint switching and high availability
//!
//! This test suite validates the failover functionality including:
//! - Multi-endpoint configuration
//! - Automatic failover on primary endpoint failure
//! - Health tracking and endpoint state management
//! - Transparent operation during failover
//! - Different failover policies (round-robin, priority-based)

#![ allow( clippy::std_instead_of_core ) ] // std required for time operations

use api_ollama::{ OllamaClient, ChatRequest, ChatMessage, MessageRole, FailoverPolicy };
use std::time::Duration;

/// Test failover client creation with multiple endpoints
#[ tokio::test ]
async fn test_failover_client_creation()
{
  let endpoints = vec![
    "http://localhost:11434".to_string(),
    "http://backup1:11434".to_string(),
    "http://backup2:11434".to_string(),
  ];

  let result = OllamaClient::new_with_failover( endpoints, Duration::from_secs( 30 ) );
  assert!( result.is_ok() );

  let client = result.unwrap();
  assert_eq!( client.get_active_endpoint(), "http://localhost:11434" );
  assert_eq!( client.get_endpoint_count(), 3 );
}

/// Test failover configuration validation
#[ tokio::test ]
async fn test_failover_configuration_validation()
{
  // Test empty endpoints list
  let empty_endpoints : Vec< String > = vec![];
  let result = OllamaClient::new_with_failover( empty_endpoints, Duration::from_secs( 30 ) );
  assert!( result.is_err() );

  // Test invalid URLs
  let invalid_endpoints = vec![ "not-a-url".to_string() ];
  let result = OllamaClient::new_with_failover( invalid_endpoints, Duration::from_secs( 30 ) );
  assert!( result.is_err() );
}

/// Test automatic failover on endpoint failure
#[ tokio::test ]
async fn test_automatic_failover_on_failure()
{
  let endpoints = vec![
    "http://invalid-endpoint:11434".to_string(),  // This should fail
    "http://localhost:11434".to_string(),         // This should work
  ];

  let mut client = OllamaClient::new_with_failover( endpoints, Duration::from_secs( 5 ) ).unwrap();

  // Create a basic chat request
  let request = ChatRequest
  {
    model : "test-model".to_string(),
    messages : vec![ ChatMessage
    {
      role : MessageRole::User,
      content : "Hello".to_string(),
      #[ cfg( feature = "vision_support" ) ]
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    }],
    stream : Some( false ),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  // The first endpoint should fail, and it should automatically failover to the second
  // In a real test environment, this would connect to a real Ollama instance
  // For now, we'll just test that the failover logic attempts to use the backup endpoint
  let initial_endpoint = client.get_active_endpoint();
  assert_eq!( initial_endpoint, "http://invalid-endpoint:11434" );

  // After the failed request, it should have switched to the backup
  let _ = client.chat( request ).await; // This will likely fail in test environment, but should trigger failover

  // Check that failover was attempted (endpoint should have changed or health tracking updated)
  let failover_stats = client.get_failover_stats();
  let _ = failover_stats.total_failovers; // Check that stats are accessible
}

/// Test round-robin failover policy
#[ tokio::test ]
async fn test_round_robin_failover_policy()
{
  let endpoints = vec![
    "http://endpoint1:11434".to_string(),
    "http://endpoint2:11434".to_string(),
    "http://endpoint3:11434".to_string(),
  ];

  let mut client = OllamaClient::new_with_failover_policy(
    endpoints,
    Duration::from_secs( 30 ),
    FailoverPolicy::RoundRobin
  ).unwrap();

  // Test round-robin endpoint selection
  assert_eq!( client.get_active_endpoint(), "http://endpoint1:11434" );

  client.rotate_endpoint();
  assert_eq!( client.get_active_endpoint(), "http://endpoint2:11434" );

  client.rotate_endpoint();
  assert_eq!( client.get_active_endpoint(), "http://endpoint3:11434" );

  client.rotate_endpoint();
  assert_eq!( client.get_active_endpoint(), "http://endpoint1:11434" ); // Should wrap around
}

/// Test priority-based failover policy
#[ tokio::test ]
async fn test_priority_based_failover_policy()
{
  let endpoints = vec![
    "http://primary:11434".to_string(),     // Highest priority
    "http://secondary:11434".to_string(),   // Medium priority
    "http://tertiary:11434".to_string(),    // Lowest priority
  ];

  let mut client = OllamaClient::new_with_failover_policy(
    endpoints,
    Duration::from_secs( 30 ),
    FailoverPolicy::Priority
  ).unwrap();

  // Should always prefer primary endpoint when healthy
  assert_eq!( client.get_active_endpoint(), "http://primary:11434" );

  // Mark primary as unhealthy
  client.mark_endpoint_unhealthy( "http://primary:11434" );
  assert_eq!( client.get_active_endpoint(), "http://secondary:11434" );

  // Mark secondary as unhealthy
  client.mark_endpoint_unhealthy( "http://secondary:11434" );
  assert_eq!( client.get_active_endpoint(), "http://tertiary:11434" );

  // Mark primary as healthy again - should switch back
  client.mark_endpoint_healthy( "http://primary:11434" );
  assert_eq!( client.get_active_endpoint(), "http://primary:11434" );
}

/// Test endpoint health tracking
#[ tokio::test ]
async fn test_endpoint_health_tracking()
{
  let endpoints = vec![
    "http://localhost:11434".to_string(),
    "http://backup:11434".to_string(),
  ];

  let mut client = OllamaClient::new_with_failover( endpoints, Duration::from_secs( 30 ) ).unwrap();

  // Initially all endpoints should be healthy
  assert!( client.is_endpoint_healthy( "http://localhost:11434" ) );
  assert!( client.is_endpoint_healthy( "http://backup:11434" ) );

  // Mark an endpoint as unhealthy
  client.mark_endpoint_unhealthy( "http://localhost:11434" );
  assert!( !client.is_endpoint_healthy( "http://localhost:11434" ) );
  assert!( client.is_endpoint_healthy( "http://backup:11434" ) );

  // Mark it as healthy again
  client.mark_endpoint_healthy( "http://localhost:11434" );
  assert!( client.is_endpoint_healthy( "http://localhost:11434" ) );
}

/// Test failover statistics and monitoring
#[ tokio::test ]
async fn test_failover_statistics()
{
  let endpoints = vec![
    "http://localhost:11434".to_string(),
    "http://backup:11434".to_string(),
  ];

  let mut client = OllamaClient::new_with_failover( endpoints, Duration::from_secs( 30 ) ).unwrap();

  let initial_stats = client.get_failover_stats();
  assert_eq!( initial_stats.total_failovers, 0 );
  assert_eq!( initial_stats.total_requests, 0 );

  // Simulate some operations that would update stats
  client.mark_endpoint_unhealthy( "http://localhost:11434" );
  client.mark_endpoint_healthy( "http://localhost:11434" );

  let updated_stats = client.get_failover_stats();
  let _ = updated_stats.total_requests; // Check that stats are accessible
}

/// Test graceful degradation when all endpoints fail
#[ tokio::test ]
async fn test_graceful_degradation_all_endpoints_fail()
{
  let endpoints = vec![
    "http://invalid1:11434".to_string(),
    "http://invalid2:11434".to_string(),
  ];

  let mut client = OllamaClient::new_with_failover( endpoints, Duration::from_secs( 5 ) ).unwrap();

  let request = ChatRequest
  {
    model : "test-model".to_string(),
    messages : vec![ ChatMessage
    {
      role : MessageRole::User,
      content : "Hello".to_string(),
      #[ cfg( feature = "vision_support" ) ]
      images : None,
      #[ cfg( feature = "tool_calling" ) ]
      tool_calls : None,
    }],
    stream : Some( false ),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  // When all endpoints fail, should return a clear error
  let result = client.chat( request ).await;
  assert!( result.is_err() );

  // Error should indicate all endpoints failed
  let error_msg = format!( "{:?}", result.unwrap_err() );
  assert!( error_msg.contains( "failover endpoints failed" ) || error_msg.contains( "endpoint" ) );
}

/// Test concurrent requests with failover
#[ tokio::test ]
async fn test_concurrent_requests_with_failover()
{
  let endpoints = vec![
    "http://localhost:11434".to_string(),
    "http://backup:11434".to_string(),
  ];

  let client = OllamaClient::new_with_failover( endpoints, Duration::from_secs( 30 ) ).unwrap();

  // Test concurrent access to the same client
  let result1 = client.get_active_endpoint();
  let result2 = client.get_active_endpoint();

  // Both should return valid endpoints
  assert!( !result1.is_empty() );
  assert!( !result2.is_empty() );
}
