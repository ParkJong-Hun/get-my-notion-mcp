mod mcp;
mod server;

use anyhow::Result;
use clap::Parser;
use server::{McpServer, ResourceHandler, ToolHandler};
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "get-my-notion-mcp")]
#[command(about = "A Model Context Protocol server")]
struct Cli {}

struct EchoToolHandler;

impl ToolHandler for EchoToolHandler {
    fn call(&self, arguments: Option<HashMap<String, serde_json::Value>>) -> Result<mcp::CallToolResult> {
        let message = arguments
            .as_ref()
            .and_then(|args| args.get("message"))
            .and_then(|v| v.as_str())
            .unwrap_or("Hello, World!");

        Ok(mcp::CallToolResult {
            content: vec![mcp::ToolContent::Text {
                text: format!("Echo: {}", message),
            }],
        })
    }
}

struct HelloResourceHandler;

impl ResourceHandler for HelloResourceHandler {
    fn read(&self, _uri: &str) -> Result<mcp::ReadResourceResult> {
        Ok(mcp::ReadResourceResult {
            contents: vec![mcp::ResourceContent::Text {
                uri: "hello://world".to_string(),
                text: "Hello from the MCP server!".to_string(),
            }],
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let _cli = Cli::parse();

    let mut server = McpServer::new();

    let echo_tool = mcp::Tool {
        name: "echo".to_string(),
        description: "Echo a message back".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "The message to echo"
                }
            }
        }),
    };

    let hello_resource = mcp::Resource {
        uri: "hello://world".to_string(),
        name: "Hello World".to_string(),
        description: Some("A simple hello world resource".to_string()),
        mime_type: Some("text/plain".to_string()),
    };

    server.add_tool(echo_tool, EchoToolHandler);
    server.add_resource(hello_resource, HelloResourceHandler);

    server.run().await
}
