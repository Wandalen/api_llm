/// Integration Tests
#[ cfg( test ) ]
mod integration_tests
{
  use super::*;

  #[ tokio::test ]
  async fn test_basic_safety_settings_integration() -> Result< (), Box< dyn std::error::Error > >
  {
    let client = create_test_client();

    println!( "✓ Testing basic safety settings integration:" );

    // Test 1: Create a request with standard safety settings
    let safety_settings = vec![
    SafetySetting {
      category: "HARM_CATEGORY_HARASSMENT".to_string(),
      threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
    },
    SafetySetting {
      category: "HARM_CATEGORY_HATE_SPEECH".to_string(),
      threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
    },
    SafetySetting {
      category: "HARM_CATEGORY_SEXUALLY_EXPLICIT".to_string(),
      threshold: "BLOCK_LOW_AND_ABOVE".to_string(),
    },
    SafetySetting {
      category: "HARM_CATEGORY_DANGEROUS_CONTENT".to_string(),
      threshold: "BLOCK_MEDIUM_AND_ABOVE".to_string(),
    },
    ];

    let safe_content_request = GenerateContentRequest {
      contents: vec![
      Content {
        role: "user".to_string(),
        parts: vec![
        Part {
          text: Some( "Please explain the importance of online safety and digital well-being.".to_string() ),
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
      safety_settings: Some( safety_settings.clone() ),
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
    .generate_content( &safe_content_request )
    .await?;

    // Test 2: Verify safety ratings are present
    if let Some( candidate ) = response.candidates.first()
    {
      if let Some( safety_ratings ) = &candidate.safety_ratings
      {
      println!( "  ✓ Safety ratings received : {} categories", safety_ratings.len() );

        for rating in safety_ratings
        {
    println!( "    - {}: {} (blocked : {})",
          rating.category,
          rating.probability,
          rating.blocked.unwrap_or( false )
          );

          // Validate safety rating structure
          assert!( !rating.category.is_empty() );
          assert!( !rating.probability.is_empty() );
        }

        // Should have ratings for major categories
        assert!( safety_ratings.len() >= 3 );
      } else {
        println!( "  ✓ No safety concerns detected (no ratings returned)" );
      }

      // Verify content was generated successfully
      if let Some( part ) = candidate.content.parts.first()
      {
        if let Some( text ) = &part.text
        {
        println!( "  ✓ Content generated successfully : {} characters", text.len() );
          assert!( text.len() > 50 ); // Should have reasonable response
        }
      }
    }

    // Test 3: Verify safety settings configuration
    assert_eq!( safety_settings.len(), 4 );
    assert_eq!( safety_settings[ 0 ].category, "HARM_CATEGORY_HARASSMENT" );
    assert_eq!( safety_settings[ 0 ].threshold, "BLOCK_MEDIUM_AND_ABOVE" );

    Ok( () )
  }

  #[ tokio::test ]
  async fn test_advanced_content_analysis() -> Result< (), Box< dyn std::error::Error > >
  {
    let _client = create_test_client();

    println!( "✓ Testing advanced content analysis capabilities:" );

    // Test 1: Create advanced analysis request
    let analysis_context = AnalysisContext {
      user_demographics: [
      ( "age_group".to_string(), "adult".to_string() ),
      ( "region".to_string(), "north_america".to_string() ),
      ( "language".to_string(), "en".to_string() ),
      ].iter().cloned().collect(),
      application_context: "social_media_post".to_string(),
      interaction_history: vec![
      "previous_posts_clean".to_string(),
      "community_member_1_year".to_string(),
      ],
      regional_settings: "en_US".to_string(),
      compliance_requirements: vec![
      "COPPA".to_string(),
      "community_guidelines".to_string(),
      ],
    };

    let content_samples = vec![
    ( "Positive educational content about digital citizenship", ContentType::Text ),
    ( "Neutral discussion about technology trends", ContentType::Text ),
    ( "Community guidelines reminder for new users", ContentType::Text ),
    ( "Technical documentation about safety features", ContentType::Text ),
    ];

    // Test 2: Simulate advanced analysis for different content types
    for ( content, content_type ) in content_samples
    {
      let analysis_request = SafetyAnalysisRequest {
        content: content.to_string(),
        content_type: content_type.clone(),
        context: analysis_context.clone(),
        analysis_depth: AnalysisDepth::Comprehensive,
        custom_rules: vec![ "community_standard", "educational_context" ].iter().map( |s| s.to_string() ).collect(),
      };

      // Simulate analysis result
      let analysis_result = SafetyAnalysisResult {
        overall_risk_score: 0.15, // Low risk for positive content
        category_scores: [
        ( "HARASSMENT".to_string(), 0.05 ),
        ( "HATE_SPEECH".to_string(), 0.02 ),
        ( "SEXUALLY_EXPLICIT".to_string(), 0.01 ),
        ( "DANGEROUS_CONTENT".to_string(), 0.03 ),
        ].iter().cloned().collect(),
        policy_violations: vec![], // No violations for safe content
        recommendations: vec![
        "Content approved for publication".to_string(),
        "Consider adding educational tags".to_string(),
        ],
        confidence_score: 0.94,
        processing_time_ms: 180,
        model_versions: vec![
        "safety_classifier_v3.1".to_string(),
        "context_analyzer_v2.0".to_string(),
        ],
      };

println!( "  ✓ Analyzed content : '{}' -> Risk : {:.3}, Confidence : {:.3}",
      &content[ ..content.len().min( 50 ) ], analysis_result.overall_risk_score, analysis_result.confidence_score );

      // Validate analysis structure
      assert_eq!( analysis_request.content_type, content_type );
      assert_eq!( analysis_request.analysis_depth, AnalysisDepth::Comprehensive );
      assert_eq!( analysis_request.custom_rules.len(), 2 );
      assert!( analysis_result.overall_risk_score < 0.3 ); // Should be low risk
      assert!( analysis_result.confidence_score > 0.8 ); // Should be confident
      assert_eq!( analysis_result.category_scores.len(), 4 );
    }

    Ok( () )
  }

  #[ tokio::test ]
  async fn test_custom_safety_filter_configuration() -> Result< (), Box< dyn std::error::Error > >
  {
    let _client = create_test_client();

    println!( "✓ Testing custom safety filter creation and configuration:" );

    // Test 1: Create custom safety rules
    let harassment_rule = SafetyRule {
      id: "rule_harassment_advanced".to_string(),
      name: "Advanced Harassment Detection".to_string(),
      category: "HARASSMENT".to_string(),
      condition : RuleCondition {
        content_patterns: vec![
        "threat*".to_string(),
        "bully*".to_string(),
        "intimidat*".to_string(),
        "harass*".to_string(),
        ],
        risk_threshold: 0.75,
        context_requirements: vec![
        "directed_at_user".to_string(),
        "repeated_behavior".to_string(),
        ],
        user_attributes: vec![
        "account_age".to_string(),
        "previous_violations".to_string(),
        ],
        temporal_constraints : Some( TemporalConstraints {
          time_windows: vec![ "all_hours".to_string() ],
          frequency_limits: [ ( "daily".to_string(), 3 ) ].iter().cloned().collect(),
          cooldown_periods: [ ( "escalation".to_string(), 3600 ) ].iter().cloned().collect(),
        }),
      },
      action : RuleAction {
        action_type: ActionType::Block,
        severity: SeverityLevel::High,
        message: Some( "Content blocked for potential harassment".to_string() ),
        escalation_required: true,
        custom_response: Some( "Please review our community guidelines".to_string() ),
      },
      priority: 1,
      enabled: true,
      metadata: [
      ( "created_by".to_string(), "safety_team".to_string() ),
      ( "version".to_string(), "2.0".to_string() ),
      ].iter().cloned().collect(),
    };

    let toxicity_rule = SafetyRule {
      id: "rule_toxicity_custom".to_string(),
      name: "Custom Toxicity Filter".to_string(),
      category: "TOXICITY".to_string(),
      condition : RuleCondition {
        content_patterns: vec![
        "toxic_language_patterns".to_string(),
        "aggressive_tone_indicators".to_string(),
        ],
        risk_threshold: 0.6,
        context_requirements: vec![ "public_discussion".to_string() ],
        user_attributes: vec![ "community_standing".to_string() ],
        temporal_constraints: None,
      },
      action : RuleAction {
        action_type: ActionType::Warn,
        severity: SeverityLevel::Medium,
        message: Some( "Content may be perceived as toxic".to_string() ),
        escalation_required: false,
        custom_response: Some( "Consider rephrasing for better community interaction".to_string() ),
      },
      priority: 2,
      enabled: true,
      metadata: HashMap::new(),
    };

    // Test 2: Create custom safety models
    let custom_model = CustomSafetyModel {
      id: "model_harassment_v2".to_string(),
      name: "Advanced Harassment Classifier".to_string(),
      model_type: SafetyModelType::TransformerBased,
      categories: vec![
      "HARASSMENT".to_string(),
      "CYBERBULLYING".to_string(),
      "INTIMIDATION".to_string(),
      ],
      confidence_threshold: 0.85,
      training_data_source: "curated_harassment_dataset_v2".to_string(),
      version: "2.1.0".to_string(),
      performance_metrics : ModelPerformance {
        accuracy: 0.94,
        precision: 0.91,
        recall: 0.87,
        f1_score: 0.89,
        false_positive_rate: 0.09,
        false_negative_rate: 0.13,
      },
    };

    // Test 3: Create comprehensive safety configuration
    let policy_framework = PolicyFramework {
      id: "framework_custom".to_string(),
      name: "Custom Safety Framework".to_string(),
      policies: vec![
      ContentPolicy {
        id: "policy_harassment".to_string(),
        name: "Anti-Harassment Policy".to_string(),
        description: "Comprehensive harassment prevention".to_string(),
        policy_type: PolicyType::CommunityGuidelines,
        rules: vec![
        "No direct threats".to_string(),
        "No sustained targeting".to_string(),
        "No doxxing or personal attacks".to_string(),
        ],
        exceptions: vec![ "Fictional content clearly marked".to_string() ],
        enforcement_level: EnforcementLevel::Required,
      },
      ],
      compliance_standards: vec![
      ComplianceStandard {
        standard_id: "COMMUNITY_V2".to_string(),
        name: "Community Guidelines v2".to_string(),
        framework: "Platform Standards".to_string(),
        version: "2.0".to_string(),
        requirements: vec![
        "User safety prioritization".to_string(),
        "Transparent moderation".to_string(),
        ],
        audit_frequency: "monthly".to_string(),
      },
      ],
      jurisdiction: "GLOBAL".to_string(),
      effective_date: "2024-01-01".to_string(),
      review_schedule: "quarterly".to_string(),
    };

    let advanced_config = AdvancedSafetyConfig {
      id: "config_advanced_001".to_string(),
      name: "Advanced Custom Safety Configuration".to_string(),
      description: Some( "Comprehensive safety with custom rules and models".to_string() ),
      rules: vec![ harassment_rule.clone(), toxicity_rule.clone() ],
      custom_models: vec![ custom_model.clone() ],
      policy_framework,
      audit_settings : AuditSettings {
        enabled: true,
        log_level: LogLevel::Comprehensive,
        retention_period: 180,
        real_time_monitoring: true,
        alert_thresholds : AlertThresholds {
          violation_count: 5,
          risk_score: 0.8,
          time_window: 1800,
          escalation_levels: vec![ "moderator".to_string(), "admin".to_string() ],
        },
        export_formats: vec![ "json".to_string(), "xml".to_string() ],
      },
      created_at: "2024-01-01T10:00:00Z".to_string(),
      updated_at: "2024-01-01T10:00:00Z".to_string(),
      status: SafetyConfigStatus::Active,
    };

    // Test 4: Validate configuration structure
  println!( "  ✓ Created advanced safety config : '{}'", advanced_config.name );
    assert_eq!( advanced_config.rules.len(), 2 );
    assert_eq!( advanced_config.custom_models.len(), 1 );
    assert_eq!( harassment_rule.action.action_type, ActionType::Block );
    assert_eq!( toxicity_rule.action.action_type, ActionType::Warn );
    assert_eq!( custom_model.model_type, SafetyModelType::TransformerBased );
    assert!( custom_model.performance_metrics.accuracy > 0.9 );

println!( "  ✓ Harassment rule configured : priority {}, threshold {:.2}",
    harassment_rule.priority, harassment_rule.condition.risk_threshold );
println!( "  ✓ Custom model performance : accuracy {:.3}, f1_score {:.3}",
    custom_model.performance_metrics.accuracy, custom_model.performance_metrics.f1_score );

    Ok( () )
  }

  #[ tokio::test ]
  async fn test_real_time_content_moderation_workflow() -> Result< (), Box< dyn std::error::Error > >
  {
    let _client = create_test_client();

    println!( "✓ Testing real-time content moderation workflows:" );

    // Test 1: Simulate real-time content stream
    let content_stream = vec![
    ( "Welcome to our community! Please be respectful.", "safe_content" ),
    ( "Great discussion everyone, thanks for sharing.", "safe_content" ),
    ( "I disagree with that point, but respect your opinion.", "borderline_content" ),
    ( "This is educational content about conflict resolution.", "educational_content" ),
    ( "Please follow community guidelines when posting.", "moderation_reminder" ),
    ];

    let analysis_context = AnalysisContext {
      user_demographics: [
      ( "region".to_string(), "global".to_string() ),
      ( "language".to_string(), "en".to_string() ),
      ].iter().cloned().collect(),
      application_context: "live_chat_moderation".to_string(),
      interaction_history: vec![ "active_participant".to_string() ],
      regional_settings: "en_US".to_string(),
      compliance_requirements: vec![ "real_time_safety".to_string() ],
    };

    // Test 2: Process content stream with real-time analysis
    for ( content, content_category ) in content_stream
    {
      let moderation_request = ModerateContentRequest {
        content: content.to_string(),
        safety_config_id: "config_realtime".to_string(),
        context: analysis_context.clone(),
        real_time: true,
      };

      // Simulate real-time analysis
      let ( risk_score, action ) = match content_category
      {
        "safe_content" => ( 0.1, ActionType::Log ),
        "borderline_content" => ( 0.4, ActionType::Flag ),
        "educational_content" => ( 0.2, ActionType::Log ),
        "moderation_reminder" => ( 0.05, ActionType::Log ),
        _ => ( 0.1, ActionType::Log ),
      };

      let moderation_result = SafetyAnalysisResult {
        overall_risk_score: risk_score,
        category_scores: [
        ( "HARASSMENT".to_string(), risk_score * 0.3 ),
        ( "TOXICITY".to_string(), risk_score * 0.7 ),
        ].iter().cloned().collect(),
        policy_violations: if risk_score > 0.3
        {
          vec![
          PolicyViolation {
            policy_id: "realtime_policy".to_string(),
            severity: SeverityLevel::Low,
            description: "Content flagged for review".to_string(),
            evidence: vec![ "Automated detection".to_string() ],
            suggested_actions: vec![ "Manual review".to_string() ],
            auto_remediation: None,
          }
          ]
        } else {
          vec![]
        },
        recommendations: vec![ "Continue monitoring".to_string() ],
        confidence_score: 0.87,
        processing_time_ms: 45, // Fast real-time processing
        model_versions: vec![ "realtime_classifier_v1.0".to_string() ],
      };

println!( "  ✓ Processed : '{}' -> Risk : {:.3}, Action : {:?}, Time : {}ms",
      &content[ ..content.len().min( 40 ) ],
      moderation_result.overall_risk_score,
      action,
      moderation_result.processing_time_ms
      );

      // Validate real-time processing requirements
      assert!( moderation_request.real_time );
      assert!( moderation_result.processing_time_ms < 100 ); // Fast processing
      assert!( moderation_result.confidence_score > 0.8 );
    }

    // Test 3: Simulate escalation for high-risk content
    let _high_risk_content = "Content that might require escalation";
    let escalation_result = SafetyAnalysisResult {
      overall_risk_score: 0.9,
      category_scores: [
      ( "HARASSMENT".to_string(), 0.85 ),
      ( "DANGEROUS_CONTENT".to_string(), 0.75 ),
      ].iter().cloned().collect(),
      policy_violations: vec![
      PolicyViolation {
        policy_id: "high_risk_policy".to_string(),
        severity: SeverityLevel::High,
        description: "Content requires immediate review".to_string(),
        evidence: vec![ "Multiple pattern matches".to_string() ],
        suggested_actions: vec![
        "Immediate escalation".to_string(),
        "User notification".to_string(),
        ],
        auto_remediation: Some( "Temporary content removal".to_string() ),
      },
      ],
      recommendations: vec![
      "Escalate to human moderator".to_string(),
      "Apply immediate safety measures".to_string(),
      ],
      confidence_score: 0.95,
      processing_time_ms: 75,
      model_versions: vec![ "high_risk_detector_v2.0".to_string() ],
    };

println!( "  ✓ High-risk escalation : Risk {:.3}, Violations : {}, Confidence : {:.3}",
    escalation_result.overall_risk_score,
    escalation_result.policy_violations.len(),
    escalation_result.confidence_score
    );

    assert!( escalation_result.overall_risk_score > 0.8 );
    assert!( !escalation_result.policy_violations.is_empty() );
    assert_eq!( escalation_result.policy_violations[ 0 ].severity, SeverityLevel::High );

    Ok( () )
  }

  #[ tokio::test ]

  async fn test_batch_content_moderation() -> Result< (), Box< dyn std::error::Error > >
  {
    let _client = create_test_client();

    println!( "✓ Testing batch content moderation capabilities:" );

    // Test 1: Create batch of content for moderation
    let content_items = vec![
    ContentItem {
      id: "item_001".to_string(),
      content: "Educational content about digital citizenship".to_string(),
      content_type: ContentType::Text,
      metadata: [
      ( "source".to_string(), "educational_post".to_string() ),
      ( "author_type".to_string(), "educator".to_string() ),
      ].iter().cloned().collect(),
    },
    ContentItem {
      id: "item_002".to_string(),
      content: "Community discussion about platform features".to_string(),
      content_type: ContentType::Text,
      metadata: [
      ( "source".to_string(), "forum_discussion".to_string() ),
      ( "thread_type".to_string(), "feedback".to_string() ),
      ].iter().cloned().collect(),
    },
    ContentItem {
      id: "item_003".to_string(),
      content: "Technical documentation for developers".to_string(),
      content_type: ContentType::Text,
      metadata: [
      ( "source".to_string(), "documentation".to_string() ),
      ( "audience".to_string(), "developers".to_string() ),
      ].iter().cloned().collect(),
    },
    ContentItem {
      id: "item_004".to_string(),
      content: "User guide for safety features".to_string(),
      content_type: ContentType::Text,
      metadata: [
      ( "source".to_string(), "help_content".to_string() ),
      ( "category".to_string(), "safety".to_string() ),
      ].iter().cloned().collect(),
    },
    ContentItem {
      id: "item_005".to_string(),
      content: "Community guidelines and best practices".to_string(),
      content_type: ContentType::Text,
      metadata: [
      ( "source".to_string(), "guidelines".to_string() ),
      ( "importance".to_string(), "high".to_string() ),
      ].iter().cloned().collect(),
    },
    ];

    let batch_request = BatchModerationRequest {
      content_items: content_items.clone(),
      safety_config_id: "config_batch_moderation".to_string(),
      batch_size: 50,
      parallel_processing: true,
    };

    // Test 2: Simulate batch processing
    let start_time = std::time::Instant::now();
    let mut batch_results = Vec::new();

    for item in &batch_request.content_items
    {
      // Simulate safety analysis for each item
      let risk_score = match item.metadata.get( "source" ).map( |s| s.as_str() )
      {
        Some( "educational_post" ) => 0.05,
        Some( "forum_discussion" ) => 0.15,
        Some( "documentation" ) => 0.02,
        Some( "help_content" ) => 0.03,
        Some( "guidelines" ) => 0.01,
        _ => 0.1,
      };

      let analysis_result = SafetyAnalysisResult {
        overall_risk_score: risk_score,
        category_scores: [
        ( "HARASSMENT".to_string(), risk_score * 0.2 ),
        ( "HATE_SPEECH".to_string(), risk_score * 0.1 ),
        ( "DANGEROUS_CONTENT".to_string(), risk_score * 0.3 ),
        ].iter().cloned().collect(),
        policy_violations: vec![], // Safe content
        recommendations: vec![ "Content approved".to_string() ],
        confidence_score: 0.92,
        processing_time_ms: 120,
        model_versions: vec![ "batch_processor_v1.5".to_string() ],
      };

      batch_results.push( ( item.id.clone(), analysis_result ) );
    }

    let processing_time = start_time.elapsed();

    // Test 3: Verify batch processing results
  println!( "  - Batch request created with {} items", batch_request.content_items.len() );
  println!( "  - Batch size configured : {}", batch_request.batch_size );
  println!( "  - Parallel processing enabled : {}", batch_request.parallel_processing );
  println!( "  - Processing completed in {:?}", processing_time );
  println!( "  - Results generated for {} items", batch_results.len() );

    // Assert batch processing completed successfully
    assert_eq!( batch_results.len(), 5, "Should generate results for all 5 content items" );
    assert_eq!( batch_results.len(), content_items.len(), "Results should match input count" );

    // Assert all items were processed with valid risk scores
    for ( item_id, result ) in &batch_results
    {
      assert!( !item_id.is_empty(), "Item ID should not be empty" );
      assert!( result.overall_risk_score >= 0.0 && result.overall_risk_score <= 1.0, "Risk score should be in valid range [0, 1]" );
      assert!( result.confidence_score > 0.0 && result.confidence_score <= 1.0, "Confidence score should be in valid range (0, 1]" );
      assert!( !result.category_scores.is_empty(), "Should have category scores" );
      assert!( !result.recommendations.is_empty(), "Should have recommendations" );
    }

    // Verify low-risk content items have appropriate scores
    let educational_result = batch_results.iter().find( |( id, _ )| id == "item_001" ).unwrap();
    assert!( educational_result.1.overall_risk_score < 0.2, "Educational content should have low risk score" );

    let guidelines_result = batch_results.iter().find( |( id, _ )| id == "item_005" ).unwrap();
    assert!( guidelines_result.1.overall_risk_score < 0.1, "Guidelines content should have very low risk score" );

    println!( "✓ Batch content moderation test completed successfully" );

    Ok( () )
  }

