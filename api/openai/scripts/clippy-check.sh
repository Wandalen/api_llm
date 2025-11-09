#!/bin/bash

# Clippy Check Script for api_openai
# 
# This script runs clippy with the custom configuration and provides
# detailed reporting for CI/CD integration and local development.

set -e  # Exit on any error

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CRATE_ROOT="$(dirname "$SCRIPT_DIR")"
PROJECT_NAME="api_openai"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default configuration
FAIL_ON_WARNINGS=${FAIL_ON_WARNINGS:-1}
VERBOSE=${VERBOSE:-0}
FIX_MODE=${FIX_MODE:-0}
OUTPUT_FORMAT=${OUTPUT_FORMAT:-"human"}

# Usage information
show_usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Run clippy with custom configuration for ${PROJECT_NAME}.

OPTIONS:
    -h, --help              Show this help message
    -v, --verbose           Enable verbose output
    -f, --fix               Automatically apply clippy suggestions
    -w, --allow-warnings    Don't fail on warnings (exit code 0)
    --json                  Output in JSON format
    --sarif                 Output in SARIF format for GitHub integration
    --workspace             Run from workspace root (auto-detected)

ENVIRONMENT VARIABLES:
    FAIL_ON_WARNINGS       Set to 0 to allow warnings (default: 1)
    VERBOSE                 Set to 1 for verbose output (default: 0)
    FIX_MODE               Set to 1 to auto-fix issues (default: 0)
    OUTPUT_FORMAT          Set format: human|json|sarif (default: human)

EXAMPLES:
    # Basic usage
    $0

    # Run with auto-fix
    $0 --fix

    # Allow warnings in CI
    FAIL_ON_WARNINGS=0 $0

    # Generate JSON report
    $0 --json > clippy-report.json

EOF
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_usage
                exit 0
                ;;
            -v|--verbose)
                VERBOSE=1
                shift
                ;;
            -f|--fix)
                FIX_MODE=1
                shift
                ;;
            -w|--allow-warnings)
                FAIL_ON_WARNINGS=0
                shift
                ;;
            --json)
                OUTPUT_FORMAT="json"
                shift
                ;;
            --sarif)
                OUTPUT_FORMAT="sarif"
                shift
                ;;
            --workspace)
                # Auto-detection handles this
                shift
                ;;
            *)
                echo -e "${RED}Unknown option: $1${NC}" >&2
                show_usage
                exit 1
                ;;
        esac
    done
}

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

log_verbose() {
    if [[ $VERBOSE -eq 1 ]]; then
        echo -e "${BLUE}[VERBOSE]${NC} $1"
    fi
}

# Check if we're in a valid Rust project
check_environment() {
    log_verbose "Checking environment..."
    
    # Check for Cargo.toml
    if [[ ! -f "$CRATE_ROOT/Cargo.toml" ]]; then
        log_error "Cargo.toml not found at $CRATE_ROOT"
        log_error "Make sure you're running this script from the correct location"
        exit 1
    fi
    
    # Check for clippy.toml
    if [[ -f "$CRATE_ROOT/clippy.toml" ]]; then
        log_verbose "Found custom clippy configuration: $CRATE_ROOT/clippy.toml"
    else
        log_warning "No custom clippy configuration found at $CRATE_ROOT/clippy.toml"
        log_warning "Using workspace-level configuration only"
    fi
    
    # Check if we need to run from workspace
    if grep -q "workspace = true" "$CRATE_ROOT/Cargo.toml" 2>/dev/null; then
        log_verbose "Package uses workspace dependencies, attempting workspace-level execution"
        # Try to find workspace root
        WORKSPACE_ROOT="$CRATE_ROOT"
        while [[ "$WORKSPACE_ROOT" != "/" ]]; do
            if [[ -f "$WORKSPACE_ROOT/Cargo.toml" ]] && grep -q '\[workspace\]' "$WORKSPACE_ROOT/Cargo.toml" 2>/dev/null; then
                log_verbose "Found workspace root: $WORKSPACE_ROOT"
                cd "$WORKSPACE_ROOT"
                break
            fi
            WORKSPACE_ROOT="$(dirname "$WORKSPACE_ROOT")"
        done
    fi
}

