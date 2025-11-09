#!/bin/bash

# Semantic Version Validation Script for api_openai
# This script validates that version changes follow our semantic versioning strategy

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\([^"]*\)"/\1/')

# Validate version format (semver)
validate_semver() {
    local version="$1"
    if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?(\+[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$ ]]; then
        echo -e "${RED}ERROR: Invalid semantic version format: $version${NC}"
        echo "Expected format: MAJOR.MINOR.PATCH[-prerelease][+buildmeta]"
        exit 1
    fi
}

# Extract version components
parse_version() {
    local version="$1"
    local base_version=$(echo "$version" | sed 's/[-+].*//')
    
    MAJOR=$(echo "$base_version" | cut -d. -f1)
    MINOR=$(echo "$base_version" | cut -d. -f2) 
    PATCH=$(echo "$base_version" | cut -d. -f3)
}

# Check if version is greater than previous
compare_versions() {
    local prev="$1"
    local curr="$2"
    
    # Parse both versions
    local prev_base=$(echo "$prev" | sed 's/[-+].*//')
    local curr_base=$(echo "$curr" | sed 's/[-+].*//')
    
    local prev_major=$(echo "$prev_base" | cut -d. -f1)
    local prev_minor=$(echo "$prev_base" | cut -d. -f2)
    local prev_patch=$(echo "$prev_base" | cut -d. -f3)
    
    local curr_major=$(echo "$curr_base" | cut -d. -f1)
    local curr_minor=$(echo "$curr_base" | cut -d. -f2)
    local curr_patch=$(echo "$curr_base" | cut -d. -f3)
    
    # Check version increment is valid
    if [[ $curr_major -gt $prev_major ]]; then
        echo "MAJOR version increment detected"
        return 0
    elif [[ $curr_major -eq $prev_major ]] && [[ $curr_minor -gt $prev_minor ]]; then
        echo "MINOR version increment detected"
        return 0
    elif [[ $curr_major -eq $prev_major ]] && [[ $curr_minor -eq $prev_minor ]] && [[ $curr_patch -gt $prev_patch ]]; then
        echo "PATCH version increment detected"
        return 0
    elif [[ "$curr" == "$prev" ]]; then
        echo "Version unchanged"
        return 0
    else
        echo -e "${RED}ERROR: Invalid version increment from $prev to $curr${NC}"
        echo "Version can only increase in semantic order"
        return 1
    fi
}

# Check for breaking changes in public API
check_breaking_changes() {
    echo "Checking for potential breaking changes..."
    
    # This is a simplified check - in practice you'd use tools like cargo-semver-checks
    if git diff --name-only HEAD~1 2>/dev/null | grep -q "src/lib.rs\|src/client.rs"; then
        echo -e "${YELLOW}WARNING: Changes detected in main API files${NC}"
        echo "Please verify that changes are backward compatible for MINOR/PATCH releases"
    fi
}

# Validate MSRV in Cargo.toml
check_msrv() {
    if grep -q "rust-version" Cargo.toml; then
        local msrv=$(grep "rust-version" Cargo.toml | sed 's/rust-version = "\([^"]*\)"/\1/')
        echo "Minimum Supported Rust Version: $msrv"
        
        # Check if MSRV changed
        if git show HEAD~1:Cargo.toml 2>/dev/null | grep -q "rust-version"; then
            local prev_msrv=$(git show HEAD~1:Cargo.toml | grep "rust-version" | sed 's/rust-version = "\([^"]*\)"/\1/')
            if [[ "$msrv" != "$prev_msrv" ]]; then
                echo -e "${YELLOW}WARNING: MSRV changed from $prev_msrv to $msrv${NC}"
                echo "MSRV changes require MAJOR version increment"
            fi
        fi
    else
        echo -e "${YELLOW}WARNING: No rust-version specified in Cargo.toml${NC}"
    fi
}

# Main validation
main() {
    echo -e "${GREEN}=== Semantic Versioning Validation ===${NC}"
    echo "Current version: $CURRENT_VERSION"
    
    # Validate current version format
    validate_semver "$CURRENT_VERSION"
    echo -e "${GREEN}✓ Version format is valid${NC}"
    
    # Check if we can compare with previous version
    if git tag --sort=-version:refname | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+' | head -1 >/dev/null 2>&1; then
        PREV_TAG=$(git tag --sort=-version:refname | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+' | head -1)
        PREV_VERSION=${PREV_TAG#v}
        
        echo "Previous version: $PREV_VERSION"
        
        # Compare versions
        if compare_versions "$PREV_VERSION" "$CURRENT_VERSION"; then
            echo -e "${GREEN}✓ Version increment is valid${NC}"
        else
            exit 1
        fi
    else
        echo "No previous version tags found - assuming initial release"
    fi
    
    # Additional checks
    check_breaking_changes
    check_msrv
    
    echo -e "${GREEN}=== Version validation complete ===${NC}"
}

# Show usage if help requested
if [[ "${1:-}" == "--help" ]] || [[ "${1:-}" == "-h" ]]; then
    echo "Usage: $0"
    echo ""
    echo "Validates semantic versioning compliance for api_openai crate"
    echo ""
    echo "This script checks:"
    echo "  - Version format follows semantic versioning"
    echo "  - Version increments are in correct order"
    echo "  - Potential breaking changes are flagged"
    echo "  - MSRV changes are detected"
    exit 0
fi

# Run main validation
main