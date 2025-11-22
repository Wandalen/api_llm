//! Token Counting Implementation
//!
//! Provides token counting and estimation for API requests.
//!
//! ## Features
//!
//! - **Multiple Strategies**: Estimation, character-based, word-based
//! - **Message Counting**: Count tokens in chat messages
//! - **Request Estimation**: Estimate costs before making API calls
//! - **Model-Aware**: Different counting for different models
//!
//! ## Usage
//!
//! ```no_run
//! # use api_huggingface::token_counter::{TokenCounter, CountingStrategy};
//! # fn example() -> Result< (), Box< dyn std::error::Error > > {
//! let counter = TokenCounter::new(CountingStrategy::Estimation);
//!
//! let text = "Hello, how are you today?";
//! let count = counter.count_tokens(text);
//! println!("Estimated tokens : {}", count.total);
//! # Ok(())
//! # }
//! ```

pub mod counter;

pub use counter::{
  TokenCounter,
  CountingStrategy,
  TokenCount,
  TokenCountError,
};
