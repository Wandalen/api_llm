//! Safety Settings functionality for the Ollama API client
//!
//! This module provides comprehensive safety settings capabilities including:
//! - Content filtering and moderation controls
//! - Harm classification and prevention mechanisms
//! - User safety configuration options
//! - Safety policy enforcement capabilities
//! - Compliance reporting and audit trails
//!
//! All functionality follows the "Thin Client, Rich API" governing principle,
//! providing explicit control with transparent safety operation management.

use serde::{ Serialize, Deserialize };
use core::time::Duration;
use std::collections::HashMap;

/// Safety configuration for content filtering and harm prevention
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub struct SafetyConfiguration
{
  /// Whether content filtering is enabled
  pub content_filtering_enabled : bool,
  /// Level of harm prevention enforcement
  pub harm_prevention_level : HarmPreventionLevel,
  /// Types of content that are explicitly allowed
  pub allowed_content_types : Vec< ContentType >,
  /// Types of content that are explicitly blocked
  pub blocked_content_types : Vec< ContentType >,
  /// Custom safety rules defined by the user
  pub custom_safety_rules : Option< Vec< String > >,
  /// Whether audit logging is enabled
  pub audit_logging_enabled : bool,
  /// Compliance mode for regulatory requirements
  pub compliance_mode : ComplianceMode,
}

/// Levels of harm prevention enforcement
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub enum HarmPreventionLevel
{
  /// Low - minimal filtering
  Low,
  /// Medium - balanced filtering
  Medium,
  /// High - strict filtering
  High,
  /// Maximum - most restrictive filtering
  Maximum,
}

/// Types of content for classification
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub enum ContentType
{
  /// General text content
  Text,
  /// Educational content
  Educational,
  /// Adult content
  Adult,
  /// Violent content
  Violence,
  /// Medical content
  Medical,
  /// Legal content
  Legal,
  /// Financial content
  Financial,
}

/// Compliance mode for regulatory requirements
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub enum ComplianceMode
{
  /// Standard compliance
  Standard,
  /// Strict compliance with enhanced controls
  Strict,
  /// Regulatory compliance for specific industries
  Regulatory,
}

/// Request for content filtering
#[ derive( Debug, Clone, Serialize ) ]
pub struct ContentFilterRequest
{
  /// Content to be filtered
  pub content : String,
  /// Safety configuration to apply
  pub safety_config : SafetyConfiguration,
  /// Categories to filter against
  pub filter_categories : Vec< FilterCategory >,
  /// Minimum severity threshold for flagging
  pub severity_threshold : SeverityLevel,
}

/// Response from content filtering
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ContentFilterResponse
{
  /// Whether the content is considered safe
  pub is_safe : bool,
  /// Filters that the content passed
  pub passed_filters : Vec< FilterCategory >,
  /// Filters that the content failed
  pub failed_filters : Vec< FilterCategory >,
  /// Overall risk score (0.0 to 1.0)
  pub risk_score : f64,
  /// Recommended action based on filtering results
  pub recommended_action : SafetyAction,
  /// Detailed filter results
  pub filter_results : Vec< FilterResult >,
  /// Audit identifier for tracking
  pub audit_id : Option< String >,
}

/// Categories for content filtering
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub enum FilterCategory
{
  /// Harassment and bullying
  Harassment,
  /// Violence and gore
  Violence,
  /// Adult and sexual content
  Adult,
  /// Hate speech
  Hate,
  /// Self-harm content
  SelfHarm,
  /// Illegal activities
  Illegal,
}

/// Severity levels for safety assessment
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub enum SeverityLevel
{
  /// Low severity
  Low,
  /// Medium severity
  Medium,
  /// High severity
  High,
  /// Critical severity
  Critical,
}

/// Actions recommended by safety assessment
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub enum SafetyAction
{
  /// Allow the content
  Allow,
  /// Warn about the content
  Warn,
  /// Block the content
  Block,
  /// Require human review
  Review,
}

/// Result of a specific filter
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct FilterResult
{
  /// Category that was filtered
  pub category : FilterCategory,
  /// Whether this filter passed
  pub passed : bool,
  /// Confidence score for this filter (0.0 to 1.0)
  pub confidence : f64,
  /// Explanation of the filter result
  pub explanation : Option< String >,
}

