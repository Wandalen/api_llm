# api_llm

Direct HTTP API bindings for major LLM providers.

## Overview

This workspace provides thin, transparent API bindings for LLM services. Each crate is a direct HTTP client that maps 1:1 to the provider's REST API without abstraction layers or automatic behaviors.

## Crates

- **api_claude** (38.4k LOC) - Anthropic Claude API client
- **api_gemini** (54.6k LOC) - Google Gemini API client
- **api_huggingface** (43.6k LOC) - Hugging Face Inference API client
- **api_ollama** (37.2k LOC) - Ollama local LLM runtime API client
- **api_openai** (68.8k LOC) - OpenAI API client
- **api_xai** (11.5k LOC) - X.AI Grok API client

## Philosophy: Thin Client, Rich API

All API bindings follow these principles:

1. **API Transparency** - Every method directly corresponds to an API endpoint
2. **Zero Client Intelligence** - No automatic decision-making
3. **Explicit Control** - Developers control all operations
4. **Information vs Action** - Clear separation of concerns

## Features

For a comprehensive feature comparison across all crates, see [api/readme.md](api/readme.md).

All crates provide:
- **Core Features**: Text generation, streaming, function calling, vision, audio, embeddings
- **Enterprise Reliability**: Retry logic, circuit breaker, rate limiting, failover, health checks
- **Observability**: Structured logging, CURL diagnostics, error diagnostics, performance metrics
- **Flexibility**: Async/sync APIs, builder patterns, feature flags, dynamic configuration

## Testing Philosophy

- **No Mocking** - All tests use real API implementations
- **Loud Failures** - Tests fail clearly when APIs unavailable
- **No Silent Passes** - Integration tests never pass silently
- **Real Implementations Only** - No stub/mock servers

## Quick Start

```rust
use api_openai::{ OpenAIClient, ChatRequest, ChatMessage, MessageRole };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = OpenAIClient::new_from_env()?;

  let request = ChatRequest::new( "gpt-4" )
    .with_message( ChatMessage::new( MessageRole::User, "Hello!" ) );

  let response = client.chat( &request ).await?;
  println!( "{}", response.choices[0].message.content );
  Ok(())
}
```

## Secret Management

API keys via environment variables or workspace secrets:

```bash
# Environment (CI/CD)
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
export GEMINI_API_KEY="AIza..."

# Workspace secrets (local development)
source secret/-secrets.sh
```

## Development

```bash
# Check all crates compile
cargo check --workspace

# Run all tests (requires API keys)
cargo test --workspace

# Run tests for specific crate
cargo test -p api_openai

# Build documentation
cargo doc --workspace --open
```

## License

MIT
