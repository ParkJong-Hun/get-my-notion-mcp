# Development Guide

This document provides detailed information for developers who want to contribute to or understand the internals of get-my-notion-mcp.

## Prerequisites

- **Rust 1.70+** for building the binary
- **Node.js 16+** for npm packaging
- Internet connection for GitHub API access

## Building from Source

### Local Development

```bash
# Clone the repository
git clone https://github.com/ParkJong-Hun/get-my-notion-mcp.git
cd get-my-notion-mcp

# Build the Rust binary
cargo build --release

# Run directly
./target/release/get-my-notion-mcp

# Or run with cargo
cargo run
```

### npm Package Development

```bash
# Install npm dependencies and build
npm install
npm run build

# Test the build
npm test

# Link for local testing
npm link
get-my-notion-mcp --help
```

## Project Structure

```
src/
├── constants.rs     # Application constants and configuration
├── utils.rs         # Utility functions and helpers
├── main.rs          # Application entry point
├── lib.rs           # Library exports
├── mcp.rs           # MCP protocol types and definitions
├── server.rs        # MCP server implementation
├── github.rs        # GitHub API client
└── handlers.rs      # Tool and resource handlers

bin/
└── get-my-notion-mcp # npm binary wrapper (Node.js script)

scripts/
└── build-and-package.sh # Build automation script

tests/
└── integration_tests.rs # Integration tests

docs/
├── DEVELOPMENT.md   # This file
├── API.md          # API documentation
└── DEPLOYMENT.md   # Deployment guide
```

## Architecture

### Core Components

1. **GitHub Client** (`github.rs`): Handles all GitHub API interactions
2. **MCP Server** (`server.rs`): Implements the MCP protocol
3. **Handlers** (`handlers.rs`): Business logic for tools and resources
4. **Constants** (`constants.rs`): Centralized configuration
5. **Utils** (`utils.rs`): Common utility functions

### Data Flow

```
Claude/Cursor → MCP Server → Handler → GitHub Client → GitHub API
                     ↓
                Response ← Response ← Response ← Response
```

## Testing

### Running Tests

```bash
# Run all Rust tests
cargo test

# Run specific test module
cargo test github::tests

# Run with output
cargo test -- --nocapture

# Test npm package
npm test
```

### Test Coverage

The project includes comprehensive tests:

- **Unit tests**: Individual component testing
- **Integration tests**: End-to-end MCP protocol testing
- **Mock tests**: HTTP API simulation with wiremock
- **Serialization tests**: JSON protocol compliance

### Test Structure

```
tests/
├── github/          # GitHub client tests
├── handlers/        # Handler logic tests  
├── integration/     # MCP protocol tests
└── utils/          # Utility function tests
```

## Configuration Management

All configuration is centralized in `src/constants.rs`:

### GitHub Configuration
- API base URL
- User agent string
- Default repository settings
- Request timeouts

### MCP Protocol Configuration
- Protocol version
- Server metadata
- Tool definitions
- Error codes

### Environment Variables

Currently, the server uses public GitHub API without authentication. For private repositories, you could extend with:

```rust
// In constants.rs
pub const GITHUB_TOKEN: Option<&str> = option_env!("GITHUB_TOKEN");
```

## Adding New Features

### Adding a New Tool

1. **Define the tool** in `constants.rs`:
```rust
pub const TOOL_NEW_FEATURE: &str = "new_feature";
```

2. **Create schema** in `utils.rs`:
```rust
pub fn new_feature_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "param": {
                "type": "string",
                "description": "Parameter description"
            }
        },
        "required": ["param"]
    })
}
```

3. **Implement handler** in `handlers.rs`:
```rust
pub struct NewFeatureHandler {
    github_client: GitHubClient,
}

impl ToolHandler for NewFeatureHandler {
    fn call(&self, arguments: Option<HashMap<String, serde_json::Value>>) -> Result<CallToolResult> {
        // Implementation
    }
}
```

4. **Register in main.rs**:
```rust
server.add_tool(create_new_feature_tool(), NewFeatureHandler::new());
```

### Adding a New Resource

Follow similar pattern but implement `ResourceHandler` trait instead.

## Error Handling

The project uses structured error handling:

1. **GitHub API errors**: Network and API response errors
2. **MCP protocol errors**: Invalid requests and responses  
3. **Business logic errors**: Tool execution failures
4. **Serialization errors**: JSON parsing issues

All errors are mapped to appropriate MCP error responses with proper error codes.

## Performance Considerations

### GitHub API Rate Limits

- **Unauthenticated**: 60 requests/hour per IP
- **Authenticated**: 5,000 requests/hour (future enhancement)

### Memory Usage

- Streaming responses for large files
- Efficient JSON parsing with serde
- Minimal memory footprint for CLI usage

### Async Operations

- Non-blocking HTTP requests with reqwest
- Tokio runtime for async operations
- Concurrent request handling capability

## Debugging

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run
```

### Common Debug Scenarios

1. **GitHub API issues**: Check rate limits and network connectivity
2. **MCP protocol issues**: Validate JSON-RPC message format
3. **Tool execution issues**: Verify parameter parsing and validation

## Publishing and Distribution

### npm Package

```bash
# Build for release
npm run build

# Publish to npm (requires authentication)
npm publish --access public

# Test installation
npm install -g @parkjonghun/get-my-notion-mcp
```

### GitHub Releases

```bash
# Create release binary
cargo build --release

# Package for different platforms
# (Consider using cargo-dist or similar tools)
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass: `cargo test`
5. Update documentation as needed
6. Submit a pull request

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy for linting: `cargo clippy`
- Maintain test coverage for new features
- Use meaningful commit messages

## Troubleshooting

### Common Issues

1. **Build failures**: Check Rust version and dependencies
2. **Network errors**: Verify GitHub API accessibility
3. **Permission issues**: Ensure binary execution permissions
4. **npm package issues**: Verify Node.js version compatibility

### Getting Help

- Check existing GitHub issues
- Review test cases for usage examples
- Consult MCP protocol documentation
- Check GitHub API documentation for rate limits and endpoints