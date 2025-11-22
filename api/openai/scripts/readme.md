# scripts

Development and release automation scripts for api_openai.

## Purpose

Contains shell scripts for CI/CD, release management, and development workflows.

## Contents

- `clippy-check.sh` - Automated clippy linting with project configuration
- `release.sh` - Release automation and versioning
- `validate-version.sh` - Version consistency validation

## Usage

All scripts should be executed from the crate root directory:

```bash
# Run clippy checks
./scripts/clippy-check.sh

# Validate version consistency
./scripts/validate-version.sh

# Perform release (requires proper permissions)
./scripts/release.sh
```

## Requirements

- Bash shell
- Rust toolchain (cargo, clippy)
- Git (for release script)
