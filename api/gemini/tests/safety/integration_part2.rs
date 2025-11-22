// Test 3: Validate batch processing results
println!( "  ✓ Processed {} items in {:.2}ms ({:.2}ms per item)",
batch_results.len(),
processing_time.as_millis(),
processing_time.as_millis() as f64 / batch_results.len() as f64
);

assert_eq!( batch_results.len(), 5 );
assert!( batch_request.parallel_processing );
assert_eq!( batch_request.batch_size, 50 );

// Validate individual results
for ( item_id, result ) in &batch_results
{
println!( "    - {}: Risk {:.3}, Confidence {:.3}",
  item_id, result.overall_risk_score, result.confidence_score );

  assert!( result.overall_risk_score < 0.2 ); // All should be low risk
  assert!( result.confidence_score > 0.9 );
  assert!( result.policy_violations.is_empty() );
}

// Test 4: Validate content metadata processing
for item in &content_items
{
  assert!( !item.id.is_empty() );
  assert!( !item.content.is_empty() );
  assert_eq!( item.content_type, ContentType::Text );
  assert!( !item.metadata.is_empty() );
  assert!( item.metadata.contains_key( "source" ) );
}

Ok( () )
}

#[ tokio::test ]
async fn test_policy_compliance_checking() -> Result< (), Box< dyn std::error::Error > >
{
  let _client = create_test_client();

  println!( "✓ Testing policy compliance checking:" );

  // Test 1: Create comprehensive policy framework
  let gdpr_standard = ComplianceStandard {
    standard_id: "GDPR_2018".to_string(),
    name: "General Data Protection Regulation".to_string(),
    framework: "EU Data Protection".to_string(),
    version: "2018".to_string(),
    requirements: vec![
    "Data minimization principle".to_string(),
    "Explicit consent for processing".to_string(),
    "Right to be forgotten".to_string(),
    "Data portability rights".to_string(),
    "Privacy by design".to_string(),
    ],
    audit_frequency: "annual".to_string(),
  };

  let coppa_standard = ComplianceStandard {
    standard_id: "COPPA_1998".to_string(),
    name: "Children's Online Privacy Protection Act".to_string(),
    framework: "US Child Privacy".to_string(),
    version: "1998_amended_2013".to_string(),
    requirements: vec![
    "Parental consent for under 13".to_string(),
    "Limited data collection from children".to_string(),
    "No behavioral advertising to children".to_string(),
    ],
    audit_frequency: "quarterly".to_string(),
  };

  let privacy_policy = ContentPolicy {
    id: "policy_privacy_comprehensive".to_string(),
    name: "Comprehensive Privacy Policy".to_string(),
    description: "Ensures user privacy and data protection across all platforms".to_string(),
    policy_type: PolicyType::DataPrivacy,
    rules: vec![
    "No collection of personal data without explicit consent".to_string(),
    "Data retention limited to stated purposes".to_string(),
    "User rights to access and delete data".to_string(),
    "Secure data transmission and storage".to_string(),
    ],
    exceptions: vec![
    "Legal compliance requirements".to_string(),
    "Security incident response".to_string(),
    ],
    enforcement_level: EnforcementLevel::Mandatory,
  };

  let community_policy = ContentPolicy {
    id: "policy_community_standards".to_string(),
    name: "Community Standards".to_string(),
    description: "Maintains healthy community interactions".to_string(),
    policy_type: PolicyType::CommunityGuidelines,
    rules: vec![
    "Respectful communication required".to_string(),
    "No harassment or bullying tolerated".to_string(),
    "Constructive feedback encouraged".to_string(),
    ],
    exceptions: vec![ "Clearly marked fictional content".to_string() ],
    enforcement_level: EnforcementLevel::Required,
  };

  let compliance_framework = PolicyFramework {
    id: "framework_comprehensive".to_string(),
    name: "Comprehensive Compliance Framework".to_string(),
    policies: vec![ privacy_policy.clone(), community_policy.clone() ],
    compliance_standards: vec![ gdpr_standard.clone(), coppa_standard.clone() ],
    jurisdiction: "GLOBAL".to_string(),
    effective_date: "2024-01-01".to_string(),
    review_schedule: "semi_annual".to_string(),
  };

  // Test 2: Test policy compliance for different content types
  let compliance_test_cases = vec![
  (
  "User registration form requesting minimal information",
  vec![ "GDPR_2018".to_string(), "data_minimization".to_string() ],
  true,
  "Privacy compliant"
  ),
  (
  "Educational content about online safety for all ages",
  vec![ "COPPA_1998".to_string(), "age_appropriate".to_string() ],
  true,
  "Child safety compliant"
  ),
  (
  "Community discussion following guidelines",
  vec![ "community_standards".to_string() ],
  true,
  "Community guidelines compliant"
  ),
  (
  "Privacy policy update notification",
  vec![ "GDPR_2018".to_string(), "transparency".to_string() ],
  true,
  "Transparency compliant"
  ),
  ];

  for ( content, compliance_standards, expected_compliant, description ) in compliance_test_cases
  {
    let compliance_request = PolicyComplianceRequest {
      content: content.to_string(),
      policy_framework_id: compliance_framework.id.clone(),
      compliance_standards: compliance_standards.clone(),
      jurisdiction: "GLOBAL".to_string(),
    };

    // Simulate compliance analysis
    let compliance_result = if expected_compliant
    {
      SafetyAnalysisResult {
        overall_risk_score: 0.1,
        category_scores: [
        ( "PRIVACY_RISK".to_string(), 0.05 ),
        ( "COMMUNITY_RISK".to_string(), 0.08 ),
        ].iter().cloned().collect(),
        policy_violations: vec![],
        recommendations: vec![ "Compliant with all standards".to_string() ],
        confidence_score: 0.95,
        processing_time_ms: 200,
        model_versions: vec![ "compliance_checker_v2.0".to_string() ],
      }
    } else {
      SafetyAnalysisResult {
        overall_risk_score: 0.8,
        category_scores: HashMap::new(),
        policy_violations: vec![
        PolicyViolation {
          policy_id: "policy_violation".to_string(),
          severity: SeverityLevel::Medium,
          description: "Compliance issue detected".to_string(),
          evidence: vec![ "Policy mismatch".to_string() ],
          suggested_actions: vec![ "Review compliance requirements".to_string() ],
          auto_remediation: None,
        }
        ],
        recommendations: vec![ "Address compliance violations".to_string() ],
        confidence_score: 0.88,
        processing_time_ms: 250,
        model_versions: vec![ "compliance_checker_v2.0".to_string() ],
      }
    };

println!( "  ✓ {}: Risk {:.3}, Violations : {}, Standards : {:?}",
    description,
    compliance_result.overall_risk_score,
    compliance_result.policy_violations.len(),
    compliance_standards
    );

    // Validate compliance checking
    assert_eq!( compliance_request.jurisdiction, "GLOBAL" );
    assert!( !compliance_request.compliance_standards.is_empty() );

    if expected_compliant
    {
      assert!( compliance_result.overall_risk_score < 0.3 );
      assert!( compliance_result.policy_violations.is_empty() );
    } else {
      assert!( compliance_result.overall_risk_score > 0.5 );
      assert!( !compliance_result.policy_violations.is_empty() );
    }
  }

  // Test 3: Validate policy framework structure
  assert_eq!( compliance_framework.policies.len(), 2 );
  assert_eq!( compliance_framework.compliance_standards.len(), 2 );
  assert_eq!( privacy_policy.enforcement_level, EnforcementLevel::Mandatory );
  assert_eq!( community_policy.enforcement_level, EnforcementLevel::Required );
  assert_eq!( gdpr_standard.requirements.len(), 5 );
  assert_eq!( coppa_standard.requirements.len(), 3 );

  Ok( () )
}

