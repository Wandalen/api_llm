//! Platform-specific features and integrations.
//!
//! This module provides advanced capabilities for specific use cases including
//! search grounding, code execution, web browsing, and custom tool integrations.
//! All features are feature-gated and include comprehensive security controls.

pub mod search_grounding;
pub mod code_execution;
pub mod web_browsing;
pub mod custom_tool;
pub mod image_generation;
pub mod api_connector;
pub mod client_impl;

use mod_interface::mod_interface;

mod private
{
  // Re-export all types from sibling modules
  pub use super::search_grounding::*;
  pub use super::code_execution::*;
  pub use super::web_browsing::*;
  pub use super::custom_tool::*;
  pub use super::image_generation::*;
  pub use super::api_connector::*;
  pub use super::client_impl::*;
}

mod_interface!
{
  exposed use
  {
    // Search Grounding
    SearchGroundingConfig,
    SearchEngine,
    GroundedResponse,
    SearchSource,
    SearchMetadata,

    // Code Execution
    CodeExecutionConfig,
    CodeRuntime,
    SecurityLevel,
    CodeExecutionResult,

    // Web Browsing
    WebBrowsingConfig,
    BrowsingResult,
    BrowsingMetadata,

    // Custom Tools
    CustomTool,
    ToolParameters,
    ParameterDefinition,
    ToolResult,

    // Image Generation
    ImageGenerationConfig,
    ImageModel,
    ImageSize,
    ImageQuality,
    ImageStyle,
    ImageResponseFormat,
    ImageResult,
    ImageMetadata,

    // API Connectors
    ApiConnectorConfig,
    ApiAuthentication,
    RateLimitConfig,
    RetryConfig,
    ApiConnector,

    // Client Extensions
    PlatformSpecificClient,
  };
}
