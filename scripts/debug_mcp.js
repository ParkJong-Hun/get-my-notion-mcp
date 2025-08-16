#!/usr/bin/env node

const { spawn } = require('child_process');

// Start the MCP server
const server = spawn('./target/release/get-my-notion-mcp', [], {
  stdio: ['pipe', 'pipe', 'pipe']
});

let receivedData = '';

server.stdout.on('data', (data) => {
  receivedData += data.toString();
  console.log('Server response:', data.toString());
});

server.stderr.on('data', (data) => {
  console.log('Server error:', data.toString());
});

server.on('close', (code) => {
  console.log(`Server exited with code ${code}`);
});

server.on('error', (err) => {
  console.log('Failed to start server:', err.message);
});

// Send initialize request
const initRequest = JSON.stringify({
  jsonrpc: "2.0",
  id: 1,
  method: "initialize",
  params: {
    protocol_version: "2024-11-05",
    capabilities: {},
    clientInfo: {
      name: "claude-code",
      version: "1.0.0"
    }
  }
}) + '\n';

console.log('Sending initialize request:', initRequest.trim());

setTimeout(() => {
  server.stdin.write(initRequest);
}, 100);

// Wait for response and then send initialized notification
setTimeout(() => {
  if (receivedData) {
    const initNotification = JSON.stringify({
      jsonrpc: "2.0",
      method: "initialized"
    }) + '\n';
    
    console.log('Sending initialized notification:', initNotification.trim());
    server.stdin.write(initNotification);
    
    // List tools after initialization
    setTimeout(() => {
      const listToolsRequest = JSON.stringify({
        jsonrpc: "2.0",
        id: 2,
        method: "tools/list"
      }) + '\n';
      
      console.log('Sending tools/list request:', listToolsRequest.trim());
      server.stdin.write(listToolsRequest);
    }, 100);
  }
}, 500);

// Close after 3 seconds
setTimeout(() => {
  console.log('Terminating server...');
  server.kill('SIGTERM');
}, 3000);