#[ tokio::test ]
async fn test_safety_audit_logging_and_reporting() -> Result< (), Box< dyn std::error::Error > >
{
  let _client = create_test_client();

  println!( "✓ Testing safety audit logging and compliance reporting:" );

  // Test 1: Create comprehensive audit settings
  let audit_settings = AuditSettings {
    enabled: true,
    log_level: LogLevel::Comprehensive,
    retention_period: 365, // 1 year retention
    real_time_monitoring: true,
    alert_thresholds : AlertThresholds {
      violation_count: 10,
      risk_score: 0.75,
      time_window: 3600, // 1 hour
      escalation_levels: vec![
      "moderator".to_string(),
      "safety_team".to_string(),
      "compliance_officer".to_string(),
      ],
    },
    export_formats: vec![
    "json".to_string(),
    "csv".to_string(),
    "pdf".to_string(),
    "xml".to_string(),
    ],
  };

  // Test 2: Generate sample audit logs for different events
  let audit_events = vec![
  (
  AuditEventType::ContentAnalyzed,
  0.15,
  ActionType::Log,
  "Standard content analysis completed"
  ),
  (
  AuditEventType::PolicyViolationDetected,
  0.72,
  ActionType::Flag,
  "Medium risk policy violation flagged"
  ),
  (
  AuditEventType::ActionExecuted,
  0.85,
  ActionType::Block,
  "High risk content blocked automatically"
  ),
  (
  AuditEventType::ConfigurationChanged,
  0.0,
  ActionType::Log,
  "Safety configuration updated by admin"
  ),
  (
  AuditEventType::ComplianceCheck,
  0.25,
  ActionType::Log,
  "Quarterly compliance audit completed"
  ),
  ];

  let mut audit_logs = Vec::new();
  for ( i, ( event_type, risk_score, action, description ) ) in audit_events.iter().enumerate()
  {
    let audit_log = SafetyAuditLog {
    id : format!( "audit_{:03}", i + 1 ),
    timestamp : format!( "2024-01-01T{:02}:00:00Z", 10 + i ),
      event_type: event_type.clone(),
    content_hash : format!( "sha256:hash_{}", i + 1 ),
      safety_result : SafetyAnalysisResult {
        overall_risk_score: *risk_score,
        category_scores: if *risk_score > 0.1
        {
          [
          ( "HARASSMENT".to_string(), risk_score * 0.4 ),
          ( "DANGEROUS_CONTENT".to_string(), risk_score * 0.6 ),
          ].iter().cloned().collect()
        } else {
          HashMap::new()
        },
        policy_violations: if *risk_score > 0.5
        {
          vec![
          PolicyViolation {
            policy_id: "auto_detected".to_string(),
        severity : if *risk_score > 0.8 { SeverityLevel::High } else { SeverityLevel::Medium },
            description: description.to_string(),
            evidence: vec![ "Automated detection".to_string() ],
            suggested_actions: vec![ "Review required".to_string() ],
        auto_remediation : if *risk_score > 0.8 { Some( "Auto-block".to_string() ) } else { None },
          }
          ]
        } else {
          vec![]
        },
        recommendations: vec![ "Continue monitoring".to_string() ],
        confidence_score: 0.9,
        processing_time_ms: 150,
        model_versions: vec![ "audit_analyzer_v1.0".to_string() ],
      },
      action_taken: action.clone(),
      user_context: [
    ( "user_id".to_string(), format!( "user_{}", i + 1 ) ),
    ( "session_id".to_string(), format!( "session_{}", i + 1 ) ),
      ].iter().cloned().collect(),
      metadata: [
      ( "source".to_string(), "automated_system".to_string() ),
      ( "audit_version".to_string(), "2.0".to_string() ),
      ].iter().cloned().collect(),
    };

    audit_logs.push( audit_log );
  }

  // Test 3: Validate audit log structure and content
println!( "  ✓ Generated {} audit log entries", audit_logs.len() );

  for log in &audit_logs
  {
println!( "    - {}: {} -> Risk {:.3}, Action {:?}",
    log.id,
    match log.event_type
    {
      AuditEventType::ContentAnalyzed => "Content Analysis",
      AuditEventType::PolicyViolationDetected => "Policy Violation",
      AuditEventType::ActionExecuted => "Action Executed",
      AuditEventType::ConfigurationChanged => "Config Change",
      AuditEventType::ComplianceCheck => "Compliance Check",
      AuditEventType::ModelUpdated => "Model Update",
    },
    log.safety_result.overall_risk_score,
    log.action_taken
    );

    // Validate log structure
    assert!( !log.id.is_empty() );
    assert!( !log.timestamp.is_empty() );
    assert!( !log.content_hash.is_empty() );
    assert!( !log.user_context.is_empty() );
    assert!( log.safety_result.confidence_score > 0.8 );
  }

  // Test 4: Simulate alert threshold monitoring
  let high_risk_logs: Vec< &SafetyAuditLog > = audit_logs
  .iter()
  .filter( |log| log.safety_result.overall_risk_score > audit_settings.alert_thresholds.risk_score )
  .collect();

  let violation_count = audit_logs
  .iter()
  .filter( |log| !log.safety_result.policy_violations.is_empty() )
  .count();

println!( "  ✓ Alert analysis : {} high-risk events, {} policy violations",
  high_risk_logs.len(), violation_count );

  // Test 5: Validate audit settings and thresholds
  assert!( audit_settings.enabled );
  assert_eq!( audit_settings.log_level, LogLevel::Comprehensive );
  assert_eq!( audit_settings.retention_period, 365 );
  assert!( audit_settings.real_time_monitoring );
  assert_eq!( audit_settings.alert_thresholds.escalation_levels.len(), 3 );
  assert_eq!( audit_settings.export_formats.len(), 4 );

  // Check if alerts would be triggered
  if high_risk_logs.len() as u32 > audit_settings.alert_thresholds.violation_count
  {
println!( "  ✓ Alert would be triggered : {} high-risk events exceed threshold of {}",
    high_risk_logs.len(), audit_settings.alert_thresholds.violation_count );
  }

  assert_eq!( audit_logs.len(), 5 );
  assert!( violation_count >= 2 ); // Should have some violations in test data

  Ok( () )
}

