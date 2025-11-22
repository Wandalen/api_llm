//! Quota Management Module Tests
//!
//! Tests for request quotas, rate limiting, and usage enforcement functionality.

use api_openai::enterprise::
{
  QuotaStatus,
  QuotaReservation,
  RequestMetadata,
  QuotaUsageDetails,
  ConcurrentUsageDetails,
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
    estimated_tokens : 500,
    model : "gpt-4".to_string(),
    request_type : "chat".to_string(),
    priority : 1,
    user_id : Some( "user123".to_string() ),
  };

  assert_eq!( metadata.user_id, Some( "user123".to_string() ) );
  assert_eq!( metadata.model, "gpt-4" );
  assert_eq!( metadata.estimated_tokens, 500 );
  assert_eq!( metadata.priority, 1 );
}

#[ tokio::test ]
async fn test_quota_reservation_structure()
{
  let reservation = QuotaReservation
  {
    concurrent : true,
    timestamp : std::time::Instant::now(),
    estimated_tokens : Some( 1000 ),
  };

  assert!( reservation.concurrent );
  assert_eq!( reservation.estimated_tokens, Some( 1000 ) );
}

#[ tokio::test ]
#[ allow( clippy::float_cmp ) ]
async fn test_quota_usage_details()
{
  let usage_details = QuotaUsageDetails
  {
    requests_used : 150,
    requests_limit : Some( 1000 ),
    tokens_used : 75000,
    tokens_limit : Some( 500_000 ),
    usage_percentage : 0.15,
    time_until_reset_seconds : 3600,
  };

  assert_eq!( usage_details.requests_used, 150 );
  assert_eq!( usage_details.requests_limit, Some( 1000 ) );
  assert_eq!( usage_details.tokens_used, 75000 );
  assert_eq!( usage_details.usage_percentage, 0.15 );
}

#[ tokio::test ]
#[ allow( clippy::float_cmp ) ]
async fn test_concurrent_usage_details()
{
  let concurrent_details = ConcurrentUsageDetails
  {
    current_requests : 5,
    max_requests : Some( 10 ),
    peak_requests : 8,
    average_requests : 3.5,
  };

  assert_eq!( concurrent_details.current_requests, 5 );
  assert_eq!( concurrent_details.max_requests, Some( 10 ) );
  assert_eq!( concurrent_details.peak_requests, 8 );
  assert_eq!( concurrent_details.average_requests, 3.5 );
}

#[ tokio::test ]
async fn test_quota_status_variants()
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
    let deserialized : QuotaStatus = serde_json::from_str( &json ).expect( "Deserialization should work" );
    assert_eq!( status, deserialized );
  }
}

#[ tokio::test ]
async fn test_request_metadata_serialization()
{
  let metadata = RequestMetadata
  {
    estimated_tokens : 500,
    model : "gpt-4".to_string(),
    request_type : "chat".to_string(),
    priority : 2,
    user_id : Some( "user_test".to_string() ),
  };

  let json = serde_json::to_string( &metadata ).expect( "Serialization should work" );
  let deserialized : RequestMetadata = serde_json::from_str( &json ).expect( "Deserialization should work" );

  assert_eq!( metadata.user_id, deserialized.user_id );
  assert_eq!( metadata.model, deserialized.model );
  assert_eq!( metadata.estimated_tokens, deserialized.estimated_tokens );
  assert_eq!( metadata.request_type, deserialized.request_type );
}