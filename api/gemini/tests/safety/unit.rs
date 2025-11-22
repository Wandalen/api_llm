mod unit_tests
{
  use super::*;

  #[ test ]
  fn test_advanced_safety_config_structure()
  {
    let rule = SafetyRule {
      id: "rule_001".to_string(),
      name: "Harassment Detection".to_string(),
      category: "HARASSMENT".to_string(),
      condition : RuleCondition {
        content_patterns: vec![ "threat*".to_string(), "bully*".to_string() ],
        risk_threshold: 0.8,
        context_requirements: vec![ "public_forum".to_string() ],
        user_attributes: vec![ "verified_user".to_string() ],
        temporal_constraints: None,
      },
      action : RuleAction {
        action_type: ActionType::Block,
        severity: SeverityLevel::High,
        message: Some( "Content violates harassment policy".to_string() ),
        escalation_required: true,
        custom_response: None,
      },
      priority: 1,
      enabled: true,
      metadata: [ ( "version".to_string(), "1.0".to_string() ) ].iter().cloned().collect(),
    };

    let policy = ContentPolicy {
      id: "policy_001".to_string(),
      name: "Community Guidelines".to_string(),
      description: "Standard community behavior expectations".to_string(),
      policy_type: PolicyType::CommunityGuidelines,
      rules: vec![ "No harassment".to_string(), "No hate speech".to_string() ],
      exceptions: vec![ "Educational context".to_string() ],
      enforcement_level: EnforcementLevel::Required,
    };

    let config = AdvancedSafetyConfig {
      id: "config_001".to_string(),
      name: "Advanced Moderation Config".to_string(),
      description: Some( "Comprehensive safety configuration".to_string() ),
      rules: vec![ rule.clone() ],
      custom_models: vec![],
      policy_framework : PolicyFramework {
        id: "framework_001".to_string(),
        name: "Standard Framework".to_string(),
        policies: vec![ policy ],
        compliance_standards: vec![],
        jurisdiction: "US".to_string(),
        effective_date: "2024-01-01".to_string(),
        review_schedule: "quarterly".to_string(),
      },
      audit_settings : AuditSettings {
        enabled: true,
        log_level: LogLevel::Detailed,
        retention_period: 90,
        real_time_monitoring: true,
        alert_thresholds : AlertThresholds {
          violation_count: 10,
          risk_score: 0.8,
          time_window: 3600,
          escalation_levels: vec![ "admin".to_string(), "security".to_string() ],
        },
        export_formats: vec![ "json".to_string(), "csv".to_string() ],
      },
      created_at: "2024-01-01T10:00:00Z".to_string(),
      updated_at: "2024-01-01T10:00:00Z".to_string(),
      status: SafetyConfigStatus::Active,
    };

    assert_eq!( config.id, "config_001" );
    assert_eq!( config.rules.len(), 1 );
    assert_eq!( rule.action.action_type, ActionType::Block );
    assert_eq!( rule.action.severity, SeverityLevel::High );
    assert!( rule.enabled );
    assert_eq!( config.status, SafetyConfigStatus::Active );
  }

  #[ test ]
  fn test_safety_rule_condition_validation()
  {
    let temporal_constraints = TemporalConstraints {
      time_windows: vec![ "9:00-17:00".to_string() ],
      frequency_limits: [ ( "hourly".to_string(), 5 ) ].iter().cloned().collect(),
      cooldown_periods: [ ( "violation".to_string(), 300 ) ].iter().cloned().collect(),
    };

    let condition = RuleCondition {
      content_patterns: vec![
      "hate*".to_string(),
      "discriminat*".to_string(),
      "threat*".to_string(),
      ],
      risk_threshold: 0.75,
      context_requirements: vec![
      "public_content".to_string(),
      "user_generated".to_string(),
      ],
      user_attributes: vec![
      "age_verified".to_string(),
      "account_standing".to_string(),
      ],
      temporal_constraints: Some( temporal_constraints.clone() ),
    };

    assert_eq!( condition.content_patterns.len(), 3 );
    assert_eq!( condition.risk_threshold, 0.75 );
    assert_eq!( condition.context_requirements.len(), 2 );
    assert_eq!( condition.user_attributes.len(), 2 );
    assert!( condition.temporal_constraints.is_some() );

    let temp = condition.temporal_constraints.unwrap();
    assert_eq!( temp.time_windows.len(), 1 );
    assert_eq!( temp.frequency_limits.get( "hourly" ), Some( &5 ) );
    assert_eq!( temp.cooldown_periods.get( "violation" ), Some( &300 ) );
  }

  #[ test ]
  fn test_custom_safety_model_structure()
  {
    let performance = ModelPerformance {
      accuracy: 0.95,
      precision: 0.92,
      recall: 0.88,
      f1_score: 0.90,
      false_positive_rate: 0.08,
      false_negative_rate: 0.12,
    };

    let model = CustomSafetyModel {
      id: "model_001".to_string(),
      name: "Advanced Harassment Detector".to_string(),
      model_type: SafetyModelType::TransformerBased,
      categories: vec![
      "HARASSMENT".to_string(),
      "CYBERBULLYING".to_string(),
      "TOXIC_LANGUAGE".to_string(),
      ],
      confidence_threshold: 0.85,
      training_data_source: "curated_corpus_v2".to_string(),
      version: "2.1.0".to_string(),
      performance_metrics: performance.clone(),
    };

    assert_eq!( model.id, "model_001" );
    assert_eq!( model.model_type, SafetyModelType::TransformerBased );
    assert_eq!( model.categories.len(), 3 );
    assert_eq!( model.confidence_threshold, 0.85 );
    assert_eq!( performance.accuracy, 0.95 );
    assert_eq!( performance.f1_score, 0.90 );
  }

  #[ test ]
  fn test_policy_framework_validation()
  {
    let compliance_standard = ComplianceStandard {
      standard_id: "GDPR_2018".to_string(),
      name: "General Data Protection Regulation".to_string(),
      framework: "EU Data Protection".to_string(),
      version: "2018".to_string(),
      requirements: vec![
      "Data minimization".to_string(),
      "Consent management".to_string(),
      "Right to deletion".to_string(),
      ],
      audit_frequency: "annual".to_string(),
    };

    let content_policy = ContentPolicy {
      id: "policy_privacy".to_string(),
      name: "Data Privacy Policy".to_string(),
      description: "Protects user privacy and personal information".to_string(),
      policy_type: PolicyType::DataPrivacy,
      rules: vec![
      "No collection of personal data without consent".to_string(),
      "Data retention limited to necessity".to_string(),
      ],
      exceptions: vec![ "Legal compliance requirements".to_string() ],
      enforcement_level: EnforcementLevel::Mandatory,
    };

    let framework = PolicyFramework {
      id: "framework_eu".to_string(),
      name: "EU Compliance Framework".to_string(),
      policies: vec![ content_policy.clone() ],
      compliance_standards: vec![ compliance_standard.clone() ],
      jurisdiction: "EU".to_string(),
      effective_date: "2024-01-01".to_string(),
      review_schedule: "annual".to_string(),
    };

    assert_eq!( framework.policies.len(), 1 );
    assert_eq!( framework.compliance_standards.len(), 1 );
    assert_eq!( content_policy.policy_type, PolicyType::DataPrivacy );
    assert_eq!( content_policy.enforcement_level, EnforcementLevel::Mandatory );
    assert_eq!( compliance_standard.requirements.len(), 3 );
  }

  #[ test ]
  fn test_safety_analysis_request_structure()
  {
    let context = AnalysisContext {
      user_demographics: [
      ( "age_group".to_string(), "adult".to_string() ),
      ( "region".to_string(), "north_america".to_string() ),
      ].iter().cloned().collect(),
      application_context: "social_media_platform".to_string(),
      interaction_history: vec![
      "previous_violation_warning".to_string(),
      "community_participation".to_string(),
      ],
      regional_settings: "en_US".to_string(),
      compliance_requirements: vec![
      "COPPA".to_string(),
      "CCPA".to_string(),
      ],
    };

    let request = SafetyAnalysisRequest {
      content: "Sample content for analysis".to_string(),
      content_type: ContentType::Text,
      context: context.clone(),
      analysis_depth: AnalysisDepth::Comprehensive,
      custom_rules: vec![ "rule_001".to_string(), "rule_002".to_string() ],
    };

    assert_eq!( request.content_type, ContentType::Text );
    assert_eq!( request.analysis_depth, AnalysisDepth::Comprehensive );
    assert_eq!( request.custom_rules.len(), 2 );
    assert_eq!( context.user_demographics.len(), 2 );
    assert_eq!( context.compliance_requirements.len(), 2 );
  }

  #[ test ]
  fn test_safety_analysis_result_structure()
  {
    let violation = PolicyViolation {
      policy_id: "policy_harassment".to_string(),
      severity: SeverityLevel::Medium,
      description: "Content contains potentially harassing language".to_string(),
      evidence: vec![
      "Pattern match: aggressive language".to_string(),
      "Context: directed at individual".to_string(),
      ],
      suggested_actions: vec![
      "Content warning".to_string(),
      "User education".to_string(),
      ],
      auto_remediation: Some( "Apply content filter".to_string() ),
    };

    let result = SafetyAnalysisResult {
      overall_risk_score: 0.72,
      category_scores: [
      ( "HARASSMENT".to_string(), 0.68 ),
      ( "HATE_SPEECH".to_string(), 0.23 ),
      ( "DANGEROUS_CONTENT".to_string(), 0.15 ),
      ].iter().cloned().collect(),
      policy_violations: vec![ violation.clone() ],
      recommendations: vec![
      "Apply content warning".to_string(),
      "Monitor user behavior".to_string(),
      ],
      confidence_score: 0.89,
      processing_time_ms: 245,
      model_versions: vec![
      "harassment_detector_v2.1".to_string(),
      "context_analyzer_v1.5".to_string(),
      ],
    };

    assert_eq!( result.overall_risk_score, 0.72 );
    assert_eq!( result.category_scores.len(), 3 );
    assert_eq!( result.policy_violations.len(), 1 );
    assert_eq!( violation.severity, SeverityLevel::Medium );
    assert_eq!( violation.evidence.len(), 2 );
    assert_eq!( result.confidence_score, 0.89 );
  }

  #[ test ]
  fn test_audit_settings_and_logging()
  {
    let alert_thresholds = AlertThresholds {
      violation_count: 5,
      risk_score: 0.7,
      time_window: 1800, // 30 minutes
      escalation_levels: vec![
      "moderator".to_string(),
      "admin".to_string(),
      "security_team".to_string(),
      ],
    };

    let audit_settings = AuditSettings {
      enabled: true,
      log_level: LogLevel::Comprehensive,
      retention_period: 365, // 1 year
      real_time_monitoring: true,
      alert_thresholds: alert_thresholds.clone(),
      export_formats: vec![
      "json".to_string(),
      "csv".to_string(),
      "pdf".to_string(),
      ],
    };

    let audit_log = SafetyAuditLog {
      id: "audit_001".to_string(),
      timestamp: "2024-01-01T10:00:00Z".to_string(),
      event_type: AuditEventType::PolicyViolationDetected,
      content_hash: "sha256:abc123...".to_string(),
      safety_result : SafetyAnalysisResult {
        overall_risk_score: 0.85,
        category_scores: HashMap::new(),
        policy_violations: vec![],
        recommendations: vec![],
        confidence_score: 0.92,
        processing_time_ms: 150,
        model_versions: vec![],
      },
      action_taken: ActionType::Flag,
      user_context: [ ( "user_id".to_string(), "user_123".to_string() ) ].iter().cloned().collect(),
      metadata: [ ( "source".to_string(), "content_moderation".to_string() ) ].iter().cloned().collect(),
    };

    assert!( audit_settings.enabled );
    assert_eq!( audit_settings.log_level, LogLevel::Comprehensive );
    assert_eq!( audit_settings.retention_period, 365 );
    assert_eq!( alert_thresholds.escalation_levels.len(), 3 );
    assert_eq!( audit_log.event_type, AuditEventType::PolicyViolationDetected );
    assert_eq!( audit_log.action_taken, ActionType::Flag );
  }

  #[ test ]
  fn test_batch_moderation_request_structure()
  {
    let content_items = vec![
    ContentItem {
      id: "item_001".to_string(),
      content: "First content item".to_string(),
      content_type: ContentType::Text,
      metadata: [ ( "source".to_string(), "user_post".to_string() ) ].iter().cloned().collect(),
    },
    ContentItem {
      id: "item_002".to_string(),
      content: "Second content item".to_string(),
      content_type: ContentType::Text,
      metadata: [ ( "source".to_string(), "comment".to_string() ) ].iter().cloned().collect(),
    },
    ];

    let batch_request = BatchModerationRequest {
      content_items: content_items.clone(),
      safety_config_id: "config_batch".to_string(),
      batch_size: 100,
      parallel_processing: true,
    };

    assert_eq!( batch_request.content_items.len(), 2 );
    assert_eq!( batch_request.batch_size, 100 );
    assert!( batch_request.parallel_processing );
    assert_eq!( content_items[ 0 ].content_type, ContentType::Text );
    assert_eq!( content_items[ 1 ].metadata.get( "source" ), Some( &"comment".to_string() ) );
  }
}
}
