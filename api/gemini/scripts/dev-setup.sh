#!/usr/bin/env bash
# Development environment setup script for api_gemini

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Setting up api_gemini development environment${NC}\n"

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ️${NC} $1"
}

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]] || ! grep -q "api_gemini" Cargo.toml; then
    print_error "Please run this script from the api_gemini project root directory"
    exit 1
fi

print_info "Detected api_gemini project directory"

# Check Rust installation
if ! command -v rustc &> /dev/null; then
    print_error "Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

RUST_VERSION=$(rustc --version)
print_status "Rust detected: $RUST_VERSION"

# Check cargo tools
print_info "Checking and installing required development tools..."

# Install or update development tools
cargo_tools=(
    "cargo-watch:cargo-watch"
    "cargo-expand:cargo-expand"
    "cargo-tarpaulin:cargo-tarpaulin"
    "cargo-audit:cargo-audit"
    "cargo-outdated:cargo-outdated"
    "cargo-tree:cargo-tree"
    "cargo-bloat:cargo-bloat"
)

for tool_spec in "${cargo_tools[@]}"; do
    IFS=':' read -r package binary <<< "$tool_spec"

    if command -v "$binary" &> /dev/null; then
        print_status "$binary already installed"
    else
        print_info "Installing $package..."
        if cargo install "$package"; then
            print_status "$package installed successfully"
        else
            print_warning "Failed to install $package (continuing anyway)"
        fi
    fi
done

# Setup pre-commit hooks
print_info "Setting up development scripts..."

# Create git hooks directory if it doesn't exist
mkdir -p .git/hooks

# Create pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# Pre-commit hook for api_gemini

echo "🔍 Running pre-commit checks..."

# Check for secrets in staged files
if git diff --staged --name-only | xargs grep -l "sk-\|pk_\|GEMINI_API_KEY.*=" 2>/dev/null; then
    echo "❌ Potential secrets found in staged files!"
    echo "Please remove any hardcoded API keys or secrets before committing."
    exit 1
fi

# Clippy check
echo "🔍 Running clippy..."
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo "❌ Clippy found issues. Please fix them before committing."
    exit 1
fi

# Quick test
echo "🧪 Running quick tests..."
if ! cargo test --lib; then
    echo "❌ Tests failed. Please fix them before committing."
    exit 1
fi

echo "✅ All pre-commit checks passed!"
EOF

chmod +x .git/hooks/pre-commit
print_status "Pre-commit hook installed"

# Setup environment file template
if [[ ! -f ".env.example" ]]; then
    cat > .env.example << 'EOF'
# Example environment configuration for api_gemini development

# Gemini API Key (required for integration tests)
# Get your key from https://makersuite.google.com/
GEMINI_API_KEY=your_api_key_here

# Logging configuration
RUST_LOG=api_gemini=debug,info

# Test timeout (for slow integration tests)
GEMINI_TEST_TIMEOUT=30

# Development mode settings
RUST_BACKTRACE=1
EOF
    print_status "Created .env.example template"
fi

# Setup secrets directory
if [[ ! -d "secret" ]]; then
    mkdir -p secret
    echo "# Place your actual API key in this directory" > secret/readme.md
    echo "secret/" >> .gitignore 2>/dev/null || true
    print_status "Created secret directory for API keys"
fi

# Create development aliases script
cat > scripts/dev-aliases.sh << 'EOF'
#!/usr/bin/env bash
# Development aliases for api_gemini

# Quick development commands
alias api-test='cargo test --features integration'
alias api-test-unit='cargo test --no-default-features --lib'
alias api-check='cargo check --all-targets --all-features'
alias api-lint='cargo clippy --all-targets --all-features -- -D warnings'
alias api-doc='cargo doc --open --all-features'
alias api-bench='cargo bench'
alias api-audit='cargo audit'
alias api-outdated='cargo outdated'
alias api-bloat='cargo bloat --release'

# Integration test with verbose output
alias api-test-verbose='RUST_LOG=debug cargo test --features integration -- --nocapture'

# Watch commands for continuous development
alias api-watch-test='cargo watch -x "test --lib"'
alias api-watch-check='cargo watch -x "check --all-targets"'

# Examples
alias api-example='cargo run --example'
alias api-examples='find examples -name "*.rs" -exec basename {} .rs \;'

# Quick quality checks
alias api-quality='cargo clippy --all-targets --all-features -- -D warnings && cargo test --lib'

echo "🛠️ Development aliases loaded! Available commands:"
echo "  api-test, api-test-unit, api-check, api-lint"
echo "  api-doc, api-bench, api-audit, api-outdated, api-bloat"
echo "  api-watch-test, api-watch-check, api-example, api-examples"
echo "  api-quality (runs lint + test)"
EOF

chmod +x scripts/dev-aliases.sh
print_status "Created development aliases script"

# Create development tasks script
cat > scripts/dev-tasks.sh << 'EOF'
#!/usr/bin/env bash
# Development task runner for api_gemini

set -euo pipefail

GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

