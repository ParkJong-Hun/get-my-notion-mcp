mod mcp;
mod server;
mod github;
mod handlers;
mod constants;
mod utils;

use anyhow::Result;
use clap::Parser;
use server::McpServer;
use handlers::*;
use utils::*;

#[derive(Parser)]
#[command(name = "get-my-notion-mcp")]
#[command(about = "MCP server for accessing my-notion GitHub repository")]
struct Cli {}

#[tokio::main]
async fn main() -> Result<()> {
    let _cli = Cli::parse();

    let mut server = McpServer::new();

    // Add tools for GitHub repository access using utility functions
    server.add_tool(create_list_files_tool(), ListFilesHandler::new());
    server.add_tool(create_get_file_content_tool(), GetFileContentHandler::new());
    server.add_tool(create_get_latest_commit_tool(), GetLatestCommitHandler::new());
    server.add_resource(create_repo_info_resource(), NotionRepoResourceHandler::new());

    server.run().await
}
