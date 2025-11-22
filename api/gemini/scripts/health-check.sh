#!/usr/bin/env bash
# Project health check script for api_gemini
# Provides comprehensive overview of project status

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

echo -e "${BLUE}🩺 api_gemini Project Health Check${NC}\n"

# Health metrics tracking
total_checks=0
passed_checks=0

# Function to run health check
health_check() {
    local check_name="$1"
    local check_command="$2"
    local severity="${3:-warning}" # error, warning, info

    ((total_checks++))
    echo -e "${BLUE}🔍 Checking: $check_name${NC}"

    if eval "$check_command" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ $check_name: OK${NC}"
        ((passed_checks++))
        return 0
    else
        case $severity in
            "error")
                echo -e "${RED}❌ $check_name: FAILED${NC}"
                ;;
            "warning")
                echo -e "${YELLOW}⚠️ $check_name: WARNING${NC}"
                ((passed_checks++)) # Don't fail overall health for warnings
                ;;
            "info")
                echo -e "${PURPLE}ℹ️ $check_name: INFO${NC}"
                ((passed_checks++)) # Don't fail overall health for info
                ;;
        esac
        return 1
    fi
}

# Health check with custom output
health_check_custom() {
    local check_name="$1"
    local check_function="$2"

    ((total_checks++))
    echo -e "${BLUE}🔍 Checking: $check_name${NC}"

    if $check_function; then
        ((passed_checks++))
        return 0
    else
        return 1
    fi
}

echo -e "${PURPLE}📋 Basic Project Health${NC}"
echo "==============================="

# 1. Project structure
health_check "Cargo.toml exists" "test -f Cargo.toml" "error"
health_check "Source directory exists" "test -d src" "error"
health_check "Tests directory exists" "test -d tests" "warning"
health_check "Examples directory exists" "test -d examples" "warning"
health_check "README exists" "test -f readme.md || test -f README.md" "warning"

echo ""
echo -e "${PURPLE}🛠️ Development Environment${NC}"
echo "==============================="

# 2. Development environment
health_check "Rust toolchain available" "command -v rustc" "error"
health_check "Cargo available" "command -v cargo" "error"
health_check "Git available" "command -v git" "warning"

# Check Rust version
rust_version_check() {
    if rustc --version | grep -q "rustc 1\.[7-9][0-9]\|rustc 1\.[0-9][0-9][0-9]"; then
        echo -e "${GREEN}✅ Rust version: Modern ($(rustc --version | cut -d' ' -f2))${NC}"
        return 0
    else
        echo -e "${YELLOW}⚠️ Rust version: Old ($(rustc --version | cut -d' ' -f2))${NC}"
        return 1
    fi
}
health_check_custom "Rust version compatibility" rust_version_check

echo ""
echo -e "${PURPLE}📦 Dependencies${NC}"
echo "==============================="

# 3. Dependencies health
health_check "Dependencies resolve" "cargo check --quiet" "error"
health_check "No security vulnerabilities" "cargo audit" "warning"

# Check for outdated dependencies
outdated_check() {
    if command -v cargo-outdated > /dev/null; then
        if outdated_output=$(cargo outdated 2>/dev/null); then
            outdated_count=$(echo "$outdated_output" | grep -c "→" || true)
            if [[ $outdated_count -eq 0 ]]; then
                echo -e "${GREEN}✅ Dependencies: Up to date${NC}"
                return 0
            else
                echo -e "${YELLOW}⚠️ Dependencies: $outdated_count outdated packages${NC}"
                return 1
            fi
        fi
    else
        echo -e "${PURPLE}ℹ️ Dependencies: cargo-outdated not available${NC}"
        return 0
    fi
}
health_check_custom "Dependency freshness" outdated_check

echo ""
echo -e "${PURPLE}🧪 Code Quality${NC}"
echo "==============================="

# 4. Code quality
health_check "Code compiles" "cargo check --all-targets --all-features --quiet" "error"
# Code formatting check removed (cargo fmt FORBIDDEN by codestyle rulebook)
health_check "Clippy clean" "cargo clippy --all-targets --all-features --quiet -- -D warnings" "warning"
health_check "Unit tests pass" "cargo test --lib --quiet" "error"
health_check "Doc tests pass" "cargo test --doc --quiet" "warning"

