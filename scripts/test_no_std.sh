#!/bin/bash
# Test script for no-std code paths
# This compiles and runs tests without the std feature to test the no-std Vec implementations

set -e

echo "🧪 Testing no-std code paths..."
echo "=================================="

cd "$(dirname "$0")/.."

# Test no-std integration tests
echo ""
echo "Running no-std integration tests..."
cargo test --package justcode-core --test no_std_integration --no-default-features --features derive

echo ""
echo "✅ No-std tests completed successfully!"
echo ""
echo "Note: These tests verify that the no-std Vec implementations"
echo "      (lines 211-215, 217, 223-228, 230 in encode.rs) work correctly."

