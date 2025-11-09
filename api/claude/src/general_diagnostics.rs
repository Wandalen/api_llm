//! General diagnostics functionality for Anthropic API client
//!
//! Provides comprehensive monitoring capabilities including request lifecycle
//! tracking, performance metrics, error analysis, and integration monitoring
//! that complement CURL diagnostics for complete observability.

mod private {}

#[ cfg( feature = "general-diagnostics" ) ]
crate::mod_interface!
{
  layer core;
  layer extended;
}
