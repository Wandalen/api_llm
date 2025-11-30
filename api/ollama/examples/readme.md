# Ollama Examples

## Purpose

Usage demonstrations for the Ollama API client showing real-world usage patterns and best practices.

## Responsibility Table

| File | Responsibility | Use Case |
|------|----------------|----------|
| `readme.md` | Document example organization | Prerequisites, running examples, troubleshooting |
| `ollama_chat_basic.rs` | Demonstrate basic chat completion | Simplest possible usage, getting started |
| `ollama_chat_streaming.rs` | Demonstrate streaming responses | Real-time output, stream handling |
| `ollama_chat_interactive.rs` | Demonstrate multi-turn conversations | Interactive chat, conversation context |
| `ollama_chat_cached_interactive.rs` | Demonstrate caching with conversations | Cache integration, performance optimization |
| `ollama_chat_assistant.rs` | Demonstrate assistant-style interactions | System prompts, role-based conversations |
| `ollama_code_assistant.rs` | Demonstrate code generation use case | Code completion, code explanation |
| `ollama_document_analyzer.rs` | Demonstrate document analysis use case | Document processing, text analysis |
| `ollama_multimodal_vision.rs` | Demonstrate vision model integration | Image inputs, multimodal requests |
| `simple_secret_example.rs` | Demonstrate secret management integration | Workspace secrets, API key handling |

## Organization Principles

- Examples demonstrate actual usage, not testing
- Each example focuses on a specific use case
- Examples are runnable with `cargo run --example <name>`
- All examples use real Ollama API (no mocks)

## Prerequisites

**IMPORTANT:** Ollama is a local LLM runtime that requires installation and setup before examples will work.

### Step 1: Install Ollama

Download and install from https://ollama.ai

- **macOS**: `brew install ollama`
- **Linux**: `curl -fsSL https://ollama.ai/install.sh | sh`
- **Windows**: Download installer from https://ollama.ai/download

### Step 2: Start Ollama Server

```bash
# Start Ollama server (runs in background)
ollama serve
```

Server runs on `http://localhost:11434` by default.

### Step 3: Pull a Model

```bash
# Recommended model for examples
ollama pull llama3.2

# For vision examples
ollama pull llama3.2-vision

# For code examples
ollama pull codellama

# Verify installation
ollama list
```

### Step 4: Test Setup

```bash
# Quick test - should respond with a greeting
ollama run llama3.2 "Hello!"
```

## Running Examples

Once Ollama is installed, running, and has models downloaded:

```bash
# Basic examples
cargo run --example ollama_chat_basic --features full
cargo run --example ollama_chat_streaming --features full

# Interactive examples
cargo run --example ollama_chat_interactive --features full

# Advanced examples
cargo run --example ollama_code_assistant --features full
cargo run --example ollama_document_analyzer --features full
```

## Troubleshooting

### Examples timeout or "connection refused"

**Cause:** Ollama server not running

**Fix:**
```bash
# Check if server is running
curl http://localhost:11434/api/tags

# Start server if not running
ollama serve
```

### "Model not found" error

**Cause:** Model not downloaded

**Fix:**
```bash
ollama pull llama3.2
```

### Examples hang on first run

**Cause:** Ollama loading model into memory (first-time setup is slow)

**Solution:** Wait 30-60 seconds. Subsequent runs will be faster.

## Requirements

- Ollama server running (default: http://localhost:11434)
- At least one model pulled (e.g., `ollama pull llama3.2`)
- Set `OLLAMA_HOST` environment variable if using non-default endpoint
- Minimum 8GB RAM (16GB+ recommended for larger models)

## Key Differences from Cloud APIs

- **No API Key:** Runs locally, no authentication needed
- **Offline Capable:** Works without internet after models are downloaded
- **Privacy:** All processing happens on your machine
- **Performance:** Depends on your hardware (CPU/GPU/RAM)
