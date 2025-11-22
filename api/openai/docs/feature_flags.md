# Feature Flags Documentation

## Overview

The API crates use a simplified, consistent feature flag configuration across all providers (OpenAI, Anthropic, Gemini, Ollama).

## Standard Feature Set

All API crates follow this standard pattern:

```toml
[features]
default = []
full = [ "integration", ... ]  # Includes all available features
integration = []               # Enables integration tests with real APIs
```

### Common Features

- **`default`**: Empty by default to minimize dependencies
- **`full`**: Includes all available features for comprehensive functionality
- **`integration`**: Enables integration tests with real API endpoints

### Provider-Specific Features

#### Gemini API
- **`diagnostics_curl`**: Enables curl command generation for debugging
- **`logging`**: Enables structured logging with tracing

#### Ollama API
- **`streaming`**: Enables streaming response functionality

## Usage Examples

### Development (minimal dependencies)
```bash
cargo build
```

### Full functionality
```bash
cargo build --features full
```

### Integration testing
```bash
cargo test --features integration
```

### Specific features
```bash
cargo build --features "integration,logging"
```

## Migration from Previous Version

The previous complex feature configuration:
```toml
# OLD - Complex nested dependencies
default = [ "enabled" ]
enabled = [
  "mod_interface/enabled",
  "former/enabled",
  "error_tools/enabled",
  "derive_tools/enabled",
]
```

Has been simplified to:
```toml
# NEW - Simple, flat configuration
default = []
full = [ "integration" ]
integration = []
```

### Benefits

1. **Simplified Build Configuration**: No complex nested feature dependencies
2. **Consistent Across Providers**: All API crates follow the same pattern
3. **Reduced Build Complexity**: Fewer opportunities for feature conflicts
4. **Better Documentation**: Clear purpose for each feature
5. **Backward Compatibility**: Still supports all necessary functionality

## Testing Feature Combinations

All common feature combinations are tested:

- No features: `cargo check --no-default-features`
- Default features: `cargo check`
- All features: `cargo check --all-features`
- Integration features: `cargo check --features integration`

## Dependency Management

### Optimized Dependency Features

Dependencies have been optimized to use only necessary features, reducing build time and binary size:

```toml
# Before - Over-specified features
tokio = { workspace = true, features = [ "fs", "macros", "rt-multi-thread" ] }
serde = { workspace = true, features = ["derive", "rc"] }

# After - Optimized minimal features
tokio = { workspace = true, features = [ "macros", "sync", "time" ] }
serde = { workspace = true, features = ["derive"] }
```

### Removed Unused Dependencies

Unused dependencies have been removed to reduce compilation overhead:

- **tokio-util**: Not used in source code, removed from openai and gemini crates
- **tokio "fs" feature**: Not used for file system operations, removed
- **tokio "rt-multi-thread" feature**: Default runtime sufficient, removed
- **serde "rc" feature**: No Arc/Rc serialization needed, removed

### Optimization Results

- **Reduced build complexity**: Fewer feature dependencies to resolve
- **Smaller binaries**: Unnecessary code eliminated at compile time
- **Faster compilation**: Less code to compile and link
- **Consistent patterns**: All API crates follow same optimization approach

This reduces build complexity while maintaining all required functionality.