/// Request for harm classification
#[ derive( Debug, Clone, Serialize ) ]
pub struct HarmClassificationRequest
{
  /// Content to classify for harm
  pub content : String,
  /// Categories to classify against
  pub classification_categories : Vec< HarmType >,
  /// Minimum confidence threshold
  pub confidence_threshold : f64,
  /// Whether to include explanations
  pub include_explanations : bool,
}

/// Response from harm classification
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct HarmClassificationResponse
{
  /// Whether the content is considered safe
  pub is_safe : bool,
  /// Detected harm categories
  pub harm_categories : Vec< HarmCategory >,
  /// Overall risk score (0.0 to 1.0)
  pub overall_risk_score : f64,
  /// Recommended action
  pub recommended_action : SafetyAction,
  /// Policy violations detected
  pub policy_violations : Vec< String >,
  /// Audit identifier for tracking
  pub audit_id : Option< String >,
}

/// Types of harm for classification
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub enum HarmType
{
  /// Violence and aggression
  Violence,
  /// Harassment and bullying
  Harassment,
  /// Adult and sexual content
  Adult,
  /// Hate speech and discrimination
  Hate,
  /// Self-harm and suicide
  SelfHarm,
  /// Illegal activities
  Illegal,
  /// Misinformation
  Misinformation,
}

/// Detected harm category with details
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct HarmCategory
{
  /// Type of harm detected
  pub category : HarmType,
  /// Confidence in detection (0.0 to 1.0)
  pub confidence : f64,
  /// Severity of the detected harm
  pub severity : SeverityLevel,
  /// Description of the detected harm
  pub description : String,
}

/// Safety policy enforcement configuration
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct SafetyPolicyEnforcement
{
  /// Level of enforcement
  pub enforcement_level : EnforcementLevel,
  /// Whether to automatically block violations
  pub auto_block_violations : bool,
  /// Harm types that require human review
  pub require_human_review : Vec< HarmType >,
  /// Rules for escalating safety issues
  pub escalation_rules : Vec< EscalationRule >,
  /// Compliance reporting configuration
  pub compliance_reporting : ComplianceReporting,
}

/// Levels of safety enforcement
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub enum EnforcementLevel
{
  /// Permissive enforcement
  Permissive,
  /// Standard enforcement
  Standard,
  /// Strict enforcement
  Strict,
  /// Maximum enforcement
  Maximum,
}

/// Rule for escalating safety issues
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct EscalationRule
{
  /// Trigger condition for escalation
  pub trigger : EscalationTrigger,
  /// Action to take when triggered
  pub action : EscalationAction,
  /// Notification channels for escalation
  pub notification_channels : Vec< String >,
}

/// Triggers for safety escalation
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub enum EscalationTrigger
{
  /// High risk score threshold
  HighRiskScore( f64 ),
  /// Multiple violations within time period
  RepeatedViolations( u32, Duration ),
  /// Specific harm type detected
  HarmTypeDetected( HarmType ),
}

/// Actions for safety escalation
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub enum EscalationAction
{
  /// Alert administrator
  AlertAdministrator,
  /// Block user temporarily
  TemporaryBlock( Duration ),
  /// Require additional review
  RequireReview,
  /// Log incident for investigation
  LogIncident,
}

/// Compliance reporting configuration
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ComplianceReporting
{
  /// Whether compliance reporting is enabled
  pub enabled : bool,
  /// Frequency of compliance reports
  pub report_frequency : ReportFrequency,
  /// Whether to include audit trail in reports
  pub include_audit_trail : bool,
  /// Retention period for compliance data in days
  pub retention_period_days : u32,
}

/// Frequency of compliance reports
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub enum ReportFrequency
{
  /// Daily reports
  Daily,
  /// Weekly reports
  Weekly,
  /// Monthly reports
  Monthly,
  /// Quarterly reports
  Quarterly,
}

/// Compliance audit trail entry
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ComplianceAuditTrail
{
  /// Unique audit identifier
  pub audit_id : String,
  /// Timestamp of the audit entry
  pub timestamp : String,
  /// User identifier (if available)
  pub user_id : Option< String >,
  /// Original request content
  pub request_content : String,
  /// Safety assessment results
  pub safety_assessment : SafetyAssessment,
  /// Compliance status
  pub compliance_status : ComplianceStatus,
  /// Whether review is required
  pub review_required : bool,
  /// Additional metadata
  pub metadata : HashMap<  String, String  >,
}

