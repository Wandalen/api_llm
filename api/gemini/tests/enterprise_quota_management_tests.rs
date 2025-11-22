//! Enterprise Quota Management Tests
//!
//! Comprehensive tests for request quotas, rate limiting, and usage enforcement functionality.

use api_gemini::enterprise::
{
  QuotaManager,
  QuotaStatus,
  RequestMetadata,
};

#[ tokio::test ]
async fn test_quota_status_creation()
{
  let status = QuotaStatus::Allowed;
  assert_eq!( status, QuotaStatus::Allowed );

  let status2 = QuotaStatus::DailyQuotaExceeded;
  assert_eq!( status2, QuotaStatus::DailyQuotaExceeded );
}

#[ tokio::test ]
async fn test_request_metadata_creation()
{
  let metadata = RequestMetadata
  {
    estimated_tokens: 500,
    model: "gemini-1.5-pro".to_string(),
    request_type: "chat".to_string(),
    priority: 1,
    user_id: Some( "user123".to_string() ),
  };

  assert_eq!( metadata.user_id, Some( "user123".to_string() ) );
  assert_eq!( metadata.model, "gemini-1.5-pro" );
  assert_eq!( metadata.estimated_tokens, 500 );
  assert_eq!( metadata.priority, 1 );
}

#[ tokio::test ]
async fn test_quota_manager_creation()
{
  let manager = QuotaManager::new( Some( 1000 ), Some( 100 ), Some( 10 ), None, None );

  // Verify manager created successfully
  let usage = manager.get_quota_usage().unwrap();
  assert_eq!( usage.daily.requests_used, 0 );
  assert_eq!( usage.hourly.requests_used, 0 );
  assert_eq!( usage.concurrent.current_requests, 0 );
}

#[ tokio::test ]
async fn test_quota_check_allowed()
{
  let manager = QuotaManager::new( Some( 1000 ), Some( 100 ), Some( 10 ), None, None );
  let request = RequestMetadata
  {
    estimated_tokens: 50,
    model: "gemini-1.5-flash".to_string(),
    request_type: "chat".to_string(),
    priority: 5,
    user_id: Some( "user123".to_string() ),
  };

  let status = manager.check_quota( &request ).unwrap();
  assert_eq!( status, QuotaStatus::Allowed );
}

#[ tokio::test ]
async fn test_quota_reservation()
{
  let manager = QuotaManager::new( Some( 1000 ), Some( 100 ), Some( 10 ), None, None );
  let request = RequestMetadata
  {
    estimated_tokens: 50,
    model: "gemini-1.5-pro".to_string(),
    request_type: "chat".to_string(),
    priority: 5,
    user_id: Some( "user123".to_string() ),
  };

  let reservation = manager.reserve_quota( &request ).unwrap();
  assert!( reservation.concurrent );
  assert_eq!( reservation.estimated_tokens, Some( 50 ) );

  // Check that counters were updated
  let usage = manager.get_quota_usage().unwrap();
  assert_eq!( usage.daily.requests_used, 1 );
  assert_eq!( usage.concurrent.current_requests, 1 );
}

#[ tokio::test ]
async fn test_quota_release()
{
  let manager = QuotaManager::new( Some( 1000 ), Some( 100 ), Some( 10 ), None, None );
  let request = RequestMetadata
  {
    estimated_tokens: 50,
    model: "gemini-1.5-flash".to_string(),
    request_type: "embedding".to_string(),
    priority: 5,
    user_id: Some( "user456".to_string() ),
  };

  let reservation = manager.reserve_quota( &request ).unwrap();

  // Verify concurrent count increased
  let usage_before = manager.get_quota_usage().unwrap();
  assert_eq!( usage_before.concurrent.current_requests, 1 );

  // Release reservation
  manager.release_quota( &reservation );

  // Verify concurrent count decreased
  let usage_after = manager.get_quota_usage().unwrap();
  assert_eq!( usage_after.concurrent.current_requests, 0 );
}

#[ tokio::test ]
async fn test_quota_usage_tracking()
{
  let manager = QuotaManager::new( Some( 1000 ), Some( 100 ), Some( 10 ), None, None );

  for i in 0..5
  {
    let request = RequestMetadata
    {
      estimated_tokens: 50,
      model: "gemini-1.5-flash".to_string(),
      request_type: "chat".to_string(),
      priority: 5,
    user_id : Some( format!( "user{i}" ) ),
    };

    let reservation = manager.reserve_quota( &request ).unwrap();
    manager.release_quota( &reservation );
  }

  let usage = manager.get_quota_usage().unwrap();
  assert_eq!( usage.daily.requests_used, 5 );
  assert_eq!( usage.per_user.len(), 5 );
}

