//! Code Execution Environment
//!
//! Types and configurations for secure code execution capabilities.

use serde::{ Serialize, Deserialize };
use core::time::Duration;

/// Configuration for code execution environment.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct CodeExecutionConfig
{
  /// Runtime environment to use
  pub runtime : CodeRuntime,
  /// Execution timeout
  pub timeout : Duration,
  /// Memory limit in bytes
  pub memory_limit : usize,
  /// Allowed imports/modules
  pub allowed_imports : Vec< String >,
  /// Security level for execution
  pub security_level : SecurityLevel,
}

impl Default for CodeExecutionConfig
{
  #[ inline ]
  fn default() -> Self
  {
    Self
    {
      runtime : CodeRuntime::Python,
      timeout : Duration::from_secs( 30 ),
      memory_limit : 128 * 1024 * 1024, // 128MB
      allowed_imports : vec![
        "os".to_string(),
        "sys".to_string(),
        "json".to_string(),
        "math".to_string(),
        "datetime".to_string(),
      ],
      security_level : SecurityLevel::Sandbox,
    }
  }
}

/// Available code runtime environments.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub enum CodeRuntime
{
  /// Python runtime
  Python,
  /// JavaScript/Node.js runtime
  JavaScript,
  /// Rust runtime
  Rust,
  /// Go runtime
  Go,
  /// Custom container runtime
  Custom
  {
    /// Runtime name
    name : String,
    /// Container image
    image : String,
  },
}

/// Security levels for code execution.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub enum SecurityLevel
{
  /// Fully sandboxed execution (recommended)
  Sandbox,
  /// Limited system access
  Restricted,
  /// Full system access (dangerous - use with caution)
  Trusted,
}

/// Result of code execution.
#[ derive( Debug, Clone, Serialize, Deserialize ) ]
pub struct CodeExecutionResult
{
  /// Standard output
  pub output : String,
  /// Error output if execution failed
  pub error : Option< String >,
  /// Execution time
  pub execution_time : Duration,
  /// Memory used in bytes
  pub memory_used : usize,
  /// Process return code
  pub return_code : i32,
}
