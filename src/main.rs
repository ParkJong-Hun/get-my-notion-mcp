mod mcp;
mod server;
mod github;
mod handlers;

use anyhow::Result;
use clap::Parser;
use server::McpServer;
use handlers::*;

#[derive(Parser)]
#[command(name = "get-my-notion-mcp")]
#[command(about = "MCP server for accessing my-notion GitHub repository")]
struct Cli {}

#[tokio::main]
async fn main() -> Result<()> {
    let _cli = Cli::parse();

    let mut server = McpServer::new();

    // Add tools for GitHub repository access
    let list_files_tool = mcp::Tool {
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

    let get_file_content_tool = mcp::Tool {
        name: "get_file_content".to_string(),
        description: "Get content of a specific file from the my-notion repository".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to the file within the repository"
                }
            },
            "required": ["path"]
        }),
    };

    let get_latest_commit_tool = mcp::Tool {
        name: "get_latest_commit".to_string(),
        description: "Get the latest commit SHA from the my-notion repository".to_string(),
        input_schema: serde_json::json!({
            "type": "object"
        }),
    };

    // Add resource for repository information
    let repo_info_resource = mcp::Resource {
        uri: "notion://repo/info".to_string(),
        name: "My Notion Repository Info".to_string(),
        description: Some("Information about the my-notion GitHub repository".to_string()),
        mime_type: Some("text/plain".to_string()),
    };

    server.add_tool(list_files_tool, ListFilesHandler::new());
    server.add_tool(get_file_content_tool, GetFileContentHandler::new());
    server.add_tool(get_latest_commit_tool, GetLatestCommitHandler::new());
    server.add_resource(repo_info_resource, NotionRepoResourceHandler::new());

    server.run().await
}
