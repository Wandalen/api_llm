# Makefile for api_gemini development workflow
.PHONY: help setup check test test-unit test-integration lint format doc clean bench audit coverage release-check examples

# Default target
help: ## Show this help message
	@echo "🛠️  api_gemini development commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "📋 Common workflows:"
	@echo "  make setup          # First-time development setup"
	@echo "  make dev            # Quick development check (format + lint + test)"
	@echo "  make test-all       # Run all tests (unit + integration)"
	@echo "  make release-check  # Verify release readiness"

# Environment setup
setup: ## Setup development environment
	@echo "🚀 Setting up development environment..."
	@./scripts/dev-setup.sh

# Code quality checks
check: ## Run cargo check on all targets
	@echo "🔍 Running cargo check..."
	@cargo check --all-targets --all-features

lint: ## Run clippy lints
	@echo "🔍 Running clippy..."
	@cargo clippy --all-targets --all-features -- -D warnings

# Testing
test: test-unit ## Run unit tests (alias for test-unit)

test-unit: ## Run unit tests only
	@echo "🧪 Running unit tests..."
	@cargo test --lib

test-integration: ## Run integration tests (requires GEMINI_API_KEY)
	@echo "🌐 Running integration tests..."
	@if [ -z "$$GEMINI_API_KEY" ]; then \
		echo "⚠️  GEMINI_API_KEY not set - integration tests require API key"; \
		echo "   Set environment variable or add to workspace secret/-secrets.sh"; \
		exit 1; \
	fi
	@cargo test --features integration

test-all: test-unit test-integration ## Run all tests (unit + integration)

test-verbose: ## Run tests with verbose output
	@echo "🧪 Running tests with verbose output..."
	@RUST_LOG=debug cargo test --lib -- --nocapture

# Development workflow
dev: lint test-unit ## Quick development check (lint + unit tests)

dev-watch: ## Watch files and run tests continuously
	@echo "👀 Watching for changes..."
	@cargo watch -x "test --lib"

# Documentation
doc: ## Build and open documentation
	@echo "📚 Building documentation..."
	@cargo doc --open --all-features

doc-check: ## Check documentation links and examples
	@echo "📚 Checking documentation..."
	@cargo test --doc --all-features

# Performance and analysis
# Benchmarks removed per requirements

bloat: ## Analyze binary size
	@echo "📊 Analyzing binary size..."
	@cargo bloat --release

audit: ## Security audit of dependencies
	@echo "🔒 Running security audit..."
	@cargo audit

outdated: ## Check for outdated dependencies
	@echo "📅 Checking for outdated dependencies..."
	@cargo outdated

coverage: ## Generate code coverage report
	@echo "📊 Generating coverage report..."
	@cargo tarpaulin --out Html --output-dir coverage
	@echo "Coverage report: coverage/tarpaulin-report.html"

# Examples
examples: ## List available examples
	@echo "📝 Available examples:"
	@find examples -name "*.rs" -exec basename {} .rs \; | sort | sed 's/^/  /'

example: ## Run an example (use: make example EXAMPLE=gemini_chat_basic)
	@if [ -z "$(EXAMPLE)" ]; then \
		echo "❌ Please specify an example: make example EXAMPLE=gemini_chat_basic"; \
		echo "Available examples:"; \
		find examples -name "*.rs" -exec basename {} .rs \; | sort | sed 's/^/  /'; \
		exit 1; \
	fi
	@echo "🚀 Running example: $(EXAMPLE)"
	@cargo run --example $(EXAMPLE)

# Maintenance
clean: ## Clean build artifacts
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean
	@rm -rf coverage/

clean-all: clean ## Clean all artifacts including cache
	@echo "🧹 Cleaning all artifacts..."
	@rm -rf target/
	@rm -rf ~/.cargo/registry/index/*

# Release preparation
release-check: lint doc-check test-unit audit ## Check release readiness
	@echo "🚀 Checking release readiness..."
	@echo "✅ All release checks passed!"

publish-dry: ## Dry run of cargo publish
	@echo "📦 Dry run cargo publish..."
	@cargo publish --dry-run

# Development utilities
deps-tree: ## Show dependency tree
	@echo "🌳 Dependency tree:"
	@cargo tree

deps-licenses: ## Show dependency licenses
	@echo "📄 Dependency licenses:"
	@cargo tree --format "{p} {l}"

init-api-key: ## Initialize API key in workspace secrets (workspace_tools 0.6.0)
	@echo "🔑 Setting up API key in workspace secrets..."
	@echo "⚠️  Note: This will add GEMINI_API_KEY to workspace root secret/-secrets.sh"
	@echo -n "Enter your Gemini API key: " && read -s key && \
		echo "" && \
		echo "export GEMINI_API_KEY=\"$$key\"" >> ../../secret/-secrets.sh && \
		chmod 600 ../../secret/-secrets.sh && \
		echo "✅ API key added to workspace secret/-secrets.sh (workspace_tools 0.6.0)"

# CI/CD removed per requirements

# Local development server simulation
dev-server: ## Start development with file watching
	@echo "🔄 Starting development server with auto-reload..."
	@cargo watch -x "check --all-targets" -x "test --lib"

# Profiling and debugging
profile: ## Profile the application (requires cargo-profiler)
	@echo "📊 Profiling application..."
	@cargo build --release
	@echo "Use 'perf' or other profiling tools with target/release/..."

debug-example: ## Run example with debugging info
	@if [ -z "$(EXAMPLE)" ]; then \
		echo "❌ Please specify an example: make debug-example EXAMPLE=gemini_chat_basic"; \
		exit 1; \
	fi
	@echo "🐛 Running example with debug info: $(EXAMPLE)"
	@RUST_LOG=debug RUST_BACKTRACE=full cargo run --example $(EXAMPLE)

# Help for specific commands
help-testing: ## Show testing help
	@echo "🧪 Testing Commands Help:"
	@echo ""
	@echo "  test-unit        - Fast unit tests, no API key required"
	@echo "  test-integration - Requires GEMINI_API_KEY environment variable"
	@echo "  test-all         - Runs both unit and integration tests"
	@echo "  test-verbose     - Shows detailed test output"
	@echo ""
	@echo "💡 Tips:"
	@echo "  - Use 'make init-api-key' to set up your API key"
	@echo "  - Set RUST_LOG=debug for detailed logging"
	@echo "  - Integration tests may take 15-30 seconds due to API processing"

help-examples: ## Show examples help
	@echo "📝 Examples Commands Help:"
	@echo ""
	@echo "  examples         - List all available examples"
	@echo "  example          - Run specific example (make example EXAMPLE=name)"
	@echo "  debug-example    - Run example with full debugging"
	@echo ""
	@echo "Available examples:"
	@find examples -name "*.rs" -exec basename {} .rs \; | sort | sed 's/^/  /'
	@echo ""
	@echo "💡 Usage: make example EXAMPLE=gemini_chat_basic"