//! API interface types for the Gemini client.
//!
//! This module contains all the API handle types (ModelsApi, TunedModelsApi, etc.)
//! that provide access to specific API endpoints.

mod models_api;
mod tuned_models_api;
mod files_api;
mod cached_content_api;

#[ cfg( feature = "chat" ) ]
mod chat_api;

#[ cfg( feature = "chat" ) ]
mod conversation_builder;

pub use models_api::ModelsApi;
pub use tuned_models_api::TunedModelsApi;
pub use files_api::FilesApi;
pub use cached_content_api::CachedContentApi;

#[ cfg( feature = "chat" ) ]
pub use chat_api::ChatApi;

#[ cfg( feature = "chat" ) ]
pub use conversation_builder::{ ConversationBuilder, ConversationSummary };