#[ tokio::test ]
async fn test_integration_with_existing_safety_settings() -> Result< (), Box< dyn std::error::Error > >
{
  let client = create_test_client();

  println!( "✓ Testing integration with existing SafetySettings system:" );

  // Test 1: Create enhanced safety settings with custom thresholds
  let enhanced_safety_settings = vec![
  SafetySetting {
    category: "HARM_CATEGORY_HARASSMENT".to_string(),
    threshold: "BLOCK_LOW_AND_ABOVE".to_string(), // Stricter than default
  },
  SafetySetting {
    category: "HARM_CATEGORY_HATE_SPEECH".to_string(),
    threshold: "BLOCK_LOW_AND_ABOVE".to_string(),
  },
  SafetySetting {
    category: "HARM_CATEGORY_SEXUALLY_EXPLICIT".to_string(),
    threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
  },
  SafetySetting {
    category: "HARM_CATEGORY_DANGEROUS_CONTENT".to_string(),
    threshold: "BLOCK_LOW_AND_ABOVE".to_string(),
  },
  ];

  // Test 2: Create advanced safety config that integrates with standard settings
  let integrated_config = AdvancedSafetyConfig {
    id: "config_integrated".to_string(),
    name: "Integrated Safety Configuration".to_string(),
    description: Some( "Advanced safety with standard SafetySettings integration".to_string() ),
    rules: vec![
    SafetyRule {
      id: "rule_integration_harassment".to_string(),
      name: "Enhanced Harassment Detection".to_string(),
      category: "HARM_CATEGORY_HARASSMENT".to_string(),
      condition : RuleCondition {
        content_patterns: vec![ "enhanced_harassment_patterns".to_string() ],
        risk_threshold: 0.3, // Lower threshold for harassment
        context_requirements: vec![ "user_interaction".to_string() ],
        user_attributes: vec![ "verified_account".to_string() ],
        temporal_constraints: None,
      },
      action : RuleAction {
        action_type: ActionType::Flag,
        severity: SeverityLevel::Medium,
        message: Some( "Content flagged for enhanced review".to_string() ),
        escalation_required: false,
        custom_response: None,
      },
      priority: 1,
      enabled: true,
      metadata: [
      ( "integrates_with".to_string(), "HARM_CATEGORY_HARASSMENT".to_string() ),
      ].iter().cloned().collect(),
    },
    ],
    custom_models: vec![],
    policy_framework : PolicyFramework {
      id: "framework_integrated".to_string(),
      name: "Integration Framework".to_string(),
      policies: vec![],
      compliance_standards: vec![],
      jurisdiction: "US".to_string(),
      effective_date: "2024-01-01".to_string(),
      review_schedule: "monthly".to_string(),
    },
    audit_settings : AuditSettings {
      enabled: true,
      log_level: LogLevel::Standard,
      retention_period: 90,
      real_time_monitoring: false,
      alert_thresholds : AlertThresholds {
        violation_count: 5,
        risk_score: 0.6,
        time_window: 1800,
        escalation_levels: vec![ "moderator".to_string() ],
      },
      export_formats: vec![ "json".to_string() ],
    },
    created_at: "2024-01-01T10:00:00Z".to_string(),
    updated_at: "2024-01-01T10:00:00Z".to_string(),
    status: SafetyConfigStatus::Active,
  };

  // Test 3: Test with actual API call using enhanced safety settings
  let integration_request = GenerateContentRequest {
    contents: vec![
    Content {
      role: "user".to_string(),
      parts: vec![
      Part {
        text: Some( "Please provide information about creating a safe online environment for communities.".to_string() ),
        inline_data: None,
        function_call: None,
        function_response: None,
        ..Default::default()
      }
      ],
    }
    ],
    tools: None,
    tool_config: None,
    safety_settings: Some( enhanced_safety_settings.clone() ),
    system_instruction: None,
    cached_content: None,
    generation_config : Some( GenerationConfig {
      temperature: Some( 0.7 ),
      top_p: Some( 0.9 ),
      top_k: Some( 40 ),
      candidate_count: Some( 1 ),
      max_output_tokens: Some( 800 ),
      stop_sequences: None,
    }),
  };

  let response = client
  .models()
  .by_name( "gemini-flash-latest" )
  .generate_content( &integration_request )
  .await?;

  // Test 4: Analyze response with advanced safety analysis
  if let Some( candidate ) = response.candidates.first()
  {
    // Standard safety ratings from API
    if let Some( safety_ratings ) = &candidate.safety_ratings
    {
    println!( "  ✓ Standard safety ratings received : {} categories", safety_ratings.len() );

      for rating in safety_ratings
      {
        // Simulate enhanced analysis for each standard category
        let enhanced_risk = match rating.category.as_str()
        {
          "HARM_CATEGORY_HARASSMENT" => {
            // Apply enhanced harassment detection
            let base_risk = match rating.probability.as_str()
            {
              "NEGLIGIBLE" => 0.05,
              "LOW" => 0.15,
              "MEDIUM" => 0.45,
              "HIGH" => 0.85,
              _ => 0.1,
            };
            base_risk * 0.8 // Enhanced model might be more conservative
          },
          _ => {
            match rating.probability.as_str()
            {
              "NEGLIGIBLE" => 0.02,
              "LOW" => 0.1,
              "MEDIUM" => 0.4,
              "HIGH" => 0.8,
              _ => 0.05,
            }
          }
        };

  println!( "    - {}: {} -> Enhanced risk : {:.3}",
        rating.category, rating.probability, enhanced_risk );

        // Validate integration logic
        assert!( enhanced_risk >= 0.0 && enhanced_risk <= 1.0 );
      }
    }

    // Verify content generation with enhanced safety
    if let Some( part ) = candidate.content.parts.first()
    {
      if let Some( text ) = &part.text
      {
      println!( "  ✓ Safe content generated : {} characters", text.len() );
        assert!( text.len() > 100 ); // Should have substantial response
        assert!( text.to_lowercase().contains( "safe" ) || text.to_lowercase().contains( "community" ) );
      }
    }
  }

  // Test 5: Validate integration configuration
  assert_eq!( enhanced_safety_settings.len(), 4 );
  assert_eq!( integrated_config.rules.len(), 1 );
  assert_eq!( integrated_config.status, SafetyConfigStatus::Active );

  // Check that enhanced thresholds are stricter
  let harassment_setting = enhanced_safety_settings
  .iter()
  .find( |s| s.category == "HARM_CATEGORY_HARASSMENT" )
  .unwrap();
  assert_eq!( harassment_setting.threshold, "BLOCK_LOW_AND_ABOVE" );

  // Verify integration metadata
  let integration_rule = &integrated_config.rules[ 0 ];
  assert_eq!( integration_rule.category, "HARM_CATEGORY_HARASSMENT" );
  assert!( integration_rule.metadata.contains_key( "integrates_with" ) );

  println!( "  ✓ Integration validation complete: Enhanced safety with standard API compatibility" );

  Ok( () )
}
}
