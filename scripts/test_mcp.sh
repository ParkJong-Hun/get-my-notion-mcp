#!/bin/bash

# Test script to simulate what Claude Code might be doing
echo "Testing MCP server connection..."

# Start the server in background
./target/release/get-my-notion-mcp &
SERVER_PID=$!

# Give it a moment to start
sleep 1

# Send initialize request
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocol_version": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test"}}}' > /tmp/mcp_input

# Try to communicate with the server
timeout 5s ./target/release/get-my-notion-mcp < /tmp/mcp_input > /tmp/mcp_output 2>&1

# Check the result
echo "Server output:"
cat /tmp/mcp_output

# Cleanup
kill $SERVER_PID 2>/dev/null
rm -f /tmp/mcp_input /tmp/mcp_output