#!/bin/bash

# Automated Release Script for api_openai
# This script automates version bumping, changelog generation, and release tagging

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
CRATE_NAME="api_openai"
DEFAULT_BRANCH="master"
REMOTE="origin"

# Help function
show_help() {
    cat << EOF
Usage: $0 [OPTIONS] <VERSION_TYPE>

Automated release script for $CRATE_NAME

VERSION_TYPE:
    major       Increment major version (breaking changes)
    minor       Increment minor version (new features)
    patch       Increment patch version (bug fixes)
    <version>   Set specific version (e.g., 1.2.3, 1.0.0-beta.1)

OPTIONS:
    -h, --help      Show this help message
    -n, --dry-run   Show what would be done without making changes
    -f, --force     Skip interactive confirmations
    --no-push      Don't push changes to remote repository
    --no-tag       Don't create git tag
    --pre <name>    Create prerelease with given identifier (alpha, beta, rc)

Examples:
    $0 patch                    # Bump patch version
    $0 minor                    # Bump minor version  
    $0 major                    # Bump major version
    $0 1.5.0                    # Set specific version
    $0 minor --pre beta         # Create minor prerelease (e.g., 1.5.0-beta.1)
    $0 patch --dry-run          # Show what patch bump would do
    
EOF
}

# Parse command line arguments
DRY_RUN=false
FORCE=false
NO_PUSH=false
NO_TAG=false
PRERELEASE=""
VERSION_TYPE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -n|--dry-run)
            DRY_RUN=true
            shift
            ;;
        -f|--force)
            FORCE=true
            shift
            ;;
        --no-push)
            NO_PUSH=true
            shift
            ;;
        --no-tag)
            NO_TAG=true
            shift
            ;;
        --pre)
            PRERELEASE="$2"
            shift 2
            ;;
        major|minor|patch)
            VERSION_TYPE="$1"
            shift
            ;;
        [0-9]*.[0-9]*.[0-9]*)
            VERSION_TYPE="$1"
            shift
            ;;
        *)
            echo -e "${RED}Error: Unknown option or invalid version: $1${NC}"
            show_help
            exit 1
            ;;
    esac
done

# Validate version type
if [[ -z "$VERSION_TYPE" ]]; then
    echo -e "${RED}Error: VERSION_TYPE is required${NC}"
    show_help
    exit 1
fi

# Get current version from Cargo.toml
get_current_version() {
    grep '^version = ' Cargo.toml | sed 's/version = "\([^"]*\)"/\1/'
}

# Calculate next version
calculate_next_version() {
    local current="$1"
    local bump_type="$2"
    local prerelease="$3"
    
    # Parse current version
    local base_version=$(echo "$current" | sed 's/[-+].*//')
    local major=$(echo "$base_version" | cut -d. -f1)
    local minor=$(echo "$base_version" | cut -d. -f2)
    local patch=$(echo "$base_version" | cut -d. -f3)
    
    case "$bump_type" in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        patch)
            patch=$((patch + 1))
            ;;
        [0-9]*.[0-9]*.[0-9]*)
            # Specific version provided
            echo "$bump_type$([ -n "$prerelease" ] && echo "-$prerelease.1")"
            return
            ;;
        *)
            echo -e "${RED}Error: Invalid version bump type: $bump_type${NC}"
            exit 1
            ;;
    esac
    
    local new_version="$major.$minor.$patch"
    if [[ -n "$prerelease" ]]; then
        new_version="$new_version-$prerelease.1"
    fi
    
    echo "$new_version"
}

# Update version in Cargo.toml
update_cargo_version() {
    local new_version="$1"
    
    if [[ "$DRY_RUN" == "true" ]]; then
        echo "Would update Cargo.toml version to: $new_version"
        return
    fi
    
    # Use sed to replace the version line
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS sed requires -i ''
        sed -i '' "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    else
        # Linux sed
        sed -i "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
    fi
    
    echo -e "${GREEN}✓ Updated Cargo.toml version to $new_version${NC}"
}

# Generate changelog entry
generate_changelog() {
    local version="$1"
    local prev_tag="$2"
    
    if [[ "$DRY_RUN" == "true" ]]; then
        echo "Would generate changelog for version $version"
        return
    fi
    
    local date=$(date +"%Y-%m-%d")
    local changelog_entry=""
    
    # Build changelog header
    changelog_entry="## [$version] - $date"$'\n\n'
    
    if [[ -n "$prev_tag" ]]; then
        # Get commits since last tag
        local commits=$(git log --pretty=format:"- %s (%h)" "$prev_tag..HEAD" --no-merges 2>/dev/null || true)
        
        if [[ -n "$commits" ]]; then
            # Categorize commits
            local breaking_changes=$(echo "$commits" | grep -i "BREAKING\|breaking:" || true)
            local features=$(echo "$commits" | grep -i "feat\|feature\|add" | grep -v -i "BREAKING" || true)
            local fixes=$(echo "$commits" | grep -i "fix\|bug" || true)
            local improvements=$(echo "$commits" | grep -i "improve\|enhance\|optimize\|perf" || true)
            local docs=$(echo "$commits" | grep -i "doc\|readme" || true)
            local other=$(echo "$commits" | grep -v -i "feat\|feature\|add\|fix\|bug\|improve\|enhance\|optimize\|perf\|doc\|readme\|BREAKING" || true)
            
            # Add categorized changes
            if [[ -n "$breaking_changes" ]]; then
                changelog_entry+="### ⚠️ BREAKING CHANGES"$'\n'"$breaking_changes"$'\n\n'
            fi
            
            if [[ -n "$features" ]]; then
                changelog_entry+="### Added"$'\n'"$features"$'\n\n'
            fi
            
            if [[ -n "$fixes" ]]; then
                changelog_entry+="### Fixed"$'\n'"$fixes"$'\n\n'
            fi
            
            if [[ -n "$improvements" ]]; then
                changelog_entry+="### Changed"$'\n'"$improvements"$'\n\n'
            fi
            
            if [[ -n "$docs" ]]; then
                changelog_entry+="### Documentation"$'\n'"$docs"$'\n\n'
            fi
            
            if [[ -n "$other" ]]; then
                changelog_entry+="### Other"$'\n'"$other"$'\n\n'
            fi
        else
            changelog_entry+="### Changed"$'\n'"- Version bump to $version"$'\n\n'
        fi
    else
        changelog_entry+="### Added"$'\n'"- Initial release"$'\n\n'
    fi
    
    # Update CHANGELOG.md
    if [[ -f "CHANGELOG.md" ]]; then
        # Insert new entry after the header
        {
            head -2 CHANGELOG.md  # Keep title and blank line
            echo "$changelog_entry"
            tail -n +3 CHANGELOG.md  # Rest of the file
        } > temp_changelog.md && mv temp_changelog.md CHANGELOG.md
    else
        # Create new CHANGELOG.md
        cat > CHANGELOG.md << EOF
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

$changelog_entry
EOF
    fi
    
    echo -e "${GREEN}✓ Updated CHANGELOG.md${NC}"
}

