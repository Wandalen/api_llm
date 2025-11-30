# workspace_tools Secret Loading Guide

This guide explains exactly what the `workspace_tools` crate expects for secret loading, based on comprehensive testing and debugging.

## Directory Structure

workspace_tools **only** looks for secrets in this exact directory structure:

```
workspace_root/
└── secret/           # Must be exactly "secret" (NO dot prefix)
    ├── -secrets.sh    # Shell script format (recommended)
    ├── -config.env     # Dotenv format
    └── secrets.toml    # Any filename works in secret directory
```

**Key findings:**
- ✅ Directory must be named `secret` (NO dot prefix - visible directory)
- ❌ `.secret`, `.secrets`, `secrets`, or any other name will NOT work
- ✅ Filename can be anything (commonly `-secrets.sh`, `-config.env`)
- ✅ Files starting with `-` are gitignored by default
- ✅ workspace_tools 0.6.0+ uses `secret/` directly (no symlinks needed)

## Supported File Formats

### 1. Shell Script Format (Recommended)

```bash
#!/bin/bash
# shell script format secrets
export API_KEY="sk-1234567890abcdef"
export DATABASE_URL="postgresql://user:pass@localhost/db" 
export DEBUG=true
export TOKEN='bearer-token-here'
```

### 2. Key-Value Format (Dotenv)

```bash
# key-value format secrets
API_KEY=sk-1234567890abcdef
DATABASE_URL=postgresql://user:pass@localhost/db
DEBUG=true
TOKEN=bearer-token-here
```

### 3. Mixed Format (Both in same file)

```bash
# mixed format - both styles work together
API_KEY=standard-key-format
export DATABASE_URL="postgresql://mixed-format/db"
REDIS_URL=redis://standard:6379
export SMTP_USER="admin@example.com"
```

## Quote Handling

workspace_tools correctly handles all quote variations:

```bash
DOUBLE_QUOTED="value with spaces and symbols !@#"
SINGLE_QUOTED='another value with spaces'
NO_QUOTES=simple_value_no_quotes
EMPTY_DOUBLE=""
EMPTY_SINGLE=''
QUOTES_INSIDE="She said 'Hello there!'"
export EXPORT_DOUBLE="exported with double quotes"
export EXPORT_SINGLE='exported with single quotes'
export EXPORT_NO_QUOTES=exported_without_quotes
```

## Loading Methods

### Load All Secrets

```rust
use workspace_tools::workspace;

let ws = workspace()?;
let secrets = ws.load_secrets_from_file("-secrets.sh")?;

for (key, value) in secrets {
    println!("{}: {}", key, mask_value(&value));
}
```

### Load Individual Secret with Fallback

```rust
// Tries secret file first, then environment variable
let api_key = ws.load_secret_key("API_KEY", "-secrets.sh")?;
```

### Secure Loading (if "secure" feature enabled)

```rust
#[cfg(feature = "secure")]
{
    use secrecy::ExposeSecret;
    
    let secure_secrets = ws.load_secrets_secure("-secrets.sh")?;
    let api_key = ws.load_secret_key_secure("API_KEY", "-secrets.sh")?;
    
    println!("key length: {}", api_key.expose_secret().len());
}
```

## Error Handling

- **Non-existent file**: Returns empty HashMap (no error)
- **Non-existent key**: Returns error with helpful message
- **Malformed lines**: Ignores gracefully, parses what it can
- **Permission issues**: Returns IO error

## Environment Variable Fallback

workspace_tools automatically falls back to environment variables:

1. First checks secret file for the key
2. If not found, checks environment variable with same name
3. If neither found, returns error

```bash
# This will be found via environment fallback
export MY_SECRET="from_environment"
```

## Workspace Setup

workspace_tools needs the workspace root configured:

```toml
# .cargo/config.toml
[env]
WORKSPACE_PATH = { value = ".", relative = true }
```

Or set environment variable:
```bash
export WORKSPACE_PATH="/path/to/workspace/root"
```

## Common Issues and Solutions

### Issue: Secrets not loading
**Check:**
1. Directory is named `secret` (NO dot prefix - visible directory)
2. File exists in `secret/` directory
3. WORKSPACE_PATH is set correctly
4. File has proper KEY=VALUE or export KEY=VALUE format

### Issue: Some keys missing
**Check:**
1. Lines have `=` character
2. No syntax errors in file
3. Keys are not commented out with `#`

### Issue: Quotes not handled correctly
**Solution:** workspace_tools handles quotes automatically - both single and double quotes are stripped from values.

## Example Test Structure

For testing, create this structure:

```rust
use tempfile::TempDir;
use std::env;

let temp_dir = TempDir::new()?;
env::set_var( "WORKSPACE_PATH", temp_dir.path() );

let ws = workspace_tools::workspace()?;
let secret_dir = ws.secret_dir();  // Creates secret directory
fs::create_dir_all( &secret_dir )?;

let secret_file = secret_dir.join( "-secrets.sh" );
let content = r#"export API_KEY="test-key"
DATABASE_URL=test-url
"#;
fs::write( &secret_file, content )?;

let secrets = ws.load_secrets_from_file( "-secrets.sh" )?;
assert_eq!( secrets.get( "API_KEY" ).unwrap(), "test-key" );
```

## Security Notes

- Files starting with `-` are automatically gitignored
- Use the "secure" feature for memory-safe secret handling
- workspace_tools supports the secrecy crate for zero-on-drop secrets
- Always validate secrets exist before using in production
- Consider using environment variables for CI/CD pipelines

## Features Required

Add these features to your Cargo.toml:

```toml
[dependencies]
workspace_tools = { version = "0.2.0", features = ["secrets", "secure", "testing"] }
```

- `secrets`: Basic secret loading functionality
- `secure`: Memory-safe secret handling with secrecy crate  
- `testing`: Test utilities for creating temporary workspaces