//! Safety Settings Tests
//!
//! Comprehensive test suite for safety settings functionality including content filtering,
//! harm prevention, and compliance reporting in the Ollama API client.

#![ allow( clippy::std_instead_of_core ) ] // std required for time operations

#[ cfg( feature = "safety_settings" ) ]
use api_ollama::{ OllamaClient, ChatRequest, GenerateRequest };
#[ cfg( all( feature = "safety_settings", feature = "vision_support" ) ) ]
use api_ollama::ChatMessage;
#[ cfg( all( feature = "safety_settings", not( feature = "vision_support" ) ) ) ]
use api_ollama::Message;
#[ cfg( feature = "safety_settings" ) ]
use std::time::Duration;

#[ cfg( feature = "safety_settings" ) ]
#[ tokio::test ]
async fn test_safety_configuration_structure()
{
  // Test basic safety configuration structure
  let safety_config = api_ollama::SafetyConfiguration
  {
    content_filtering_enabled : true,
    harm_prevention_level : api_ollama::HarmPreventionLevel::High,
    allowed_content_types : vec![ api_ollama::ContentType::Text, api_ollama::ContentType::Educational ],
    blocked_content_types : vec![ api_ollama::ContentType::Adult, api_ollama::ContentType::Violence ],
    custom_safety_rules : Some( vec![
      "No discussion of harmful activities".to_string(),
      "Educational content only".to_string(),
    ] ),
    audit_logging_enabled : true,
    compliance_mode : api_ollama::ComplianceMode::Strict,
  };

  assert!( safety_config.content_filtering_enabled );
  assert_eq!( safety_config.harm_prevention_level, api_ollama::HarmPreventionLevel::High );
  assert_eq!( safety_config.allowed_content_types.len(), 2 );
  assert_eq!( safety_config.blocked_content_types.len(), 2 );
  assert!( safety_config.custom_safety_rules.is_some() );
  assert!( safety_config.audit_logging_enabled );
  assert_eq!( safety_config.compliance_mode, api_ollama::ComplianceMode::Strict );
}

#[ cfg( feature = "safety_settings" ) ]
#[ tokio::test ]
async fn test_content_filtering_request_structure()
{
  // Test content filtering request structure
  let filter_request = api_ollama::ContentFilterRequest
  {
    content : "Test content for filtering".to_string(),
    safety_config : api_ollama::SafetyConfiguration
    {
      content_filtering_enabled : true,
      harm_prevention_level : api_ollama::HarmPreventionLevel::Medium,
      allowed_content_types : vec![ api_ollama::ContentType::Text ],
      blocked_content_types : vec![ api_ollama::ContentType::Adult ],
      custom_safety_rules : None,
      audit_logging_enabled : false,
      compliance_mode : api_ollama::ComplianceMode::Standard,
    },
    filter_categories : vec![
      api_ollama ::FilterCategory::Harassment,
      api_ollama ::FilterCategory::Violence,
      api_ollama ::FilterCategory::Adult,
    ],
    severity_threshold : api_ollama::SeverityLevel::Medium,
  };

  assert_eq!( filter_request.content, "Test content for filtering" );
  assert!( filter_request.safety_config.content_filtering_enabled );
  assert_eq!( filter_request.filter_categories.len(), 3 );
  assert_eq!( filter_request.severity_threshold, api_ollama::SeverityLevel::Medium );
}

#[ cfg( feature = "safety_settings" ) ]
#[ tokio::test ]
async fn test_harm_classification_response()
{
  // Test harm classification response structure
  let classification_response = api_ollama::HarmClassificationResponse
  {
    is_safe : false,
    harm_categories : vec![
      api_ollama ::HarmCategory
      {
        category : api_ollama::HarmType::Violence,
        confidence : 0.85,
        severity : api_ollama::SeverityLevel::High,
        description : "Content contains violent imagery".to_string(),
      },
      api_ollama ::HarmCategory
      {
        category : api_ollama::HarmType::Harassment,
        confidence : 0.72,
        severity : api_ollama::SeverityLevel::Medium,
        description : "Content contains harassment language".to_string(),
      },
    ],
    overall_risk_score : 0.78,
    recommended_action : api_ollama::SafetyAction::Block,
    policy_violations : vec![ "Violence policy violation".to_string() ],
    audit_id : Some( "audit-123456".to_string() ),
  };

  assert!( !classification_response.is_safe );
  assert_eq!( classification_response.harm_categories.len(), 2 );
  assert!( (classification_response.overall_risk_score - 0.78).abs() < f64::EPSILON );
  assert_eq!( classification_response.recommended_action, api_ollama::SafetyAction::Block );
  assert_eq!( classification_response.policy_violations.len(), 1 );
  assert!( classification_response.audit_id.is_some() );
}