# Main release process
main() {
    echo -e "${BLUE}=== Automated Release Process for $CRATE_NAME ===${NC}"
    
    # Verify we're in the right directory
    if [[ ! -f "Cargo.toml" ]] || ! grep -q "name = \"$CRATE_NAME\"" Cargo.toml; then
        echo -e "${RED}Error: Not in $CRATE_NAME directory${NC}"
        exit 1
    fi
    
    # Check git status
    if [[ $(git status --porcelain | wc -l) -gt 0 ]] && [[ "$FORCE" != "true" ]]; then
        echo -e "${RED}Error: Working directory is not clean${NC}"
        echo "Commit or stash your changes first, or use --force to proceed anyway"
        exit 1
    fi
    
    # Get current version and calculate next
    CURRENT_VERSION=$(get_current_version)
    NEW_VERSION=$(calculate_next_version "$CURRENT_VERSION" "$VERSION_TYPE" "$PRERELEASE")
    
    echo "Current version: $CURRENT_VERSION"
    echo "New version: $NEW_VERSION"
    
    # Get previous tag for changelog
    PREV_TAG=$(git tag --sort=-version:refname | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+' | head -1 || echo "")
    if [[ -n "$PREV_TAG" ]]; then
        echo "Previous tag: $PREV_TAG"
    else
        echo "No previous version tags found"
    fi
    
    # Confirmation
    if [[ "$FORCE" != "true" ]] && [[ "$DRY_RUN" != "true" ]]; then
        echo
        read -p "Proceed with release? [y/N] " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Release cancelled"
            exit 0
        fi
    fi
    
    # Dry run summary
    if [[ "$DRY_RUN" == "true" ]]; then
        echo -e "${YELLOW}=== DRY RUN - No changes will be made ===${NC}"
        echo "Would perform the following actions:"
        echo "1. Update Cargo.toml version: $CURRENT_VERSION → $NEW_VERSION"
        echo "2. Generate changelog entry for version $NEW_VERSION"
        echo "3. Run validation script"
        echo "4. Commit changes"
        if [[ "$NO_TAG" != "true" ]]; then
            echo "5. Create git tag: v$NEW_VERSION"
        fi
        if [[ "$NO_PUSH" != "true" ]]; then
            echo "6. Push changes and tag to $REMOTE"
        fi
        exit 0
    fi
    
    # Perform release steps
    echo -e "${BLUE}Starting release process...${NC}"
    
    # 1. Update version
    update_cargo_version "$NEW_VERSION"
    
    # 2. Generate changelog
    generate_changelog "$NEW_VERSION" "$PREV_TAG"
    
    # 3. Run validation
    echo "Running version validation..."
    if [[ -x "scripts/validate-version.sh" ]]; then
        ./scripts/validate-version.sh
    else
        echo -e "${YELLOW}Warning: Version validation script not found or not executable${NC}"
    fi
    
    # 4. Run tests to ensure nothing is broken
    echo "Running tests..."
    cargo test --all-features
    
    # 5. Commit changes
    git add Cargo.toml CHANGELOG.md
    git commit -m "chore: release version $NEW_VERSION

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"
    
    echo -e "${GREEN}✓ Committed version bump${NC}"
    
    # 6. Create tag
    if [[ "$NO_TAG" != "true" ]]; then
        git tag -a "v$NEW_VERSION" -m "Release v$NEW_VERSION"
        echo -e "${GREEN}✓ Created tag v$NEW_VERSION${NC}"
    fi
    
    # 7. Push changes
    if [[ "$NO_PUSH" != "true" ]]; then
        git push "$REMOTE" HEAD
        if [[ "$NO_TAG" != "true" ]]; then
            git push "$REMOTE" "v$NEW_VERSION"
        fi
        echo -e "${GREEN}✓ Pushed changes to $REMOTE${NC}"
    fi
    
    echo
    echo -e "${GREEN}🎉 Release $NEW_VERSION completed successfully!${NC}"
    echo
    echo "Next steps:"
    echo "- GitHub Actions will automatically:"
    echo "  • Run full CI/CD pipeline"
    echo "  • Publish to crates.io"
    echo "  • Create GitHub release with generated changelog"
    echo "- Monitor the release workflow at:"
    echo "  https://github.com/$(git config remote.$REMOTE.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/')/actions"
}

# Run main function
main "$@"