echo ""
echo -e "${PURPLE}📁 File Organization${NC}"
echo "==============================="

# 5. File organization
file_count_check() {
    local src_files=$(find src -name "*.rs" | wc -l)
    local test_files=$(find tests -name "*.rs" 2>/dev/null | wc -l || echo 0)
    local example_files=$(find examples -name "*.rs" 2>/dev/null | wc -l || echo 0)

    echo -e "${GREEN}✅ File organization: $src_files source, $test_files test, $example_files example files${NC}"
    return 0
}
health_check_custom "File organization" file_count_check

# Check for common files
health_check "License file present" "test -f LICENSE || test -f LICENSE.md || test -f LICENSE.txt" "info"
health_check "Changelog present" "test -f CHANGELOG.md || test -f changelog.md" "info"

echo ""
echo -e "${PURPLE}🔧 Configuration${NC}"
echo "==============================="

# 6. Configuration
api_key_check() {
    if [[ -n "${GEMINI_API_KEY:-}" ]]; then
        echo -e "${GREEN}✅ API Key: Available in environment${NC}"
        return 0
    elif [[ -f "secret/gemini_api_key" ]]; then
        echo -e "${GREEN}✅ API Key: Available in secret file${NC}"
        return 0
    else
        echo -e "${YELLOW}⚠️ API Key: Not configured (integration tests will be skipped)${NC}"
        return 1
    fi
}
health_check_custom "API key configuration" api_key_check

# Git configuration
git_status_check() {
    if git status --porcelain 2>/dev/null | grep -q .; then
        dirty_files=$(git status --porcelain | wc -l)
        echo -e "${YELLOW}⚠️ Git status: $dirty_files uncommitted changes${NC}"
        return 1
    else
        echo -e "${GREEN}✅ Git status: Clean working directory${NC}"
        return 0
    fi
}
if command -v git > /dev/null && git rev-parse --git-dir > /dev/null 2>&1; then
    health_check_custom "Git working directory" git_status_check
fi

echo ""
echo -e "${PURPLE}📊 Project Metrics${NC}"
echo "==============================="

# 7. Project metrics
project_metrics() {
    local src_lines=$(find src -name "*.rs" -exec wc -l {} + 2>/dev/null | tail -1 | awk '{print $1}' || echo 0)
    local test_files=$(find tests -name "*.rs" 2>/dev/null | wc -l || echo 0)
    local example_files=$(find examples -name "*.rs" 2>/dev/null | wc -l || echo 0)

    echo "  📝 Source lines: $src_lines"
    echo "  🧪 Test files: $test_files"
    echo "  📚 Examples: $example_files"

    # Feature count
    local features=$(grep -c "^[a-zA-Z_].*=" Cargo.toml | tail -1 || echo 0)
    echo "  🏗️ Cargo features: $features"

    # Dependencies count
    local deps=$(grep -A 100 "^\[dependencies\]" Cargo.toml | grep -c "^[a-zA-Z_]" || echo 0)
    echo "  📦 Dependencies: $deps"

    echo -e "${GREEN}✅ Project metrics: Generated${NC}"
    return 0
}
health_check_custom "Project size analysis" project_metrics

echo ""
echo -e "${PURPLE}🎯 Health Summary${NC}"
echo "==============================="

# Calculate health percentage
health_percentage=$((passed_checks * 100 / total_checks))

echo "📊 Overall Health Score: $passed_checks/$total_checks checks passed ($health_percentage%)"

if [[ $health_percentage -ge 90 ]]; then
    echo -e "${GREEN}🎉 Excellent health! Project is in great shape.${NC}"
elif [[ $health_percentage -ge 75 ]]; then
    echo -e "${GREEN}✅ Good health! Minor issues to address.${NC}"
elif [[ $health_percentage -ge 60 ]]; then
    echo -e "${YELLOW}⚠️ Moderate health. Some attention needed.${NC}"
else
    echo -e "${RED}❌ Poor health. Significant issues require attention.${NC}"
fi

echo ""
echo -e "${BLUE}💡 Quick Commands:${NC}"
echo "  make dev            # Quick development check"
echo "  make test-all       # Run all tests (if API key configured)"
echo "  # CI/CD not available per requirements"
echo "  make setup          # Setup development environment"

exit 0