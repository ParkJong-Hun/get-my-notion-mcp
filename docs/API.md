# API Documentation

This document describes the MCP tools and resources provided by get-my-notion-mcp.

## Tools

### `list_files`

Lists files and directories in the my-notion GitHub repository.

**Parameters:**
- `path` (optional, string): Specific directory path to browse. Defaults to repository root.

**Example usage:**
```
"List files in the src directory"
"Show me what's in the docs folder"
"List all files in the repository"
```

**Response format:**
```
Files in repository:

- **README.md** (abc123d)
  Path: README.md
  Type: file
  Size: 1024 bytes

- **src** (def456a)
  Path: src
  Type: dir

...
```

---

### `get_file_content`

Retrieves the content of a specific file from the repository.

**Parameters:**
- `path` (required, string): File path within the repository

**Example usage:**
```
"Show me the content of README.md"
"Get the source code from src/main.rs"
"What's in the package.json file?"
```

**Response format:**
```
Content of file: README.md

```
# My Notion

This is the content of the file...
```
```

---

### `get_latest_commit`

Gets the SHA hash of the latest commit on the main branch.

**Parameters:** None

**Example usage:**
```
"What's the latest commit?"
"Has the repository been updated recently?"
"Show me the current commit hash"
```

**Response format:**
```
Latest commit SHA: abc123def456789...
```

## Resources

### `notion://repo/info`

Provides general information about the my-notion repository.

**Example usage:**
```
"Give me information about the repository"
"Show repository details"
```

**Response format:**
```
Repository: ParkJong-Hun/my-notion
Latest commit: abc123def456789...
Access via: https://github.com/ParkJong-Hun/my-notion
```

## Error Handling

The server provides structured error responses for common scenarios:

### Tool Not Found
```json
{
  "error": {
    "code": -32601,
    "message": "Tool not found: 'invalid_tool'"
  }
}
```

### Missing Required Parameters
```json
{
  "error": {
    "code": -32603,
    "message": "Path parameter is required"
  }
}
```

### GitHub API Errors
```json
{
  "error": {
    "code": -32603,
    "message": "GitHub API request failed: 404"
  }
}
```

### Rate Limit Exceeded
```json
{
  "error": {
    "code": -32603,
    "message": "GitHub API request failed: 429"
  }
}
```

## Rate Limits

- **GitHub API**: 60 requests per hour for unauthenticated requests
- **File size**: No explicit limit, but large files may take longer to fetch
- **Concurrent requests**: Server handles requests sequentially

## Supported File Types

The server can fetch any text-based file from the repository:

- Source code (.rs, .js, .py, .java, etc.)
- Documentation (.md, .txt, .rst, etc.)
- Configuration (.json, .yaml, .toml, etc.)
- Data files (.csv, .xml, etc.)

Binary files are returned as base64-encoded content when possible.

## MCP Protocol Compliance

This server implements MCP version `2025-08-16` with:

- ✅ **Server initialization**: Handshake and capability negotiation
- ✅ **Tool listing**: Dynamic tool discovery
- ✅ **Tool execution**: Parameter validation and execution
- ✅ **Resource listing**: Available resource enumeration
- ✅ **Resource reading**: Content retrieval
- ✅ **Error handling**: Structured error responses
- ✅ **JSON-RPC**: Compliant message format

## Usage Examples

### Basic File Browsing
```
User: "What files are in the my-notion repository?"
Assistant: [Lists all files in the root directory]

User: "Show me what's in the src folder"
Assistant: [Lists files in src/ directory]
```

### Code Review
```
User: "Show me the main application code"
Assistant: [Gets content of src/main.rs or similar]

User: "What does the README say?"
Assistant: [Gets content of README.md]
```

### Repository Status
```
User: "Has this repository been updated recently?"
Assistant: [Shows latest commit information]

User: "Give me an overview of this repository"
Assistant: [Shows repository information resource]
```

### Advanced Usage
```
User: "Compare the current main.rs with what was there before"
Assistant: [Gets current file content, then latest commit info for comparison]

User: "Find all TypeScript files in the project"
Assistant: [Lists files and filters for .ts extensions]
```