/// Safety assessment for audit trails
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct SafetyAssessment
{
  /// Overall risk score
  pub risk_score : f64,
  /// Detected harm categories
  pub detected_categories : Vec< HarmType >,
  /// Policy violations
  pub policy_violations : Vec< String >,
  /// Action taken based on assessment
  pub action_taken : SafetyAction,
}

/// Compliance status for audit trails
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub enum ComplianceStatus
{
  /// Content complies with policies
  Compliant,
  /// Content flagged for review
  Flagged,
  /// Content violates policies
  Violation,
  /// Pending review
  PendingReview,
}

/// Request for compliance reporting
#[ derive( Debug, Clone, Serialize ) ]
pub struct ComplianceReportRequest
{
  /// Type of report to generate
  pub report_type : ReportType,
  /// Start date for report (ISO 8601 format)
  pub start_date : String,
  /// End date for report (ISO 8601 format)
  pub end_date : String,
  /// Whether to include detailed information
  pub include_details : bool,
  /// Format for the report
  pub format : ReportFormat,
}

/// Types of compliance reports
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub enum ReportType
{
  /// Safety violations report
  SafetyViolations,
  /// Content filtering summary
  ContentFiltering,
  /// Harm classification summary
  HarmClassification,
  /// Audit trail report
  AuditTrail,
  /// Comprehensive compliance report
  Comprehensive,
}

/// Formats for compliance reports
#[ derive( Debug, Clone, Serialize, Deserialize, PartialEq ) ]
pub enum ReportFormat
{
  /// JSON format
  Json,
  /// CSV format
  Csv,
  /// PDF format
  Pdf,
  /// HTML format
  Html,
}

/// Response from compliance report generation
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct ComplianceReportResponse
{
  /// Unique report identifier
  pub report_id : String,
  /// Timestamp when report was generated
  pub generated_at : String,
  /// Total number of requests in report period
  pub total_requests : u64,
  /// Number of violations detected
  pub violations_detected : u64,
  /// Summary of violations by category
  pub violation_summary : HashMap<  String, u64  >,
  /// Report data (format depends on requested format)
  pub report_data : String,
  /// Download URL for the report (if applicable)
  pub download_url : Option< String >,
}

/// Safety status information
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct SafetyStatus
{
  /// Whether safety features are enabled
  pub safety_enabled : bool,
  /// Current safety configuration
  pub current_config : Option< SafetyConfiguration >,
  /// Number of requests processed
  pub requests_processed : u64,
  /// Number of violations detected
  pub violations_detected : u64,
  /// Last update timestamp
  pub last_updated : String,
}

/// Safety performance metrics
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct SafetyPerformanceMetrics
{
  /// Total requests processed by safety system
  pub total_requests_processed : u64,
  /// Average time for harm classification in milliseconds
  pub average_classification_time_ms : f64,
  /// Cache hit rate for safety assessments
  pub cache_hit_rate : f64,
  /// False positive rate
  pub false_positive_rate : f64,
  /// False negative rate
  pub false_negative_rate : f64,
  /// System uptime percentage
  pub uptime_percentage : f64,
}

impl SafetyConfiguration
{
  /// Create a new safety configuration with default settings
  #[ inline ]
  #[ must_use ]
  pub fn new() -> Self
  {
    Self
    {
      content_filtering_enabled : true,
      harm_prevention_level : HarmPreventionLevel::Medium,
      allowed_content_types : vec![ ContentType::Text, ContentType::Educational ],
      blocked_content_types : vec![ ContentType::Adult, ContentType::Violence ],
      custom_safety_rules : None,
      audit_logging_enabled : true,
      compliance_mode : ComplianceMode::Standard,
    }
  }

  /// Set content filtering enabled
  #[ inline ]
  #[ must_use ]
  pub fn with_content_filtering( mut self, enabled : bool ) -> Self
  {
    self.content_filtering_enabled = enabled;
    self
  }

  /// Set harm prevention level
  #[ inline ]
  #[ must_use ]
  pub fn with_harm_prevention_level( mut self, level : HarmPreventionLevel ) -> Self
  {
    self.harm_prevention_level = level;
    self
  }

