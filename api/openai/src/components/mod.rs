//! This module defines shared data structures and components used across various
//! `OpenAI` API groups. It includes common types for requests, responses,
//! and specific components like chat, audio, and image-related structures.
//!
//! # Component Organization
//!
//! Components are logically organized into the following groups:
//!
//! ## Core Components
//! Foundation components used across all API endpoints:
//! - [`models`] - `OpenAI` model definitions
//! - [`common`] - Common types and utilities
//! - [`input`] - Common input handling
//! - [`output`] - Common output handling
//! - [`query`] - Query parameter handling
//!
//! ## Endpoint Components
//!
//! ### Chat & Completions
//! - [`chat_shared`] - Chat completion components
//! - [`completions_legacy`] - Legacy completions (deprecated)
//!
//! ### Assistants
//! - [`assistants_shared`] - Assistant API components
//!
//! ### Files & Storage
//! - [`files`] - File operations
//! - [`uploads`] - File uploads
//! - [`vector_stores_shared`] - Vector store management
//!
//! ### Media Processing
//! - [`audio`] - Audio processing
//! - [`images`] - Image generation
//!
//! ### Real-time Communication
//! - [`realtime_shared`] - Real-time API components
//!
//! ### Batch Operations
//! - [`batch_shared`] - Batch operations
//! - [`fine_tuning_shared`] - Fine-tuning jobs
//!
//! ### Content Processing
//! - [`moderations`] - Content moderation
//! - [`embeddings`] - Text embeddings
//!
//! ### Administration
//! - [`administration_shared`] - Admin operations
//! - [`audit_logs_shared`] - Audit logging
//! - [`usage_shared`] - Usage tracking
//!
//! ### Specialized Components
//! - [`responses`] - Response handling
//! - [`tools`] - Tool definitions

mod private
{
}

// === CORE COMPONENTS ===
pub mod common;
pub mod input;
pub mod models;
pub mod output;
pub mod query;

// === CHAT & COMPLETIONS ===
pub mod chat_shared;
pub mod completions_legacy;

// === ASSISTANTS ===
pub mod assistants_shared;

// === FILES & STORAGE ===
pub mod files;
pub mod uploads;
pub mod vector_stores_shared;

// === MEDIA PROCESSING ===
pub mod audio;
pub mod images;

// === REAL-TIME COMMUNICATION ===
pub mod realtime_shared;

// === BATCH OPERATIONS ===
pub mod batch_shared;
pub mod fine_tuning_shared;

// === CONTENT PROCESSING ===
pub mod embeddings;
pub mod moderations;

// === ADMINISTRATION ===
pub mod administration_shared;
pub mod audit_logs_shared;
pub mod usage_shared;

// === SPECIALIZED COMPONENTS ===
pub mod responses;
pub mod tools;
pub mod embeddings_request;

crate ::mod_interface!
{
  exposed use administration_shared;
  exposed use assistants_shared;
  exposed use audio;
  exposed use audit_logs_shared;
  exposed use batch_shared;
  exposed use chat_shared;
  exposed use common;
  exposed use completions_legacy;
  exposed use embeddings;
  exposed use files;
  exposed use fine_tuning_shared;
  exposed use images;
  exposed use input;
  exposed use models;
  exposed use moderations;
  exposed use output;
  exposed use query;
  exposed use realtime_shared;
  exposed use responses;
  exposed use tools;
  exposed use uploads;
  exposed use usage_shared;
  exposed use vector_stores_shared;
  exposed use embeddings_request;
}