#[ tokio::test ]
async fn test_daily_quota_exceeded()
{
  let manager = QuotaManager::new( Some( 5 ), None, None, None, None );

  // Reserve 5 requests (at limit)
  for i in 0..5
  {
    let request = RequestMetadata
    {
      estimated_tokens: 50,
      model: "gemini-1.5-flash".to_string(),
      request_type: "chat".to_string(),
      priority: 5,
    user_id : Some( format!( "user{i}" ) ),
    };
    let _ = manager.reserve_quota( &request ).unwrap();
  }

  // 6th request should exceed quota
  let request = RequestMetadata
  {
    estimated_tokens: 50,
    model: "gemini-1.5-flash".to_string(),
    request_type: "chat".to_string(),
    priority: 5,
    user_id: Some( "user_overflow".to_string() ),
  };

  let status = manager.check_quota( &request ).unwrap();
  assert_eq!( status, QuotaStatus::DailyQuotaExceeded );
}

#[ tokio::test ]
async fn test_hourly_quota_exceeded()
{
  let manager = QuotaManager::new( None, Some( 3 ), None, None, None );

  // Reserve 3 requests (at hourly limit)
  for i in 0..3
  {
    let request = RequestMetadata
    {
      estimated_tokens: 50,
      model: "gemini-1.5-flash".to_string(),
      request_type: "chat".to_string(),
      priority: 5,
    user_id : Some( format!( "user{i}" ) ),
    };
    let _ = manager.reserve_quota( &request ).unwrap();
  }

  // 4th request should exceed hourly quota
  let request = RequestMetadata
  {
    estimated_tokens: 50,
    model: "gemini-1.5-flash".to_string(),
    request_type: "chat".to_string(),
    priority: 5,
    user_id: Some( "user_overflow".to_string() ),
  };

  let status = manager.check_quota( &request ).unwrap();
  assert_eq!( status, QuotaStatus::HourlyQuotaExceeded );
}

#[ tokio::test ]
async fn test_concurrent_limit_exceeded()
{
  let manager = QuotaManager::new( None, None, Some( 2 ), None, None );

  // Reserve 2 concurrent requests (at limit)
  let request1 = RequestMetadata
  {
    estimated_tokens: 50,
    model: "gemini-1.5-flash".to_string(),
    request_type: "chat".to_string(),
    priority: 5,
    user_id: Some( "user1".to_string() ),
  };
  let _res1 = manager.reserve_quota( &request1 ).unwrap();

  let request2 = RequestMetadata
  {
    estimated_tokens: 50,
    model: "gemini-1.5-flash".to_string(),
    request_type: "chat".to_string(),
    priority: 5,
    user_id: Some( "user2".to_string() ),
  };
  let _res2 = manager.reserve_quota( &request2 ).unwrap();

  // 3rd concurrent request should exceed limit
  let request3 = RequestMetadata
  {
    estimated_tokens: 50,
    model: "gemini-1.5-flash".to_string(),
    request_type: "chat".to_string(),
    priority: 5,
    user_id: Some( "user3".to_string() ),
  };

  let status = manager.check_quota( &request3 ).unwrap();
  assert_eq!( status, QuotaStatus::ConcurrentLimitExceeded );
}

#[ tokio::test ]
async fn test_token_limit_enforcement()
{
  let manager = QuotaManager::new( None, None, None, Some( 200 ), None );

  // Reserve requests with total 200 tokens (at limit)
  for i in 0..4
  {
    let request = RequestMetadata
    {
      estimated_tokens: 50,
      model: "gemini-1.5-flash".to_string(),
      request_type: "chat".to_string(),
      priority: 5,
    user_id : Some( format!( "user{i}" ) ),
    };
    let _ = manager.reserve_quota( &request ).unwrap();
  }

  // Next request would exceed daily token limit
  let request = RequestMetadata
  {
    estimated_tokens: 50,
    model: "gemini-1.5-flash".to_string(),
    request_type: "chat".to_string(),
    priority: 5,
    user_id: Some( "user_overflow".to_string() ),
  };

  let status = manager.check_quota( &request ).unwrap();
  assert_eq!( status, QuotaStatus::DailyQuotaExceeded );
}

