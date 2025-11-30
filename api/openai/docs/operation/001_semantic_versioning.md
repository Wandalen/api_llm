# Semantic Versioning Strategy for api_openai

## Overview

This document defines the semantic versioning strategy for the `api_openai` crate, following [Semantic Versioning 2.0.0](https://semver.org/) principles adapted for our specific use case as an API client library.

## Version Format

We use the standard `MAJOR.MINOR.PATCH` version format:

- **MAJOR**: Version when making incompatible API changes
- **MINOR**: Version when adding functionality in a backward-compatible manner
- **PATCH**: Version when making backward-compatible bug fixes

## Version Increment Guidelines

### MAJOR Version Changes (Breaking Changes)

Increment the MAJOR version when making changes that break backward compatibility:

#### Public API Changes
- Removing public functions, methods, or types
- Changing function signatures (parameters, return types)
- Changing the behavior of existing APIs in incompatible ways
- Removing or renaming public fields in structs
- Changing error types or error handling behavior

#### Dependency Changes
- Upgrading dependencies to versions with breaking changes that affect our public API
- Changing minimum supported Rust version (MSRV) by more than 6 months
- Removing support for major OpenAI API versions

#### Examples
```rust
// MAJOR: Removing a public function
// Before (v1.x.x)
pub fn deprecated_method() -> Result< String >
{ ... }

// After (v2.0.0) - function removed entirely

// MAJOR: Changing function signature
// Before (v1.x.x)
pub fn create_completion(prompt: &str) -> Result< Response >
{ ... }

// After (v2.0.0)
pub fn create_completion(request: CompletionRequest) -> Result< Response >
{ ... }
```

### MINOR Version Changes (New Features)

Increment the MINOR version when adding new functionality that maintains backward compatibility:

#### API Extensions
- Adding new public functions, methods, or types
- Adding new optional parameters to existing functions (with defaults)
- Adding new fields to structs (with defaults or using non-exhaustive patterns)
- Adding new enum variants (when marked as non-exhaustive)
- Adding support for new OpenAI API endpoints

#### Feature Additions
- Adding new cargo features that don't affect default behavior
- Adding new error types or error cases that don't change existing error handling
- Performance improvements that don't change API behavior

#### Examples
```rust
// MINOR: Adding new optional parameter
// Before (v1.0.x)
pub fn create_completion(prompt: &str) -> Result< Response >
{ ... }

// After (v1.1.0)
pub fn create_completion(prompt: &str) -> Result< Response >
{ ... }
pub fn create_completion_with_options(prompt: &str, options: Option< CompletionOptions >) -> Result< Response >
{ ... }

// MINOR: Adding new field to non-exhaustive struct
#[non_exhaustive]
pub struct CompletionRequest {
    pub prompt: String,
    pub max_tokens: Option<u32>,
    // After v1.1.0
    pub temperature: Option<f32>, // New field with default
}
```

### PATCH Version Changes (Bug Fixes)

Increment the PATCH version for backward-compatible bug fixes:

#### Bug Fixes
- Fixing incorrect behavior that doesn't match documented API
- Fixing memory leaks or performance issues
- Correcting error messages or error handling
- Fixing compilation issues with existing features
- Documentation corrections and improvements

#### Internal Changes
- Code refactoring that doesn't affect public API
- Updating internal dependencies without API changes
- Test improvements and additions
- CI/CD pipeline improvements

#### Examples
```rust
// PATCH: Fixing a bug in existing functionality
// Before (v1.0.0) - bug: always returns empty string
pub fn format_response(response: &Response) -> String
{
    "".to_string() // Bug!
}

// After (v1.0.1) - bug fixed
pub fn format_response(response: &Response) -> String
{
    response.content.clone() // Fixed
}
```

## Pre-release and Build Metadata

### Pre-release Versions
Use pre-release identifiers for unstable versions:
- `alpha`: Very early development, API may change significantly
- `beta`: Feature-complete but may have bugs, API should be stable
- `rc` (release candidate): Ready for release pending final testing

Format: `1.2.3-alpha.1`, `1.2.3-beta.2`, `1.2.3-rc.1`

### Build Metadata
Build metadata may be included for specific builds:
Format: `1.2.3+build.123`, `1.2.3-alpha.1+git.abc123`

## Special Considerations for API Clients

### OpenAI API Version Support
- Adding support for new OpenAI API versions: **MINOR**
- Removing support for deprecated OpenAI API versions: **MAJOR**
- Changing default API version: **MAJOR**

### Error Handling
- Adding new error variants to non-exhaustive enums: **MINOR**
- Changing error types or removing error variants: **MAJOR**
- Improving error messages without changing types: **PATCH**

### Authentication and Security
- Changes to credential handling that affect API: **MAJOR**
- Adding new authentication methods: **MINOR**
- Fixing security vulnerabilities: **PATCH** (with security advisory)

## Version Management Process

### 1. Version Planning
- Review all changes since last release
- Classify each change as MAJOR, MINOR, or PATCH
- The highest classification determines the version increment

### 2. Version Bumping
- Update version in `Cargo.toml`
- Update workspace dependency versions if needed
- Update documentation with version-specific information
- Create version-specific migration guides for MAJOR versions

### 3. Release Validation
- All tests must pass (`ctest3`)
- No clippy warnings with pedantic lints
- Documentation builds successfully
- Integration tests pass with real API

### 4. Release Communication
- **MAJOR**: Detailed migration guide and breaking change documentation
- **MINOR**: Feature announcement and usage examples
- **PATCH**: Brief changelog entry

## Changelog Requirements

Maintain a `CHANGELOG.md` file with the following format:

```markdown
# Changelog

## [Unreleased]

## [1.2.1] - 2024-01-15
### Fixed
- Fixed authentication header encoding issue (#123)
- Corrected timeout handling in streaming responses

## [1.2.0] - 2024-01-10
### Added
- Support for new GPT-4 Turbo model
- New `with_timeout()` method for custom request timeouts

### Changed
- Improved error messages for rate limiting

## [1.1.0] - 2024-01-01
### Added
- Support for function calling in chat completions
- New `CompletionOptions` struct for advanced configuration

### Deprecated
- `create_simple_completion()` - use `create_completion()` instead
```

## Compatibility Policy

### Minimum Supported Rust Version (MSRV)
- MSRV changes require **MAJOR** version increment
- Maintain compatibility with Rust versions from last 12 months minimum
- Document MSRV in README and Cargo.toml

### Dependency Management
- Pin major versions of critical dependencies
- Document dependency requirements in specification
- Test against multiple dependency versions where feasible

## Automation and CI/CD Integration

### Version Validation
- CI checks that version numbers follow SemVer format
- Automated tests verify no breaking changes in MINOR/PATCH releases
- Integration tests run against multiple OpenAI API versions

### Release Process
- Automated changelog generation from conventional commits
- Version increment suggestions based on commit analysis
- Automated publishing to crates.io after manual approval

## Migration Support

### Deprecation Policy
- Features marked deprecated in MINOR releases
- Deprecated features removed in next MAJOR release
- Minimum 6 months between deprecation and removal
- Clear migration path provided for all deprecated features

### Breaking Change Communication
- All breaking changes documented with before/after examples
- Migration scripts or tools provided where feasible
- Community notification through appropriate channels

## Version History Tracking

### Git Tags
- All releases tagged with format `v{version}` (e.g., `v1.2.3`)
- Pre-releases tagged with full version (e.g., `v1.2.3-alpha.1`)
- Tags signed with maintainer GPG key

### Documentation Versioning
- API documentation versioned for each release
- Version-specific examples and guides
- Legacy version documentation maintained for 2+ major versions

This strategy ensures predictable, reliable versioning that communicates changes clearly to users while maintaining the stability expected from a production API client library.