# Anthropic Claude API Client for Rust

[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental)

**Production-ready Rust client for Anthropic's Claude API with 83% feature coverage**

A comprehensive, feature-rich HTTP client for interacting with Anthropic's Claude API. Built with safety, performance, and developer experience in mind.

## ✨ Features

- **💬 Messages API** - Full conversational interface support
- **💾 Prompt Caching** - ~90% cost savings with Anthropic's caching
- **📡 Streaming** - Server-Sent Events (SSE) streaming responses
- **🛠️ Tool Calling** - Complete function calling and tool integration
- **🖼️ Vision Support** - Image analysis capabilities
- **📊 Enterprise Ready** - Rate limiting, retries, circuit breaker, failover
- **🔍 Rich Diagnostics** - Curl command generation and detailed error context
- **🔄 Sync & Async APIs** - Both synchronous and asynchronous interfaces

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
api_claude = { version = "0.1.0", features = ["full"] }
```

### Basic Usage

```rust,ignore
use api_claude::{ Client, Secret, CreateMessageRequest, Message, Role, Content };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with API key
    let secret = Secret::new("sk-ant-api03-your-key-here".to_string())?;
    let client = Client::new(secret);

    // Create a message request
    let request = CreateMessageRequest::builder()
        .model("claude-sonnet-4-5-20250929".to_string())
        .max_tokens(1000)
        .messages(vec![
            Message {
                role: Role::User,
                content: vec![Content::Text {
                    r#type: "text".to_string(),
                    text: "Hello, Claude! How are you?".to_string(),
                }],
                cache_control: None,
            }
        ])
        .build();

    // Send request and get response
    let response = client.create_message(request).await?;
    println!("Claude: {:?}", response.content);

    Ok(())
}
```

## 🔧 Configuration

Set your API key via environment variable:

```bash
export ANTHROPIC_API_KEY="sk-ant-api03-your-key-here"
```

Or use workspace secrets (see [Secret Loading Guide](docs/secret_loading.md)).

## 📚 Documentation

- **[API Reference](https://docs.rs/api_claude)** - Complete API documentation
- **[Examples](examples/)** - Real-world usage examples
- **[Testing Guide](tests/readme.md)** - Testing organization and NO MOCKING policy
- **[Secret Loading](docs/secret_loading.md)** - Authentication and secret management
- **[Specification](spec.md)** - Detailed project specification

## 🏗️ Project Structure

```text
api_claude/
├── src/          # Library source code
├── tests/        # Comprehensive test suite (435 tests)
├── examples/     # Usage demonstrations
├── docs/         # Architecture and design documentation
└── spec.md       # Project specification
```

## 🧪 Testing

```bash
# Run all tests
cargo nextest run --all-features

# Run with linting
cargo clippy --all-targets --all-features -- -D warnings

# Documentation tests
cargo test --doc --all-features
```

**⚠️ Note:** Integration tests require valid `ANTHROPIC_API_KEY` and use real API calls (NO MOCKING).

## 🔧 Building

```bash
# Build with all features
cargo build --all-features

# Build for release
cargo build --release --all-features
```

## 📋 Feature Flags

Fine-grained control via feature flags:

- `authentication` - API key management
- `rate-limiting` - Token bucket rate limiting
- `retry-logic` - Request retry mechanisms
- `circuit-breaker` - Circuit breaker pattern
- `streaming` - SSE streaming support
- `full` - All features enabled

See `Cargo.toml` for complete list.

## 📝 License

This project is licensed under the MIT License.

## 🤝 Contributing

Contributions welcome! Please ensure all tests pass before submitting PR.