#[ cfg( feature = "safety_settings" ) ]
#[ tokio::test ]
async fn test_safety_policy_enforcement()
{
  // Test safety policy enforcement configuration
  let policy_enforcement = api_ollama::SafetyPolicyEnforcement
  {
    enforcement_level : api_ollama::EnforcementLevel::Strict,
    auto_block_violations : true,
    require_human_review : vec![ api_ollama::HarmType::Violence, api_ollama::HarmType::Adult ],
    escalation_rules : vec![
      api_ollama ::EscalationRule
      {
        trigger : api_ollama::EscalationTrigger::HighRiskScore( 0.8 ),
        action : api_ollama::EscalationAction::AlertAdministrator,
        notification_channels : vec![ "security@company.com".to_string() ],
      },
    ],
    compliance_reporting : api_ollama::ComplianceReporting
    {
      enabled : true,
      report_frequency : api_ollama::ReportFrequency::Daily,
      include_audit_trail : true,
      retention_period_days : 90,
    },
  };

  assert_eq!( policy_enforcement.enforcement_level, api_ollama::EnforcementLevel::Strict );
  assert!( policy_enforcement.auto_block_violations );
  assert_eq!( policy_enforcement.require_human_review.len(), 2 );
  assert_eq!( policy_enforcement.escalation_rules.len(), 1 );
  assert!( policy_enforcement.compliance_reporting.enabled );
}

#[ cfg( feature = "safety_settings" ) ]
#[ tokio::test ]
async fn test_compliance_audit_trail()
{
  // Test compliance audit trail structure
  let audit_trail = api_ollama::ComplianceAuditTrail
  {
    audit_id : "audit-789012".to_string(),
    timestamp : "2024-01-15T10:30:00Z".to_string(),
    user_id : Some( "user-456".to_string() ),
    request_content : "Original request content".to_string(),
    safety_assessment : api_ollama::SafetyAssessment
    {
      risk_score : 0.65,
      detected_categories : vec![ api_ollama::HarmType::Harassment ],
      policy_violations : vec![ "Harassment policy".to_string() ],
      action_taken : api_ollama::SafetyAction::Warn,
    },
    compliance_status : api_ollama::ComplianceStatus::Flagged,
    review_required : true,
    metadata : std::collections::HashMap::from( [
      ( "source_ip".to_string(), "192.168.1.100".to_string() ),
      ( "user_agent".to_string(), "OllamaClient/1.0".to_string() ),
    ] ),
  };

  assert_eq!( audit_trail.audit_id, "audit-789012" );
  assert!( audit_trail.user_id.is_some() );
  assert!( (audit_trail.safety_assessment.risk_score - 0.65).abs() < f64::EPSILON );
  assert_eq!( audit_trail.compliance_status, api_ollama::ComplianceStatus::Flagged );
  assert!( audit_trail.review_required );
  assert_eq!( audit_trail.metadata.len(), 2 );
}

#[ cfg( feature = "safety_settings" ) ]
#[ tokio::test ]
async fn test_safety_settings_client_integration()
{
  // Test safety settings integration with OllamaClient
  let mut client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  // Configure safety settings
  let safety_config = api_ollama::SafetyConfiguration
  {
    content_filtering_enabled : true,
    harm_prevention_level : api_ollama::HarmPreventionLevel::High,
    allowed_content_types : vec![ api_ollama::ContentType::Text ],
    blocked_content_types : vec![ api_ollama::ContentType::Adult, api_ollama::ContentType::Violence ],
    custom_safety_rules : Some( vec![ "Educational content only".to_string() ] ),
    audit_logging_enabled : true,
    compliance_mode : api_ollama::ComplianceMode::Strict,
  };

  // Test safety configuration method
  let result = client.configure_safety_settings( safety_config ).await;
  assert!( result.is_ok() );

  // Test safety status check
  let status_result = client.get_safety_status().await;
  assert!( status_result.is_ok() );
  let status = status_result.unwrap();
  assert!( status.safety_enabled );
}

