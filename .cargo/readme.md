# cargo configuration directory

This directory contains cargo configuration files that customize cargo's behavior for this workspace.

## purpose

The `.cargo` directory allows you to configure cargo settings that apply to this specific workspace, overriding global settings. This is useful for:

- Setting workspace-specific environment variables
- Defining custom build configurations
- Configuring registry settings for private crates
- Setting up cross-compilation targets
- Defining custom cargo commands and aliases

## files

### config.toml

Main configuration file that can contain:

- **[env]** - Environment variables available during build/test/run
- **[build]** - Build configuration (target, rustflags, etc.)
- **[alias]** - Custom cargo command aliases
- **[registry]** - Registry configuration for publishing/fetching crates
- **[target]** - Target-specific configuration
- **[term]** - Terminal output configuration

## benefits of workspace-relative configuration

1. **Consistent environment** - All developers get the same build environment
2. **Reproducible builds** - Build configuration is version controlled
3. **Simplified setup** - No need to set environment variables manually
4. **Path resolution** - Enables workspace-relative path resolution for secrets and resources

## documentation

See [cargo configuration documentation](https://doc.rust-lang.org/cargo/reference/config.html) for complete configuration options.
