#!/bin/bash

# Manual test script for chat examples - Math reasoning test
# Tests context retention and arithmetic capabilities

set -e

echo "=== Math Context Test ==="
echo "Testing: x=13, then x*3?"
echo ""

# Source API key
source /home/user1/pro/secret/-secrets.sh

# Build the example
echo "Building interactive_chat example..."
cargo build --example interactive_chat --features="full" --quiet

echo ""
echo "=== Running Test ==="
echo ""

# Create input sequence
INPUT="x=13
x*3?
quit"

# Run the example with input
echo "$INPUT" | cargo run --example interactive_chat --features="full" --quiet 2>&1 | tee /tmp/chat_test_output.txt

echo ""
echo "=== Analyzing Results ==="
echo ""

# Check if output contains 39
if grep -q "39" /tmp/chat_test_output.txt; then
  echo "✅ SUCCESS: Found '39' in output"
  echo ""
  echo "Relevant output:"
  grep -A2 -B2 "39" /tmp/chat_test_output.txt
else
  echo "❌ FAILURE: Did not find '39' in output"
  echo ""
  echo "Full output:"
  cat /tmp/chat_test_output.txt
  exit 1
fi