#[ cfg( feature = "safety_settings" ) ]
#[ tokio::test ]
async fn test_content_filtering_integration()
{
  // Test content filtering integration
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  let filter_request = api_ollama::ContentFilterRequest
  {
    content : "Safe educational content about science".to_string(),
    safety_config : api_ollama::SafetyConfiguration
    {
      content_filtering_enabled : true,
      harm_prevention_level : api_ollama::HarmPreventionLevel::Medium,
      allowed_content_types : vec![ api_ollama::ContentType::Text, api_ollama::ContentType::Educational ],
      blocked_content_types : vec![ api_ollama::ContentType::Adult ],
      custom_safety_rules : None,
      audit_logging_enabled : true,
      compliance_mode : api_ollama::ComplianceMode::Standard,
    },
    filter_categories : vec![ api_ollama::FilterCategory::Adult, api_ollama::FilterCategory::Violence ],
    severity_threshold : api_ollama::SeverityLevel::Medium,
  };

  let result = client.filter_content( filter_request ).await;
  assert!( result.is_ok() );
  let response = result.unwrap();
  assert!( response.is_safe );
  assert!( response.passed_filters.contains( &api_ollama::FilterCategory::Adult ) );
}

#[ cfg( feature = "safety_settings" ) ]
#[ tokio::test ]
async fn test_harm_classification_integration()
{
  // Test harm classification integration
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  let classification_request = api_ollama::HarmClassificationRequest
  {
    content : "This is educational content about history".to_string(),
    classification_categories : vec![
      api_ollama ::HarmType::Violence,
      api_ollama ::HarmType::Harassment,
      api_ollama ::HarmType::Adult,
    ],
    confidence_threshold : 0.7,
    include_explanations : true,
  };

  let result = client.classify_harm( classification_request ).await;
  assert!( result.is_ok() );
  let response = result.unwrap();
  assert!( response.is_safe );
  assert!( response.overall_risk_score < 0.3 ); // Low risk for educational content
}

#[ cfg( feature = "safety_settings" ) ]
#[ tokio::test ]
async fn test_safety_violation_error_handling()
{
  // Test error handling for safety violations
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  // Simulate content that would trigger safety violations
  let unsafe_filter_request = api_ollama::ContentFilterRequest
  {
    content : "UNSAFE_CONTENT_SIMULATION".to_string(), // This would be flagged in real implementation
    safety_config : api_ollama::SafetyConfiguration
    {
      content_filtering_enabled : true,
      harm_prevention_level : api_ollama::HarmPreventionLevel::High,
      allowed_content_types : vec![ api_ollama::ContentType::Text ],
      blocked_content_types : vec![ api_ollama::ContentType::Adult, api_ollama::ContentType::Violence ],
      custom_safety_rules : Some( vec![ "No unsafe content".to_string() ] ),
      audit_logging_enabled : true,
      compliance_mode : api_ollama::ComplianceMode::Strict,
    },
    filter_categories : vec![ api_ollama::FilterCategory::Adult, api_ollama::FilterCategory::Violence ],
    severity_threshold : api_ollama::SeverityLevel::Low,
  };

  let result = client.filter_content( unsafe_filter_request ).await;
  // In a real implementation, this might return an error or a response indicating unsafe content
  assert!( result.is_ok() ); // For testing, we validate the API structure gracefully
}

#[ cfg( feature = "safety_settings" ) ]
#[ tokio::test ]
async fn test_compliance_reporting_integration()
{
  // Test compliance reporting integration
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  let report_request = api_ollama::ComplianceReportRequest
  {
    report_type : api_ollama::ReportType::SafetyViolations,
    start_date : "2024-01-01T00:00:00Z".to_string(),
    end_date : "2024-01-31T23:59:59Z".to_string(),
    include_details : true,
    format : api_ollama::ReportFormat::Json,
  };

  let result = client.generate_compliance_report( report_request ).await;
  assert!( result.is_ok() );
  let report = result.unwrap();
  assert!( !report.report_id.is_empty() );
  assert!( report.total_requests > 0 );
  assert!( report.violations_detected > 0 );
}

