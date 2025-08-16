# Deployment Guide

This guide covers different deployment strategies for get-my-notion-mcp.

## Installation Methods

### Method 1: npm/npx (Recommended for End Users)

**Advantages:**
- One-command installation
- Automatic updates
- Cross-platform compatibility
- No manual binary management

**Installation:**
```bash
npm install -g @parkjonghun/get-my-notion-mcp
```

**Usage in Cursor:**
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

**Usage in Claude Code:**
```bash
claude mcp add get-my-notion-mcp -- npx -y @parkjonghun/get-my-notion-mcp
```

### Method 2: Direct Binary (For Developers)

**Advantages:**
- No Node.js dependency
- Faster startup time
- Direct control over binary

**Build and install:**
```bash
cargo build --release
cp target/release/get-my-notion-mcp /usr/local/bin/
```

**Usage in Cursor:**
```json
{
  "mcpServers": {
    "get-my-notion-mcp": {
      "command": "/usr/local/bin/get-my-notion-mcp"
    }
  }
}
```

**Usage in Claude Code:**
```bash
claude mcp add get-my-notion-mcp -- /usr/local/bin/get-my-notion-mcp
```

### Method 3: Development Mode

**For active development:**
```json
{
  "mcpServers": {
    "get-my-notion-mcp": {
      "command": "cargo",
      "args": ["run", "--release"],
      "cwd": "/path/to/get-my-notion-mcp"
    }
  }
}
```

### Method 4: GitHub Releases

**For specific versions:**
```bash
# Download from GitHub releases
curl -L https://github.com/ParkJong-Hun/get-my-notion-mcp/releases/latest/download/get-my-notion-mcp-linux -o get-my-notion-mcp
chmod +x get-my-notion-mcp
./get-my-notion-mcp
```

## Platform-Specific Instructions

### macOS

**Homebrew (Future):**
```bash
# When available
brew install parkjonghun/tap/get-my-notion-mcp
```

**Manual Installation:**
```bash
# Download macOS binary
curl -L https://github.com/ParkJong-Hun/get-my-notion-mcp/releases/latest/download/get-my-notion-mcp-macos -o /usr/local/bin/get-my-notion-mcp
chmod +x /usr/local/bin/get-my-notion-mcp
```

### Linux

**Debian/Ubuntu:**
```bash
# Download and install
wget https://github.com/ParkJong-Hun/get-my-notion-mcp/releases/latest/download/get-my-notion-mcp-linux
sudo mv get-my-notion-mcp-linux /usr/local/bin/get-my-notion-mcp
sudo chmod +x /usr/local/bin/get-my-notion-mcp
```

**Arch Linux (AUR - Future):**
```bash
# When available
yay -S get-my-notion-mcp
```

### Windows

**Direct Download:**
```powershell
# Download Windows binary
Invoke-WebRequest -Uri "https://github.com/ParkJong-Hun/get-my-notion-mcp/releases/latest/download/get-my-notion-mcp-windows.exe" -OutFile "get-my-notion-mcp.exe"
```

**Chocolatey (Future):**
```powershell
# When available
choco install get-my-notion-mcp
```

## Configuration Locations

### Cursor Settings

**macOS:**
```
~/Library/Application Support/Cursor/User/settings.json
```

**Windows:**
```
%APPDATA%\Cursor\User\settings.json
```

**Linux:**
```
~/.config/Cursor/User/settings.json
```

### Claude Code Configuration

Claude Code stores MCP configurations in its own config directory. Use the CLI commands for setup:

```bash
# List configured servers
claude mcp list

# Add server
claude mcp add get-my-notion-mcp -- npx -y @parkjonghun/get-my-notion-mcp

# Remove server
claude mcp remove get-my-notion-mcp
```

## Environment Variables

### Optional Configuration

```bash
# Enable debug logging
export RUST_LOG=debug

# Custom GitHub API timeout (not implemented yet)
export GITHUB_TIMEOUT=30

# Custom user agent (not implemented yet)
export GITHUB_USER_AGENT="my-custom-agent"
```

## Docker Deployment

### Dockerfile

```dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/get-my-notion-mcp /usr/local/bin/
EXPOSE 3000
CMD ["get-my-notion-mcp"]
```

### Usage with Docker

```bash
# Build image
docker build -t get-my-notion-mcp .

# Run container
docker run -p 3000:3000 get-my-notion-mcp
```

**Cursor configuration with Docker:**
```json
{
  "mcpServers": {
    "get-my-notion-mcp": {
      "command": "docker",
      "args": ["run", "--rm", "-i", "get-my-notion-mcp"]
    }
  }
}
```

## CI/CD Pipeline

### GitHub Actions

```yaml
name: Build and Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    
    runs-on: ${{ matrix.os }}
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    
    - name: Build
      run: cargo build --release
    
    - name: Upload artifacts
      uses: actions/upload-artifact@v3
      with:
        name: get-my-notion-mcp-${{ matrix.os }}
        path: target/release/get-my-notion-mcp*
```

### npm Publishing

```yaml
- name: Setup Node.js
  uses: actions/setup-node@v3
  with:
    node-version: '18'
    registry-url: 'https://registry.npmjs.org'

- name: Build and publish
  run: |
    npm run build
    npm publish --access public
  env:
    NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

## Performance Considerations

### Memory Usage
- **Binary size**: ~5-10MB
- **Runtime memory**: ~10-20MB
- **Startup time**: <100ms

### Network Usage
- **GitHub API calls**: 1-3 requests per operation
- **Rate limits**: 60 requests/hour (unauthenticated)
- **Bandwidth**: Minimal (JSON responses only)

### Scaling
- **Concurrent users**: Limited by GitHub API rate limits
- **Horizontal scaling**: Each instance has independent rate limits
- **Caching**: Consider implementing for frequently accessed files

## Security Considerations

### Network Security
- All GitHub API calls use HTTPS
- No sensitive data stored locally
- Public repository access only

### Binary Security
- Rust memory safety features
- No dynamic code execution
- Minimal attack surface

### Deployment Security
- Verify binary checksums
- Use official releases only
- Regular security updates

## Monitoring and Logging

### Built-in Logging
```bash
# Enable debug logs
RUST_LOG=debug get-my-notion-mcp

# Log to file
RUST_LOG=info get-my-notion-mcp 2>&1 | tee mcp.log
```

### Health Checks
```bash
# Verify server is working
echo '{"jsonrpc":"2.0","id":"test","method":"initialize","params":{"protocolVersion":"2025-08-16","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | get-my-notion-mcp
```

### Metrics Collection
- GitHub API response times
- Error rates by tool
- Usage patterns by file type

## Troubleshooting Deployment

### Common Issues

1. **Binary not found**
   - Verify PATH configuration
   - Check file permissions
   - Confirm binary architecture matches system

2. **Network connectivity**
   - Test GitHub API access: `curl https://api.github.com`
   - Check firewall settings
   - Verify DNS resolution

3. **Permission errors**
   - Ensure execute permissions: `chmod +x get-my-notion-mcp`
   - Check user privileges
   - Verify directory permissions

4. **Version conflicts**
   - Use specific versions in configuration
   - Clear npm cache: `npm cache clean --force`
   - Reinstall: `npm uninstall -g @parkjonghun/get-my-notion-mcp && npm install -g @parkjonghun/get-my-notion-mcp`

### Support Channels

- **GitHub Issues**: Bug reports and feature requests
- **Documentation**: API and deployment guides
- **Community**: Discussions and help