# Build clippy command
build_clippy_command() {
    local cmd="cargo clippy"
    
    # Add package specification if running from workspace
    if [[ -f "Cargo.toml" ]] && grep -q '\[workspace\]' "Cargo.toml" 2>/dev/null; then
        # Try to find the correct package name
        local package_name
        if [[ -f "$CRATE_ROOT/Cargo.toml" ]]; then
            package_name=$(grep '^name = ' "$CRATE_ROOT/Cargo.toml" | head -1 | sed 's/name = "\(.*\)"/\1/')
            if [[ -n "$package_name" ]]; then
                cmd="$cmd --package $package_name"
                log_verbose "Using package name: $package_name"
            fi
        fi
    fi
    
    # Add targets
    cmd="$cmd --all-targets"
    
    # Add features if available
    if grep -q "all-features" "$CRATE_ROOT/Cargo.toml" 2>/dev/null; then
        cmd="$cmd --all-features"
    fi
    
    # Add fix mode if requested
    if [[ $FIX_MODE -eq 1 ]]; then
        cmd="$cmd --fix"
        log_verbose "Auto-fix mode enabled"
    fi
    
    # Add output format options
    case $OUTPUT_FORMAT in
        json)
            cmd="$cmd --message-format json"
            ;;
        sarif)
            cmd="$cmd --message-format json" # Convert to SARIF later
            ;;
    esac
    
    # Add clippy arguments
    if [[ $FAIL_ON_WARNINGS -eq 1 ]]; then
        cmd="$cmd -- -D warnings"
    else
        cmd="$cmd -- -W warnings"
    fi
    
    echo "$cmd"
}

# Convert JSON output to SARIF format for GitHub integration
convert_to_sarif() {
    local json_input="$1"
    cat << EOF
{
  "\$schema": "https://schemastore.azurewebsites.net/schemas/json/sarif-2.1.0.json",
  "version": "2.1.0",
  "runs": [
    {
      "tool": {
        "driver": {
          "name": "clippy",
          "informationUri": "https://doc.rust-lang.org/clippy/",
          "version": "$(rustc --version | cut -d' ' -f2)"
        }
      },
      "results": []
    }
  ]
}
EOF
}

# Main execution function
run_clippy() {
    local cmd
    local exit_code=0
    local temp_output=""
    
    cmd=$(build_clippy_command)
    
    log_info "Running clippy with custom configuration..."
    log_verbose "Command: $cmd"
    
    # Create temporary file for output if needed
    if [[ "$OUTPUT_FORMAT" == "sarif" ]]; then
        temp_output=$(mktemp)
    fi
    
    # Execute clippy
    if [[ "$OUTPUT_FORMAT" == "sarif" ]]; then
        # Capture JSON output and convert to SARIF
        if eval "$cmd" > "$temp_output" 2>&1; then
            convert_to_sarif "$(cat "$temp_output")"
        else
            exit_code=$?
            convert_to_sarif "$(cat "$temp_output")"
        fi
        rm -f "$temp_output"
    else
        # Direct execution
        if ! eval "$cmd"; then
            exit_code=$?
        fi
    fi
    
    # Report results
    if [[ $exit_code -eq 0 ]]; then
        log_success "Clippy check passed!"
    else
        if [[ $FAIL_ON_WARNINGS -eq 1 ]]; then
            log_error "Clippy check failed with warnings/errors"
        else
            log_warning "Clippy found warnings (allowed)"
            exit_code=0
        fi
    fi
    
    return $exit_code
}

# Cleanup function
cleanup() {
    # Remove any temporary files if they exist
    if [[ -n "$temp_output" ]] && [[ -f "$temp_output" ]]; then
        rm -f "$temp_output"
    fi
}

# Set up trap for cleanup
trap cleanup EXIT

# Main execution
main() {
    parse_args "$@"
    
    log_info "Starting clippy check for $PROJECT_NAME"
    log_verbose "Script directory: $SCRIPT_DIR"
    log_verbose "Crate root: $CRATE_ROOT"
    log_verbose "Working directory: $(pwd)"
    
    check_environment
    
    if ! run_clippy; then
        log_error "Clippy check completed with issues"
        exit 1
    fi
    
    log_success "Clippy check completed successfully"
}

# Run main function with all arguments
main "$@"