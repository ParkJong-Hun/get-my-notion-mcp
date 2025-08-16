use crate::github::GitHubClient;
use crate::mcp::*;
use crate::server::{ResourceHandler, ToolHandler};
use anyhow::Result;
use std::collections::HashMap;

pub struct ListFilesHandler {
    github_client: GitHubClient,
}

impl ListFilesHandler {
    pub fn new() -> Self {
        Self {
            github_client: GitHubClient::new("ParkJong-Hun".to_string(), "my-notion".to_string()),
        }
    }
}

impl ToolHandler for ListFilesHandler {
    fn call(&self, arguments: Option<HashMap<String, serde_json::Value>>) -> Result<CallToolResult> {
        let rt = tokio::runtime::Runtime::new()?;
        
        let path = arguments
            .as_ref()
            .and_then(|args| args.get("path"))
            .and_then(|v| v.as_str());

        let files = rt.block_on(async {
            self.github_client.list_files(path).await
        })?;

        let mut content = String::new();
        content.push_str("Files in repository:\n\n");
        
        for file in files {
            content.push_str(&format!(
                "- **{}** ({})\n  Path: {}\n  Type: {}\n",
                file.name, 
                file.sha[..7].to_string(),
                file.path,
                file.file_type
            ));
            if let Some(size) = file.size {
                content.push_str(&format!("  Size: {} bytes\n", size));
            }
            content.push('\n');
        }

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: content }],
        })
    }
}

pub struct GetFileContentHandler {
    github_client: GitHubClient,
}

impl GetFileContentHandler {
    pub fn new() -> Self {
        Self {
            github_client: GitHubClient::new("ParkJong-Hun".to_string(), "my-notion".to_string()),
        }
    }
}

impl ToolHandler for GetFileContentHandler {
    fn call(&self, arguments: Option<HashMap<String, serde_json::Value>>) -> Result<CallToolResult> {
        let rt = tokio::runtime::Runtime::new()?;
        
        let path = arguments
            .as_ref()
            .and_then(|args| args.get("path"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Path parameter is required"))?;

        let content = rt.block_on(async {
            self.github_client.get_file_content(path).await
        })?;

        let response_text = format!(
            "Content of file: {}\n\n```\n{}\n```",
            path,
            content
        );

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: response_text }],
        })
    }
}

pub struct GetLatestCommitHandler {
    github_client: GitHubClient,
}

impl GetLatestCommitHandler {
    pub fn new() -> Self {
        Self {
            github_client: GitHubClient::new("ParkJong-Hun".to_string(), "my-notion".to_string()),
        }
    }
}

impl ToolHandler for GetLatestCommitHandler {
    fn call(&self, _arguments: Option<HashMap<String, serde_json::Value>>) -> Result<CallToolResult> {
        let rt = tokio::runtime::Runtime::new()?;
        
        let sha = rt.block_on(async {
            self.github_client.get_latest_commit_sha().await
        })?;

        let response_text = format!("Latest commit SHA: {}", sha);

        Ok(CallToolResult {
            content: vec![ToolContent::Text { text: response_text }],
        })
    }
}

pub struct NotionRepoResourceHandler {
    github_client: GitHubClient,
}

impl NotionRepoResourceHandler {
    pub fn new() -> Self {
        Self {
            github_client: GitHubClient::new("ParkJong-Hun".to_string(), "my-notion".to_string()),
        }
    }
}

impl ResourceHandler for NotionRepoResourceHandler {
    fn read(&self, uri: &str) -> Result<ReadResourceResult> {
        let rt = tokio::runtime::Runtime::new()?;
        
        if uri == "notion://repo/info" {
            let sha = rt.block_on(async {
                self.github_client.get_latest_commit_sha().await
            })?;
            
            let info = format!(
                "Repository: ParkJong-Hun/my-notion\nLatest commit: {}\nAccess via: https://github.com/ParkJong-Hun/my-notion",
                sha
            );
            
            Ok(ReadResourceResult {
                contents: vec![ResourceContent::Text {
                    uri: uri.to_string(),
                    text: info,
                }],
            })
        } else {
            Err(anyhow::anyhow!("Unknown resource URI: {}", uri))
        }
    }
}