  /// Set allowed content types
  #[ inline ]
  #[ must_use ]
  pub fn with_allowed_content_types( mut self, types : Vec< ContentType > ) -> Self
  {
    self.allowed_content_types = types;
    self
  }

  /// Set blocked content types
  #[ inline ]
  #[ must_use ]
  pub fn with_blocked_content_types( mut self, types : Vec< ContentType > ) -> Self
  {
    self.blocked_content_types = types;
    self
  }

  /// Set custom safety rules
  #[ inline ]
  #[ must_use ]
  pub fn with_custom_safety_rules( mut self, rules : Vec< String > ) -> Self
  {
    self.custom_safety_rules = Some( rules );
    self
  }

  /// Set audit logging enabled
  #[ inline ]
  #[ must_use ]
  pub fn with_audit_logging( mut self, enabled : bool ) -> Self
  {
    self.audit_logging_enabled = enabled;
    self
  }

  /// Set compliance mode
  #[ inline ]
  #[ must_use ]
  pub fn with_compliance_mode( mut self, mode : ComplianceMode ) -> Self
  {
    self.compliance_mode = mode;
    self
  }
}

impl Default for SafetyConfiguration
{
  #[ inline ]
  fn default() -> Self
  {
    Self::new()
  }
}

impl ContentFilterRequest
{
  /// Create a new content filter request
  #[ inline ]
  #[ must_use ]
  pub fn new( content : String, safety_config : SafetyConfiguration ) -> Self
  {
    Self
    {
      content,
      safety_config,
      filter_categories : vec![
        FilterCategory::Adult,
        FilterCategory::Violence,
        FilterCategory::Harassment,
        FilterCategory::Hate,
      ],
      severity_threshold : SeverityLevel::Medium,
    }
  }

  /// Set filter categories
  #[ inline ]
  #[ must_use ]
  pub fn with_filter_categories( mut self, categories : Vec< FilterCategory > ) -> Self
  {
    self.filter_categories = categories;
    self
  }

  /// Set severity threshold
  #[ inline ]
  #[ must_use ]
  pub fn with_severity_threshold( mut self, threshold : SeverityLevel ) -> Self
  {
    self.severity_threshold = threshold;
    self
  }
}

impl HarmClassificationRequest
{
  /// Create a new harm classification request
  #[ inline ]
  #[ must_use ]
  pub fn new( content : String ) -> Self
  {
    Self
    {
      content,
      classification_categories : vec![
        HarmType::Violence,
        HarmType::Harassment,
        HarmType::Adult,
        HarmType::Hate,
        HarmType::SelfHarm,
        HarmType::Illegal,
      ],
      confidence_threshold : 0.7,
      include_explanations : true,
    }
  }

  /// Set classification categories
  #[ inline ]
  #[ must_use ]
  pub fn with_classification_categories( mut self, categories : Vec< HarmType > ) -> Self
  {
    self.classification_categories = categories;
    self
  }

  /// Set confidence threshold
  #[ inline ]
  #[ must_use ]
  pub fn with_confidence_threshold( mut self, threshold : f64 ) -> Self
  {
    self.confidence_threshold = threshold.clamp( 0.0, 1.0 );
    self
  }

  /// Set whether to include explanations
  #[ inline ]
  #[ must_use ]
  pub fn with_explanations( mut self, include : bool ) -> Self
  {
    self.include_explanations = include;
    self
  }
}

/// Validate safety configuration for consistency
///
/// # Errors
///
/// Returns an error if the configuration contains contradictions
#[ inline ]
pub fn validate_safety_configuration( config : &SafetyConfiguration ) -> Result< (), String >
{
  // Check for overlapping allowed and blocked content types
  for allowed_type in &config.allowed_content_types
  {
    if config.blocked_content_types.contains( allowed_type )
    {
      return Err( format!( "Content type {allowed_type:?} cannot be both allowed and blocked" ) );
    }
  }

  // Validate harm prevention level consistency
  if config.harm_prevention_level == HarmPreventionLevel::Low && config.compliance_mode == ComplianceMode::Strict
  {
    return Err( "Low harm prevention level is incompatible with strict compliance mode".to_string() );
  }

  Ok( () )
}