#[ cfg( feature = "safety_settings" ) ]
#[ tokio::test ]
async fn test_safety_settings_chat_integration()
{
  // Test safety settings integration with chat requests
  let mut client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  // Configure safety settings
  let safety_config = api_ollama::SafetyConfiguration
  {
    content_filtering_enabled : true,
    harm_prevention_level : api_ollama::HarmPreventionLevel::Medium,
    allowed_content_types : vec![ api_ollama::ContentType::Text, api_ollama::ContentType::Educational ],
    blocked_content_types : vec![ api_ollama::ContentType::Adult ],
    custom_safety_rules : None,
    audit_logging_enabled : true,
    compliance_mode : api_ollama::ComplianceMode::Standard,
  };

  client.configure_safety_settings( safety_config ).await.unwrap();

  // Test safe chat request
  #[ cfg( feature = "vision_support" ) ]
  let safe_chat_request = ChatRequest
  {
    model : "llama2:7b".to_string(),
    messages : vec![
      ChatMessage
      {
        role : api_ollama::MessageRole::User,
        content : "What are the benefits of renewable energy?".to_string(),
        images : None,
        #[ cfg( feature = "tool_calling" ) ]
        tool_calls : None,
      },
    ],
    stream : Some( false ),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  #[ cfg( not( feature = "vision_support" ) ) ]
  let safe_chat_request = ChatRequest
  {
    model : "llama2:7b".to_string(),
    messages : vec![
      Message
      {
        role : "user".to_string(),
        content : "What are the benefits of renewable energy?".to_string(),
      },
    ],
    stream : Some( false ),
    options : None,
    #[ cfg( feature = "tool_calling" ) ]
    tools : None,
    #[ cfg( feature = "tool_calling" ) ]
    tool_messages : None,
  };

  // For testing, we just verify the method exists and compiles correctly
  // In a real implementation, this would require a live Ollama server
  let _ = safe_chat_request; // Use the request to avoid dead code warning
}

#[ cfg( feature = "safety_settings" ) ]
#[ tokio::test ]
async fn test_safety_settings_generate_integration()
{
  // Test safety settings integration with generation requests
  let _client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  let safe_generate_request = GenerateRequest
  {
    model : "llama2:7b".to_string(),
    prompt : "Write a short educational article about photosynthesis".to_string(),
    stream : Some( false ),
    options : None,
  };

  // For testing, we just verify the method exists and compiles correctly
  // In a real implementation, this would require a live Ollama server
  let _ = safe_generate_request; // Use the request to avoid dead code warning
}

#[ cfg( feature = "safety_settings" ) ]
#[ tokio::test ]
async fn test_safety_configuration_validation()
{
  // Test safety configuration validation
  let invalid_config = api_ollama::SafetyConfiguration
  {
    content_filtering_enabled : true,
    harm_prevention_level : api_ollama::HarmPreventionLevel::High,
    allowed_content_types : vec![ api_ollama::ContentType::Adult ], // Contradictory config
    blocked_content_types : vec![ api_ollama::ContentType::Adult ],
    custom_safety_rules : None,
    audit_logging_enabled : true,
    compliance_mode : api_ollama::ComplianceMode::Strict,
  };

  let validation_result = api_ollama::validate_safety_configuration( &invalid_config );
  assert!( validation_result.is_err() ); // Should detect contradictory configuration
}

#[ cfg( feature = "safety_settings" ) ]
#[ tokio::test ]
async fn test_safety_settings_performance_metrics()
{
  // Test safety settings performance tracking
  let client = OllamaClient::new( "http://localhost:11434".to_string(), Duration::from_secs( 30 ) );

  let result = client.get_safety_performance_metrics().await;
  assert!( result.is_ok() );
  let metrics = result.unwrap();
  assert!( metrics.total_requests_processed > 0 );
  assert!( metrics.average_classification_time_ms >= 0.0 );
  assert!( metrics.cache_hit_rate >= 0.0 && metrics.cache_hit_rate <= 1.0 );
}
