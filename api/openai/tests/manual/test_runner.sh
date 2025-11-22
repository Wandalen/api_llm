#!/bin/bash

# Standalone test runner for api_openai
# Since api_openai is excluded from the workspace, we need special handling

set -e

echo "Running api_openai secret error path tests..."

# Create a temporary workspace-like environment
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-../../target}"

# Ensure we have the required dependencies available
if [ ! -d "../../target" ]; then
    echo "Building workspace dependencies first..."
    cd ../..
    cargo build --all-features --release
    cd api/openai
fi

echo "Running secret error path tests directly..."
cargo test --test secret_error_paths --verbose 2>&1 | tee test_output.log

echo "Secret error path tests completed!"
echo "Check test_output.log for detailed results"