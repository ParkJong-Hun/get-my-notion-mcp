# Get My Notion MCP Server

A Model Context Protocol (MCP) server that provides access to the [my-notion GitHub repository](https://github.com/ParkJong-Hun/my-notion) data. This server allows AI assistants like Claude and Cursor to fetch the latest files, content, and commit information from the repository.

## ✨ Features

- 📁 **Browse Files**: List files and directories in the repository
- 📄 **Read Content**: Get the content of any file
- 🔄 **Track Changes**: Check the latest commit information
- 🔗 **Repository Info**: Access repository metadata

## 🚀 Quick Start

### Install

```bash
npm install -g @parkjonghun/get-my-notion-mcp
```

### Configure with Claude Code

```bash
claude mcp add get-my-notion-mcp -- npx -y @parkjonghun/get-my-notion-mcp
```

### Configure with Cursor

Add to your Cursor settings:

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

**Settings location:**
- **macOS**: `~/Library/Application Support/Cursor/User/settings.json`
- **Windows**: `%APPDATA%\Cursor\User\settings.json`
- **Linux**: `~/.config/Cursor/User/settings.json`

## 💬 Example Usage

Once configured, try these commands:

- *"List all files in the my-notion repository"*
- *"Show me the README.md content"*
- *"What's the latest commit?"*
- *"Browse the src directory"*

## 🔧 Alternative Installation Methods

### Direct Binary
```bash
# Build from source
git clone https://github.com/ParkJong-Hun/get-my-notion-mcp.git
cd get-my-notion-mcp
cargo build --release

# Use directly
./target/release/get-my-notion-mcp
```

### Configuration for Direct Binary
```json
{
  "mcpServers": {
    "get-my-notion-mcp": {
      "command": "/path/to/get-my-notion-mcp"
    }
  }
}
```

## 📚 Documentation

- **[API Reference](docs/API.md)** - Detailed tool and resource documentation
- **[Development Guide](docs/DEVELOPMENT.md)** - Contributing and building from source
- **[Deployment Guide](docs/DEPLOYMENT.md)** - Advanced installation and configuration

## 🔧 How It Works

This MCP server connects to the [my-notion GitHub repository](https://github.com/ParkJong-Hun/my-notion) and provides three main tools:

- **`list_files`** - Browse repository structure
- **`get_file_content`** - Read any file content  
- **`get_latest_commit`** - Get current commit info

All data is fetched in real-time from the GitHub API, so you always get the latest information.

## ⚡ Requirements

- No authentication needed (uses public GitHub API)
- Internet connection for GitHub API access
- Rate limit: 60 requests per hour

## 🛠️ Troubleshooting

**Connection issues?** Verify internet access and GitHub API availability.

**Rate limits?** The server handles GitHub's 60 requests/hour limit automatically.

**Installation problems?** Check our [Deployment Guide](docs/DEPLOYMENT.md) for alternative methods.

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🔗 Links

- **[my-notion repository](https://github.com/ParkJong-Hun/my-notion)** - The repository this server accesses
- **[Model Context Protocol](https://github.com/modelcontextprotocol)** - Official MCP specification
- **[Claude Code](https://docs.anthropic.com/en/docs/claude-code)** - Claude's code assistant
- **[Cursor](https://cursor.sh)** - AI-powered code editor