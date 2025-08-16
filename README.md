# Get My Notion MCP Server

A Model Context Protocol (MCP) server that provides access to the [my-notion GitHub repository](https://github.com/ParkJong-Hun/my-notion) data. This server allows AI assistants like Claude to fetch the latest files, content, and commit information from the repository.

## Features

- 📁 **List Files**: Browse repository files and directories
- 📄 **Get File Content**: Retrieve content of specific files
- 🔄 **Latest Commit**: Get the most recent commit SHA
- 🔗 **Repository Info**: Access repository metadata

## Quick Start

### Option 1: Install via npm (Recommended)

```bash
npm install -g @parkjonghun/get-my-notion-mcp
```

### Option 2: Build from Source

```bash
git clone https://github.com/ParkJong-Hun/get-my-notion-mcp.git
cd get-my-notion-mcp
npm run build
```

## Configuration

### With Claude Code

Add the MCP server using the Claude Code CLI:

```bash
claude mcp add get-my-notion-mcp -- npx -y @parkjonghun/get-my-notion-mcp
```

Or manually add to your configuration:

```bash
# Add the server
claude mcp add get-my-notion-mcp

# Configure the command
npx -y @parkjonghun/get-my-notion-mcp
```

### With Cursor

Add the following configuration to your Cursor settings:

```json
{
  "mcpServers": {
    "get-my-notion-mcp": {
      "command": "npx",
      "args": ["-y", "@parkjonghun/get-my-notion-mcp"]
    }
  }
}
```

**Location of Cursor settings:**

- **macOS**: `~/Library/Application Support/Cursor/User/settings.json`
- **Windows**: `%APPDATA%\Cursor\User\settings.json`
- **Linux**: `~/.config/Cursor/User/settings.json`

## Usage Examples

Once configured, you can use these commands in Claude Code or Cursor:

- **"List all files in the my-notion repository"**
- **"Show me the content of README.md from the repository"**
- **"What's the latest commit in the my-notion repo?"**
- **"Browse the src directory structure"**
- **"Get repository information"**

## Available Tools

### `list_files`
Lists files and directories in the repository.

**Parameters:**
- `path` (optional): Specific directory path to browse

**Example usage:**
```
"List files in the src directory"
```

### `get_file_content`
Retrieves the content of a specific file.

**Parameters:**
- `path` (required): File path within the repository

**Example usage:**
```
"Show me the content of src/main.rs"
```

### `get_latest_commit`
Gets the SHA of the latest commit on the main branch.

**Example usage:**
```
"What's the latest commit SHA?"
```

## Available Resources

### `notion://repo/info`
Provides repository information including latest commit details.

## Configuration

The server is pre-configured to access the `ParkJong-Hun/my-notion` repository. To modify the target repository, update the repository details in:

- `src/handlers.rs` - Update the owner and repo names in handler constructors
- `src/main.rs` - Update tool descriptions if needed

## API Details

This server uses the GitHub REST API to fetch repository data. No authentication is required as it only accesses public repository information.

### Rate Limits

- GitHub API allows up to 60 requests per hour for unauthenticated requests
- The server includes appropriate error handling for rate limit scenarios

## Development

### Prerequisites

- **Rust 1.70+** for building the binary
- **Node.js 16+** for npm packaging
- Internet connection for GitHub API access

### Building Locally

```bash
# Clone the repository
git clone https://github.com/ParkJong-Hun/get-my-notion-mcp.git
cd get-my-notion-mcp

# Install dependencies and build
npm install
npm run build

# Test the build
npm test

# Link for local testing
npm link
get-my-notion-mcp --help
```

### Running Tests

```bash
# Run Rust tests
cargo test

# Test npm package
npm test
```

### Publishing to npm

```bash
# Build for release
npm run build

# Publish (requires npm login)
npm publish --access public
```

### Test Coverage

The project includes comprehensive tests:
- Unit tests for GitHub API client
- Handler logic tests with mock servers  
- Integration tests for MCP protocol compliance
- Serialization/deserialization tests

### Project Structure

```
src/
├── constants.rs     # Application constants
├── utils.rs         # Utility functions
├── main.rs          # Application entry point
├── lib.rs           # Library exports
├── mcp.rs           # MCP protocol types
├── server.rs        # MCP server implementation
├── github.rs        # GitHub API client
└── handlers.rs      # Tool and resource handlers
bin/
└── get-my-notion-mcp # npm binary wrapper
scripts/
└── build-and-package.sh # Build script
tests/
└── integration_tests.rs # Integration tests
```

## MCP Protocol Compliance

This server implements MCP version `2025-08-16` and supports:

- ✅ Server initialization
- ✅ Tool listing and execution
- ✅ Resource listing and reading
- ✅ Error handling
- ✅ JSON-RPC communication

## Example Interactions

### Listing Repository Files
```
User: "What files are in the my-notion repository?"
Assistant: [Uses list_files tool to show repository structure]
```

### Reading File Content
```
User: "Show me the main README file"
Assistant: [Uses get_file_content tool with path "README.md"]
```

### Getting Latest Updates
```
User: "Has the repository been updated recently?"
Assistant: [Uses get_latest_commit tool to check latest commit SHA]
```

## Troubleshooting

### Common Issues

1. **Connection Errors**
   - Ensure internet connectivity
   - Check if GitHub API is accessible

2. **Rate Limiting**
   - Wait for rate limit reset (typically 1 hour)
   - Consider implementing GitHub token authentication for higher limits

3. **File Not Found**
   - Verify file paths are correct
   - Check if files exist in the repository

### Logging

Run with debug logging:
```bash
RUST_LOG=debug cargo run
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass: `cargo test`
5. Submit a pull request

## License

This project is licensed under the MIT License.

## Related Projects

- [Model Context Protocol](https://github.com/modelcontextprotocol) - Official MCP specification
- [Claude Code](https://docs.anthropic.com/en/docs/claude-code) - Claude's code-aware AI assistant
- [my-notion](https://github.com/ParkJong-Hun/my-notion) - The target repository for this MCP server