case "${1:-help}" in
    "setup")
        echo -e "${BLUE}Setting up development environment...${NC}"
        ./scripts/dev-setup.sh
        ;;

    "test-all")
        echo -e "${BLUE}Running comprehensive test suite...${NC}"
        cargo test --lib
        if [[ -n "${GEMINI_API_KEY:-}" ]]; then
            cargo test --features integration
        else
            echo "⚠️ Skipping integration tests (no GEMINI_API_KEY)"
        fi
        ;;

    "quality-check")
        echo -e "${BLUE}Running quality checks...${NC}"
        cargo clippy --all-targets --all-features -- -D warnings
        cargo test --lib
        ;;

    "security-audit")
        echo -e "${BLUE}Running security audit...${NC}"
        cargo audit
        cargo outdated
        ;;

    "benchmark")
        echo -e "${BLUE}Running benchmarks...${NC}"
        cargo bench
        ;;

    "coverage")
        echo -e "${BLUE}Generating code coverage report...${NC}"
        cargo tarpaulin --out Html --output-dir coverage
        echo "Coverage report generated in coverage/tarpaulin-report.html"
        ;;

    "docs")
        echo -e "${BLUE}Building and opening documentation...${NC}"
        cargo doc --open --all-features
        ;;

    "clean-all")
        echo -e "${BLUE}Cleaning all build artifacts...${NC}"
        cargo clean
        rm -rf target/
        rm -rf coverage/
        ;;

    "release-check")
        echo -e "${BLUE}Checking release readiness...${NC}"
        cargo clippy --all-targets --all-features -- -D warnings
        cargo test --lib
        cargo doc --all-features
        cargo audit
        echo -e "${GREEN}✅ Release checks passed!${NC}"
        ;;

    "help"|*)
        echo "🛠️ api_gemini development tasks:"
        echo ""
        echo "  setup           - Setup development environment"
        echo "  test-all        - Run all tests (unit + integration)"
        echo "  quality-check   - Run format, lint, and unit tests"
        echo "  security-audit  - Run security and dependency audits"
        echo "  benchmark       - Run performance benchmarks"
        echo "  coverage        - Generate code coverage report"
        echo "  docs            - Build and open documentation"
        echo "  clean-all       - Clean all build artifacts"
        echo "  release-check   - Verify release readiness"
        echo "  help            - Show this help message"
        echo ""
        echo "Usage: ./scripts/dev-tasks.sh <task-name>"
        ;;
esac
EOF

chmod +x scripts/dev-tasks.sh
print_status "Created development tasks script"

# Create VS Code configuration
mkdir -p .vscode
if [[ ! -f ".vscode/settings.json" ]]; then
    cat > .vscode/settings.json << 'EOF'
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.checkOnSave.allTargets": true,
    "rust-analyzer.checkOnSave.extraArgs": ["--", "-D", "warnings"],
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.procMacro.enable": true,
    "editor.formatOnSave": true,
    "editor.rulers": [100],
    "files.exclude": {
        "**/target": true,
        "**/.git": true
    },
    "rust-analyzer.lens.enable": true,
    "rust-analyzer.lens.run.enable": true,
    "rust-analyzer.lens.debug.enable": true
}
EOF
    print_status "Created VS Code settings"
fi

if [[ ! -f ".vscode/launch.json" ]]; then
    cat > .vscode/launch.json << 'EOF'
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Debug Example",
            "type": "lldb",
            "request": "launch",
            "program": "${workspaceFolder}/target/debug/examples/${input:exampleName}",
            "args": [],
            "cwd": "${workspaceFolder}",
            "env": {
                "RUST_LOG": "debug",
                "RUST_BACKTRACE": "1"
            }
        },
        {
            "name": "Debug Tests",
            "type": "lldb",
            "request": "launch",
            "program": "${workspaceFolder}/target/debug/deps/${input:testName}",
            "args": ["--nocapture"],
            "cwd": "${workspaceFolder}",
            "env": {
                "RUST_LOG": "debug",
                "RUST_BACKTRACE": "1"
            }
        }
    ],
    "inputs": [
        {
            "id": "exampleName",
            "description": "Example name",
            "default": "chat",
            "type": "promptString"
        },
        {
            "id": "testName",
            "description": "Test name",
            "default": "api_gemini",
            "type": "promptString"
        }
    ]
}
EOF
    print_status "Created VS Code launch configuration"
fi

if [[ ! -f ".vscode/tasks.json" ]]; then
    cat > .vscode/tasks.json << 'EOF'
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "cargo check",
            "type": "shell",
            "command": "cargo",
            "args": ["check", "--all-targets", "--all-features"],
            "group": "build",
            "presentation": {
                "reveal": "silent",
                "panel": "shared"
            },
            "problemMatcher": "$rustc"
        },
        {
            "label": "cargo test",
            "type": "shell",
            "command": "cargo",
            "args": ["test", "--lib"],
            "group": "test",
            "presentation": {
                "reveal": "always",
                "panel": "new"
            }
        },
        {
            "label": "cargo run example",
            "type": "shell",
            "command": "cargo",
            "args": ["run", "--example", "${input:exampleName}"],
            "group": "build",
            "presentation": {
                "reveal": "always",
                "panel": "new"
            }
        }
    ]
}
EOF
    print_status "Created VS Code tasks"
fi

# Final setup verification
echo ""
print_info "Verifying installation..."

# Test basic commands
if cargo check --quiet; then
    print_status "Cargo check passed"
else
    print_warning "Cargo check failed - please review any error messages above"
fi

if cargo test --lib --quiet > /dev/null 2>&1; then
    print_status "Unit tests passed"
else
    print_warning "Unit tests failed - please review test output"
fi

echo ""
print_status "Development environment setup complete!"
echo ""
print_info "Next steps:"
echo "  1. Copy .env.example to .env and add your GEMINI_API_KEY"
echo "  2. Run 'source scripts/dev-aliases.sh' to load development aliases"
echo "  3. Run './scripts/dev-tasks.sh help' to see available development tasks"
echo "  4. Try 'api-test' to run unit tests"
echo "  5. Try 'cargo run --example chat' to test an example"
echo ""
print_info "Happy coding! 🦀"