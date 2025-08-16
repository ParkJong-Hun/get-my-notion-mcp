// GitHub API related constants
pub mod github {
    pub const API_BASE_URL: &str = "https://api.github.com";
    pub const USER_AGENT: &str = "get-my-notion-mcp";
    pub const DEFAULT_OWNER: &str = "ParkJong-Hun";
    pub const DEFAULT_REPO: &str = "my-notion";
    pub const DEFAULT_BRANCH: &str = "main";
    pub const BASE64_ENCODING: &str = "base64";
}

// MCP Protocol related constants
pub mod mcp {
    pub const PROTOCOL_VERSION: &str = "2025-08-16";
    pub const SERVER_NAME: &str = "get-my-notion-mcp";
    pub const SERVER_VERSION: &str = "0.1.2";
    pub const SERVER_DESCRIPTION: &str = "MCP server for accessing my-notion GitHub repository";
    
    // Tool names
    pub const TOOL_LIST_FILES: &str = "list_files";
    pub const TOOL_GET_FILE_CONTENT: &str = "get_file_content";
    pub const TOOL_GET_LATEST_COMMIT: &str = "get_latest_commit";
    
    // Resource URIs
    pub const RESOURCE_REPO_INFO: &str = "notion://repo/info";
    
    // Parameter names
    pub const PARAM_PATH: &str = "path";
    pub const PARAM_MESSAGE: &str = "message";
}

// Error messages
pub mod errors {
    pub const GITHUB_API_FAILED: &str = "GitHub API request failed";
    pub const PATH_REQUIRED: &str = "Path parameter is required";
    pub const COMMIT_SHA_EXTRACT_FAILED: &str = "Could not extract commit SHA";
    pub const TOOL_NOT_FOUND: &str = "Tool not found";
    pub const RESOURCE_NOT_FOUND: &str = "Resource not found";
    pub const TOOL_EXECUTION_FAILED: &str = "Tool execution failed";
    pub const RESOURCE_READ_FAILED: &str = "Resource read failed";
    pub const UNKNOWN_RESOURCE_URI: &str = "Unknown resource URI";
}

// JSON-RPC error codes
pub mod rpc_errors {
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INTERNAL_ERROR: i32 = -32603;
}

// Schema definitions
pub mod schemas {
    use serde_json::json;
    
    pub fn list_files_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Optional path within the repository (default: root)"
                }
            }
        })
    }
    
    pub fn get_file_content_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file within the repository"
                }
            },
            "required": ["path"]
        })
    }
    
    pub fn get_latest_commit_schema() -> serde_json::Value {
        json!({
            "type": "object"
        })
    }
}

// MIME types
pub mod mime_types {
    pub const TEXT_PLAIN: &str = "text/plain";
    pub const APPLICATION_JSON: &str = "application/json";
}