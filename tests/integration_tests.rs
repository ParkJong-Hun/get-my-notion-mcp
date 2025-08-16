use get_my_notion_mcp::mcp::*;
use get_my_notion_mcp::server::McpServer;
use get_my_notion_mcp::handlers::*;
use serde_json;
use std::collections::HashMap;

#[tokio::test]
async fn test_mcp_server_initialization() {
    let mut server = McpServer::new();

    let list_files_tool = Tool {
        name: "list_files".to_string(),
        description: "List files in the my-notion GitHub repository".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Optional path within the repository (default: root)"
                }
            }
        }),
    };

    let repo_info_resource = Resource {
        uri: "notion://repo/info".to_string(),
        name: "My Notion Repository Info".to_string(),
        description: Some("Information about the my-notion GitHub repository".to_string()),
        mime_type: Some("text/plain".to_string()),
    };

    server.add_tool(list_files_tool, ListFilesHandler::new());
    server.add_resource(repo_info_resource, NotionRepoResourceHandler::new());

    // Test that server can be created and tools/resources added
    assert!(true);
}

#[test]
fn test_mcp_request_serialization() {
    let initialize_request = McpRequest::Initialize {
        id: "1".to_string(),
        params: InitializeParams {
            protocol_version: "2025-08-16".to_string(),
            capabilities: ClientCapabilities {
                roots: Some(RootsCapability {
                    list_changed: Some(false),
                }),
                sampling: Some(SamplingCapability {}),
            },
            client_info: ClientInfo {
                name: "test-client".to_string(),
                version: "1.0.0".to_string(),
            },
        },
    };

    let serialized = serde_json::to_string(&initialize_request).unwrap();
    assert!(serialized.contains("initialize"));
    assert!(serialized.contains("test-client"));
}

#[test]
fn test_mcp_response_serialization() {
    let initialize_response = McpResponse::Initialize {
        id: "1".to_string(),
        result: InitializeResult {
            protocol_version: "2025-08-16".to_string(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: Some(false),
                }),
                resources: Some(ResourcesCapability {
                    subscribe: Some(false),
                    list_changed: Some(false),
                }),
            },
            server_info: ServerInfo {
                name: "get-my-notion-mcp".to_string(),
                version: "0.1.5".to_string(),
            },
        },
    };

    let serialized = serde_json::to_string(&initialize_response).unwrap();
    assert!(serialized.contains("get-my-notion-mcp"));
    assert!(serialized.contains("2025-08-16"));
}

#[test]
fn test_tool_content_serialization() {
    let tool_content = ToolContent::Text {
        text: "Hello, World!".to_string(),
    };

    let serialized = serde_json::to_string(&tool_content).unwrap();
    assert!(serialized.contains("text"));
    assert!(serialized.contains("Hello, World!"));
}

#[test]
fn test_resource_content_serialization() {
    let resource_content = ResourceContent::Text {
        uri: "notion://repo/info".to_string(),
        text: "Repository information".to_string(),
    };

    let serialized = serde_json::to_string(&resource_content).unwrap();
    assert!(serialized.contains("notion://repo/info"));
    assert!(serialized.contains("Repository information"));
}

#[test]
fn test_mcp_error_serialization() {
    let error = McpError {
        code: -32601,
        message: "Method not found".to_string(),
        data: Some(serde_json::json!({"additional": "info"})),
    };

    let serialized = serde_json::to_string(&error).unwrap();
    assert!(serialized.contains("-32601"));
    assert!(serialized.contains("Method not found"));
    assert!(serialized.contains("additional"));
}

#[test]
fn test_call_tool_params() {
    let mut arguments = HashMap::new();
    arguments.insert("path".to_string(), serde_json::Value::String("README.md".to_string()));

    let params = CallToolParams {
        name: "get_file_content".to_string(),
        arguments: Some(arguments),
    };

    let serialized = serde_json::to_string(&params).unwrap();
    assert!(serialized.contains("get_file_content"));
    assert!(serialized.contains("README.md"));
}

#[test]
fn test_read_resource_params() {
    let params = ReadResourceParams {
        uri: "notion://repo/info".to_string(),
    };

    let serialized = serde_json::to_string(&params).unwrap();
    assert!(serialized.contains("notion://repo/info"));
}