#[ tokio::test ]
#[ allow( clippy::float_cmp ) ]
async fn test_usage_percentage_calculation()
{
  let manager = QuotaManager::new( Some( 100 ), None, None, None, None );

  // Reserve 50 daily requests (50%)
  for i in 0..50
  {
    let request = RequestMetadata
    {
      estimated_tokens: 50,
      model: "gemini-1.5-flash".to_string(),
      request_type: "chat".to_string(),
      priority: 5,
    user_id : Some( format!( "user{i}" ) ),
    };
    let reservation = manager.reserve_quota( &request ).unwrap();
    manager.release_quota( &reservation );
  }

  let usage = manager.get_quota_usage().unwrap();
  assert_eq!( usage.daily.usage_percentage, 0.5 ); // 50/100 = 0.5
}

#[ tokio::test ]
async fn test_per_user_tracking()
{
  let manager = QuotaManager::new( Some( 1000 ), None, None, None, None );

  // User A makes 5 requests
  for _ in 0..5
  {
    let request = RequestMetadata
    {
      estimated_tokens: 50,
      model: "gemini-1.5-flash".to_string(),
      request_type: "chat".to_string(),
      priority: 5,
      user_id: Some( "user_a".to_string() ),
    };
    let reservation = manager.reserve_quota( &request ).unwrap();
    manager.release_quota( &reservation );
  }

  // User B makes 3 requests
  for _ in 0..3
  {
    let request = RequestMetadata
    {
      estimated_tokens: 50,
      model: "gemini-1.5-flash".to_string(),
      request_type: "chat".to_string(),
      priority: 5,
      user_id: Some( "user_b".to_string() ),
    };
    let reservation = manager.reserve_quota( &request ).unwrap();
    manager.release_quota( &reservation );
  }

  let usage = manager.get_quota_usage().unwrap();

  // Verify per-user usage
  let user_a = usage.per_user.get( "user_a" ).unwrap();
  assert_eq!( user_a.daily_requests, 5 );
  assert_eq!( user_a.usage_rank, 1 ); // User A has highest usage

  let user_b = usage.per_user.get( "user_b" ).unwrap();
  assert_eq!( user_b.daily_requests, 3 );
  assert_eq!( user_b.usage_rank, 2 ); // User B has second highest usage
}

#[ tokio::test ]
async fn test_quota_status_serialization()
{
  let statuses = vec![
  QuotaStatus::Allowed,
  QuotaStatus::DailyQuotaExceeded,
  QuotaStatus::HourlyQuotaExceeded,
  QuotaStatus::ConcurrentLimitExceeded,
  QuotaStatus::RateLimitExceeded,
  ];

  // Test serialization/deserialization
  for status in statuses
  {
    let json = serde_json::to_string( &status ).expect( "Serialization should work" );
    let deserialized: QuotaStatus = serde_json::from_str( &json ).expect( "Deserialization should work" );
    assert_eq!( status, deserialized );
  }
}

#[ tokio::test ]
async fn test_request_metadata_serialization()
{
  let metadata = RequestMetadata
  {
    estimated_tokens: 500,
    model: "gemini-1.5-pro".to_string(),
    request_type: "chat".to_string(),
    priority: 2,
    user_id: Some( "user_test".to_string() ),
  };

  let json = serde_json::to_string( &metadata ).expect( "Serialization should work" );
  let deserialized: RequestMetadata = serde_json::from_str( &json ).expect( "Deserialization should work" );

  assert_eq!( metadata.user_id, deserialized.user_id );
  assert_eq!( metadata.model, deserialized.model );
  assert_eq!( metadata.estimated_tokens, deserialized.estimated_tokens );
  assert_eq!( metadata.request_type, deserialized.request_type );
}

#[ tokio::test ]
#[ allow( clippy::float_cmp ) ]
async fn test_efficiency_metrics_calculation()
{
  let manager = QuotaManager::new( Some( 100 ), None, None, None, None );

  // Make 10 requests with 50 tokens each
  for i in 0..10
  {
    let request = RequestMetadata
    {
      estimated_tokens: 50,
      model: "gemini-1.5-flash".to_string(),
      request_type: "chat".to_string(),
      priority: 5,
    user_id : Some( format!( "user{i}" ) ),
    };
    let reservation = manager.reserve_quota( &request ).unwrap();
    manager.release_quota( &reservation );
  }

  let usage = manager.get_quota_usage().unwrap();

  // Verify efficiency metrics
  assert_eq!( usage.efficiency_metrics.avg_tokens_per_request, 50.0 ); // 500 tokens / 10 requests
  assert_eq!( usage.efficiency_metrics.quota_utilization, 0.1 ); // 10/100 = 0.1
}
