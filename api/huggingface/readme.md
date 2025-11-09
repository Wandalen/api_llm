# api_huggingface

HuggingFace API client library providing access to large language models (LLMs) and embeddings through the HuggingFace Inference API.

## 🎯 Architecture: Stateless HTTP Client

**This API crate is designed as a stateless HTTP client with zero persistence requirements.** It provides:
- Direct HTTP calls to the HuggingFace Inference API
- In-memory operation state only (resets on restart)
- No external storage dependencies (databases, files, caches)
- No configuration persistence beyond environment variables

This ensures lightweight, containerized deployments and eliminates operational complexity.

## 🏛️ Governing Principle: "Thin Client, Rich API"

This library strictly adheres to the **"Thin Client, Rich API"** governing principle, ensuring transparency and developer control:

- **🔍 API Transparency**: Every server-side capability is exposed directly without abstraction layers
- **🚫 Zero Client Intelligence**: No automatic decision-making or hidden behaviors in the client
- **⚙️ Explicit Control**: Developers explicitly configure and control all library behaviors
- **📊 Information vs Action**: Methods provide information; developers decide actions

This principle ensures predictable behavior, explicit control, and transparent operations across all API interactions.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
api_huggingface = "0.2.0"
```

### Basic Usage

```rust
use api_huggingface::{
  Client,
  environment::HuggingFaceEnvironmentImpl,
  components::{input::InferenceParameters, models::Models},
  secret::Secret,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Load API key from environment variable
  let api_key = Secret::load_from_env("HUGGINGFACE_API_KEY")?;

  // Create client
  let env = HuggingFaceEnvironmentImpl::build(api_key, None)?;
  let client = Client::build(env)?;

  // Generate text
  let params = InferenceParameters::new()
    .with_temperature(0.7)
    .with_max_new_tokens(100);

  let response = client.inference()
    .create_with_parameters(
      "What is the capital of France?",
      Models::llama_3_1_8b_instruct(),
      params
    )
    .await?;

  println!("Response: {:?}", response);
  Ok(())
}
```

## Environment Setup

```bash
# Set your HuggingFace API key
export HUGGINGFACE_API_KEY="hf_..."

# Run your application
cargo run
```

Get your API key from [huggingface.co/settings/tokens](https://huggingface.co/settings/tokens)

## Documentation

- **[Features Overview](docs/features.md)** - Complete feature list, status tracking, and cargo features
- **[API Reference](docs/api_reference.md)** - Comprehensive API documentation with examples
- **[Examples](examples/readme.md)** - Working code examples for common use cases
- **[Specification](spec.md)** - Detailed technical specification

## Cargo Features

- `default`: Core functionality (async inference and embeddings)
- `full`: All features including integration tests
- `sync`: Synchronous API wrappers for blocking contexts
- `inference-streaming`: Streaming support for text generation
- `embeddings-similarity`: Similarity calculation utilities
- `integration`: Enable integration tests with real API calls

**Example usage:**

```toml
# Minimal async build
api_huggingface = "0.2.0"

# With synchronous API
api_huggingface = { version = "0.2.0", features = ["sync"] }

# Everything (recommended for development)
api_huggingface = { version = "0.2.0", features = ["full"] }
```

## Requirements

- Rust 1.70.0 or later
- HuggingFace API key ([sign up here](https://huggingface.co/settings/tokens))

## License

MIT License - see LICENSE file for details.
