#!/bin/bash
set -e

echo "Building get-my-notion-mcp for release..."

# Build the Rust binary
cargo build --release

# Ensure bin directory exists
mkdir -p bin

# Check if the binary was built successfully
if [ ! -f "target/release/get-my-notion-mcp" ]; then
    echo "Error: Binary not found at target/release/get-my-notion-mcp"
    exit 1
fi

echo "Build completed successfully!"
echo "Binary location: $(pwd)/target/release/get-my-notion-mcp"
echo "Binary size: $(du -h target/release/get-my-notion-mcp | cut -f1)"

# Test the binary
echo "Testing binary..."
if ./target/release/get-my-notion-mcp --help > /dev/null 2>&1; then
    echo "Binary test passed!"
else
    echo "Warning: Binary test failed"
fi

echo "Ready for npm publish!"
echo ""
echo "To publish to npm:"
echo "  npm publish --access public"
echo ""
echo "To test locally:"
echo "  npm link"
echo "  get-my-notion-mcp --help"